use portable_pty::{native_pty_system, CommandBuilder, MasterPty, PtySize};
use std::io::Read;
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
    Output(String, TextColor),
    Closed,
}

pub struct TerminalEngine {
    pub cols: u16,
    pub rows: u16,
    pty_master: Option<Box<dyn MasterPty + Send>>,
    pub receiver: Option<Receiver<PtyMessage>>,
    _task: Option<JoinHandle<()>>,
}

impl TerminalEngine {
    pub fn new(cols: u16, rows: u16) -> Self {
        Self {
            cols,
            rows,
            pty_master: None,
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

        self.pty_master = Some(pair.master);

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
                        
                        // Robustes Filtern von ANSI-Escape-Codes (CSI und OSC)
                        let mut clean_text = String::new();
                        let mut chars = text.chars().peekable();
                        while let Some(c) = chars.next() {
                            if c == '\x1b' {
                                if let Some(&next) = chars.peek() {
                                    if next == '[' {
                                        chars.next(); // Consume '['
                                        // CSI: Consume bis zum Endzeichen (0x40 - 0x7E)
                                        while let Some(cc) = chars.next() {
                                            if cc >= '@' && cc <= '~' {
                                                break;
                                            }
                                        }
                                    } else if next == ']' {
                                        chars.next(); // Consume ']'
                                        // OSC (z.B. Window Title): Consume bis BEL (\x07) oder ST (\x1b\\)
                                        while let Some(cc) = chars.next() {
                                            if cc == '\x07' {
                                                break;
                                            } else if cc == '\x1b' {
                                                if let Some(&next_st) = chars.peek() {
                                                    if next_st == '\\' {
                                                        chars.next();
                                                    }
                                                }
                                                break;
                                            }
                                        }
                                    } else {
                                        chars.next(); // Unbekannte kurze Escape-Sequenz überspringen
                                    }
                                }
                            } else if c != '\r' && c != '\x07' && c != '\x08' {
                                clean_text.push(c);
                            }
                        }

                        // Split by newline and send
                        for c in clean_text.chars() {
                            if c == '\n' {
                                let line = line_buffer.clone();
                                line_buffer.clear();
                                let color = determine_color(&line);
                                let _ = tx.send(PtyMessage::Output(line, color));
                            } else {
                                line_buffer.push(c);
                            }
                        }
                        
                        // Sende unvollständige Zeile (wie Prompts), falls am Ende des Buffers
                        if !line_buffer.is_empty() {
                            let color = determine_color(&line_buffer);
                            // Senden ohne newline zu leeren, wir nehmen an, dass es eine Zeile ist
                            let _ = tx.send(PtyMessage::Output(line_buffer.clone(), color));
                            line_buffer.clear();
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
        // Dropping master PTY schließt die Verbindung, was den Reader unblockt und den Task beendet
        self.pty_master = None;
        self.receiver = None;
        self._task = None;
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
