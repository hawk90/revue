//! SortableList widget tests
//!
//! Construction, selection, moving, push/remove and the basic drag state
//! machine are also covered by the in-source tests in
//! src/widget/sortable/mod.rs; the tests here cover what those do not:
//! rendering, key/mouse handling, the Draggable implementation, reorder
//! callbacks and drop targets.

mod core;
mod helper;
mod types;

use std::cell::RefCell;
use std::rc::Rc;

use revue::event::{DragData, DropResult, Key, KeyEvent, MouseButton, MouseEvent, MouseEventKind};
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::{Draggable, EventResult, Interactive, RenderContext};
use revue::widget::{SortableList, View};

fn abc() -> SortableList {
    SortableList::new(["A", "B", "C"])
}

fn labels(list: &SortableList) -> Vec<&str> {
    list.items().iter().map(|i| i.label.as_str()).collect()
}

fn render(list: &SortableList, w: u16, h: u16) -> Buffer {
    let mut buffer = Buffer::new(w, h);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, w, h));
    list.render(&mut ctx);
    buffer
}

fn row(buffer: &Buffer, y: u16) -> String {
    (0..buffer.width())
        .filter_map(|x| buffer.get(x, y).map(|c| c.symbol))
        .collect()
}

fn recorder() -> (Rc<RefCell<Vec<(usize, usize)>>>, impl FnMut(usize, usize)) {
    let calls = Rc::new(RefCell::new(Vec::new()));
    let sink = calls.clone();
    (calls, move |from, to| sink.borrow_mut().push((from, to)))
}

fn mouse(x: u16, y: u16, kind: MouseEventKind) -> MouseEvent {
    MouseEvent::new(x, y, kind)
}

const AREA: Rect = Rect {
    x: 0,
    y: 0,
    width: 10,
    height: 5,
};

// ---------------------------------------------------------------------------
// Remove / drag edge cases
// ---------------------------------------------------------------------------

#[test]
fn test_sortable_list_remove_updates_selection() {
    let mut list = abc();
    list.set_selected(Some(2)); // Select last item

    list.remove(2); // Remove selected item

    // Selection should move to new last item
    assert_eq!(list.selected(), Some(1));
}

#[test]
fn test_sortable_list_remove_all() {
    let mut list = SortableList::new(["A"]);
    list.set_selected(Some(0));

    list.remove(0);

    assert!(list.items().is_empty());
    assert_eq!(list.selected(), None);
}

#[test]
fn test_sortable_list_cancel_drag_out_of_bounds() {
    let mut list = SortableList::new(["A"]);
    list.set_selected(Some(0));
    list.start_drag();

    // Remove the item while dragging
    list.items_mut().clear();

    list.cancel_drag();
    assert!(!list.is_dragging());
}

#[test]
fn test_sortable_list_end_drag_same_position() {
    let mut list = abc();
    list.set_selected(Some(1));
    list.start_drag();
    list.drop_target = Some(1); // Same position

    list.end_drag();

    assert_eq!(labels(&list), vec!["A", "B", "C"]);
    assert!(!list.is_dragging());
}

#[test]
fn test_sortable_list_start_drag_without_selection() {
    let mut list = abc();
    list.start_drag();
    assert!(!list.is_dragging());
}

#[test]
fn test_sortable_list_start_drag_with_stale_selection() {
    let mut list = abc();
    list.set_selected(Some(7));
    list.start_drag();
    assert!(!list.is_dragging());
}

#[test]
fn test_sortable_list_move_down_on_empty_list() {
    let mut list = SortableList::new(Vec::<String>::new());
    list.set_selected(Some(0));
    list.move_down();
    assert!(list.items().is_empty());
}

// ---------------------------------------------------------------------------
// Drop targets and end_drag reordering
// ---------------------------------------------------------------------------

#[test]
fn test_sortable_list_update_drop_target_needs_drag() {
    let mut list = abc();
    list.update_drop_target(2, 0);
    assert_eq!(list.drop_target, None);
}

#[test]
fn test_sortable_list_update_drop_target() {
    let mut list = abc();
    list.set_selected(Some(0));
    list.start_drag();

    list.update_drop_target(12, 10);
    assert_eq!(list.drop_target, Some(2));

    // Past the last item it targets the end of the list
    list.update_drop_target(40, 10);
    assert_eq!(list.drop_target, Some(3));
}

#[test]
fn test_sortable_list_end_drag_moves_item_down() {
    let mut list = abc();
    list.set_selected(Some(0));
    list.start_drag();
    list.update_drop_target(3, 0); // after "C"
    list.end_drag();

    assert_eq!(labels(&list), vec!["B", "C", "A"]);
    assert_eq!(list.selected(), Some(2));
    assert_eq!(list.order(), vec![1, 2, 0]);
}

#[test]
fn test_sortable_list_end_drag_moves_item_up() {
    let mut list = abc();
    list.set_selected(Some(2));
    list.start_drag();
    list.update_drop_target(0, 0); // before "A"
    list.end_drag();

    assert_eq!(labels(&list), vec!["C", "A", "B"]);
    assert_eq!(list.selected(), Some(0));
}

#[test]
fn test_sortable_list_end_drag_clears_drag_state_after_reorder() {
    let mut list = abc();
    list.set_selected(Some(0));
    list.start_drag();
    list.update_drop_target(2, 0);
    list.end_drag();

    assert_eq!(labels(&list), vec!["B", "A", "C"]);
    assert!(!list.is_dragging());
    assert!(list.items().iter().all(|i| !i.dragging));
    assert_eq!(list.drop_target, None);
}

#[test]
fn test_sortable_list_end_drag_without_target_keeps_order() {
    let mut list = abc();
    list.set_selected(Some(0));
    list.start_drag();
    list.end_drag();

    assert_eq!(labels(&list), vec!["A", "B", "C"]);
    assert!(!list.is_dragging());
}

// ---------------------------------------------------------------------------
// on_reorder callback
// ---------------------------------------------------------------------------

#[test]
fn test_sortable_list_on_reorder_called_by_move() {
    let (calls, cb) = recorder();
    let mut list = abc().on_reorder(cb);
    list.set_selected(Some(0));

    list.move_down();
    list.move_down();
    list.move_up();
    list.move_up();
    list.move_up(); // at the top: no move, no call

    assert_eq!(*calls.borrow(), vec![(0, 1), (1, 2), (2, 1), (1, 0)]);
}

#[test]
fn test_sortable_list_on_reorder_called_by_end_drag() {
    let (calls, cb) = recorder();
    let mut list = abc().on_reorder(cb);
    list.set_selected(Some(0));
    list.start_drag();
    list.update_drop_target(3, 0);
    list.end_drag();

    assert_eq!(*calls.borrow(), vec![(0, 2)]);
}

#[test]
fn test_sortable_list_on_reorder_not_called_without_move() {
    let (calls, cb) = recorder();
    let mut list = abc().on_reorder(cb);
    list.set_selected(Some(1));
    list.start_drag();
    list.update_drop_target(1, 0);
    list.end_drag();
    list.cancel_drag();

    assert!(calls.borrow().is_empty());
}

// ---------------------------------------------------------------------------
// Rendering
// ---------------------------------------------------------------------------

#[test]
fn test_sortable_list_render() {
    let buffer = render(&abc(), 8, 4);
    assert_eq!(row(&buffer, 0), "≡   A   ");
    assert_eq!(row(&buffer, 1), "≡   B   ");
    assert_eq!(row(&buffer, 2), "≡   C   ");
    assert_eq!(row(&buffer, 3), "        ");
}

#[test]
fn test_sortable_list_render_with_selection() {
    let mut list = abc();
    list.set_selected(Some(1));
    let buffer = render(&list, 8, 3);
    assert_eq!(row(&buffer, 1), "≡ ▶ B   ");
    assert_eq!(
        buffer.get(4, 1).unwrap().fg,
        Some(Color::rgb(100, 150, 255))
    );
}

#[test]
fn test_sortable_list_render_without_handles() {
    let mut list = abc().handles(false);
    list.set_selected(Some(0));
    let buffer = render(&list, 6, 2);
    assert_eq!(row(&buffer, 0), "▶ A   ");
    assert_eq!(row(&buffer, 1), "  B   ");
}

#[test]
fn test_sortable_list_render_colors() {
    let mut list = abc().item_color(Color::RED).selected_color(Color::GREEN);
    list.set_selected(Some(1));
    let buffer = render(&list, 8, 3);
    assert_eq!(buffer.get(4, 0).unwrap().fg, Some(Color::RED));
    assert_eq!(buffer.get(4, 1).unwrap().fg, Some(Color::GREEN));
}

#[test]
fn test_sortable_list_render_dragging() {
    let mut list = abc();
    list.set_selected(Some(0));
    list.start_drag();
    list.update_drop_target(2, 0);
    let buffer = render(&list, 6, 4);

    // The dragged item shows the move handle in the drag color
    assert_eq!(row(&buffer, 0), "↕ ▶ A ");
    assert_eq!(
        buffer.get(4, 0).unwrap().fg,
        Some(Color::rgb(255, 200, 100))
    );
    assert_eq!(row(&buffer, 1), "≡   B ");
    // The drop target row becomes a drop indicator line
    assert_eq!(row(&buffer, 2), "──────");
}

#[test]
fn test_sortable_list_render_drop_indicator_at_end() {
    let mut list = abc();
    list.set_selected(Some(0));
    list.start_drag();
    list.update_drop_target(9, 0);
    let buffer = render(&list, 6, 5);
    assert_eq!(row(&buffer, 2), "≡   C ");
    assert_eq!(row(&buffer, 3), "──────");
}

#[test]
fn test_sortable_list_render_truncates_label() {
    let list = SortableList::new(["Long label"]);
    let buffer = render(&list, 7, 1);
    assert_eq!(row(&buffer, 0), "≡   Lon");
}

// ---------------------------------------------------------------------------
// Keyboard
// ---------------------------------------------------------------------------

#[test]
fn test_sortable_list_handle_key() {
    let mut list = abc();
    let down = KeyEvent::new(Key::Down);
    assert_eq!(list.handle_key(&down), EventResult::ConsumedAndRender);
    assert_eq!(list.selected(), Some(0));
    list.handle_key(&KeyEvent::new(Key::Char('j')));
    assert_eq!(list.selected(), Some(1));
    list.handle_key(&KeyEvent::new(Key::Up));
    list.handle_key(&KeyEvent::new(Key::Char('k')));
    assert_eq!(list.selected(), Some(0));

    list.handle_key(&KeyEvent::new(Key::End));
    assert_eq!(list.selected(), Some(2));
    list.handle_key(&KeyEvent::new(Key::Home));
    assert_eq!(list.selected(), Some(0));

    assert_eq!(
        list.handle_key(&KeyEvent::new(Key::Enter)),
        EventResult::Ignored
    );
    // Nothing above moved an item
    assert_eq!(labels(&list), vec!["A", "B", "C"]);
}

#[test]
fn test_sortable_list_handle_key_move() {
    let mut list = abc();
    list.set_selected(Some(0));

    let mut shift_down = KeyEvent::new(Key::Down);
    shift_down.shift = true;
    list.handle_key(&shift_down);
    assert_eq!(labels(&list), vec!["B", "A", "C"]);
    assert_eq!(list.selected(), Some(1));

    let mut alt_down = KeyEvent::new(Key::Char('j'));
    alt_down.alt = true;
    list.handle_key(&alt_down);
    assert_eq!(labels(&list), vec!["B", "C", "A"]);

    let mut shift_up = KeyEvent::new(Key::Up);
    shift_up.shift = true;
    list.handle_key(&shift_up);
    assert_eq!(labels(&list), vec!["B", "A", "C"]);
    assert_eq!(list.selected(), Some(1));
}

#[test]
fn test_sortable_list_home_end_on_empty_list() {
    let mut list = SortableList::new(Vec::<String>::new());
    list.handle_key(&KeyEvent::new(Key::End));
    assert_eq!(list.selected(), None);
    list.handle_key(&KeyEvent::new(Key::Home));
    assert_eq!(list.selected(), None);
}

#[test]
fn test_sortable_list_escape_cancels_drag() {
    let mut list = abc();
    let esc = KeyEvent::new(Key::Escape);
    assert_eq!(list.handle_key(&esc), EventResult::Ignored);

    list.set_selected(Some(1));
    list.start_drag();
    assert_eq!(list.handle_key(&esc), EventResult::ConsumedAndRender);
    assert!(!list.is_dragging());
}

// ---------------------------------------------------------------------------
// Mouse
// ---------------------------------------------------------------------------

#[test]
fn test_sortable_list_handle_mouse_click_selects() {
    let mut list = abc();
    let r = list.handle_mouse(&mouse(5, 1, MouseEventKind::Down(MouseButton::Left)), AREA);
    assert_eq!(r, EventResult::ConsumedAndRender);
    assert_eq!(list.selected(), Some(1));
    // Clicking the label, not the handle, does not start a drag
    assert!(!list.is_dragging());

    // Below the last item selects the last item
    list.handle_mouse(&mouse(5, 4, MouseEventKind::Down(MouseButton::Left)), AREA);
    assert_eq!(list.selected(), Some(2));
}

#[test]
fn test_sortable_list_handle_mouse_outside_area() {
    let mut list = abc();
    let r = list.handle_mouse(&mouse(20, 1, MouseEventKind::Down(MouseButton::Left)), AREA);
    assert_eq!(r, EventResult::Ignored);
    assert_eq!(list.selected(), None);
}

#[test]
fn test_sortable_list_handle_mouse_drag_and_drop() {
    let (calls, cb) = recorder();
    let mut list = abc().on_reorder(cb);

    // Press on the handle of "A" to start dragging it
    list.handle_mouse(&mouse(0, 0, MouseEventKind::Down(MouseButton::Left)), AREA);
    assert!(list.is_dragging());

    list.handle_mouse(&mouse(0, 3, MouseEventKind::Drag(MouseButton::Left)), AREA);
    assert_eq!(list.drop_target, Some(3));

    list.handle_mouse(&mouse(0, 3, MouseEventKind::Up(MouseButton::Left)), AREA);
    assert_eq!(labels(&list), vec!["B", "C", "A"]);
    assert_eq!(list.selected(), Some(2));
    assert!(!list.is_dragging());
    assert_eq!(*calls.borrow(), vec![(0, 2)]);
}

#[test]
fn test_sortable_list_handle_mouse_drag_without_press_is_ignored() {
    let mut list = abc();
    let r = list.handle_mouse(&mouse(0, 2, MouseEventKind::Drag(MouseButton::Left)), AREA);
    assert_eq!(r, EventResult::Ignored);
    assert_eq!(list.drop_target, None);
}

#[test]
fn test_sortable_list_handle_mouse_scroll() {
    let mut list = abc();
    list.handle_mouse(&mouse(0, 0, MouseEventKind::ScrollDown), AREA);
    let buffer = render(&list, 6, 3);
    assert_eq!(row(&buffer, 0), "≡   B ");

    // Scrolling stops with the last item at the top
    list.handle_mouse(&mouse(0, 0, MouseEventKind::ScrollDown), AREA);
    list.handle_mouse(&mouse(0, 0, MouseEventKind::ScrollDown), AREA);
    let buffer = render(&list, 6, 3);
    assert_eq!(row(&buffer, 0), "≡   C ");

    list.handle_mouse(&mouse(0, 0, MouseEventKind::ScrollUp), AREA);
    list.handle_mouse(&mouse(0, 0, MouseEventKind::ScrollUp), AREA);
    list.handle_mouse(&mouse(0, 0, MouseEventKind::ScrollUp), AREA);
    let buffer = render(&list, 6, 3);
    assert_eq!(row(&buffer, 0), "≡   A ");
}

#[test]
fn test_sortable_list_handle_mouse_on_empty_list() {
    let mut list = SortableList::new(Vec::<String>::new());
    let r = list.handle_mouse(&mouse(0, 0, MouseEventKind::Down(MouseButton::Left)), AREA);
    assert_eq!(r, EventResult::Ignored);
    assert_eq!(list.selected(), None);
    assert!(!list.is_dragging());
}

// ---------------------------------------------------------------------------
// Draggable
// ---------------------------------------------------------------------------

#[test]
fn test_sortable_list_draggable_source() {
    let mut list = abc();
    assert!(!list.can_drag());
    assert!(list.drag_data().is_none());
    assert!(list.drag_preview().is_none());

    list.set_selected(Some(1));
    assert!(list.can_drag());
    let data = list.drag_data().unwrap();
    assert_eq!(data.as_list_index(), Some(1));
    assert_eq!(data.display_label(), "B");
    assert_eq!(list.drag_preview().as_deref(), Some("↕ B"));
}

#[test]
fn test_sortable_list_can_drop() {
    assert!(abc().can_drop());
}

#[test]
fn test_sortable_list_accepted_types() {
    assert_eq!(abc().accepted_types(), &["list_item"]);
}

#[test]
fn test_sortable_list_drag_start_and_end() {
    let mut list = abc();
    list.set_selected(Some(0));
    list.on_drag_start();
    assert!(list.is_dragging());
    list.update_drop_target(2, 0);
    list.on_drag_end(DropResult::Accepted);
    assert_eq!(labels(&list), vec!["B", "A", "C"]);
    assert!(!list.is_dragging());

    for result in [DropResult::Rejected, DropResult::Cancelled] {
        let mut list = abc();
        list.set_selected(Some(0));
        list.on_drag_start();
        list.update_drop_target(2, 0);
        list.on_drag_end(result);
        assert_eq!(labels(&list), vec!["A", "B", "C"]);
        assert!(!list.is_dragging());
    }
}

#[test]
fn test_sortable_list_on_drop() {
    let (calls, cb) = recorder();
    let mut list = abc().on_reorder(cb);
    list.drop_target = Some(0);

    assert!(list.on_drop(DragData::list_item(2, "C")));
    assert_eq!(labels(&list), vec!["C", "A", "B"]);
    assert_eq!(list.selected(), Some(0));
    assert_eq!(*calls.borrow(), vec![(2, 0)]);
}

#[test]
fn test_sortable_list_on_drop_rejects() {
    let mut list = abc();
    // No drop target yet
    assert!(!list.on_drop(DragData::list_item(0, "A")));

    list.drop_target = Some(1);
    // Wrong payload type, same position, and an index past the end
    assert!(!list.on_drop(DragData::text("A")));
    assert!(!list.on_drop(DragData::list_item(1, "B")));
    assert!(!list.on_drop(DragData::list_item(9, "?")));
    assert_eq!(labels(&list), vec!["A", "B", "C"]);
}
