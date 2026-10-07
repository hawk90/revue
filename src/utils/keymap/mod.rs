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

mod parse;
mod presets;

use crate::event::KeyBinding;
use std::collections::HashMap;

pub use parse::{format_key_binding, parse_key_binding};
pub use presets::{emacs_preset, vim_preset};

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
        self.pending.push(key.normalized());

        let chord = KeyChord {
            keys: self.pending.clone(),
        };

        // Check global bindings first
        if let Some(action) = self.global_bindings.get(&chord) {
            self.pending.clear();
            return LookupResult::Action(action.clone());
        }

        // Check mode-specific bindings
        let mode_bindings = self.bindings.get(&self.current_mode);
        if let Some(action) = mode_bindings.and_then(|m| m.get(&chord)) {
            self.pending.clear();
            return LookupResult::Action(action.clone());
        }

        // Check if this could be a prefix of a longer chord, global or mode
        let is_prefix = |existing: &KeyChord| {
            existing.keys.len() > self.pending.len() && existing.keys.starts_with(&self.pending)
        };
        if self.global_bindings.keys().any(is_prefix)
            || mode_bindings.is_some_and(|m| m.keys().any(is_prefix))
        {
            return LookupResult::Pending;
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
