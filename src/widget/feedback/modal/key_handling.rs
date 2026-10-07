//! Button selection and key handling, with focus-trap release

use super::Modal;
use crate::event::FocusManager;

impl Modal {
    /// Get selected button index
    pub fn selected_button(&self) -> usize {
        self.selected_button
    }

    /// Select next button
    pub fn next_button(&mut self) {
        if !self.buttons.is_empty() {
            self.selected_button = (self.selected_button + 1) % self.buttons.len();
        }
    }

    /// Select previous button
    pub fn prev_button(&mut self) {
        if !self.buttons.is_empty() {
            self.selected_button = self
                .selected_button
                .checked_sub(1)
                .unwrap_or(self.buttons.len() - 1);
        }
    }

    /// Handle key input, returns Some(button_index) if button confirmed
    pub fn handle_key(&mut self, key: &crate::event::Key) -> Option<usize> {
        use crate::event::Key;

        match key {
            Key::Enter | Key::Char(' ') => {
                if !self.buttons.is_empty() {
                    Some(self.selected_button)
                } else {
                    None
                }
            }
            Key::Left | Key::Char('h') => {
                self.prev_button();
                None
            }
            Key::Right | Key::Char('l') => {
                self.next_button();
                None
            }
            Key::Tab => {
                self.next_button();
                None
            }
            Key::Escape => {
                self.hide();
                None
            }
            _ => None,
        }
    }

    /// Handle key input with focus manager integration
    ///
    /// Like `handle_key`, but also releases the focus trap on Escape
    /// or when a button is confirmed.
    pub fn handle_key_with_focus(
        &mut self,
        key: &crate::event::Key,
        fm: &mut FocusManager,
    ) -> Option<usize> {
        let result = self.handle_key(key);

        // Release focus trap on escape or button confirm
        match key {
            crate::event::Key::Escape => {
                if let Some(mut trap) = self.focus_trap.take() {
                    trap.deactivate(fm);
                }
            }
            crate::event::Key::Enter | crate::event::Key::Char(' ') if result.is_some() => {
                if let Some(mut trap) = self.focus_trap.take() {
                    trap.deactivate(fm);
                }
            }
            _ => {}
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modal_button_navigation() {
        let mut m = Modal::new().ok_cancel();

        assert_eq!(m.selected_button(), 0);

        m.next_button();
        assert_eq!(m.selected_button(), 1);

        m.next_button(); // Wraps around
        assert_eq!(m.selected_button(), 0);

        m.prev_button(); // Wraps around
        assert_eq!(m.selected_button(), 1);
    }

    #[test]
    fn test_modal_handle_key() {
        use crate::event::Key;

        let mut m = Modal::new().yes_no();
        m.show();

        // Navigate buttons
        m.handle_key(&Key::Right);
        assert_eq!(m.selected_button(), 1);

        m.handle_key(&Key::Left);
        assert_eq!(m.selected_button(), 0);

        // Confirm selection
        let result = m.handle_key(&Key::Enter);
        assert_eq!(result, Some(0));

        // Escape closes
        m.handle_key(&Key::Escape);
        assert!(!m.is_visible());
    }

    #[test]
    fn test_modal_selected_button_initial() {
        let m = Modal::new();
        assert_eq!(m.selected_button(), 0);
    }

    #[test]
    fn test_modal_next_button_empty() {
        let mut m = Modal::new();
        m.next_button(); // Should not panic
        assert_eq!(m.selected_button(), 0);
    }

    #[test]
    fn test_modal_prev_button_empty() {
        let mut m = Modal::new();
        m.prev_button(); // Should not panic
        assert_eq!(m.selected_button(), 0);
    }

    #[test]
    fn test_modal_handle_key_no_buttons() {
        use crate::event::Key;

        let mut m = Modal::new();
        let result = m.handle_key(&Key::Enter);
        assert_eq!(result, None);
    }

    #[test]
    fn test_modal_handle_key_unknown() {
        use crate::event::Key;

        let mut m = Modal::new().ok();
        let result = m.handle_key(&Key::Char('x'));
        assert_eq!(result, None);
    }

    #[test]
    fn test_modal_handle_key_with_focus_escape() {
        use crate::event::Key;

        let mut fm = FocusManager::new();
        fm.register(1);
        fm.register(10);
        fm.register(11);
        fm.focus(1);

        let mut m = Modal::new().ok_cancel();
        m.show_with_focus_trap(&mut fm, 100, &[10, 11]);

        // Escape should hide modal and release trap
        m.handle_key_with_focus(&Key::Escape, &mut fm);
        assert!(!m.is_visible());
        assert!(!fm.is_trapped());
        assert_eq!(fm.current(), Some(1));
    }

    #[test]
    fn test_modal_handle_key_with_focus_confirm() {
        use crate::event::Key;

        let mut fm = FocusManager::new();
        fm.register(1);
        fm.register(10);
        fm.register(11);
        fm.focus(1);

        let mut m = Modal::new().ok_cancel();
        m.show_with_focus_trap(&mut fm, 100, &[10, 11]);

        // Enter confirms and releases trap
        let result = m.handle_key_with_focus(&Key::Enter, &mut fm);
        assert_eq!(result, Some(0));
        assert!(!fm.is_trapped());
    }

    #[test]
    fn test_modal_focus_trap_tab_does_not_release() {
        use crate::event::Key;

        let mut fm = FocusManager::new();
        fm.register(1);
        fm.register(10);
        fm.register(11);
        fm.focus(1);

        let mut m = Modal::new().ok_cancel();
        m.show_with_focus_trap(&mut fm, 100, &[10, 11]);

        // Tab should navigate buttons but keep trap active
        m.handle_key_with_focus(&Key::Tab, &mut fm);
        assert!(m.has_focus_trap());
        assert!(fm.is_trapped());
    }
}
