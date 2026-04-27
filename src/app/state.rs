use cosmic::iced::{Point, Rectangle, Size};
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnimationPhase {
    Collapsed,
    Expanding,
    Expanded,
    Collapsing,
    Hidden,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum CornerPosition {
    #[default]
    Free,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl CornerPosition {
    pub fn corner_rect(self, window_width: f32, window_height: f32) -> Rectangle {
        let w = 400.0_f32;
        let h = 250.0_f32;
        let m = 20.0_f32;
        let (x, y) = match self {
            CornerPosition::Free        => (window_width * 0.06, window_height * 0.09),
            CornerPosition::TopLeft     => (m, m),
            CornerPosition::TopRight    => (window_width - w - m, m),
            CornerPosition::BottomLeft  => (m, window_height - h - m),
            CornerPosition::BottomRight => (window_width - w - m, window_height - h - m),
        };
        Rectangle::new(Point::new(x, y), Size::new(w, h))
    }
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
                Point::ORIGIN,
                Size::new(width, height)
            ),
        }
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.corner_rect = Rectangle::new(
            Point::new(width - 450.0, height - 300.0),
            Size::new(400.0, 250.0)
        );
        self.center_rect = Rectangle::new(
            Point::ORIGIN,
            Size::new(width, height)
        );
    }
}
