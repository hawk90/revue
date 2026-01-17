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
        self.bindings.insert(binding, action);
    }

    /// Get the action for a key binding
    pub fn get(&self, binding: &KeyBinding) -> Option<&A> {
        self.bindings.get(binding)
    }
}

impl<A: Clone> Default for KeyMap<A> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // Key Tests
    // =========================================================================

    #[test]
    fn test_key_char() {
        let key = Key::Char('a');
        assert_eq!(key, Key::Char('a'));
    }

    #[test]
    fn test_key_enter() {
        let key = Key::Enter;
        assert_eq!(key, Key::Enter);
    }

    #[test]
    fn test_key_escape() {
        let key = Key::Escape;
        assert_eq!(key, Key::Escape);
    }

    #[test]
    fn test_key_tab() {
        let key = Key::Tab;
        assert_eq!(key, Key::Tab);
    }

    #[test]
    fn test_key_backtab() {
        let key = Key::BackTab;
        assert_eq!(key, Key::BackTab);
    }

    #[test]
    fn test_key_backspace() {
        let key = Key::Backspace;
        assert_eq!(key, Key::Backspace);
    }

    #[test]
    fn test_key_delete() {
        let key = Key::Delete;
        assert_eq!(key, Key::Delete);
    }

    #[test]
    fn test_key_arrows() {
        assert_eq!(Key::Up, Key::Up);
        assert_eq!(Key::Down, Key::Down);
        assert_eq!(Key::Left, Key::Left);
        assert_eq!(Key::Right, Key::Right);
    }

    #[test]
    fn test_key_home_end() {
        assert_eq!(Key::Home, Key::Home);
        assert_eq!(Key::End, Key::End);
    }

    #[test]
    fn test_key_page_up_down() {
        assert_eq!(Key::PageUp, Key::PageUp);
        assert_eq!(Key::PageDown, Key::PageDown);
    }

    #[test]
    fn test_key_function() {
        let f1 = Key::F(1);
        let f12 = Key::F(12);
        assert_eq!(f1, Key::F(1));
        assert_eq!(f12, Key::F(12));
        assert_ne!(f1, f12);
    }

    #[test]
    fn test_key_insert() {
        assert_eq!(Key::Insert, Key::Insert);
    }

    #[test]
    fn test_key_null() {
        assert_eq!(Key::Null, Key::Null);
    }

    #[test]
    fn test_key_unknown() {
        assert_eq!(Key::Unknown, Key::Unknown);
    }

    #[test]
    fn test_key_equality() {
        assert_eq!(Key::Char('a'), Key::Char('a'));
        assert_ne!(Key::Char('a'), Key::Char('b'));
        assert_ne!(Key::Char('a'), Key::Enter);
    }

    #[test]
    fn test_key_clone() {
        let key = Key::Char('x');
        let cloned = key;
        assert_eq!(key, cloned);
    }

    #[test]
    fn test_key_debug() {
        let key = Key::Enter;
        let debug = format!("{:?}", key);
        assert!(debug.contains("Enter"));
    }

    #[test]
    fn test_key_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(Key::Char('a'));
        set.insert(Key::Enter);
        set.insert(Key::Char('a')); // Duplicate
        assert_eq!(set.len(), 2);
    }

    // =========================================================================
    // Key::ctrl and Key::alt Tests
    // =========================================================================

    #[test]
    fn test_key_ctrl() {
        let binding = Key::ctrl('c');
        assert_eq!(binding.key, Key::Char('c'));
        assert!(binding.ctrl);
        assert!(!binding.alt);
        assert!(!binding.shift);
    }

    #[test]
    fn test_key_alt() {
        let binding = Key::alt('x');
        assert_eq!(binding.key, Key::Char('x'));
        assert!(!binding.ctrl);
        assert!(binding.alt);
        assert!(!binding.shift);
    }

    // =========================================================================
    // KeyBinding Tests
    // =========================================================================

    #[test]
    fn test_keybinding_creation() {
        let binding = KeyBinding {
            key: Key::Enter,
            ctrl: false,
            alt: false,
            shift: false,
        };
        assert_eq!(binding.key, Key::Enter);
        assert!(!binding.ctrl);
        assert!(!binding.alt);
        assert!(!binding.shift);
    }

    #[test]
    fn test_keybinding_with_ctrl() {
        let binding = KeyBinding {
            key: Key::Char('s'),
            ctrl: true,
            alt: false,
            shift: false,
        };
        assert!(binding.ctrl);
    }

    #[test]
    fn test_keybinding_with_alt() {
        let binding = KeyBinding {
            key: Key::Char('f'),
            ctrl: false,
            alt: true,
            shift: false,
        };
        assert!(binding.alt);
    }

    #[test]
    fn test_keybinding_with_shift() {
        let binding = KeyBinding {
            key: Key::Tab,
            ctrl: false,
            alt: false,
            shift: true,
        };
        assert!(binding.shift);
    }

    #[test]
    fn test_keybinding_with_multiple_modifiers() {
        let binding = KeyBinding {
            key: Key::Char('k'),
            ctrl: true,
            alt: true,
            shift: true,
        };
        assert!(binding.ctrl);
        assert!(binding.alt);
        assert!(binding.shift);
    }

    #[test]
    fn test_keybinding_equality() {
        let b1 = KeyBinding {
            key: Key::Char('a'),
            ctrl: true,
            alt: false,
            shift: false,
        };
        let b2 = KeyBinding {
            key: Key::Char('a'),
            ctrl: true,
            alt: false,
            shift: false,
        };
        let b3 = KeyBinding {
            key: Key::Char('a'),
            ctrl: false,
            alt: false,
            shift: false,
        };
        assert_eq!(b1, b2);
        assert_ne!(b1, b3);
    }

    #[test]
    fn test_keybinding_clone() {
        let binding = KeyBinding {
            key: Key::Escape,
            ctrl: false,
            alt: true,
            shift: false,
        };
        let cloned = binding.clone();
        assert_eq!(binding, cloned);
    }

    #[test]
    fn test_keybinding_debug() {
        let binding = Key::ctrl('c');
        let debug = format!("{:?}", binding);
        assert!(debug.contains("Char"));
        assert!(debug.contains("ctrl"));
    }

    #[test]
    fn test_keybinding_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(Key::ctrl('c'));
        set.insert(Key::ctrl('v'));
        set.insert(Key::ctrl('c')); // Duplicate
        assert_eq!(set.len(), 2);
    }

    // =========================================================================
    // KeyMap Tests
    // =========================================================================

    #[derive(Debug, Clone, PartialEq)]
    enum TestAction {
        Save,
        Quit,
        Copy,
        Paste,
        Undo,
    }

    #[test]
    fn test_keymap_new() {
        let map: KeyMap<TestAction> = KeyMap::new();
        assert!(map.bindings.is_empty());
    }

    #[test]
    fn test_keymap_default() {
        let map: KeyMap<TestAction> = KeyMap::default();
        assert!(map.bindings.is_empty());
    }

    #[test]
    fn test_keymap_bind() {
        let mut map = KeyMap::new();
        map.bind(Key::ctrl('s'), TestAction::Save);
        assert_eq!(map.bindings.len(), 1);
    }

    #[test]
    fn test_keymap_get() {
        let mut map = KeyMap::new();
        map.bind(Key::ctrl('s'), TestAction::Save);

        let result = map.get(&Key::ctrl('s'));
        assert_eq!(result, Some(&TestAction::Save));
    }

    #[test]
    fn test_keymap_get_not_found() {
        let map: KeyMap<TestAction> = KeyMap::new();
        let result = map.get(&Key::ctrl('x'));
        assert!(result.is_none());
    }

    #[test]
    fn test_keymap_multiple_bindings() {
        let mut map = KeyMap::new();
        map.bind(Key::ctrl('s'), TestAction::Save);
        map.bind(Key::ctrl('q'), TestAction::Quit);
        map.bind(Key::ctrl('c'), TestAction::Copy);
        map.bind(Key::ctrl('v'), TestAction::Paste);
        map.bind(Key::ctrl('z'), TestAction::Undo);

        assert_eq!(map.get(&Key::ctrl('s')), Some(&TestAction::Save));
        assert_eq!(map.get(&Key::ctrl('q')), Some(&TestAction::Quit));
        assert_eq!(map.get(&Key::ctrl('c')), Some(&TestAction::Copy));
        assert_eq!(map.get(&Key::ctrl('v')), Some(&TestAction::Paste));
        assert_eq!(map.get(&Key::ctrl('z')), Some(&TestAction::Undo));
    }

    #[test]
    fn test_keymap_overwrite_binding() {
        let mut map = KeyMap::new();
        map.bind(Key::ctrl('s'), TestAction::Save);
        map.bind(Key::ctrl('s'), TestAction::Quit); // Overwrite

        assert_eq!(map.get(&Key::ctrl('s')), Some(&TestAction::Quit));
    }

    #[test]
    fn test_keymap_with_different_modifiers() {
        let mut map = KeyMap::new();

        // Same key, different modifiers
        let ctrl_a = KeyBinding {
            key: Key::Char('a'),
            ctrl: true,
            alt: false,
            shift: false,
        };
        let alt_a = KeyBinding {
            key: Key::Char('a'),
            ctrl: false,
            alt: true,
            shift: false,
        };

        map.bind(ctrl_a.clone(), TestAction::Copy);
        map.bind(alt_a.clone(), TestAction::Paste);

        assert_eq!(map.get(&ctrl_a), Some(&TestAction::Copy));
        assert_eq!(map.get(&alt_a), Some(&TestAction::Paste));
    }

    #[test]
    fn test_keymap_with_non_char_keys() {
        let mut map = KeyMap::new();

        let f1_binding = KeyBinding {
            key: Key::F(1),
            ctrl: false,
            alt: false,
            shift: false,
        };
        let enter_binding = KeyBinding {
            key: Key::Enter,
            ctrl: false,
            alt: false,
            shift: false,
        };

        map.bind(f1_binding.clone(), TestAction::Save);
        map.bind(enter_binding.clone(), TestAction::Quit);

        assert_eq!(map.get(&f1_binding), Some(&TestAction::Save));
        assert_eq!(map.get(&enter_binding), Some(&TestAction::Quit));
    }

    #[test]
    fn test_keymap_with_string_action() {
        let mut map: KeyMap<String> = KeyMap::new();
        map.bind(Key::ctrl('s'), "save".to_string());
        map.bind(Key::ctrl('q'), "quit".to_string());

        assert_eq!(map.get(&Key::ctrl('s')), Some(&"save".to_string()));
        assert_eq!(map.get(&Key::ctrl('q')), Some(&"quit".to_string()));
    }

    #[test]
    fn test_keymap_with_integer_action() {
        let mut map: KeyMap<i32> = KeyMap::new();
        map.bind(Key::ctrl('1'), 1);
        map.bind(Key::ctrl('2'), 2);
        map.bind(Key::ctrl('3'), 3);

        assert_eq!(map.get(&Key::ctrl('1')), Some(&1));
        assert_eq!(map.get(&Key::ctrl('2')), Some(&2));
        assert_eq!(map.get(&Key::ctrl('3')), Some(&3));
    }
}
