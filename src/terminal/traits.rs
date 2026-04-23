use crate::config::Config;
use cosmic::iced::keyboard::{Key, Modifiers};

/// Abstract interface for terminal backends.
/// Allows mocking the terminal for tests or swapping the PTY implementation.
pub trait Terminal: Send {
    /// Send a keypress to the underlying terminal process.
    fn send_key(&mut self, key: &Key, modifiers: Modifiers, config: &Config);
    
    /// Scroll the viewport up by `lines`.
    fn scroll_up(&self, lines: usize);
    
    /// Scroll the viewport down by `lines`.
    fn scroll_down(&self, lines: usize);
    
    /// Reset the viewport scroll to the bottom.
    fn reset_scroll(&self);
    
    /// Resize the terminal grid and the underlying process.
    fn resize(&mut self, cols: u16, rows: u16);
    
    /// Returns true if the grid content has changed since the last frame.
    fn is_dirty(&self) -> bool;
}
