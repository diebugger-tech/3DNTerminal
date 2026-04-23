use cosmic::iced::keyboard::{Key, Modifiers, key::Named};
use crate::config::Config;

/// Converts a cosmic::iced keyboard Key and Modifiers into VT100/ANSI byte sequences.
pub fn key_to_ansi(key: &Key, modifiers: Modifiers, _config: &Config) -> Option<Vec<u8>> {
    let mut bytes = Vec::new();

    if modifiers.control() {
        if let Key::Character(c) = key {
            let ch = c.chars().next().unwrap_or('\0').to_ascii_lowercase();
            if ch >= 'a' && ch <= 'z' {
                let code = ch as u8 - b'a' + 1;
                bytes.push(code);
                return Some(bytes);
            }
        }
    }

    match key {
        Key::Named(Named::Enter) => bytes.push(b'\r'),
        Key::Named(Named::Backspace) => bytes.push(0x08),
        Key::Named(Named::Tab) => bytes.push(b'\t'),
        Key::Named(Named::Escape) => bytes.push(0x1b),
        Key::Named(Named::ArrowUp) => bytes.extend_from_slice(b"\x1b[A"),
        Key::Named(Named::ArrowDown) => bytes.extend_from_slice(b"\x1b[B"),
        Key::Named(Named::ArrowRight) => bytes.extend_from_slice(b"\x1b[C"),
        Key::Named(Named::ArrowLeft) => bytes.extend_from_slice(b"\x1b[D"),
        Key::Named(Named::Delete) => bytes.extend_from_slice(b"\x1b[3~"),
        Key::Named(Named::Home) => bytes.extend_from_slice(b"\x1b[H"),
        Key::Named(Named::End) => bytes.extend_from_slice(b"\x1b[F"),
        Key::Named(Named::PageUp) => bytes.extend_from_slice(b"\x1b[5~"),
        Key::Named(Named::PageDown) => bytes.extend_from_slice(b"\x1b[6~"),
        Key::Character(c) => {
            bytes.extend_from_slice(c.as_bytes());
        }
        _ => return None,
    }

    if bytes.is_empty() {
        None
    } else {
        Some(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmic::iced::keyboard::Modifiers;

    #[test]
    fn test_ctrl_c() {
        let config = Config::default();
        let key = Key::Character("c".into());
        let mods = Modifiers::CTRL;
        assert_eq!(key_to_ansi(&key, mods, &config), Some(vec![3]));
    }

    #[test]
    fn test_arrow_up() {
        let config = Config::default();
        let key = Key::Named(Named::ArrowUp);
        let mods = Modifiers::empty();
        assert_eq!(key_to_ansi(&key, mods, &config), Some(b"\x1b[A".to_vec()));
    }
}
