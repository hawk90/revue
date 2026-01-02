//! Layered key handling pattern
//!
//! Provides a standard pattern for handling keyboard input with proper priority.
//! Ensures modals, dialogs, and popups take precedence over normal navigation.
//!
//! # Priority Layers
//!
//! 1. **Quit check** - Always check first (unless in blocking modal)
//! 2. **Modals/Dialogs** - Full-screen overlays (help, forms)
//! 3. **Confirm dialogs** - Yes/No confirmations
//! 4. **Popups** - Temporary overlays (search, filters)
//! 5. **View-specific** - Normal navigation and actions
//!
//! # Example
//!
//! ```ignore
//! use revue::patterns::KeyHandler;
//! use crossterm::event::KeyCode;
//!
//! struct App {
//!     view_mode: ViewMode,
//!     confirm: ConfirmState,
//!     popup_active: bool,
//!     quit: bool,
//! }
//!
//! impl KeyHandler for App {
//!     fn should_quit(&self, key: &KeyCode) -> bool {
//!         matches!(key, KeyCode::Char('q')) && !self.confirm.is_active()
//!     }
//!
//!     fn in_modal(&self) -> bool {
//!         matches!(self.view_mode, ViewMode::Help | ViewMode::Form)
//!     }
//!
//!     fn handle_modal_key(&mut self, key: &KeyCode) -> bool {
//!         match self.view_mode {
//!             ViewMode::Help => self.handle_help_key(key),
//!             ViewMode::Form => self.handle_form_key(key),
//!             _ => true,
//!         }
//!     }
//!
//!     fn has_confirm(&self) -> bool {
//!         self.confirm.is_active()
//!     }
//!
//!     fn handle_confirm_key(&mut self, key: &KeyCode) -> bool {
//!         match key {
//!             KeyCode::Char('y') | KeyCode::Enter => {
//!                 self.confirm.execute(|action| self.do_action(action));
//!             }
//!             _ => self.confirm.cancel(),
//!         }
//!         true
//!     }
//!
//!     fn handle_view_key(&mut self, key: &KeyCode) -> bool {
//!         match self.view_mode {
//!             ViewMode::Main => self.handle_main_key(key),
//!             ViewMode::Detail => self.handle_detail_key(key),
//!             _ => true,
//!         }
//!     }
//! }
//! ```

use crossterm::event::KeyCode;

/// Layered key handling
///
/// Implement this trait to get standardized key handling with proper priority.
///
/// The default `handle_key` implementation checks layers in order:
/// 1. Quit (if `should_quit` returns true, returns false to exit)
/// 2. Modals (if `in_modal`, calls `handle_modal_key`)
/// 3. Confirm (if `has_confirm`, calls `handle_confirm_key`)
/// 4. Popups (if `has_popup`, calls `handle_popup_key`)
/// 5. View (calls `handle_view_key`)
pub trait KeyHandler {
    /// Handle a key press
    ///
    /// Returns `false` if app should quit, `true` otherwise.
    ///
    /// This is the main entry point for key handling.
    /// Override this if you need custom priority logic.
    fn handle_key(&mut self, key: &KeyCode) -> bool {
        // 1. Quit check (highest priority)
        if self.should_quit(key) {
            return false; // Exit app
        }

        // 2. Modal/Dialog layer
        if self.in_modal() {
            return self.handle_modal_key(key);
        }

        // 3. Confirm dialog layer
        if self.has_confirm() {
            return self.handle_confirm_key(key);
        }

        // 4. Popup layer
        if self.has_popup() {
            return self.handle_popup_key(key);
        }

        // 5. Normal view handling
        self.handle_view_key(key)
    }

    /// Check if app should quit
    ///
    /// # Example
    ///
    /// ```ignore
    /// fn should_quit(&self, key: &KeyCode) -> bool {
    ///     matches!(key, KeyCode::Char('q'))
    ///         && !self.confirm.is_active()
    ///         && !self.in_modal()
    /// }
    /// ```
    fn should_quit(&self, key: &KeyCode) -> bool;

    /// Check if in modal/dialog mode
    ///
    /// Modals are full-screen overlays like help screens or forms.
    ///
    /// # Example
    ///
    /// ```ignore
    /// fn in_modal(&self) -> bool {
    ///     matches!(self.view_mode, ViewMode::Help | ViewMode::Form)
    /// }
    /// ```
    fn in_modal(&self) -> bool {
        false
    }

    /// Handle key in modal mode
    ///
    /// # Example
    ///
    /// ```ignore
    /// fn handle_modal_key(&mut self, key: &KeyCode) -> bool {
    ///     match key {
    ///         KeyCode::Esc => {
    ///             self.view_mode = ViewMode::Main;
    ///         }
    ///         // ... modal-specific keys ...
    ///     }
    ///     true
    /// }
    /// ```
    fn handle_modal_key(&mut self, _key: &KeyCode) -> bool {
        true
    }

    /// Check if confirmation dialog is active
    ///
    /// # Example
    ///
    /// ```ignore
    /// fn has_confirm(&self) -> bool {
    ///     self.confirm.is_active()
    /// }
    /// ```
    fn has_confirm(&self) -> bool {
        false
    }

    /// Handle key in confirmation mode
    ///
    /// # Example
    ///
    /// ```ignore
    /// fn handle_confirm_key(&mut self, key: &KeyCode) -> bool {
    ///     match key {
    ///         KeyCode::Char('y') | KeyCode::Enter => {
    ///             self.confirm.execute(|action| self.do_action(action));
    ///         }
    ///         _ => self.confirm.cancel(),
    ///     }
    ///     true
    /// }
    /// ```
    fn handle_confirm_key(&mut self, _key: &KeyCode) -> bool {
        true
    }

    /// Check if popup is active
    ///
    /// Popups are temporary overlays like search or filter palettes.
    ///
    /// # Example
    ///
    /// ```ignore
    /// fn has_popup(&self) -> bool {
    ///     self.search_active || self.filter_active
    /// }
    /// ```
    fn has_popup(&self) -> bool {
        false
    }

    /// Handle key in popup mode
    ///
    /// # Example
    ///
    /// ```ignore
    /// fn handle_popup_key(&mut self, key: &KeyCode) -> bool {
    ///     if self.search_active {
    ///         return self.handle_search_key(key);
    ///     }
    ///     true
    /// }
    /// ```
    fn handle_popup_key(&mut self, _key: &KeyCode) -> bool {
        true
    }

    /// Handle key in normal view mode
    ///
    /// This is where main navigation and actions happen.
    ///
    /// # Example
    ///
    /// ```ignore
    /// fn handle_view_key(&mut self, key: &KeyCode) -> bool {
    ///     match self.view_mode {
    ///         ViewMode::Main => match key {
    ///             KeyCode::Char('j') => self.next(),
    ///             KeyCode::Char('k') => self.prev(),
    ///             KeyCode::Enter => self.open_detail(),
    ///             // ...
    ///         }
    ///         ViewMode::Detail => {
    ///             // ... detail keys ...
    ///         }
    ///     }
    ///     true
    /// }
    /// ```
    fn handle_view_key(&mut self, key: &KeyCode) -> bool;
}

/// Common key patterns
///
/// Helper functions for common key checks.
pub mod common {
    use crossterm::event::KeyCode;

    /// Check if key is a navigation key (j/k/up/down)
    pub fn is_nav_key(key: &KeyCode) -> bool {
        matches!(
            key,
            KeyCode::Char('j')
                | KeyCode::Char('k')
                | KeyCode::Up
                | KeyCode::Down
                | KeyCode::Char('g')
                | KeyCode::Char('G')
        )
    }

    /// Check if key is horizontal navigation (h/l/left/right)
    pub fn is_horizontal_nav(key: &KeyCode) -> bool {
        matches!(
            key,
            KeyCode::Char('h') | KeyCode::Char('l') | KeyCode::Left | KeyCode::Right
        )
    }

    /// Check if key is a confirm key (y/Y/Enter)
    pub fn is_confirm_key(key: &KeyCode) -> bool {
        matches!(
            key,
            KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter
        )
    }

    /// Check if key is a cancel key (n/N/Esc)
    pub fn is_cancel_key(key: &KeyCode) -> bool {
        matches!(key, KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc)
    }

    /// Check if key is a quit key (q/Q)
    pub fn is_quit_key(key: &KeyCode) -> bool {
        matches!(key, KeyCode::Char('q') | KeyCode::Char('Q'))
    }

    /// Check if key is help (?)
    pub fn is_help_key(key: &KeyCode) -> bool {
        matches!(key, KeyCode::Char('?'))
    }

    /// Check if key is refresh (r/R)
    pub fn is_refresh_key(key: &KeyCode) -> bool {
        matches!(key, KeyCode::Char('r') | KeyCode::Char('R'))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    struct TestApp {
        in_modal: bool,
        has_confirm: bool,
        has_popup: bool,
        quit: bool,
        last_key: Option<KeyCode>,
    }

    impl KeyHandler for TestApp {
        fn should_quit(&self, key: &KeyCode) -> bool {
            matches!(key, KeyCode::Char('q'))
        }

        fn in_modal(&self) -> bool {
            self.in_modal
        }

        fn handle_modal_key(&mut self, key: &KeyCode) -> bool {
            self.last_key = Some(*key);
            true
        }

        fn has_confirm(&self) -> bool {
            self.has_confirm
        }

        fn handle_confirm_key(&mut self, key: &KeyCode) -> bool {
            self.last_key = Some(*key);
            true
        }

        fn has_popup(&self) -> bool {
            self.has_popup
        }

        fn handle_popup_key(&mut self, key: &KeyCode) -> bool {
            self.last_key = Some(*key);
            true
        }

        fn handle_view_key(&mut self, key: &KeyCode) -> bool {
            self.last_key = Some(*key);
            true
        }
    }

    #[test]
    fn test_quit() {
        let mut app = TestApp {
            in_modal: false,
            has_confirm: false,
            has_popup: false,
            quit: false,
            last_key: None,
        };

        assert!(!app.handle_key(&KeyCode::Char('q')));
    }

    #[test]
    fn test_modal_priority() {
        let mut app = TestApp {
            in_modal: true,
            has_confirm: true,
            has_popup: true,
            quit: false,
            last_key: None,
        };

        app.handle_key(&KeyCode::Char('j'));
        assert_eq!(app.last_key, Some(KeyCode::Char('j')));
    }

    #[test]
    fn test_key_helpers() {
        use common::*;

        assert!(is_nav_key(&KeyCode::Char('j')));
        assert!(is_nav_key(&KeyCode::Char('k')));
        assert!(!is_nav_key(&KeyCode::Char('a')));

        assert!(is_confirm_key(&KeyCode::Char('y')));
        assert!(is_confirm_key(&KeyCode::Enter));

        assert!(is_cancel_key(&KeyCode::Esc));
        assert!(is_cancel_key(&KeyCode::Char('n')));
    }
}
