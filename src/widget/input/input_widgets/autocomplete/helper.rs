use super::core::Autocomplete;

/// Helper function to create an autocomplete widget
pub fn autocomplete() -> Autocomplete {
    Autocomplete::new()
}

#[cfg(test)]
mod tests {
    use super::super::types::Suggestion;
    use super::*;
    use crate::event::{Key, KeyEvent};
    use crate::style::Color;
    use crate::utils::FilterMode;

    // =========================================================================
    // autocomplete helper tests
    // =========================================================================

    #[test]
    fn test_autocomplete_helper_creates_widget() {
        let widget = autocomplete();
        // Should create a widget with default values
        assert!(!widget.is_focused());
    }

    #[test]
    fn test_autocomplete_helper_default_value() {
        let widget = autocomplete();
        assert_eq!(widget.get_value(), "");
    }

    #[test]
    fn test_autocomplete_helper_chainable() {
        let widget = autocomplete()
            .value("test")
            .placeholder("Enter text...")
            .min_chars(1)
            .max_suggestions(10);
        assert_eq!(widget.get_value(), "test");
    }

    #[test]
    fn test_autocomplete_helper_with_value() {
        let widget = autocomplete().value("hello");
        assert_eq!(widget.get_value(), "hello");
    }

    #[test]
    fn test_autocomplete_helper_with_placeholder() {
        let widget = autocomplete().placeholder("Search...");
        let _ = widget;
    }

    #[test]
    fn test_autocomplete_helper_with_filter_mode() {
        let widget = autocomplete().filter_mode(FilterMode::Prefix);
        let _ = widget;
    }

    #[test]
    fn test_autocomplete_helper_with_min_chars() {
        let widget = autocomplete().min_chars(2);
        let _ = widget;
    }

    #[test]
    fn test_autocomplete_helper_with_max_suggestions() {
        let widget = autocomplete().max_suggestions(5);
        let _ = widget;
    }

    #[test]
    fn test_autocomplete_helper_focus() {
        let mut widget = autocomplete();
        assert!(!widget.is_focused());
        widget.focus();
        assert!(widget.is_focused());
    }

    #[test]
    fn test_autocomplete_helper_blur() {
        let mut widget = autocomplete();
        widget.focus();
        assert!(widget.is_focused());
        widget.blur();
        assert!(!widget.is_focused());
    }

    #[test]
    fn test_autocomplete_helper_set_value() {
        let mut widget = autocomplete();
        widget.set_value("new value");
        assert_eq!(widget.get_value(), "new value");
    }

    #[test]
    fn test_autocomplete_helper_set_suggestions() {
        let mut widget = autocomplete();
        let suggestions = vec![Suggestion::new("Option 1"), Suggestion::new("Option 2")];
        widget.set_suggestions(suggestions);
        let _ = widget;
    }

    #[test]
    fn test_autocomplete_helper_no_selection_initially() {
        let widget = autocomplete();
        assert!(widget.selected_suggestion().is_none());
    }

    #[test]
    fn test_autocomplete_helper_input_style() {
        let widget = autocomplete().input_style(Color::WHITE, Color::BLACK);
        let _ = widget;
    }

    #[test]
    fn test_autocomplete_helper_dropdown_style() {
        let widget =
            autocomplete().dropdown_style(Color::BLUE, Color::WHITE, Color::rgb(0, 0, 139));
        let _ = widget;
    }

    #[test]
    fn test_autocomplete_helper_highlight_fg() {
        let widget = autocomplete().highlight_fg(Color::YELLOW);
        let _ = widget;
    }

    #[test]
    fn test_autocomplete_helper_handle_key() {
        let mut widget = autocomplete();
        widget.set_suggestions(vec![Suggestion::new("Test")]);
        widget.focus();
        let handled = widget.handle_key(KeyEvent::new(Key::Char('t')));
        // Key should be handled
        assert!(handled);
    }

    #[test]
    fn test_autocomplete_helper_accept_selection() {
        let mut widget = autocomplete();
        widget.set_suggestions(vec![Suggestion::new("Item")]);
        widget.set_value("i");
        widget.focus();
        // Set up selection by typing and matching
        let accepted = widget.accept_selection();
        // May accept a selection if there's a match
        let _ = accepted;
    }

    #[test]
    fn test_autocomplete_helper_with_string_value() {
        let widget = autocomplete().value(String::from("test"));
        assert_eq!(widget.get_value(), "test");
    }

    #[test]
    fn test_autocomplete_helper_empty_value() {
        let widget = autocomplete().value("");
        assert_eq!(widget.get_value(), "");
    }

    #[test]
    fn test_autocomplete_helper_chained_builders() {
        let widget = autocomplete()
            .value("search")
            .placeholder("Type to search...")
            .min_chars(1)
            .max_suggestions(8)
            .filter_mode(FilterMode::Prefix)
            .input_style(Color::rgb(255, 255, 255), Color::rgb(0, 0, 0))
            .dropdown_style(
                Color::rgb(50, 50, 50),
                Color::rgb(255, 255, 255),
                Color::rgb(100, 100, 255),
            )
            .highlight_fg(Color::YELLOW);
        assert_eq!(widget.get_value(), "search");
    }
}
