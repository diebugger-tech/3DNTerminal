mod logging;

use cosmic::{
    app::{Core, Settings, Task},
    iced::{
        Length, Subscription,
        mouse,
        keyboard,
        Event,
        widget::{
            canvas::{self, Cache, Canvas, Frame, Geometry},
        },
        Rectangle, Point, Size, Color,
    },
    Application, Element, Theme,
    widget::container,
};
use threednterminal::terminal::traits::Terminal;
use threednterminal::{terminal, ui, config, AnimationPhase};
use threednterminal::app::events::Message;

use std::time::{Instant, Duration};

// EffectEngine removed as requested - running directly on desktop

pub struct App {
    core: Core,
    cache: Cache,
    // --- Terminal State ---
    terminal_engine: terminal::TerminalEngine,
    config: config::Config,

    // --- Animation State ---
    phase: AnimationPhase,
    progress: f32, // 0.0 to 1.0
    last_update: Instant,
    start_time: Instant,
    
    // --- Input State ---
    cursor_pos: Point,
    last_click_time: Instant,
    cursor_visible: bool,
    
    // Config
    corner_rect: Rectangle,
    center_rect: Rectangle,
    
    // UI Components
    window_controls: ui::window_controls::WindowControls,
    last_p2: Point,
}

impl canvas::Program<Message, Theme> for App {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &cosmic::iced::Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let geometry = self.cache.draw(renderer, bounds.size(), |frame: &mut Frame| {
            let params = ui::hologram::HologramParams {
                phase: self.phase,
                progress: self.progress,
                start_time: self.start_time,
                corner_rect: self.corner_rect,
                center_rect: self.center_rect,
                cursor_visible: self.cursor_visible,
                window_controls: Some(&self.window_controls),
            };
            
            let (_, _, alpha) = ui::hologram::calculate_3d_geometry(&params);
            
            if alpha > 0.0 {
                frame.fill_rectangle(Point::ORIGIN, bounds.size(), Color::from_rgba(0.02, 0.02, 0.05, alpha));
            }

            ui::hologram::draw(frame, &self.terminal_engine.grid, &params);
        });
        vec![geometry]
    }
}

impl Application for App {
    type Executor = cosmic::executor::Default;
    type Flags    = ();
    type Message  = Message;
    const APP_ID: &'static str = "de.diebugger.3dnterminal";

    fn core(&self) -> &Core             { &self.core }
    fn core_mut(&mut self) -> &mut Core { &mut self.core }

    fn init(core: Core, _flags: ()) -> (Self, Task<Message>) {
        let mut terminal_engine = terminal::TerminalEngine::new(80, 24);
        if let Err(e) = terminal_engine.spawn_shell() {
            eprintln!("Failed to spawn shell: {}", e);
        }
        let config = config::Config::load("~/.config/3dnterminal/config.toml").unwrap_or_default();

        let corner_rect = Rectangle::new(Point::new(1280.0 - 450.0, 720.0 - 300.0), Size::new(400.0, 250.0));
        let center_rect = Rectangle::new(Point::new(1280.0 * 0.06, 720.0 * 0.09), Size::new(1280.0 * 0.88, 720.0 * 0.82));

        let app = App {
            core,
            terminal_engine,
            config,
            last_update: Instant::now(),
            start_time: Instant::now(),
            cursor_pos: Point::ORIGIN,
            last_click_time: Instant::now() - Duration::from_secs(10),
            cursor_visible: true,
            cache: Cache::new(),
            phase: AnimationPhase::Expanded,
            progress: 1.0,
            corner_rect,
            center_rect,
            window_controls: ui::window_controls::WindowControls::new(),
            last_p2: Point::ORIGIN,
        };

        // Initial button positioning removed - now stateless
        (app, Task::none())
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        if !matches!(message, Message::Tick(_)) {
            tracing::debug!("Update Message: {:?}", message);
        }
        match message {
            Message::Tick(now) => {
                let dt = now.duration_since(self.last_update).as_secs_f32();
                self.last_update = now;

                let mut needs_redraw = false;

                if self.phase == AnimationPhase::Expanding || self.phase == AnimationPhase::Collapsing {
                    self.progress += dt / 0.6; // Gesamtdauer 600ms
                    if self.progress >= 1.0 {
                        self.progress = 0.0;
                        self.phase = if self.phase == AnimationPhase::Expanding { AnimationPhase::Expanded } else { AnimationPhase::Collapsed };
                    }
                    needs_redraw = true; // Immer redraw während Animation
                } else if self.phase == AnimationPhase::Collapsed {
                    // Hover effekt wenn collapsed: Breathe Animation (benötigt ständigen Redraw)
                    needs_redraw = true;
                }
                
                // Grid Dirty Check
                if self.terminal_engine.is_dirty() {
                    needs_redraw = true;
                }

                let current_cursor_visible = (now.duration_since(self.start_time).as_millis() / 500) % 2 == 0;
                if self.cursor_visible != current_cursor_visible {
                    self.cursor_visible = current_cursor_visible;
                    needs_redraw = true;
                }

                if self.phase == AnimationPhase::Expanded || self.phase == AnimationPhase::Collapsed {
                    let rect = if self.phase == AnimationPhase::Expanded {
                        self.center_rect
                    } else {
                        self.corner_rect
                    };
                    
                    let margin_y = (rect.height * 0.05).clamp(10.0, 30.0);
                    let base_font_size = (rect.height / 30.0).clamp(10.0, 18.0);
                    
                    self.last_p2 = Point::new(
                        rect.x + rect.width,
                        rect.y + margin_y + base_font_size,
                    );
                }

                if needs_redraw {
                    self.cache.clear();
                }
            }
            Message::ToggleTerminal => {
                let old_phase = self.phase;
                if self.phase == AnimationPhase::Collapsed {
                    self.phase = AnimationPhase::Expanding;
                    self.progress = 0.0;
                } else if self.phase == AnimationPhase::Expanded {
                    self.phase = AnimationPhase::Collapsing;
                    self.progress = 0.0;
                }
                tracing::info!("ToggleTerminal: {:?} -> {:?}", old_phase, self.phase);
                
                self.cache.clear();
            }
            Message::KeyPressed(key, modifiers, _text) => {
                tracing::debug!("KeyPressed: {:?} (mods: {:?})", key, modifiers);
                if key == self.config.flip_key {
                    let old_phase = self.phase;
                    if self.phase == AnimationPhase::Collapsed {
                        self.phase = AnimationPhase::Expanding;
                        self.progress = 0.0;
                    } else if self.phase == AnimationPhase::Expanded {
                        self.phase = AnimationPhase::Collapsing;
                        self.progress = 0.0;
                    }
                    tracing::info!("FlipKey Triggered: {:?} -> {:?}", old_phase, self.phase);
                    self.cache.clear();
                    return Task::none();
                }

                if self.phase == AnimationPhase::Expanded {
                    self.terminal_engine.send_key(&key, modifiers, &self.config);
                }
            }
            Message::Scroll(_) => {}
            Message::WindowResized(width, height) => {
                // Update rects
                self.corner_rect = Rectangle::new(Point::new(width - 450.0, height - 300.0), Size::new(400.0, 250.0));
                self.center_rect = Rectangle::new(Point::new(width * 0.06, height * 0.09), Size::new(width * 0.88, height * 0.82));
                
                // Calculate new grid size based on expanded mode (center_rect)
                let base_font_size = (self.center_rect.height / 30.0).clamp(10.0, 18.0);
                let char_width = base_font_size * 0.6;
                let line_height = base_font_size * 1.5;
                
                let margin_x = (self.center_rect.width * 0.05).clamp(5.0, 20.0);
                let margin_y = (self.center_rect.height * 0.05).clamp(10.0, 30.0);
                
                let usable_width = self.center_rect.width - (margin_x * 2.0);
                let usable_height = self.center_rect.height - (margin_y * 2.0) - (base_font_size * 3.0); // minus header
                
                let cols = (usable_width / char_width).max(20.0) as u16;
                let rows = (usable_height / line_height).max(10.0) as u16;
                
                self.terminal_engine.resize(cols, rows);
                if let Ok(mut grid) = self.terminal_engine.grid.lock() {
                    grid.resize(cols as usize, rows as usize);
                }
                
                self.cache.clear();
            }
            Message::CursorMoved(_) => {}
            Message::MouseClicked(_) => {}
            Message::MinimizeTerminal => {
                tracing::info!("Message: MinimizeTerminal");
                if self.phase == AnimationPhase::Expanded {
                    self.phase = AnimationPhase::Collapsing;
                    self.progress = 0.0;
                    self.cache.clear();
                }
            }
            Message::MaximizeTerminal => {
                tracing::info!("Message: MaximizeTerminal (Reserved)");
            }
            Message::CloseApp => {
                tracing::info!("Message: CloseApp - Shutting down");
                // In a real app, we might want to send a close request to the window manager
                std::process::exit(0);
            }
            Message::TerminalClosed => {}
            Message::RawMouseEvent(ev) => {
                match ev {
                    mouse::Event::CursorMoved { position } => {
                        self.cursor_pos = position;
                        self.cache.clear();
                    }
                    mouse::Event::ButtonReleased(mouse::Button::Left) => {
                        let click_pos = self.cursor_pos;
                        tracing::info!("🎯 Left Click at {:?}", click_pos);

                        let now = Instant::now();
                        let is_double_click = now.duration_since(self.last_click_time) < Duration::from_millis(300);
                        self.last_click_time = now;
                        
                        let params = ui::hologram::HologramParams {
                            phase: self.phase,
                            progress: self.progress,
                            start_time: self.start_time,
                            corner_rect: self.corner_rect,
                            center_rect: self.center_rect,
                            cursor_visible: self.cursor_visible,
                            window_controls: None,
                        };
                        let quad = ui::hologram::get_quad(&params);
                        
                        // 1. Check window controls first (stateless hit_test)
                        let btn_size = (self.center_rect.width * 0.03).clamp(12.0, 26.0);
                        if let Some(action) = self.window_controls.hit_test(click_pos, self.last_p2, btn_size) {
                            let msg = match action {
                                ui::window_controls::ButtonAction::Minimize => Message::MinimizeTerminal,
                                ui::window_controls::ButtonAction::Maximize => Message::MaximizeTerminal,
                                ui::window_controls::ButtonAction::Close    => Message::CloseApp,
                            };
                            tracing::info!("🎯 Button getroffen: {:?}", msg);
                            return self.update(msg);
                        }

                        // 2. Then check general quad interaction
                        if let Some((_u, v)) = ui::math::project_onto_quad(self.cursor_pos, &quad) {
                            if self.phase == AnimationPhase::Collapsed {
                                tracing::info!("Terminal expanded via click");
                                self.phase = AnimationPhase::Expanding;
                                self.progress = 0.0;
                                self.cache.clear();
                            } else if self.phase == AnimationPhase::Expanded {
                                if is_double_click || v < 0.1 { // Header click or double click
                                    tracing::info!("Terminal collapsed via header click/double-click");
                                    self.phase = AnimationPhase::Collapsing;
                                    self.progress = 0.0;
                                    self.cache.clear();
                                }
                            }
                        }
                    }
                    mouse::Event::WheelScrolled { delta } => {
                        if self.phase == AnimationPhase::Expanded {
                            let scroll_val = match delta {
                                mouse::ScrollDelta::Lines { y, .. } => y,
                                mouse::ScrollDelta::Pixels { y, .. } => y.signum(),
                            };
                            if scroll_val > 0.0 {
                                self.terminal_engine.scroll_up(3);
                            } else if scroll_val < 0.0 {
                                self.terminal_engine.scroll_down(3);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let canvas = Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill);

        container(canvas)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_theme: &Theme| container::Style {
                background: Some(Color::from_rgba(0.0, 0.0, 0.0, 0.01).into()),
                ..Default::default()
            })
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        let tick = cosmic::iced::time::every(Duration::from_millis(16)).map(Message::Tick);
        
        let events = cosmic::iced::event::listen_with(|event, _status, _window_id| {
            // Log ALL mouse events as requested
            if let Event::Mouse(mouse_event) = &event {
                tracing::info!("Subscription: Raw Mouse Event: {:?}", mouse_event);
            }

            match event {
                    Event::Mouse(ev) => {
                        return Some(Message::RawMouseEvent(ev));
                    }
                    Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, text, .. }) => {
                        return Some(Message::KeyPressed(key, modifiers, text.map(|t| t.to_string())));
                    }
                    Event::Window(cosmic::iced::window::Event::Resized(size)) => {
                        return Some(Message::WindowResized(size.width, size.height));
                    }
                    _ => {}
                }
                None
            });

        Subscription::batch(vec![tick, events])
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    logging::init();
    let settings = Settings::default()
        .size(Size::new(1280.0, 720.0))
        .transparent(true)
        .client_decorations(false);
    cosmic::app::run::<App>(settings, ())?;
    Ok(())
}
