//! Multi-widget interaction and edge case tests
//!
//! Tests for scenarios where multiple widgets are used together,
//! and edge cases that combine several features at once.

use revue::event::{Key, KeyEvent};
use revue::widget::{select, textarea, Select};

// =============================================================================
// Focus Guard Tests — only focused widget should process events
// =============================================================================

#[test]
fn test_two_inputs_only_focused_responds() {
    use revue::widget::Input;

    let mut input1 = Input::new().value("A").focused(true);
    let mut input2 = Input::new().value("B").focused(false); // explicitly not focused

    let event = KeyEvent::new(Key::Char('X'));
    let changed1 = input1.handle_key_event(&event);
    let changed2 = input2.handle_key_event(&event);

    assert!(changed1, "focused input should process key");
    assert!(!changed2, "unfocused input should ignore key");
    assert!(input1.get_value().contains('X'));
    assert_eq!(input2.get_value(), "B");
}

#[test]
fn test_textarea_unfocused_ignores_keys() {
    let mut ta = textarea(); // focused=false by default
    let handled = ta.handle_key(&Key::Char('a'));
    assert!(!handled);
    assert_eq!(ta.get_content(), ""); // nothing inserted
}

#[test]
fn test_textarea_focused_accepts_keys() {
    let mut ta = textarea().focused(true);
    let handled = ta.handle_key(&Key::Char('a'));
    assert!(handled);
    assert_eq!(ta.get_content(), "a");
}

#[test]
fn test_select_unfocused_ignores_keys() {
    let mut s = Select::new().options(vec!["A", "B"]);
    // focused=false by default
    s.handle_key(&Key::Enter);
    assert!(!s.is_open(), "unfocused select should not open on Enter");
}

#[test]
fn test_select_focused_accepts_keys() {
    let mut s = Select::new().options(vec!["A", "B"]).focused(true);
    s.handle_key(&Key::Enter);
    assert!(s.is_open(), "focused select should open on Enter");
}

// =============================================================================
// Select Focus Loss Auto-Close Tests
// =============================================================================

#[test]
fn test_select_closes_on_focus_loss() {
    let mut s = Select::new()
        .options(vec!["A", "B"])
        .focused(true)
        .searchable(true);
    s.open();
    s.set_query("test");
    assert!(s.is_open());

    // Simulate focus loss
    s = s.focused(false);
    assert!(!s.is_open(), "should close on focus loss");
    assert_eq!(s.query(), "", "should clear query on focus loss");
}

// =============================================================================
// Select Edge Cases
// =============================================================================

#[test]
fn test_select_selected_before_options() {
    // Setting selected before options — should not panic
    let s = Select::new().selected(5).options(vec!["A", "B"]);
    assert!(s.selected_index() < s.len());
}

#[test]
fn test_select_options_replace_resets() {
    let mut s = Select::new().options(vec!["A", "B", "C"]).selected(2);
    assert_eq!(s.value(), Some("C"));

    // Replace options — selection should be recalculated
    s = s.options(vec!["X", "Y"]);
    assert!(s.selected_index() < s.len());
}

#[test]
fn test_select_get_value_alias() {
    let s = Select::new().options(vec!["Hello"]);
    assert_eq!(s.value(), s.get_value());
}

#[test]
fn test_select_empty_options_no_panic() {
    let s = Select::new();
    assert_eq!(s.value(), None);
    assert_eq!(s.get_value(), None);
    assert_eq!(s.selected_index(), 0);
    assert!(s.is_empty());
}

// =============================================================================
// TextArea Edge Cases
// =============================================================================

#[test]
fn test_textarea_editor_preset() {
    use revue::widget::TextArea;
    let editor = TextArea::editor();
    // editor() should enable line numbers and wrap
    let content = editor.content("Hello\nWorld");
    assert_eq!(content.line_count(), 2);
}

#[test]
fn test_textarea_empty_content_operations() {
    let mut ta = textarea().focused(true);
    // Delete on empty — should not panic
    ta.handle_key(&Key::Backspace);
    ta.handle_key(&Key::Delete);
    assert_eq!(ta.get_content(), "");
}

#[test]
fn test_textarea_rapid_undo_redo() {
    let mut ta = textarea().focused(true);
    ta.insert_char('a');
    ta.insert_char('b');
    ta.insert_char('c');
    assert_eq!(ta.get_content(), "abc");

    ta.undo();
    ta.undo();
    ta.undo();
    ta.undo(); // extra undo — should not panic
    assert_eq!(ta.get_content(), "");

    ta.redo();
    assert_eq!(ta.get_content(), "a");
    ta.redo();
    ta.redo();
    ta.redo(); // extra redo — should not panic
    assert_eq!(ta.get_content(), "abc");
}

#[test]
fn test_textarea_insert_then_backspace_all() {
    let mut ta = textarea().focused(true);
    for ch in "Hello, 세계!".chars() {
        ta.insert_char(ch);
    }
    assert_eq!(ta.get_content(), "Hello, 세계!");

    // Backspace everything
    for _ in 0..20 {
        ta.delete_char_before();
    }
    assert_eq!(ta.get_content(), "");
}

// =============================================================================
// Combobox Edge Cases
// =============================================================================

#[test]
fn test_combobox_get_value() {
    use revue::widget::Combobox;
    let c = Combobox::new().options(["Apple", "Banana"]);
    assert_eq!(c.get_value(), "");
}

// =============================================================================
// Input Control Character Filtering
// =============================================================================

#[test]
fn test_input_rejects_control_chars() {
    use revue::widget::Input;
    let mut input = Input::new().focused(true);

    // Try to type a newline — should be rejected
    let handled = input.handle_key_event(&KeyEvent::new(Key::Char('\n')));
    assert!(!handled, "newline should be rejected in single-line input");

    // Try to type a null byte
    let handled = input.handle_key_event(&KeyEvent::new(Key::Char('\0')));
    assert!(!handled, "null should be rejected");

    // Normal char should work
    let handled = input.handle_key_event(&KeyEvent::new(Key::Char('a')));
    assert!(handled);
}

#[test]
fn test_input_set_value_strips_control_chars() {
    use revue::widget::Input;
    let mut input = Input::new();
    input.set_value("Hello\nWorld\t!");
    // Newline and tab should be stripped
    assert!(!input.get_value().contains('\n'));
    assert!(!input.get_value().contains('\t'));
    assert!(input.get_value().contains("Hello"));
    assert!(input.get_value().contains("World"));
}

// =============================================================================
// DataGrid Filter + Selection Interaction
// =============================================================================

#[test]
fn test_datagrid_navigate_after_filter() {
    use revue::widget::datagrid::{DataGrid, GridColumn, GridRow};

    let mut grid = DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .row(GridRow::new().cell("name", "Alpha"))
        .row(GridRow::new().cell("name", "Beta"))
        .row(GridRow::new().cell("name", "Gamma"));

    grid.select_next();
    grid.select_next();
    assert_eq!(grid.selected_row, 2);

    grid.set_filter("a"); // matches Alpha, Beta, Gamma (all have 'a')
    assert_eq!(grid.selected_row, 0); // reset
    assert!(grid.filtered_count() > 0);
}

// =============================================================================
// Rendering with Zero/Tiny Areas
// =============================================================================

#[test]
fn test_select_render_1x1() {
    use revue::layout::Rect;
    use revue::render::Buffer;
    use revue::widget::traits::{RenderContext, View};

    let mut buffer = Buffer::new(1, 1);
    let area = Rect::new(0, 0, 1, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = select().options(vec!["Test"]);
    s.render(&mut ctx); // should not panic
}

#[test]
fn test_textarea_render_0x0() {
    use revue::layout::Rect;
    use revue::render::Buffer;
    use revue::widget::traits::{RenderContext, View};

    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 0, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let ta = textarea().focused(true).content("Hello");
    ta.render(&mut ctx); // should not panic
}
