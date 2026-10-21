//! LogViewer rendering: empty input, the column layout, highlights, a matrix
//! over options, sizes and keys, and the widget inside a stack
//!
//! The view is a `ScrollState` and the selection a `KeyedSelection` (#973,
//! #975): the view follows the selection and the tail, and the selection
//! stays on its entry.

use proptest::prelude::*;
use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::data::log_viewer::{LogEntry, LogFilter, LogLevel, LogViewer};
use revue::widget::traits::{RenderContext, View};

const SELECTED_BG: Color = Color::rgb(50, 50, 80);

fn render(viewer: &LogViewer, width: u16, height: u16) -> Buffer {
    let mut buffer = Buffer::new(width.max(1), height.max(1));
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, width, height));
    viewer.render(&mut ctx);
    buffer
}

fn row_text(buffer: &Buffer, y: u16) -> String {
    (0..buffer.width())
        .filter_map(|x| buffer.get(x, y))
        .filter(|cell| !cell.is_continuation())
        .map(|cell| cell.symbol)
        .collect::<String>()
        .trim_end()
        .to_string()
}

/// Text of row `y` from column `x` for `len` cells
fn cells(buffer: &Buffer, x: u16, y: u16, len: u16) -> String {
    (x..x + len)
        .filter_map(|x| buffer.get(x, y))
        .map(|cell| cell.symbol)
        .collect()
}

/// Message only: no line numbers, timestamps, levels or sources, no tail
fn bare() -> LogViewer {
    LogViewer::new()
        .show_line_numbers(false)
        .show_timestamps(false)
        .show_levels(false)
        .show_source(false)
        .tail_mode(false)
}

fn highlighted_rows(buffer: &Buffer) -> Vec<u16> {
    (0..buffer.height())
        .filter(|&y| buffer.get(0, y).unwrap().bg == Some(SELECTED_BG))
        .collect()
}

// =========================================================================
// Empty input
// =========================================================================

#[test]
fn test_log_viewer_empty_renders_the_message_at_any_size() {
    let viewer = LogViewer::new();
    for w in 0..40 {
        for h in 0..6 {
            render(&viewer, w, h);
        }
    }
    let buffer = render(&viewer, 30, 5);
    assert_eq!(row_text(&buffer, 2).trim_start(), "No log entries");
    assert_eq!(cells(&buffer, 8, 2, 2), "No", "centered: (30 - 14) / 2");
}

#[test]
fn test_log_viewer_empty_ignores_every_action() {
    let mut viewer = LogViewer::new();
    for key in [
        Key::Up,
        Key::Down,
        Key::PageUp,
        Key::PageDown,
        Key::Home,
        Key::End,
        Key::Enter,
        Key::Char('f'),
        Key::Char('w'),
        Key::Char('b'),
        Key::Char('n'),
        Key::Char('N'),
        Key::Char(']'),
        Key::Char('['),
    ] {
        assert!(viewer.handle_key(&key), "{key:?}");
    }
    assert!(!viewer.handle_key(&Key::Char('x')));
    viewer.jump_to_line(10);
    viewer.jump_to_timestamp(0);
    viewer.search("x");
    viewer.next_match();
    viewer.prev_match();
    viewer.scroll_up(5);
    viewer.scroll_down(5);
    viewer.toggle_selected_expanded();
    assert!(viewer.selected_entry().is_none());
    assert_eq!(viewer.selected_text(), None);
    assert_eq!(viewer.export_filtered(), "");
    assert_eq!(viewer.export_formatted(), "");
    assert_eq!(viewer.search_match_count(), 0);
    assert!(viewer.bookmarked_entries().is_empty());
}

#[test]
fn test_log_viewer_filter_hiding_everything_shows_the_empty_message() {
    let mut viewer = bare();
    viewer.load("INFO a\nDEBUG b");
    viewer.set_min_level(LogLevel::Error);
    assert_eq!(viewer.filtered_len(), 0);
    assert!(viewer.selected_entry().is_none());
    let buffer = render(&viewer, 30, 3);
    assert_eq!(row_text(&buffer, 1).trim_start(), "No log entries");
}

#[test]
fn test_log_viewer_zero_size_areas_with_entries() {
    let mut viewer = LogViewer::new();
    viewer.load("2024-01-15 10:30:00 INFO [app] started");
    for (w, h) in [(0, 0), (0, 3), (40, 0), (1, 1), (2, 1)] {
        render(&viewer, w, h);
    }
}

// =========================================================================
// Edges: the column layout
// =========================================================================

/// Every column on: line number right-aligned in 5, a space, the bookmark
/// in 2, the timestamp in 12, the level in 5, the source in 12, then the
/// message.
#[test]
fn test_log_viewer_column_layout() {
    let mut viewer = LogViewer::new().tail_mode(false);
    viewer.load("2024-01-15 10:30:00 ERROR [db] failed to connect");
    viewer.toggle_bookmark();
    let buffer = render(&viewer, 80, 1);
    assert_eq!(cells(&buffer, 0, 0, 5), "    1");
    assert_eq!(cells(&buffer, 6, 0, 1), "★");
    assert_eq!(cells(&buffer, 8, 0, 11), "2024-01-15 ", "cut to 11 cells");
    assert_eq!(cells(&buffer, 20, 0, 1), "✗");
    assert_eq!(cells(&buffer, 25, 0, 2), "db");
    assert_eq!(cells(&buffer, 37, 0, 17), "failed to connect");
}

/// Each prefix column, when turned off, gives its width to the message.
#[test]
fn test_log_viewer_message_starts_after_the_shown_columns() {
    for mask in 0..16u8 {
        let mut viewer = LogViewer::new()
            .show_line_numbers(mask & 1 != 0)
            .show_timestamps(mask & 2 != 0)
            .show_levels(mask & 4 != 0)
            .show_source(mask & 8 != 0)
            .tail_mode(false);
        viewer.load("12:00:01 WARN [svc] disk low");
        let x = [(1, 6), (2, 12), (4, 5), (8, 12)]
            .iter()
            .filter(|(bit, _)| mask & bit != 0)
            .map(|(_, w)| w)
            .sum::<u16>()
            + 2;
        let buffer = render(&viewer, 80, 1);
        assert_eq!(cells(&buffer, x, 0, 8), "disk low", "mask {mask:04b}");
    }
}

#[test]
fn test_log_viewer_message_is_cut_at_the_area_edge() {
    let mut viewer = bare();
    viewer.load("abcdefghijklmnopqrstuvwxyz");
    let buffer = render(&viewer, 10, 1);
    assert_eq!(row_text(&buffer, 0), "  abcdefgh");
}

#[test]
fn test_log_viewer_narrow_area_keeps_the_prefix_in_place() {
    let mut viewer = LogViewer::new().tail_mode(false);
    viewer.load("2024-01-15 10:30:00 INFO [app] started");
    let buffer = render(&viewer, 12, 1);
    assert_eq!(cells(&buffer, 0, 0, 5), "    1");
    assert_eq!(cells(&buffer, 8, 0, 4), "2024");
}

#[test]
fn test_log_viewer_selected_row_is_highlighted_across_the_width() {
    let mut viewer = bare();
    viewer.load("one\ntwo\nthree");
    viewer.select_next();
    let buffer = render(&viewer, 20, 3);
    assert_eq!(highlighted_rows(&buffer), [1]);
    assert!((0..20).all(|x| buffer.get(x, 1).unwrap().bg == Some(SELECTED_BG)));
    assert_eq!(buffer.get(0, 0).unwrap().bg, None);
}

#[test]
fn test_log_viewer_search_highlights_the_matched_characters() {
    let mut viewer = bare();
    viewer.load("서버 오류 발생\nall good");
    viewer.search("오류");
    assert_eq!(viewer.search_match_count(), 1);
    viewer.select_next(); // off the matched row, so its colors are not the selection's
    let buffer = render(&viewer, 30, 2);
    // "  " bookmark, then "서버 " (5 cells), then "오류" (4 cells)
    let lit: Vec<u16> = (0..30)
        .filter(|&x| buffer.get(x, 0).unwrap().bg == Some(Color::YELLOW))
        .collect();
    assert_eq!(lit.first(), Some(&7));
    assert!(lit.iter().all(|&x| (7..11).contains(&x)), "{lit:?}");
    assert_eq!(buffer.get(7, 0).unwrap().fg, Some(Color::BLACK));
    assert!((0..30).all(|x| buffer.get(x, 1).unwrap().bg != Some(Color::YELLOW)));
}

/// The messages on each row of a `bare()` viewer drawn `width` wide: the
/// six cells after the bookmark column, clear of the right-hand markers
fn messages(viewer: &LogViewer, width: u16, height: u16) -> Vec<String> {
    let buffer = render(viewer, width, height);
    (0..height)
        .map(|y| cells(&buffer, 2, y, 6).trim_end().to_string())
        .collect()
}

fn numbered(viewer: &mut LogViewer, n: usize) {
    for i in 1..=n {
        viewer.push(&format!("l{i}"));
    }
}

// =========================================================================
// Scrolling and selection (#858, #973, #975)
// =========================================================================

/// Tail mode used to scroll to the last entry and draw it alone on the top
/// row (#858).
#[test]
fn test_log_viewer_tail_shows_a_full_last_page() {
    let mut viewer = bare().tail_mode(true);
    numbered(&mut viewer, 50);
    assert_eq!(messages(&viewer, 30, 4), ["l47", "l48", "l49", "l50"]);
    viewer.push("l51");
    assert_eq!(messages(&viewer, 30, 4), ["l48", "l49", "l50", "l51"]);
}

/// Down past the bottom row used to leave the selection off screen (#858).
#[test]
fn test_log_viewer_view_follows_the_selection_down() {
    let mut viewer = bare();
    numbered(&mut viewer, 20);
    render(&viewer, 30, 4);
    for _ in 0..6 {
        viewer.handle_key(&Key::Down);
    }
    assert_eq!(viewer.selected_text().as_deref(), Some("l7"));
    let buffer = render(&viewer, 30, 4);
    assert_eq!(highlighted_rows(&buffer), [3]);
    assert_eq!(cells(&buffer, 2, 3, 6).trim_end(), "l7");
}

/// Moving up inside the page keeps the page.
#[test]
fn test_log_viewer_moving_up_on_screen_keeps_the_view() {
    let mut viewer = bare();
    numbered(&mut viewer, 20);
    render(&viewer, 30, 4);
    for _ in 0..6 {
        viewer.handle_key(&Key::Down);
    }
    viewer.handle_key(&Key::Up);
    assert_eq!(messages(&viewer, 30, 4), ["l4", "l5", "l6", "l7"]);
}

/// #975: trimming old entries moved the selection to the next one.
#[test]
fn test_log_viewer_trimming_keeps_the_selected_entry() {
    let mut viewer = LogViewer::new().tail_mode(false).max_entries(3);
    for i in 1..=3 {
        viewer.push(&format!("l{i}"));
    }
    viewer.select_next();
    viewer.push("l4");
    assert_eq!(viewer.selected_entry().unwrap().message, "l2");
}

/// #975: `clear_filter` moved the selection to the entry at its position.
#[test]
fn test_log_viewer_clear_filter_keeps_the_selected_entry() {
    let mut viewer = LogViewer::new().tail_mode(false);
    viewer.load("INFO a\nERROR b\nINFO c");
    viewer.set_min_level(LogLevel::Error);
    viewer.clear_filter();
    assert_eq!(viewer.selected_entry().unwrap().message, "b");
}

/// Reading back through a trimmed log: the rows on screen stay put as old
/// entries go.
#[test]
fn test_log_viewer_trimming_keeps_a_scrolled_back_view_still() {
    let mut viewer = bare().tail_mode(true).max_entries(10);
    numbered(&mut viewer, 10);
    render(&viewer, 30, 4);
    viewer.scroll_up(3);
    assert!(!viewer.is_tail_mode());
    assert_eq!(messages(&viewer, 30, 4), ["l4", "l5", "l6", "l7"]);
    viewer.push("l11");
    viewer.push("l12");
    assert_eq!(messages(&viewer, 30, 4), ["l4", "l5", "l6", "l7"]);
}

/// Scrolling back to the newest entries starts following again; the
/// selection stays where it was.
#[test]
fn test_log_viewer_scrolling_to_the_end_resumes_the_tail() {
    let mut viewer = bare().tail_mode(true);
    numbered(&mut viewer, 10);
    render(&viewer, 30, 4);
    viewer.scroll_up(2);
    viewer.scroll_down(5);
    assert!(viewer.is_tail_mode());
    viewer.push("l11");
    assert_eq!(messages(&viewer, 30, 4), ["l8", "l9", "l10", "l11"]);
    // Tail mode moves only the view: the selection is still the first entry,
    // selected when it arrived
    assert_eq!(viewer.selected_text().as_deref(), Some("l1"));
}

#[test]
fn test_log_viewer_tail_indicator_only_in_tail_mode() {
    let mut viewer = bare().tail_mode(true);
    viewer.load("a");
    assert!(row_text(&render(&viewer, 30, 2), 0).ends_with("◉ TAIL"));
    viewer.toggle_tail();
    assert!(!row_text(&render(&viewer, 30, 2), 0).contains("TAIL"));
}

#[test]
fn test_log_viewer_scroll_marker_only_when_entries_overflow() {
    let mut viewer = bare();
    viewer.load("a\nb");
    let buffer = render(&viewer, 10, 3);
    assert!((0..3).all(|y| buffer.get(9, y).unwrap().symbol != '█'));

    viewer.load(
        &(0..20)
            .map(|i| format!("line {i}"))
            .collect::<Vec<_>>()
            .join("\n"),
    );
    let buffer = render(&viewer, 10, 3);
    assert_eq!(buffer.get(9, 0).unwrap().symbol, '█', "at the top");
}

#[test]
fn test_log_viewer_entries_from_push_entry_render_their_fields() {
    let mut viewer = LogViewer::new().show_timestamps(false).tail_mode(false);
    viewer.push_entry(
        LogEntry::new("raw", 42)
            .level(LogLevel::Warning)
            .source("net")
            .message("slow reply"),
    );
    let buffer = render(&viewer, 60, 1);
    assert_eq!(cells(&buffer, 0, 0, 5), "   42");
    assert_eq!(cells(&buffer, 8, 0, 1), "⚠");
    assert_eq!(cells(&buffer, 13, 0, 3), "net");
    assert_eq!(cells(&buffer, 25, 0, 10), "slow reply");
}

// =========================================================================
// Matrix
// =========================================================================

fn line() -> impl Strategy<Value = String> {
    prop_oneof![
        "[a-z ]{1,20}".prop_map(|m| format!("INFO {m}")),
        "[a-z]{1,8}".prop_map(|m| format!("2024-01-15 10:30:00 ERROR [db] {m}")),
        "[a-z]{1,8}".prop_map(|m| format!("12:00:01 WARN [svc] {m} 한글")),
        "[a-z]{1,8}".prop_map(|m| format!("{{\"level\":\"debug\",\"msg\":\"{m}\"}}")),
        "[a-z]{1,12}",
    ]
}

fn keys() -> impl Strategy<Value = Vec<Key>> {
    prop::collection::vec(
        prop_oneof![
            Just(Key::Up),
            Just(Key::Down),
            Just(Key::PageUp),
            Just(Key::PageDown),
            Just(Key::Home),
            Just(Key::End),
            Just(Key::Char('f')),
            Just(Key::Char('b')),
            Just(Key::Char('n')),
            Just(Key::Char('N')),
            Just(Key::Char(']')),
            Just(Key::Char('[')),
        ],
        0..15,
    )
}

fn viewer(lines: &[String], mask: u8, min_level: Option<LogLevel>, query: &str) -> LogViewer {
    let mut viewer = LogViewer::new()
        .show_line_numbers(true)
        .show_timestamps(mask & 1 != 0)
        .show_levels(mask & 2 != 0)
        .show_source(mask & 4 != 0)
        .tail_mode(mask & 8 != 0);
    let (first, rest) = lines.split_at(lines.len() / 2);
    viewer.load(&first.join("\n"));
    for line in rest {
        viewer.push(line);
    }
    if let Some(level) = min_level {
        viewer.set_filter(LogFilter::new().min_level(level));
    }
    if !query.is_empty() {
        viewer.search(query);
    }
    viewer
}

fn level() -> impl Strategy<Value = Option<LogLevel>> {
    prop_oneof![
        Just(None),
        Just(Some(LogLevel::Info)),
        Just(Some(LogLevel::Warning)),
        Just(Some(LogLevel::Error)),
    ]
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    /// The rows shown are filtered entries in order (line numbers rise),
    /// at most one row is highlighted and it is the selected entry, and the
    /// export has one line per filtered entry.
    #[test]
    fn rows_are_the_filtered_entries_in_order(
        lines in prop::collection::vec(line(), 0..30),
        mask in 0u8..16,
        min_level in level(),
        query in "[a-z]{0,2}",
        keys in keys(),
        h in 1u16..12,
    ) {
        let mut viewer = viewer(&lines, mask, min_level, &query);
        for key in &keys {
            viewer.handle_key(key);
        }
        let buffer = render(&viewer, 70, h);
        prop_assert_eq!(
            viewer.export_filtered().lines().filter(|l| !l.is_empty()).count(),
            viewer.filtered_len()
        );
        if viewer.filtered_len() == 0 {
            prop_assert!(viewer.selected_entry().is_none());
            return Ok(());
        }

        let numbers: Vec<usize> = (0..h)
            .map(|y| cells(&buffer, 0, y, 5))
            .take_while(|n| !n.trim().is_empty())
            .map(|n| n.trim().parse().unwrap())
            .collect();
        prop_assert!(!numbers.is_empty());
        prop_assert!(numbers.windows(2).all(|p| p[0] < p[1]), "{:?}", numbers);

        let lit = highlighted_rows(&buffer);
        prop_assert!(lit.len() <= 1, "{:?}", lit);
        if let Some(&y) = lit.first() {
            let selected = viewer.selected_entry().unwrap().line_number;
            prop_assert_eq!(numbers[usize::from(y)], selected);
        }
    }

    /// Any entries, options, keys and area: rendering never panics and never
    /// draws outside its area.
    #[test]
    fn render_stays_inside_its_area(
        lines in prop::collection::vec(line(), 0..12),
        mask in 0u8..16,
        keys in keys(),
        w in 0u16..50,
        h in 0u16..8,
    ) {
        let mut viewer = viewer(&lines, mask, None, "a");
        for key in &keys {
            viewer.handle_key(key);
        }
        let sentinel = '#';
        let mut buffer = Buffer::new(w + 4, h + 4);
        for y in 0..h + 4 {
            for x in 0..w + 4 {
                buffer.set(x, y, revue::render::Cell::new(sentinel));
            }
        }
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(2, 2, w, h));
        viewer.render(&mut ctx);
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
/// viewer draws its rows in its own box, given by the stylesheet, and the
/// sibling stays below it.
#[test]
fn test_log_viewer_in_a_stack_with_a_stylesheet() {
    use revue::prelude::*;
    use revue::testing::PipelineHarness;

    struct Screen;
    impl View for Screen {
        fn render(&self, ctx: &mut RenderContext) {
            let mut viewer = LogViewer::new()
                .show_timestamps(false)
                .show_source(false)
                .tail_mode(false)
                .element_id("log");
            viewer.load("INFO first\nERROR second\nWARN third\nINFO fourth");
            vstack()
                .child(viewer)
                .child(Text::new("footer"))
                .render(ctx);
        }
        fn widget_type(&self) -> &'static str {
            "Screen"
        }
    }

    let mut h = PipelineHarness::with_css("#log { height: 3; }", 40, 6).dom_from_render(true);
    h.draw(&Screen);
    let buffer = h.buffer();
    let shown: Vec<String> = (0..6).map(|y| row_text(buffer, y)).collect();
    assert!(shown[0].contains("INF first"), "{shown:#?}");
    assert!(shown[2].contains("WRN third"), "{shown:#?}");
    assert_eq!(shown[3], "footer", "{shown:#?}");
    assert_eq!(buffer.get(0, 0).unwrap().bg, Some(SELECTED_BG));
}
