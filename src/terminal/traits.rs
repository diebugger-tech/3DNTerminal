use cosmic::iced::keyboard::{Key, Modifiers};
use crate::config::Config;

pub trait Terminal {
    fn send_key(&mut self, key: &Key, modifiers: Modifiers, config: &Config);
    fn resize(&mut self, cols: u16, rows: u16);
    fn is_dirty(&self) -> bool;
    fn scroll_up(&self, lines: usize);
    fn scroll_down(&self, lines: usize);
    fn reset_scroll(&self) {}
}
