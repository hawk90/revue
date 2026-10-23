//! DataGrid cell editing tests

use proptest::prelude::*;
use revue::event::Key;
use revue::widget::data::datagrid::{DataGrid, GridColumn, GridRow};

#[test]
fn test_cell_edit_start() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("name", "Name").editable(true))
        .row(GridRow::new().cell("name", "Alice"));

    assert!(!grid.is_editing());

    // Start editing
    assert!(grid.start_edit());
    assert!(grid.is_editing());
    assert_eq!(grid.edit_buffer(), Some("Alice"));
}

#[test]
fn test_cell_edit_non_editable() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("name", "Name").editable(false))
        .row(GridRow::new().cell("name", "Alice"));

    // Should not be able to edit non-editable column
    assert!(!grid.start_edit());
    assert!(!grid.is_editing());
}

#[test]
fn test_cell_edit_commit() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("name", "Name").editable(true))
        .row(GridRow::new().cell("name", "Alice"));

    grid.start_edit();

    // Type some text
    grid.handle_key(&Key::Backspace); // Delete 'e'
    grid.handle_key(&Key::Backspace); // Delete 'c'
    grid.handle_key(&Key::Backspace); // Delete 'i'
    grid.handle_key(&Key::Backspace); // Delete 'l'
    grid.handle_key(&Key::Backspace); // Delete 'A'
    grid.handle_key(&Key::Char('B'));
    grid.handle_key(&Key::Char('o'));
    grid.handle_key(&Key::Char('b'));

    // Commit with Enter
    grid.handle_key(&Key::Enter);

    assert!(!grid.is_editing());
    assert_eq!(grid.rows[0].get("name"), Some("Bob"));
}

#[test]
fn test_cell_edit_cancel() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("name", "Name").editable(true))
        .row(GridRow::new().cell("name", "Alice"));

    grid.start_edit();

    // Type some text
    grid.handle_key(&Key::Char('X'));
    grid.handle_key(&Key::Char('Y'));
    grid.handle_key(&Key::Char('Z'));

    // Cancel with Escape
    grid.handle_key(&Key::Escape);

    assert!(!grid.is_editing());
    // Value should be unchanged
    assert_eq!(grid.rows[0].get("name"), Some("Alice"));
}

#[test]
fn test_cell_edit_cursor_movement() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("name", "Name").editable(true))
        .row(GridRow::new().cell("name", "Test"));

    grid.start_edit();
    assert_eq!(grid.edit_state.cursor, 4); // At end

    // Move cursor left
    grid.handle_key(&Key::Left);
    assert_eq!(grid.edit_state.cursor, 3);

    // Move to start
    grid.handle_key(&Key::Home);
    assert_eq!(grid.edit_state.cursor, 0);

    // Move to end
    grid.handle_key(&Key::End);
    assert_eq!(grid.edit_state.cursor, 4);
}

#[test]
fn test_edit_delete_key() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("a", "A").editable(true))
        .row(GridRow::new().cell("a", "ABC"));

    grid.start_edit();
    grid.handle_key(&Key::Home); // Move to start
    grid.handle_key(&Key::Delete); // Delete 'A'

    assert_eq!(grid.edit_buffer(), Some("BC"));
}

#[test]
fn test_edit_right_key() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("a", "A").editable(true))
        .row(GridRow::new().cell("a", "AB"));

    grid.start_edit();
    grid.handle_key(&Key::Home);
    assert_eq!(grid.edit_state.cursor, 0);

    grid.handle_key(&Key::Right);
    assert_eq!(grid.edit_state.cursor, 1);
}

#[test]
fn test_edit_start_out_of_bounds() {
    let mut grid = DataGrid::new().column(GridColumn::new("a", "A").editable(true));

    // No rows, can't edit
    assert!(!grid.start_edit());
}

#[test]
fn test_commit_edit_invalid_state() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("a", "A").editable(true))
        .row(GridRow::new().cell("a", "test"));

    // Not editing, commit should fail
    assert!(!grid.commit_edit());
}

#[test]
fn test_edit_add_new_cell() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("a", "A").editable(true))
        .row(GridRow::new()); // Row without the cell

    grid.start_edit();
    grid.handle_key(&Key::Char('X'));
    grid.commit_edit();

    assert_eq!(grid.rows[0].get("a"), Some("X"));
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    /// Typing and cursor keys edit the cell like a plain char-indexed model,
    /// and Enter writes the result into the cell
    #[test]
    fn edit_keys_follow_a_char_model(
        initial in "[a-zé한]{0,6}",
        keys in prop::collection::vec(prop_oneof![
            "[a-zé한]".prop_map(|s| Key::Char(s.chars().next().unwrap())),
            Just(Key::Left),
            Just(Key::Right),
            Just(Key::Home),
            Just(Key::End),
            Just(Key::Backspace),
            Just(Key::Delete),
        ], 0..30),
    ) {
        let mut grid = DataGrid::new()
            .column(GridColumn::new("name", "Name").editable(true))
            .row(GridRow::new().cell("name", initial.clone()));
        prop_assert!(grid.start_edit());

        let mut text: Vec<char> = initial.chars().collect();
        let mut cursor = text.len();
        for key in &keys {
            grid.handle_key(key);
            match key {
                Key::Char(c) => {
                    text.insert(cursor, *c);
                    cursor += 1;
                }
                Key::Left => cursor = cursor.saturating_sub(1),
                Key::Right => cursor = (cursor + 1).min(text.len()),
                Key::Home => cursor = 0,
                Key::End => cursor = text.len(),
                Key::Backspace if cursor > 0 => {
                    cursor -= 1;
                    text.remove(cursor);
                }
                Key::Delete if cursor < text.len() => {
                    text.remove(cursor);
                }
                _ => {}
            }
            let model: String = text.iter().collect();
            prop_assert_eq!(grid.edit_buffer(), Some(model.as_str()));
            prop_assert_eq!(grid.edit_state.cursor, cursor);
        }

        grid.handle_key(&Key::Enter);
        let model: String = text.into_iter().collect();
        prop_assert!(!grid.is_editing());
        prop_assert_eq!(grid.rows[0].get("name"), Some(model.as_str()));
    }
}

// The standard editing keys (#985)

#[test]
fn test_cell_edit_takes_the_modified_editing_keys() {
    use revue::event::KeyEvent;
    let mut grid = DataGrid::new()
        .column(GridColumn::new("name", "Name").editable(true))
        .row(GridRow::new().cell("name", "Ada Lovelace"));
    grid.start_edit();
    assert!(grid.handle_key_event(&KeyEvent::ctrl(Key::Backspace)));
    assert_eq!(grid.edit_buffer(), Some("Ada "));
    assert_eq!(grid.edit_state.cursor, 4);
    assert!(grid.handle_key_event(&KeyEvent::ctrl(Key::Char('z'))));
    assert_eq!(grid.edit_buffer(), Some("Ada Lovelace"));
    // A Ctrl letter the editor does not use is the app's, not typed
    assert!(!grid.handle_key_event(&KeyEvent::ctrl(Key::Char('k'))));
    assert_eq!(grid.edit_buffer(), Some("Ada Lovelace"));
    // A control character is not typed
    grid.handle_key(&Key::Char('\u{7}'));
    assert_eq!(grid.edit_buffer(), Some("Ada Lovelace"));
}

#[test]
fn test_cell_edit_reads_back_a_written_buffer_and_cursor() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("name", "Name").editable(true))
        .row(GridRow::new().cell("name", "Alice"));
    grid.start_edit();
    // The fields are public: what is written there is edited next
    grid.edit_state.buffer = "Bob".to_string();
    grid.edit_state.cursor = 1;
    grid.handle_key(&Key::Char('x'));
    assert_eq!(grid.edit_buffer(), Some("Bxob"));
    assert_eq!(grid.edit_state.cursor, 2);
    grid.handle_key(&Key::Enter);
    assert_eq!(grid.rows[0].get("name"), Some("Bxob"));
}
