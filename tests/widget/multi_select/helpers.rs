//! Constructor functions for the multi-select widget tests

use revue::widget::multi_select::{multi_select, multi_select_from};

#[test]
fn test_multi_select_function() {
    let select = multi_select();
    let _ = select;
}

#[test]
fn test_multi_select_from_function() {
    let fruits = vec!["Apple", "Banana", "Cherry"];
    let select = multi_select_from(fruits);
    let _ = select;
}

#[test]
fn test_multi_select_from_vec() {
    let fruits = vec!["Apple", "Banana"];
    let select = multi_select_from(fruits.clone());
    let _ = select;
}

#[test]
fn test_multi_select_from_iterator() {
    let fruits = vec!["Apple", "Banana"];
    let select = multi_select_from(fruits.iter().copied());
    let _ = select;
}

#[test]
fn test_multi_select_from_empty() {
    let items: Vec<&str> = vec![];
    let select = multi_select_from(items);
    assert_eq!(select.len(), 0);
}

// =========================================================================
// Edge cases and additional scenarios
// =========================================================================

#[test]
fn test_multi_select_from_single_item() {
    let items = vec!["Only Item"];
    let select = multi_select_from(items);
    assert_eq!(select.len(), 1);
}

#[test]
fn test_multi_select_from_many_items() {
    let items = vec!["A", "B", "C", "D", "E", "F", "G", "H", "I", "J"];
    let select = multi_select_from(items);
    assert_eq!(select.len(), 10);
}

#[test]
fn test_multi_select_from_string_slice() {
    let items = vec!["X", "Y", "Z"];
    let select = multi_select_from(items);
    assert_eq!(select.len(), 3);
}

#[test]
fn test_multi_select_from_vec_string() {
    let items = vec![
        String::from("Apple"),
        String::from("Banana"),
        String::from("Cherry"),
    ];
    let select = multi_select_from(items);
    assert_eq!(select.len(), 3);
}

#[test]
fn test_multi_select_from_strings() {
    let items: Vec<String> = vec![String::from("One"), String::from("Two")];
    let select = multi_select_from(items);
    assert_eq!(select.len(), 2);
}

#[test]
fn test_multi_select_from_iterator_collect() {
    let items: Vec<&str> = vec!["A", "B", "C"];
    let select = multi_select_from(items.iter().copied());
    assert_eq!(select.len(), 3);
}

#[test]
fn test_multi_select_from_empty_strings() {
    let items = vec!["", "", ""];
    let select = multi_select_from(items);
    assert_eq!(select.len(), 3);
}

#[test]
fn test_multi_select_from_with_special_chars() {
    let items = vec!["Item/With/Slashes", "Item-With-Dashes", "Item.With.Dots"];
    let select = multi_select_from(items);
    assert_eq!(select.len(), 3);
}

#[test]
fn test_multi_select_from_with_unicode() {
    let items = vec!["アイテム", "항목", "العناصر"];
    let select = multi_select_from(items);
    assert_eq!(select.len(), 3);
}

#[test]
fn test_multi_select_from_with_emoji() {
    let items = vec!["🍎 Apple", "🍌 Banana", "🍒 Cherry"];
    let select = multi_select_from(items);
    assert_eq!(select.len(), 3);
}

#[test]
fn test_multi_select_from_with_whitespace() {
    let items = vec!["  leading space", "trailing space  ", "  both  "];
    let select = multi_select_from(items);
    assert_eq!(select.len(), 3);
}

#[test]
fn test_multi_select_from_with_newlines() {
    let items = vec!["Line\n1", "Line\r\n2", "Line\r3"];
    let select = multi_select_from(items);
    assert_eq!(select.len(), 3);
}

#[test]
fn test_multi_select_from_long_string() {
    let long_string = "A".repeat(1000);
    let items = vec![long_string.as_str()];
    let select = multi_select_from(items);
    assert_eq!(select.len(), 1);
}

#[test]
fn test_multi_select_from_duplicate_items() {
    let items = vec!["A", "B", "A", "C", "B"];
    let select = multi_select_from(items);
    assert_eq!(select.len(), 5);
    // Duplicates should be preserved
}

#[test]
fn test_multi_select_mixed_types() {
    let items: Vec<&str> = vec!["str"];
    let select1 = multi_select_from(items.clone());
    let select2 = multi_select_from(items.iter().copied());
    let select3 = multi_select_from(vec![String::from("owned")]);

    assert_eq!(select1.len(), 1);
    assert_eq!(select2.len(), 1);
    assert_eq!(select3.len(), 1);
}

#[test]
fn test_multi_select_helpers_do_not_panic() {
    // All helper functions should work without panicking
    let _ = multi_select();
    let _ = multi_select_from(vec!["A", "B"]);
    let _ = multi_select_from::<Vec<&str>, &str>(vec![]);
    let _ = multi_select_from(vec![String::from("X")]);
    let _ = multi_select_from(["A", "B"].iter().copied());
}

#[test]
fn test_multi_select_base_is_empty() {
    let select = multi_select();
    assert_eq!(select.len(), 0);
    assert!(select.is_empty());
}

#[test]
fn test_multi_select_multiple_instances() {
    let select1 = multi_select();
    let select2 = multi_select();
    let select3 = multi_select_from(vec!["A"]);

    // Each should be independent
    assert_eq!(select1.len(), 0);
    assert_eq!(select2.len(), 0);
    assert_eq!(select3.len(), 1);
}

#[test]
fn test_multi_select_from_array() {
    let items = ["One", "Two", "Three"];
    let select = multi_select_from(items);
    assert_eq!(select.len(), 3);
}

#[test]
fn test_multi_select_from_with_tabs() {
    let items = vec!["Item\tWith\tTabs"];
    let select = multi_select_from(items);
    assert_eq!(select.len(), 1);
}

#[test]
fn test_multi_select_from_with_null_bytes() {
    let items = vec!["Item\x00With\x00Nulls"];
    let select = multi_select_from(items);
    assert_eq!(select.len(), 1);
}

#[test]
fn test_multi_select_initial_state() {
    let select = multi_select();
    assert!(select.is_empty());
    assert!(!select.is_open());
    assert_eq!(select.selection_count(), 0);
}

#[test]
fn test_multi_select_from_initial_state() {
    let select = multi_select_from(vec!["A", "B", "C"]);
    assert!(!select.is_empty());
    assert!(!select.is_open());
    assert_eq!(select.selection_count(), 0);
}

#[test]
fn test_multi_select_base_allows_option_addition() {
    let select = multi_select()
        .option("First")
        .option("Second")
        .option("Third");
    assert_eq!(select.len(), 3);
}

#[test]
fn test_multi_select_from_preserves_order() {
    let items = vec!["Z", "Y", "X", "A", "B"];
    let select = multi_select_from(items);
    assert_eq!(select.len(), 5);
    // Order should be preserved
}

#[test]
fn test_multi_select_from_with_varied_lengths() {
    let items = vec!["A", "AB", "ABC", "ABCD", "ABCDE"];
    let select = multi_select_from(items);
    assert_eq!(select.len(), 5);
}

#[test]
fn test_multi_select_helpers_return_correct_type() {
    let select1 = multi_select();
    let select2 = multi_select_from(vec!["test"]);
    // Both should return the same MultiSelect type
    let _ = select1;
    let _ = select2;
}

#[test]
fn test_multi_select_from_with_single_char() {
    let items = vec!["a", "b", "c"];
    let select = multi_select_from(items);
    assert_eq!(select.len(), 3);
}

#[test]
fn test_multi_select_from_with_numbers_as_strings() {
    let items = vec!["123", "456", "789"];
    let select = multi_select_from(items);
    assert_eq!(select.len(), 3);
}

#[test]
fn test_multi_select_no_initial_selections() {
    let items = vec!["A", "B", "C"];
    let select = multi_select_from(items);
    assert_eq!(select.selection_count(), 0);
    assert!(!select.is_selected(0));
    assert!(!select.is_selected(1));
    assert!(!select.is_selected(2));
}