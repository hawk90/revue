//! VirtualList rendering: empty input, edges, a matrix over sizes and
//! moves, and the widget inside a stack with a stylesheet

use proptest::prelude::*;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::data::virtuallist::VirtualList;
use revue::widget::traits::{RenderContext, View};

const SELECTED_BG: Color = Color::CYAN;

fn render<T: ToString + Clone>(list: &VirtualList<T>, width: u16, height: u16) -> Buffer {
    let mut buffer = Buffer::new(width, height);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, width, height));
    list.render(&mut ctx);
    buffer
}

/// Row `y` up to column `width`, continuation cells skipped
fn row_text(buffer: &Buffer, y: u16, width: u16) -> String {
    (0..width)
        .filter_map(|x| buffer.get(x, y))
        .filter(|cell| !cell.is_continuation())
        .map(|cell| cell.symbol)
        .collect::<String>()
        .trim_end()
        .to_string()
}

fn rows(buffer: &Buffer, width: u16) -> Vec<String> {
    (0..buffer.height())
        .map(|y| row_text(buffer, y, width))
        .collect()
}

fn highlighted(buffer: &Buffer) -> Vec<u16> {
    (0..buffer.height())
        .filter(|&y| buffer.get(0, y).unwrap().bg == Some(SELECTED_BG))
        .collect()
}

fn numbers(n: usize) -> VirtualList<usize> {
    VirtualList::new((0..n).collect()).selected_style(Color::BLACK, SELECTED_BG)
}

// =========================================================================
// #857
// =========================================================================

#[test]
fn test_virtual_list_draws_with_the_custom_renderer() {
    let list = VirtualList::new(vec![1, 2])
        .show_scrollbar(false)
        .renderer(|v, i, selected| format!("{i}:{v}{}", if selected { "*" } else { "" }));
    assert_eq!(rows(&render(&list, 10, 2), 10), ["0:1*", "1:2"]);
}

#[test]
fn test_virtual_list_item_taller_than_the_viewport() {
    let mut list = VirtualList::new(vec!["a", "b"])
        .item_height(3)
        .show_scrollbar(false)
        .selected_style(Color::BLACK, SELECTED_BG);
    let buffer = render(&list, 4, 2);
    assert_eq!(rows(&buffer, 4), ["a", ""]);
    assert_eq!(highlighted(&buffer), [0, 1]);

    list.select_next();
    assert_eq!(rows(&render(&list, 4, 2), 4), ["b", ""]);
    list.page_up(2);
    assert_eq!(
        list.selected_index(),
        Some(0),
        "a page is at least one item"
    );
}

// =========================================================================
// Empty input
// =========================================================================

#[test]
fn test_virtual_list_empty_renders_nothing_at_any_size() {
    let list: VirtualList<String> = VirtualList::new(vec![]);
    for w in 0..20 {
        for h in 0..5 {
            let buffer = render(&list, w, h);
            assert!(rows(&buffer, w).iter().all(String::is_empty), "{w}x{h}");
        }
    }
}

#[test]
fn test_virtual_list_empty_ignores_every_action() {
    let mut list: VirtualList<String> = VirtualList::new(vec![]);
    list.select_next();
    list.select_prev();
    list.select_first();
    list.select_last();
    list.page_down(10);
    list.page_up(10);
    list.jump_to(0);
    list.scroll_by(5);
    list.scroll_by(-5);
    list.set_scroll_position(1.0);
    list.ensure_visible(0);
    assert_eq!(list.remove(0), None);
    assert_eq!(list.selected_index(), None);
    assert_eq!(list.selected_item(), None);
    assert_eq!(list.visible_range(10), 0..0);
    render(&list, 10, 3);
}

#[test]
fn test_virtual_list_zero_size_areas_draw_nothing() {
    let list = numbers(5);
    for (w, h) in [(0, 0), (0, 5), (5, 0), (1, 5)] {
        let buffer = render(&list, w, h);
        assert!(rows(&buffer, w).iter().all(String::is_empty), "{w}x{h}");
    }
}

// =========================================================================
// Edges
// =========================================================================

#[test]
fn test_virtual_list_wide_characters_take_two_cells() {
    let list = VirtualList::new(vec!["한글ab"]).show_scrollbar(false);
    let buffer = render(&list, 8, 1);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '한');
    assert!(buffer.get(1, 0).unwrap().is_continuation());
    assert_eq!(buffer.get(2, 0).unwrap().symbol, '글');
    assert!(buffer.get(3, 0).unwrap().is_continuation());
    assert_eq!(row_text(&buffer, 0, 8), "한글ab");
}

#[test]
fn test_virtual_list_wide_character_is_never_cut_in_half() {
    let list = VirtualList::new(vec!["한글"]).show_scrollbar(false);
    let buffer = render(&list, 3, 1);
    assert_eq!(row_text(&buffer, 0, 3), "한");
    assert_eq!(buffer.get(2, 0).unwrap().symbol, ' ');
}

#[test]
fn test_virtual_list_narrowest_area_with_a_scrollbar() {
    let list = numbers(10);
    let buffer = render(&list, 2, 3);
    assert_eq!(rows(&buffer, 1), ["0", "1", "2"]);
    assert_eq!(buffer.get(1, 0).unwrap().symbol, '█');
}

#[test]
fn test_virtual_list_selection_at_both_ends() {
    let mut list = numbers(20).show_scrollbar(false);
    list.select_last();
    let buffer = render(&list, 4, 5);
    assert_eq!(rows(&buffer, 4), ["15", "16", "17", "18", "19"]);
    assert_eq!(highlighted(&buffer), [4]);

    list.select_first();
    let buffer = render(&list, 4, 5);
    assert_eq!(rows(&buffer, 4)[0], "0");
    assert_eq!(highlighted(&buffer), [0]);
}

#[test]
fn test_virtual_list_highlight_covers_every_row_of_a_tall_item() {
    let list = numbers(4).item_height(2).selected(1).show_scrollbar(false);
    let buffer = render(&list, 4, 8);
    assert_eq!(rows(&buffer, 4), ["0", "", "1", "", "2", "", "3", ""]);
    assert_eq!(highlighted(&buffer), [2, 3]);
}

#[test]
fn test_virtual_list_scrollbar_only_when_items_overflow() {
    let fits = numbers(3);
    let buffer = render(&fits, 5, 3);
    assert!((0..3).all(|y| buffer.get(4, y).unwrap().symbol == ' '));

    let mut over = numbers(30);
    let buffer = render(&over, 5, 3);
    assert_eq!(buffer.get(4, 0).unwrap().symbol, '█');
    assert_eq!(buffer.get(4, 2).unwrap().symbol, '░');

    over.select_last();
    let buffer = render(&over, 5, 3);
    assert_eq!(buffer.get(4, 0).unwrap().symbol, '░');
    assert_eq!(buffer.get(4, 2).unwrap().symbol, '█');
}

#[test]
fn test_virtual_list_jump_past_the_view_keeps_the_thumb_inside() {
    let mut list = numbers(20);
    list.jump_to(19);
    let buffer = render(&list, 5, 4);
    let thumb = (0..4)
        .filter(|&y| buffer.get(4, y).unwrap().symbol == '█')
        .count();
    assert!(thumb > 0, "thumb went missing");
}

// =========================================================================
// Matrix
// =========================================================================

#[derive(Debug, Clone, Copy)]
enum Move {
    Next,
    Prev,
    First,
    Last,
    PageDown,
    PageUp,
    Jump(usize),
}

fn moves() -> impl Strategy<Value = Vec<Move>> {
    prop::collection::vec(
        prop_oneof![
            Just(Move::Next),
            Just(Move::Prev),
            Just(Move::First),
            Just(Move::Last),
            Just(Move::PageDown),
            Just(Move::PageUp),
            (0usize..50).prop_map(Move::Jump),
        ],
        0..12,
    )
}

fn walk(list: &mut VirtualList<usize>, moves: &[Move], height: u16) {
    for m in moves {
        match *m {
            Move::Next => list.select_next(),
            Move::Prev => list.select_prev(),
            Move::First => list.select_first(),
            Move::Last => list.select_last(),
            Move::PageDown => list.page_down(height),
            Move::PageUp => list.page_up(height),
            Move::Jump(i) => list.jump_to(i),
        }
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    /// After any moves the selected item is on screen, only its rows are
    /// highlighted, and the items shown are consecutive.
    #[test]
    fn the_selection_is_shown_and_rows_are_consecutive(
        n in 0usize..40,
        item_height in 1u16..4,
        w in 4u16..12,
        h in 1u16..12,
        scrollbar in any::<bool>(),
        wrap in any::<bool>(),
        mv in moves(),
    ) {
        let mut list = numbers(n)
            .item_height(item_height)
            .show_scrollbar(scrollbar)
            .wrap_navigation(wrap);
        walk(&mut list, &mv, h);
        let buffer = render(&list, w, h);
        let content = if scrollbar { w - 1 } else { w };

        let Some(selected) = list.selected_index() else {
            prop_assert_eq!(n, 0);
            return Ok(());
        };
        let lit = highlighted(&buffer);
        prop_assert_eq!(lit.len(), usize::from(item_height.min(h)), "{:?}", lit);
        prop_assert!(lit.windows(2).all(|p| p[1] == p[0] + 1), "{:?}", lit);
        prop_assert_eq!(row_text(&buffer, lit[0], content), selected.to_string());

        let starts: Vec<usize> = (0..h)
            .step_by(usize::from(item_height))
            .map(|y| row_text(&buffer, y, content))
            .take_while(|t| !t.is_empty())
            .map(|t| t.parse().unwrap())
            .collect();
        prop_assert!(starts.windows(2).all(|p| p[1] == p[0] + 1), "{:?}", starts);
    }

    /// Any list, any area, any moves: rendering never panics and never
    /// draws outside its area.
    #[test]
    fn render_stays_inside_its_area(
        n in 0usize..30,
        item_height in 1u16..5,
        w in 0u16..16,
        h in 0u16..8,
        scrollbar in any::<bool>(),
        mv in moves(),
    ) {
        let mut list = VirtualList::new((0..n).map(|i| format!("항목{i}")).collect())
            .item_height(item_height)
            .show_scrollbar(scrollbar);
        for m in &mv {
            if let Move::Jump(i) = m {
                list.jump_to(*i);
            } else {
                list.select_next();
            }
        }
        let sentinel = '#';
        let mut buffer = Buffer::new(w + 4, h + 4);
        for y in 0..h + 4 {
            for x in 0..w + 4 {
                buffer.set(x, y, revue::render::Cell::new(sentinel));
            }
        }
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(2, 2, w, h));
        list.render(&mut ctx);
        for y in 0..h + 4 {
            for x in 0..w + 4 {
                let inside = (2..2 + w).contains(&x) && (2..2 + h).contains(&y);
                if !inside {
                    prop_assert_eq!(buffer.get(x, y).unwrap().symbol, sentinel, "({}, {})", x, y);
                }
            }
        }
    }
}

// =========================================================================
// Cross-module
// =========================================================================

/// Inside a vstack with a sibling, through the stylesheet and DOM path: the
/// selection is in view, ordinary items take `color`, the selected one keeps
/// its highlight and the sibling stays.
#[test]
fn test_virtual_list_in_a_stack_with_a_stylesheet() {
    use revue::prelude::*;
    use revue::testing::PipelineHarness;

    struct Screen;
    impl View for Screen {
        fn render(&self, ctx: &mut RenderContext) {
            vstack()
                .child(
                    VirtualList::new((0..50).map(|i| format!("row {i}")).collect())
                        .selected(30)
                        .selected_style(Color::BLACK, SELECTED_BG)
                        .renderer(|s, _, _| s.to_uppercase())
                        .element_id("vl"),
                )
                .child(Text::new("footer"))
                .render(ctx);
        }
        fn widget_type(&self) -> &'static str {
            "Screen"
        }
    }

    let ink = Color::rgb(17, 34, 51);
    let mut h = PipelineHarness::with_css("#vl { color: #112233; height: 4; }", 20, 6)
        .dom_from_render(true);
    h.draw(&Screen);
    let buffer = h.buffer();
    let shown: Vec<String> = (0..6).map(|y| row_text(buffer, y, 19)).collect();
    assert!(shown.contains(&"ROW 30".to_string()), "{shown:#?}");
    assert!(
        shown.contains(&"footer".to_string()),
        "sibling lost: {shown:#?}"
    );
    let lit: Vec<u16> = (0..6)
        .filter(|&y| buffer.get(0, y).unwrap().bg == Some(SELECTED_BG))
        .collect();
    assert_eq!(lit.len(), 1, "{shown:#?}");
    assert_eq!(shown[usize::from(lit[0])], "ROW 30");
    let plain = (0..6)
        .find(|&y| shown[usize::from(y)].starts_with("ROW") && !lit.contains(&y))
        .unwrap();
    assert_eq!(buffer.get(0, plain).unwrap().fg, Some(ink), "plain row");
    assert_eq!(buffer.get(0, lit[0]).unwrap().fg, Some(Color::BLACK));
}

// ---------------------------------------------------------------------------
// #971: rows, the row offset, ScrollMode and jump alignment
// ---------------------------------------------------------------------------

#[test]
fn test_virtual_list_draws_variable_heights() {
    let list = VirtualList::new(vec!["A", "BB", "CCC"])
        .variable_height(|s, _| s.len() as u16)
        .show_scrollbar(false);
    assert_eq!(
        rows(&render(&list, 4, 6), 4),
        ["A", "BB", "", "CCC", "", ""]
    );
}

#[test]
fn test_virtual_list_draws_the_row_offset() {
    let mut list = VirtualList::new(vec!["a", "b", "c"])
        .item_height(2)
        .show_scrollbar(false);
    list.scroll_by(1);
    assert_eq!(rows(&render(&list, 4, 4), 4), ["", "b", "", "c"]);
}

#[test]
fn test_virtual_list_center_mode_centers_the_selection() {
    use revue::widget::data::virtuallist::ScrollMode;
    let list = VirtualList::new((0..20).collect::<Vec<_>>())
        .scroll_mode(ScrollMode::Center)
        .selected(10)
        .show_scrollbar(false);
    assert_eq!(rows(&render(&list, 4, 5), 4)[2], "10");
}

#[test]
fn test_virtual_list_jump_alignment() {
    use revue::widget::data::virtuallist::ScrollAlignment;
    let mut list = VirtualList::new((0..100).collect::<Vec<_>>()).show_scrollbar(false);
    render(&list, 4, 20);
    list.jump_to_with_alignment(50, ScrollAlignment::Center);
    assert_eq!(rows(&render(&list, 4, 20), 4)[10], "50");
    list.jump_to_with_alignment(50, ScrollAlignment::End);
    assert_eq!(rows(&render(&list, 4, 20), 4)[19], "50");
}

#[test]
fn test_virtual_list_jump_to_the_end_fills_the_view() {
    let mut list = VirtualList::new((0..20).collect::<Vec<_>>()).show_scrollbar(false);
    list.jump_to(19);
    assert_eq!(rows(&render(&list, 4, 4), 4), ["16", "17", "18", "19"]);
}
