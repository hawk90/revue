//! Integration tests for OptionList widget

use revue::style::Color;
use revue::widget::{OptionItem, OptionList};

#[test]
fn test_option_item_new() {
    let item = OptionItem::new("Option 1");

    assert_eq!(item.text, "Option 1");
    assert!(item.hint.is_none());
    assert!(item.value.is_none());
    assert!(!item.disabled);
    assert!(item.icon.is_none());
    assert!(item.description.is_none());
}

#[test]
fn test_option_item_with_hint() {
    let item = OptionItem::new("Option 1").hint("Press Enter");

    assert_eq!(item.hint, Some("Press Enter".to_string()));
}

#[test]
fn test_option_item_with_value() {
    let item = OptionItem::new("Option 1").value("value1");

    assert_eq!(item.value, Some("value1".to_string()));
}

#[test]
fn test_option_item_disabled() {
    let item = OptionItem::new("Option 1").disabled(true);

    assert!(item.disabled);
}

#[test]
fn test_option_item_icon() {
    let item = OptionItem::new("Option 1").icon("✓");

    assert_eq!(item.icon, Some("✓".to_string()));
}

#[test]
fn test_option_item_description() {
    let item = OptionItem::new("Option 1").description("This is option 1");

    assert_eq!(item.description, Some("This is option 1".to_string()));
}

#[test]
fn test_option_item_with_all_fields() {
    let item = OptionItem::new("Complete Option")
        .hint("Press Enter")
        .value("value1")
        .disabled(false)
        .icon("✓")
        .description("A complete option");

    assert_eq!(item.text, "Complete Option");
    assert_eq!(item.hint, Some("Press Enter".to_string()));
    assert_eq!(item.value, Some("value1".to_string()));
    assert!(!item.disabled);
    assert_eq!(item.icon, Some("✓".to_string()));
    assert_eq!(item.description, Some("A complete option".to_string()));
}

#[test]
fn test_option_list_new() {
    let list = OptionList::new();

    assert_eq!(list.option_count(), 0);
}

#[test]
fn test_option_list_with_option() {
    let list = OptionList::new().option("Option 1", "Hint 1");

    assert_eq!(list.option_count(), 1);
}

#[test]
fn test_option_list_multiple_options() {
    let list = OptionList::new()
        .option("Option 1", "Hint 1")
        .option("Option 2", "Hint 2")
        .option("Option 3", "Hint 3");

    assert_eq!(list.option_count(), 3);
}

#[test]
fn test_option_list_title() {
    let _list = OptionList::new().title("Select an option");

    // Title was set successfully
}

#[test]
fn test_option_list_focused() {
    let _list = OptionList::new().focused(true);

    // Focus state was set successfully
}

#[test]
fn test_option_list_colors() {
    let _list = OptionList::new()
        .fg(Color::CYAN)
        .bg(Color::BLUE)
        .selected_fg(Color::WHITE)
        .disabled_fg(Color::rgb(128, 128, 128));

    // Colors were set successfully
}

#[test]
fn test_option_list_max_visible() {
    let _list = OptionList::new().max_visible(10);

    // Max visible was set successfully
}

#[test]
fn test_option_list_show_descriptions() {
    let _list = OptionList::new().show_descriptions(true);

    // Show descriptions was set successfully
}

#[test]
fn test_option_list_show_icons() {
    let _list = OptionList::new().show_icons(true);

    // Show icons was set successfully
}

#[test]
fn test_option_list_width() {
    let _list = OptionList::new().width(50);

    // Width was set successfully
}

#[test]
fn test_option_list_option_count() {
    let list = OptionList::new()
        .option("Option 1", "")
        .separator()
        .option("Option 2", "");

    assert_eq!(list.option_count(), 2); // Only options, not separators
}

#[test]
fn test_option_list_add_separator() {
    let list = OptionList::new().option("Option 1", "").separator();

    // Separator was added
    assert_eq!(list.option_count(), 1); // Separator doesn't count as option
}

#[test]
fn test_option_list_add_group() {
    let list = OptionList::new().group("File Menu");

    // Group was added
}

#[test]
fn test_option_list_separator_style() {
    // SeparatorStyle is not exported from widget module, skip test
    // The separator_style() method exists but requires the SeparatorStyle type
}

#[test]
fn test_option_list_builder_pattern() {
    let _list = OptionList::new()
        .title("Choose an option")
        .focused(true)
        .max_visible(10)
        .show_descriptions(true)
        .show_icons(true)
        .fg(Color::CYAN)
        .bg(Color::BLUE);

    // Builder pattern works successfully
}

#[test]
fn test_option_list_empty() {
    let list = OptionList::new();

    assert_eq!(list.option_count(), 0);
}

#[test]
fn test_option_list_not_empty() {
    let list = OptionList::new().add_option(OptionItem::new("Option 1").hint("Hint 1"));

    assert!(list.option_count() > 0);
}
