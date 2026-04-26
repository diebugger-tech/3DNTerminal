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
            fg: Color::from_rgb(0.9, 0.9, 0.9),
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
        
        let mut grid = Self {
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
        };
        
        grid.welcome_message();
        grid
    }

    fn welcome_message(&mut self) {
        let msg = "3DNTerminal - Frontend Mode Active";
        for (i, c) in msg.chars().enumerate() {
            if i < self.cols {
                self.cells[0][i].char = c;
                self.cells[0][i].fg = Color::from_rgb(0.4, 1.0, 0.8);
            }
        }
        let sub = "Shell backend removed.";
        for (i, c) in sub.chars().enumerate() {
            if i < self.cols {
                self.cells[1][i].char = c;
                self.cells[1][i].fg = Color::from_rgb(0.6, 0.6, 0.6);
            }
        }
    }

    pub fn resize(&mut self, new_cols: usize, new_rows: usize) {
        if new_cols == self.cols && new_rows == self.rows {
            return;
        }
        let empty_cell = Cell::default();
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
        self.viewport_offset = (self.viewport_offset + lines).min(self.scrollback.len());
        self.dirty = true;
    }

    pub fn scroll_down(&mut self, lines: usize) {
        self.viewport_offset = self.viewport_offset.saturating_sub(lines);
        self.dirty = true;
    }
}
