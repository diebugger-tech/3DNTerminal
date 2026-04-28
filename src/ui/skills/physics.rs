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
            content: "PHYSICS ENGINE DASHBOARD".to_string(),
            position: Point::new(rect.x + 20.0, y),
            color: self.color(),
            size: Pixels(18.0),
            ..Default::default()
        });
        
        let start_x = rect.x + 20.0;
        let mut current_y = y + 50.0;

        // 1. Breathe Effect
        self.draw_switch(frame, start_x, current_y, params.physics.breathe, "Breathe (Sinus-Hover)", alpha);
        current_y += 50.0;

        // 2. Magnetic Cursor
        self.draw_switch(frame, start_x, current_y, params.physics.magnetic, "Magnetic Cursor", alpha);
        current_y += 50.0;

        // 3. Master Reduce Motion (A11Y-Link)
        let is_static = params.physics.reduce_motion || params.a11y.reduce_motion > 0.8;
        self.draw_switch(frame, start_x, current_y, is_static, "Static Mode (No Motion)", alpha);
    }

    fn draw_menu_extension(&self, _frame: &mut Frame, _rect: Rectangle, _alpha: f32, _params: &TerminalParams) {
        // Removed as requested: No indicators in the hamburger model
    }

    fn on_menu_click(&self, _pos: Point, _rect: Rectangle, _config: &mut Config) -> bool {
        false 
    }

    fn on_click(&self, pos: Point, rect: Rectangle, config: &mut Config) -> bool {
        let start_x = rect.x + 20.0;
        let y = rect.y + 80.0;
        let mut current_y = y + 50.0;

        // Breathe
        if self.hit_test_switch(pos, start_x, current_y) {
            config.physics.breathe = !config.physics.breathe;
            return true;
        }
        current_y += 50.0;

        // Magnetic
        if self.hit_test_switch(pos, start_x, current_y) {
            config.physics.magnetic = !config.physics.magnetic;
            return true;
        }
        current_y += 50.0;

        // Static Mode
        if self.hit_test_switch(pos, start_x, current_y) {
            config.physics.reduce_motion = !config.physics.reduce_motion;
            return true;
        }

        false
    }
}

impl PhysicsSkill {
    fn draw_switch(&self, frame: &mut Frame, x: f32, y: f32, active: bool, label: &str, alpha: f32) {
        let width = 40.0;
        let height = 20.0;
        let bg_color = if active { self.color() } else { Color::from_rgba(0.2, 0.2, 0.2, alpha) };
        
        frame.fill_rectangle(Point::new(x, y), Size::new(width, height), bg_color);
        let knob_x = if active { x + width - 18.0 } else { x + 2.0 };
        frame.fill_rectangle(Point::new(knob_x, y + 2.0), Size::new(16.0, 16.0), Color::WHITE);

        frame.fill_text(Text {
            content: label.to_string(),
            position: Point::new(x + width + 10.0, y + 2.0),
            color: if active { Color::WHITE } else { Color::from_rgba(0.7, 0.7, 0.7, alpha) },
            size: Pixels(14.0),
            ..Default::default()
        });
    }

    fn hit_test_switch(&self, pos: Point, x: f32, y: f32) -> bool {
        pos.x >= x && pos.x <= x + 200.0 && pos.y >= y && pos.y <= y + 20.0
    }
}
