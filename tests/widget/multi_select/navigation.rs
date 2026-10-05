//! Navigation for the multi-select widget tests

use revue::widget::MultiSelect;

fn create_test_select() -> MultiSelect {
    MultiSelect::new().options(vec!["Apple", "Banana", "Cherry", "Date", "Elderberry"])
}

/// Move the dropdown cursor down `n` rows from the top.
fn cursor_at(select: &mut MultiSelect, n: usize) {
    for _ in 0..n {
        select.cursor_down();
    }
    assert_eq!(select.get_dropdown_cursor(), n);
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

// Dropdown cursor tests

#[test]
fn test_cursor_down_increments() {
    let mut select = create_test_select();

    select.cursor_down();

    assert_eq!(select.get_dropdown_cursor(), 1);
}

#[test]
fn test_cursor_down_wraps_to_start() {
    let mut select = create_test_select();
    cursor_at(&mut select, 4);

    select.cursor_down();

    assert_eq!(select.get_dropdown_cursor(), 0);
}

#[test]
fn test_cursor_down_with_empty_filtered() {
    let mut select = create_test_select();
    select.set_query("zzz");
    assert!(select.get_filtered().is_empty());

    select.cursor_down();

    assert_eq!(select.get_dropdown_cursor(), 0);
    assert_eq!(select.current_option(), None);
}

#[test]
fn test_cursor_up_decrements() {
    let mut select = create_test_select();
    cursor_at(&mut select, 2);

    select.cursor_up();

    assert_eq!(select.get_dropdown_cursor(), 1);
}

#[test]
fn test_cursor_up_from_zero_wraps_to_end() {
    let mut select = create_test_select();
    assert_eq!(select.get_dropdown_cursor(), 0);

    select.cursor_up();

    assert_eq!(select.get_dropdown_cursor(), 4);
}

#[test]
fn test_cursor_up_with_empty_filtered() {
    let mut select = create_test_select();
    select.set_query("zzz");
    assert!(select.get_filtered().is_empty());

    select.cursor_up();

    assert_eq!(select.get_dropdown_cursor(), 0);
    assert_eq!(select.current_option(), None);
}

// Tag cursor tests

#[test]
fn test_tag_cursor_left_from_none_to_last() {
    let mut select = MultiSelect::new()
        .options(vec!["A", "B", "C"])
        .selected_indices(vec![0, 1, 2]);

    select.tag_cursor_left();

    assert_eq!(select.get_tag_cursor(), Some(2));
}

#[test]
fn test_tag_cursor_left_decrements() {
    let mut select = MultiSelect::new()
        .options(vec!["A", "B", "C", "D"])
        .selected_indices(vec![0, 1, 2, 3]);
    tag_cursor_at(&mut select, 2);

    select.tag_cursor_left();

    assert_eq!(select.get_tag_cursor(), Some(1));
}

#[test]
fn test_tag_cursor_left_at_start_stays() {
    let mut select = MultiSelect::new()
        .options(vec!["A", "B", "C"])
        .selected_indices(vec![0, 1, 2]);
    tag_cursor_at(&mut select, 0);

    select.tag_cursor_left();

    assert_eq!(select.get_tag_cursor(), Some(0));
}

#[test]
fn test_tag_cursor_left_with_empty_selections() {
    let mut select = create_test_select();

    select.tag_cursor_left();

    assert_eq!(select.get_tag_cursor(), None);
}

#[test]
fn test_tag_cursor_right_from_none_does_nothing() {
    let mut select = MultiSelect::new()
        .options(vec!["A", "B", "C"])
        .selected_indices(vec![0, 1, 2]);

    select.tag_cursor_right();

    assert_eq!(select.get_tag_cursor(), None);
}

#[test]
fn test_tag_cursor_right_increments() {
    let mut select = MultiSelect::new()
        .options(vec!["A", "B", "C", "D"])
        .selected_indices(vec![0, 1, 2, 3]);
    tag_cursor_at(&mut select, 1);

    select.tag_cursor_right();

    assert_eq!(select.get_tag_cursor(), Some(2));
}

#[test]
fn test_tag_cursor_right_from_last_to_none() {
    let mut select = MultiSelect::new()
        .options(vec!["A", "B", "C"])
        .selected_indices(vec![0, 1, 2]);
    tag_cursor_at(&mut select, 2);

    select.tag_cursor_right();

    assert_eq!(select.get_tag_cursor(), None);
}

#[test]
fn test_tag_cursor_right_from_second_last_to_last() {
    let mut select = MultiSelect::new()
        .options(vec!["A", "B", "C"])
        .selected_indices(vec![0, 1, 2]);
    tag_cursor_at(&mut select, 1);

    select.tag_cursor_right();

    assert_eq!(select.get_tag_cursor(), Some(2));
}

// Current option tests

#[test]
fn test_current_option_with_filtered() {
    let select = create_test_select();
    assert_eq!(select.current_option(), Some(0));
}

#[test]
fn test_current_option_with_cursor_at_position() {
    let mut select = create_test_select();
    cursor_at(&mut select, 2);

    assert_eq!(select.current_option(), Some(2));
}

#[test]
fn test_current_option_follows_filtered_order() {
    let mut select = create_test_select();
    // "er" matches Cherry and Elderberry; the cursor indexes the filtered
    // list, so current_option maps back to the original option index.
    select.set_query("er");
    assert_eq!(select.get_filtered(), &[2, 4]);
    select.cursor_down();

    assert_eq!(select.current_option(), Some(4));
}

#[test]
fn test_current_option_with_empty_filtered() {
    let mut select = create_test_select();
    select.set_query("zzz");

    assert_eq!(select.current_option(), None);
}

// Combined navigation tests

#[test]
fn test_full_dropdown_navigation_cycle() {
    let mut select = create_test_select();

    assert_eq!(select.get_dropdown_cursor(), 0);

    select.cursor_down();
    assert_eq!(select.get_dropdown_cursor(), 1);

    select.cursor_down();
    assert_eq!(select.get_dropdown_cursor(), 2);

    select.cursor_up();
    assert_eq!(select.get_dropdown_cursor(), 1);

    select.cursor_up();
    assert_eq!(select.get_dropdown_cursor(), 0);

    // Wrap to end
    select.cursor_up();
    assert_eq!(select.get_dropdown_cursor(), 4);

    // Wrap to start
    select.cursor_down();
    assert_eq!(select.get_dropdown_cursor(), 0);
}

#[test]
fn test_full_tag_navigation_cycle() {
    let mut select = MultiSelect::new()
        .options(vec!["A", "B", "C", "D"])
        .selected_indices(vec![0, 1, 2, 3]);

    assert_eq!(select.get_tag_cursor(), None);

    select.tag_cursor_left();
    assert_eq!(select.get_tag_cursor(), Some(3));

    select.tag_cursor_left();
    assert_eq!(select.get_tag_cursor(), Some(2));

    select.tag_cursor_left();
    assert_eq!(select.get_tag_cursor(), Some(1));

    select.tag_cursor_left();
    assert_eq!(select.get_tag_cursor(), Some(0));

    // At start, stay
    select.tag_cursor_left();
    assert_eq!(select.get_tag_cursor(), Some(0));

    select.tag_cursor_right();
    assert_eq!(select.get_tag_cursor(), Some(1));

    select.tag_cursor_right();
    assert_eq!(select.get_tag_cursor(), Some(2));

    select.tag_cursor_right();
    assert_eq!(select.get_tag_cursor(), Some(3));

    // Past the end leaves tag navigation
    select.tag_cursor_right();
    assert_eq!(select.get_tag_cursor(), None);
}
