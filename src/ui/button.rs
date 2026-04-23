use cosmic::iced::{Point, Color, widget::canvas::{Frame, Path, Stroke, Text}, Pixels, Size};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ButtonState {
    Normal,
    Hover,
    Pressed,
    Disabled,
}

pub struct Button {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub text: String,
    pub tooltip: String,
    pub state: ButtonState,
}

impl Button {
    pub fn new(x: f32, y: f32, width: f32, height: f32, text: &str, tooltip: &str) -> Self {
        Self {
            x,
            y,
            width,
            height,
            text: text.to_string(),
            tooltip: tooltip.to_string(),
            state: ButtonState::Normal,
        }
    }

    pub fn is_clicked(&self, cursor_x: f32, cursor_y: f32) -> bool {
        cursor_x >= self.x && cursor_x <= self.x + self.width &&
        cursor_y >= self.y && cursor_y <= self.y + self.height
    }

    pub fn set_hover(&mut self, is_hover: bool) {
        if self.state != ButtonState::Disabled && self.state != ButtonState::Pressed {
            self.state = if is_hover { ButtonState::Hover } else { ButtonState::Normal };
        }
    }

    pub fn draw(&self, frame: &mut Frame, alpha: f32) {
        if alpha <= 0.0 { return; }

        let color = match self.state {
            ButtonState::Normal => Color::from_rgba(0.4, 1.0, 0.8, 0.6 * alpha),
            ButtonState::Hover => Color::from_rgba(0.6, 1.0, 0.9, 1.0 * alpha),
            ButtonState::Pressed => Color::from_rgba(1.0, 1.0, 1.0, 1.0 * alpha),
            ButtonState::Disabled => Color::from_rgba(0.4, 1.0, 0.8, 0.2 * alpha),
        };

        // Draw background
        if self.state == ButtonState::Hover || self.state == ButtonState::Pressed {
            frame.fill_rectangle(
                Point::new(self.x, self.y),
                Size::new(self.width, self.height),
                Color::from_rgba(0.4, 1.0, 0.8, 0.2 * alpha)
            );
        }

        // Draw border
        let path = Path::rectangle(Point::new(self.x, self.y), Size::new(self.width, self.height));
        frame.stroke(&path, Stroke::default().with_color(color).with_width(1.0));

        // Draw text (icon)
        let font_size = self.height * 0.7;
        frame.fill_text(Text {
            content: self.text.clone(),
            position: Point::new(self.x + self.width / 2.0, self.y + self.height / 2.0),
            color,
            size: Pixels(font_size),
            horizontal_alignment: cosmic::iced::alignment::Horizontal::Center,
            vertical_alignment: cosmic::iced::alignment::Vertical::Center,
            ..Default::default()
        });

        // Draw tooltip if hovered
        if self.state == ButtonState::Hover {
            frame.fill_text(Text {
                content: self.tooltip.clone(),
                position: Point::new(self.x, self.y - 15.0),
                color: Color::from_rgba(0.4, 1.0, 0.8, 0.8 * alpha),
                size: Pixels(12.0),
                ..Default::default()
            });
        }
    }
}
