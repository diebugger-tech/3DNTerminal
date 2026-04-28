use cosmic::iced::{Color, Rectangle, Point, Pixels};
use cosmic::iced::widget::canvas::{Frame, Text};
use crate::ui::two_d::TerminalParams;
use crate::config::Config;
use crate::ui::skill::TerminalSkill;

pub struct PhysicsSkill;

impl TerminalSkill for PhysicsSkill {
    fn id(&self) -> &'static str { "physics" }
    fn label(&self) -> &'static str { "Physics" }
    fn subtitle(&self) -> &'static str { "Gravity / 3D / Static" }
    fn color(&self) -> Color { Color::from_rgba(1.0, 0.6, 0.0, 1.0) }

    fn draw_overlay(&self, frame: &mut Frame, rect: Rectangle, alpha: f32, params: &TerminalParams) {
        let y = rect.y + 80.0;
        frame.fill_text(Text {
            content: "PHYSICS ENGINE STATUS".to_string(),
            position: Point::new(rect.x + 20.0, y),
            color: self.color(),
            size: Pixels(18.0),
            ..Default::default()
        });
        
        frame.fill_text(Text {
            content: format!("Current Mode: {:?}", params.physics_mode),
            position: Point::new(rect.x + 20.0, y + 40.0),
            color: Color::from_rgba(0.8, 0.8, 0.8, alpha),
            size: Pixels(16.0),
            ..Default::default()
        });

        frame.fill_text(Text {
            content: "Toggle modes via sidebar menu for now.".to_string(),
            position: Point::new(rect.x + 20.0, y + 80.0),
            color: Color::from_rgba(0.5, 0.5, 0.5, alpha),
            size: Pixels(14.0),
            ..Default::default()
        });
    }
}
