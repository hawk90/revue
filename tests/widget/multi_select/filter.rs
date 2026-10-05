//! Search and filter functionality for the multi-select widget tests

use revue::event::Key;
use revue::widget::MultiSelect;

fn create_test_select() -> MultiSelect {
    MultiSelect::new().options(vec!["Apple", "Banana", "Cherry", "Date", "Elderberry"])
}

// Query tests

#[test]
fn test_query_initial_empty() {
    let select = create_test_select();
    assert_eq!(select.query(), "");
}

#[test]
fn test_query_after_set() {
    let mut select = create_test_select();
    select.set_query("app");
    assert_eq!(select.query(), "app");
}

#[test]
fn test_set_query_updates_filter() {
    let mut select = create_test_select();

    select.set_query("app");

    assert_eq!(select.get_query(), "app");
    // Only "Apple" matches "app"
    assert_eq!(select.get_filtered(), &[0]);
}

#[test]
fn test_set_query_with_string() {
    let mut select = create_test_select();

    select.set_query(String::from("banana"));

    assert_eq!(select.get_query(), "banana");
    assert_eq!(select.get_filtered(), &[1]);
}

#[test]
fn test_set_query_empty() {
    let mut select = create_test_select();
    select.set_query("test");

    select.set_query("");

    assert_eq!(select.get_query(), "");
    // Filter resets to show all options
    assert_eq!(select.get_filtered(), &[0, 1, 2, 3, 4]);
    assert_eq!(select.get_dropdown_cursor(), 0);
}

#[test]
fn test_set_query_resets_cursor() {
    let mut select = create_test_select();
    select.cursor_down();
    select.cursor_down();
    select.cursor_down();
    assert_eq!(select.get_dropdown_cursor(), 3);

    select.set_query("test");

    assert_eq!(select.get_dropdown_cursor(), 0);
}

#[test]
fn test_set_query_no_matches() {
    let mut select = create_test_select();

    select.set_query("zzz");

    assert!(select.get_filtered().is_empty());
    assert_eq!(select.current_option(), None);
}

#[test]
fn test_set_query_multiple_matches_ranked() {
    let mut select = MultiSelect::new().options(vec!["Apple", "Applepie", "Pineapple", "Banana"]);

    select.set_query("app");

    // Apple, Applepie and Pineapple match; the prefix matches outrank the
    // mid-word match in "Pineapple", and "Banana" is filtered out.
    assert_eq!(select.get_filtered(), &[0, 1, 2]);
}

#[test]
fn test_clear_query() {
    let mut select = create_test_select();
    select.set_query("app");
    assert_eq!(select.get_filtered(), &[0]);

    select.clear_query();

    assert_eq!(select.get_query(), "");
    assert_eq!(select.get_filtered().len(), 5);
}

#[test]
fn test_clear_query_resets_cursor() {
    let mut select = create_test_select();
    select.set_query("e");
    select.cursor_down();
    select.cursor_down();
    assert_eq!(select.get_dropdown_cursor(), 2);

    select.clear_query();

    assert_eq!(select.get_dropdown_cursor(), 0);
}

#[test]
fn test_clear_query_already_empty() {
    let mut select = create_test_select();

    select.clear_query();

    assert_eq!(select.get_query(), "");
    assert_eq!(select.get_filtered().len(), 5);
}

// Fuzzy match tests

#[test]
fn test_get_match_with_empty_query() {
    let select = create_test_select();
    assert_eq!(select.get_match("Apple"), None);
}

#[test]
fn test_get_match_with_query() {
    let mut select = create_test_select();
    select.set_query("app");

    let m = select.get_match("Apple").expect("app should match Apple");

    assert_eq!(m.indices, vec![0, 1, 2]);
}

#[test]
fn test_get_match_no_match() {
    let mut select = create_test_select();
    select.set_query("xyz");
    assert_eq!(select.get_match("Apple"), None);
}

#[test]
fn test_get_match_partial_match() {
    let mut select = create_test_select();
    select.set_query("app");

    let m = select
        .get_match("Pineapple")
        .expect("app should match Pineapple");

    assert_eq!(m.indices, vec![4, 5, 6]);
}

#[test]
fn test_get_match_is_case_insensitive() {
    let mut select = create_test_select();
    select.set_query("APP");

    let m = select.get_match("apple").expect("matching ignores case");

    assert_eq!(m.indices, vec![0, 1, 2]);
}

// Typing into the open dropdown drives the same filter

#[test]
fn test_typing_filters_and_backspace_widens() {
    let mut select = create_test_select();
    select.handle_key(&Key::Enter); // open
    assert!(select.is_open());

    select.handle_key(&Key::Char('z'));
    select.handle_key(&Key::Char('z'));
    assert_eq!(select.get_query(), "zz");
    assert!(select.get_filtered().is_empty());

    select.handle_key(&Key::Backspace);
    select.handle_key(&Key::Backspace);
    assert_eq!(select.get_query(), "");
    assert_eq!(select.get_filtered().len(), 5);
}

// Integration tests

#[test]
fn test_search_workflow() {
    let mut select = create_test_select();

    select.set_query("app");
    assert_eq!(select.query(), "app");
    assert_eq!(select.get_filtered(), &[0]);

    select.clear_query();
    assert_eq!(select.query(), "");
    assert_eq!(select.get_filtered().len(), 5);
}

#[test]
fn test_search_and_navigate() {
    let mut select = create_test_select();
    // "an" only matches "Banana"
    select.set_query("an");
    assert_eq!(select.get_filtered(), &[1]);

    select.cursor_down();

    // Wraps around since there is only one item
    assert_eq!(select.get_dropdown_cursor(), 0);
    assert_eq!(select.current_option(), Some(1));
}

#[test]
fn test_search_after_close() {
    let mut select = create_test_select();
    select.open();
    select.set_query("app");

    select.close();

    assert_eq!(select.query(), "");
    assert_eq!(select.get_filtered().len(), 5);
}
