use cosmic::iced::{Color, Rectangle, Point, Pixels, Size};
use cosmic::iced::widget::canvas::{Frame, Text};
use crate::ui::two_d::TerminalParams;
use crate::ui::skill::TerminalSkill;
use crate::config::Config;

pub struct PhysicsSkill;

impl TerminalSkill for PhysicsSkill {
    fn id(&self) -> &'static str { "physics" }
    fn label(&self) -> &'static str { "Physics" }
    fn subtitle(&self) -> &'static str { "Breathe / Magnetic / A11Y" }
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
            content: "ACTIVE MODULES:".to_string(),
            position: Point::new(rect.x + 20.0, y + 40.0),
            color: Color::from_rgba(0.8, 0.8, 0.8, alpha),
            size: Pixels(16.0),
            ..Default::default()
        });

        let status = format!(
            "Breathe: {}\nMagnetic: {}\nA11Y Mode: {}",
            if params.physics.breathe { "ON" } else { "OFF" },
            if params.physics.magnetic { "ON" } else { "OFF" },
            if params.physics.reduce_motion { "ACTIVE" } else { "INACTIVE" }
        );

        frame.fill_text(Text {
            content: status,
            position: Point::new(rect.x + 20.0, y + 70.0),
            color: Color::from_rgba(0.5, 0.5, 0.5, alpha),
            size: Pixels(14.0),
            ..Default::default()
        });

        frame.fill_text(Text {
            content: "Toggle modes via sidebar menu indicator.".to_string(),
            position: Point::new(rect.x + 20.0, y + 140.0),
            color: Color::from_rgba(0.4, 0.4, 0.4, alpha),
            size: Pixels(12.0),
            ..Default::default()
        });
    }

    fn draw_menu_extension(&self, frame: &mut Frame, rect: Rectangle, _alpha: f32, params: &TerminalParams) {
        // Kleiner visueller Indikator im Hamburger Menü
        let color = if params.physics.reduce_motion { 
            Color::from_rgb(1.0, 0.2, 0.2) 
        } else if params.physics.breathe {
            self.color()
        } else {
            Color::from_rgba(0.5, 0.5, 0.5, 0.5)
        };
        
        frame.fill_rectangle(
            Point::new(rect.x + rect.width - 25.0, rect.y + 5.0), 
            Size::new(20.0, 20.0), 
            color
        );
    }

    fn on_menu_click(&self, _pos: Point, _rect: Rectangle, config: &mut Config) -> bool {
        // Toggle durchrotieren beim Klick auf die Extension im Menü
        if config.physics.reduce_motion {
            config.physics.reduce_motion = false;
            config.physics.breathe = true;
            config.physics.magnetic = true;
        } else if config.physics.breathe {
            config.physics.breathe = false;
        } else if config.physics.magnetic {
            config.physics.magnetic = false;
        } else {
            config.physics.reduce_motion = true;
        }
        true
    }
}
