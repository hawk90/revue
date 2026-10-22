//! Wide glyphs keep their right half (#974): every widget that draws a
//! CJK character or an emoji leaves a continuation cell after it, and never
//! draws half of one at an area's edge. The terminal output writes every
//! cell that is not a continuation, so a plain cell there erases the glyph.

use revue::core::app::{Inspector as AppInspector, WidgetInfo};
use revue::layout::Rect;
use revue::prelude::*;
use revue::render::Buffer;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{
    avatar, multi_select, Alignment, Breadcrumb, BreadcrumbItem, DebugEvent, DebugOverlay,
    DebugPosition, JsonViewer, OverlayQueue, RichText, SortableList, Switch, SwitchStyle, Terminal,
    Toast, ToastPosition, Tree, TreeNode,
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
    // A 3-column area at column 1 with a cell already drawn right of it:
    // "ab日" has no room for 日's right half, so 日 is left out and the
    // neighbor's cell stays (in a stack the neighbor may be drawn first)
    fn drawn(v: &dyn View) -> Buffer {
        let mut b = Buffer::new(6, 1);
        b.put_str(0, 0, "......");
        b.set(4, 0, revue::render::Cell::new('Z'));
        v.render(&mut RenderContext::new(&mut b, Rect::new(1, 0, 3, 1)));
        b
    }
    for b in [
        drawn(&RichText::new().text("ab日")),
        drawn(&Text::new("ab日")),
    ] {
        assert_eq!(row(&b, 0), ".ab Z.");
        assert!(!b.get(4, 0).unwrap().is_continuation());
    }
    // A Text that fits keeps its glyph whole
    let b = drawn(&Text::new("a日"));
    assert_eq!(b.get(2, 0).unwrap().symbol, '日');
    assert!(b.get(3, 0).unwrap().is_continuation());
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
fn terminal_cursor_goes_after_both_columns_of_a_wide_character() {
    let mut t = Terminal::new(10, 1);
    t.focus();
    t.write("한");
    let b = draw(&t, 10, 1);
    assert_eq!(b.get(0, 0).unwrap().symbol, '한');
    assert!(b.get(1, 0).unwrap().is_continuation());
    assert_eq!(b.get(2, 0).unwrap().symbol, '█');
}

#[test]
fn terminal_gives_a_combining_mark_no_column() {
    // e + U+0301 is one column; x follows it
    let mut t = Terminal::new(10, 1);
    t.write("e\u{301}x");
    let b = draw(&t, 10, 1);
    assert_eq!(b.get(0, 0).unwrap().symbol, 'e');
    assert_eq!(b.get(1, 0).unwrap().symbol, 'x');
}

#[test]
fn terminal_wraps_a_wide_character_at_every_line_end() {
    // Five syllables in 5 columns: two per line, the last column left over
    let mut t = Terminal::new(5, 3);
    t.write("가나다라마");
    let b = draw(&t, 5, 3);
    assert_eq!(row(&b, 0).trim_end(), "가나");
    assert_eq!(row(&b, 1).trim_end(), "다라");
    assert_eq!(row(&b, 2).trim_end(), "마");
    // The column a wide character could not use stays blank
    assert_eq!(b.get(4, 0).unwrap().symbol, ' ');
}

#[test]
fn terminal_one_column_wide_does_not_hang_on_a_wide_character() {
    // A guard for the one-column branch: a wide character cannot fit at
    // all, so it takes the one column and the line wraps, as narrow ones
    // do, instead of wrapping again and again; drawing it leaves a space
    let mut t = Terminal::new(1, 3);
    t.write("한a");
    let b = draw(&t, 1, 3);
    assert_eq!(b.get(0, 0).unwrap().symbol, ' ');
    assert_eq!(b.get(0, 1).unwrap().symbol, 'a');
}

#[test]
fn tree_icon_in_a_narrow_area_and_at_depth() {
    // A wide icon that would cross the area's edge is not drawn half
    let tree = Tree::new().node(TreeNode::new("ab").icon('📁'));
    let b = draw(&tree, 2, 1);
    assert_eq!(b.get(1, 0).unwrap().symbol, ' ');
    // Under a parent, the icon follows the connector
    let tree = Tree::new().node(
        TreeNode::new("p")
            .expanded(true)
            .child(TreeNode::new("c").icon('📁')),
    );
    let b = draw(&tree, 20, 2);
    let r = row(&b, 1);
    assert!(r.contains("📁c"), "{r:?}");
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
    let x = (0..20u16)
        .find(|&x| b.get(x, 0).unwrap().symbol == '한')
        .unwrap();
    assert!(b.get(x + 1, 0).unwrap().is_continuation());
    assert_eq!(b.get(x + 2, 0).unwrap().symbol, '글');
    assert!(b.get(x + 3, 0).unwrap().is_continuation());
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

/// The column of the first cell holding `ch` on row `y`
fn col_of(b: &Buffer, y: u16, ch: char) -> Option<u16> {
    (0..b.width()).find(|&x| b.get(x, y).unwrap().symbol == ch)
}

#[test]
fn avatar_initials_fit_by_width() {
    // Medium: two columns between the edges hold one wide initial; the
    // second would cross the edge and is left out
    let b = draw(&avatar("김 철수").medium().circle(), 10, 1);
    assert_eq!(b.get(1, 0).unwrap().symbol, '김');
    assert!(b.get(2, 0).unwrap().is_continuation());
    assert_eq!(b.get(3, 0).unwrap().symbol, '◗');
    // Large: three columns hold a wide initial and a blank
    let b = draw(&avatar("김 철수").large().circle(), 10, 3);
    assert_eq!(b.get(1, 1).unwrap().symbol, '김');
    assert!(b.get(2, 1).unwrap().is_continuation());
    assert_eq!(b.get(3, 1).unwrap().symbol, ' ');
    assert_eq!(b.get(4, 1).unwrap().symbol, '│');
    // ASCII initials still take one column each
    let b = draw(&avatar("Ada Lovelace").medium().circle(), 10, 1);
    assert_eq!(row(&b, 0).chars().take(4).collect::<String>(), "◖AL◗");
}

#[test]
fn breadcrumb_wide_item_icons_and_a_narrow_area() {
    let bc = Breadcrumb::new()
        .home(false)
        .item(BreadcrumbItem::new("Docs").icon('📁'))
        .item(BreadcrumbItem::new("Work").icon('💼'));
    let b = draw(&bc, 40, 1);
    assert_eq!(row(&b, 0).trim_end(), "📁 Docs › 💼 Work");
    assert_eq!(
        bc.measure(40, 1).unwrap().0 as usize,
        revue::utils::display_width("📁 Docs › 💼 Work")
    );
    // In 3 columns the home icon fits; nothing is drawn half
    let b = draw(&Breadcrumb::new().push("x"), 3, 1);
    for x in 0..3 {
        let c = b.get(x, 0).unwrap();
        if c.is_continuation() {
            assert_eq!(b.get(x - 1, 0).unwrap().symbol, '🏠');
        }
    }
}

#[test]
fn switch_emoji_keeps_its_right_half_and_the_label_follows() {
    let s = Switch::new().style(SwitchStyle::Emoji).label("on");
    let b = draw(&s, 20, 1);
    let x = col_of(&b, 0, '❌').unwrap();
    assert!(b.get(x + 1, 0).unwrap().is_continuation());
    let r = row(&b, 0);
    assert!(r.contains('❌') && r.contains("on"), "{r:?}");
}

#[test]
fn sortable_list_label_cut_at_the_edge_and_styled_throughout() {
    // Selected: every cell of the label, the right halves too, is bold
    let mut list = SortableList::new(vec!["한글한글"]);
    list.selected = Some(0);
    let b = draw(&list, 20, 1);
    let x = col_of(&b, 0, '한').unwrap();
    for dx in 0..8 {
        assert!(
            b.get(x + dx, 0)
                .unwrap()
                .modifier
                .contains(revue::render::Modifier::BOLD),
            "column {}",
            x + dx
        );
    }
    // In an area that ends inside the third syllable, it is left out
    let w = x + 5;
    let b = draw(&list, w, 1);
    assert_eq!(b.get(x + 4, 0).unwrap().symbol, ' ');
    assert!(!b.get(x + 4, 0).unwrap().is_continuation());
}

#[test]
fn json_viewer_cuts_a_wide_value_at_the_edge() {
    let v = JsonViewer::from_content(r#"{"k": "한글한글한글한글"}"#);
    for w in 8..20 {
        let b = draw(&v, w, 3);
        // The value's syllables that are drawn sit two columns apart, in
        // order, up to the edge: none is erased by the next one
        if let Some(y) = (0..3).find(|&y| col_of(&b, y, '한').is_some()) {
            let heads: Vec<(u16, char)> = (0..w)
                .map(|x| (x, b.get(x, y).unwrap().symbol))
                .filter(|&(_, c)| c == '한' || c == '글')
                .collect();
            for pair in heads.windows(2) {
                assert_eq!(pair[1].0, pair[0].0 + 2, "width {w}: {heads:?}");
                assert_ne!(pair[1].1, pair[0].1, "width {w}: {heads:?}");
            }
            // At most one column is left unused before the edge
            let (last, _) = *heads.last().unwrap();
            assert!(
                w - (last + 2) <= 1 || heads.len() == 8,
                "width {w}: {heads:?}"
            );
        }
        for y in 0..3 {
            // The last column never holds a glyph whose right half is cut
            let last = b.get(w - 1, y).unwrap();
            assert!(
                revue::utils::char_width(last.symbol) < 2,
                "width {w} row {y}: {:?}",
                last.symbol
            );
            for x in 1..w {
                if b.get(x, y).unwrap().is_continuation() {
                    let head = b.get(x - 1, y).unwrap().symbol;
                    assert_eq!(revue::utils::char_width(head), 2, "width {w} ({x},{y})");
                }
            }
        }
    }
}

#[test]
fn inspector_tree_and_properties_in_columns() {
    let mut i = AppInspector::new();
    i.set_root(WidgetInfo::new("Text", Rect::new(0, 0, 10, 3)).property("내용", "한글"));
    i.show();
    let b = draw(&i, 60, 12);
    let rows: Vec<String> = (0..12).map(|y| row(&b, y)).collect();
    assert!(rows.iter().any(|r| r.contains("내용: 한글")), "{rows:#?}");
    // Each syllable has its continuation
    for y in 0..12 {
        if let Some(x) = col_of(&b, y, '내') {
            assert!(b.get(x + 1, y).unwrap().is_continuation());
            assert_eq!(b.get(x + 2, y).unwrap().symbol, '용');
        }
    }
}

#[test]
fn debug_overlay_panel_right_edge_leaves_no_half_glyph() {
    // A top-left 11-column panel ends at column 10, inside the sixth 한
    // (columns 10-11): its right half at 11 is blanked
    let overlay = DebugOverlay::wrap(Text::new("한글한글한글한글한글"))
        .position(DebugPosition::TopLeft)
        .width(11)
        .show_metrics(false);
    let b = draw(&overlay, 20, 4);
    assert_eq!(b.get(10, 0).unwrap().symbol, '┐');
    assert!(!b.get(11, 0).unwrap().is_continuation());
    assert_eq!(b.get(11, 0).unwrap().symbol, ' ');
    // Past it the view's glyphs are whole
    assert_eq!(b.get(12, 0).unwrap().symbol, '한');
    assert!(b.get(13, 0).unwrap().is_continuation());
}
