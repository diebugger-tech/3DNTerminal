use cosmic::iced::{Rectangle, Color, Point, Size, Pixels};
use cosmic::iced::widget::canvas::{Frame, Path, Stroke, Text};
use crate::ui::two_d::TerminalParams;
use crate::config::PhysicsMode;

pub fn draw(frame: &mut Frame, rect: Rectangle, alpha: f32, params: &TerminalParams) {
    if !params.settings_open { return; }

    let settings_w = 400.0;
    let settings_h = 300.0;
    let settings_rect = Rectangle {
        x: rect.x + (rect.width - settings_w) / 2.0,
        y: rect.y + (rect.height - settings_h) / 2.0,
        width: settings_w,
        height: settings_h,
    };

    let settings_path = Path::rectangle(Point::new(settings_rect.x, settings_rect.y), Size::new(settings_rect.width, settings_rect.height));
    frame.fill(&settings_path, Color::from_rgba(0.01, 0.01, 0.05, 0.98 * alpha));
    frame.stroke(&settings_path, Stroke::default().with_color(Color::from_rgba(1.0, 0.6, 0.0, 0.8 * alpha)).with_width(2.0));

    frame.fill_text(Text {
        content: "ENGINE SETTINGS".to_string(),
        position: Point::new(settings_rect.x + 20.0, settings_rect.y + 35.0),
        color: Color::from_rgba(1.0, 0.6, 0.0, alpha),
        size: Pixels(22.0),
        ..Default::default()
    });

    // Physics Info
    let physics_y = settings_rect.y + 80.0;
    frame.fill_text(Text {
        content: "Physics Engine".to_string(),
        position: Point::new(settings_rect.x + 20.0, physics_y),
        color: Color::from_rgba(1.0, 0.8, 0.4, alpha),
        size: Pixels(18.0),
        ..Default::default()
    });

    let status_text = match params.physics_mode {
        PhysicsMode::Static => "[ STATIC ]",
        PhysicsMode::Breathe => "[ BREATHE ]",
        PhysicsMode::Hologram3D => "[ 3D HOLOGRAM ]",
    };
    
    frame.fill_text(Text {
        content: status_text.to_string(),
        position: Point::new(settings_rect.x + 20.0, physics_y + 25.0),
        color: match params.physics_mode {
            PhysicsMode::Static => Color::from_rgb(1.0, 0.3, 0.0),
            PhysicsMode::Breathe => Color::from_rgb(0.0, 1.0, 0.8),
            PhysicsMode::Hologram3D => Color::from_rgb(1.0, 0.8, 0.0),
        },
        size: Pixels(16.0),
        ..Default::default()
    });

    frame.fill_text(Text {
        content: "(Cycle via Sidebar -> Physics)".to_string(),
        position: Point::new(settings_rect.x + 20.0, physics_y + 50.0),
        color: Color::from_rgba(1.0, 0.6, 0.0, 0.4 * alpha),
        size: Pixels(12.0),
        ..Default::default()
    });
}
