mod logging;

use std::time::{Instant, Duration};
use cosmic::{
    app::{Core, Settings, Task},
    iced::{
        Length, Subscription,
        mouse,
        keyboard,
        Event,
        widget::{
            canvas::{self, Cache, Canvas, Frame, Geometry},
            stack, image,
        },
        Rectangle, Point, Size, Color,
    },
    Application, Element, Theme,
    widget::container,
};
use threednterminal::effects::crossfade::CrossfadeManager;
use threednterminal::terminal::traits::Terminal;
use threednterminal::{terminal, ui, config, effects, AnimationPhase};

#[derive(Debug, Clone)]
pub enum Message {
    Tick(Instant),
    ToggleTerminal,
    KeyPressed(keyboard::Key, keyboard::Modifiers, Option<String>),
    Scroll(f32),
    WindowResized(f32, f32),
    CursorMoved(Point),
    MouseClicked(Instant),
    TerminalClosed,
    MinimizeTerminal,
    MaximizeTerminal,
    CloseApp,
}


use std::sync::mpsc;

struct EffectEngine {
    receiver: mpsc::Receiver<Vec<u8>>,
}

impl EffectEngine {
    fn start(width: u32, height: u32) -> Self {
        let (tx, rx) = mpsc::channel();
        tokio::task::spawn_blocking(move || {
            let mut crossfade = CrossfadeManager::new(width, height);
            let mut bg_pixmap = tiny_skia::Pixmap::new(width, height).unwrap();
            let mut last_update = Instant::now();
            
            loop {
                let now = Instant::now();
                let dt = now.duration_since(last_update).as_secs_f32();
                last_update = now;
                
                crossfade.tick(dt, &mut bg_pixmap.as_mut());
                if tx.send(bg_pixmap.data().to_vec()).is_err() {
                    break;
                }
                
                // Limitiere den Background-Thread auf ~60 FPS
                std::thread::sleep(Duration::from_millis(16));
            }
        });
        Self { receiver: rx }
    }
}

pub struct App {
    core: Core,
    cache: Cache,
    // --- Terminal State ---
    terminal_engine: terminal::TerminalEngine,
    config: config::Config,

    // --- Background Effects ---
    effect_engine: EffectEngine,
    bg_handle: Option<image::Handle>,

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

        let app = App {
            core,
            effect_engine: EffectEngine::start(1280, 720),
            terminal_engine,
            config,
            last_update: Instant::now(),
            start_time: Instant::now(),
            cursor_pos: Point::ORIGIN,
            last_click_time: Instant::now() - Duration::from_secs(10),
            cursor_visible: true,
            cache: Cache::new(),
            bg_handle: None,
            phase: AnimationPhase::Expanded,
            progress: 1.0,
            // Standard Positionen
            // Ecke unten-rechts (mit kleinem Abstand zum Rand) und deutlich kompakter als das große Fenster
            corner_rect: Rectangle::new(Point::new(1280.0 - 450.0, 720.0 - 300.0), Size::new(400.0, 250.0)),
            center_rect: Rectangle::new(Point::new(1280.0 * 0.06, 720.0 * 0.09), Size::new(1280.0 * 0.88, 720.0 * 0.82)),
        };
        (app, Task::none())
    }

    fn update(&mut self, message: Message) -> Task<Message> {
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
                
                // --- UPDATE BACKGROUND EFFECTS ---
                while let Ok(frame_data) = self.effect_engine.receiver.try_recv() {
                    self.bg_handle = Some(image::Handle::from_rgba(
                        1280,
                        720,
                        frame_data
                    ));
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

                if needs_redraw {
                    self.cache.clear();
                }
            }
            Message::ToggleTerminal => {
                if self.phase == AnimationPhase::Collapsed {
                    self.phase = AnimationPhase::Expanding;
                    self.progress = 0.0;
                    // Shell bleibt aktiv, wir rufen nicht mehr spawn_shell() auf
                } else if self.phase == AnimationPhase::Expanded {
                    self.phase = AnimationPhase::Collapsing;
                    self.progress = 0.0;
                    // Shell bleibt aktiv, wir rufen nicht mehr kill_shell() auf
                    // Der Text bleibt in der Ecke sichtbar
                }
                self.cache.clear();
            }
            Message::KeyPressed(key, modifiers, _text) => {
                if key == self.config.flip_key {
                    if self.phase == AnimationPhase::Collapsed {
                        self.phase = AnimationPhase::Expanding;
                        self.progress = 0.0;
                    } else if self.phase == AnimationPhase::Expanded {
                        self.phase = AnimationPhase::Collapsing;
                        self.progress = 0.0;
                    }
                    self.cache.clear();
                    return Task::none();
                }

                if self.phase == AnimationPhase::Expanded {
                    self.terminal_engine.send_key(&key, modifiers, &self.config);
                }
            }
            Message::Scroll(delta) => {
                if self.phase == AnimationPhase::Expanded {
                    if delta > 0.0 {
                        self.terminal_engine.scroll_up(3);
                    } else if delta < 0.0 {
                        self.terminal_engine.scroll_down(3);
                    }
                }
            }
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
            Message::CursorMoved(pos) => {
                self.cursor_pos = pos;
            }
            Message::MouseClicked(now) => {
                let is_double_click = now.duration_since(self.last_click_time) < Duration::from_millis(300);
                self.last_click_time = now;
                
                let params = ui::hologram::HologramParams {
                    phase: self.phase,
                    progress: self.progress,
                    start_time: self.start_time,
                    corner_rect: self.corner_rect,
                    center_rect: self.center_rect,
                    cursor_visible: self.cursor_visible,
                };
                let quad = ui::hologram::get_quad(&params);
                if ui::math::is_point_in_quad(self.cursor_pos, &quad) {
                    if self.phase == AnimationPhase::Collapsed {
                        self.phase = AnimationPhase::Expanding;
                        self.progress = 0.0;
                        self.cache.clear();
                    } else if self.phase == AnimationPhase::Expanded {
                        if is_double_click {
                            self.phase = AnimationPhase::Collapsing;
                            self.progress = 0.0;
                            self.cache.clear();
                        } else {
                            let min_y = quad[0].y.min(quad[1].y);
                            let max_y = quad[3].y.max(quad[2].y);
                            let height = max_y - min_y;
                            if self.cursor_pos.y < min_y + height * 0.15 { // Top 15% header click
                                self.phase = AnimationPhase::Collapsing;
                                self.progress = 0.0;
                                self.cache.clear();
                            }
                        }
                    }
                }
            }
            Message::TerminalClosed => {}
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let canvas = Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill);

        let mut layers: Vec<Element<'_, Message>> = Vec::new();
        
        // Hintergrund-Effekte rendern (falls vorhanden)
        if let Some(handle) = &self.bg_handle {
            let img = image(handle.clone())
                .width(Length::Fill)
                .height(Length::Fill)
                .content_fit(cosmic::iced::ContentFit::Fill);
            layers.push(img.into());
        }
        
        // Canvas (Hologramm) drüberlegen
        layers.push(canvas.into());

        container(stack(layers))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        let tick = cosmic::iced::time::every(Duration::from_millis(16)).map(Message::Tick);
        
        let events = cosmic::iced::event::listen_with(|event, status, _window_id| {
            if status == cosmic::iced::event::Status::Ignored {
                match event {
                    Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, text, .. }) => {
                        return Some(Message::KeyPressed(key, modifiers, text.map(|t| t.to_string())));
                    }
                    Event::Mouse(mouse::Event::CursorMoved { position }) => {
                        return Some(Message::CursorMoved(position));
                    }
                    Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                        return Some(Message::MouseClicked(Instant::now()));
                    }
                    Event::Mouse(mouse::Event::WheelScrolled { delta }) => {
                        match delta {
                            mouse::ScrollDelta::Lines { y, .. } => return Some(Message::Scroll(y)),
                            mouse::ScrollDelta::Pixels { y, .. } => return Some(Message::Scroll(y.signum())),
                        }
                    }
                    Event::Window(cosmic::iced::window::Event::Resized(size)) => {
                        return Some(Message::WindowResized(size.width, size.height));
                    }
                    _ => {}
                }
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
