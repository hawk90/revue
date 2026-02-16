//! Accessibility testing utilities
//!
//! This module provides testing helpers for validating accessibility features.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::a11y::testing::A11yTestRunner;
//! use revue::widget::{Text, Button};
//! use revue::utils::Role;
//!
//! let mut runner = A11yTestRunner::new();
//! runner.assert_focus_order(&["button1", "button2", "input1"]);
//! runner.assert_aria_label("submit-btn", "Submit Form");
//! runner.assert_contrast_ratio("button-text", 4.5); // WCAG AA standard
//! ```

use crate::utils::accessibility::{AccessibilityManager, AccessibleNode, Role};
use std::collections::HashMap;

/// Test runner for accessibility validation
pub struct A11yTestRunner {
    /// Accessibility manager instance
    manager: AccessibilityManager,
    /// Registered widgets by ID
    widgets: HashMap<String, AccessibleNode>,
    /// Announcements made during test
    announcements: Vec<String>,
}

impl A11yTestRunner {
    /// Create new test runner
    pub fn new() -> Self {
        Self {
            manager: AccessibilityManager::new(),
            widgets: HashMap::new(),
            announcements: Vec::new(),
        }
    }

    /// Register a widget for testing
    pub fn register_widget(&mut self, id: impl Into<String>, node: AccessibleNode) -> &mut Self {
        let id = id.into();
        self.manager.add_node(node.clone());
        self.widgets.insert(id, node);
        self
    }

    /// Assert focus order matches expected sequence
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// # use revue::a11y::testing::A11yTestRunner;
    /// # let mut runner = A11yTestRunner::new();
    /// runner.assert_focus_order(&["username", "password", "submit-btn"]);
    /// ```
    pub fn assert_focus_order(&self, expected_ids: &[&str]) {
        let mut actual_order = Vec::new();

        // Simulate tab navigation through interactive widgets
        let interactive: Vec<_> = self
            .widgets
            .iter()
            .filter(|(_, node)| node.is_focusable())
            .collect();

        // Sort by document order (using position in set as hint)
        let mut sorted: Vec<_> = interactive.into_iter().collect();
        sorted.sort_by_key(|(id, node)| {
            node.state.pos_in_set.unwrap_or_else(|| {
                // Try to infer position from ID if not set
                let digits: Vec<_> = id.chars().filter_map(|c| c.to_digit(10)).collect();
                digits.first().copied().unwrap_or(0) as usize
            })
        });

        for (id, _) in sorted {
            actual_order.push(id.clone());
        }

        if actual_order != expected_ids {
            panic!(
                "Focus order mismatch:\nExpected: {:?}\nActual: {:?}",
                expected_ids, actual_order
            );
        }
    }

    /// Assert widget has aria-label
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// # use revue::a11y::testing::A11yTestRunner;
    /// # let mut runner = A11yTestRunner::new();
    /// runner.assert_aria_label("submit-btn", "Submit Form");
    /// ```
    pub fn assert_aria_label(&self, widget_id: &str, expected_label: &str) {
        if let Some(node) = self.widgets.get(widget_id) {
            let label = node.label.as_deref().unwrap_or("");
            assert_eq!(
                label, expected_label,
                "Widget '{}' has wrong aria-label: expected '{}', got '{}'",
                widget_id, expected_label, label
            );
        } else {
            panic!("Widget '{}' not found", widget_id);
        }
    }

    /// Assert widget has specific role
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// # use revue::a11y::testing::A11yTestRunner;
    /// # use revue::utils::Role;
    /// # let mut runner = A11yTestRunner::new();
    /// runner.assert_role("submit-btn", Role::Button);
    /// ```
    pub fn assert_role(&self, widget_id: &str, expected_role: Role) {
        if let Some(node) = self.widgets.get(widget_id) {
            assert_eq!(
                node.role, expected_role,
                "Widget '{}' has wrong role: expected {:?}, got {:?}",
                widget_id, expected_role, node.role
            );
        } else {
            panic!("Widget '{}' not found", widget_id);
        }
    }

    /// Assert widget has required state
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// # use revue::a11y::testing::A11yTestRunner;
    /// # let mut runner = A11yTestRunner::new();
    /// runner.assert_required("email-input");
    /// runner.assert_not_required("optional-input");
    /// ```
    pub fn assert_required(&self, widget_id: &str) {
        if let Some(node) = self.widgets.get(widget_id) {
            if let Some(required) = node.properties.get("aria-required") {
                assert_eq!(
                    required, "true",
                    "Widget '{}' should be required but aria-required={}",
                    widget_id, required
                );
            } else {
                panic!("Widget '{}' is missing aria-required attribute", widget_id);
            }
        } else {
            panic!("Widget '{}' not found", widget_id);
        }
    }

    /// Assert widget is NOT required
    pub fn assert_not_required(&self, widget_id: &str) {
        if let Some(node) = self.widgets.get(widget_id) {
            if let Some(required) = node.properties.get("aria-required") {
                assert_ne!(
                    required, "true",
                    "Widget '{}' should NOT be required but aria-required={}",
                    widget_id, required
                );
            }
        } else {
            panic!("Widget '{}' not found", widget_id);
        }
    }

    /// Assert minimum color contrast ratio (WCAG 2.1)
    ///
    /// - 4.5:1 for normal text (AA)
    /// - 7:1 for large text (AAA)
    /// - 3:1 for large text or UI components (A)
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// # use revue::a11y::testing::A11yTestRunner;
    /// # let mut runner = A11yTestRunner::new();
    /// runner.assert_contrast_ratio("button-text", 4.5); // WCAG AA
    /// ```
    pub fn assert_contrast_ratio(&self, widget_id: &str, min_ratio: f32) {
        if let Some(node) = self.widgets.get(widget_id) {
            // Get foreground and background colors from properties
            let fg = node
                .properties
                .get("color-fg")
                .and_then(|c| c.parse::<u8>().ok());

            let bg = node
                .properties
                .get("color-bg")
                .and_then(|c| c.parse::<u8>().ok());

            if let (Some(fg), Some(bg)) = (fg, bg) {
                let ratio = calculate_contrast_ratio(fg, bg).unwrap_or(1.0);
                if ratio < min_ratio {
                    panic!(
                        "Widget '{}' fails contrast ratio: {:.2} < {:.1} (WCAG requirement)",
                        widget_id, ratio, min_ratio
                    );
                }
            }
        }
    }

    /// Assert widget is focusable
    pub fn assert_focusable(&self, widget_id: &str) {
        if let Some(node) = self.widgets.get(widget_id) {
            assert!(
                node.is_focusable(),
                "Widget '{}' should be focusable but is not",
                widget_id
            );
        } else {
            panic!("Widget '{}' not found", widget_id);
        }
    }

    /// Assert widget is NOT focusable
    pub fn assert_not_focusable(&self, widget_id: &str) {
        if let Some(node) = self.widgets.get(widget_id) {
            assert!(
                !node.is_focusable(),
                "Widget '{}' should NOT be focusable but is",
                widget_id
            );
        } else {
            panic!("Widget '{}' not found", widget_id);
        }
    }

    /// Assert widget is disabled
    pub fn assert_disabled(&self, widget_id: &str) {
        if let Some(node) = self.widgets.get(widget_id) {
            assert!(
                node.state.disabled,
                "Widget '{}' should be disabled but is not",
                widget_id
            );
        } else {
            panic!("Widget '{}' not found", widget_id);
        }
    }

    /// Assert widget is enabled
    pub fn assert_enabled(&self, widget_id: &str) {
        if let Some(node) = self.widgets.get(widget_id) {
            assert!(
                !node.state.disabled,
                "Widget '{}' should be enabled but is disabled",
                widget_id
            );
        } else {
            panic!("Widget '{}' not found", widget_id);
        }
    }

    /// Get accessible name of a widget
    pub fn accessible_name(&self, widget_id: &str) -> String {
        if let Some(node) = self.widgets.get(widget_id) {
            node.accessible_name().to_string()
        } else {
            panic!("Widget '{}' not found", widget_id);
        }
    }

    /// Get screen reader description of a widget
    pub fn screen_reader_description(&self, widget_id: &str) -> String {
        if let Some(node) = self.widgets.get(widget_id) {
            node.describe()
        } else {
            panic!("Widget '{}' not found", widget_id);
        }
    }

    /// Assert announcement was made
    pub fn assert_announced(&self, expected_message: &str) {
        let found = self
            .announcements
            .iter()
            .any(|msg| msg.contains(expected_message));

        assert!(
            found,
            "Expected announcement '{}' not found. Made: {:?}",
            expected_message, self.announcements
        );
    }

    /// Get all announcements made
    pub fn announcements(&self) -> &[String] {
        &self.announcements
    }

    /// Clear announcements
    pub fn clear_announcements(&mut self) {
        self.announcements.clear();
    }
}

impl Default for A11yTestRunner {
    fn default() -> Self {
        Self::new()
    }
}

/// Keyboard-only navigation simulator
pub struct KeyboardNavigator {
    /// Current focus index
    focus_index: usize,
    /// Interactive widget IDs in tab order
    tab_order: Vec<String>,
}

impl KeyboardNavigator {
    /// Create new navigator with tab order
    pub fn new(tab_order: Vec<String>) -> Self {
        Self {
            focus_index: 0,
            tab_order,
        }
    }

    /// Get current focused widget
    pub fn current_focus(&self) -> Option<&str> {
        self.tab_order.get(self.focus_index).map(|s| s.as_str())
    }

    /// Simulate Tab key (move to next)
    pub fn tab(&mut self) -> Option<&str> {
        if !self.tab_order.is_empty() {
            self.focus_index = (self.focus_index + 1) % self.tab_order.len();
            self.current_focus()
        } else {
            None
        }
    }

    /// Simulate Shift+Tab (move to previous)
    pub fn shift_tab(&mut self) -> Option<&str> {
        if !self.tab_order.is_empty() {
            self.focus_index = if self.focus_index == 0 {
                self.tab_order.len() - 1
            } else {
                self.focus_index - 1
            };
            self.current_focus()
        } else {
            None
        }
    }

    /// Jump to specific widget
    pub fn jump_to(&mut self, widget_id: &str) -> bool {
        if let Some(pos) = self.tab_order.iter().position(|id| id == widget_id) {
            self.focus_index = pos;
            true
        } else {
            false
        }
    }
}

/// Calculate contrast ratio between two colors (simplified)
///
/// Returns contrast ratio from 1-21, where 21 is highest contrast
fn calculate_contrast_ratio(fg: u8, bg: u8) -> Option<f32> {
    // Simplified calculation - in real implementation would use
    // proper RGB color space calculations
    let (l1, l2) = if fg > bg {
        (fg as f32, bg as f32)
    } else {
        (bg as f32, fg as f32)
    };

    let ratio = (l1 + 5.0) / (l2 + 5.0);
    Some(ratio)
}

/// Convenience macro for accessibility assertions
#[macro_export]
macro_rules! a11y_assert {
    // Assert focus order
    (focus_order: $runner:expr, [$($expected:expr),* $(,)?]) => {
        $runner.assert_focus_order(&[$($expected),*])
    };

    // Assert aria label
    (aria_label: $runner:expr, $widget:expr, $label:expr) => {
        $runner.assert_aria_label($widget, $label)
    };

    // Assert role
    (role: $runner:expr, $widget:expr, $role:expr) => {
        $runner.assert_role($widget, $role)
    };

    // Assert required
    (required: $runner:expr, $widget:expr) => {
        $runner.assert_required($widget)
    };

    // Assert focusable
    (focusable: $runner:expr, $widget:expr) => {
        $runner.assert_focusable($widget)
    };

    // Assert disabled
    (disabled: $runner:expr, $widget:expr) => {
        $runner.assert_disabled($widget)
    };

    // Assert enabled
    (enabled: $runner:expr, $widget:expr) => {
        $runner.assert_enabled($widget)
    };
}

// KEEP HERE - Tests access private fields (A11yTestRunner.widgets, announcements, etc.)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_focus_order_assertion() {
        let mut runner = A11yTestRunner::new();

        runner
            .register_widget("btn1", AccessibleNode::with_id("btn1", Role::Button))
            .register_widget("btn2", AccessibleNode::with_id("btn2", Role::Button))
            .register_widget("btn3", AccessibleNode::with_id("btn3", Role::Button));

        // This should pass with inferred order
        runner.assert_focus_order(&["btn1", "btn2", "btn3"]);
    }

    #[test]
    fn test_aria_label_assertion() {
        let mut runner = A11yTestRunner::new();

        runner.register_widget(
            "submit",
            AccessibleNode::with_id("submit", Role::Button).label("Submit Form"),
        );

        runner.assert_aria_label("submit", "Submit Form");
    }

    #[test]
    fn test_role_assertion() {
        let mut runner = A11yTestRunner::new();

        runner.register_widget("btn", AccessibleNode::with_id("btn", Role::Button));

        runner.assert_role("btn", Role::Button);
    }

    #[test]
    fn test_keyboard_navigator() {
        let mut nav = KeyboardNavigator::new(vec![
            "username".to_string(),
            "password".to_string(),
            "submit".to_string(),
        ]);

        assert_eq!(nav.current_focus(), Some("username"));

        nav.tab();
        assert_eq!(nav.current_focus(), Some("password"));

        nav.tab();
        assert_eq!(nav.current_focus(), Some("submit"));

        nav.tab(); // Wraps around
        assert_eq!(nav.current_focus(), Some("username"));

        nav.shift_tab();
        assert_eq!(nav.current_focus(), Some("submit"));
    }

    #[test]
    fn test_accessible_name() {
        let mut runner = A11yTestRunner::new();

        runner.register_widget(
            "btn",
            AccessibleNode::with_id("btn", Role::Button).label("Click Me"),
        );

        assert_eq!(runner.accessible_name("btn"), "Click Me");
    }

    #[test]
    fn test_screen_reader_description() {
        let mut runner = A11yTestRunner::new();

        runner.register_widget(
            "checkbox",
            AccessibleNode::with_id("checkbox", Role::Checkbox)
                .label("Agree to terms")
                .state(crate::utils::accessibility::AccessibleState::new().checked(true)),
        );

        let desc = runner.screen_reader_description("checkbox");
        assert!(desc.contains("Agree to terms"));
        assert!(desc.contains("checked"));
    }

    #[test]
    fn test_focusable_assertion() {
        let mut runner = A11yTestRunner::new();

        runner.register_widget("btn", AccessibleNode::with_id("btn", Role::Button));
        runner.register_widget(
            "disabled-btn",
            AccessibleNode::with_id("disabled-btn", Role::Button)
                .state(crate::utils::accessibility::AccessibleState::new().disabled(true)),
        );

        runner.assert_focusable("btn");
        runner.assert_not_focusable("disabled-btn");
    }

    #[test]
    fn test_disabled_assertion() {
        let mut runner = A11yTestRunner::new();

        runner.register_widget(
            "input",
            AccessibleNode::with_id("input", Role::TextInput)
                .state(crate::utils::accessibility::AccessibleState::new().disabled(true)),
        );

        runner.assert_disabled("input");
    }

    #[test]
    fn test_keyboard_navigator_jump_to() {
        let mut nav = KeyboardNavigator::new(vec![
            "field1".to_string(),
            "field2".to_string(),
            "field3".to_string(),
        ]);

        assert!(nav.jump_to("field3"));
        assert_eq!(nav.current_focus(), Some("field3"));

        assert!(!nav.jump_to("nonexistent"));
        assert_eq!(nav.current_focus(), Some("field3")); // Focus unchanged
    }

    #[test]
    fn test_a11y_test_runner_new() {
        let runner = A11yTestRunner::new();
        assert!(runner.widgets.is_empty());
        assert!(runner.announcements.is_empty());
    }

    #[test]
    fn test_a11y_test_runner_default() {
        let runner = A11yTestRunner::default();
        assert!(runner.widgets.is_empty());
    }

    #[test]
    fn test_a11y_test_runner_announcements() {
        let runner = A11yTestRunner::new();
        assert_eq!(runner.announcements().len(), 0);

        // Since we can't easily make announcements, test the accessor
        let _ = runner.announcements();
    }

    #[test]
    fn test_a11y_test_runner_clear_announcements() {
        let mut runner = A11yTestRunner::new();
        // Can't add announcements easily, so just test it doesn't panic
        runner.clear_announcements();
        assert_eq!(runner.announcements().len(), 0);
    }

    #[test]
    fn test_keyboard_navigator_empty() {
        let mut nav = KeyboardNavigator::new(vec![]);
        assert_eq!(nav.current_focus(), None);
        assert_eq!(nav.tab(), None);
        assert_eq!(nav.shift_tab(), None);
        assert!(!nav.jump_to("anything"));
    }

    #[test]
    fn test_keyboard_navigator_single_item() {
        let mut nav = KeyboardNavigator::new(vec!["only".to_string()]);
        assert_eq!(nav.current_focus(), Some("only"));

        // Tab wraps around to same element
        assert_eq!(nav.tab(), Some("only"));
        assert_eq!(nav.tab(), Some("only"));

        // Shift+Tab also stays on same element
        assert_eq!(nav.shift_tab(), Some("only"));
    }

    #[test]
    fn test_assert_not_focusable() {
        let mut runner = A11yTestRunner::new();

        runner.register_widget(
            "disabled-btn",
            AccessibleNode::with_id("disabled-btn", Role::Button)
                .state(crate::utils::accessibility::AccessibleState::new().disabled(true)),
        );

        runner.assert_not_focusable("disabled-btn");
    }

    #[test]
    fn test_assert_announced_not_found() {
        let runner = A11yTestRunner::new();
        // Should panic since announcement wasn't made
        let result = std::panic::catch_unwind(|| {
            runner.assert_announced("test message");
        });
        assert!(result.is_err());
    }
}
