use cosmic::iced::keyboard::{Key, Modifiers};
use cosmic::iced::Point;
use std::time::Instant;
use crate::app::state::CornerPosition;

#[derive(Debug, Clone)]
pub enum Message {
    Tick(Instant),
    ToggleTerminal,
    KeyPressed(Key, Modifiers, Option<String>),
    Scroll(f32),
    WindowResized(f32, f32),
    CursorMoved(Point),
    MouseClicked(Instant),
    TerminalClosed,
    MinimizeTerminal,
    MaximizeTerminal,
    CloseApp,
    RestoreLast,
    SetCorner(CornerPosition),
    StartDragging(Point),
    DragTo(Point),
    StopDragging,
    RawMouseEvent(cosmic::iced::mouse::Event),
}
