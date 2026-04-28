use cosmic::iced::{Color, Rectangle, Point, Pixels, Size};
use cosmic::iced::widget::canvas::{Frame, Text, Path};
use crate::ui::two_d::TerminalParams;
use crate::config::Config;
use crate::ui::skill::TerminalSkill;

pub struct SettingsSkill;

impl TerminalSkill for SettingsSkill {
    fn id(&self) -> &'static str { "settings" }
    fn label(&self) -> &'static str { "Settings" }
    fn subtitle(&self) -> &'static str { "Engine Configuration" }
    fn color(&self) -> Color { Color::from_rgba(1.0, 0.4, 0.0, 1.0) }
    fn icon(&self) -> Option<crate::ui::icons::IconType> { Some(crate::ui::icons::IconType::Settings) }

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

    fn draw_menu_extension(&self, frame: &mut Frame, rect: Rectangle, alpha: f32, params: &TerminalParams) {
        // Draw a small toggle switch for Glow
        let switch_w = 40.0;
        let switch_h = 20.0;
        let switch_rect = Rectangle::new(Point::new(rect.x + (rect.width - switch_w), rect.y + 5.0), Size::new(switch_w, switch_h));
        
        let path = Path::rounded_rectangle(switch_rect.position(), switch_rect.size(), (switch_h / 2.0).into());
        frame.stroke(&path, cosmic::iced::widget::canvas::Stroke::default().with_color(Color::from_rgba(1.0, 1.0, 1.0, 0.2 * alpha)));
        
        if params.glow_active {
            frame.fill(&path, Color::from_rgba(0.0, 1.0, 0.5, 0.3 * alpha));
            frame.fill_rectangle(Point::new(switch_rect.x + switch_w - 18.0, switch_rect.y + 2.0), Size::new(16.0, 16.0), Color::WHITE);
        } else {
            frame.fill(&path, Color::from_rgba(0.5, 0.5, 0.5, 0.1 * alpha));
            frame.fill_rectangle(Point::new(switch_rect.x + 2.0, switch_rect.y + 2.0), Size::new(16.0, 16.0), Color::from_rgba(0.5, 0.5, 0.5, alpha));
        }
    }

    fn on_menu_click(&self, pos: Point, rect: Rectangle, config: &mut Config) -> bool {
        let switch_w = 40.0;
        let switch_h = 20.0;
        let switch_rect = Rectangle::new(Point::new(rect.x + (rect.width - switch_w), rect.y + 5.0), Size::new(switch_w, switch_h));
        
        if switch_rect.contains(pos) {
            config.glow_active = !config.glow_active;
            return true;
        }
        false
    }
}
