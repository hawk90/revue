//! Autocomplete widget for input suggestions
//!
//! Provides a text input with dropdown suggestions based on user input.

mod core;
mod helper;
mod types;

#[cfg(test)]
mod tests {
    use super::*;

    use crate::event::{Key, KeyEvent};
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::style::Color;
    use crate::utils::FilterMode;
    use crate::widget::traits::RenderContext;

    #[test]
    fn test_suggestion_new() {
        let s = Suggestion::new("test");
        assert_eq!(s.label, "test");
        assert_eq!(s.value, "test");
    }

    #[test]
    fn test_suggestion_with_value() {
        let s = Suggestion::with_value("Display", "actual_value");
        assert_eq!(s.label, "Display");
        assert_eq!(s.value, "actual_value");
    }

    #[test]
    fn test_autocomplete_new() {
        let ac = Autocomplete::new();
        assert_eq!(ac.get_value(), "");
        assert!(!ac.is_focused());
    }

    #[test]
    fn test_autocomplete_suggestions() {
        let mut ac = Autocomplete::new().suggestions(vec!["apple", "banana", "cherry"]);
        // Can't access private suggestions field, test through public API
        ac.set_value("apple");
        assert!(ac.selected_suggestion().is_some());
    }

    #[test]
    fn test_autocomplete_filter() {
        let mut ac = Autocomplete::new()
            .suggestions(vec!["apple", "apricot", "banana"])
            .min_chars(1);

        ac.set_value("ap");
        // Test that filtering works through public API
        assert!(ac.selected_suggestion().is_some());
    }

    #[test]
    fn test_autocomplete_accept() {
        let mut ac = Autocomplete::new().suggestions(vec!["apple", "banana"]);

        ac.set_value("a");
        assert!(ac.accept_selection());
        assert_eq!(ac.get_value(), "apple");
    }

    #[test]
    fn test_autocomplete_key_handling() {
        let mut ac = Autocomplete::new();
        ac.focus();

        ac.handle_key(KeyEvent::new(Key::Char('h')));
        ac.handle_key(KeyEvent::new(Key::Char('i')));
        assert_eq!(ac.get_value(), "hi");

        ac.handle_key(KeyEvent::new(Key::Backspace));
        assert_eq!(ac.get_value(), "h");
    }

    #[test]
    fn test_autocomplete_default() {
        let ac = Autocomplete::default();
        assert_eq!(ac.get_value(), "");
    }

    #[test]
    fn test_autocomplete_blur() {
        let mut ac = Autocomplete::new().suggestions(vec!["apple", "banana"]);

        ac.focus();
        ac.set_value("a");
        assert!(ac.is_focused());

        ac.blur();
        assert!(!ac.is_focused());
        // Can't test dropdown_visible directly
    }

    #[test]
    fn test_autocomplete_selected_suggestion_empty() {
        let ac = Autocomplete::new().suggestions(vec!["apple", "banana"]);
        // No filter applied yet
        assert!(ac.selected_suggestion().is_none());
    }

    #[test]
    fn test_autocomplete_accept_selection_empty() {
        let mut ac = Autocomplete::new();
        // No suggestions, should return false
        assert!(!ac.accept_selection());
    }

    #[test]
    fn test_autocomplete_min_chars() {
        let mut ac = Autocomplete::new()
            .suggestions(vec!["apple", "banana"])
            .min_chars(3);

        ac.set_value("ap");
        // Should not filter because min_chars is 3
        assert!(ac.selected_suggestion().is_none());

        ac.set_value("app");
        // Now should filter
        assert!(ac.selected_suggestion().is_some());
    }

    #[test]
    fn test_autocomplete_filter_mode_prefix() {
        let mut ac = Autocomplete::new()
            .suggestions(vec!["apple", "application", "banana"])
            .filter_mode(FilterMode::Prefix)
            .min_chars(1);

        ac.set_value("app");
        // Should have matches
        assert!(ac.selected_suggestion().is_some());
    }

    #[test]
    fn test_autocomplete_filter_mode_contains() {
        let mut ac = Autocomplete::new()
            .suggestions(vec!["apple", "pineapple", "banana"])
            .filter_mode(FilterMode::Contains)
            .min_chars(1);

        ac.set_value("apple");
        // Should have matches (apple and pineapple)
        assert!(ac.selected_suggestion().is_some());
    }

    #[test]
    fn test_autocomplete_filter_mode_exact() {
        let mut ac = Autocomplete::new()
            .suggestions(vec!["apple", "APPLE", "apples"])
            .filter_mode(FilterMode::Exact)
            .min_chars(1);

        ac.set_value("apple");
        // Should have matches (apple and APPLE - case insensitive)
        assert!(ac.selected_suggestion().is_some());
    }

    #[test]
    fn test_autocomplete_filter_mode_none() {
        let mut ac = Autocomplete::new()
            .suggestions(vec!["apple", "banana", "cherry"])
            .filter_mode(FilterMode::None)
            .min_chars(1);

        ac.set_value("xyz");
        // All suggestions shown despite filter
        assert!(ac.selected_suggestion().is_some());
    }

    #[test]
    fn test_autocomplete_handle_key_delete() {
        let mut ac = Autocomplete::new();
        ac.focus();

        ac.handle_key(KeyEvent::new(Key::Char('a')));
        ac.handle_key(KeyEvent::new(Key::Char('b')));
        ac.handle_key(KeyEvent::new(Key::Home));
        ac.handle_key(KeyEvent::new(Key::Delete));
        assert_eq!(ac.get_value(), "b");
    }

    #[test]
    fn test_autocomplete_handle_key_escape() {
        let mut ac = Autocomplete::new()
            .suggestions(vec!["a1", "a2"])
            .min_chars(1);

        ac.focus();
        ac.handle_key(KeyEvent::new(Key::Char('a')));
        // Can't test dropdown_visible directly, but ensure no panic
        ac.handle_key(KeyEvent::new(Key::Escape));
        assert!(ac.is_focused());
    }

    #[test]
    fn test_autocomplete_handle_key_tab() {
        let mut ac = Autocomplete::new()
            .suggestions(vec!["apple", "banana"])
            .min_chars(1);

        ac.focus();
        ac.handle_key(KeyEvent::new(Key::Char('a')));
        ac.handle_key(KeyEvent::new(Key::Tab));

        assert_eq!(ac.get_value(), "apple");
    }

    #[test]
    fn test_autocomplete_handle_key_unhandled() {
        let mut ac = Autocomplete::new();
        ac.focus();

        let handled = ac.handle_key(KeyEvent::new(Key::F(1)));
        assert!(!handled);
    }

    #[test]
    fn test_suggestion_description_and_icon() {
        let s = Suggestion::new("test")
            .description("A test suggestion")
            .icon('✓');

        assert_eq!(s.description, Some("A test suggestion".to_string()));
        assert_eq!(s.icon, Some('✓'));
    }

    #[test]
    fn test_suggestion_from() {
        let s: Suggestion = "quick".into();
        assert_eq!(s.label, "quick");
        assert_eq!(s.value, "quick");
    }

    #[test]
    fn test_autocomplete_set_suggestions() {
        let mut ac = Autocomplete::new();
        ac.set_suggestions(vec![Suggestion::new("one"), Suggestion::new("two")]);
        // Test through public API
        ac.set_value("one");
        assert!(ac.selected_suggestion().is_some());
    }
}

// Re-exports
pub use core::Autocomplete;
pub use helper::autocomplete;
pub use types::Suggestion;
