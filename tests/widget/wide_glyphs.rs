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
    multi_select, Alignment, OverlayQueue, RichText, Terminal, Toast, ToastPosition, Tree, TreeNode,
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
