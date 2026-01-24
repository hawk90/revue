//! Combobox/Autocomplete widget combining text input with searchable dropdown
//!
//! Features:
//! - Text input with dropdown suggestions
//! - Multiple filter modes (fuzzy, prefix, exact, contains)
//! - Keyboard navigation (arrow, enter, escape)
//! - Highlight matching text
//! - Allow custom values (free-form input)
//! - Multiple selection variant
//! - Loading and empty states

mod actions;
mod filter;
mod input;
mod option;
mod render;
mod state;
#[cfg(test)]
mod tests {
    use super::*;

    use super::super::*;
    use crate::prelude::RenderContext;
    use crate::render::Buffer;
    use crate::style::Color;
    use crate::utils::FilterMode;
    use crate::widget::traits::View;

    #[test]
    fn test_combobox_new() {
        let cb = Combobox::new();
        assert!(cb.input().is_empty());
        assert!(!cb.is_open());
        assert_eq!(cb.option_count(), 0);
    }

    #[test]
    fn test_combobox_options() {
        let cb = Combobox::new().options(vec!["Apple", "Banana", "Cherry"]);
        assert_eq!(cb.option_count(), 3);
        assert_eq!(cb.filtered_count(), 3);
    }

    #[test]
    fn test_combobox_options_with() {
        let cb = Combobox::new().options_with(vec![
            ComboOption::new("Apple").value("apple"),
            ComboOption::new("Banana").disabled(true),
        ]);
        assert_eq!(cb.option_count(), 2);
    }

    #[test]
    fn test_combobox_filtering_fuzzy() {
        let mut cb = Combobox::new()
            .options(vec!["Hello World", "Help Me", "Goodbye"])
            .filter_mode(FilterMode::Fuzzy);

        cb.set_input("hw");
        assert_eq!(cb.filtered_count(), 1); // Only "Hello World" matches "hw"
    }

    #[test]
    fn test_combobox_filtering_prefix() {
        let mut cb = Combobox::new()
            .options(vec!["Hello", "Help", "World"])
            .filter_mode(FilterMode::Prefix);

        cb.set_input("Hel");
        assert_eq!(cb.filtered_count(), 2); // "Hello" and "Help"
    }

    #[test]
    fn test_combobox_filtering_contains() {
        let mut cb = Combobox::new()
            .options(vec!["Hello", "Shell", "World"])
            .filter_mode(FilterMode::Contains);

        cb.set_input("ell");
        assert_eq!(cb.filtered_count(), 2); // "Hello" and "Shell"
    }

    #[test]
    fn test_combobox_filtering_exact() {
        let mut cb = Combobox::new()
            .options(vec!["Hello", "hello", "HELLO"])
            .filter_mode(FilterMode::Exact);

        cb.set_input("hello");
        assert_eq!(cb.filtered_count(), 3); // All match (case-insensitive)
    }

    #[test]
    fn test_combobox_navigation() {
        let mut cb = Combobox::new().options(vec!["A", "B", "C"]);

        cb.open_dropdown();
        assert!(cb.is_open());

        cb.select_next();
        assert_eq!(cb.selected_idx, 1);

        cb.select_next();
        assert_eq!(cb.selected_idx, 2);

        cb.select_next(); // Wraps
        assert_eq!(cb.selected_idx, 0);

        cb.select_prev(); // Wraps backward
        assert_eq!(cb.selected_idx, 2);

        cb.select_first();
        assert_eq!(cb.selected_idx, 0);

        cb.select_last();
        assert_eq!(cb.selected_idx, 2);
    }

    #[test]
    fn test_combobox_select_current() {
        let mut cb = Combobox::new().options(vec!["Apple", "Banana"]);

        cb.open_dropdown();
        cb.select_next(); // Select "Banana"
        cb.select_current();

        assert_eq!(cb.input(), "Banana");
        assert!(!cb.is_open()); // Closes after selection
    }

    #[test]
    fn test_combobox_multi_select() {
        let mut cb = Combobox::new()
            .options(vec!["A", "B", "C"])
            .multi_select(true);

        cb.open_dropdown();
        cb.select_current(); // Select "A"
        assert!(cb.is_selected("A"));
        assert!(cb.is_open()); // Stays open in multi-select

        cb.select_next();
        cb.select_current(); // Select "B"
        assert!(cb.is_selected("A"));
        assert!(cb.is_selected("B"));

        // Toggle off
        cb.select_first();
        cb.select_current(); // Deselect "A"
        assert!(!cb.is_selected("A"));
        assert!(cb.is_selected("B"));
    }

    #[test]
    fn test_combobox_allow_custom() {
        let cb = Combobox::new()
            .options(vec!["Apple", "Banana"])
            .allow_custom(true)
            .value("Custom Value");

        assert_eq!(cb.selected_value(), Some("Custom Value"));
    }

    #[test]
    fn test_combobox_disabled_option() {
        let mut cb = Combobox::new().options_with(vec![
            ComboOption::new("Enabled"),
            ComboOption::new("Disabled").disabled(true),
        ]);

        cb.open_dropdown();
        cb.select_next(); // Try to select disabled option
        let selected = cb.select_current();
        assert!(!selected); // Should not select
    }

    #[test]
    fn test_combobox_input_manipulation() {
        let mut cb = Combobox::new();

        cb.insert_char('H');
        cb.insert_char('i');
        assert_eq!(cb.input(), "Hi");
        assert_eq!(cb.cursor, 2);

        cb.delete_backward();
        assert_eq!(cb.input(), "H");
        assert_eq!(cb.cursor, 1);

        cb.move_left();
        assert_eq!(cb.cursor, 0);

        cb.insert_char('O');
        assert_eq!(cb.input(), "OH");

        cb.move_to_end();
        assert_eq!(cb.cursor, 2);

        cb.move_to_start();
        assert_eq!(cb.cursor, 0);
    }

    #[test]
    fn test_combobox_handle_key() {
        use crate::event::Key;

        let mut cb = Combobox::new().options(vec!["Apple", "Banana"]);

        // Type to filter
        cb.handle_key(&Key::Char('a'));
        assert_eq!(cb.input(), "a");
        assert!(cb.is_open()); // Opens on typing

        // Navigate
        cb.handle_key(&Key::Down);
        assert_eq!(cb.selected_idx, 1);

        // Select
        cb.handle_key(&Key::Enter);
        assert!(!cb.is_open());

        // Escape
        cb.open_dropdown();
        cb.handle_key(&Key::Escape);
        assert!(!cb.is_open());
    }

    #[test]
    fn test_combobox_loading_state() {
        let cb = Combobox::new().loading(true).loading_text("Fetching...");

        assert!(cb.is_loading());
    }

    #[test]
    fn test_combobox_render_closed() {
        let mut buffer = Buffer::new(30, 10);
        let area = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let cb = Combobox::new()
            .options(vec!["Option 1", "Option 2"])
            .placeholder("Select...");

        cb.render(&mut ctx);

        // Should show dropdown arrow
        // The arrow is at width - 2
    }

    #[test]
    fn test_combobox_render_open() {
        let mut buffer = Buffer::new(30, 10);
        let area = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut cb = Combobox::new().options(vec!["Apple", "Banana"]);
        cb.open_dropdown();

        cb.render(&mut ctx);

        // Options should be rendered below input
    }

    #[test]
    fn test_combobox_helper() {
        let cb = combobox().option("Test").placeholder("Pick one");
        assert_eq!(cb.option_count(), 1);
    }

    #[test]
    fn test_combobox_clear_input() {
        let mut cb = Combobox::new().options(vec!["A", "B"]).value("test");

        assert_eq!(cb.input(), "test");
        cb.clear_input();
        assert!(cb.input().is_empty());
    }

    #[test]
    fn test_combobox_scroll() {
        let mut cb = Combobox::new()
            .options(vec!["A", "B", "C", "D", "E", "F", "G", "H"])
            .max_visible(3);

        cb.open_dropdown();

        // Navigate to end
        for _ in 0..7 {
            cb.select_next();
        }

        // Should have scrolled
        assert!(cb.scroll_offset > 0);
    }

    #[test]
    fn test_combo_option_builder() {
        let opt = ComboOption::new("Label")
            .value("value")
            .disabled(true)
            .group("Category");

        assert_eq!(opt.label, "Label");
        assert_eq!(opt.get_value(), "value");
        assert!(opt.disabled);
        assert_eq!(opt.group, Some("Category".to_string()));
    }

    #[test]
    fn test_combobox_empty_filter() {
        let mut cb = Combobox::new().options(vec!["Apple", "Banana"]);

        cb.set_input("xyz"); // No match
        assert_eq!(cb.filtered_count(), 0);
    }

    #[test]
    fn test_filter_mode_default() {
        assert_eq!(FilterMode::default(), FilterMode::Fuzzy);
    }

    // Additional tests for coverage

    #[test]
    fn test_combobox_builder_methods() {
        let cb = Combobox::new()
            .loading_text("Please wait...")
            .empty_text("Nothing found")
            .width(50)
            .input_style(Color::WHITE, Color::BLACK)
            .selected_style(Color::BLACK, Color::WHITE)
            .highlight_fg(Color::YELLOW)
            .fg(Color::WHITE)
            .bg(Color::BLACK);

        assert_eq!(cb.loading_text, "Please wait...");
        assert_eq!(cb.empty_text, "Nothing found");
        assert_eq!(cb.width, Some(50));
    }

    #[test]
    fn test_combobox_selected_values() {
        let cb = Combobox::new()
            .multi_select(true)
            .selected_values(vec!["A".to_string(), "B".to_string()]);

        assert_eq!(cb.selected_values_ref(), &["A", "B"]);
    }

    #[test]
    fn test_combobox_delete_forward() {
        let mut cb = Combobox::new().value("Hello");
        cb.move_to_start();
        cb.delete_forward();
        assert_eq!(cb.input(), "ello");
    }

    #[test]
    fn test_combobox_delete_forward_at_end() {
        let mut cb = Combobox::new().value("Hi");
        // Cursor at end, delete_forward should do nothing
        cb.delete_forward();
        assert_eq!(cb.input(), "Hi");
    }

    #[test]
    fn test_combobox_delete_backward_at_start() {
        let mut cb = Combobox::new().value("Hi");
        cb.move_to_start();
        cb.delete_backward();
        assert_eq!(cb.input(), "Hi"); // Nothing deleted
    }

    #[test]
    fn test_combobox_move_right_at_end() {
        let mut cb = Combobox::new().value("Hi");
        cb.move_right(); // Already at end
        assert_eq!(cb.cursor, 2);
    }

    #[test]
    fn test_combobox_move_left_at_start() {
        let mut cb = Combobox::new().value("Hi");
        cb.move_to_start();
        cb.move_left(); // Already at start
        assert_eq!(cb.cursor, 0);
    }

    #[test]
    fn test_combobox_toggle_dropdown() {
        let mut cb = Combobox::new().options(vec!["A", "B"]);
        assert!(!cb.is_open());

        cb.toggle_dropdown();
        assert!(cb.is_open());

        cb.toggle_dropdown();
        assert!(!cb.is_open());
    }

    #[test]
    fn test_combobox_handle_key_down_when_closed() {
        use crate::event::Key;

        let mut cb = Combobox::new().options(vec!["Apple", "Banana"]);
        assert!(!cb.is_open());

        cb.handle_key(&Key::Down);
        assert!(cb.is_open()); // Down opens dropdown
    }

    #[test]
    fn test_combobox_handle_key_tab_completion() {
        use crate::event::Key;

        let mut cb = Combobox::new().options(vec!["Apple", "Banana"]);
        cb.open_dropdown();
        cb.handle_key(&Key::Tab);

        assert_eq!(cb.input(), "Apple"); // First option filled
    }

    #[test]
    fn test_combobox_handle_key_delete() {
        use crate::event::Key;

        let mut cb = Combobox::new().value("Hello");
        cb.move_to_start();
        cb.handle_key(&Key::Delete);
        assert_eq!(cb.input(), "ello");
    }

    #[test]
    fn test_combobox_handle_key_home_end() {
        use crate::event::Key;

        let mut cb = Combobox::new().value("Hello");
        cb.handle_key(&Key::Home);
        assert_eq!(cb.cursor, 0);

        cb.handle_key(&Key::End);
        assert_eq!(cb.cursor, 5);
    }

    #[test]
    fn test_combobox_handle_key_left_right() {
        use crate::event::Key;

        let mut cb = Combobox::new().value("Hi");
        cb.handle_key(&Key::Left);
        assert_eq!(cb.cursor, 1);

        cb.handle_key(&Key::Right);
        assert_eq!(cb.cursor, 2);
    }

    #[test]
    fn test_combobox_handle_key_up_when_open() {
        use crate::event::Key;

        let mut cb = Combobox::new().options(vec!["A", "B", "C"]);
        cb.open_dropdown();
        cb.select_next(); // Go to B
        cb.handle_key(&Key::Up);
        assert_eq!(cb.selected_idx, 0); // Back to A
    }

    #[test]
    fn test_combobox_handle_key_unhandled() {
        use crate::event::Key;

        let mut cb = Combobox::new();
        let handled = cb.handle_key(&Key::F(1));
        assert!(!handled);
    }

    #[test]
    fn test_combobox_selected_value_from_option() {
        let cb = Combobox::new()
            .options(vec!["Apple", "Banana"])
            .value("Apple");

        assert_eq!(cb.selected_value(), Some("Apple"));
    }

    #[test]
    fn test_combobox_selected_value_multi_select_returns_none() {
        let cb = Combobox::new()
            .options(vec!["A", "B"])
            .multi_select(true)
            .value("A");

        assert_eq!(cb.selected_value(), None);
    }

    #[test]
    fn test_combobox_selected_value_no_match_no_custom() {
        let cb = Combobox::new()
            .options(vec!["Apple", "Banana"])
            .value("Custom");

        assert_eq!(cb.selected_value(), None);
    }

    #[test]
    fn test_combobox_get_match_non_fuzzy() {
        let cb = Combobox::new()
            .options(vec!["Apple"])
            .filter_mode(FilterMode::Prefix)
            .value("App");

        // get_match only works for fuzzy mode
        assert!(cb.get_match("Apple").is_none());
    }

    #[test]
    fn test_combobox_select_on_empty_filtered() {
        let mut cb = Combobox::new().options(vec!["Apple"]);
        cb.set_input("xyz"); // No matches
        let selected = cb.select_current();
        assert!(!selected);
    }

    #[test]
    fn test_combobox_navigation_empty_options() {
        let mut cb = Combobox::new();
        cb.select_next(); // Should not panic
        cb.select_prev();
        cb.select_last();
        assert_eq!(cb.selected_idx, 0);
    }

    #[test]
    fn test_combobox_render_loading_state() {
        let mut buffer = Buffer::new(30, 10);
        let area = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut cb = Combobox::new()
            .options(vec!["A", "B"])
            .loading(true)
            .loading_text("Loading...")
            .width(30); // Set explicit width
        cb.open_dropdown();

        cb.render(&mut ctx);

        // Verify loading indicator is shown (at width - 2)
        assert_eq!(buffer.get(28, 0).unwrap().symbol, '⟳');
    }

    #[test]
    fn test_combobox_render_empty_state() {
        let mut buffer = Buffer::new(30, 10);
        let area = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut cb = Combobox::new()
            .options(vec!["Apple", "Banana"])
            .empty_text("No results");
        cb.set_input("xyz"); // No matches
        cb.open_dropdown();

        cb.render(&mut ctx);
        // Empty state should be rendered
    }

    #[test]
    fn test_combobox_render_with_scroll_indicators() {
        let mut buffer = Buffer::new(30, 5);
        let area = Rect::new(0, 0, 30, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut cb = Combobox::new()
            .options(vec!["A", "B", "C", "D", "E", "F", "G", "H", "I", "J"])
            .max_visible(3);
        cb.open_dropdown();

        // Navigate down to trigger scroll
        for _ in 0..5 {
            cb.select_next();
        }

        cb.render(&mut ctx);
        // Scroll indicators should be visible
    }

    #[test]
    fn test_combobox_render_multi_select() {
        let mut buffer = Buffer::new(30, 10);
        let area = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut cb = Combobox::new()
            .options(vec!["A", "B", "C"])
            .multi_select(true);
        cb.open_dropdown();
        cb.select_current(); // Select "A"

        cb.render(&mut ctx);
        // Multi-select checkboxes should be rendered
    }

    #[test]
    fn test_combobox_render_with_input() {
        let mut buffer = Buffer::new(30, 10);
        let area = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut cb = Combobox::new().options(vec!["Apple", "Banana"]);
        cb.set_input("App");
        cb.open_dropdown();

        cb.render(&mut ctx);
        // Input and filtered options should be rendered with highlights
    }

    #[test]
    fn test_combobox_render_disabled_option() {
        let mut buffer = Buffer::new(30, 10);
        let area = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut cb = Combobox::new().options_with(vec![
            ComboOption::new("Enabled"),
            ComboOption::new("Disabled").disabled(true),
        ]);
        cb.open_dropdown();

        cb.render(&mut ctx);
        // Disabled option should be rendered with disabled color
    }

    #[test]
    fn test_combobox_render_small_area() {
        let mut buffer = Buffer::new(2, 1);
        let area = Rect::new(0, 0, 2, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let cb = Combobox::new().options(vec!["A"]);
        cb.render(&mut ctx);
        // Should handle small area gracefully (early return)
    }

    #[test]
    fn test_combobox_render_height_one() {
        let mut buffer = Buffer::new(30, 1);
        let area = Rect::new(0, 0, 30, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut cb = Combobox::new().options(vec!["A", "B"]);
        cb.open_dropdown();
        cb.render(&mut ctx);
        // Dropdown shouldn't render when height is 1
    }

    #[test]
    fn test_combobox_default() {
        let cb = Combobox::default();
        assert!(cb.input().is_empty());
        assert!(!cb.is_open());
    }

    #[test]
    fn test_combo_option_from_string() {
        let opt: ComboOption = "Test".into();
        assert_eq!(opt.label, "Test");
        assert_eq!(opt.get_value(), "Test");
    }

    #[test]
    fn test_combobox_ensure_visible_scroll_up() {
        let mut cb = Combobox::new()
            .options(vec!["A", "B", "C", "D", "E", "F", "G", "H"])
            .max_visible(3);

        cb.open_dropdown();

        // Scroll down
        for _ in 0..7 {
            cb.select_next();
        }
        assert!(cb.scroll_offset > 0);

        // Now scroll back up
        for _ in 0..7 {
            cb.select_prev();
        }
        assert_eq!(cb.scroll_offset, 0);
    }

    #[test]
    fn test_combobox_handle_key_enter_not_open_allow_custom() {
        use crate::event::Key;

        let mut cb = Combobox::new()
            .options(vec!["A", "B"])
            .allow_custom(true)
            .value("Custom");

        // Enter when not open with allow_custom
        let handled = cb.handle_key(&Key::Enter);
        assert!(handled);
    }

    #[test]
    fn test_combobox_option_with_separate_value() {
        let mut cb = Combobox::new()
            .options_with(vec![ComboOption::new("Display Name").value("actual_value")]);

        cb.open_dropdown();
        cb.select_current();

        // Input should be label, but value lookup should work
        assert_eq!(cb.input(), "Display Name");
    }

    #[test]
    fn test_combobox_cursor_render_boundary() {
        let mut buffer = Buffer::new(10, 5);
        let area = Rect::new(0, 0, 10, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let cb = Combobox::new().value("Very long text that exceeds width");
        cb.render(&mut ctx);
        // Should handle cursor at boundary correctly
    }

    #[test]
    fn test_combobox_render_highlighted_option() {
        let mut buffer = Buffer::new(30, 10);
        let area = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut cb = Combobox::new().options(vec!["Apple", "Banana", "Cherry"]);
        cb.open_dropdown();
        cb.select_next(); // Highlight "Banana"

        cb.render(&mut ctx);
        // Should render with selected style
    }
}

pub use option::ComboOption;

use super::traits::WidgetProps;
use crate::style::Color;
use crate::utils::FilterMode;
use crate::{impl_props_builders, impl_styled_view};

/// A combobox widget with text input and searchable dropdown
#[derive(Clone, Debug)]
pub struct Combobox {
    /// Available options
    pub(super) options: Vec<ComboOption>,
    /// Current input value
    pub(super) input: String,
    /// Cursor position in input
    pub(super) cursor: usize,
    /// Whether dropdown is open
    pub(super) open: bool,
    /// Selected index in filtered list
    pub(super) selected_idx: usize,
    /// Filtered option indices
    pub(super) filtered: Vec<usize>,
    /// Filter mode
    pub(super) filter_mode: FilterMode,
    /// Allow custom values not in options
    pub(super) allow_custom: bool,
    /// Multiple selection mode
    pub(super) multi_select: bool,
    /// Selected values (for multi-select)
    pub(super) selected_values: Vec<String>,
    /// Placeholder text
    pub(super) placeholder: String,
    /// Loading state
    pub(super) loading: bool,
    /// Loading text
    pub(super) loading_text: String,
    /// Empty state text
    pub(super) empty_text: String,
    /// Max visible options in dropdown
    pub(super) max_visible: usize,
    /// Scroll offset in dropdown
    pub(super) scroll_offset: usize,
    // Styling
    pub(super) fg: Option<Color>,
    pub(super) bg: Option<Color>,
    pub(super) input_fg: Option<Color>,
    pub(super) input_bg: Option<Color>,
    pub(super) selected_fg: Option<Color>,
    pub(super) selected_bg: Option<Color>,
    pub(super) highlight_fg: Option<Color>,
    pub(super) disabled_fg: Option<Color>,
    /// Fixed width
    pub(super) width: Option<u16>,
    /// CSS styling properties
    pub(super) props: WidgetProps,
}

impl Combobox {
    /// Create a new combobox
    pub fn new() -> Self {
        Self {
            options: Vec::new(),
            input: String::new(),
            cursor: 0,
            open: false,
            selected_idx: 0,
            filtered: Vec::new(),
            filter_mode: FilterMode::Fuzzy,
            allow_custom: false,
            multi_select: false,
            selected_values: Vec::new(),
            placeholder: "Type to search...".to_string(),
            loading: false,
            loading_text: "Loading...".to_string(),
            empty_text: "No results".to_string(),
            max_visible: 5,
            scroll_offset: 0,
            fg: None,
            bg: None,
            input_fg: None,
            input_bg: None,
            selected_fg: Some(Color::WHITE),
            selected_bg: Some(Color::BLUE),
            highlight_fg: Some(Color::YELLOW),
            disabled_fg: Some(Color::rgb(128, 128, 128)),
            width: None,
            props: WidgetProps::new(),
        }
    }

    /// Set options from strings
    pub fn options<I, S>(mut self, options: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.options = options.into_iter().map(|s| ComboOption::new(s)).collect();
        self.update_filter();
        self
    }

    /// Set options from ComboOption items
    pub fn options_with<I>(mut self, options: I) -> Self
    where
        I: IntoIterator<Item = ComboOption>,
    {
        self.options = options.into_iter().collect();
        self.update_filter();
        self
    }

    /// Add a single option
    pub fn option(mut self, option: impl Into<ComboOption>) -> Self {
        self.options.push(option.into());
        self.update_filter();
        self
    }

    /// Set filter mode
    pub fn filter_mode(mut self, mode: FilterMode) -> Self {
        self.filter_mode = mode;
        self.update_filter();
        self
    }

    /// Allow custom values not in the options list
    pub fn allow_custom(mut self, allow: bool) -> Self {
        self.allow_custom = allow;
        self
    }

    /// Enable multiple selection mode
    pub fn multi_select(mut self, multi: bool) -> Self {
        self.multi_select = multi;
        self
    }

    /// Set placeholder text
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = text.into();
        self
    }

    /// Set loading state
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    /// Set loading text
    pub fn loading_text(mut self, text: impl Into<String>) -> Self {
        self.loading_text = text.into();
        self
    }

    /// Set empty state text
    pub fn empty_text(mut self, text: impl Into<String>) -> Self {
        self.empty_text = text.into();
        self
    }

    /// Set max visible options
    pub fn max_visible(mut self, count: usize) -> Self {
        self.max_visible = count.max(1);
        self
    }

    /// Set fixed width
    pub fn width(mut self, width: u16) -> Self {
        self.width = Some(width);
        self
    }

    /// Set foreground color
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Set background color
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    /// Set input field colors
    pub fn input_style(mut self, fg: Color, bg: Color) -> Self {
        self.input_fg = Some(fg);
        self.input_bg = Some(bg);
        self
    }

    /// Set selected option colors
    pub fn selected_style(mut self, fg: Color, bg: Color) -> Self {
        self.selected_fg = Some(fg);
        self.selected_bg = Some(bg);
        self
    }

    /// Set highlight color for matched characters
    pub fn highlight_fg(mut self, color: Color) -> Self {
        self.highlight_fg = Some(color);
        self
    }

    /// Set initial input value
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.input = value.into();
        self.cursor = self.input.chars().count();
        self.update_filter();
        self
    }

    /// Set pre-selected values (for multi-select)
    pub fn selected_values(mut self, values: Vec<String>) -> Self {
        self.selected_values = values;
        self
    }
}

impl Default for Combobox {
    fn default() -> Self {
        Self::new()
    }
}

impl crate::widget::traits::View for Combobox {
    fn render(&self, ctx: &mut crate::widget::traits::RenderContext) {
        // Delegate to the render module
        render::render_combobox(self, ctx);
    }

    crate::impl_view_meta!("Combobox");
}

impl_styled_view!(Combobox);
impl_props_builders!(Combobox);

/// Helper function to create a combobox
pub fn combobox() -> Combobox {
    Combobox::new()
}
