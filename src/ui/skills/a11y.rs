use cosmic::iced::{Color, Rectangle, Point, Pixels, Size};
use cosmic::iced::widget::canvas::{Frame, Text};
use crate::ui::two_d::TerminalParams;
use crate::ui::skill::TerminalSkill;
use crate::config::{Config, ColorFilter};

pub struct A11ySkill;

impl TerminalSkill for A11ySkill {
    fn id(&self) -> &'static str { "a11y" }
    fn label(&self) -> &'static str { "Accessibility" }
    fn subtitle(&self) -> &'static str { "Filters / Damping / Tremor" }
    fn color(&self) -> Color { Color::from_rgba(0.4, 1.0, 0.8, 1.0) } // Cyan/Teal
    fn icon(&self) -> Option<crate::ui::icons::IconType> { Some(crate::ui::icons::IconType::A11y) }

    fn draw_overlay(&self, frame: &mut Frame, rect: Rectangle, alpha: f32, params: &TerminalParams) {
        let y = rect.y + 80.0;
        frame.fill_text(Text {
            content: "ACCESSIBILITY DASHBOARD".to_string(),
            position: Point::new(rect.x + 20.0, y),
            color: self.color(),
            size: Pixels(18.0),
            ..Default::default()
        });
        
        let start_x = rect.x + 20.0;
        let mut current_y = y + 50.0;

        // 1. Tremor Damping
        self.draw_switch(frame, start_x, current_y, params.a11y.tremor_damping > 0.0, "Tremor Compensation", alpha);
        if params.a11y.tremor_damping > 0.0 {
            self.draw_slider(frame, start_x + 30.0, current_y + 30.0, 200.0, params.a11y.tremor_damping, "Damping Intensity", alpha);
            current_y += 70.0;
        } else {
            current_y += 50.0;
        }

        // 2. Reduce Motion
        self.draw_switch(frame, start_x, current_y, params.a11y.reduce_motion > 0.0, "Reduce Motion", alpha);
        if params.a11y.reduce_motion > 0.0 {
            self.draw_slider(frame, start_x + 30.0, current_y + 30.0, 200.0, params.a11y.reduce_motion, "Animation Damping", alpha);
            current_y += 70.0;
        } else {
            current_y += 50.0;
        }

        // 3. Color Filters
        frame.fill_text(Text {
            content: "VISION FILTERS:".to_string(),
            position: Point::new(start_x, current_y),
            color: Color::from_rgba(0.7, 0.7, 0.7, alpha),
            size: Pixels(14.0),
            ..Default::default()
        });
        current_y += 30.0;

        let filters = [
            (ColorFilter::None, "Standard"),
            (ColorFilter::Protanopia, "Protanopia"),
            (ColorFilter::Deuteranopia, "Deuteranopia"),
            (ColorFilter::Tritanopia, "Tritanopia"),
        ];

        for (filter, name) in filters {
            let is_active = params.a11y.color_filter == filter;
            self.draw_switch(frame, start_x, current_y, is_active, name, alpha);
            current_y += 40.0;
        }
    }

    fn draw_menu_extension(&self, _frame: &mut Frame, _rect: Rectangle, _alpha: f32, _params: &TerminalParams) {
        // Removed as requested: No indicators in the hamburger model
    }

    fn on_menu_click(&self, _pos: Point, _rect: Rectangle, _config: &mut Config) -> bool {
        // Now opens the sub-menu via OverlayAction
        false 
    }

    fn on_click(&self, pos: Point, rect: Rectangle, config: &mut Config) -> bool {
        let start_x = rect.x + 20.0;
        let y = rect.y + 80.0;
        let mut current_y = y + 50.0;

        // Hit-test Switches & Sliders
        // Tremor
        if self.hit_test_switch(pos, start_x, current_y) {
            config.a11y.tremor_damping = if config.a11y.tremor_damping > 0.0 { 0.0 } else { 0.5 };
            return true;
        }
        if config.a11y.tremor_damping > 0.0 {
            if self.hit_test_slider(pos, start_x + 30.0, current_y + 30.0, 200.0) {
                config.a11y.tremor_damping = ((pos.x - (start_x + 30.0)) / 200.0).clamp(0.01, 1.0);
                return true;
            }
            current_y += 70.0;
        } else {
            current_y += 50.0;
        }

        // Motion
        if self.hit_test_switch(pos, start_x, current_y) {
            config.a11y.reduce_motion = if config.a11y.reduce_motion > 0.0 { 0.0 } else { 0.8 };
            return true;
        }
        if config.a11y.reduce_motion > 0.0 {
            if self.hit_test_slider(pos, start_x + 30.0, current_y + 30.0, 200.0) {
                config.a11y.reduce_motion = ((pos.x - (start_x + 30.0)) / 200.0).clamp(0.01, 1.0);
                return true;
            }
            current_y += 70.0;
        } else {
            current_y += 50.0;
        }

        current_y += 30.0; // Filter Heading

        // Filters
        let filters = [ColorFilter::None, ColorFilter::Protanopia, ColorFilter::Deuteranopia, ColorFilter::Tritanopia];
        for filter in filters {
            if self.hit_test_switch(pos, start_x, current_y) {
                config.a11y.color_filter = filter;
                return true;
            }
            current_y += 40.0;
        }

        false
    }
}

impl A11ySkill {
    fn draw_switch(&self, frame: &mut Frame, x: f32, y: f32, active: bool, label: &str, alpha: f32) {
        let width = 40.0;
        let height = 20.0;
        let bg_color = if active { self.color() } else { Color::from_rgba(0.2, 0.2, 0.2, alpha) };
        
        // Track
        frame.fill_rectangle(Point::new(x, y), Size::new(width, height), bg_color);
        
        // Knob
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

    fn draw_slider(&self, frame: &mut Frame, x: f32, y: f32, width: f32, value: f32, label: &str, alpha: f32) {
        frame.fill_text(Text {
            content: format!("{}: {:.1}", label, value),
            position: Point::new(x, y - 5.0),
            color: Color::from_rgba(0.6, 0.6, 0.6, alpha),
            size: Pixels(11.0),
            ..Default::default()
        });

        frame.fill_rectangle(Point::new(x, y + 5.0), Size::new(width, 2.0), Color::from_rgba(0.3, 0.3, 0.3, alpha));
        frame.fill_rectangle(Point::new(x + (width * value) - 3.0, y), Size::new(6.0, 12.0), self.color());
    }

    fn hit_test_slider(&self, pos: Point, x: f32, y: f32, width: f32) -> bool {
        pos.x >= x && pos.x <= x + width && pos.y >= y - 5.0 && pos.y <= y + 15.0
    }
}
