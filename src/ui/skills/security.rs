use cosmic::iced::{Color, Rectangle, Point, Pixels};
use cosmic::iced::widget::canvas::{Frame, Text};
use crate::ui::two_d::TerminalParams;
use crate::ui::skill::TerminalSkill;
use crate::config::Config;

pub struct SecuritySkill;

impl TerminalSkill for SecuritySkill {
    fn id(&self) -> &'static str { "security" }
    fn label(&self) -> &'static str { "Security & AI Firewall" }
    fn subtitle(&self) -> &'static str { "Human-in-the-Loop Protection" }
    fn color(&self) -> Color { Color::from_rgb(1.0, 0.2, 0.2) } // Alarm-Rot
    fn icon(&self) -> Option<crate::ui::icons::IconType> { Some(crate::ui::icons::IconType::Anchor) }

    fn draw_overlay(&self, frame: &mut Frame, rect: Rectangle, alpha: f32, _params: &TerminalParams) {
        let y = rect.y + 80.0;
        frame.fill_text(Text {
            content: "SECURITY MONITOR".to_string(),
            position: Point::new(rect.x + 20.0, y),
            color: self.color(),
            size: Pixels(18.0),
            ..Default::default()
        });

        let start_x = rect.x + 20.0;
        let mut current_y = y + 50.0;

        // Hier werden die Sicherheits-Features visualisiert
        self.draw_status(frame, start_x, current_y, true, "AI-Firewall: ACTIVE", alpha);
        current_y += 40.0;
        self.draw_status(frame, start_x, current_y, true, "Human-in-the-Loop: ENABLED", alpha);
        current_y += 40.0;
        self.draw_status(frame, start_x, current_y, false, "Root-Mode: DISABLED", alpha);

        frame.fill_text(Text {
            content: "All AI-generated commands require manual approval.".to_string(),
            position: Point::new(rect.x + 20.0, current_y + 60.0),
            color: Color::from_rgba(0.7, 0.7, 0.7, alpha),
            size: Pixels(14.0),
            ..Default::default()
        });
    }

    fn on_click(&self, _pos: Point, _rect: Rectangle, _config: &mut Config) -> bool {
        // Zukünftige Toggles für Sicherheitsstufen
        false
    }
}

impl SecuritySkill {
    fn draw_status(&self, frame: &mut Frame, x: f32, y: f32, active: bool, label: &str, alpha: f32) {
        let color = if active { Color::from_rgb(0.0, 1.0, 0.0) } else { Color::from_rgba(0.5, 0.5, 0.5, alpha) };
        frame.fill_rectangle(Point::new(x, y + 4.0), cosmic::iced::Size::new(12.0, 12.0), color);
        
        frame.fill_text(Text {
            content: label.to_string(),
            position: Point::new(x + 25.0, y),
            color: Color::from_rgba(1.0, 1.0, 1.0, alpha),
            size: Pixels(14.0),
            ..Default::default()
        });
    }
}
