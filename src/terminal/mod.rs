pub mod grid;
pub mod traits;

use std::sync::{Arc, Mutex};
use grid::TerminalGrid;
use crate::config::Config;
use cosmic::iced::keyboard::{Key, Modifiers};

pub struct TerminalEngine {
    pub cols: u16,
    pub rows: u16,
    pub grid: Arc<Mutex<TerminalGrid>>,
}

impl TerminalEngine {
    pub fn new(cols: u16, rows: u16) -> Self {
        Self {
            cols,
            rows,
            grid: Arc::new(Mutex::new(TerminalGrid::new(cols as usize, rows as usize))),
        }
    }

    pub fn spawn_shell(&mut self) -> Result<(), crate::error::AppError> {
        // No-op: Backend removed
        tracing::info!("TerminalEngine: Backend disabled (Frontend Only Mode)");
        Ok(())
    }
}

impl traits::Terminal for TerminalEngine {
    fn send_key(&mut self, _key: &Key, _modifiers: Modifiers, _config: &Config) {}
    fn resize(&mut self, cols: u16, rows: u16) {
        self.cols = cols;
        self.rows = rows;
        if let Ok(mut g) = self.grid.lock() {
            g.resize(cols as usize, rows as usize);
        }
    }
    fn is_dirty(&self) -> bool {
        if let Ok(mut g) = self.grid.lock() {
            let d = g.dirty;
            g.dirty = false;
            d
        } else {
            false
        }
    }
    fn scroll_up(&self, lines: usize) {
        if let Ok(mut g) = self.grid.lock() {
            g.scroll_up(lines);
        }
    }
    fn scroll_down(&self, lines: usize) {
        if let Ok(mut g) = self.grid.lock() {
            g.scroll_down(lines);
        }
    }
}
