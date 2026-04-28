use cosmic::iced::{Color, Rectangle, Point, Pixels};
use cosmic::iced::widget::canvas::{Frame, Text};
use crate::ui::two_d::TerminalParams;
use crate::config::Config;
use crate::ui::skill::TerminalSkill;

pub struct SettingsSkill;

impl TerminalSkill for SettingsSkill {
    fn id(&self) -> &'static str { "settings" }
    fn label(&self) -> &'static str { "Settings" }
    fn subtitle(&self) -> &'static str { "Engine Configuration" }
    fn color(&self) -> Color { Color::from_rgba(1.0, 0.4, 0.0, 1.0) }

    fn draw_overlay(&self, frame: &mut Frame, rect: Rectangle, alpha: f32, _params: &TerminalParams) {
        let y = rect.y + 80.0;
        frame.fill_text(Text {
            content: "ENGINE CONFIGURATION".to_string(),
            position: Point::new(rect.x + 20.0, y),
            color: self.color(),
            size: Pixels(18.0),
            ..Default::default()
        });
        
        frame.fill_text(Text {
            content: "System v0.1.0-alpha".to_string(),
            position: Point::new(rect.x + 20.0, y + 40.0),
            color: Color::from_rgba(0.8, 0.8, 0.8, alpha),
            size: Pixels(14.0),
            ..Default::default()
        });

        frame.fill_text(Text {
            content: "Hologram Renderer: Active".to_string(),
            position: Point::new(rect.x + 20.0, y + 70.0),
            color: Color::from_rgba(0.4, 0.8, 0.4, alpha),
            size: Pixels(14.0),
            ..Default::default()
        });
    }
}
