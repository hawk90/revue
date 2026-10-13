//! Autocomplete builder and state tests: construction, suggestions, value, placeholder, focus, the selected suggestion, CSS ids and classes

use super::{dropdown, fruits, key, render_rows, type_str};
use revue::event::Key;
use revue::style::Color;
use revue::utils::FilterMode;
use revue::widget::autocomplete::{autocomplete, Autocomplete, Suggestion};
use revue::widget::traits::StyledView;
use revue::widget::traits::View;

#[test]
fn test_autocomplete_new() {
    let a = Autocomplete::new();
    assert_eq!(a.get_value(), "");
    assert!(!a.is_focused());
    assert!(a.selected_suggestion().is_none());
}

#[test]
fn test_autocomplete_default() {
    let a = Autocomplete::default();
    assert_eq!(a.get_value(), "");
    assert!(!a.is_focused());
}

#[test]
fn test_autocomplete_helper() {
    let a = autocomplete();
    assert_eq!(a.get_value(), "");
}

#[test]
fn test_autocomplete_suggestions_from_strings() {
    let mut a = Autocomplete::new().suggestions(vec!["test".to_string()]);
    type_str(&mut a, "t");
    assert_eq!(a.selected_suggestion().unwrap().label, "test");
}

#[test]
fn test_autocomplete_suggestions_from_slice() {
    let mut a = Autocomplete::new()
        .suggestions(["a", "b", "c"])
        .filter_mode(FilterMode::None);
    type_str(&mut a, "x");
    assert_eq!(dropdown(&a, 10, 5), vec!["a", "b", "c"]);
}

#[test]
fn test_autocomplete_suggestions_with_suggestion_structs() {
    let mut a = Autocomplete::new().suggestions(vec![
        Suggestion::with_value("Item 1", "one"),
        Suggestion::new("Item 2"),
    ]);
    type_str(&mut a, "Item");
    assert_eq!(dropdown(&a, 10, 5), vec!["Item 1", "Item 2"]);
    assert!(a.accept_selection());
    assert_eq!(a.get_value(), "one");
}

#[test]
fn test_autocomplete_suggestions_empty() {
    let mut a = Autocomplete::new().suggestions::<Vec<String>, String>(vec![]);
    type_str(&mut a, "a");
    assert!(a.selected_suggestion().is_none());
    assert!(dropdown(&a, 10, 5).is_empty());
}

#[test]
fn test_autocomplete_value() {
    let a = Autocomplete::new().value("test");
    assert_eq!(a.get_value(), "test");
}

#[test]
fn test_autocomplete_value_string() {
    let a = Autocomplete::new().value(String::from("owned"));
    assert_eq!(a.get_value(), "owned");
}

#[test]
fn test_autocomplete_value_empty() {
    let a = Autocomplete::new().value("");
    assert_eq!(a.get_value(), "");
}

#[test]
fn test_autocomplete_value_puts_cursor_at_end() {
    let mut a = Autocomplete::new().value("test");
    type_str(&mut a, "!");
    assert_eq!(a.get_value(), "test!");
}

#[test]
fn test_autocomplete_placeholder() {
    let a = Autocomplete::new().placeholder("Search...");
    assert_eq!(render_rows(&a, 12, 1)[0], "Search...   ");
}

#[test]
fn test_autocomplete_placeholder_string() {
    let a = Autocomplete::new().placeholder(String::from("Type here"));
    assert_eq!(render_rows(&a, 12, 1)[0].trim_end(), "Type here");
}

#[test]
fn test_autocomplete_placeholder_empty() {
    let a = Autocomplete::new().placeholder("");
    assert_eq!(render_rows(&a, 12, 1)[0].trim(), "");
}

#[test]
fn test_autocomplete_builder_chain() {
    let a = Autocomplete::new()
        .value("test")
        .placeholder("Search")
        .filter_mode(FilterMode::Prefix)
        .min_chars(2)
        .max_suggestions(5)
        .input_style(Color::WHITE, Color::BLACK)
        .dropdown_style(Color::GREEN, Color::WHITE, Color::BLUE)
        .highlight_fg(Color::YELLOW)
        .suggestions(vec!["a", "b", "c"]);

    assert_eq!(a.get_value(), "test");
}

#[test]
fn test_autocomplete_get_value_unicode() {
    let a = Autocomplete::new().value("こんにちは");
    assert_eq!(a.get_value(), "こんにちは");
}

#[test]
fn test_autocomplete_selected_suggestion_none() {
    let a = Autocomplete::new();
    assert!(a.selected_suggestion().is_none());
}

#[test]
fn test_autocomplete_selected_suggestion_with_suggestions() {
    let mut a = fruits().value("app");
    // The builder value does not filter until focus
    assert!(a.selected_suggestion().is_none());
    a.focus();
    assert_eq!(a.selected_suggestion().unwrap().label, "apple");
}

#[test]
fn test_autocomplete_is_focused_default() {
    let a = Autocomplete::new();
    assert!(!a.is_focused());
}

#[test]
fn test_autocomplete_set_value_string() {
    let mut a = Autocomplete::new();
    a.set_value("test");
    assert_eq!(a.get_value(), "test");
}

#[test]
fn test_autocomplete_set_value_empty() {
    let mut a = Autocomplete::new().value("x");
    a.set_value("");
    assert_eq!(a.get_value(), "");
}

#[test]
fn test_autocomplete_set_value_overwrite() {
    let mut a = Autocomplete::new().value("first");
    a.set_value("second");
    assert_eq!(a.get_value(), "second");
}

#[test]
fn test_autocomplete_set_value_unicode() {
    let mut a = Autocomplete::new();
    a.set_value("Привет");
    assert_eq!(a.get_value(), "Привет");
}

#[test]
fn test_autocomplete_set_suggestions_empty() {
    let mut a = fruits().value("a");
    a.focus();
    assert!(a.selected_suggestion().is_some());
    a.set_suggestions(vec![]);
    assert!(a.selected_suggestion().is_none());
}

#[test]
fn test_autocomplete_set_suggestions_vec() {
    let mut a = Autocomplete::new().value("Item");
    a.set_suggestions(vec![Suggestion::new("Item 1"), Suggestion::new("Item 2")]);
    assert_eq!(dropdown(&a, 10, 5), vec!["Item 1", "Item 2"]);
}

#[test]
fn test_autocomplete_set_suggestions_overwrite() {
    let mut a = Autocomplete::new().value("x").filter_mode(FilterMode::None);
    a.set_suggestions(vec!["a".into(), "b".into()]);
    a.set_suggestions(vec!["c".into(), "d".into(), "e".into()]);
    assert_eq!(dropdown(&a, 10, 5), vec!["c", "d", "e"]);
}

#[test]
fn test_autocomplete_focus() {
    let mut a = Autocomplete::new();
    a.focus();
    assert!(a.is_focused());
}

#[test]
fn test_autocomplete_focus_twice() {
    let mut a = Autocomplete::new();
    a.focus();
    a.focus();
    assert!(a.is_focused());
}

#[test]
fn test_autocomplete_blur() {
    let mut a = Autocomplete::new();
    a.focus();
    a.blur();
    assert!(!a.is_focused());
}

#[test]
fn test_autocomplete_blur_twice() {
    let mut a = Autocomplete::new();
    a.focus();
    a.blur();
    a.blur();
    assert!(!a.is_focused());
}

#[test]
fn test_autocomplete_blur_without_focus() {
    let mut a = Autocomplete::new();
    a.blur();
    assert!(!a.is_focused());
}

#[test]
fn test_autocomplete_blur_hides_dropdown() {
    let mut a = fruits().value("app");
    a.focus();
    assert!(!dropdown(&a, 20, 6).is_empty());
    a.blur();
    assert!(dropdown(&a, 20, 6).is_empty());
    // Navigation keys are no longer consumed
    assert!(!key(&mut a, Key::Down));
}

#[test]
fn test_autocomplete_focus_blur_focus_cycle() {
    let mut a = Autocomplete::new();
    assert!(!a.is_focused());
    a.focus();
    assert!(a.is_focused());
    a.blur();
    assert!(!a.is_focused());
    a.focus();
    assert!(a.is_focused());
}

#[test]
fn test_autocomplete_accept_selection_none() {
    let mut a = Autocomplete::new();
    assert!(!a.accept_selection());
    assert_eq!(a.get_value(), "");
}

#[test]
fn test_autocomplete_accept_selection_with_suggestions() {
    let mut a = fruits().value("app");
    a.focus();
    assert!(a.accept_selection());
    assert_eq!(a.get_value(), "apple");
    // Dropdown closes after accepting
    assert!(dropdown(&a, 20, 6).is_empty());
}

#[test]
fn test_autocomplete_element_id() {
    let a = Autocomplete::new().element_id("search-box");
    assert_eq!(View::id(&a), Some("search-box"));
}

#[test]
fn test_autocomplete_view_id_none() {
    let a = Autocomplete::new();
    assert_eq!(View::id(&a), None);
}

#[test]
fn test_autocomplete_classes() {
    let a = Autocomplete::new().class("input").class("search");
    assert!(a.has_class("input"));
    assert!(a.has_class("search"));
    assert!(!a.has_class("hidden"));
}

#[test]
fn test_autocomplete_styled_view_methods() {
    let mut a = Autocomplete::new();

    a.set_id("my-autocomplete");
    assert_eq!(View::id(&a), Some("my-autocomplete"));

    a.add_class("active");
    assert!(a.has_class("active"));

    a.remove_class("active");
    assert!(!a.has_class("active"));

    a.toggle_class("visible");
    assert!(a.has_class("visible"));

    a.toggle_class("visible");
    assert!(!a.has_class("visible"));
}

#[test]
fn test_autocomplete_meta() {
    let a = Autocomplete::new()
        .element_id("my-element")
        .class("class1")
        .class("class2");

    let meta = a.meta();
    assert_eq!(meta.id, Some("my-element".to_string()));
    assert_eq!(meta.classes.len(), 2);
    assert!(meta.classes.contains("class1"));
    assert!(meta.classes.contains("class2"));
}

#[test]
fn test_autocomplete_multiple_independent_instances() {
    let mut a1 = Autocomplete::new().value("first");
    let mut a2 = Autocomplete::new().value("second");

    key(&mut a1, Key::Char('x'));
    key(&mut a2, Key::Char('y'));

    assert_eq!(a1.get_value(), "firstx");
    assert_eq!(a2.get_value(), "secondy");
}
