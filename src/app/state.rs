use cosmic::iced::{Point, Rectangle, Size};
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnimationPhase {
    Collapsed,
    Expanding,
    Expanded,
    Collapsing,
}

pub struct AppState {
    pub phase: AnimationPhase,
    pub progress: f32,
    pub cursor_visible: bool,
    pub start_time: Instant,
    pub corner_rect: Rectangle,
    pub center_rect: Rectangle,
}

impl AppState {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            phase: AnimationPhase::Expanded,
            progress: 1.0,
            cursor_visible: true,
            start_time: Instant::now(),
            corner_rect: Rectangle::new(
                Point::new(width - 450.0, height - 300.0),
                Size::new(400.0, 250.0)
            ),
            center_rect: Rectangle::new(
                Point::new(width * 0.06, height * 0.09),
                Size::new(width * 0.88, height * 0.82)
            ),
        }
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.corner_rect = Rectangle::new(
            Point::new(width - 450.0, height - 300.0),
            Size::new(400.0, 250.0)
        );
        self.center_rect = Rectangle::new(
            Point::new(width * 0.06, height * 0.09),
            Size::new(width * 0.88, height * 0.82)
        );
    }
}
