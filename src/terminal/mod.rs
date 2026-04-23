use portable_pty::{native_pty_system, CommandBuilder, MasterPty, PtySize};
use std::io::{Read, Write};
use std::sync::mpsc::{self, Receiver};
use tokio::task::JoinHandle;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextColor {
    Red,
    Yellow,
    Cyan,
    Green,
    White,
}

#[derive(Debug, Clone)]
pub enum PtyMessage {
    UpdateLine(String, TextColor),
    FinishLine,
    Closed,
}

pub struct TerminalEngine {
    pub cols: u16,
    pub rows: u16,
    pty_master: Option<Box<dyn MasterPty + Send>>,
    pty_writer: Option<Box<dyn Write + Send>>,
    pub receiver: Option<Receiver<PtyMessage>>,
    _task: Option<JoinHandle<()>>,
}

impl TerminalEngine {
    pub fn new(cols: u16, rows: u16) -> Self {
        Self {
            cols,
            rows,
            pty_master: None,
            pty_writer: None,
            receiver: None,
            _task: None,
        }
    }

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
    }

    pub fn spawn_shell(&mut self) {
        if self.pty_master.is_some() {
            return;
        }

        let pty_system = native_pty_system();
        let pair = match pty_system.openpty(PtySize {
            rows: self.rows,
            cols: self.cols,
            pixel_width: 0,
            pixel_height: 0,
        }) {
            Ok(p) => p,
            Err(_) => return,
        };

        let cmd = CommandBuilder::new("/bin/bash");

        let mut child = match pair.slave.spawn_command(cmd) {
            Ok(c) => c,
            Err(_) => return,
        };

        drop(pair.slave);

        let mut reader = match pair.master.try_clone_reader() {
            Ok(r) => r,
            Err(_) => return,
        };

        let master = pair.master;

        self.pty_writer = match master.take_writer() {
            Ok(w) => Some(w),
            Err(_) => return,
        };

        self.pty_master = Some(master);

        let (tx, rx) = mpsc::channel();
        self.receiver = Some(rx);

        let task = tokio::task::spawn_blocking(move || {
            let mut buf = [0u8; 1024];
            let mut line_buffer = String::new();

            loop {
                match reader.read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => {
                        let text = String::from_utf8_lossy(&buf[..n]);
                        
                        let mut chars = text.chars().peekable();
                        let mut updated = false;

                        while let Some(c) = chars.next() {
                            if c == '\x1b' {
                                if let Some(&next) = chars.peek() {
                                    if next == '[' {
                                        chars.next(); // Consume '['
                                        while let Some(cc) = chars.next() {
                                            if cc >= '@' && cc <= '~' {
                                                // \x1b[K = Clear to end of line
                                                if cc == 'K' {
                                                    line_buffer.clear();
                                                    updated = true;
                                                }
                                                break;
                                            }
                                        }
                                    } else if next == ']' {
                                        chars.next(); // Consume ']'
                                        while let Some(cc) = chars.next() {
                                            if cc == '\x07' { break; }
                                            if cc == '\x1b' {
                                                if let Some(&next_st) = chars.peek() {
                                                    if next_st == '\\' { chars.next(); }
                                                }
                                                break;
                                            }
                                        }
                                    } else {
                                        chars.next(); 
                                    }
                                }
                            } else if c == '\n' {
                                let _ = tx.send(PtyMessage::FinishLine);
                                line_buffer.clear();
                                updated = false;
                            } else if c == '\r' {
                                if let Some(&'\n') = chars.peek() {
                                    // ignore, \n handles it
                                } else {
                                    line_buffer.clear();
                                    updated = true;
                                }
                            } else if c == '\x08' || c == '\x7f' {
                                line_buffer.pop();
                                updated = true;
                            } else if c != '\x07' {
                                line_buffer.push(c);
                                updated = true;
                            }
                        }

                        if updated {
                            let color = determine_color(&line_buffer);
                            let _ = tx.send(PtyMessage::UpdateLine(line_buffer.clone(), color));
                        }
                    }
                    Err(_) => break,
                }
            }
            let _ = child.wait();
            let _ = tx.send(PtyMessage::Closed);
        });

        self._task = Some(task);
    }

    pub fn kill_shell(&mut self) {
        self.pty_writer = None;
        self.pty_master = None;
        self.receiver = None;
        self._task = None;
    }

    pub fn send_key(&mut self, key_str: &str) {
        if let Some(writer) = &mut self.pty_writer {
            let _ = writer.write_all(key_str.as_bytes());
            let _ = writer.flush();
        }
    }
}

fn determine_color(line: &str) -> TextColor {
    let lower = line.to_lowercase();
    if lower.contains("error") {
        TextColor::Red
    } else if lower.contains("warning") {
        TextColor::Yellow
    } else if lower.contains("done") || lower.contains("✓") {
        TextColor::Cyan
    } else if line.trim_start().starts_with('>') || line.trim_start().starts_with('$') {
        TextColor::Green
    } else {
        TextColor::White
    }
}
