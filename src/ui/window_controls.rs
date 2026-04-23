use cosmic::iced::{Point, widget::canvas::Frame};
use super::button::Button;
use crate::app::events::Message;

pub struct WindowControls {
    pub minimize_btn: Button,
    pub maximize_btn: Button,
    pub close_btn: Button,
}

impl WindowControls {
    pub fn new() -> Self {
        Self {
            minimize_btn: Button::new(0.0, 0.0, 30.0, 30.0, "−", "Minimize"),
            maximize_btn: Button::new(0.0, 0.0, 30.0, 30.0, "□", "Maximize"),
            close_btn: Button::new(0.0, 0.0, 30.0, 30.0, "×", "Close [Alt+Q]"),
        }
    }

    pub fn update_positions(&mut self, top_right: Point, button_size: f32, spacing: f32) {
        let x_close = top_right.x - button_size;
        let x_max = x_close - button_size - spacing;
        let x_min = x_max - button_size - spacing;
        let y = top_right.y;

        self.close_btn.x = x_close;
        self.close_btn.y = y;
        self.maximize_btn.x = x_max;
        self.maximize_btn.y = y;
        self.minimize_btn.x = x_min;
        self.minimize_btn.y = y;
        
        self.minimize_btn.width = button_size;
        self.minimize_btn.height = button_size;
        self.maximize_btn.width = button_size;
        self.maximize_btn.height = button_size;
        self.close_btn.width = button_size;
        self.close_btn.height = button_size;
    }

    pub fn draw(&self, frame: &mut Frame, alpha: f32) {
        self.minimize_btn.draw(frame, alpha);
        self.maximize_btn.draw(frame, alpha);
        self.close_btn.draw(frame, alpha);
    }

    /// Checks if a normalized point (u, v) hits any button.
    /// Returns the message to dispatch if clicked.
    pub fn update_hover(&mut self, u: f32, v: f32) {
        // Assume header is top 10% (v < 0.1)
        // Buttons are at the right end (u > 0.8)
        let is_header = v < 0.1;
        
        self.minimize_btn.set_hover(is_header && u > 0.85 && u < 0.9);
        self.maximize_btn.set_hover(is_header && u > 0.9 && u < 0.95);
        self.close_btn.set_hover(is_header && u > 0.95);
    }

    pub fn handle_click(&self, u: f32, v: f32) -> Option<Message> {
        let is_header = v < 0.1;
        if is_header {
            if u > 0.95 {
                return Some(Message::CloseApp);
            } else if u > 0.9 && u < 0.95 {
                return Some(Message::MaximizeTerminal);
            } else if u > 0.85 && u < 0.9 {
                return Some(Message::MinimizeTerminal);
            }
        }
        None
    }
}
