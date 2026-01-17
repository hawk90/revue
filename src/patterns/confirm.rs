//! Confirmation dialog pattern
//!
//! Provides a simple state machine for confirmation dialogs.
//! Prevents accidental destructive actions by requiring explicit confirmation.
//!
//! # Example
//!
//! ```ignore
//! use revue::patterns::{ConfirmAction, ConfirmState};
//!
//! struct App {
//!     confirm: ConfirmState,
//!     items: Vec<Item>,
//! }
//!
//! impl App {
//!     fn handle_key(&mut self, key: &Key) -> bool {
//!         // Confirm dialog takes priority
//!         if self.confirm.is_active() {
//!             match key {
//!                 Key::Char('y') | Key::Enter => {
//!                     self.confirm.execute(|action| {
//!                         match action {
//!                             ConfirmAction::Delete => self.delete_item(),
//!                             ConfirmAction::Custom(msg) => self.custom_action(msg),
//!                         }
//!                     });
//!                 }
//!                 _ => self.confirm.cancel(),
//!             }
//!             return true;
//!         }
//!
//!         // Normal key handling
//!         match key {
//!             Key::Char('d') => {
//!                 self.confirm.request(ConfirmAction::Delete);
//!             }
//!             // ...
//!         }
//!         true
//!     }
//!
//!     fn render_footer(&self, ctx: &mut RenderContext) {
//!         if let Some(msg) = self.confirm.message() {
//!             ctx.draw_text(0, 0, msg, YELLOW);
//!             return;
//!         }
//!         // ... render normal footer ...
//!     }
//! }
//! ```

/// Common confirmation actions
///
/// You can extend this with your own action types or use `Custom` for one-off cases.
#[derive(Clone, Debug, PartialEq)]
pub enum ConfirmAction {
    /// Delete confirmation
    Delete,
    /// Open in browser confirmation
    OpenBrowser,
    /// Restart confirmation
    Restart,
    /// Abort/Cancel confirmation
    Abort,
    /// Custom action with message
    Custom(String),
}

impl ConfirmAction {
    /// Get the confirmation message for this action
    ///
    /// # Example
    ///
    /// ```ignore
    /// let action = ConfirmAction::Delete;
    /// assert_eq!(action.message(), "Delete? (y/n)");
    /// ```
    pub fn message(&self) -> &str {
        match self {
            ConfirmAction::Delete => "Delete? (y/n)",
            ConfirmAction::OpenBrowser => "Open in browser? (y/n)",
            ConfirmAction::Restart => "Restart? (y/n)",
            ConfirmAction::Abort => "Abort? (y/n)",
            ConfirmAction::Custom(msg) => msg,
        }
    }
}

/// Confirmation dialog state
///
/// Manages the current confirmation action (if any).
#[derive(Clone, Debug, Default)]
pub struct ConfirmState {
    action: Option<ConfirmAction>,
}

impl ConfirmState {
    /// Create a new confirm state (no action pending)
    pub fn new() -> Self {
        Self { action: None }
    }

    /// Request confirmation for an action
    ///
    /// # Example
    ///
    /// ```ignore
    /// app.confirm.request(ConfirmAction::Delete);
    /// ```
    pub fn request(&mut self, action: ConfirmAction) {
        self.action = Some(action);
    }

    /// Check if confirmation is active
    pub fn is_active(&self) -> bool {
        self.action.is_some()
    }

    /// Get current action (if any)
    pub fn get(&self) -> Option<&ConfirmAction> {
        self.action.as_ref()
    }

    /// Get confirmation message
    pub fn message(&self) -> Option<&str> {
        self.action.as_ref().map(|a| a.message())
    }

    /// Cancel confirmation (don't execute action)
    ///
    /// # Example
    ///
    /// ```ignore
    /// match key {
    ///     Key::Char('n') | Key::Esc => app.confirm.cancel(),
    ///     // ...
    /// }
    /// ```
    pub fn cancel(&mut self) {
        self.action = None;
    }

    /// Execute the confirmation action
    ///
    /// Takes the action and passes it to the provided closure.
    /// Clears the confirmation state after execution.
    ///
    /// # Example
    ///
    /// ```ignore
    /// self.confirm.execute(|action| {
    ///     match action {
    ///         ConfirmAction::Delete => self.delete_item(),
    ///         ConfirmAction::OpenBrowser => self.open_browser(),
    ///         ConfirmAction::Custom(msg) => println!("{}", msg),
    ///     }
    /// });
    /// ```
    pub fn execute<F>(&mut self, f: F)
    where
        F: FnOnce(ConfirmAction),
    {
        if let Some(action) = self.action.take() {
            f(action);
        }
    }

    /// Execute with explicit action check
    ///
    /// Similar to `execute`, but provides the action as a reference first.
    /// Useful when you need to check the action before committing.
    ///
    /// # Example
    ///
    /// ```ignore
    /// self.confirm.execute_if(|action| {
    ///     if let ConfirmAction::Delete = action {
    ///         // Pre-check passed
    ///         true
    ///     } else {
    ///         false
    ///     }
    /// }, |action| {
    ///     // Actually execute
    ///     self.handle_action(action);
    /// });
    /// ```
    pub fn execute_if<P, F>(&mut self, predicate: P, f: F)
    where
        P: FnOnce(&ConfirmAction) -> bool,
        F: FnOnce(ConfirmAction),
    {
        if let Some(action) = &self.action {
            if predicate(action) {
                let action = self.action.take().unwrap();
                f(action);
            }
        }
    }

    /// Clear confirmation state
    pub fn clear(&mut self) {
        self.action = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_confirm_action_message() {
        assert_eq!(ConfirmAction::Delete.message(), "Delete? (y/n)");
        assert_eq!(
            ConfirmAction::OpenBrowser.message(),
            "Open in browser? (y/n)"
        );
        assert_eq!(
            ConfirmAction::Custom("Really?".to_string()).message(),
            "Really?"
        );
    }

    #[test]
    fn test_confirm_state() {
        let mut confirm = ConfirmState::new();
        assert!(!confirm.is_active());

        confirm.request(ConfirmAction::Delete);
        assert!(confirm.is_active());
        assert_eq!(confirm.message(), Some("Delete? (y/n)"));

        confirm.cancel();
        assert!(!confirm.is_active());
    }

    #[test]
    fn test_execute() {
        let mut confirm = ConfirmState::new();
        confirm.request(ConfirmAction::Delete);

        let mut executed = false;
        confirm.execute(|action| {
            assert_eq!(action, ConfirmAction::Delete);
            executed = true;
        });

        assert!(executed);
        assert!(!confirm.is_active());
    }

    #[test]
    fn test_execute_if() {
        let mut confirm = ConfirmState::new();
        confirm.request(ConfirmAction::Delete);

        let mut executed = false;
        confirm.execute_if(
            |action| matches!(action, ConfirmAction::Delete),
            |_| executed = true,
        );

        assert!(executed);
        assert!(!confirm.is_active());
    }

    #[test]
    fn test_execute_if_rejects() {
        let mut confirm = ConfirmState::new();
        confirm.request(ConfirmAction::Delete);

        let mut executed = false;
        confirm.execute_if(
            |action| matches!(action, ConfirmAction::OpenBrowser),
            |_| executed = true,
        );

        assert!(!executed);
        assert!(confirm.is_active()); // Should still be active
    }

    #[test]
    fn test_confirm_action_clone() {
        let action = ConfirmAction::Delete;
        let cloned = action.clone();
        assert_eq!(action, cloned);
    }

    #[test]
    fn test_confirm_action_debug() {
        let action = ConfirmAction::Restart;
        let debug = format!("{:?}", action);
        assert!(debug.contains("Restart"));
    }

    #[test]
    fn test_confirm_action_all_messages() {
        assert_eq!(ConfirmAction::Delete.message(), "Delete? (y/n)");
        assert_eq!(
            ConfirmAction::OpenBrowser.message(),
            "Open in browser? (y/n)"
        );
        assert_eq!(ConfirmAction::Restart.message(), "Restart? (y/n)");
        assert_eq!(ConfirmAction::Abort.message(), "Abort? (y/n)");
        assert_eq!(
            ConfirmAction::Custom("Custom message".to_string()).message(),
            "Custom message"
        );
    }

    #[test]
    fn test_confirm_action_eq() {
        assert_eq!(ConfirmAction::Delete, ConfirmAction::Delete);
        assert_ne!(ConfirmAction::Delete, ConfirmAction::Restart);
        assert_eq!(
            ConfirmAction::Custom("a".to_string()),
            ConfirmAction::Custom("a".to_string())
        );
        assert_ne!(
            ConfirmAction::Custom("a".to_string()),
            ConfirmAction::Custom("b".to_string())
        );
    }

    #[test]
    fn test_confirm_state_default() {
        let confirm = ConfirmState::default();
        assert!(!confirm.is_active());
        assert!(confirm.get().is_none());
        assert!(confirm.message().is_none());
    }

    #[test]
    fn test_confirm_state_get() {
        let mut confirm = ConfirmState::new();
        assert!(confirm.get().is_none());

        confirm.request(ConfirmAction::Delete);
        let action = confirm.get();
        assert!(action.is_some());
        assert_eq!(action.unwrap(), &ConfirmAction::Delete);
    }

    #[test]
    fn test_confirm_state_clear() {
        let mut confirm = ConfirmState::new();
        confirm.request(ConfirmAction::Delete);
        assert!(confirm.is_active());

        confirm.clear();
        assert!(!confirm.is_active());
    }

    #[test]
    fn test_execute_no_action() {
        let mut confirm = ConfirmState::new();

        let mut executed = false;
        confirm.execute(|_| {
            executed = true;
        });

        assert!(!executed);
    }

    #[test]
    fn test_execute_if_no_action() {
        let mut confirm = ConfirmState::new();

        let mut predicate_called = false;
        let mut executed = false;
        confirm.execute_if(
            |_| {
                predicate_called = true;
                true
            },
            |_| executed = true,
        );

        assert!(!predicate_called);
        assert!(!executed);
    }

    #[test]
    fn test_confirm_state_clone() {
        let mut confirm = ConfirmState::new();
        confirm.request(ConfirmAction::OpenBrowser);

        let cloned = confirm.clone();
        assert!(cloned.is_active());
        assert_eq!(cloned.get(), Some(&ConfirmAction::OpenBrowser));
    }

    #[test]
    fn test_confirm_state_debug() {
        let mut confirm = ConfirmState::new();
        confirm.request(ConfirmAction::Abort);

        let debug = format!("{:?}", confirm);
        assert!(debug.contains("Abort"));
    }

    #[test]
    fn test_request_replaces_previous() {
        let mut confirm = ConfirmState::new();
        confirm.request(ConfirmAction::Delete);
        assert_eq!(confirm.get(), Some(&ConfirmAction::Delete));

        confirm.request(ConfirmAction::Restart);
        assert_eq!(confirm.get(), Some(&ConfirmAction::Restart));
    }

    #[test]
    fn test_execute_clears_state() {
        let mut confirm = ConfirmState::new();
        confirm.request(ConfirmAction::Delete);

        confirm.execute(|_| {});

        assert!(!confirm.is_active());
        assert!(confirm.get().is_none());
    }
}
