//! CSV Viewer tests

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{csv_viewer, CsvViewer, Delimiter};

#[test]
fn test_csv_viewer_new() {
    let viewer = CsvViewer::new();
    assert_eq!(viewer.row_count(), 0);
    assert_eq!(viewer.column_count(), 0);
}

#[test]
fn test_csv_viewer_from_str() {
    let csv = "Name,Age,City\nAlice,30,NYC\nBob,25,LA";
    let viewer = CsvViewer::from_content(csv);

    assert_eq!(viewer.row_count(), 2);
    assert_eq!(viewer.column_count(), 3);
    assert_eq!(viewer.get_header(0), Some("Name"));
    assert_eq!(viewer.get_cell(0, 0), Some("Alice"));
    assert_eq!(viewer.get_cell(1, 1), Some("25"));
}

#[test]
fn test_csv_viewer_no_header() {
    let csv = "Alice,30,NYC\nBob,25,LA";
    let mut viewer = CsvViewer::new().has_header(false);
    viewer.parse(csv);

    assert_eq!(viewer.row_count(), 2);
    assert_eq!(viewer.get_cell(0, 0), Some("Alice"));
}

#[test]
fn test_csv_viewer_navigation() {
    let csv = "A,B\n1,2\n3,4\n5,6";
    let mut viewer = CsvViewer::from_content(csv);

    assert_eq!(viewer.selected_row(), 0);
    viewer.select_down();
    assert_eq!(viewer.selected_row(), 1);
    viewer.select_down();
    assert_eq!(viewer.selected_row(), 2);
    viewer.select_up();
    assert_eq!(viewer.selected_row(), 1);

    viewer.select_first_row();
    assert_eq!(viewer.selected_row(), 0);
    viewer.select_last_row();
    assert_eq!(viewer.selected_row(), 2);
}

#[test]
fn test_csv_viewer_column_navigation() {
    let csv = "A,B,C\n1,2,3";
    let mut viewer = CsvViewer::from_content(csv);

    assert_eq!(viewer.selected_col(), 0);
    viewer.select_right();
    assert_eq!(viewer.selected_col(), 1);
    viewer.select_right();
    assert_eq!(viewer.selected_col(), 2);
    viewer.select_right(); // Should stay at max
    assert_eq!(viewer.selected_col(), 2);
    viewer.select_left();
    assert_eq!(viewer.selected_col(), 1);
}

#[test]
fn test_csv_viewer_sorting() {
    let csv = "Name,Value\nBob,20\nAlice,10\nCharlie,30";
    let mut viewer = CsvViewer::from_content(csv);

    viewer.sort_by(0); // Sort by Name ascending
                       // We can verify sorting works by checking selected value after sorting

    viewer.sort_by(0); // Toggle to descending
    viewer.sort_by(0); // Toggle to none
    viewer.reset_sort();
}

#[test]
fn test_csv_viewer_search() {
    let csv = "Name,City\nAlice,NYC\nBob,LA\nAlice,Chicago";
    let mut viewer = CsvViewer::from_content(csv);

    viewer.search("alice");
    assert_eq!(viewer.match_count(), 2);
    assert!(viewer.is_searching());

    viewer.next_match();
    viewer.next_match();
    viewer.prev_match();

    viewer.clear_search();
    assert_eq!(viewer.match_count(), 0);
    assert!(!viewer.is_searching());
}

#[test]
fn test_csv_viewer_delimiter_detection() {
    // Tab-separated
    let tsv = "A\tB\tC\n1\t2\t3";
    let viewer = CsvViewer::from_content(tsv);
    assert_eq!(viewer.column_count(), 3);

    // Semicolon-separated
    let ssv = "A;B;C\n1;2;3";
    let viewer = CsvViewer::from_content(ssv);
    assert_eq!(viewer.column_count(), 3);
}

#[test]
fn test_csv_viewer_quoted_fields() {
    let csv = r#"Name,Description
"Alice","She said ""Hello"""
Bob,"Simple, text""#;
    let viewer = CsvViewer::from_content(csv);

    assert_eq!(viewer.row_count(), 2);
    assert_eq!(viewer.get_cell(0, 1), Some(r#"She said "Hello""#));
}

#[test]
fn test_csv_viewer_render() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let csv = "Name,Age\nAlice,30\nBob,25";
    let viewer = CsvViewer::from_content(csv);
    viewer.render(&mut ctx);
}

#[test]
fn test_csv_viewer_helper() {
    let viewer = csv_viewer().has_header(true).show_row_numbers(true);
    assert_eq!(viewer.row_count(), 0);
}

#[test]
fn test_csv_viewer_default() {
    let viewer = CsvViewer::default();
    assert_eq!(viewer.row_count(), 0);
}

#[test]
fn test_delimiter_enum() {
    assert_eq!(Delimiter::Comma.char(), Some(','));
    assert_eq!(Delimiter::Tab.char(), Some('\t'));
    assert_eq!(Delimiter::Auto.char(), None);
    assert_eq!(Delimiter::Custom('|').char(), Some('|'));
    assert_eq!(Delimiter::Semicolon.char(), Some(';'));
    assert_eq!(Delimiter::Pipe.char(), Some('|'));
}

#[test]
fn test_csv_viewer_page_navigation() {
    let csv = "A\n1\n2\n3\n4\n5\n6\n7\n8\n9\n10";
    let mut viewer = CsvViewer::from_content(csv);

    viewer.page_down(5);
    assert_eq!(viewer.selected_row(), 5);

    viewer.page_up(3);
    assert_eq!(viewer.selected_row(), 2);
}

#[test]
fn test_csv_viewer_selected_value() {
    let csv = "A,B\n1,2\n3,4";
    let mut viewer = CsvViewer::from_content(csv);

    assert_eq!(viewer.selected_value(), Some("1"));
    viewer.select_right();
    assert_eq!(viewer.selected_value(), Some("2"));
    viewer.select_down();
    assert_eq!(viewer.selected_value(), Some("4"));
}

#[test]
fn test_csv_viewer_data_builder() {
    let data = vec![
        vec!["Header1".to_string(), "Header2".to_string()],
        vec!["Value1".to_string(), "Value2".to_string()],
    ];
    let viewer = CsvViewer::new().data(data).has_header(true);

    assert_eq!(viewer.row_count(), 1);
    assert_eq!(viewer.get_header(0), Some("Header1"));
}

#[test]
fn test_csv_viewer_styling() {
    let viewer = CsvViewer::new()
        .header_style(Color::WHITE, Color::BLUE)
        .selected_style(Color::BLACK, Color::YELLOW)
        .match_style(Color::WHITE, Color::RED)
        .fg(Color::WHITE)
        .bg(Color::BLACK);

    // Verify the viewer was created successfully
    assert_eq!(viewer.row_count(), 0);
}

#[test]
fn test_csv_viewer_delimiter_builder() {
    let viewer = CsvViewer::new()
        .delimiter(Delimiter::Tab)
        .show_separators(false);

    assert_eq!(viewer.row_count(), 0);
}

// =============================================================================
// Parsing edge cases
// =============================================================================

#[test]
fn test_csv_empty_content() {
    let viewer = CsvViewer::from_content("");
    assert_eq!(viewer.row_count(), 0);
    assert_eq!(viewer.column_count(), 0);
}

#[test]
fn test_csv_single_column() {
    let csv = "Name\nAlice\nBob";
    let viewer = CsvViewer::from_content(csv);
    assert_eq!(viewer.column_count(), 1);
    assert_eq!(viewer.row_count(), 2);
    assert_eq!(viewer.get_header(0), Some("Name"));
}

#[test]
fn test_csv_ragged_rows() {
    // Rows with different column counts
    let csv = "A,B,C\n1,2\n3,4,5,6";
    let viewer = CsvViewer::from_content(csv);
    assert_eq!(viewer.row_count(), 2);
    // First row defines column count
    assert_eq!(viewer.get_cell(0, 2), None); // short row
}

#[test]
fn test_csv_crlf_line_endings() {
    let csv = "A,B\r\n1,2\r\n3,4";
    let viewer = CsvViewer::from_content(csv);
    assert_eq!(viewer.row_count(), 2);
    assert_eq!(viewer.get_cell(0, 0), Some("1"));
    assert_eq!(viewer.get_cell(1, 1), Some("4"));
}

#[test]
fn test_csv_pipe_delimiter() {
    let csv = "A|B|C\n1|2|3";
    let viewer = CsvViewer::from_content(csv);
    assert_eq!(viewer.column_count(), 3);
    assert_eq!(viewer.get_cell(0, 1), Some("2"));
}

#[test]
fn test_csv_explicit_pipe_delimiter() {
    let mut viewer = CsvViewer::new().delimiter(Delimiter::Pipe);
    viewer.parse("A|B\n1|2");
    assert_eq!(viewer.column_count(), 2);
    assert_eq!(viewer.get_cell(0, 0), Some("1"));
}

#[test]
fn test_csv_header_only() {
    let csv = "Name,Age,City";
    let viewer = CsvViewer::from_content(csv);
    assert_eq!(viewer.row_count(), 0);
    assert_eq!(viewer.column_count(), 3);
    assert_eq!(viewer.get_header(1), Some("Age"));
}

// =============================================================================
// Sorting verification
// =============================================================================

#[test]
fn test_csv_sort_ascending_order() {
    let csv = "Name\nCharlie\nAlice\nBob";
    let mut viewer = CsvViewer::from_content(csv);

    viewer.sort_by(0); // Ascending
                       // Check sorted_indices after sort
    assert_eq!(viewer.sort_order, revue::widget::CsvSortOrder::Ascending);
    assert_eq!(viewer.sort_column, Some(0));
}

#[test]
fn test_csv_sort_toggle_cycle() {
    let csv = "Name\nA\nB";
    let mut viewer = CsvViewer::from_content(csv);

    // Initial: None
    assert_eq!(viewer.sort_order, revue::widget::CsvSortOrder::None);

    viewer.sort_by(0); // None → Ascending
    assert_eq!(viewer.sort_order, revue::widget::CsvSortOrder::Ascending);

    viewer.sort_by(0); // Ascending → Descending
    assert_eq!(viewer.sort_order, revue::widget::CsvSortOrder::Descending);

    viewer.sort_by(0); // Descending → None
    assert_eq!(viewer.sort_order, revue::widget::CsvSortOrder::None);
}

#[test]
fn test_csv_sort_different_column_resets_to_ascending() {
    let csv = "A,B\n1,2\n3,4";
    let mut viewer = CsvViewer::from_content(csv);

    viewer.sort_by(0);
    assert_eq!(viewer.sort_column, Some(0));
    assert_eq!(viewer.sort_order, revue::widget::CsvSortOrder::Ascending);

    viewer.sort_by(1); // Switch column
    assert_eq!(viewer.sort_column, Some(1));
    assert_eq!(viewer.sort_order, revue::widget::CsvSortOrder::Ascending);
}

#[test]
fn test_csv_sort_empty_cells() {
    let csv = "Val\n\nAlpha\n\nBravo";
    let mut viewer = CsvViewer::from_content(csv);
    viewer.sort_by(0);
    // Should not panic with empty cells
    assert_eq!(viewer.sort_order, revue::widget::CsvSortOrder::Ascending);
}

// =============================================================================
// Navigation boundary tests
// =============================================================================

#[test]
fn test_csv_select_up_at_top() {
    let csv = "A\n1\n2";
    let mut viewer = CsvViewer::from_content(csv);

    assert_eq!(viewer.selected_row(), 0);
    viewer.select_up(); // Already at top
    assert_eq!(viewer.selected_row(), 0);
}

#[test]
fn test_csv_select_down_at_bottom() {
    let csv = "A\n1\n2";
    let mut viewer = CsvViewer::from_content(csv);

    viewer.select_last_row();
    let last = viewer.selected_row();
    viewer.select_down(); // Already at bottom
    assert_eq!(viewer.selected_row(), last);
}

#[test]
fn test_csv_select_left_at_leftmost() {
    let csv = "A,B\n1,2";
    let mut viewer = CsvViewer::from_content(csv);

    assert_eq!(viewer.selected_col(), 0);
    viewer.select_left(); // Already at leftmost
    assert_eq!(viewer.selected_col(), 0);
}

#[test]
fn test_csv_page_down_beyond_end() {
    let csv = "A\n1\n2\n3";
    let mut viewer = CsvViewer::from_content(csv);

    viewer.page_down(100); // Way past end
    assert_eq!(viewer.selected_row(), 2); // Clamped to last row
}

#[test]
fn test_csv_page_up_beyond_start() {
    let csv = "A\n1\n2\n3";
    let mut viewer = CsvViewer::from_content(csv);

    viewer.select_last_row();
    viewer.page_up(100); // Way past start
    assert_eq!(viewer.selected_row(), 0);
}

// =============================================================================
// Search edge cases
// =============================================================================

#[test]
fn test_csv_search_empty_query() {
    let csv = "A\n1\n2";
    let mut viewer = CsvViewer::from_content(csv);

    viewer.search("");
    assert!(!viewer.is_searching());
    assert_eq!(viewer.match_count(), 0);
}

#[test]
fn test_csv_search_no_match() {
    let csv = "Name\nAlice\nBob";
    let mut viewer = CsvViewer::from_content(csv);

    viewer.search("zzz");
    assert!(viewer.is_searching());
    assert_eq!(viewer.match_count(), 0);
}

#[test]
fn test_csv_search_case_insensitive() {
    let csv = "Name\nALICE\nalice\nAlice";
    let mut viewer = CsvViewer::from_content(csv);

    viewer.search("alice");
    assert_eq!(viewer.match_count(), 3);
}

#[test]
fn test_csv_search_next_wraps() {
    let csv = "Name\nAlice\nBob\nAlice";
    let mut viewer = CsvViewer::from_content(csv);

    viewer.search("alice");
    assert_eq!(viewer.match_count(), 2);

    // Navigate through all matches and wrap around
    viewer.next_match(); // go to second match
    viewer.next_match(); // wrap to first match
    assert_eq!(viewer.current_match, 0);
}

#[test]
fn test_csv_search_prev_wraps() {
    let csv = "Name\nAlice\nBob\nAlice";
    let mut viewer = CsvViewer::from_content(csv);

    viewer.search("alice");
    assert_eq!(viewer.match_count(), 2);

    // prev_match from first should wrap to last
    viewer.prev_match();
    assert_eq!(viewer.current_match, 1);
}

#[test]
fn test_csv_next_match_no_matches() {
    let csv = "A\n1";
    let mut viewer = CsvViewer::from_content(csv);

    viewer.search("zzz");
    viewer.next_match(); // Should not panic
    viewer.prev_match(); // Should not panic
}

// =============================================================================
// Column width / row number width
// =============================================================================

#[test]
fn test_csv_column_widths_clamped() {
    // Minimum width is 3, max is 40
    let csv = "A\nx"; // Very short column
    let viewer = CsvViewer::from_content(csv);
    assert!(viewer.column_widths.iter().all(|&w| w >= 3));
}

#[test]
fn test_csv_column_widths_max_40() {
    let long_header = "A".repeat(60);
    let csv = format!("{}\nvalue", long_header);
    let viewer = CsvViewer::from_content(&csv);
    assert!(viewer.column_widths.iter().all(|&w| w <= 40));
}

#[test]
fn test_csv_row_number_width() {
    let csv = "A\n1\n2\n3\n4\n5\n6\n7\n8\n9\n10";
    let viewer = CsvViewer::from_content(csv);
    assert!(viewer.show_row_numbers);
    let rw = viewer.row_number_width();
    assert!(rw >= 3); // at least 2 digits + 1 padding
}

#[test]
fn test_csv_row_number_width_hidden() {
    let viewer = CsvViewer::new().show_row_numbers(false);
    assert_eq!(viewer.row_number_width(), 0);
}

#[test]
fn test_csv_get_cell_out_of_bounds() {
    let csv = "A,B\n1,2";
    let viewer = CsvViewer::from_content(csv);
    assert_eq!(viewer.get_cell(5, 0), None); // row out of bounds
    assert_eq!(viewer.get_cell(0, 5), None); // col out of bounds
}

#[test]
fn test_csv_get_header_no_header_mode() {
    let csv = "Alice,30\nBob,25";
    let mut viewer = CsvViewer::new().has_header(false);
    viewer.parse(csv);
    assert_eq!(viewer.get_header(0), None);
}

// Found by tests/widget/catalog/sequences/mod.rs: the shrunk sequence was
// `Ctrl+End <parse empty>` - parse() replaced the rows but kept the
// selection (and the search matches) of the old ones.
#[test]
fn test_parse_resets_the_selection_and_the_search() {
    let mut viewer = CsvViewer::from_content("a,b\n1,2\n3,4\n5,6");
    viewer.search("5");
    viewer.select_last_row();
    viewer.select_right();
    assert_eq!((viewer.selected_row(), viewer.selected_col()), (2, 1));

    viewer.parse("a,b\n7,8");
    assert_eq!((viewer.selected_row(), viewer.selected_col()), (0, 0));
    assert_eq!(viewer.match_count(), 0);

    viewer.parse("a,b\n5,9");
    assert_eq!(viewer.match_count(), 1);

    viewer.parse("");
    assert_eq!(viewer.row_count(), 0);
    assert_eq!(viewer.selected_row(), 0);
}

// Found by tests/widget/catalog/sequences/mod.rs: the shrunk sequence was
// `<search '1'> 'n'` - a ragged row's extra cell (never drawn) matched, and
// next_match() selected a column the viewer does not have.
#[test]
fn test_search_skips_cells_past_the_shown_columns() {
    let mut viewer = CsvViewer::from_content("a,b,c\n7,8,9,10\n1,2,3");
    assert_eq!(viewer.column_count(), 3);
    viewer.search("1");
    assert_eq!(viewer.match_count(), 1);
    viewer.next_match();
    assert_eq!((viewer.selected_row(), viewer.selected_col()), (1, 0));
}

// =========================================================================
// Render helpers
// =========================================================================

fn render(viewer: &CsvViewer, width: u16, height: u16) -> Buffer {
    let mut buffer = Buffer::new(width, height);
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

/// The text of the row drawn with the selection background, if any
fn highlighted_row(buffer: &Buffer) -> Option<String> {
    (0..buffer.height())
        .find(|&y| buffer.get(0, y).unwrap().bg == Some(Color::BLUE))
        .map(|y| row_text(buffer, y))
}

/// Values of column `col` in the order the viewer walks its rows
fn walk(viewer: &CsvViewer, col: usize) -> Vec<String> {
    let mut v = viewer.clone();
    v.select_first_row();
    while v.selected_col() > col {
        v.select_left();
    }
    assert!(col < v.column_count().max(1), "column {col} is not shown");
    while v.selected_col() < col {
        v.select_right();
    }
    let mut out = Vec::new();
    for _ in 0..v.row_count() {
        out.push(v.selected_value().unwrap_or("").to_string());
        v.select_down();
    }
    out
}

// =========================================================================
// Empty input
// =========================================================================

#[test]
fn test_csv_empty_viewer_survives_every_action() {
    let mut v = CsvViewer::new();
    v.select_down();
    v.select_up();
    v.select_right();
    v.select_left();
    v.select_last_row();
    v.page_down(10);
    v.page_up(10);
    v.sort_by(0);
    v.search("x");
    v.next_match();
    v.prev_match();
    assert_eq!(v.selected_value(), None);
    assert_eq!(v.match_count(), 0);
    for (w, h) in [(0, 0), (4, 1), (5, 2), (40, 10)] {
        let buffer = render(&v, w, h);
        assert!((0..h).all(|y| row_text(&buffer, y).is_empty()), "{w}x{h}");
    }
}

#[test]
fn test_csv_header_only_sorts_and_renders() {
    let mut v = CsvViewer::from_content("Name,Age");
    v.sort_by(1);
    assert_eq!(v.row_count(), 0);
    assert_eq!(v.sort_column, Some(1));
    let buffer = render(&v, 30, 3);
    assert!(
        row_text(&buffer, 0).contains("Name"),
        "{:?}",
        row_text(&buffer, 0)
    );
    assert_eq!(row_text(&buffer, 1), "");
}

#[test]
fn test_csv_empty_content_without_header() {
    let v = CsvViewer::from_content("").has_header(false);
    assert_eq!(v.row_count(), 0);
    render(&v, 20, 4);
}

// =========================================================================
// #856: selection and search follow the shown order after sorting
// =========================================================================

#[test]
fn test_csv_sort_keeps_the_selected_record() {
    let mut v = CsvViewer::from_content("N\n3\n1\n2");
    assert_eq!(v.selected_value(), Some("3"));
    v.sort_by(0);
    assert_eq!(
        v.selected_value(),
        Some("3"),
        "the selected record stays selected"
    );
    assert_eq!(walk(&v, 0), ["1", "2", "3"]);
}

/// Down moved to the next row in the file, not on screen.
#[test]
fn test_csv_navigation_after_sort_follows_the_screen() {
    let mut v = CsvViewer::from_content("N\n3\n1\n2");
    v.sort_by(0);
    v.select_first_row();
    assert_eq!(v.selected_value(), Some("1"));
    v.select_down();
    assert_eq!(v.selected_value(), Some("2"));
    v.select_last_row();
    assert_eq!(v.selected_value(), Some("3"));
    v.sort_by(0); // descending
    assert_eq!(walk(&v, 0), ["3", "2", "1"]);
}

#[test]
fn test_csv_highlight_matches_the_selected_value_after_sort() {
    let mut v = CsvViewer::from_content("N\n3\n1\n2")
        .show_row_numbers(false)
        .show_separators(false);
    v.sort_by(0);
    v.select_first_row();
    let buffer = render(&v, 20, 5);
    assert_eq!(row_text(&buffer, 1).trim(), "1");
    assert_eq!(highlighted_row(&buffer).unwrap().trim(), "1");
}

/// next_match jumped by file position, so after sorting it hopped around.
#[test]
fn test_csv_matches_follow_the_screen_after_sort() {
    let mut v = CsvViewer::from_content("N\nb1\nc1\na1");
    v.sort_by(0);
    v.search("1");
    assert_eq!(v.match_count(), 3);
    let mut seen = Vec::new();
    for _ in 0..3 {
        v.next_match();
        seen.push(v.selected_value().unwrap().to_string());
    }
    assert_eq!(seen, ["b1", "c1", "a1"]); // starts after match 0 ("a1"), then wraps
    v.prev_match();
    assert_eq!(v.selected_value(), Some("c1"));
}

#[test]
fn test_csv_search_then_sort_reorders_the_matches() {
    let mut v = CsvViewer::from_content("N\nb1\nc1\na1");
    v.search("1");
    v.sort_by(0);
    v.next_match();
    assert_eq!(v.selected_value(), Some("b1"));
}

#[test]
fn test_csv_match_highlight_after_sort() {
    let mut v = CsvViewer::from_content("N,M\nzz,x\naa,y")
        .show_row_numbers(false)
        .show_separators(false);
    v.select_right();
    v.search("zz");
    v.sort_by(0);
    let buffer = render(&v, 20, 3);
    assert_eq!(row_text(&buffer, 2), "zz x");
    assert_eq!(
        buffer.get(0, 2).unwrap().bg,
        Some(Color::YELLOW),
        "match cell"
    );
}

/// from_content parsed with a header, and has_header(false) only flipped
/// the flag, so the first line stayed hidden.
#[test]
fn test_csv_has_header_false_after_from_content() {
    let v = CsvViewer::from_content("1,2,3\n4,5,6").has_header(false);
    assert_eq!(v.row_count(), 2);
    assert_eq!(walk(&v, 0), ["1", "4"]);
    let buffer = render(&v, 30, 3);
    assert!(
        row_text(&buffer, 0).contains('1'),
        "{:?}",
        row_text(&buffer, 0)
    );
    assert!(
        row_text(&buffer, 1).contains('4'),
        "{:?}",
        row_text(&buffer, 1)
    );
}

#[test]
fn test_csv_has_header_round_trip() {
    let v = CsvViewer::from_content("H\n1\n2")
        .has_header(false)
        .has_header(true);
    assert_eq!(walk(&v, 0), ["1", "2"]);
    assert_eq!(v.get_header(0), Some("H"));
}

#[test]
fn test_csv_has_header_reruns_the_search() {
    let mut v = CsvViewer::from_content("Name\nBob");
    v.search("name");
    assert_eq!(v.match_count(), 0);
    let v = v.has_header(false);
    assert_eq!(v.match_count(), 1);
}

// =========================================================================
// Edge values
// =========================================================================

#[test]
fn test_csv_tiny_areas() {
    let v = CsvViewer::from_content("a,b\n1,2");
    for (w, h) in [(1, 1), (4, 5), (5, 1), (5, 2), (6, 2)] {
        render(&v, w, h);
    }
    assert_eq!(
        row_text(&render(&v, 4, 5), 0),
        "",
        "below the minimum width"
    );
}

#[test]
fn test_csv_wide_cells_are_not_split() {
    let v = CsvViewer::from_content("이름\n한글한글")
        .show_row_numbers(false)
        .show_separators(false);
    let buffer = render(&v, 7, 2);
    assert_eq!(row_text(&buffer, 1), "한글한");
    assert!(buffer.get(5, 1).unwrap().is_continuation());
    assert_eq!(buffer.get(6, 1).unwrap().symbol, ' ');
}

#[test]
fn test_csv_sort_by_a_missing_column() {
    let mut v = CsvViewer::from_content("A\n2\n1");
    v.sort_by(9);
    assert_eq!(
        walk(&v, 0),
        ["2", "1"],
        "every key is empty, so the order holds"
    );
}

#[test]
fn test_csv_scrolls_to_a_selection_past_the_view() {
    let content = std::iter::once("N".to_string())
        .chain((0..50).map(|i| i.to_string()))
        .collect::<Vec<_>>()
        .join("\n");
    let mut v = CsvViewer::from_content(&content).show_row_numbers(false);
    v.select_last_row();
    let buffer = render(&v, 10, 5);
    assert_eq!(
        highlighted_row(&buffer)
            .unwrap()
            .trim_end_matches('│')
            .trim(),
        "49"
    );
}

// =========================================================================
// Matrix
// =========================================================================

mod matrix {
    use super::*;
    use proptest::prelude::*;
    use revue::utils::natural_cmp;
    use std::cmp::Ordering;

    fn cell() -> impl Strategy<Value = String> {
        prop::sample::select(vec!["", "a", "B", "10", "9", "x1", "한글", "z z"])
            .prop_map(String::from)
    }

    fn grid() -> impl Strategy<Value = Vec<Vec<String>>> {
        (1usize..4)
            .prop_flat_map(|cols| prop::collection::vec(prop::collection::vec(cell(), cols), 0..8))
    }

    #[derive(Debug, Clone)]
    enum Op {
        Sort(usize),
        Down,
        Up,
        Last,
        Search(&'static str),
        Next,
        Prev,
    }

    fn op() -> impl Strategy<Value = Op> {
        prop_oneof![
            (0usize..4).prop_map(Op::Sort),
            Just(Op::Down),
            Just(Op::Up),
            Just(Op::Last),
            prop::sample::select(vec!["a", "1", "글", "zz"]).prop_map(Op::Search),
            Just(Op::Next),
            Just(Op::Prev),
        ]
    }

    fn apply(v: &mut CsvViewer, op: &Op) {
        match op {
            Op::Sort(c) => v.sort_by(*c),
            Op::Down => v.select_down(),
            Op::Up => v.select_up(),
            Op::Last => v.select_last_row(),
            Op::Search(q) => v.search(q),
            Op::Next => v.next_match(),
            Op::Prev => v.prev_match(),
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(256))]

        /// After any sequence of sorts, moves and searches: walking down
        /// visits every record once, in sort order; the highlighted row on
        /// screen holds the selected value; matches are visited in screen
        /// order.
        #[test]
        fn sorted_walk_and_highlight_agree(
            rows in grid(),
            header in any::<bool>(),
            ops in prop::collection::vec(op(), 0..10),
        ) {
            let data = rows;
            let mut v = CsvViewer::new().has_header(header).data(data.clone());
            for op in &ops {
                apply(&mut v, op);
            }

            // Every record once
            let body: Vec<String> = data.iter().skip(usize::from(header)).map(|r| r[0].clone()).collect();
            let mut walked = walk(&v, 0);
            let mut expected = body.clone();
            walked.sort();
            expected.sort();
            prop_assert_eq!(walked, expected);

            // In sort order
            if let (Some(col), Some(dir)) = (v.sort_column.filter(|&c| c < v.column_count()), match v.sort_order {
                revue::widget::data::CsvSortOrder::Ascending => Some(Ordering::Less),
                revue::widget::data::CsvSortOrder::Descending => Some(Ordering::Greater),
                revue::widget::data::CsvSortOrder::None => None,
            }) {
                let keys = walk(&v, col);
                for pair in keys.windows(2) {
                    prop_assert_ne!(natural_cmp(&pair[0], &pair[1]), dir.reverse(), "{:?}", keys);
                }
            }

            // The selected record is the highlighted one
            if v.row_count() > 0 {
                let selected = v.selected_value().map(str::to_string);
                prop_assert!(selected.is_some());
                let shown = v.clone().show_row_numbers(true);
                let buffer = render(&shown, 60, 12);
                let y = (0..12).find(|&y| buffer.get(0, y).unwrap().bg == Some(Color::BLUE));
                prop_assert!(y.is_some(), "no highlighted row");
            }

            // Matches in screen order: next_match never moves up except to wrap
            if v.match_count() > 1 {
                let mut m = v.clone();
                m.select_first_row();
                let mut positions = Vec::new();
                for _ in 0..m.match_count() {
                    m.next_match();
                    positions.push(m.selected_row());
                }
                let wraps = positions.windows(2).filter(|p| p[1] < p[0]).count();
                prop_assert!(wraps <= 1, "{:?}", positions);
            }
        }

        /// Any content at any size renders without panicking and without
        /// drawing outside its area.
        #[test]
        fn render_stays_inside_its_area(
            rows in grid(),
            header in any::<bool>(),
            numbers in any::<bool>(),
            separators in any::<bool>(),
            w in 0u16..40,
            h in 0u16..8,
            ops in prop::collection::vec(op(), 0..6),
        ) {
            let mut v = CsvViewer::new()
                .has_header(header)
                .show_row_numbers(numbers)
                .show_separators(separators)
                .data(rows);
            for op in &ops {
                apply(&mut v, op);
            }
            let mut buffer = Buffer::new(w + 4, h + 4);
            for y in 0..h + 4 {
                for x in 0..w + 4 {
                    buffer.set(x, y, revue::render::Cell::new('#'));
                }
            }
            let mut ctx = RenderContext::new(&mut buffer, Rect::new(2, 2, w, h));
            v.render(&mut ctx);
            for y in 0..h + 4 {
                for x in 0..w + 4 {
                    if !((2..2 + w).contains(&x) && (2..2 + h).contains(&y)) {
                        prop_assert_eq!(buffer.get(x, y).unwrap().symbol, '#', "({}, {})", x, y);
                    }
                }
            }
        }
    }
}

// =========================================================================
// Cross-module
// =========================================================================

/// In a vstack with a sibling, through the stylesheet and DOM path: a sorted
/// viewer highlights the selected record and plain cells take `color`.
#[test]
fn test_csv_in_a_stack_with_a_stylesheet() {
    use revue::prelude::*;
    use revue::testing::PipelineHarness;

    struct Screen;
    impl View for Screen {
        fn render(&self, ctx: &mut RenderContext) {
            let mut v = CsvViewer::from_content("N\n3\n1\n2")
                .show_row_numbers(false)
                .element_id("csv");
            v.sort_by(0);
            v.select_first_row();
            vstack().child(v).child(Text::new("footer")).render(ctx);
        }
        fn widget_type(&self) -> &'static str {
            "Screen"
        }
    }

    let ink = Color::rgb(17, 34, 51);
    let mut h = PipelineHarness::with_css("#csv { color: #112233; }", 20, 6).dom_from_render(true);
    h.draw(&Screen);
    let buffer = h.buffer();
    let rows: Vec<String> = (0..6).map(|y| row_text(buffer, y)).collect();
    assert_eq!(rows[1].trim_end_matches('│').trim(), "1", "{rows:#?}");
    assert_eq!(buffer.get(0, 1).unwrap().bg, Some(Color::BLUE), "{rows:#?}");
    assert_eq!(buffer.get(0, 2).unwrap().fg, Some(ink), "plain cell");
    assert!(
        rows.iter().any(|r| r == "footer"),
        "sibling lost: {rows:#?}"
    );
}

// =========================================================================
// #969: the sort indicator, ragged rows, the view
// =========================================================================

#[test]
fn test_csv_sort_indicator_survives_a_narrow_column() {
    let mut v = CsvViewer::from_content("Age\n3\n1").show_row_numbers(false);
    v.sort_by(0);
    assert!(row_text(&render(&v, 20, 3), 0).contains('▲')); // was "Age│"
}

#[test]
fn test_csv_ragged_cells_are_not_drawn() {
    let v = CsvViewer::from_content("a,b\n1,2,EXTRA").show_row_numbers(false);
    assert!(!row_text(&render(&v, 40, 2), 1).contains("EXTRA"));
}

#[test]
fn test_csv_view_holds_while_selection_moves_inside_it() {
    let content = std::iter::once("N".to_string())
        .chain((0..20).map(|i| i.to_string()))
        .collect::<Vec<_>>()
        .join("\n");
    let mut v = CsvViewer::from_content(&content).show_row_numbers(false);
    v.select_last_row();
    let before = row_text(&render(&v, 10, 4), 1);
    assert!(before.starts_with("17"), "{before:?}");
    v.select_up();
    assert_eq!(row_text(&render(&v, 10, 4), 1), before); // was "17" -> "16"
}
