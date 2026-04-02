//! CSV Viewer widget for displaying tabular CSV data
//!
//! Features:
//! - Auto-detect delimiters (comma, tab, semicolon, pipe)
//! - Header row detection
//! - Column sorting (ascending/descending)
//! - Search across all cells
//! - Virtual scrolling for large files
//! - Column width auto-sizing
//! - Row numbering

mod core;
mod helpers;
mod types;
mod view;

pub use core::CsvViewer;
pub use helpers::csv_viewer;
pub use types::{Delimiter, SortOrder};

crate::impl_styled_view!(CsvViewer);
crate::impl_props_builders!(CsvViewer);

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_csv() -> &'static str {
        "Name,Age,City\nAlice,30,Seoul\nBob,25,Tokyo\nCharlie,35,Beijing"
    }

    #[test]
    fn test_csv_viewer_from_content() {
        let v = CsvViewer::from_content(sample_csv());
        assert_eq!(v.row_count(), 3); // excludes header
        assert_eq!(v.column_count(), 3);
    }

    #[test]
    fn test_csv_viewer_get_header() {
        let v = CsvViewer::from_content(sample_csv());
        assert_eq!(v.get_header(0), Some("Name"));
        assert_eq!(v.get_header(1), Some("Age"));
        assert_eq!(v.get_header(2), Some("City"));
        assert_eq!(v.get_header(99), None);
    }

    #[test]
    fn test_csv_viewer_get_cell() {
        let v = CsvViewer::from_content(sample_csv());
        assert_eq!(v.get_cell(0, 0), Some("Alice"));
        assert_eq!(v.get_cell(1, 1), Some("25"));
        assert_eq!(v.get_cell(2, 2), Some("Beijing"));
    }

    #[test]
    fn test_csv_viewer_navigation() {
        let mut v = CsvViewer::from_content(sample_csv());
        assert_eq!(v.selected_row(), 0);
        assert_eq!(v.selected_col(), 0);

        v.select_down();
        assert_eq!(v.selected_row(), 1);

        v.select_right();
        assert_eq!(v.selected_col(), 1);

        v.select_up();
        assert_eq!(v.selected_row(), 0);

        v.select_left();
        assert_eq!(v.selected_col(), 0);
    }

    #[test]
    fn test_csv_viewer_navigation_bounds() {
        let mut v = CsvViewer::from_content(sample_csv());
        v.select_up(); // Can't go above 0
        assert_eq!(v.selected_row(), 0);

        v.select_left(); // Can't go left of 0
        assert_eq!(v.selected_col(), 0);

        v.select_last_row();
        assert_eq!(v.selected_row(), 2);

        v.select_down(); // Can't go past last
        assert_eq!(v.selected_row(), 2);
    }

    #[test]
    fn test_csv_viewer_first_last_row() {
        let mut v = CsvViewer::from_content(sample_csv());
        v.select_last_row();
        assert_eq!(v.selected_row(), 2);

        v.select_first_row();
        assert_eq!(v.selected_row(), 0);
    }

    #[test]
    fn test_csv_viewer_page_navigation() {
        let mut v = CsvViewer::from_content(sample_csv());
        v.page_down(2);
        assert_eq!(v.selected_row(), 2);

        v.page_up(2);
        assert_eq!(v.selected_row(), 0);
    }

    #[test]
    fn test_csv_viewer_sort() {
        let mut v = CsvViewer::from_content(sample_csv());
        v.sort_by(1); // Sort by Age ascending
        assert_eq!(v.sort_order, SortOrder::Ascending);
        assert_eq!(v.sort_column, Some(1));

        v.sort_by(1); // Toggle to descending
        assert_eq!(v.sort_order, SortOrder::Descending);

        v.sort_by(1); // Toggle to none
        assert_eq!(v.sort_order, SortOrder::None);
    }

    #[test]
    fn test_csv_viewer_search() {
        let mut v = CsvViewer::from_content(sample_csv());
        v.search("bob");
        assert_eq!(v.match_count(), 1);
        assert!(v.is_searching());

        v.clear_search();
        assert_eq!(v.match_count(), 0);
        assert!(!v.is_searching());
    }

    #[test]
    fn test_csv_viewer_search_multiple_matches() {
        let csv = "Name,City\nAlice,Seoul\nBob,Seoul\nCharlie,Beijing";
        let mut v = CsvViewer::from_content(csv);
        v.search("seoul");
        assert_eq!(v.match_count(), 2);

        v.next_match();
        assert_eq!(v.selected_row(), 1); // Second match
    }

    #[test]
    fn test_csv_viewer_selected_value() {
        let v = CsvViewer::from_content(sample_csv());
        assert_eq!(v.selected_value(), Some("Alice"));
    }

    #[test]
    fn test_csv_viewer_empty() {
        let v = CsvViewer::new();
        assert_eq!(v.row_count(), 0);
        assert_eq!(v.column_count(), 0);
        assert_eq!(v.selected_value(), None);
    }

    #[test]
    fn test_csv_viewer_no_header() {
        let v = CsvViewer::from_content("1,2,3\n4,5,6").has_header(false);
        assert_eq!(v.row_count(), 2);
        assert_eq!(v.get_header(0), None);
    }

    #[test]
    fn test_csv_viewer_delimiter_types() {
        assert_eq!(Delimiter::Comma.char(), Some(','));
        assert_eq!(Delimiter::Tab.char(), Some('\t'));
        assert_eq!(Delimiter::Semicolon.char(), Some(';'));
        assert_eq!(Delimiter::Pipe.char(), Some('|'));
        assert_eq!(Delimiter::Auto.char(), None);
        assert_eq!(Delimiter::Custom('#').char(), Some('#'));
    }
}
