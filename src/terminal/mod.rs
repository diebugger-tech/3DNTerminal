pub mod grid;

use portable_pty::{native_pty_system, CommandBuilder, MasterPty, PtySize};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use tokio::task::JoinHandle;
use grid::TerminalGrid;

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

    /// Resizes the PTY process to the given columns and rows.
    pub fn resize(&mut self, cols: u16, rows: u16) {
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
        // Resize grid will be done in Phase 2
    }

    /// Spawns the underlying shell process (/bin/bash) and starts the reader thread.
    pub fn spawn_shell(&mut self) -> Result<(), String> {
        if self.pty_master.is_some() {
            return Ok(());
        }

        let pty_system = native_pty_system();
        let pair = pty_system.openpty(PtySize {
            rows: self.rows,
            cols: self.cols,
            pixel_width: 0,
            pixel_height: 0,
        }).map_err(|e| format!("Failed to open PTY: {}", e))?;

        let cmd = CommandBuilder::new("/bin/bash");

        let mut child = pair.slave.spawn_command(cmd)
            .map_err(|e| format!("Failed to spawn shell: {}", e))?;

        drop(pair.slave);

        let mut reader = pair.master.try_clone_reader()
            .map_err(|e| format!("Failed to clone PTY reader: {}", e))?;

        let master = pair.master;

        self.pty_writer = Some(master.take_writer()
            .map_err(|e| format!("Failed to take PTY writer: {}", e))?);

        self.pty_master = Some(master);

        let grid_clone = Arc::clone(&self.grid);

        let task = tokio::task::spawn_blocking(move || {
            let mut buf = [0u8; 8192]; // Larger buffer for performance
            let mut parser = vte::Parser::new();

            loop {
                match reader.read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => {
                        let mut grid = grid_clone.lock().unwrap();
                        parser.advance(&mut *grid, &buf[..n]);
                        // Lock is dropped here, allowing GUI to render
                    }
                    Err(_) => break,
                }
            }
            let _ = child.wait();
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

    pub fn send_key(&mut self, key_str: &str) {
        if let Some(writer) = &mut self.pty_writer {
            let _ = writer.write_all(key_str.as_bytes());
            let _ = writer.flush();
        }
    }
}

