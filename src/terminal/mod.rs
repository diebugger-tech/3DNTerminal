pub mod grid;
pub mod input;
pub mod traits;

use portable_pty::{native_pty_system, CommandBuilder, MasterPty, PtySize};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use tokio::task::JoinHandle;
use grid::TerminalGrid;

use crate::config::Config;
use cosmic::iced::keyboard::{Key, Modifiers};
use crate::error::AppError;

/// Manages the background PTY process and VTE parser execution.
/// It spawns a pseudo-terminal, executes a shell, and continuously reads output,
/// which is then parsed by `vte::Parser` and written to the thread-safe `TerminalGrid`.
pub struct TerminalEngine {
    pub cols: u16,
    pub rows: u16,
    pty_master: Option<Box<dyn MasterPty + Send>>,
    pty_writer: Option<Box<dyn Write + Send>>,
    pub grid: Arc<Mutex<TerminalGrid>>,
    _task: Option<JoinHandle<()>>,
}

impl TerminalEngine {
    /// Creates a new `TerminalEngine` instance with the specified dimensions.
    pub fn new(cols: u16, rows: u16) -> Self {
        Self {
            cols,
            rows,
            pty_master: None,
            pty_writer: None,
            grid: Arc::new(Mutex::new(TerminalGrid::new(cols as usize, rows as usize))),
            _task: None,
        }
    }

    /// Spawns the underlying shell process (/bin/bash) and starts the reader thread.
    pub fn spawn_shell(&mut self) -> Result<(), AppError> {
        if self.pty_master.is_some() {
            return Ok(());
        }

        tracing::info!("Spawning shell (/bin/bash) with size {}x{}", self.cols, self.rows);

        let pty_system = native_pty_system();
        let pair = pty_system.openpty(PtySize {
            rows: self.rows,
            cols: self.cols,
            pixel_width: 0,
            pixel_height: 0,
        }).map_err(|e| AppError::Pty(format!("Failed to open PTY: {}", e)))?;

        let cmd = CommandBuilder::new("/bin/bash");

        let mut child = pair.slave.spawn_command(cmd)
            .map_err(|e| AppError::Pty(format!("Failed to spawn shell: {}", e)))?;

        drop(pair.slave);

        let mut reader = pair.master.try_clone_reader()
            .map_err(|e| AppError::Pty(format!("Failed to clone PTY reader: {}", e)))?;

        let master = pair.master;

        self.pty_writer = Some(master.take_writer()
            .map_err(|e| AppError::Pty(format!("Failed to take PTY writer: {}", e)))?);

        self.pty_master = Some(master);

        let grid_clone = Arc::clone(&self.grid);

        let task = tokio::task::spawn_blocking(move || {
            let mut buf = [0u8; 8192]; // Larger buffer for performance
            let mut parser = vte::Parser::new();

            loop {
                match reader.read(&mut buf) {
                    Ok(0) => {
                        tracing::info!("PTY reader reached EOF");
                        break;
                    }
                    Ok(n) => {
                        if let Ok(mut grid) = grid_clone.lock() {
                            parser.advance(&mut *grid, &buf[..n]);
                        }
                    }
                    Err(e) => {
                        tracing::error!("PTY reader error: {}", e);
                        break;
                    }
                }
            }
            let _ = child.wait();
            tracing::info!("Shell process exited");
        });

        self._task = Some(task);
        Ok(())
    }

    /// Kills the running shell by dropping the PTY writer and master handles.
    #[allow(dead_code)]
    pub fn kill_shell(&mut self) {
        self.pty_writer = None;
        self.pty_master = None;
        self._task = None;
    }
}

impl traits::Terminal for TerminalEngine {
    fn send_key(&mut self, key: &Key, modifiers: Modifiers, config: &Config) {
        if let Some(bytes) = input::key_to_ansi(key, modifiers, config) {
            if let Some(writer) = &mut self.pty_writer {
                let _ = writer.write_all(&bytes);
                let _ = writer.flush();
            }
            self.reset_scroll();
        }
    }

    fn reset_scroll(&self) {
        if let Ok(mut g) = self.grid.lock() {
            g.viewport_offset = 0;
            g.dirty = true;
        }
    }

    fn scroll_up(&self, lines: usize) {
        if let Ok(mut g) = self.grid.lock() {
            g.scroll_up(lines);
            g.dirty = true;
        }
    }

    fn scroll_down(&self, lines: usize) {
        if let Ok(mut g) = self.grid.lock() {
            g.scroll_down(lines);
            g.dirty = true;
        }
    }

    fn resize(&mut self, cols: u16, rows: u16) {
        self.cols = cols;
        self.rows = rows;
        if let Some(master) = &mut self.pty_master {
            let _ = master.resize(PtySize {
                rows: self.rows,
                cols: self.cols,
                pixel_width: 0,
                pixel_height: 0,
            });
        }
        if let Ok(mut g) = self.grid.lock() {
            g.resize(self.cols as usize, self.rows as usize);
        }
    }

    fn is_dirty(&self) -> bool {
        if let Ok(mut g) = self.grid.lock() {
            let dirty = g.dirty;
            g.dirty = false;
            dirty
        } else {
            false
        }
    }
}

