//! Core types for the multi-select widget tests

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::View;
use revue::widget::{MultiSelect, MultiSelectOption};

/// Every option label, in option order (read through `select_all`).
fn labels(select: &MultiSelect) -> Vec<String> {
    let mut all = select.clone();
    all.select_all();
    all.get_selected_labels()
        .into_iter()
        .map(String::from)
        .collect()
}

/// Render into an `area_width` x 1 buffer and return the column of the
/// dropdown arrow, which the widget draws in the last column it occupies.
fn rendered_width(select: &MultiSelect, area_width: u16) -> u16 {
    let mut buffer = Buffer::new(area_width, 1);
    let area = Rect::new(0, 0, area_width, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    select.render(&mut ctx);
    let arrow = (0..area_width)
        .find(|&x| buffer.get(x, 0).map(|c| c.symbol) == Some('▼'))
        .expect("dropdown arrow should be drawn");
    arrow + 1
}

// MultiSelectOption tests

#[test]
fn test_multiselectoption_new() {
    let option = MultiSelectOption::new("Apple", "apple");

    assert_eq!(option.label, "Apple");
    assert_eq!(option.value, "apple");
    assert!(!option.disabled);
}

#[test]
fn test_multiselectoption_new_with_string_types() {
    let option = MultiSelectOption::new(String::from("Label"), String::from("value"));

    assert_eq!(option.label, "Label");
    assert_eq!(option.value, "value");
}

#[test]
fn test_multiselectoption_simple() {
    let option = MultiSelectOption::simple("Apple");

    assert_eq!(option.label, "Apple");
    assert_eq!(option.value, "Apple");
    assert!(!option.disabled);
}

#[test]
fn test_multiselectoption_disabled_true() {
    let option = MultiSelectOption::simple("Apple").disabled(true);
    assert!(option.disabled);
}

#[test]
fn test_multiselectoption_disabled_false() {
    let option = MultiSelectOption::simple("Apple").disabled(false);
    assert!(!option.disabled);
}

#[test]
fn test_multiselectoption_clone() {
    let option1 = MultiSelectOption::new("Apple", "apple").disabled(true);

    let option2 = option1.clone();

    assert_eq!(option1.label, option2.label);
    assert_eq!(option1.value, option2.value);
    assert_eq!(option1.disabled, option2.disabled);
}

// MultiSelect tests - Constructors

#[test]
fn test_multiselect_new() {
    let select = MultiSelect::new();

    assert!(select.is_empty());
    assert!(select.get_selected_indices().is_empty());
    assert!(!select.is_open());
    assert_eq!(select.get_dropdown_cursor(), 0);
    assert_eq!(select.get_tag_cursor(), None);
    assert!(select.get_query().is_empty());
    assert!(select.get_filtered().is_empty());
    assert_eq!(select.get_placeholder(), "Select...");
    assert_eq!(select.get_max_selections(), None);
    assert_eq!(select.get_width(), None);
    assert!(select.get_searchable());
    assert_eq!(select.get_highlight_fg(), Some(Color::YELLOW));
    assert_eq!(select.get_tag_bg(), Some(Color::rgb(60, 60, 140)));
}

#[test]
fn test_multiselect_default() {
    let select = MultiSelect::default();

    assert!(select.is_empty());
    assert!(select.get_selected_indices().is_empty());
    assert_eq!(select.get_placeholder(), "Select...");
}

#[test]
fn test_multiselect_clone() {
    let select1 = MultiSelect::new()
        .options(vec!["Apple", "Banana"])
        .selected_indices(vec![0]);

    let select2 = select1.clone();

    assert_eq!(select1.len(), select2.len());
    assert_eq!(
        select1.get_selected_indices(),
        select2.get_selected_indices()
    );
    assert_eq!(select1.get_placeholder(), select2.get_placeholder());
    assert_eq!(labels(&select1), labels(&select2));
}

// MultiSelect tests - Builder methods

#[test]
fn test_multiselect_options() {
    let select = MultiSelect::new().options(vec!["Apple", "Banana", "Cherry"]);

    assert_eq!(select.len(), 3);
    assert_eq!(labels(&select), vec!["Apple", "Banana", "Cherry"]);
}

#[test]
fn test_multiselect_options_empty() {
    let select = MultiSelect::new().options(vec![""; 0]);

    assert!(select.is_empty());
    assert!(select.get_filtered().is_empty());
}

#[test]
fn test_multiselect_options_resets_filter() {
    let mut select = MultiSelect::new().options(vec!["Apple"]);
    select.set_query("zzz");
    assert!(select.get_filtered().is_empty());

    select = select.options(vec!["Banana", "Cherry"]);

    assert_eq!(select.get_filtered(), &[0, 1]);
}

#[test]
fn test_multiselect_options_detailed() {
    let options = vec![
        MultiSelectOption::new("Apple", "apple").disabled(true),
        MultiSelectOption::new("Banana", "banana"),
    ];

    let mut select = MultiSelect::new().options_detailed(options);

    assert_eq!(select.len(), 2);
    // The disabled option cannot be selected, the enabled one can
    select.select_option(0);
    assert!(!select.is_selected(0));
    select.select_option(1);
    assert!(select.is_selected(1));
    assert_eq!(select.get_selected_values(), vec!["banana"]);
}

#[test]
fn test_multiselect_option() {
    let select = MultiSelect::new()
        .option("Apple")
        .option("Banana")
        .option("Cherry");

    assert_eq!(select.len(), 3);
    assert_eq!(labels(&select), vec!["Apple", "Banana", "Cherry"]);
}

#[test]
fn test_multiselect_option_resets_filter() {
    let mut select = MultiSelect::new().option("Banana");
    select.set_query("zzz");
    assert!(select.get_filtered().is_empty());

    select = select.option("Apple");

    assert_eq!(select.get_filtered(), &[0, 1]);
}

#[test]
fn test_multiselect_option_detailed() {
    let option = MultiSelectOption::new("Apple", "apple").disabled(true);

    let mut select = MultiSelect::new().option_detailed(option);

    assert_eq!(select.len(), 1);
    select.select_option(0);
    assert!(!select.is_selected(0));
}

#[test]
fn test_multiselect_selected_indices_valid() {
    let select = MultiSelect::new()
        .options(vec!["Apple", "Banana", "Cherry"])
        .selected_indices(vec![0, 2]);

    assert_eq!(select.get_selected_indices(), &[0, 2]);
}

#[test]
fn test_multiselect_selected_indices_filters_invalid() {
    let select = MultiSelect::new()
        .options(vec!["Apple", "Banana", "Cherry"])
        .selected_indices(vec![0, 2, 5, 10]);

    assert_eq!(select.get_selected_indices(), &[0, 2]);
}

#[test]
fn test_multiselect_selected_indices_empty() {
    let select = MultiSelect::new()
        .options(vec!["Apple", "Banana"])
        .selected_indices(vec![]);

    assert!(select.get_selected_indices().is_empty());
}

#[test]
fn test_multiselect_selected_values() {
    let select = MultiSelect::new()
        .options(vec!["Apple", "Banana", "Cherry"])
        .selected_values(vec!["Apple", "Cherry"]);

    assert_eq!(select.get_selected_indices(), &[0, 2]);
}

#[test]
fn test_multiselect_selected_values_not_found() {
    let select = MultiSelect::new()
        .options(vec!["Apple", "Banana"])
        .selected_values(vec!["Apple", "Orange"]);

    assert_eq!(select.get_selected_indices(), &[0]);
}

#[test]
fn test_multiselect_placeholder() {
    let select = MultiSelect::new().placeholder("Choose items...");
    assert_eq!(select.get_placeholder(), "Choose items...");
}

#[test]
fn test_multiselect_placeholder_with_string() {
    let select = MultiSelect::new().placeholder(String::from("Custom placeholder"));
    assert_eq!(select.get_placeholder(), "Custom placeholder");
}

#[test]
fn test_multiselect_max_selections() {
    let select = MultiSelect::new().max_selections(3);
    assert_eq!(select.get_max_selections(), Some(3));
}

#[test]
fn test_multiselect_width() {
    let select = MultiSelect::new().width(50);
    assert_eq!(select.get_width(), Some(50));
}

#[test]
fn test_multiselect_searchable_true() {
    let select = MultiSelect::new().searchable(true);
    assert!(select.get_searchable());
}

#[test]
fn test_multiselect_searchable_false() {
    let select = MultiSelect::new().searchable(false);
    assert!(!select.get_searchable());
}

#[test]
fn test_multiselect_highlight_fg() {
    let select = MultiSelect::new().highlight_fg(Color::RED);
    assert_eq!(select.get_highlight_fg(), Some(Color::RED));
}

#[test]
fn test_multiselect_tag_bg() {
    let select = MultiSelect::new().tag_bg(Color::BLUE);
    assert_eq!(select.get_tag_bg(), Some(Color::BLUE));
}

// MultiSelect tests - Display width (observed through the rendered arrow)

#[test]
fn test_display_width_with_custom_width() {
    let select = MultiSelect::new().width(30);
    assert_eq!(rendered_width(&select, 100), 30);
}

#[test]
fn test_display_width_custom_width_capped_by_max() {
    let select = MultiSelect::new().width(150);
    assert_eq!(rendered_width(&select, 100), 100);
}

#[test]
fn test_display_width_from_options() {
    let select = MultiSelect::new().options(vec!["Apple", "Banana", "Strawberry"]);
    // "Strawberry" (10) + 4 = 14
    assert_eq!(rendered_width(&select, 100), 14);
}

#[test]
fn test_display_width_from_placeholder_when_empty() {
    let select = MultiSelect::new().placeholder("Select an item please");
    // "Select an item please" (21) + 4 = 25
    assert_eq!(rendered_width(&select, 100), 25);
}

#[test]
fn test_display_width_capped_by_max() {
    let select = MultiSelect::new().options(vec!["Very long option name that exceeds maximum"]);
    assert_eq!(rendered_width(&select, 20), 20);
}

// MultiSelect tests - Chained builders

#[test]
fn test_multiselect_builder_chain() {
    let select = MultiSelect::new()
        .options(vec!["Apple", "Banana", "Cherry"])
        .selected_indices(vec![0, 1])
        .placeholder("Pick fruits")
        .max_selections(2)
        .width(40)
        .searchable(false)
        .highlight_fg(Color::GREEN)
        .tag_bg(Color::RED);

    assert_eq!(select.len(), 3);
    assert_eq!(select.get_selected_indices(), &[0, 1]);
    assert_eq!(select.get_placeholder(), "Pick fruits");
    assert_eq!(select.get_max_selections(), Some(2));
    assert_eq!(select.get_width(), Some(40));
    assert!(!select.get_searchable());
    assert_eq!(select.get_highlight_fg(), Some(Color::GREEN));
    assert_eq!(select.get_tag_bg(), Some(Color::RED));
}
