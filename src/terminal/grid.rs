use vte::{Params, Perform};
use cosmic::iced::Color;

fn color_from_256(id: u8) -> Color {
    match id {
        0..=7 => {
            let r = if id & 1 != 0 { 0.8 } else { 0.0 };
            let g = if id & 2 != 0 { 0.8 } else { 0.0 };
            let b = if id & 4 != 0 { 0.8 } else { 0.0 };
            Color::from_rgb(r, g, b)
        }
        8..=15 => {
            let r = if id & 1 != 0 { 1.0 } else { 0.3 };
            let g = if id & 2 != 0 { 1.0 } else { 0.3 };
            let b = if id & 4 != 0 { 1.0 } else { 0.3 };
            Color::from_rgb(r, g, b)
        }
        16..=231 => {
            let mut val = id - 16;
            let b = val % 6; val /= 6;
            let g = val % 6; val /= 6;
            let r = val % 6;
            let step = |c| if c == 0 { 0.0 } else { (c as f32 * 40.0 + 55.0) / 255.0 };
            Color::from_rgb(step(r), step(g), step(b))
        }
        232..=255 => {
            let gray = ((id - 232) as f32 * 10.0 + 8.0) / 255.0;
            Color::from_rgb(gray, gray, gray)
        }
    }
}

#[allow(dead_code)]
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
    pub current_fg: Color,
    pub current_bg: Color,
    pub scrollback: Vec<Vec<Cell>>,
    pub max_scrollback: usize,
    pub viewport_offset: usize,
    pub dirty: bool,
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
            scrollback: Vec::new(),
            max_scrollback: 1000,
            viewport_offset: 0,
            dirty: true,
        }
    }

    pub fn resize(&mut self, new_cols: usize, new_rows: usize) {
        if new_cols == self.cols && new_rows == self.rows {
            return;
        }
        
        let empty_cell = Cell { char: ' ', fg: self.default_fg, bg: self.default_bg, bold: false, italic: false };
        
        if new_rows > self.rows {
            for _ in self.rows..new_rows {
                self.cells.push(vec![empty_cell; new_cols]);
            }
        } else if new_rows < self.rows {
            self.cells.truncate(new_rows);
            if self.cursor_y >= new_rows {
                self.cursor_y = new_rows.saturating_sub(1);
            }
        }
        
        for row in self.cells.iter_mut() {
            row.resize(new_cols, empty_cell);
        }
        
        self.cols = new_cols;
        self.rows = new_rows;
        if self.cursor_x >= new_cols {
            self.cursor_x = new_cols.saturating_sub(1);
        }
        self.dirty = true;
    }

    pub fn get_visible_row(&self, y: usize) -> Option<&Vec<Cell>> {
        let total_lines = self.scrollback.len() + self.rows;
        let start_index = total_lines.saturating_sub(self.rows + self.viewport_offset);
        let abs_y = start_index + y;

        if abs_y < self.scrollback.len() {
            self.scrollback.get(abs_y)
        } else {
            self.cells.get(abs_y.saturating_sub(self.scrollback.len()))
        }
    }

    pub fn scroll_up(&mut self, lines: usize) {
        let old_offset = self.viewport_offset;
        self.viewport_offset = (self.viewport_offset + lines).min(self.scrollback.len());
        if old_offset != self.viewport_offset { self.dirty = true; }
    }

    pub fn scroll_down(&mut self, lines: usize) {
        let old_offset = self.viewport_offset;
        self.viewport_offset = self.viewport_offset.saturating_sub(lines);
        if old_offset != self.viewport_offset { self.dirty = true; }
    }

    fn new_line(&mut self) {
        if self.cursor_y < self.rows - 1 {
            self.cursor_y += 1;
        } else {
            // Scroll down: push first line to scrollback, add new line at end
            let old_line = self.cells.remove(0);
            self.scrollback.push(old_line);
            if self.scrollback.len() > self.max_scrollback {
                self.scrollback.remove(0);
            }
            self.cells.push(vec![Cell {
                char: ' ',
                fg: self.default_fg,
                bg: self.default_bg,
                bold: false,
                italic: false,
            }; self.cols]);
        }
        self.dirty = true;
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
        self.dirty = true;
    }

    fn execute(&mut self, byte: u8) {
        match byte {
            b'\n' | b'\x0B' | b'\x0C' => { self.new_line(); self.dirty = true; }
            b'\r' => { self.cursor_x = 0; self.dirty = true; }
            b'\x08' => { // Backspace
                if self.cursor_x > 0 {
                    self.cursor_x -= 1;
                    self.dirty = true;
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
        self.dirty = true; // Any CSI implies dirty
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
                            self.cells[self.cursor_y][i] = Cell { char: ' ', fg: self.current_fg, bg: self.current_bg, ..Cell::default() };
                        }
                        for r in (self.cursor_y + 1)..self.rows {
                            for c in 0..self.cols {
                                self.cells[r][c] = Cell { char: ' ', fg: self.current_fg, bg: self.current_bg, ..Cell::default() };
                            }
                        }
                    }
                    2 => { // All
                        for r in 0..self.rows {
                            for c in 0..self.cols {
                                self.cells[r][c] = Cell { char: ' ', fg: self.current_fg, bg: self.current_bg, ..Cell::default() };
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
                            self.cells[self.cursor_y][i] = Cell { char: ' ', fg: self.current_fg, bg: self.current_bg, ..Cell::default() };
                        }
                    }
                    1 => { // Left
                        for i in 0..=self.cursor_x {
                            self.cells[self.cursor_y][i] = Cell { char: ' ', fg: self.current_fg, bg: self.current_bg, ..Cell::default() };
                        }
                    }
                    2 => { // All
                        for i in 0..self.cols {
                            self.cells[self.cursor_y][i] = Cell { char: ' ', fg: self.current_fg, bg: self.current_bg, ..Cell::default() };
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
                
                let mut it = params.iter().flat_map(|p| p.iter().copied());
                while let Some(param) = it.next() {
                    match param {
                        0 => {
                            self.current_fg = self.default_fg;
                            self.current_bg = self.default_bg;
                        }
                        38 => {
                            if let Some(format) = it.next() {
                                if format == 2 {
                                    let r = it.next().unwrap_or(0);
                                    let g = it.next().unwrap_or(0);
                                    let b = it.next().unwrap_or(0);
                                    self.current_fg = Color::from_rgb(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);
                                } else if format == 5 {
                                    let id = it.next().unwrap_or(0);
                                    self.current_fg = color_from_256(id as u8);
                                }
                            }
                        }
                        48 => {
                            if let Some(format) = it.next() {
                                if format == 2 {
                                    let r = it.next().unwrap_or(0);
                                    let g = it.next().unwrap_or(0);
                                    let b = it.next().unwrap_or(0);
                                    self.current_bg = Color::from_rgb(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);
                                } else if format == 5 {
                                    let id = it.next().unwrap_or(0);
                                    self.current_bg = color_from_256(id as u8);
                                }
                            }
                        }
                        31 => self.current_fg = Color::from_rgb(1.0, 0.3, 0.3),
                        32 => self.current_fg = Color::from_rgb(0.3, 1.0, 0.3),
                        33 => self.current_fg = Color::from_rgb(1.0, 1.0, 0.3),
                        34 => self.current_fg = Color::from_rgb(0.3, 0.3, 1.0),
                        36 => self.current_fg = Color::from_rgb(0.3, 1.0, 1.0),
                        37 => self.current_fg = self.default_fg,
                        39 => self.current_fg = self.default_fg,
                        
                        41 => self.current_bg = Color::from_rgb(1.0, 0.3, 0.3),
                        42 => self.current_bg = Color::from_rgb(0.3, 1.0, 0.3),
                        43 => self.current_bg = Color::from_rgb(1.0, 1.0, 0.3),
                        44 => self.current_bg = Color::from_rgb(0.3, 0.3, 1.0),
                        46 => self.current_bg = Color::from_rgb(0.3, 1.0, 1.0),
                        47 => self.current_bg = self.default_fg,
                        49 => self.current_bg = self.default_bg,
                        
                        90 => self.current_fg = Color::from_rgb(0.5, 0.5, 0.5), // Gray
                        _ => {}
                    }
                }
                self.dirty = true;
            }
            _ => {}
        }
    }
    fn esc_dispatch(&mut self, _intermediates: &[u8], _ignore: bool, _byte: u8) {}
}
