//! Splitter divider resizing and keyboard handling

use revue::event::Key;
use revue::layout::Rect;
use revue::widget::{pane, splitter, Splitter};

fn widths(s: &Splitter) -> Vec<u16> {
    // 42 columns: 41 for the panes plus a one-column divider. An odd pane
    // budget keeps every ratio used below away from a rounding boundary.
    s.pane_areas(Rect::new(0, 0, 42, 10))
        .iter()
        .map(|(_, r)| r.width)
        .collect()
}

fn two_panes() -> Splitter {
    splitter().pane(pane("left")).pane(pane("right"))
}

#[test]
fn test_splitter_resize_without_active_divider_does_nothing() {
    let mut s = two_panes();
    s.resize(10);
    assert_eq!(widths(&s), vec![20, 21]);
}

#[test]
fn test_splitter_resize_moves_divider() {
    let mut s = two_panes();
    s.start_resize(0);
    s.resize(10);
    assert_eq!(widths(&s), vec![24, 17]);
    s.resize(-20);
    assert_eq!(widths(&s), vec![16, 25]);
}

#[test]
fn test_splitter_resize_ratio_is_clamped() {
    let mut s = two_panes();
    s.start_resize(0);
    s.resize(100);
    // Ratios clamp at 0.9 / 0.1
    assert_eq!(widths(&s), vec![36, 5]);
}

#[test]
fn test_splitter_stop_resize() {
    let mut s = two_panes();
    s.start_resize(0);
    s.stop_resize();
    s.resize(10);
    assert_eq!(widths(&s), vec![20, 21]);
}

#[test]
fn test_splitter_start_resize_rejects_out_of_range_divider() {
    let mut s = two_panes();
    // Two panes have a single divider (index 0)
    s.start_resize(1);
    s.resize(10);
    assert_eq!(widths(&s), vec![20, 21]);
}

#[test]
fn test_splitter_start_resize_without_panes() {
    let mut s = splitter();
    s.start_resize(0);
    s.resize(10);
    assert!(s.pane_areas(Rect::new(0, 0, 10, 10)).is_empty());
    assert!(!s.handle_key(&Key::Right));
}

#[test]
fn test_splitter_arrow_keys_need_active_divider() {
    let mut s = two_panes();
    assert!(!s.handle_key(&Key::Right));
    assert!(!s.handle_key(&Key::Enter));
    assert_eq!(widths(&s), vec![20, 21]);
}

#[test]
fn test_splitter_keys_resize_in_steps_of_five() {
    let mut s = two_panes();
    s.start_resize(0);
    assert!(s.handle_key(&Key::Right));
    assert_eq!(widths(&s), vec![22, 19]);
    assert!(s.handle_key(&Key::Char('l')));
    assert!(s.handle_key(&Key::Down));
    assert!(s.handle_key(&Key::Char('j')));
    assert_eq!(widths(&s), vec![28, 13]);
    assert!(s.handle_key(&Key::Left));
    assert!(s.handle_key(&Key::Char('h')));
    assert!(s.handle_key(&Key::Up));
    assert!(s.handle_key(&Key::Char('k')));
    assert_eq!(widths(&s), vec![20, 21]);
}

#[test]
fn test_splitter_enter_and_escape_end_resize() {
    for key in [Key::Enter, Key::Escape] {
        let mut s = two_panes();
        s.start_resize(0);
        assert!(s.handle_key(&key));
        assert!(!s.handle_key(&Key::Right));
        assert_eq!(widths(&s), vec![20, 21]);
    }
}

#[test]
fn test_splitter_tab_focuses_next_pane() {
    let mut s = two_panes();
    assert_eq!(s.focused(), Some("left"));
    assert!(s.handle_key(&Key::Tab));
    assert_eq!(s.focused(), Some("right"));
    assert!(s.handle_key(&Key::Tab));
    assert_eq!(s.focused(), Some("left"));
}

#[test]
fn test_splitter_focus_skips_collapsed_pane() {
    let mut s = splitter()
        .pane(pane("a"))
        .pane(pane("b").collapsible())
        .pane(pane("c"));
    s.toggle_pane(1);
    s.focus_next();
    assert_eq!(s.focused(), Some("c"));
    s.focus_prev();
    assert_eq!(s.focused(), Some("a"));
}

#[test]
fn test_splitter_toggle_pane_hides_it_from_layout() {
    let mut s = splitter()
        .pane(pane("a"))
        .pane(pane("b").collapsible())
        .pane(pane("c"));
    s.toggle_pane(1);
    let ids: Vec<String> = s
        .pane_areas(Rect::new(0, 0, 41, 10))
        .into_iter()
        .map(|(id, _)| id)
        .collect();
    assert_eq!(ids, vec!["a", "c"]);
    s.toggle_pane(1);
    assert_eq!(s.pane_areas(Rect::new(0, 0, 41, 10)).len(), 3);
    // Out-of-range index is ignored
    s.toggle_pane(9);
    assert_eq!(s.pane_areas(Rect::new(0, 0, 41, 10)).len(), 3);
}
