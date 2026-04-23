use std::sync::{Arc, Mutex};

pub struct TerminalEngine {
    pub cols:   u16,
    pub rows:   u16,
    pub output: Arc<Mutex<String>>,
}

impl TerminalEngine {
    pub fn new(cols: u16, rows: u16) -> Self {
        Self {
            cols,
            rows,
            output: Arc::new(Mutex::new(String::new())),
        }
    }

    pub fn resize(&mut self, cols: u16, rows: u16) {
        self.cols = cols;
        self.rows = rows;
    }
}
