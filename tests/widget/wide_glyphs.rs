//! Wide glyphs keep their right half (#974): every widget that draws a
//! CJK character or an emoji leaves a continuation cell after it, and never
//! draws half of one at an area's edge. The terminal output writes every
//! cell that is not a continuation, so a plain cell there erases the glyph.

use revue::layout::Rect;
use revue::prelude::*;
use revue::render::Buffer;
use revue::testing::PipelineHarness;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{
    multi_select, Alignment, DebugEvent, DebugOverlay, DebugPosition, JsonViewer, OverlayQueue,
    RichText, SortableList, Terminal, Toast, ToastPosition, Tree, TreeNode,
};

fn draw(v: &dyn View, w: u16, h: u16) -> Buffer {
    let mut b = Buffer::new(w, h);
    v.render(&mut RenderContext::new(&mut b, Rect::new(0, 0, w, h)));
    b
}

/// The symbols of row `y`, a wide glyph once
fn row(b: &Buffer, y: u16) -> String {
    (0..b.width())
        .map(|x| b.get(x, y).unwrap())
        .filter(|c| !c.is_continuation())
        .map(|c| c.symbol)
        .collect()
}

#[test]
fn toast_message() {
    let t = Toast::new("한글")
        .position(ToastPosition::TopLeft)
        .show_icon(false);
    let b = draw(&t, 20, 5);
    // Border at x = 1, message from x = 3
    assert_eq!(b.get(3, 2).unwrap().symbol, '한');
    assert!(b.get(4, 2).unwrap().is_continuation());
}

#[test]
fn rich_text_and_text_keep_a_wide_glyph_inside_their_area() {
    // A 3-column view next to a Text: "ab日" has no room for 日's right half
    let mut h = PipelineHarness::new(5, 1);
    h.draw(
        &hstack()
            .child_sized(RichText::new().text("ab日"), 3)
            .child(Text::new("Z")),
    );
    assert_eq!(h.buffer().get(3, 0).unwrap().symbol, 'Z');
    assert_ne!(h.buffer().get(2, 0).unwrap().symbol, '日');

    let mut h = PipelineHarness::new(5, 1);
    h.draw(
        &hstack()
            .child_sized(Text::new("ab日"), 3)
            .child(Text::new("Z")),
    );
    assert_eq!(h.buffer().get(3, 0).unwrap().symbol, 'Z');
    assert_ne!(h.buffer().get(2, 0).unwrap().symbol, '日');
}

#[test]
fn justified_text() {
    let t = Text::new("한글 텍스트 wide").align(Alignment::Justify);
    let b = draw(&t, 30, 1);
    assert_eq!(b.get(0, 0).unwrap().symbol, '한');
    assert!(b.get(1, 0).unwrap().is_continuation());
    assert_eq!(b.get(2, 0).unwrap().symbol, '글');
}

#[test]
fn tree_icon() {
    // The label starts after both columns of 📁
    let tree = Tree::new().node(TreeNode::new("ab").icon('📁'));
    let b = draw(&tree, 20, 3);
    assert_eq!(row(&b, 0).trim_end(), " 📁ab");
}

#[test]
fn terminal_takes_two_columns_per_wide_character() {
    let mut t = Terminal::new(10, 1);
    t.write("한글ab");
    let b = draw(&t, 10, 1);
    assert_eq!(b.get(0, 0).unwrap().symbol, '한');
    assert!(b.get(1, 0).unwrap().is_continuation());
    assert_eq!(b.get(2, 0).unwrap().symbol, '글');
    assert_eq!(b.get(4, 0).unwrap().symbol, 'a');
}

#[test]
fn terminal_wraps_a_wide_character_that_does_not_fit() {
    // In 3 columns, 글 does not fit after "a한": it goes to the next line,
    // as on a real terminal
    let mut t = Terminal::new(3, 2);
    t.write("a한글");
    let b = draw(&t, 3, 2);
    assert_eq!(row(&b, 0).trim_end(), "a한");
    assert_eq!(row(&b, 1).trim_end(), "글");
}

#[test]
fn multi_select_tag_and_dropdown_label() {
    // The tag: '[' at 0, '한' at 1, its right half at 2
    let mut s = multi_select().options(vec!["한글"]).width(20);
    s.select_option(0);
    let b = draw(&s, 20, 1);
    assert_eq!(b.get(1, 0).unwrap().symbol, '한');
    assert!(b.get(2, 0).unwrap().is_continuation());

    // The dropdown row, through the overlay queue: '한' at 4, its right half at 5
    s.open();
    let mut b = Buffer::new(20, 3);
    let mut q = OverlayQueue::new();
    {
        let mut ctx = RenderContext::new(&mut b, Rect::new(0, 0, 20, 1)).with_overlay_queue(&mut q);
        s.render(&mut ctx);
    }
    q.render_to(&mut b);
    assert_eq!(b.get(4, 1).unwrap().symbol, '한');
    assert!(b.get(5, 1).unwrap().is_continuation());
}

#[test]
fn json_viewer_keys_and_values() {
    let v = JsonViewer::from_content(r#"{"이름": "한글"}"#);
    let b = draw(&v, 30, 3);
    let text: String = (0..3).map(|y| row(&b, y)).collect::<Vec<_>>().join("\n");
    assert!(text.contains("이름") && text.contains("한글"), "{text}");
    // Each syllable keeps its right half
    let (x, y) = (0..3u16)
        .flat_map(|y| (0..30u16).map(move |x| (x, y)))
        .find(|&(x, y)| b.get(x, y).unwrap().symbol == '이')
        .unwrap();
    assert!(b.get(x + 1, y).unwrap().is_continuation());
    assert_eq!(b.get(x + 2, y).unwrap().symbol, '름');
}

#[test]
fn sortable_list_label() {
    let list = SortableList::new(vec!["한글", "b"]);
    let b = draw(&list, 20, 2);
    assert!(row(&b, 0).contains("한글"), "{:?}", row(&b, 0));
    assert!(row(&b, 1).contains('b'));
}

#[test]
fn debug_overlay_event_takes_two_columns_per_glyph() {
    let mut overlay = DebugOverlay::wrap(Text::new(""))
        .position(DebugPosition::TopLeft)
        .width(20)
        .show_metrics(false)
        .show_events(true);
    overlay.log_event(DebugEvent::Custom("한글ab".into()));
    let b = draw(&overlay, 20, 6);
    // Row 3 is the event: border, then 한 in columns 1-2
    assert_eq!(b.get(1, 3).unwrap().symbol, '한');
    assert!(b.get(2, 3).unwrap().is_continuation());
    assert_eq!(b.get(3, 3).unwrap().symbol, '글');
}

#[test]
fn debug_overlay_panel_edge_leaves_no_half_glyph() {
    // A 20-column view; the 11-column panel starts at column 9, inside the
    // fifth 한 (columns 8-9)
    let overlay = DebugOverlay::wrap(Text::new("한글한글한글한글한글"))
        .width(11)
        .show_metrics(false);
    let b = draw(&overlay, 20, 4);
    assert_eq!(b.get(9, 0).unwrap().symbol, '┌');
    assert_eq!(b.get(8, 0).unwrap().symbol, ' ');
}
