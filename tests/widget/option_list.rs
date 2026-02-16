//! Integration tests for option_list widget

use revue::widget::{option_list, option_item, OptionEntry, OptionItem, OptionList};
use revue::widget::OptionSeparatorStyle as SeparatorStyle;
use revue::widget::traits::DISABLED_FG;
use revue::style::Color;

// ============================================================================
// Basic OptionList tests
// ============================================================================

#[test]
fn test_option_list_new() {
    let list = OptionList::new();
    assert_eq!(list.option_count(), 0);
}

#[test]
fn test_add_options() {
    let list = OptionList::new()
        .option("Option 1", "Ctrl+1")
        .option("Option 2", "Ctrl+2");
    assert_eq!(list.option_count(), 2);
}

#[test]
fn test_separators() {
    let list = OptionList::new()
        .option("A", "")
        .separator()
        .option("B", "");
    assert_eq!(list.__test_entries().len(), 3);
    assert_eq!(list.option_count(), 2);
}

#[test]
fn test_groups() {
    let list = OptionList::new()
        .group("Group 1")
        .option("A", "")
        .group("Group 2")
        .option("B", "");
    assert_eq!(list.__test_entries().len(), 4);
    assert_eq!(list.option_count(), 2);
}

#[test]
fn test_navigation() {
    let mut list = OptionList::new()
        .option("A", "")
        .option("B", "")
        .option("C", "")
        .focused(true);

    assert_eq!(list.__test_highlighted(), 0);
    list.highlight_next();
    assert_eq!(list.__test_highlighted(), 1);
    list.highlight_previous();
    assert_eq!(list.__test_highlighted(), 0);
    list.highlight_last();
    assert_eq!(list.__test_highlighted(), 2);
    list.highlight_first();
    assert_eq!(list.__test_highlighted(), 0);
}

#[test]
fn test_selection() {
    let mut list = OptionList::new()
        .option("A", "")
        .option("B", "")
        .focused(true);

    assert!(list.__test_selected().is_none());
    list.highlight_next();
    list.select_highlighted();
    assert_eq!(list.__test_selected(), Some(1));

    let selected = list.get_selected().unwrap();
    assert_eq!(selected.text, "B");
}

#[test]
fn test_disabled_skip() {
    let mut list = OptionList::new()
        .add_option(OptionItem::new("A"))
        .add_option(OptionItem::new("B").disabled(true))
        .add_option(OptionItem::new("C"))
        .focused(true);

    assert_eq!(list.__test_highlighted(), 0);
    list.highlight_next(); // Should skip B
    assert_eq!(list.__test_highlighted(), 2);
}

#[test]
fn test_disabled_no_select() {
    let mut list = OptionList::new()
        .add_option(OptionItem::new("A").disabled(true))
        .focused(true);

    assert!(!list.select_highlighted());
    assert!(list.__test_selected().is_none());
}

#[test]
fn test_get_selected_value() {
    let mut list = OptionList::new()
        .add_option(OptionItem::new("Display").value("actual_value"))
        .focused(true);

    list.select_highlighted();
    assert_eq!(list.get_selected_value(), Some("actual_value"));
}

#[test]
fn test_separator_styles() {
    let list = OptionList::new().separator_style(SeparatorStyle::Double);
    assert_eq!(list.__test_separator_char(), "═");
}

#[test]
fn test_helper_functions() {
    let list = option_list().option("Test", "hint");
    assert_eq!(list.option_count(), 1);

    let item = option_item("Test").hint("hint").value("val");
    assert_eq!(item.hint, Some("hint".to_string()));
    assert_eq!(item.value, Some("val".to_string()));
}

#[test]
fn test_disabled_fg_constant() {
    // Verify that DISABLED_FG constant is properly imported and usable
    let list = OptionList::new().add_option(OptionItem::new("Disabled").disabled(true));

    // The disabled_fg should default to DISABLED_FG when not explicitly set
    assert_eq!(list.__test_disabled_fg(), &None);
    // When rendering, disabled items should use DISABLED_FG as fallback
    assert_eq!(DISABLED_FG.r, 100);
    assert_eq!(DISABLED_FG.g, 100);
    assert_eq!(DISABLED_FG.b, 100);
}

// ============================================================================
// SeparatorStyle enum tests
// ============================================================================

#[test]
fn test_separator_style_default() {
    assert_eq!(SeparatorStyle::default(), SeparatorStyle::Line);
}

#[test]
fn test_separator_style_clone() {
    let style = SeparatorStyle::Dashed;
    assert_eq!(style, style.clone());
}

#[test]
fn test_separator_style_copy() {
    let s1 = SeparatorStyle::Double;
    let s2 = s1;
    assert_eq!(s1, SeparatorStyle::Double);
    assert_eq!(s2, SeparatorStyle::Double);
}

#[test]
fn test_separator_style_partial_eq() {
    assert_eq!(SeparatorStyle::Line, SeparatorStyle::Line);
    assert_eq!(SeparatorStyle::Dashed, SeparatorStyle::Dashed);
    assert_eq!(SeparatorStyle::Double, SeparatorStyle::Double);
    assert_eq!(SeparatorStyle::Blank, SeparatorStyle::Blank);
    assert_ne!(SeparatorStyle::Line, SeparatorStyle::Dashed);
}

#[test]
fn test_separator_style_debug() {
    let debug_str = format!("{:?}", SeparatorStyle::Blank);
    assert!(debug_str.contains("Blank"));
}

// ============================================================================
// OptionItem tests
// ============================================================================

#[test]
fn test_option_item_new() {
    let item = OptionItem::new("Test");
    assert_eq!(item.text, "Test");
    assert!(item.hint.is_none());
    assert!(item.value.is_none());
    assert!(!item.disabled);
    assert!(item.icon.is_none());
    assert!(item.description.is_none());
}

#[test]
fn test_option_item_hint() {
    let item = OptionItem::new("Test").hint("Ctrl+S");
    assert_eq!(item.hint, Some("Ctrl+S".to_string()));
}

#[test]
fn test_option_item_value() {
    let item = OptionItem::new("Test").value("save");
    assert_eq!(item.value, Some("save".to_string()));
}

#[test]
fn test_option_item_disabled() {
    let item = OptionItem::new("Test").disabled(true);
    assert!(item.disabled);
}

#[test]
fn test_option_item_icon() {
    let item = OptionItem::new("Test").icon("📁");
    assert_eq!(item.icon, Some("📁".to_string()));
}

#[test]
fn test_option_item_description() {
    let item = OptionItem::new("Test").description("A test option");
    assert_eq!(item.description, Some("A test option".to_string()));
}

#[test]
fn test_option_item_builder_chain() {
    let item = OptionItem::new("Save")
        .hint("Ctrl+S")
        .value("save_cmd")
        .disabled(false)
        .icon("💾")
        .description("Save the file");

    assert_eq!(item.text, "Save");
    assert_eq!(item.hint, Some("Ctrl+S".to_string()));
    assert_eq!(item.value, Some("save_cmd".to_string()));
    assert_eq!(item.icon, Some("💾".to_string()));
    assert_eq!(item.description, Some("Save the file".to_string()));
}

// ============================================================================
// OptionEntry enum tests
// ============================================================================

#[test]
fn test_option_entry_clone() {
    let entry = OptionEntry::Group("Test".to_string());
    let cloned = entry.clone();
    // Can't assert equality, but verify cloning works
    if let OptionEntry::Group(name) = cloned {
        assert_eq!(name, "Test");
    }
}

#[test]
fn test_option_entry_debug() {
    let item = OptionItem::new("Test");
    let entry = OptionEntry::Option(item);
    let debug_str = format!("{:?}", entry);
    assert!(debug_str.contains("Option"));
}

// ============================================================================
// OptionList::new tests
// ============================================================================

#[test]
fn test_option_list_new_default_values() {
    let list = OptionList::new();
    assert_eq!(list.__test_entries().len(), 0);
    assert_eq!(list.__test_highlighted(), 0);
    assert!(list.__test_selected().is_none());
    assert_eq!(list.__test_separator_style(), SeparatorStyle::Line);
    assert!(list.__test_title().is_none());
    assert!(list.__test_width().is_none());
    assert!(!list.__test_show_descriptions());
    assert!(list.__test_fg().is_none());
    assert!(list.__test_highlighted_fg().is_none());
    assert!(list.__test_selected_fg().is_none());
    assert!(list.__test_disabled_fg().is_none());
    assert!(list.__test_bg().is_none());
    assert!(list.__test_highlighted_bg().is_none());
    assert_eq!(list.__test_max_visible(), 10);
    assert_eq!(list.__test_scroll_offset(), 0);
    assert!(!list.__test_focused());
    assert!(list.__test_show_icons());
}

// ============================================================================
// OptionList builder tests
// ============================================================================

#[test]
fn test_option_list_separator_style() {
    let list = OptionList::new().separator_style(SeparatorStyle::Dashed);
    assert_eq!(list.__test_separator_style(), SeparatorStyle::Dashed);
}

#[test]
fn test_option_list_title() {
    let list = OptionList::new().title("Main Menu");
    assert_eq!(list.__test_title(), &Some("Main Menu".to_string()));
}

#[test]
fn test_option_list_width() {
    let list = OptionList::new().width(60);
    assert_eq!(list.__test_width(), &Some(60));
}

#[test]
fn test_option_list_show_descriptions() {
    let list = OptionList::new().show_descriptions(true);
    assert!(list.__test_show_descriptions());
}

#[test]
fn test_option_list_colors() {
    let list = OptionList::new()
        .fg(Color::WHITE)
        .bg(Color::BLACK)
        .highlighted_fg(Color::CYAN)
        .highlighted_bg(Color::BLUE)
        .selected_fg(Color::GREEN)
        .disabled_fg(Color::rgb(128, 128, 128));

    assert_eq!(list.__test_fg(), &Some(Color::WHITE));
    assert_eq!(list.__test_bg(), &Some(Color::BLACK));
    assert_eq!(list.__test_highlighted_fg(), &Some(Color::CYAN));
    assert_eq!(list.__test_highlighted_bg(), &Some(Color::BLUE));
    assert_eq!(list.__test_selected_fg(), &Some(Color::GREEN));
    assert_eq!(list.__test_disabled_fg(), &Some(Color::rgb(128, 128, 128)));
}

#[test]
fn test_option_list_max_visible() {
    let list = OptionList::new().max_visible(5);
    assert_eq!(list.__test_max_visible(), 5);
}

#[test]
fn test_option_list_focused() {
    let list = OptionList::new().focused(true);
    assert!(list.__test_focused());
}

#[test]
fn test_option_list_show_icons() {
    let list = OptionList::new().show_icons(false);
    assert!(!list.__test_show_icons());
}

#[test]
fn test_option_list_full_builder_chain() {
    let list = OptionList::new()
        .title("Menu")
        .width(50)
        .max_visible(8)
        .focused(true)
        .show_descriptions(true)
        .show_icons(false)
        .separator_style(SeparatorStyle::Double);

    assert_eq!(list.__test_title(), &Some("Menu".to_string()));
    assert_eq!(list.__test_width(), &Some(50));
    assert_eq!(list.__test_max_visible(), 8);
    assert!(list.__test_focused());
    assert!(list.__test_show_descriptions());
    assert!(!list.__test_show_icons());
    assert_eq!(list.__test_separator_style(), SeparatorStyle::Double);
}

// ============================================================================
// OptionList::add_option tests
// ============================================================================

#[test]
fn test_add_option_full_item() {
    let list = OptionList::new().add_option(
        OptionItem::new("Full Option")
            .hint("Ctrl+F")
            .value("full")
            .icon("📄"),
    );
    assert_eq!(list.option_count(), 1);
}

// ============================================================================
// OptionList selection tests
// ============================================================================

#[test]
fn test_select() {
    let mut list = OptionList::new()
        .option("A", "")
        .option("B", "")
        .option("C", "");

    list.select(1);
    assert_eq!(list.__test_selected(), Some(1));
    assert_eq!(list.__test_highlighted(), 1);
}

#[test]
fn test_select_disabled() {
    let mut list = OptionList::new()
        .add_option(OptionItem::new("A"))
        .add_option(OptionItem::new("B").disabled(true))
        .add_option(OptionItem::new("C"));

    list.select(1); // Try to select disabled option
    assert_eq!(list.__test_selected(), None); // Should not select
}

#[test]
fn test_clear_selection() {
    let mut list = OptionList::new().option("A", "").focused(true);

    list.highlight_next();
    list.select_highlighted();
    assert!(list.__test_selected().is_some());

    list.clear_selection();
    assert!(list.__test_selected().is_none());
}

#[test]
fn test_get_selected_none() {
    let list = OptionList::new().option("A", "");
    assert!(list.get_selected().is_none());
}

#[test]
fn test_get_selected_value_fallback_to_text() {
    let mut list = OptionList::new()
        .add_option(OptionItem::new("Display Only"))
        .focused(true);

    list.select_highlighted();
    assert_eq!(list.get_selected_value(), Some("Display Only"));
}

// ============================================================================
// OptionList highlight tests
// ============================================================================

#[test]
fn test_highlight_next_at_end() {
    let mut list = OptionList::new()
        .option("A", "")
        .option("B", "")
        .focused(true);

    list.highlight_last();
    assert_eq!(list.__test_highlighted(), 1);

    list.highlight_next(); // Already at end
    assert_eq!(list.__test_highlighted(), 1);
}

#[test]
fn test_highlight_previous_at_start() {
    let mut list = OptionList::new()
        .option("A", "")
        .option("B", "")
        .focused(true);

    list.highlight_previous(); // Already at start
    assert_eq!(list.__test_highlighted(), 0);
}

#[test]
fn test_highlight_last_empty() {
    let mut list = OptionList::new().focused(true);
    list.highlight_last();
    // Empty list stays at 0
    assert_eq!(list.__test_highlighted(), 0);
}

#[test]
fn test_option_list_all_disabled() {
    let mut list = OptionList::new()
        .add_option(OptionItem::new("A").disabled(true))
        .add_option(OptionItem::new("B").disabled(true))
        .focused(true);

    list.highlight_next();
    // Should move through all disabled options to the end
    assert_eq!(list.__test_highlighted(), 1);
}

#[test]
fn test_get_highlighted_none() {
    let list = OptionList::new();
    assert!(list.get_highlighted().is_none());
}

// ============================================================================
// OptionList separator_char tests
// ============================================================================

#[test]
fn test_separator_char_line() {
    let list = OptionList::new().separator_style(SeparatorStyle::Line);
    assert_eq!(list.__test_separator_char(), "─");
}

#[test]
fn test_separator_char_dashed() {
    let list = OptionList::new().separator_style(SeparatorStyle::Dashed);
    assert_eq!(list.__test_separator_char(), "╌");
}

#[test]
fn test_separator_char_double() {
    let list = OptionList::new().separator_style(SeparatorStyle::Double);
    assert_eq!(list.__test_separator_char(), "═");
}

#[test]
fn test_separator_char_blank() {
    let list = OptionList::new().separator_style(SeparatorStyle::Blank);
    assert_eq!(list.__test_separator_char(), " ");
}

// ============================================================================
// OptionList Default tests
// ============================================================================

#[test]
fn test_option_list_default() {
    let list = OptionList::default();
    assert_eq!(list.option_count(), 0);
}

// ============================================================================
// Helper function tests
// ============================================================================

#[test]
fn test_option_list_helper() {
    let list = option_list();
    assert_eq!(list.option_count(), 0);
}

#[test]
fn test_option_item_helper() {
    let item = option_item("Test");
    assert_eq!(item.text, "Test");
}

// ============================================================================
// OptionList Clone tests
// ============================================================================

#[test]
fn test_option_list_clone() {
    let list = OptionList::new()
        .option("A", "Ctrl+A")
        .separator()
        .group("Group")
        .title("Test");

    let cloned = list.clone();
    assert_eq!(cloned.__test_entries().len(), list.__test_entries().len());
    assert_eq!(cloned.__test_title(), list.__test_title());
}

// ============================================================================
// Edge case tests
// ============================================================================

#[test]
fn test_option_list_with_empty_hint() {
    let list = OptionList::new().option("Test", "");
    // Empty hint should not be added
    assert_eq!(list.option_count(), 1);
}
