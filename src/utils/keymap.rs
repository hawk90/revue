//! Extended keymap utilities
//!
//! Provides configurable key binding management for TUI applications.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::utils::keymap::{KeymapConfig, Mode, bind};
//!
//! let mut keymap = KeymapConfig::new();
//!
//! // Add mode-specific bindings
//! keymap.bind(Mode::Normal, "j", "move_down");
//! keymap.bind(Mode::Normal, "k", "move_up");
//! keymap.bind(Mode::Insert, "Escape", "exit_insert");
//!
//! // Parse and execute
//! keymap.set_mode(Mode::Normal);
//! let action = keymap.lookup("j");
//! ```

use crate::event::{Key, KeyBinding};
use std::collections::HashMap;

/// Input mode
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Mode {
    /// Normal mode (navigation)
    #[default]
    Normal,
    /// Insert mode (text input)
    Insert,
    /// Visual mode (selection)
    Visual,
    /// Command mode (ex commands)
    Command,
    /// Search mode
    Search,
    /// Custom mode
    Custom(u8),
}

impl Mode {
    /// Get mode name
    pub fn name(&self) -> &'static str {
        match self {
            Mode::Normal => "NORMAL",
            Mode::Insert => "INSERT",
            Mode::Visual => "VISUAL",
            Mode::Command => "COMMAND",
            Mode::Search => "SEARCH",
            Mode::Custom(_) => "CUSTOM",
        }
    }
}

/// Key chord (multiple keys)
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct KeyChord {
    /// Keys in the chord
    pub keys: Vec<KeyBinding>,
}

impl KeyChord {
    /// Create single key chord
    pub fn single(key: KeyBinding) -> Self {
        Self { keys: vec![key] }
    }

    /// Create multi-key chord
    pub fn multi(keys: Vec<KeyBinding>) -> Self {
        Self { keys }
    }

    /// Parse from string (e.g., "Ctrl-x Ctrl-s")
    pub fn parse(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.is_empty() {
            return None;
        }

        let keys: Option<Vec<KeyBinding>> = parts.iter().map(|p| parse_key_binding(p)).collect();
        keys.map(|k| Self { keys: k })
    }
}

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
    };

    parts.push(&key_str);
    parts.join("-")
}

/// Keymap configuration
#[derive(Clone, Debug)]
pub struct KeymapConfig {
    /// Mode-specific bindings
    bindings: HashMap<Mode, HashMap<KeyChord, String>>,
    /// Current mode
    current_mode: Mode,
    /// Pending keys for multi-key chords
    pending: Vec<KeyBinding>,
    /// Timeout for multi-key chords (ms)
    chord_timeout: u64,
    /// Global bindings (active in all modes)
    global_bindings: HashMap<KeyChord, String>,
}

impl KeymapConfig {
    /// Create new keymap config
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
            current_mode: Mode::Normal,
            pending: Vec::new(),
            chord_timeout: 1000,
            global_bindings: HashMap::new(),
        }
    }

    /// Set current mode
    pub fn set_mode(&mut self, mode: Mode) {
        self.current_mode = mode;
        self.pending.clear();
    }

    /// Get current mode
    pub fn mode(&self) -> Mode {
        self.current_mode
    }

    /// Add a binding to a specific mode
    pub fn bind(&mut self, mode: Mode, keys: &str, action: impl Into<String>) {
        if let Some(chord) = KeyChord::parse(keys) {
            self.bindings
                .entry(mode)
                .or_default()
                .insert(chord, action.into());
        }
    }

    /// Add a global binding (all modes)
    pub fn bind_global(&mut self, keys: &str, action: impl Into<String>) {
        if let Some(chord) = KeyChord::parse(keys) {
            self.global_bindings.insert(chord, action.into());
        }
    }

    /// Remove a binding
    pub fn unbind(&mut self, mode: Mode, keys: &str) {
        if let Some(chord) = KeyChord::parse(keys) {
            if let Some(mode_bindings) = self.bindings.get_mut(&mode) {
                mode_bindings.remove(&chord);
            }
        }
    }

    /// Look up action for a key
    pub fn lookup(&mut self, key: KeyBinding) -> LookupResult {
        self.pending.push(key);

        let chord = KeyChord {
            keys: self.pending.clone(),
        };

        // Check global bindings first
        if let Some(action) = self.global_bindings.get(&chord) {
            self.pending.clear();
            return LookupResult::Action(action.clone());
        }

        // Check mode-specific bindings
        if let Some(mode_bindings) = self.bindings.get(&self.current_mode) {
            if let Some(action) = mode_bindings.get(&chord) {
                self.pending.clear();
                return LookupResult::Action(action.clone());
            }

            // Check if this could be a prefix of a longer chord
            for existing_chord in mode_bindings.keys() {
                if existing_chord.keys.len() > self.pending.len()
                    && existing_chord.keys.starts_with(&self.pending)
                {
                    return LookupResult::Pending;
                }
            }
        }

        // No match and no prefix match
        self.pending.clear();
        LookupResult::None
    }

    /// Clear pending keys
    pub fn clear_pending(&mut self) {
        self.pending.clear();
    }

    /// Get pending keys
    pub fn pending_keys(&self) -> &[KeyBinding] {
        &self.pending
    }

    /// Check if there are pending keys
    pub fn has_pending(&self) -> bool {
        !self.pending.is_empty()
    }

    /// Set chord timeout
    pub fn chord_timeout(&mut self, ms: u64) {
        self.chord_timeout = ms;
    }

    /// Get all bindings for a mode
    pub fn bindings_for_mode(&self, mode: Mode) -> Vec<(&KeyChord, &str)> {
        self.bindings
            .get(&mode)
            .map(|m| m.iter().map(|(k, v)| (k, v.as_str())).collect())
            .unwrap_or_default()
    }

    /// Get all global bindings
    pub fn global_bindings(&self) -> Vec<(&KeyChord, &str)> {
        self.global_bindings
            .iter()
            .map(|(k, v)| (k, v.as_str()))
            .collect()
    }
}

impl Default for KeymapConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of key lookup
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LookupResult {
    /// No matching binding
    None,
    /// Matched an action
    Action(String),
    /// Could be part of a longer chord, waiting for more keys
    Pending,
}

/// Vim-style keymap preset
pub fn vim_preset() -> KeymapConfig {
    let mut config = KeymapConfig::new();

    // Normal mode
    config.bind(Mode::Normal, "h", "move_left");
    config.bind(Mode::Normal, "j", "move_down");
    config.bind(Mode::Normal, "k", "move_up");
    config.bind(Mode::Normal, "l", "move_right");
    config.bind(Mode::Normal, "i", "enter_insert");
    config.bind(Mode::Normal, "a", "append");
    config.bind(Mode::Normal, "A", "append_end");
    config.bind(Mode::Normal, "o", "open_below");
    config.bind(Mode::Normal, "O", "open_above");
    config.bind(Mode::Normal, "v", "enter_visual");
    config.bind(Mode::Normal, ":", "enter_command");
    config.bind(Mode::Normal, "/", "search_forward");
    config.bind(Mode::Normal, "?", "search_backward");
    config.bind(Mode::Normal, "n", "search_next");
    config.bind(Mode::Normal, "N", "search_prev");
    config.bind(Mode::Normal, "g g", "goto_first");
    config.bind(Mode::Normal, "G", "goto_last");
    config.bind(Mode::Normal, "Ctrl-u", "page_up");
    config.bind(Mode::Normal, "Ctrl-d", "page_down");
    config.bind(Mode::Normal, "d d", "delete_line");
    config.bind(Mode::Normal, "y y", "yank_line");
    config.bind(Mode::Normal, "p", "paste_after");
    config.bind(Mode::Normal, "P", "paste_before");
    config.bind(Mode::Normal, "u", "undo");
    config.bind(Mode::Normal, "Ctrl-r", "redo");

    // Insert mode
    config.bind(Mode::Insert, "Escape", "exit_insert");
    config.bind(Mode::Insert, "Ctrl-c", "exit_insert");

    // Visual mode
    config.bind(Mode::Visual, "Escape", "exit_visual");
    config.bind(Mode::Visual, "h", "extend_left");
    config.bind(Mode::Visual, "j", "extend_down");
    config.bind(Mode::Visual, "k", "extend_up");
    config.bind(Mode::Visual, "l", "extend_right");
    config.bind(Mode::Visual, "y", "yank_selection");
    config.bind(Mode::Visual, "d", "delete_selection");

    // Command mode
    config.bind(Mode::Command, "Escape", "exit_command");
    config.bind(Mode::Command, "Enter", "execute_command");

    // Global
    config.bind_global("Ctrl-c", "quit");
    config.bind_global("Ctrl-z", "suspend");

    config
}

/// Emacs-style keymap preset
pub fn emacs_preset() -> KeymapConfig {
    let mut config = KeymapConfig::new();

    // Navigation
    config.bind(Mode::Normal, "Ctrl-p", "move_up");
    config.bind(Mode::Normal, "Ctrl-n", "move_down");
    config.bind(Mode::Normal, "Ctrl-b", "move_left");
    config.bind(Mode::Normal, "Ctrl-f", "move_right");
    config.bind(Mode::Normal, "Ctrl-a", "line_start");
    config.bind(Mode::Normal, "Ctrl-e", "line_end");
    config.bind(Mode::Normal, "Alt-<", "goto_first");
    config.bind(Mode::Normal, "Alt->", "goto_last");
    config.bind(Mode::Normal, "Ctrl-v", "page_down");
    config.bind(Mode::Normal, "Alt-v", "page_up");

    // Editing
    config.bind(Mode::Normal, "Ctrl-d", "delete_char");
    config.bind(Mode::Normal, "Ctrl-k", "kill_line");
    config.bind(Mode::Normal, "Ctrl-y", "yank");
    config.bind(Mode::Normal, "Ctrl-w", "cut_region");
    config.bind(Mode::Normal, "Alt-w", "copy_region");

    // Search
    config.bind(Mode::Normal, "Ctrl-s", "search_forward");
    config.bind(Mode::Normal, "Ctrl-r", "search_backward");

    // Undo
    config.bind(Mode::Normal, "Ctrl-/", "undo");
    config.bind(Mode::Normal, "Ctrl-x u", "undo");

    // File operations
    config.bind(Mode::Normal, "Ctrl-x Ctrl-s", "save");
    config.bind(Mode::Normal, "Ctrl-x Ctrl-c", "quit");
    config.bind(Mode::Normal, "Ctrl-x Ctrl-f", "open_file");

    config
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_key_binding() {
        let binding = parse_key_binding("j").unwrap();
        assert_eq!(binding.key, Key::Char('j'));
        assert!(!binding.ctrl);

        let binding = parse_key_binding("Ctrl-c").unwrap();
        assert_eq!(binding.key, Key::Char('c'));
        assert!(binding.ctrl);

        let binding = parse_key_binding("Ctrl-Alt-Delete").unwrap();
        assert_eq!(binding.key, Key::Delete);
        assert!(binding.ctrl);
        assert!(binding.alt);
    }

    #[test]
    fn test_format_key_binding() {
        let binding = KeyBinding {
            key: Key::Char('c'),
            ctrl: true,
            alt: false,
            shift: false,
        };
        assert_eq!(format_key_binding(&binding), "Ctrl-c");

        let binding = KeyBinding {
            key: Key::Enter,
            ctrl: false,
            alt: false,
            shift: false,
        };
        assert_eq!(format_key_binding(&binding), "Enter");
    }

    #[test]
    fn test_key_chord_parse() {
        let chord = KeyChord::parse("Ctrl-x Ctrl-s").unwrap();
        assert_eq!(chord.keys.len(), 2);
        assert!(chord.keys[0].ctrl);
        assert!(chord.keys[1].ctrl);
    }

    #[test]
    fn test_keymap_single_key() {
        let mut keymap = KeymapConfig::new();
        keymap.bind(Mode::Normal, "j", "move_down");

        let binding = parse_key_binding("j").unwrap();
        let result = keymap.lookup(binding);
        assert_eq!(result, LookupResult::Action("move_down".to_string()));
    }

    #[test]
    fn test_keymap_multi_key() {
        let mut keymap = KeymapConfig::new();
        keymap.bind(Mode::Normal, "g g", "goto_first");

        let g1 = parse_key_binding("g").unwrap();
        let result = keymap.lookup(g1.clone());
        assert_eq!(result, LookupResult::Pending);

        let result = keymap.lookup(g1);
        assert_eq!(result, LookupResult::Action("goto_first".to_string()));
    }

    #[test]
    fn test_keymap_no_match() {
        let mut keymap = KeymapConfig::new();
        keymap.bind(Mode::Normal, "j", "move_down");

        let binding = parse_key_binding("x").unwrap();
        let result = keymap.lookup(binding);
        assert_eq!(result, LookupResult::None);
    }

    #[test]
    fn test_keymap_modes() {
        let mut keymap = KeymapConfig::new();
        keymap.bind(Mode::Normal, "i", "enter_insert");
        keymap.bind(Mode::Insert, "Escape", "exit_insert");

        keymap.set_mode(Mode::Normal);
        let i = parse_key_binding("i").unwrap();
        let result = keymap.lookup(i);
        assert_eq!(result, LookupResult::Action("enter_insert".to_string()));

        keymap.set_mode(Mode::Insert);
        let esc = parse_key_binding("Escape").unwrap();
        let result = keymap.lookup(esc);
        assert_eq!(result, LookupResult::Action("exit_insert".to_string()));
    }

    #[test]
    fn test_vim_preset() {
        let mut keymap = vim_preset();

        let j = parse_key_binding("j").unwrap();
        let result = keymap.lookup(j);
        assert_eq!(result, LookupResult::Action("move_down".to_string()));
    }

    #[test]
    fn test_emacs_preset() {
        let mut keymap = emacs_preset();

        let ctrl_n = parse_key_binding("Ctrl-n").unwrap();
        let result = keymap.lookup(ctrl_n);
        assert_eq!(result, LookupResult::Action("move_down".to_string()));
    }

    #[test]
    fn test_global_bindings() {
        let mut keymap = KeymapConfig::new();
        keymap.bind_global("Ctrl-c", "quit");

        keymap.set_mode(Mode::Normal);
        let ctrl_c = parse_key_binding("Ctrl-c").unwrap();
        let result = keymap.lookup(ctrl_c.clone());
        assert_eq!(result, LookupResult::Action("quit".to_string()));

        keymap.set_mode(Mode::Insert);
        let result = keymap.lookup(ctrl_c);
        assert_eq!(result, LookupResult::Action("quit".to_string()));
    }
}
