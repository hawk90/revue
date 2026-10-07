//! Parsing and formatting of key binding strings

use crate::event::{Key, KeyBinding};

/// Parse a single key binding string
pub fn parse_key_binding(s: &str) -> Option<KeyBinding> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }

    let mut ctrl = false;
    let mut alt = false;
    let mut shift = false;
    let mut key_part = s;

    // Parse modifiers
    loop {
        let lower = key_part.to_lowercase();
        if lower.starts_with("ctrl-") || lower.starts_with("c-") {
            ctrl = true;
            key_part = if lower.starts_with("ctrl-") {
                &key_part[5..]
            } else {
                &key_part[2..]
            };
        } else if lower.starts_with("alt-") || lower.starts_with("m-") {
            alt = true;
            key_part = if lower.starts_with("alt-") {
                &key_part[4..]
            } else {
                &key_part[2..]
            };
        } else if lower.starts_with("shift-") || lower.starts_with("s-") {
            shift = true;
            key_part = if lower.starts_with("shift-") {
                &key_part[6..]
            } else {
                &key_part[2..]
            };
        } else {
            break;
        }
    }

    let key = parse_key(key_part)?;

    Some(KeyBinding {
        key,
        ctrl,
        alt,
        shift,
    })
}

/// Parse key name to Key enum
fn parse_key(s: &str) -> Option<Key> {
    let lower = s.to_lowercase();
    match lower.as_str() {
        "enter" | "return" | "cr" => Some(Key::Enter),
        "escape" | "esc" => Some(Key::Escape),
        "tab" => Some(Key::Tab),
        "backtab" | "s-tab" => Some(Key::BackTab),
        "backspace" | "bs" => Some(Key::Backspace),
        "delete" | "del" => Some(Key::Delete),
        "up" => Some(Key::Up),
        "down" => Some(Key::Down),
        "left" => Some(Key::Left),
        "right" => Some(Key::Right),
        "home" => Some(Key::Home),
        "end" => Some(Key::End),
        "pageup" | "pgup" => Some(Key::PageUp),
        "pagedown" | "pgdn" => Some(Key::PageDown),
        "insert" | "ins" => Some(Key::Insert),
        "space" => Some(Key::Char(' ')),
        "f1" => Some(Key::F(1)),
        "f2" => Some(Key::F(2)),
        "f3" => Some(Key::F(3)),
        "f4" => Some(Key::F(4)),
        "f5" => Some(Key::F(5)),
        "f6" => Some(Key::F(6)),
        "f7" => Some(Key::F(7)),
        "f8" => Some(Key::F(8)),
        "f9" => Some(Key::F(9)),
        "f10" => Some(Key::F(10)),
        "f11" => Some(Key::F(11)),
        "f12" => Some(Key::F(12)),
        _ => {
            // Single character
            let chars: Vec<char> = s.chars().collect();
            if chars.len() == 1 {
                Some(Key::Char(chars[0]))
            } else {
                None
            }
        }
    }
}

/// Format a key binding for display
pub fn format_key_binding(binding: &KeyBinding) -> String {
    let mut parts = Vec::new();

    if binding.ctrl {
        parts.push("Ctrl");
    }
    if binding.alt {
        parts.push("Alt");
    }
    if binding.shift {
        parts.push("Shift");
    }

    let key_str = match binding.key {
        Key::Char(' ') => "Space".to_string(),
        Key::Char(c) => c.to_string(),
        Key::Enter => "Enter".to_string(),
        Key::Escape => "Esc".to_string(),
        Key::Tab => "Tab".to_string(),
        Key::BackTab => "BackTab".to_string(),
        Key::Backspace => "Backspace".to_string(),
        Key::Delete => "Del".to_string(),
        Key::Up => "↑".to_string(),
        Key::Down => "↓".to_string(),
        Key::Left => "←".to_string(),
        Key::Right => "→".to_string(),
        Key::Home => "Home".to_string(),
        Key::End => "End".to_string(),
        Key::PageUp => "PgUp".to_string(),
        Key::PageDown => "PgDn".to_string(),
        Key::Insert => "Ins".to_string(),
        Key::F(n) => format!("F{}", n),
        Key::Null => "Null".to_string(),
        Key::Unknown => "Unknown".to_string(),
    };

    parts.push(&key_str);
    parts.join("-")
}
