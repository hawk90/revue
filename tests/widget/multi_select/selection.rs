//! Selection manipulation for the multi-select widget tests

use revue::widget::{MultiSelect, MultiSelectOption};

fn create_test_select() -> MultiSelect {
    MultiSelect::new().options(vec!["Apple", "Banana", "Cherry", "Date", "Elderberry"])
}

/// Put the tag cursor on tag `pos` (entering from the right, as the Left key does).
fn tag_cursor_at(select: &mut MultiSelect, pos: usize) {
    let last = select.selection_count() - 1;
    select.tag_cursor_left();
    for _ in pos..last {
        select.tag_cursor_left();
    }
    assert_eq!(select.get_tag_cursor(), Some(pos));
}

// State query tests

#[test]
fn test_is_open_after_open() {
    // Arrange
    let mut select = create_test_select();

    // Act
    select.open();

    // Assert
    assert!(select.is_open());
}

#[test]
fn test_is_open_after_close() {
    // Arrange
    let mut select = create_test_select();
    select.open();

    // Act
    select.close();

    // Assert
    assert!(!select.is_open());
}

#[test]
fn test_get_selected_values_empty() {
    // Arrange
    let select = create_test_select();

    // Act
    let values = select.get_selected_values();

    // Assert
    assert!(values.is_empty());
}

#[test]
fn test_get_selected_values() {
    // Arrange
    let select = MultiSelect::new()
        .options(vec!["Apple", "Banana", "Cherry"])
        .selected_indices(vec![0, 2]);

    // Act
    let values = select.get_selected_values();

    // Assert
    assert_eq!(values.len(), 2);
    assert_eq!(values[0], "Apple");
    assert_eq!(values[1], "Cherry");
}

#[test]
fn test_get_selected_labels() {
    // Arrange
    let select = MultiSelect::new()
        .options_detailed(vec![
            MultiSelectOption::new("Display Apple", "apple"),
            MultiSelectOption::new("Display Banana", "banana"),
        ])
        .selected_indices(vec![0, 1]);

    // Act
    let labels = select.get_selected_labels();

    // Assert
    assert_eq!(labels.len(), 2);
    assert_eq!(labels[0], "Display Apple");
    assert_eq!(labels[1], "Display Banana");
}

#[test]
fn test_selection_count_empty() {
    // Arrange
    let select = create_test_select();

    // Act
    let count = select.selection_count();

    // Assert
    assert_eq!(count, 0);
}

#[test]
fn test_selection_count_with_selections() {
    // Arrange
    let select = MultiSelect::new()
        .options(vec!["A", "B", "C", "D"])
        .selected_indices(vec![0, 1, 3]);

    // Act
    let count = select.selection_count();

    // Assert
    assert_eq!(count, 3);
}

#[test]
fn test_is_selected_true() {
    // Arrange
    let select = MultiSelect::new()
        .options(vec!["Apple", "Banana"])
        .selected_indices(vec![0]);

    // Act
    let result = select.is_selected(0);

    // Assert
    assert!(result);
}

#[test]
fn test_is_selected_false() {
    // Arrange
    let select = MultiSelect::new()
        .options(vec!["Apple", "Banana"])
        .selected_indices(vec![0]);

    // Act
    let result = select.is_selected(1);

    // Assert
    assert!(!result);
}

#[test]
fn test_can_select_more_unlimited() {
    // Arrange
    let select = MultiSelect::new()
        .options(vec!["A", "B", "C"])
        .selected_indices(vec![0, 1, 2]);

    // Act
    let result = select.can_select_more();

    // Assert
    assert!(result);
}

#[test]
fn test_can_select_more_with_limit_available() {
    // Arrange
    let select = MultiSelect::new()
        .options(vec!["A", "B", "C", "D"])
        .selected_indices(vec![0, 1])
        .max_selections(3);

    // Act
    let result = select.can_select_more();

    // Assert
    assert!(result);
}

#[test]
fn test_can_select_more_at_limit() {
    // Arrange
    let select = MultiSelect::new()
        .options(vec!["A", "B", "C", "D"])
        .selected_indices(vec![0, 1, 2])
        .max_selections(3);

    // Act
    let result = select.can_select_more();

    // Assert
    assert!(!result);
}

#[test]
fn test_is_empty_false() {
    // Arrange
    let select = MultiSelect::new().options(vec!["A", "B"]);

    // Act
    let result = select.is_empty();

    // Assert
    assert!(!result);
}

// Dropdown state tests

#[test]
fn test_open_resets_filter() {
    // Arrange
    let mut select = MultiSelect::new()
        .options(vec!["A", "B"])
        .selected_indices(vec![0]);
    select.set_query("zzz");
    select.tag_cursor_left();
    assert!(select.get_filtered().is_empty());
    assert_eq!(select.get_tag_cursor(), Some(0));

    // Act
    select.open();

    // Assert
    assert!(select.is_open());
    assert_eq!(select.get_filtered(), &[0, 1]);
    assert_eq!(select.get_tag_cursor(), None);
}

#[test]
fn test_close_clears_query() {
    // Arrange
    let mut select = MultiSelect::new().options(vec!["A", "B"]);
    select.open();
    select.set_query("test");
    assert!(select.get_filtered().is_empty());

    // Act
    select.close();

    // Assert
    assert!(!select.is_open());
    assert!(select.get_query().is_empty());
    assert_eq!(select.get_filtered(), &[0, 1]);
}

#[test]
fn test_toggle_from_closed() {
    // Arrange
    let mut select = create_test_select();

    // Act
    select.toggle();

    // Assert
    assert!(select.is_open());
}

#[test]
fn test_toggle_from_open() {
    // Arrange
    let mut select = create_test_select();
    select.open();

    // Act
    select.toggle();

    // Assert
    assert!(!select.is_open());
}

// Selection manipulation tests

#[test]
fn test_select_option_out_of_bounds() {
    // Arrange
    let mut select = MultiSelect::new().options(vec!["A", "B", "C"]);

    // Act
    select.select_option(10);

    // Assert
    assert_eq!(select.selection_count(), 0);
}

#[test]
fn test_select_option_already_selected() {
    // Arrange
    let mut select = MultiSelect::new()
        .options(vec!["A", "B", "C"])
        .selected_indices(vec![0]);

    // Act
    select.select_option(0);

    // Assert
    assert_eq!(select.selection_count(), 1);
}

#[test]
fn test_deselect_option() {
    // Arrange
    let mut select = MultiSelect::new()
        .options(vec!["A", "B", "C"])
        .selected_indices(vec![0, 1, 2]);

    // Act
    select.deselect_option(1);

    // Assert
    assert_eq!(select.selection_count(), 2);
    assert!(!select.is_selected(1));
    assert!(select.is_selected(0));
    assert!(select.is_selected(2));
}

#[test]
fn test_deselect_option_not_selected() {
    // Arrange
    let mut select = MultiSelect::new()
        .options(vec!["A", "B", "C"])
        .selected_indices(vec![0, 2]);

    // Act
    select.deselect_option(1);

    // Assert
    assert_eq!(select.selection_count(), 2);
}

#[test]
fn test_toggle_option_from_selected() {
    // Arrange
    let mut select = MultiSelect::new()
        .options(vec!["A", "B", "C"])
        .selected_indices(vec![0, 1]);

    // Act
    select.toggle_option(1);

    // Assert
    assert!(!select.is_selected(1));
    assert!(select.is_selected(0));
}

#[test]
fn test_clear_selection() {
    // Arrange
    let mut select = MultiSelect::new()
        .options(vec!["A", "B", "C"])
        .selected_indices(vec![0, 1, 2]);

    // Act
    select.clear_selection();

    // Assert
    assert_eq!(select.selection_count(), 0);
}

#[test]
fn test_clear_selection_already_empty() {
    // Arrange
    let mut select = create_test_select();

    // Act
    select.clear_selection();

    // Assert
    assert_eq!(select.selection_count(), 0);
}

#[test]
fn test_select_all() {
    // Arrange
    let mut select = MultiSelect::new().options(vec!["A", "B", "C"]);

    // Act
    select.select_all();

    // Assert
    assert_eq!(select.selection_count(), 3);
    assert!(select.is_selected(0));
    assert!(select.is_selected(1));
    assert!(select.is_selected(2));
}

#[test]
fn test_select_all_with_disabled() {
    // Arrange
    let mut select = MultiSelect::new().options_detailed(vec![
        MultiSelectOption::simple("A"),
        MultiSelectOption::simple("B").disabled(true),
        MultiSelectOption::simple("C"),
    ]);

    // Act
    select.select_all();

    // Assert
    assert_eq!(select.selection_count(), 2);
    assert!(select.is_selected(0));
    assert!(!select.is_selected(1));
    assert!(select.is_selected(2));
}

#[test]
fn test_select_all_with_max_limit() {
    // Arrange
    let mut select = MultiSelect::new()
        .options(vec!["A", "B", "C", "D", "E"])
        .max_selections(3);

    // Act
    select.select_all();

    // Assert
    assert_eq!(select.selection_count(), 3);
}

#[test]
fn test_select_all_empty() {
    // Arrange
    let mut select = MultiSelect::new();

    // Act
    select.select_all();

    // Assert
    assert_eq!(select.selection_count(), 0);
}

#[test]
fn test_remove_last_tag() {
    // Arrange
    let mut select = MultiSelect::new()
        .options(vec!["A", "B", "C"])
        .selected_indices(vec![0, 1, 2]);

    // Act
    select.remove_last_tag();

    // Assert
    assert_eq!(select.selection_count(), 2);
    assert!(!select.is_selected(2));
    assert!(select.is_selected(0));
    assert!(select.is_selected(1));
}

#[test]
fn test_remove_last_tag_from_single() {
    // Arrange
    let mut select = MultiSelect::new()
        .options(vec!["A", "B", "C"])
        .selected_indices(vec![0]);

    // Act
    select.remove_last_tag();

    // Assert
    assert_eq!(select.selection_count(), 0);
}

#[test]
fn test_remove_last_tag_from_empty() {
    // Arrange
    let mut select = create_test_select();

    // Act
    select.remove_last_tag();

    // Assert
    assert_eq!(select.selection_count(), 0);
}

#[test]
fn test_remove_tag_at_cursor_middle() {
    // Arrange
    let mut select = MultiSelect::new()
        .options(vec!["A", "B", "C", "D"])
        .selected_indices(vec![0, 1, 2]);
    tag_cursor_at(&mut select, 1);

    // Act
    select.remove_tag_at_cursor();

    // Assert
    assert_eq!(select.selection_count(), 2);
    assert_eq!(select.get_selected_indices(), &[0, 2]);
    assert_eq!(select.get_tag_cursor(), Some(1));
}

#[test]
fn test_remove_tag_at_cursor_first() {
    // Arrange
    let mut select = MultiSelect::new()
        .options(vec!["A", "B", "C"])
        .selected_indices(vec![0, 1, 2]);
    tag_cursor_at(&mut select, 0);

    // Act
    select.remove_tag_at_cursor();

    // Assert
    assert_eq!(select.selection_count(), 2);
    assert_eq!(select.get_selected_indices(), &[1, 2]);
    assert_eq!(select.get_tag_cursor(), Some(0));
}

#[test]
fn test_remove_tag_at_cursor_last() {
    // Arrange
    let mut select = MultiSelect::new()
        .options(vec!["A", "B", "C"])
        .selected_indices(vec![0, 1, 2]);
    tag_cursor_at(&mut select, 2);

    // Act
    select.remove_tag_at_cursor();

    // Assert
    assert_eq!(select.selection_count(), 2);
    assert_eq!(select.get_selected_indices(), &[0, 1]);
    assert_eq!(select.get_tag_cursor(), Some(1));
}

#[test]
fn test_remove_tag_at_cursor_single_item() {
    // Arrange
    let mut select = MultiSelect::new()
        .options(vec!["A", "B", "C"])
        .selected_indices(vec![1]);
    tag_cursor_at(&mut select, 0);

    // Act
    select.remove_tag_at_cursor();

    // Assert
    assert_eq!(select.selection_count(), 0);
    assert_eq!(select.get_tag_cursor(), None);
}

#[test]
fn test_remove_tag_at_cursor_no_cursor() {
    // Arrange
    let mut select = MultiSelect::new()
        .options(vec!["A", "B"])
        .selected_indices(vec![0, 1]);

    // Act
    select.remove_tag_at_cursor();

    // Assert
    assert_eq!(select.selection_count(), 2);
}

// Found by tests/event_sequences.rs: the shrunk sequence was
// `<select all> Escape Left <clear selection>` - the tag cursor kept
// pointing at a tag that clear_selection() had removed.
#[test]
fn test_tag_cursor_follows_a_shrinking_selection() {
    use revue::event::Key;
    let mut s = MultiSelect::new().options(vec!["a", "b", "c", "d"]);
    s.select_all();
    s.handle_key(&Key::Left);
    assert_eq!(s.get_tag_cursor(), Some(3));

    s.remove_last_tag();
    assert_eq!(s.get_tag_cursor(), Some(2));
    s.deselect_option(2);
    assert_eq!(s.get_tag_cursor(), Some(1));
    s.clear_selection();
    assert_eq!(s.get_tag_cursor(), None);
}
