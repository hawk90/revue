//! Navigation for the multi-select widget tests

use revue::widget::multi_select::MultiSelect;

fn create_test_select() -> MultiSelect {
    MultiSelect::new().options(vec!["Apple", "Banana", "Cherry", "Date", "Elderberry"])
}

// Dropdown cursor tests

#[test]
fn test_cursor_down_increments() {
    // Arrange
    let mut select = create_test_select();

    // Act
    select.cursor_down();

    // Assert
    assert_eq!(select.dropdown_cursor, 1);
}

#[test]
fn test_cursor_down_wraps_to_start() {
    // Arrange
    let mut select = create_test_select();
    select.dropdown_cursor = 4;

    // Act
    select.cursor_down();

    // Assert
    assert_eq!(select.dropdown_cursor, 0);
}

#[test]
fn test_cursor_down_with_empty_filtered() {
    // Arrange
    let mut select = create_test_select();
    select.filtered = vec![];
    select.dropdown_cursor = 2;

    // Act
    select.cursor_down();

    // Assert
    assert_eq!(select.dropdown_cursor, 2);
}

#[test]
fn test_cursor_up_decrements() {
    // Arrange
    let mut select = create_test_select();
    select.dropdown_cursor = 2;

    // Act
    select.cursor_up();

    // Assert
    assert_eq!(select.dropdown_cursor, 1);
}

#[test]
fn test_cursor_up_wraps_to_end() {
    // Arrange
    let mut select = create_test_select();
    select.dropdown_cursor = 0;

    // Act
    select.cursor_up();

    // Assert
    assert_eq!(select.dropdown_cursor, 4);
}

#[test]
fn test_cursor_up_with_empty_filtered() {
    // Arrange
    let mut select = create_test_select();
    select.filtered = vec![];
    select.dropdown_cursor = 2;

    // Act
    select.cursor_up();

    // Assert
    assert_eq!(select.dropdown_cursor, 2);
}

#[test]
fn test_cursor_up_from_zero_wraps() {
    // Arrange
    let mut select = create_test_select();
    select.dropdown_cursor = 0;

    // Act
    select.cursor_up();

    // Assert
    assert_eq!(select.dropdown_cursor, 4);
}

// Tag cursor tests

#[test]
fn test_tag_cursor_left_from_none_to_last() {
    // Arrange
    let mut select = MultiSelect::new()
        .options(vec!["A", "B", "C"])
        .selected_indices(vec![0, 1, 2]);

    // Act
    select.tag_cursor_left();

    // Assert
    assert_eq!(select.tag_cursor, Some(2));
}

#[test]
fn test_tag_cursor_left_decrements() {
    // Arrange
    let mut select = MultiSelect::new()
        .options(vec!["A", "B", "C", "D"])
        .selected_indices(vec![0, 1, 2, 3]);
    select.tag_cursor = Some(2);

    // Act
    select.tag_cursor_left();

    // Assert
    assert_eq!(select.tag_cursor, Some(1));
}

#[test]
fn test_tag_cursor_left_at_start_stays() {
    // Arrange
    let mut select = MultiSelect::new()
        .options(vec!["A", "B", "C"])
        .selected_indices(vec![0, 1, 2]);
    select.tag_cursor = Some(0);

    // Act
    select.tag_cursor_left();

    // Assert
    assert_eq!(select.tag_cursor, Some(0));
}

#[test]
fn test_tag_cursor_left_with_empty_selections() {
    // Arrange
    let mut select = create_test_select();

    // Act
    select.tag_cursor_left();

    // Assert
    assert_eq!(select.tag_cursor, None);
}

#[test]
fn test_tag_cursor_right_from_none_does_nothing() {
    // Arrange
    let mut select = MultiSelect::new()
        .options(vec!["A", "B", "C"])
        .selected_indices(vec![0, 1, 2]);

    // Act
    select.tag_cursor_right();

    // Assert
    assert_eq!(select.tag_cursor, None);
}

#[test]
fn test_tag_cursor_right_increments() {
    // Arrange
    let mut select = MultiSelect::new()
        .options(vec!["A", "B", "C", "D"])
        .selected_indices(vec![0, 1, 2, 3]);
    select.tag_cursor = Some(1);

    // Act
    select.tag_cursor_right();

    // Assert
    assert_eq!(select.tag_cursor, Some(2));
}

#[test]
fn test_tag_cursor_right_from_last_to_none() {
    // Arrange
    let mut select = MultiSelect::new()
        .options(vec!["A", "B", "C"])
        .selected_indices(vec![0, 1, 2]);
    select.tag_cursor = Some(2);

    // Act
    select.tag_cursor_right();

    // Assert
    assert_eq!(select.tag_cursor, None);
}

#[test]
fn test_tag_cursor_right_from_second_last_to_none() {
    // Arrange
    let mut select = MultiSelect::new()
        .options(vec!["A", "B", "C"])
        .selected_indices(vec![0, 1, 2]);
    select.tag_cursor = Some(1);

    // Act
    select.tag_cursor_right();

    // Assert
    assert_eq!(select.tag_cursor, Some(2));
}

// Current option tests

#[test]
fn test_current_option_with_filtered() {
    // Arrange
    let select = create_test_select();

    // Act
    let option = select.current_option();

    // Assert
    assert_eq!(option, Some(0));
}

#[test]
fn test_current_option_with_cursor_at_position() {
    // Arrange
    let mut select = create_test_select();
    select.dropdown_cursor = 2;

    // Act
    let option = select.current_option();

    // Assert
    assert_eq!(option, Some(2));
}

#[test]
fn test_current_option_with_empty_filtered() {
    // Arrange
    let mut select = create_test_select();
    select.filtered = vec![];

    // Act
    let option = select.current_option();

    // Assert
    assert_eq!(option, None);
}

#[test]
fn test_current_option_out_of_bounds() {
    // Arrange
    let mut select = create_test_select();
    select.dropdown_cursor = 10;

    // Act
    let option = select.current_option();

    // Assert
    assert_eq!(option, None);
}

// Combined navigation tests

#[test]
fn test_full_dropdown_navigation_cycle() {
    // Arrange
    let mut select = create_test_select();

    // Act & Assert - Start
    assert_eq!(select.dropdown_cursor, 0);

    // Move down
    select.cursor_down();
    assert_eq!(select.dropdown_cursor, 1);

    select.cursor_down();
    assert_eq!(select.dropdown_cursor, 2);

    // Move up
    select.cursor_up();
    assert_eq!(select.dropdown_cursor, 1);

    // Wrap to end
    select.cursor_up();
    assert_eq!(select.dropdown_cursor, 0);

    select.cursor_up();
    assert_eq!(select.dropdown_cursor, 4);

    // Wrap to start
    select.cursor_down();
    assert_eq!(select.dropdown_cursor, 0);
}

#[test]
fn test_full_tag_navigation_cycle() {
    // Arrange
    let mut select = MultiSelect::new()
        .options(vec!["A", "B", "C", "D"])
        .selected_indices(vec![0, 1, 2, 3]);

    // Act & Assert - Start from none
    assert_eq!(select.tag_cursor, None);

    // Move left to last
    select.tag_cursor_left();
    assert_eq!(select.tag_cursor, Some(3));

    // Move left
    select.tag_cursor_left();
    assert_eq!(select.tag_cursor, Some(2));

    select.tag_cursor_left();
    assert_eq!(select.tag_cursor, Some(1));

    // At start, stay
    select.tag_cursor_left();
    assert_eq!(select.tag_cursor, Some(0));

    select.tag_cursor_left();
    assert_eq!(select.tag_cursor, Some(0));

    // Move right
    select.tag_cursor_right();
    assert_eq!(select.tag_cursor, Some(1));

    // Move to end and exit
    select.tag_cursor_right();
    assert_eq!(select.tag_cursor, Some(2));

    select.tag_cursor_right();
    assert_eq!(select.tag_cursor, Some(3));

    select.tag_cursor_right();
    assert_eq!(select.tag_cursor, None);
}