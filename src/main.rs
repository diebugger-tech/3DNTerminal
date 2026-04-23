mod effects;
mod terminal;

use std::time::{Instant, Duration};
use cosmic::{
    app::{Core, Settings, Task},
    iced::{
        Length, Subscription,
        mouse,
        widget::{
            canvas::{self, Cache, Canvas, Frame, Geometry, Path, Stroke},
            mouse_area,
        },
        Rectangle, Point, Size, Color, Pixels,
    },
    Application, Element, Theme,
    widget::container,
};
use effects::crossfade::CrossfadeManager;

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
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnimationPhase {
    Collapsed,
    Expanding,
    Expanded,
    Collapsing,
}

pub struct App {
    core: Core,
    crossfade: CrossfadeManager,
    cache: Cache,
    
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
                let hover = (time * 2.0).sin() * 5.0;
                let mut rect = self.corner_rect;
                rect.y += hover;
                (rect, -18.0, 0.0) // Komplett transparent in der Ecke
            }
            AnimationPhase::Expanded => (self.center_rect, 0.0, 1.0),
            AnimationPhase::Expanding | AnimationPhase::Collapsing => {
                let t = if self.phase == AnimationPhase::Expanding { self.progress } else { 1.0 - self.progress };
                let eased_t = cubic_bezier(t);
                let alpha = eased_t; // Interpoliert von 0.0 zu 1.0
                
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
        
        // Simuliere die Breite durch cos(angle)
        let w = rect.width * cos_a;
        let h = rect.height;
        
        // Perspektivische Verzerrung (Trapez)
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

        // Fenster-Body (Holographisches Blau)
        frame.fill(&path, Color::from_rgba(0.05, 0.1, 0.2, 0.8));
        
        // Glow-Effekt (Holographischer Rand / Box-Shadow Simulation)
        for i in 1..=4 {
            let glow_width = i as f32 * 5.0;
            let glow_alpha = 0.25 / i as f32;
            frame.stroke(&path, Stroke::default()
                .with_color(Color::from_rgba(0.4, 1.0, 0.8, glow_alpha))
                .with_width(glow_width));
        }

        // Innerer scharfer Rand
        frame.stroke(&path, Stroke::default()
            .with_color(Color::from_rgb(0.4, 1.0, 0.8))
            .with_width(1.5));
            
        // "Interface" Text Simulation
        if cos_a > 0.1 {
            frame.fill_text(canvas::Text {
                content: "SYSTEM: NEURAL_LINK ACTIVE".to_string(),
                position: Point::new(p1.x + 20.0, p1.y + 30.0),
                color: Color::from_rgb(0.4, 1.0, 0.8),
                size: Pixels(14.0),
                ..canvas::Text::default()
            });
            
            // Kleine Status-Box oben rechts
            let box_w = 120.0 * cos_a;
            frame.fill_rectangle(
                Point::new(p2.x - box_w - 10.0, p2.y + 10.0),
                Size::new(box_w, 20.0),
                Color::from_rgba(0.4, 1.0, 0.8, 0.2)
            );
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
        let app = App {
            core,
            crossfade: CrossfadeManager::new(1280, 720),
            last_update: Instant::now(),
            start_time: Instant::now(),
            cache: Cache::new(),
            phase: AnimationPhase::Expanded,
            progress: 1.0,
            // Standard Positionen
            corner_rect: Rectangle::new(Point::new(1280.0 - 680.0, 720.0 - 460.0), Size::new(640.0, 420.0)),
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
                self.cache.clear(); // IMMER den Cache leeren, damit Hover & Pulse funktionieren
            }
            Message::ToggleTerminal => {
                if self.phase == AnimationPhase::Collapsed {
                    self.phase = AnimationPhase::Expanding;
                    self.progress = 0.0;
                } else if self.phase == AnimationPhase::Expanded {
                    self.phase = AnimationPhase::Collapsing;
                    self.progress = 0.0;
                }
                self.cache.clear();
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        let canvas = Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill);

        mouse_area(
            container(canvas)
                .width(Length::Fill)
                .height(Length::Fill)
        )
        .on_press(Message::ToggleTerminal)
        .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        cosmic::iced::time::every(Duration::from_millis(16)).map(Message::Tick)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::default()
        .size(Size::new(1280.0, 720.0));
    cosmic::app::run::<App>(settings, ())?;
    Ok(())
}
