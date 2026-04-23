mod effects;
mod terminal;
mod ui;

use std::time::{Instant, Duration};
use cosmic::{
    app::{Core, Settings, Task},
    iced::{
        Length, Subscription,
        mouse,
        keyboard,
        Event,
        widget::{
            canvas::{self, Cache, Canvas, Frame, Geometry, Path, Stroke},
            stack, image,
        },
        Rectangle, Point, Size, Color, Pixels,
    },
    Application, Element, Theme,
    widget::container,
};
use effects::crossfade::CrossfadeManager;

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
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnimationPhase {
    Collapsed,
    Expanding,
    Expanded,
    Collapsing,
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
            // 2. Terminal Fenster & Alpha Berechnung
            let (target_rect, rotate_y, alpha) = self.calculate_3d_geometry();
            
            // 1. Hintergrund-Effekt (Crossfade Manager)
            if alpha > 0.0 {
                frame.fill_rectangle(Point::ORIGIN, bounds.size(), Color::from_rgba(0.02, 0.02, 0.05, alpha));
            }

            // 3. 3D-Trapez zeichnen
            self.draw_3d_window(frame, target_rect, rotate_y);
        });
        vec![geometry]
    }
}

impl App {
    fn calculate_3d_geometry(&self) -> (Rectangle, f32, f32) {
        let switch_t = 0.416; // 250ms / 600ms mark
        
        match self.phase {
            AnimationPhase::Collapsed => {
                // Sanftes Schweben (Breathe)
                let time = self.start_time.elapsed().as_secs_f32();
                let hover = (time * 2.0).sin() * 8.0; // Etwas stärkeres Schweben
                let mut rect = self.corner_rect;
                rect.y += hover;
                (rect, -18.0, 0.4) // Basis-Alpha 0.4 (halbtransparent in der Ecke)
            }
            AnimationPhase::Expanded => (self.center_rect, 0.0, 1.0),
            AnimationPhase::Expanding | AnimationPhase::Collapsing => {
                let t = if self.phase == AnimationPhase::Expanding { self.progress } else { 1.0 - self.progress };
                let eased_t = ui::math::cubic_bezier(t);
                let alpha = 0.4 + (0.6 * eased_t); // Interpoliert von 0.4 zu 1.0
                
                if eased_t < switch_t {
                    // Phase 1: Flip Out (Corner)
                    let p = eased_t / switch_t;
                    let angle = p * 90.0;
                    (self.corner_rect, angle, alpha)
                } else {
                    // Phase 2: Flip In (Center)
                    let p = (eased_t - switch_t) / (1.0 - switch_t);
                    let angle = 90.0 * (1.0 - p);
                    (self.center_rect, angle, alpha)
                }
            }
        }
    }

    fn get_quad(&self) -> [Point; 4] {
        let (rect, angle_y, _) = self.calculate_3d_geometry();
        let center = rect.center();
        let rad = angle_y.to_radians();
        let cos_a = rad.cos();
        let w = rect.width * cos_a;
        let h = rect.height;
        let perspective = (rad.sin() * 40.0).abs();
        
        let p1 = Point::new(center.x - w/2.0, center.y - h/2.0 + perspective);
        let p2 = Point::new(center.x + w/2.0, center.y - h/2.0 - perspective);
        let p3 = Point::new(center.x + w/2.0, center.y + h/2.0 + perspective);
        let p4 = Point::new(center.x - w/2.0, center.y + h/2.0 - perspective);
        [p1, p2, p3, p4]
    }

    fn draw_3d_window(&self, frame: &mut Frame, rect: Rectangle, angle_y: f32) {
        let quad = self.get_quad();
        let p1 = quad[0];
        let p2 = quad[1];
        let p3 = quad[2];
        let p4 = quad[3];
        
        let rad = angle_y.to_radians();
        let cos_a = rad.cos();
        let w = rect.width * cos_a;
        let h = rect.height;

        let path = Path::new(|b| {
            b.move_to(p1);
            b.line_to(p2);
            b.line_to(p3);
            b.line_to(p4);
            b.close();
        });

        let alpha = self.calculate_3d_geometry().2;
        
        // Background and border alpha logic
        let bg_alpha = 0.8 * alpha; // Geht von 0.32 bis 0.8
        let border_alpha = alpha;   // Geht von 0.4 bis 1.0

        if bg_alpha > 0.0 {
            // Fenster-Body (Holographisches Blau)
            frame.fill(&path, Color::from_rgba(0.05, 0.1, 0.2, bg_alpha));
            
            // Glow-Effekt
            for i in 1..=6 {
                let glow_width = i as f32 * 4.0;
                let glow_alpha = (0.4 / i as f32) * border_alpha;
                frame.stroke(&path, Stroke::default()
                    .with_color(Color::from_rgba(0.4, 1.0, 0.8, glow_alpha))
                    .with_width(glow_width));
            }

            // Innerer scharfer Rand
            frame.stroke(&path, Stroke::default()
                .with_color(Color::from_rgba(0.4, 1.0, 0.8, border_alpha))
                .with_width(2.0)); 
        }

        // --- Dynamische Text-Größen & Zonen ---
        // Basis-Schriftgröße gekoppelt an die tatsächliche Fensterhöhe
        let base_font_size = (rect.height / 30.0).clamp(10.0, 18.0); 
        // Beim 3D-Flip skalieren wir die Textgröße horizontal mit (erzeugt den Perspektiven-Effekt)
        let font_size = base_font_size * cos_a.max(0.3);
        let line_height = font_size * 1.5;
        
        // Ränder relativ zur projizierten Breite/Höhe
        let margin_x = (w * 0.05).clamp(5.0, 20.0);
        let margin_y = (h * 0.05).clamp(10.0, 30.0);
        
        // Damit Text beim starken Rotieren nicht aus dem schmalen Rahmen bricht,
        // faden wir ihn aus, wenn cos_a < 0.4 wird (Clipping-Simulation)
        let flip_alpha = (cos_a * 2.5).clamp(0.0, 1.0);
        let text_alpha = flip_alpha; 

        if text_alpha > 0.0 {
            // --- TOP ZONE (System Header) ---
            if border_alpha > 0.0 {
                let top_y = p1.y + margin_y + font_size;
                frame.fill_text(canvas::Text {
                    content: "SYSTEM: NEURAL_LINK ACTIVE".to_string(),
                    position: Point::new(p1.x + margin_x, top_y),
                    color: Color::from_rgba(0.4, 1.0, 0.8, border_alpha * flip_alpha),
                    size: Pixels(font_size),
                    ..canvas::Text::default()
                });
                
                let box_w = 120.0 * (rect.width / 1126.0) * cos_a;
                frame.fill_rectangle(
                    Point::new(p2.x - box_w - margin_x, p2.y + margin_y),
                    Size::new(box_w, font_size * 1.5),
                    Color::from_rgba(0.4, 1.0, 0.8, 0.2 * border_alpha * flip_alpha)
                );
            }

            // --- TERMINAL OUTPUT ---
            // Wir zeigen ausschließlich den echten Output vom PTY (keine künstlichen Elemente)
            if let Ok(grid) = self.terminal_engine.grid.lock() {
                let start_y = p1.y + margin_y + (font_size * 2.0); // Platz für Top Zone
                
                for y in 0..grid.rows {
                    let current_y = start_y + (y as f32 * line_height);
                    
                    if current_y > p4.y - margin_y {
                        break; // Text-Clipping unten
                    }
                    
                    if let Some(row) = grid.get_visible_row(y) {
                        for x in 0..grid.cols {
                            if x >= row.len() { break; }
                            let cell = row[x];
                            if cell.char == ' ' && cell.bg == Color::TRANSPARENT {
                                continue;
                            }
                        
                        // Approx character width
                        let char_width = font_size * 0.6;
                        let pos = Point::new(p1.x + margin_x + (x as f32 * char_width), current_y);
                        
                        if cell.bg != Color::TRANSPARENT {
                            let mut bg_c = cell.bg;
                            bg_c.a *= text_alpha;
                            frame.fill_rectangle(
                                Point::new(pos.x, pos.y - font_size),
                                Size::new(char_width, line_height),
                                bg_c
                            );
                        }
                        
                        if cell.char != ' ' {
                            let mut fg_c = cell.fg;
                            fg_c.a *= text_alpha;
                            frame.fill_text(canvas::Text {
                                content: cell.char.to_string(),
                                position: pos,
                                color: fg_c,
                                size: Pixels(font_size),
                                font: cosmic::iced::Font::MONOSPACE,
                                ..canvas::Text::default()
                            });
                        }
                    }
                }
                }
                
                // Draw Cursor
                if self.cursor_visible && border_alpha > 0.0 {
                    let total_lines = grid.scrollback.len() + grid.rows;
                    let start_index = total_lines.saturating_sub(grid.rows + grid.viewport_offset);
                    let abs_cursor_y = grid.scrollback.len() + grid.cursor_y;
                    
                    if abs_cursor_y >= start_index && abs_cursor_y < start_index + grid.rows {
                        let screen_y = abs_cursor_y - start_index;
                        let current_y = start_y + (screen_y as f32 * line_height);
                        
                        if current_y <= p4.y - margin_y {
                            let char_width = font_size * 0.6;
                            let cursor_pos = Point::new(
                                p1.x + margin_x + (grid.cursor_x as f32 * char_width),
                                current_y
                            );
                            
                            frame.fill_rectangle(
                                Point::new(cursor_pos.x, cursor_pos.y - font_size),
                                Size::new(char_width, line_height),
                                Color::from_rgba(0.4, 1.0, 0.8, text_alpha * 0.8)
                            );
                        }
                    }
                }
            }
        }
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

        let app = App {
            core,
            effect_engine: EffectEngine::start(1280, 720),
            terminal_engine,
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

                // Grid Dirty Check & Cursor Blink
                if let Ok(mut grid) = self.terminal_engine.grid.lock() {
                    if grid.dirty {
                        needs_redraw = true;
                        grid.dirty = false;
                    }
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
            Message::KeyPressed(key, modifiers, text) => {
                if self.phase == AnimationPhase::Expanded {
                    let mut seq = String::new();
                    
                    if let Ok(mut grid) = self.terminal_engine.grid.lock() {
                        grid.viewport_offset = 0; // Reset scroll on keypress
                    }
                    
                    if modifiers.control() {
                        match key {
                            keyboard::Key::Character(c) => {
                                if let Some(ch) = c.chars().next() {
                                    let ch_lower = ch.to_ascii_lowercase();
                                    if ch_lower == 'c' {
                                        seq.push('\x03'); // Ctrl+C
                                    } else if ch_lower == 'd' {
                                        seq.push('\x04'); // Ctrl+D
                                    } else if ch_lower == 'l' {
                                        seq.push('\x0c'); // Ctrl+L
                                    } else if ch_lower >= 'a' && ch_lower <= 'z' {
                                        let ctrl_code = (ch_lower as u8) - b'a' + 1;
                                        seq.push(ctrl_code as char);
                                    }
                                }
                            }
                            _ => {}
                        }
                    } else {
                        match key {
                            keyboard::Key::Named(keyboard::key::Named::Enter) => seq.push('\r'),
                            keyboard::Key::Named(keyboard::key::Named::Backspace) => seq.push('\x7f'),
                            keyboard::Key::Named(keyboard::key::Named::ArrowUp) => seq.push_str("\x1b[A"),
                            keyboard::Key::Named(keyboard::key::Named::ArrowDown) => seq.push_str("\x1b[B"),
                            keyboard::Key::Named(keyboard::key::Named::ArrowRight) => seq.push_str("\x1b[C"),
                            keyboard::Key::Named(keyboard::key::Named::ArrowLeft) => seq.push_str("\x1b[D"),
                            keyboard::Key::Named(keyboard::key::Named::Tab) => seq.push('\t'),
                            _ => {
                                if let Some(t) = text {
                                    seq.push_str(&t);
                                }
                            }
                        }
                    }
                    
                    if !seq.is_empty() {
                        self.terminal_engine.send_key(&seq);
                    }
                }
            }
            Message::Scroll(delta) => {
                if self.phase == AnimationPhase::Expanded {
                    if let Ok(mut grid) = self.terminal_engine.grid.lock() {
                        if delta > 0.0 {
                            grid.scroll_up(3); // Scroll 3 lines per tick
                        } else if delta < 0.0 {
                            grid.scroll_down(3);
                        }
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
                
                let quad = self.get_quad();
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
                        if key == keyboard::Key::Named(keyboard::key::Named::F12) {
                            return Some(Message::ToggleTerminal);
                        }
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
    let settings = Settings::default()
        .size(Size::new(1280.0, 720.0))
        .transparent(true)
        .client_decorations(false);
    cosmic::app::run::<App>(settings, ())?;
    Ok(())
}
