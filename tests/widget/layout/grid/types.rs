//! Grid widget types tests

use super::super::grid::{TrackSize, GridPlacement, GridItem, GridAlign};
use crate::widget::Text;

// =========================================================================
// TrackSize enum tests
// =========================================================================

#[test]
fn test_track_size_fixed() {
    let track = TrackSize::Fixed(10);
    assert_eq!(track, TrackSize::Fixed(10));
    assert_ne!(track, TrackSize::Fixed(20));
}

#[test]
fn test_track_size_fr() {
    let track = TrackSize::Fr(2.0);
    assert_eq!(track, TrackSize::Fr(2.0));
    assert_ne!(track, TrackSize::Fr(1.0));
}

#[test]
fn test_track_size_auto() {
    let track = TrackSize::Auto;
    assert_eq!(track, TrackSize::Auto);
    assert_ne!(track, TrackSize::Fixed(10));
}

#[test]
fn test_track_size_min_content() {
    let track = TrackSize::MinContent;
    assert_eq!(track, TrackSize::MinContent);
}

#[test]
fn test_track_size_max_content() {
    let track = TrackSize::MaxContent;
    assert_eq!(track, TrackSize::MaxContent);
}

#[test]
fn test_track_size_percent() {
    let track = TrackSize::Percent(50.0);
    assert_eq!(track, TrackSize::Percent(50.0));
    assert_ne!(track, TrackSize::Percent(75.0));
}

#[test]
fn test_track_size_default() {
    let track = TrackSize::default();
    assert_eq!(track, TrackSize::Fr(1.0));
}

#[test]
fn test_track_size_clone() {
    let track1 = TrackSize::Fr(2.5);
    let track2 = track1.clone();
    assert_eq!(track1, track2);
}

#[test]
fn test_track_size_copy() {
    let track1 = TrackSize::Fixed(10);
    let track2 = track1;
    assert_eq!(track1, TrackSize::Fixed(10));
    assert_eq!(track2, TrackSize::Fixed(10));
}

#[test]
fn test_track_size_partial_eq() {
    assert_eq!(TrackSize::Auto, TrackSize::Auto);
    assert_eq!(TrackSize::Fixed(10), TrackSize::Fixed(10));
    assert_eq!(TrackSize::Fr(1.0), TrackSize::Fr(1.0));
    assert_ne!(TrackSize::Auto, TrackSize::Fixed(10));
    assert_ne!(TrackSize::Fixed(10), TrackSize::Fixed(20));
}

// =========================================================================
// GridPlacement struct tests
// =========================================================================

#[test]
fn test_grid_placement_default() {
    let placement = GridPlacement::default();
    assert_eq!(placement.col_start, 0);
    assert_eq!(placement.col_end, 0);
    assert_eq!(placement.row_start, 0);
    assert_eq!(placement.row_end, 0);
}

#[test]
fn test_grid_placement_cell() {
    let placement = GridPlacement::cell(2, 3);
    assert_eq!(placement.col_start, 2);
    assert_eq!(placement.col_end, 3);
    assert_eq!(placement.row_start, 3);
    assert_eq!(placement.row_end, 4);
}

#[test]
fn test_grid_placement_col_span() {
    let placement = GridPlacement::col_span(1, 3);
    assert_eq!(placement.col_start, 1);
    assert_eq!(placement.col_end, 4);
    assert_eq!(placement.row_start, 0);
    assert_eq!(placement.row_end, 0);
}

#[test]
fn test_grid_placement_row_span() {
    let placement = GridPlacement::row_span(2, 4);
    assert_eq!(placement.col_start, 0);
    assert_eq!(placement.col_end, 0);
    assert_eq!(placement.row_start, 2);
    assert_eq!(placement.row_end, 6);
}

#[test]
fn test_grid_placement_area() {
    let placement = GridPlacement::area(1, 2, 3, 4);
    assert_eq!(placement.col_start, 1);
    assert_eq!(placement.col_end, 4); // 1 + 3
    assert_eq!(placement.row_start, 2);
    assert_eq!(placement.row_end, 6); // 2 + 4
}

#[test]
fn test_grid_placement_span_cols() {
    let placement = GridPlacement::cell(2, 3).span_cols(5);
    assert_eq!(placement.col_start, 2);
    assert_eq!(placement.col_end, 7);
    assert_eq!(placement.row_start, 3);
    assert_eq!(placement.row_end, 4);
}

#[test]
fn test_grid_placement_span_cols_no_start() {
    let placement = GridPlacement {
        col_start: 0,
        col_end: 0,
        row_start: 1,
        row_end: 2,
    };
    let result = placement.span_cols(3);
    // Should not modify if col_start is 0
    assert_eq!(result.col_start, 0);
    assert_eq!(result.col_end, 0);
}

#[test]
fn test_grid_placement_span_rows() {
    let placement = GridPlacement::cell(2, 3).span_rows(2);
    assert_eq!(placement.col_start, 2);
    assert_eq!(placement.col_end, 3);
    assert_eq!(placement.row_start, 3);
    assert_eq!(placement.row_end, 5);
}

#[test]
fn test_grid_placement_span_rows_no_start() {
    let placement = GridPlacement {
        col_start: 1,
        col_end: 2,
        row_start: 0,
        row_end: 0,
    };
    let result = placement.span_rows(3);
    // Should not modify if row_start is 0
    assert_eq!(result.row_start, 0);
    assert_eq!(result.row_end, 0);
}

#[test]
fn test_grid_placement_clone() {
    let placement1 = GridPlacement::cell(2, 3);
    let placement2 = placement1.clone();
    assert_eq!(placement1.col_start, placement2.col_start);
    assert_eq!(placement1.col_end, placement2.col_end);
    assert_eq!(placement1.row_start, placement2.row_start);
    assert_eq!(placement1.row_end, placement2.row_end);
}

#[test]
fn test_grid_placement_copy() {
    let placement1 = GridPlacement::cell(1, 1);
    let placement2 = placement1;
    assert_eq!(placement1, placement2);
}

#[test]
fn test_grid_placement_debug() {
    let placement = GridPlacement::cell(2, 3);
    let debug_str = format!("{:?}", placement);
    assert!(debug_str.contains("GridPlacement"));
}

// =========================================================================
// GridItem struct tests
// =========================================================================

#[test]
fn test_grid_item_new() {
    let item = GridItem::new(Text::new("Test"));
    assert_eq!(item.placement.col_start, 0);
    assert_eq!(item.placement.col_end, 0);
    assert_eq!(item.placement.row_start, 0);
    assert_eq!(item.placement.row_end, 0);
}

#[test]
fn test_grid_item_place() {
    let placement = GridPlacement::cell(2, 3);
    let item = GridItem::new(Text::new("Test")).place(placement);
    assert_eq!(item.placement.col_start, 2);
    assert_eq!(item.placement.col_end, 3);
    assert_eq!(item.placement.row_start, 3);
    assert_eq!(item.placement.row_end, 4);
}

#[test]
fn test_grid_item_at() {
    let item = GridItem::new(Text::new("Test")).at(2, 3);
    assert_eq!(item.placement.col_start, 2);
    assert_eq!(item.placement.col_end, 3);
    assert_eq!(item.placement.row_start, 3);
    assert_eq!(item.placement.row_end, 4);
}

#[test]
fn test_grid_item_col_span() {
    let item = GridItem::new(Text::new("Test")).at(1, 1).col_span(3);
    assert_eq!(item.placement.col_start, 1);
    assert_eq!(item.placement.col_end, 4);
    assert_eq!(item.placement.row_start, 1);
    assert_eq!(item.placement.row_end, 2);
}

#[test]
fn test_grid_item_row_span() {
    let item = GridItem::new(Text::new("Test")).at(1, 1).row_span(2);
    assert_eq!(item.placement.col_start, 1);
    assert_eq!(item.placement.col_end, 2);
    assert_eq!(item.placement.row_start, 1);
    assert_eq!(item.placement.row_end, 3);
}

#[test]
fn test_grid_item_col() {
    let item = GridItem::new(Text::new("Test")).col(5);
    assert_eq!(item.placement.col_start, 5);
    assert_eq!(item.placement.col_end, 6);
    assert_eq!(item.placement.row_start, 0);
    assert_eq!(item.placement.row_end, 0);
}

#[test]
fn test_grid_item_col_preserves_end() {
    let item = GridItem::new(Text::new("Test")).at(1, 1).col_span(3).col(5);
    assert_eq!(item.placement.col_start, 5);
    // col_end should be preserved when already set
    assert_eq!(item.placement.col_end, 4);
}

#[test]
fn test_grid_item_row() {
    let item = GridItem::new(Text::new("Test")).row(3);
    assert_eq!(item.placement.col_start, 0);
    assert_eq!(item.placement.col_end, 0);
    assert_eq!(item.placement.row_start, 3);
    assert_eq!(item.placement.row_end, 4);
}

#[test]
fn test_grid_item_row_preserves_end() {
    let item = GridItem::new(Text::new("Test")).at(1, 1).row_span(2).row(5);
    assert_eq!(item.placement.row_start, 5);
    // row_end should be preserved when already set
    assert_eq!(item.placement.row_end, 3);
}

#[test]
fn test_grid_item_builder_chain() {
    let item = GridItem::new(Text::new("Test"))
        .at(2, 3)
        .col_span(2)
        .row_span(4);
    assert_eq!(item.placement.col_start, 2);
    assert_eq!(item.placement.col_end, 4);
    assert_eq!(item.placement.row_start, 3);
    assert_eq!(item.placement.row_end, 7);
}

// =========================================================================
// GridAlign enum tests
// =========================================================================

#[test]
fn test_grid_align_stretch() {
    let align = GridAlign::Stretch;
    assert_eq!(align, GridAlign::Stretch);
}

#[test]
fn test_grid_align_start() {
    let align = GridAlign::Start;
    assert_eq!(align, GridAlign::Start);
}

#[test]
fn test_grid_align_center() {
    let align = GridAlign::Center;
    assert_eq!(align, GridAlign::Center);
}

#[test]
fn test_grid_align_end() {
    let align = GridAlign::End;
    assert_eq!(align, GridAlign::End);
}

#[test]
fn test_grid_align_default() {
    let align = GridAlign::default();
    assert_eq!(align, GridAlign::Stretch);
}

#[test]
fn test_grid_align_clone() {
    let align1 = GridAlign::Center;
    let align2 = align1.clone();
    assert_eq!(align1, align2);
}

#[test]
fn test_grid_align_copy() {
    let align1 = GridAlign::End;
    let align2 = align1;
    assert_eq!(align1, GridAlign::End);
    assert_eq!(align2, GridAlign::End);
}

#[test]
fn test_grid_align_partial_eq() {
    assert_eq!(GridAlign::Start, GridAlign::Start);
    assert_ne!(GridAlign::Start, GridAlign::End);
    assert_eq!(GridAlign::Center, GridAlign::Center);
}

#[test]
fn test_grid_align_debug() {
    let align = GridAlign::Center;
    let debug_str = format!("{:?}", align);
    assert!(debug_str.contains("Center") || debug_str.contains("GridAlign"));
}

// =========================================================================
// Integration tests
// =========================================================================

#[test]
fn test_grid_placement_complex_area() {
    let placement = GridPlacement::area(1, 1, 2, 3);
    assert_eq!(placement.col_start, 1);
    assert_eq!(placement.col_end, 3);
    assert_eq!(placement.row_start, 1);
    assert_eq!(placement.row_end, 4);
}

#[test]
fn test_grid_placement_combined_span() {
    let placement = GridPlacement::cell(2, 2).span_cols(3).span_rows(2);
    assert_eq!(placement.col_start, 2);
    assert_eq!(placement.col_end, 5);
    assert_eq!(placement.row_start, 2);
    assert_eq!(placement.row_end, 4);
}

#[test]
fn test_grid_item_full_placement() {
    let item = GridItem::new(Text::new("Full cell")).place(GridPlacement::area(1, 1, 2, 2));
    assert_eq!(item.placement.col_start, 1);
    assert_eq!(item.placement.col_end, 3);
    assert_eq!(item.placement.row_start, 1);
    assert_eq!(item.placement.row_end, 3);
}