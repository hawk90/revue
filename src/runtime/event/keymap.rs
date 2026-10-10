//! Key definitions and keymaps

/// Key code
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    /// Character key
    Char(char),
    /// Enter/Return
    Enter,
    /// Escape
    Escape,
    /// Tab
    Tab,
    /// Shift+Tab (back tab)
    BackTab,
    /// Backspace
    Backspace,
    /// Delete
    Delete,
    /// Arrow up
    Up,
    /// Arrow down
    Down,
    /// Arrow left
    Left,
    /// Arrow right
    Right,
    /// Home
    Home,
    /// End
    End,
    /// Page up
    PageUp,
    /// Page down
    PageDown,
    /// Function key (F1-F12)
    F(u8),
    /// Insert
    Insert,
    /// Null (no key)
    Null,
    /// Unknown key (not recognized)
    Unknown,
}

impl Key {
    /// Create a Ctrl+key binding
    pub fn ctrl(ch: char) -> KeyBinding {
        KeyBinding {
            key: Key::Char(ch),
            ctrl: true,
            alt: false,
            shift: false,
        }
    }

    /// Create an Alt+key binding
    pub fn alt(ch: char) -> KeyBinding {
        KeyBinding {
            key: Key::Char(ch),
            ctrl: false,
            alt: true,
            shift: false,
        }
    }
}

/// A key binding with modifiers
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyBinding {
    /// The key code
    pub key: Key,
    /// Ctrl modifier
    pub ctrl: bool,
    /// Alt modifier
    pub alt: bool,
    /// Shift modifier
    pub shift: bool,
}

impl KeyBinding {
    /// The canonical form used to compare bindings
    ///
    /// `BackTab` already means Shift+Tab, and terminals differ on whether
    /// they also report the Shift flag with it (crossterm does), so a
    /// BackTab binding never carries `shift`.
    pub(crate) fn normalized(mut self) -> Self {
        if self.key == Key::BackTab {
            self.shift = false;
        }
        self
    }
}

/// Keymap for mapping keys to actions
pub struct KeyMap<A> {
    bindings: std::collections::HashMap<KeyBinding, A>,
}

impl<A: Clone> KeyMap<A> {
    /// Create a new empty keymap
    pub fn new() -> Self {
        Self {
            bindings: std::collections::HashMap::new(),
        }
    }

    /// Bind a key to an action
    pub fn bind(&mut self, binding: KeyBinding, action: A) {
        self.bindings.insert(binding.normalized(), action);
    }

    /// Get the action for a key binding
    pub fn get(&self, binding: &KeyBinding) -> Option<&A> {
        self.bindings.get(&binding.clone().normalized())
    }
}

impl<A: Clone> Default for KeyMap<A> {
    fn default() -> Self {
        Self::new()
    }
}
