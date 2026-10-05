//! Constructor functions for the multi-select widget tests

use revue::widget::{multi_select, multi_select_from, MultiSelect};

/// Every option label, in option order (read through `select_all`).
fn labels(select: &MultiSelect) -> Vec<String> {
    let mut all = select.clone();
    all.select_all();
    all.get_selected_labels()
        .into_iter()
        .map(String::from)
        .collect()
}

#[test]
fn test_multi_select_base_is_empty_and_closed() {
    let select = multi_select();
    assert_eq!(select.len(), 0);
    assert!(select.is_empty());
    assert!(!select.is_open());
    assert_eq!(select.selection_count(), 0);
}

#[test]
fn test_multi_select_from_empty() {
    let items: Vec<&str> = vec![];
    let select = multi_select_from(items);
    assert_eq!(select.len(), 0);
    assert!(select.is_empty());
}

#[test]
fn test_multi_select_from_single_item() {
    let select = multi_select_from(vec!["Only Item"]);
    assert_eq!(select.len(), 1);
    assert_eq!(labels(&select), vec!["Only Item"]);
}

#[test]
fn test_multi_select_from_vec_string() {
    let items = vec![
        String::from("Apple"),
        String::from("Banana"),
        String::from("Cherry"),
    ];
    let select = multi_select_from(items);
    assert_eq!(labels(&select), vec!["Apple", "Banana", "Cherry"]);
}

#[test]
fn test_multi_select_from_iterator() {
    let fruits = ["Apple", "Banana"];
    let select = multi_select_from(fruits.iter().copied());
    assert_eq!(labels(&select), vec!["Apple", "Banana"]);
}

#[test]
fn test_multi_select_from_array() {
    let select = multi_select_from(["One", "Two", "Three"]);
    assert_eq!(labels(&select), vec!["One", "Two", "Three"]);
}

#[test]
fn test_multi_select_from_preserves_order() {
    let select = multi_select_from(vec!["Z", "Y", "X", "A", "B"]);
    assert_eq!(labels(&select), vec!["Z", "Y", "X", "A", "B"]);
}

#[test]
fn test_multi_select_from_duplicate_items() {
    let select = multi_select_from(vec!["A", "B", "A", "C", "B"]);
    assert_eq!(select.len(), 5);
    // Duplicates are preserved as separate options
    assert_eq!(labels(&select), vec!["A", "B", "A", "C", "B"]);
}

#[test]
fn test_multi_select_from_keeps_labels_verbatim() {
    let long = "A".repeat(1000);
    let items = vec![
        "",
        "Item/With/Slashes",
        "アイテム",
        "항목",
        "العناصر",
        "🍎 Apple",
        "  both  ",
        "Line\n1",
        "Line\r\n2",
        "Item\tWith\tTabs",
        "Item\x00With\x00Nulls",
        long.as_str(),
    ];
    let select = multi_select_from(items.clone());
    assert_eq!(select.len(), items.len());
    assert_eq!(labels(&select), items);
    // Label and value are the same for options created from strings
    let mut all = select.clone();
    all.select_all();
    assert_eq!(all.get_selected_values(), items);
}

#[test]
fn test_multi_select_from_no_initial_selections() {
    let select = multi_select_from(vec!["A", "B", "C"]);
    assert!(!select.is_empty());
    assert!(!select.is_open());
    assert_eq!(select.selection_count(), 0);
    assert!(!select.is_selected(0));
    assert!(!select.is_selected(1));
    assert!(!select.is_selected(2));
}
