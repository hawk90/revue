//! Core types for the multi-select widget tests

use revue::style::Color;
use revue::widget::multi_select::{MultiSelect, MultiSelectOption};

// MultiSelectOption tests

#[test]
fn test_multiselectoption_new() {
    // Arrange & Act
    let option = MultiSelectOption::new("Apple", "apple");

    // Assert
    assert_eq!(option.label, "Apple");
    assert_eq!(option.value, "apple");
    assert!(!option.disabled);
}

#[test]
fn test_multiselectoption_new_with_string_types() {
    // Arrange & Act
    let option = MultiSelectOption::new(String::from("Label"), String::from("value"));

    // Assert
    assert_eq!(option.label, "Label");
    assert_eq!(option.value, "value");
}

#[test]
fn test_multiselectoption_simple() {
    // Arrange & Act
    let option = MultiSelectOption::simple("Apple");

    // Assert
    assert_eq!(option.label, "Apple");
    assert_eq!(option.value, "Apple");
    assert!(!option.disabled);
}

#[test]
fn test_multiselectoption_disabled_true() {
    // Arrange & Act
    let option = MultiSelectOption::simple("Apple").disabled(true);

    // Assert
    assert!(option.disabled);
}

#[test]
fn test_multiselectoption_disabled_false() {
    // Arrange & Act
    let option = MultiSelectOption::simple("Apple").disabled(false);

    // Assert
    assert!(!option.disabled);
}

#[test]
fn test_multiselectoption_clone() {
    // Arrange
    let option1 = MultiSelectOption::new("Apple", "apple").disabled(true);

    // Act
    let option2 = option1.clone();

    // Assert
    assert_eq!(option1.label, option2.label);
    assert_eq!(option1.value, option2.value);
    assert_eq!(option1.disabled, option2.disabled);
}

// MultiSelect tests - Constructors

#[test]
fn test_multiselect_new() {
    // Arrange & Act
    let select = MultiSelect::new();

    // Assert
    assert!(select.options.is_empty());
    assert!(select.selected.is_empty());
    assert!(!select.open);
    assert_eq!(select.dropdown_cursor, 0);
    assert_eq!(select.tag_cursor, None);
    assert!(select.query.is_empty());
    assert!(select.filtered.is_empty());
    assert_eq!(select.placeholder, "Select...");
    assert_eq!(select.max_selections, None);
    assert_eq!(select.width, None);
    assert!(select.searchable);
    assert_eq!(select.highlight_fg, Some(Color::YELLOW));
    assert_eq!(select.tag_bg, Some(Color::rgb(60, 60, 140)));
}

#[test]
fn test_multiselect_default() {
    // Arrange & Act
    let select = MultiSelect::default();

    // Assert
    assert!(select.options.is_empty());
    assert!(select.selected.is_empty());
    assert_eq!(select.placeholder, "Select...");
}

#[test]
fn test_multiselect_clone() {
    // Arrange
    let select1 = MultiSelect::new()
        .options(vec!["Apple", "Banana"])
        .selected_indices(vec![0]);

    // Act
    let select2 = select1.clone();

    // Assert
    assert_eq!(select1.options.len(), select2.options.len());
    assert_eq!(select1.selected.len(), select2.selected.len());
    assert_eq!(select1.placeholder, select2.placeholder);
}

// MultiSelect tests - Builder methods

#[test]
fn test_multiselect_options() {
    // Arrange
    let options = vec!["Apple", "Banana", "Cherry"];

    // Act
    let select = MultiSelect::new().options(options);

    // Assert
    assert_eq!(select.options.len(), 3);
    assert_eq!(select.options[0].label, "Apple");
    assert_eq!(select.options[1].label, "Banana");
    assert_eq!(select.options[2].label, "Cherry");
}

#[test]
fn test_multiselect_options_empty() {
    // Arrange & Act
    let select = MultiSelect::new().options(vec![""; 0]);

    // Assert
    assert!(select.options.is_empty());
    assert!(select.filtered.is_empty());
}

#[test]
fn test_multiselect_options_resets_filter() {
    // Arrange
    let mut select = MultiSelect::new().options(vec!["Apple"]);
    select.filtered = vec![];

    // Act
    select = select.options(vec!["Banana", "Cherry"]);

    // Assert
    assert_eq!(select.filtered.len(), 2);
}

#[test]
fn test_multiselect_options_detailed() {
    // Arrange
    let options = vec![
        MultiSelectOption::new("Apple", "apple").disabled(true),
        MultiSelectOption::new("Banana", "banana"),
    ];

    // Act
    let select = MultiSelect::new().options_detailed(options);

    // Assert
    assert_eq!(select.options.len(), 2);
    assert!(select.options[0].disabled);
    assert!(!select.options[1].disabled);
}

#[test]
fn test_multiselect_option() {
    // Arrange & Act
    let select = MultiSelect::new()
        .option("Apple")
        .option("Banana")
        .option("Cherry");

    // Assert
    assert_eq!(select.options.len(), 3);
    assert_eq!(select.options[0].label, "Apple");
    assert_eq!(select.options[1].label, "Banana");
    assert_eq!(select.options[2].label, "Cherry");
}

#[test]
fn test_multiselect_option_resets_filter() {
    // Arrange
    let mut select = MultiSelect::new();
    select.filtered = vec![];

    // Act
    select = select.option("Apple");

    // Assert
    assert_eq!(select.filtered.len(), 1);
}

#[test]
fn test_multiselect_option_detailed() {
    // Arrange
    let option = MultiSelectOption::new("Apple", "apple").disabled(true);

    // Act
    let select = MultiSelect::new().option_detailed(option);

    // Assert
    assert_eq!(select.options.len(), 1);
    assert!(select.options[0].disabled);
}

#[test]
fn test_multiselect_selected_indices_valid() {
    // Arrange & Act
    let select = MultiSelect::new()
        .options(vec!["Apple", "Banana", "Cherry"])
        .selected_indices(vec![0, 2]);

    // Assert
    assert_eq!(select.selected.len(), 2);
    assert_eq!(select.selected[0], 0);
    assert_eq!(select.selected[1], 2);
}

#[test]
fn test_multiselect_selected_indices_filters_invalid() {
    // Arrange & Act
    let select = MultiSelect::new()
        .options(vec!["Apple", "Banana", "Cherry"])
        .selected_indices(vec![0, 2, 5, 10]);

    // Assert
    assert_eq!(select.selected.len(), 2);
    assert!(!select.selected.contains(&5));
    assert!(!select.selected.contains(&10));
}

#[test]
fn test_multiselect_selected_indices_empty() {
    // Arrange & Act
    let select = MultiSelect::new()
        .options(vec!["Apple", "Banana"])
        .selected_indices(vec![]);

    // Assert
    assert!(select.selected.is_empty());
}

#[test]
fn test_multiselect_selected_values() {
    // Arrange & Act
    let select = MultiSelect::new()
        .options(vec!["Apple", "Banana", "Cherry"])
        .selected_values(vec!["Apple", "Cherry"]);

    // Assert
    assert_eq!(select.selected.len(), 2);
    assert_eq!(select.selected[0], 0);
    assert_eq!(select.selected[1], 2);
}

#[test]
fn test_multiselect_selected_values_not_found() {
    // Arrange & Act
    let select = MultiSelect::new()
        .options(vec!["Apple", "Banana"])
        .selected_values(vec!["Apple", "Orange"]);

    // Assert
    assert_eq!(select.selected.len(), 1);
    assert_eq!(select.selected[0], 0);
}

#[test]
fn test_multiselect_placeholder() {
    // Arrange & Act
    let select = MultiSelect::new().placeholder("Choose items...");

    // Assert
    assert_eq!(select.placeholder, "Choose items...");
}

#[test]
fn test_multiselect_placeholder_with_string() {
    // Arrange & Act
    let select = MultiSelect::new().placeholder(String::from("Custom placeholder"));

    // Assert
    assert_eq!(select.placeholder, "Custom placeholder");
}

#[test]
fn test_multiselect_max_selections() {
    // Arrange & Act
    let select = MultiSelect::new().max_selections(3);

    // Assert
    assert_eq!(select.max_selections, Some(3));
}

#[test]
fn test_multiselect_width() {
    // Arrange & Act
    let select = MultiSelect::new().width(50);

    // Assert
    assert_eq!(select.width, Some(50));
}

#[test]
fn test_multiselect_searchable_true() {
    // Arrange & Act
    let select = MultiSelect::new().searchable(true);

    // Assert
    assert!(select.searchable);
}

#[test]
fn test_multiselect_searchable_false() {
    // Arrange & Act
    let select = MultiSelect::new().searchable(false);

    // Assert
    assert!(!select.searchable);
}

#[test]
fn test_multiselect_highlight_fg() {
    // Arrange & Act
    let select = MultiSelect::new().highlight_fg(Color::RED);

    // Assert
    assert_eq!(select.highlight_fg, Some(Color::RED));
}

#[test]
fn test_multiselect_tag_bg() {
    // Arrange & Act
    let select = MultiSelect::new().tag_bg(Color::BLUE);

    // Assert
    assert_eq!(select.tag_bg, Some(Color::BLUE));
}

// MultiSelect tests - Display width

#[test]
fn test_display_width_with_custom_width() {
    // Arrange
    let select = MultiSelect::new().width(30);

    // Act
    let width = select.display_width(100);

    // Assert
    assert_eq!(width, 30);
}

#[test]
fn test_display_width_custom_width_capped_by_max() {
    // Arrange
    let select = MultiSelect::new().width(150);

    // Act
    let width = select.display_width(100);

    // Assert
    assert_eq!(width, 100);
}

#[test]
fn test_display_width_from_options() {
    // Arrange
    let select = MultiSelect::new().options(vec!["Apple", "Banana", "Strawberry"]);

    // Act
    let width = select.display_width(100);

    // Assert
    // "Strawberry" (10) + 4 = 14
    assert_eq!(width, 14);
}

#[test]
fn test_display_width_from_placeholder_when_empty() {
    // Arrange
    let select = MultiSelect::new().placeholder("Select an item please");

    // Act
    let width = select.display_width(100);

    // Assert
    // "Select an item please" (21) + 4 = 25
    assert_eq!(width, 25);
}

#[test]
fn test_display_width_capped_by_max() {
    // Arrange
    let select = MultiSelect::new().options(vec!["Very long option name that exceeds maximum"]);

    // Act
    let width = select.display_width(20);

    // Assert
    assert_eq!(width, 20);
}

// MultiSelect tests - Chained builders

#[test]
fn test_multiselect_builder_chain() {
    // Arrange & Act
    let select = MultiSelect::new()
        .options(vec!["Apple", "Banana", "Cherry"])
        .selected_indices(vec![0, 1])
        .placeholder("Pick fruits")
        .max_selections(2)
        .width(40)
        .searchable(false)
        .highlight_fg(Color::GREEN)
        .tag_bg(Color::RED);

    // Assert
    assert_eq!(select.options.len(), 3);
    assert_eq!(select.selected.len(), 2);
    assert_eq!(select.placeholder, "Pick fruits");
    assert_eq!(select.max_selections, Some(2));
    assert_eq!(select.width, Some(40));
    assert!(!select.searchable);
    assert_eq!(select.highlight_fg, Some(Color::GREEN));
    assert_eq!(select.tag_bg, Some(Color::RED));
}