//! MultiSelect on the screen: the header (placeholder, tags, query, arrow),
//! the dropdown rows, their colors, the inline and overlay paths, empty and
//! offset areas, and a model of what the rows and tags show

use proptest::prelude::*;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::{Color, Style, VisualStyle};
use revue::widget::theme::{DISABLED_FG, LIGHT_GRAY, PLACEHOLDER_FG, SECONDARY_TEXT};
use revue::widget::traits::RenderContext;
use revue::widget::View;
use revue::widget::{multi_select, MultiSelect, MultiSelectOption, OverlayQueue};

use crate::traits::placement::{assert_same_when_offset, assert_zero_areas_draw_nothing};

/// The default header colors (unfocused)
const FG: Color = Color::WHITE;
const BG: Color = Color::rgb(50, 50, 50);
/// The default tag colors, and those of the tag under the tag cursor
const TAG_BG: Color = Color::rgb(60, 60, 140);
const CURSOR_TAG_BG: Color = Color::rgb(100, 100, 200);
/// The colors of the dropdown row under the cursor
const CURSOR_ROW_BG: Color = Color::rgb(80, 80, 150);

fn fruits() -> MultiSelect {
    multi_select().options(vec!["Apple", "Banana", "Cherry"])
}

/// Render inline (no overlay queue) into `area` of a `w` x `h` buffer
fn render_at(v: &MultiSelect, w: u16, h: u16, area: Rect) -> Buffer {
    let mut buffer = Buffer::new(w, h);
    let mut ctx = RenderContext::new(&mut buffer, area);
    v.render(&mut ctx);
    buffer
}

/// Render inline into a whole `w` x `h` buffer
fn render_in(v: &MultiSelect, w: u16, h: u16) -> Buffer {
    render_at(v, w, h, Rect::new(0, 0, w, h))
}

/// Render through an overlay queue at `area` of a `w` x `h` buffer, then
/// draw the queue the way the app does. Also returns the queued areas.
fn render_overlay(v: &MultiSelect, w: u16, h: u16, area: Rect) -> (Buffer, Vec<Rect>) {
    let mut buffer = Buffer::new(w, h);
    let mut queue = OverlayQueue::new();
    {
        let mut ctx = RenderContext::new(&mut buffer, area).with_overlay_queue(&mut queue);
        v.render(&mut ctx);
    }
    let entries = queue.drain_sorted();
    let areas = entries.iter().map(|e| e.area).collect();
    let mut queue = OverlayQueue::new();
    for e in entries {
        queue.push(e);
    }
    queue.render_to(&mut buffer);
    (buffer, areas)
}

fn row(b: &Buffer, y: u16) -> String {
    (0..b.width())
        .map(|x| b.get(x, y).unwrap().symbol)
        .collect()
}

/// The symbols of columns `from..to` of row `y`
fn cols(b: &Buffer, y: u16, from: u16, to: u16) -> String {
    (from..to).map(|x| b.get(x, y).unwrap().symbol).collect()
}

fn fg(b: &Buffer, x: u16, y: u16) -> Option<Color> {
    b.get(x, y).unwrap().fg
}

fn bg(b: &Buffer, x: u16, y: u16) -> Option<Color> {
    b.get(x, y).unwrap().bg
}

fn same(a: &Buffer, b: &Buffer) -> bool {
    a.width() == b.width()
        && a.height() == b.height()
        && (0..a.height()).all(|y| (0..a.width()).all(|x| a.get(x, y) == b.get(x, y)))
}

fn blank(b: &Buffer) -> bool {
    same(b, &Buffer::new(b.width(), b.height()))
}

/// Render into an `area_width` x 1 buffer and return the column of the
/// dropdown arrow, which the widget draws in the last column it occupies.
fn rendered_width(select: &MultiSelect, area_width: u16) -> u16 {
    let mut buffer = Buffer::new(area_width, 1);
    let area = Rect::new(0, 0, area_width, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    select.render(&mut ctx);
    let arrow = (0..area_width)
        .find(|&x| buffer.get(x, 0).map(|c| c.symbol) == Some('▼'))
        .expect("dropdown arrow should be drawn");
    arrow + 1
}

// MultiSelect tests - Display width (observed through the rendered arrow)

#[test]
fn test_display_width_with_custom_width() {
    let select = MultiSelect::new().width(30);
    assert_eq!(rendered_width(&select, 100), 30);
}

#[test]
fn test_display_width_custom_width_capped_by_max() {
    let select = MultiSelect::new().width(150);
    assert_eq!(rendered_width(&select, 100), 100);
}

#[test]
fn test_display_width_from_options() {
    let select = MultiSelect::new().options(vec!["Apple", "Banana", "Strawberry"]);
    // "Strawberry" (10) + 4 = 14
    assert_eq!(rendered_width(&select, 100), 14);
}

#[test]
fn test_display_width_from_placeholder_when_empty() {
    let select = MultiSelect::new().placeholder("Select an item please");
    // "Select an item please" (21) + 4 = 25
    assert_eq!(rendered_width(&select, 100), 25);
}

#[test]
fn test_display_width_capped_by_max() {
    let select = MultiSelect::new().options(vec!["Very long option name that exceeds maximum"]);
    assert_eq!(rendered_width(&select, 20), 20);
}

// Placement: empty, narrow and offset areas; inline and overlay paths

/// One widget in each state the header and dropdown draw differently
fn states() -> Vec<MultiSelect> {
    let mut selected = fruits();
    selected.select_option(2);
    selected.select_option(0);
    let mut tag_cursor = selected.clone();
    tag_cursor.tag_cursor_left();
    let mut open = selected.clone();
    open.open();
    open.cursor_down();
    let mut query = fruits();
    query.open();
    query.set_query("an");
    vec![fruits(), selected, tag_cursor, open, query]
}

#[test]
fn zero_areas_draw_nothing() {
    for v in states() {
        assert_zero_areas_draw_nothing(&v);
    }
}

#[test]
fn areas_narrower_than_five_columns_draw_and_queue_nothing() {
    for v in states() {
        for w in 1..5 {
            assert!(blank(&render_at(&v, 10, 4, Rect::new(0, 0, w, 4))), "{w}");
            let (b, queued) = render_overlay(&v, 10, 4, Rect::new(0, 0, w, 4));
            assert!(blank(&b) && queued.is_empty(), "{w}");
        }
    }
}

#[test]
fn same_when_offset() {
    for v in states() {
        assert_same_when_offset(v, 30, 6);
    }
}

#[test]
fn overlay_draws_what_inline_draws_at_the_area_position() {
    // The app draws through the overlay layer. MultiSelect adds the area's
    // position to the overlay (unlike Tooltip, #1053), so an overlay below a
    // one-row area matches the inline rows drawn inside a taller area.
    let mut v = fruits();
    v.select_option(1);
    v.open();
    for (x, y) in [(0, 0), (3, 2), (7, 5)] {
        let inline = render_at(&v, 40, 16, Rect::new(x, y, 30, 8));
        let (overlay, areas) = render_overlay(&v, 40, 16, Rect::new(x, y, 30, 1));
        assert!(same(&inline, &overlay), "at ({x}, {y})");
        assert_eq!(areas, [Rect::new(x, y + 1, 10, 3)]);
        assert_eq!(cols(&overlay, y + 2, x, x + 10), "[x] Banana");
    }
}

#[test]
fn overlay_opens_above_the_header_without_room_below() {
    let v = {
        let mut v = fruits();
        v.open();
        v
    };
    let (b, areas) = render_overlay(&v, 30, 6, Rect::new(2, 5, 20, 1));
    assert_eq!(areas, [Rect::new(2, 2, 10, 3)]);
    assert_eq!(cols(&b, 2, 2, 12), "[ ] Apple ");
    assert_eq!(cols(&b, 4, 2, 12), "[ ] Cherry");
    // The header keeps its row and its arrow
    assert_eq!(b.get(11, 5).unwrap().symbol, '▲');
    // With room for one row less than needed below, it still opens above
    let (b, areas) = render_overlay(&v, 30, 6, Rect::new(0, 3, 20, 1));
    assert_eq!(areas, [Rect::new(0, 0, 10, 3)]);
    assert_eq!(b.get(9, 3).unwrap().symbol, '▲');
    // (Without room above either it covers the header: draft-ms-3.)
}

#[test]
fn inline_rows_stay_in_the_area_and_overlay_rows_leave_it() {
    let mut v = fruits();
    v.open();
    let inline = render_at(&v, 30, 5, Rect::new(0, 0, 30, 2));
    assert_eq!(cols(&inline, 1, 0, 9), "[ ] Apple");
    assert!(row(&inline, 2).trim().is_empty());
    let (overlay, _) = render_overlay(&v, 30, 5, Rect::new(0, 0, 30, 2));
    assert_eq!(cols(&overlay, 3, 0, 10), "[ ] Cherry");
}

#[test]
fn closed_widgets_queue_nothing() {
    let mut v = fruits();
    v.select_option(0);
    let (b, areas) = render_overlay(&v, 30, 5, Rect::new(0, 0, 30, 1));
    assert!(areas.is_empty());
    assert!(same(&b, &render_in(&v, 30, 5)));
}

// The header

#[test]
fn closed_without_selection_shows_the_placeholder() {
    let v = multi_select().options(vec!["Strawberry"]);
    let b = render_in(&v, 30, 1);
    // "Strawberry" (10) + 4 = 14 columns: placeholder, fill, arrow
    assert_eq!(row(&b, 0), format!("{:<13}▼{:16}", "Select...", ""));
    assert_eq!(fg(&b, 0, 0), Some(PLACEHOLDER_FG));
    for x in 9..13 {
        assert_eq!((fg(&b, x, 0), bg(&b, x, 0)), (Some(FG), Some(BG)), "{x}");
    }
    assert_eq!(fg(&b, 13, 0), Some(FG));
    assert_eq!(bg(&b, 14, 0), None);
    let b = render_in(&v.placeholder("Pick"), 30, 1);
    assert!(row(&b, 0).starts_with("Pick         ▼"));
}

#[test]
fn an_open_or_filled_header_shows_no_placeholder() {
    let mut open = fruits();
    open.open();
    let b = render_in(&open, 30, 1);
    assert_eq!(row(&b, 0), format!("{:9}▲{:20}", "", ""));
    let mut filled = fruits().width(30);
    filled.select_option(0);
    assert!(!row(&render_in(&filled, 30, 1), 0).contains("Select"));
}

#[test]
fn tags_show_the_selection_in_order() {
    let mut v = fruits().width(30);
    v.select_option(2);
    v.select_option(0);
    let b = render_in(&v, 30, 1);
    assert_eq!(row(&b, 0), format!("[Cherry] [Apple] {:12}▼", ""));
    for x in (0..8).chain(9..16) {
        assert_eq!(
            (fg(&b, x, 0), bg(&b, x, 0)),
            (Some(SECONDARY_TEXT), Some(TAG_BG)),
            "{x}"
        );
    }
    // The gap between tags is header background
    assert_eq!(bg(&b, 8, 0), Some(BG));
    let b = render_in(&v.tag_bg(Color::GREEN), 30, 1);
    assert_eq!(bg(&b, 0, 0), Some(Color::GREEN));
    assert_eq!(bg(&b, 15, 0), Some(Color::GREEN));
}

#[test]
fn the_tag_under_the_tag_cursor_is_highlighted() {
    let mut v = fruits().width(30).tag_bg(Color::GREEN);
    v.select_option(0);
    v.select_option(1);
    v.tag_cursor_left(); // on Banana
    let b = render_in(&v, 30, 1);
    for x in 8..16 {
        assert_eq!(
            (fg(&b, x, 0), bg(&b, x, 0)),
            (Some(Color::WHITE), Some(CURSOR_TAG_BG)),
            "{x}"
        );
    }
    assert_eq!(bg(&b, 0, 0), Some(Color::GREEN));
    v.tag_cursor_right(); // off the tags
    assert_eq!(bg(&render_in(&v, 30, 1), 8, 0), Some(Color::GREEN));
}

#[test]
fn a_tag_that_does_not_fit_becomes_dots() {
    let mut v = multi_select().options(vec!["abc", "defgh"]).width(12);
    v.select_all();
    let b = render_in(&v, 20, 1);
    assert_eq!(cols(&b, 0, 0, 12), "[abc] ...  ▼");
    for x in 6..9 {
        assert_eq!(fg(&b, x, 0), Some(LIGHT_GRAY), "{x}");
    }
}

#[test]
fn the_query_follows_the_tags_in_cyan() {
    let mut v = fruits().width(30);
    v.select_option(0);
    v.open();
    v.set_query("an");
    let b = render_in(&v, 30, 1);
    assert_eq!(row(&b, 0), format!("[Apple]  an{:18}▲", ""));
    assert_eq!(fg(&b, 9, 0), Some(Color::CYAN));
    assert_eq!(fg(&b, 10, 0), Some(Color::CYAN));
    // Not shown when the widget is not searchable or the query is empty
    let mut plain = fruits().width(30).searchable(false);
    plain.select_option(0);
    plain.open();
    assert_eq!(
        row(&render_in(&plain, 30, 1), 0),
        format!("[Apple] {:21}▲", "")
    );
}

#[test]
fn focus_disabled_and_color_builders_change_the_header() {
    let base = fruits().width(12);
    // The fill at column 10 and the arrow at column 11
    let header = |v: &MultiSelect| {
        let b = render_in(v, 20, 1);
        (bg(&b, 10, 0), fg(&b, 11, 0))
    };
    assert_eq!(header(&base), (Some(BG), Some(FG)));
    let (focused_bg, _) = header(&base.clone().focused(true));
    assert_ne!(focused_bg, Some(BG));
    // Disabled: gray arrow, and no focus tint
    let disabled = header(&base.clone().disabled(true));
    assert_eq!(disabled.1, Some(DISABLED_FG));
    assert_eq!(header(&base.clone().disabled(true).focused(true)), disabled);
    assert_eq!(
        header(&base.fg(Color::RED).bg(Color::BLUE)),
        (Some(Color::BLUE), Some(Color::RED))
    );
}

#[test]
fn stylesheet_colors_reach_the_header_and_the_rows() {
    let style = Style {
        visual: VisualStyle {
            color: Color::MAGENTA,
            background: Color::GREEN,
            ..VisualStyle::default()
        },
        ..Default::default()
    };
    let mut v = fruits();
    v.open();
    let mut buffer = Buffer::new(30, 5);
    v.render(&mut RenderContext::with_style(
        &mut buffer,
        Rect::new(0, 0, 30, 5),
        &style,
    ));
    // The header fill and arrow, and a row that is not under the cursor
    assert_eq!(bg(&buffer, 0, 0), Some(Color::GREEN));
    assert_eq!(fg(&buffer, 9, 0), Some(Color::MAGENTA));
    assert_eq!(
        (fg(&buffer, 4, 2), bg(&buffer, 4, 2)),
        (Some(Color::MAGENTA), Some(Color::GREEN))
    );
    // The cursor row keeps its own colors
    assert_eq!(
        (fg(&buffer, 4, 1), bg(&buffer, 4, 1)),
        (Some(Color::WHITE), Some(CURSOR_ROW_BG))
    );
}

#[test]
fn a_width_below_three_is_raised_to_three() {
    // Open, so that no placeholder covers the arrow (draft-ms-1)
    for w in [0, 1, 2, 3] {
        let mut v = fruits().width(w);
        v.open();
        let b = render_in(&v, 20, 1);
        assert_eq!(cols(&b, 0, 0, 4), "  ▲ ", "{w}");
        assert_eq!((bg(&b, 2, 0), bg(&b, 3, 0)), (None, None), "{w}");
        assert_eq!(bg(&b, 1, 0), Some(BG), "{w}");
    }
}

// The dropdown

#[test]
fn open_rows_show_checkboxes_and_the_cursor() {
    let mut v = fruits();
    v.select_option(1);
    v.open();
    let b = render_in(&v, 30, 5);
    assert_eq!(cols(&b, 1, 0, 10), "[ ] Apple ");
    assert_eq!(cols(&b, 2, 0, 10), "[x] Banana");
    assert_eq!(cols(&b, 3, 0, 10), "[ ] Cherry");
    assert!(row(&b, 4).trim().is_empty());
    // The cursor row is highlighted across the widget's width, and no further
    for x in 0..10 {
        assert_eq!(
            (fg(&b, x, 1), bg(&b, x, 1)),
            (Some(Color::WHITE), Some(CURSOR_ROW_BG)),
            "{x}"
        );
        assert_eq!((fg(&b, x, 2), bg(&b, x, 2)), (Some(FG), Some(BG)), "{x}");
    }
    assert_eq!(bg(&b, 10, 1), None);
    v.cursor_down();
    let b = render_in(&v, 30, 5);
    assert_eq!(bg(&b, 0, 1), Some(BG));
    assert_eq!(bg(&b, 0, 2), Some(CURSOR_ROW_BG));
}

#[test]
fn disabled_options_are_drawn_gray() {
    let mut v = multi_select()
        .option_detailed(MultiSelectOption::new("Alpha", "a"))
        .option_detailed(MultiSelectOption::new("Beta", "b").disabled(true));
    v.open();
    let b = render_in(&v, 30, 3);
    assert_eq!(cols(&b, 2, 0, 8), "[ ] Beta");
    for x in 4..8 {
        assert_eq!(fg(&b, x, 2), Some(DISABLED_FG), "{x}");
    }
    assert_eq!(fg(&b, 0, 2), Some(FG)); // the checkbox
    assert_eq!(fg(&b, 4, 1), Some(Color::WHITE)); // Alpha, under the cursor
}

#[test]
fn query_matches_take_the_highlight_color() {
    for (v, color) in [
        (fruits(), Color::YELLOW),
        (fruits().highlight_fg(Color::MAGENTA), Color::MAGENTA),
    ] {
        let mut v = v.width(30);
        v.open();
        v.set_query("an");
        let b = render_in(&v, 30, 3);
        assert_eq!(cols(&b, 1, 0, 10), "[ ] Banana");
        assert!(row(&b, 2).trim().is_empty(), "only Banana matches");
        let matched = v.get_match("Banana").unwrap().indices;
        assert!(!matched.is_empty());
        for j in 0..6 {
            let want = if matched.contains(&j) {
                color
            } else {
                Color::WHITE
            };
            assert_eq!(fg(&b, 4 + j as u16, 1), Some(want), "{j}");
        }
    }
}

#[test]
fn at_most_ten_rows_are_drawn() {
    let mut v = multi_select().options((1..=12).map(|i| format!("Item {i}")).collect());
    v.open();
    let b = render_in(&v, 30, 14);
    assert_eq!(cols(&b, 10, 0, 11), "[ ] Item 10");
    assert!(row(&b, 11).trim().is_empty());
    let (_, areas) = render_overlay(&v, 30, 14, Rect::new(0, 0, 30, 1));
    assert_eq!(areas[0].height, 10);
}

#[test]
fn labels_are_cut_at_the_widget_width() {
    let mut v = fruits().width(8);
    v.open();
    let b = render_in(&v, 30, 3);
    assert_eq!(cols(&b, 2, 0, 9), "[ ] Bana ");
}

#[test]
fn wide_labels_advance_two_columns() {
    // Only the positions; the continuation cells are #974 (draft-ms-c974)
    let mut v = multi_select().options(vec!["한글", "ab"]).width(20);
    v.select_option(0);
    let b = render_in(&v, 20, 1);
    let at = |b: &Buffer, x, y| b.get(x, y).unwrap().symbol;
    assert_eq!(
        (at(&b, 1, 0), at(&b, 3, 0), at(&b, 5, 0)),
        ('한', '글', ']')
    );
    v.open();
    let b = render_in(&v, 20, 2);
    assert_eq!((at(&b, 4, 1), at(&b, 6, 1)), ('한', '글'));
}

// A model of what the rows and tags show

const POOL: [&str; 12] = [
    "Apple",
    "Banana",
    "Cherry",
    "Date",
    "Elder",
    "Fig",
    "Grape",
    "한글",
    "日本語",
    "Kiwi",
    "Lemon",
    "Mango",
];
const QUERIES: [&str; 6] = ["", "a", "an", "e", "zz", "한"];

/// The columns the widget takes: its width (at least 3), or the longest
/// label plus 4, within the area
fn display_width(v: &MultiSelect, labels: &[&str], area_width: u16) -> u16 {
    match v.get_width() {
        Some(w) => w.max(3).min(area_width),
        None => {
            let longest = labels
                .iter()
                .map(|l| revue::utils::display_width(l))
                .max()
                .unwrap_or_else(|| revue::utils::display_width(v.get_placeholder()));
            ((longest + 4) as u16).min(area_width)
        }
    }
}

/// Read `label` drawn from column `x` of row `y` (a wide glyph takes two
/// columns), up to column `end`; returns how many characters matched and
/// the column after them
fn read_label(b: &Buffer, x: u16, y: u16, label: &str, end: u16) -> (usize, u16) {
    let mut cx = x;
    for (n, ch) in label.chars().enumerate() {
        let w = revue::utils::char_width(ch) as u16;
        if cx + w > end || b.get(cx, y).unwrap().symbol != ch {
            return (n, cx);
        }
        cx += w;
    }
    (label.chars().count(), cx)
}

proptest! {
    /// Whatever the options, selection, width, query and cursor, the
    /// dropdown shows the first ten filtered options in order: `[x]` exactly
    /// for the selected ones, each label cut at the widget's width, its
    /// query matches in the highlight color and disabled ones gray, and the
    /// cursor row alone in the cursor colors. The header's tags are the
    /// first selected labels, in selection order.
    #[test]
    fn prop_rows_and_tags_follow_the_state(
        n in 1usize..=12,
        disabled in prop::collection::vec(any::<bool>(), 12),
        picks in prop::collection::vec(0usize..12, 0..8),
        width in prop::option::of(3u16..40),
        area_width in 5u16..50,
        open in any::<bool>(),
        query in 0usize..QUERIES.len(),
        moves in 0usize..14,
    ) {
        let labels = &POOL[..n];
        let mut v = labels.iter().zip(&disabled).fold(multi_select(), |v, (l, &d)| {
            v.option_detailed(MultiSelectOption::new(*l, *l).disabled(d))
        });
        if let Some(w) = width {
            v = v.width(w);
        }
        for &p in &picks {
            v.select_option(p);
        }
        if open {
            v.open();
            v.set_query(QUERIES[query]);
            for _ in 0..moves {
                v.cursor_down();
            }
        }
        let dw = display_width(&v, labels, area_width);
        let (b, _) = render_overlay(&v, area_width, 14, Rect::new(0, 0, area_width, 1));

        // Tags: "[label]" for the first selected options, in order
        let selected: Vec<&str> = v.get_selected_labels();
        let mut x = 0;
        for label in &selected {
            if b.get(x, 0).unwrap().symbol != '[' || bg(&b, x, 0) != Some(TAG_BG) {
                break;
            }
            let (got, end) = read_label(&b, x + 1, 0, label, dw);
            prop_assert_eq!(got, label.chars().count(), "tag {:?}", label);
            prop_assert_eq!(b.get(end, 0).unwrap().symbol, ']');
            x = end + 2;
        }

        // Rows
        let shown = if open { v.get_filtered().len().min(10) } else { 0 };
        for r in 0..shown {
            let y = r as u16 + 1;
            let idx = v.get_filtered()[r];
            let label = POOL[idx];
            let mark = if v.is_selected(idx) { "[x]" } else { "[ ]" };
            prop_assert_eq!(cols(&b, y, 0, 3), mark, "row {}", r);
            let (got, _) = read_label(&b, 4, y, label, dw);
            let fits = label
                .chars()
                .scan(4u16, |cx, ch| {
                    *cx += revue::utils::char_width(ch) as u16;
                    Some(*cx)
                })
                .take_while(|&end| end <= dw)
                .count();
            prop_assert_eq!(got, fits, "row {} {:?}", r, label);
            let cursor = r == v.get_dropdown_cursor();
            let row_bg = if cursor { CURSOR_ROW_BG } else { BG };
            prop_assert_eq!(bg(&b, 0, y), Some(row_bg));
            prop_assert_eq!(bg(&b, dw - 1, y), Some(row_bg));
            let matched = v.get_match(label).map(|m| m.indices).unwrap_or_default();
            let mut cx = 4;
            for (j, ch) in label.chars().take(fits).enumerate() {
                let want = if matched.contains(&j) {
                    Color::YELLOW
                } else if disabled[idx] {
                    DISABLED_FG
                } else if cursor {
                    Color::WHITE
                } else {
                    FG
                };
                prop_assert_eq!(fg(&b, cx, y), Some(want), "row {} char {}", r, j);
                cx += revue::utils::char_width(ch) as u16;
            }
        }
        prop_assert!(row(&b, shown as u16 + 1).trim().is_empty());
    }
}
