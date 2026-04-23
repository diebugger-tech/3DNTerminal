mod effects;
mod terminal;

use std::time::{Instant, Duration};
use std::collections::VecDeque;
use cosmic::{
    app::{Core, Settings, Task},
    iced::{
        Length, Subscription,
        mouse,
        keyboard,
        Event,
        widget::{
            canvas::{self, Cache, Canvas, Frame, Geometry, Path, Stroke},
            mouse_area,
            stack, image,
        },
        Rectangle, Point, Size, Color, Pixels,
    },
    Application, Element, Theme,
    widget::container,
};
use effects::crossfade::CrossfadeManager;
use terminal::TextColor;

// --- Easing Helper ---
fn cubic_bezier(t: f32) -> f32 {
    let p1 = 1.0; // Control Point 1 (y)
    let p2 = 0.3; // Control Point 2 (y)
    // (0.16, 1, 0.3, 1) Näherung
    let t2 = t * t;
    let t3 = t2 * t;
    (1.0 - t3) * 0.0 + 3.0 * (1.0 - t2) * t * p1 + 3.0 * (1.0 - t) * t2 * p2 + t3 * 1.0
}

#[derive(Debug, Clone)]
pub enum Message {
    Tick(Instant),
    ToggleTerminal,
    KeyPressed(keyboard::Key, keyboard::Modifiers, Option<String>),
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
    terminal_lines: VecDeque<(String, Color)>,
    is_new_line: bool,

    // --- Background Effects ---
    effect_engine: EffectEngine,
    bg_handle: Option<image::Handle>,

    // --- Animation State ---
    phase: AnimationPhase,
    progress: f32, // 0.0 to 1.0
    last_update: Instant,
    start_time: Instant,
    
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
                let eased_t = cubic_bezier(t);
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

    fn draw_3d_window(&self, frame: &mut Frame, rect: Rectangle, angle_y: f32) {
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
            if !self.terminal_lines.is_empty() {
                let start_y = p4.y - margin_y; // Beginne ganz unten
                let mut current_y = start_y;
                let top_limit = p1.y + margin_y + (font_size * 3.0); // Platz für Top Zone lassen
                
                for (text, color) in &self.terminal_lines {
                    let mut c = *color;
                    c.a *= text_alpha; // Fade beim Flip anwenden

                    frame.fill_text(canvas::Text {
                        content: text.clone(),
                        position: Point::new(p1.x + margin_x, current_y),
                        color: c,
                        size: Pixels(font_size),
                        ..canvas::Text::default()
                    });

                    current_y -= line_height;
                    if current_y < top_limit {
                        break; // Text-Clipping oben (Middle Zone endet hier)
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
        terminal_engine.spawn_shell(); // Shell nur EINMAL beim Start spawnen

        let app = App {
            core,
            effect_engine: EffectEngine::start(1280, 720),
            terminal_engine,
            terminal_lines: VecDeque::new(),
            is_new_line: true,
            last_update: Instant::now(),
            start_time: Instant::now(),
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

                if self.phase == AnimationPhase::Expanding || self.phase == AnimationPhase::Collapsing {
                    self.progress += dt / 0.6; // Gesamtdauer 600ms
                    if self.progress >= 1.0 {
                        self.progress = 0.0;
                        self.phase = if self.phase == AnimationPhase::Expanding { AnimationPhase::Expanded } else { AnimationPhase::Collapsed };
                    }
                }
                
                // --- UPDATE BACKGROUND EFFECTS ---
                // Der Background-Thread schickt uns fertige Frames. Wir nehmen immer das aktuellste.
                while let Ok(frame_data) = self.effect_engine.receiver.try_recv() {
                    self.bg_handle = Some(image::Handle::from_rgba(
                        1280,
                        720,
                        frame_data
                    ));
                }

                // PTY Polling
                if let Some(rx) = &self.terminal_engine.receiver {
                    while let Ok(msg) = rx.try_recv() {
                        match msg {
                            terminal::PtyMessage::UpdateLine(text, color) => {
                                let iced_color = match color {
                                    TextColor::Red => Color::from_rgb(1.0, 0.3, 0.3),
                                    TextColor::Yellow => Color::from_rgb(1.0, 1.0, 0.3),
                                    TextColor::Cyan => Color::from_rgb(0.3, 1.0, 1.0),
                                    TextColor::Green => Color::from_rgb(0.3, 1.0, 0.3),
                                    TextColor::White => Color::from_rgb(0.9, 0.9, 0.9),
                                };
                                
                                if self.is_new_line || self.terminal_lines.is_empty() {
                                    self.terminal_lines.push_front((text, iced_color));
                                    self.is_new_line = false;
                                } else {
                                    if let Some(front) = self.terminal_lines.front_mut() {
                                        *front = (text, iced_color);
                                    }
                                }
                                
                                if self.terminal_lines.len() > 30 {
                                    self.terminal_lines.pop_back();
                                }
                            }
                            terminal::PtyMessage::FinishLine => {
                                self.is_new_line = true;
                            }
                            terminal::PtyMessage::Closed => {
                                // Eventuell später neu starten oder UI updaten
                            }
                        }
                    }
                }

                self.cache.clear(); // IMMER den Cache leeren, damit Hover & Pulse funktionieren
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

        mouse_area(
            container(stack(layers))
                .width(Length::Fill)
                .height(Length::Fill)
        )
        .on_press(Message::ToggleTerminal)
        .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        let tick = cosmic::iced::time::every(Duration::from_millis(16)).map(Message::Tick);
        
        let keyboard_events = cosmic::iced::event::listen_with(|event, status, _window_id| {
            if status == cosmic::iced::event::Status::Ignored {
                if let Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, text, .. }) = event {
                    return Some(Message::KeyPressed(key, modifiers, text.map(|t| t.to_string())));
                }
            }
            None
        });

        Subscription::batch(vec![tick, keyboard_events])
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
