use vte::{Params, Perform};
use cosmic::iced::Color;

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub char: char,
    pub fg: Color,
    pub bg: Color,
    pub bold: bool,
    pub italic: bool,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            char: ' ',
            fg: Color::from_rgb(0.9, 0.9, 0.9), // White
            bg: Color::TRANSPARENT,
            bold: false,
            italic: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TerminalGrid {
    pub cells: Vec<Vec<Cell>>,
    pub cols: usize,
    pub rows: usize,
    pub cursor_x: usize,
    pub cursor_y: usize,
    pub default_fg: Color,
    pub default_bg: Color,
    current_fg: Color,
    current_bg: Color,
}

impl TerminalGrid {
    pub fn new(cols: usize, rows: usize) -> Self {
        let default_fg = Color::from_rgb(0.9, 0.9, 0.9);
        let default_bg = Color::TRANSPARENT;
        
        Self {
            cells: vec![vec![Cell::default(); cols]; rows],
            cols,
            rows,
            cursor_x: 0,
            cursor_y: 0,
            default_fg,
            default_bg,
            current_fg: default_fg,
            current_bg: default_bg,
        }
    }

    fn new_line(&mut self) {
        if self.cursor_y < self.rows - 1 {
            self.cursor_y += 1;
        } else {
            // Scroll down: remove first line, add new line at end
            self.cells.remove(0);
            self.cells.push(vec![Cell {
                char: ' ',
                fg: self.default_fg,
                bg: self.default_bg,
                bold: false,
                italic: false,
            }; self.cols]);
        }
    }
}

impl Perform for TerminalGrid {
    fn print(&mut self, c: char) {
        if self.cursor_x >= self.cols {
            self.cursor_x = 0;
            self.new_line();
        }
        
        self.cells[self.cursor_y][self.cursor_x] = Cell {
            char: c,
            fg: self.current_fg,
            bg: self.current_bg,
            bold: false,
            italic: false,
        };
        self.cursor_x += 1;
    }

    fn execute(&mut self, byte: u8) {
        match byte {
            b'\n' | b'\x0B' | b'\x0C' => self.new_line(),
            b'\r' => self.cursor_x = 0,
            b'\x08' => { // Backspace
                if self.cursor_x > 0 {
                    self.cursor_x -= 1;
                }
            }
            _ => {}
        }
    }

    fn hook(&mut self, _params: &Params, _intermediates: &[u8], _ignore: bool, _action: char) {}
    fn put(&mut self, _byte: u8) {}
    fn unhook(&mut self) {}
    fn osc_dispatch(&mut self, _params: &[&[u8]], _bell_terminated: bool) {}

    fn csi_dispatch(&mut self, params: &Params, _intermediates: &[u8], _ignore: bool, action: char) {
        match action {
            'A' => { // Cursor Up
                let n = params.iter().next().map_or(1, |x| x[0] as usize).max(1);
                self.cursor_y = self.cursor_y.saturating_sub(n);
            }
            'B' => { // Cursor Down
                let n = params.iter().next().map_or(1, |x| x[0] as usize).max(1);
                self.cursor_y = (self.cursor_y + n).min(self.rows - 1);
            }
            'C' => { // Cursor Forward
                let n = params.iter().next().map_or(1, |x| x[0] as usize).max(1);
                self.cursor_x = (self.cursor_x + n).min(self.cols - 1);
            }
            'D' => { // Cursor Backward
                let n = params.iter().next().map_or(1, |x| x[0] as usize).max(1);
                self.cursor_x = self.cursor_x.saturating_sub(n);
            }
            'H' | 'f' => { // Cursor Position
                let mut it = params.iter();
                let y = it.next().map_or(1, |x| x[0] as usize).max(1).saturating_sub(1);
                let x = it.next().map_or(1, |x| x[0] as usize).max(1).saturating_sub(1);
                self.cursor_y = y.min(self.rows - 1);
                self.cursor_x = x.min(self.cols - 1);
            }
            'J' => { // Erase in Display
                let mode = params.iter().next().map_or(0, |x| x[0]);
                match mode {
                    0 => { // Below
                        for i in self.cursor_x..self.cols {
                            self.cells[self.cursor_y][i].char = ' ';
                        }
                        for r in (self.cursor_y + 1)..self.rows {
                            for c in 0..self.cols {
                                self.cells[r][c].char = ' ';
                            }
                        }
                    }
                    2 => { // All
                        for r in 0..self.rows {
                            for c in 0..self.cols {
                                self.cells[r][c].char = ' ';
                            }
                        }
                        self.cursor_x = 0;
                        self.cursor_y = 0;
                    }
                    _ => {}
                }
            }
            'K' => { // Erase in Line
                let mode = params.iter().next().map_or(0, |x| x[0]);
                match mode {
                    0 => { // Right
                        for i in self.cursor_x..self.cols {
                            self.cells[self.cursor_y][i].char = ' ';
                        }
                    }
                    1 => { // Left
                        for i in 0..=self.cursor_x {
                            self.cells[self.cursor_y][i].char = ' ';
                        }
                    }
                    2 => { // All
                        for i in 0..self.cols {
                            self.cells[self.cursor_y][i].char = ' ';
                        }
                    }
                    _ => {}
                }
            }
            'm' => { // SGR (Select Graphic Rendition)
                if params.is_empty() {
                    self.current_fg = self.default_fg;
                    self.current_bg = self.default_bg;
                    return;
                }
                for param in params.iter() {
                    match param[0] {
                        0 => {
                            self.current_fg = self.default_fg;
                            self.current_bg = self.default_bg;
                        }
                        31 => self.current_fg = Color::from_rgb(1.0, 0.3, 0.3),
                        32 => self.current_fg = Color::from_rgb(0.3, 1.0, 0.3),
                        33 => self.current_fg = Color::from_rgb(1.0, 1.0, 0.3),
                        34 => self.current_fg = Color::from_rgb(0.3, 0.3, 1.0),
                        36 => self.current_fg = Color::from_rgb(0.3, 1.0, 1.0),
                        37 => self.current_fg = self.default_fg,
                        90 => self.current_fg = Color::from_rgb(0.5, 0.5, 0.5), // Gray
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    fn esc_dispatch(&mut self, _intermediates: &[u8], _ignore: bool, _byte: u8) {}
}
