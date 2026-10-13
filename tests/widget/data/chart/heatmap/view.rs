//! HeatMap layout tests: title, labels, legend, line packing, zero and offset areas

use super::super::{
    assert_same_when_offset, assert_zero_areas_draw_nothing, render, render_rows, rows,
};
use super::fg;
use revue::widget::data::chart::{heatmap, ColorScale, HeatMap};

#[test]
fn test_title() {
    let rows = render_rows(&HeatMap::new(vec![vec![0.5]]).title("Test Title"), 20, 2);
    assert_eq!(rows[0].trim_end(), "Test Title");
    assert_eq!(rows[1].trim_end(), "██");
}

#[test]
fn test_labels() {
    let hm = HeatMap::new(vec![vec![0.5, 0.8], vec![0.2, 0.9]])
        .row_labels(vec!["R1".into(), "R2".into()])
        .col_labels(vec!["C1".into(), "C2".into()]);
    let rows = render_rows(&hm, 30, 3);
    assert!(rows[0].contains("C1") && rows[0].contains("C2"), "{rows:?}");
    assert!(rows[1].starts_with("    R1 "), "{rows:?}");
    assert!(rows[2].starts_with("    R2 "), "{rows:?}");
    assert_eq!(rows[1].matches('█').count(), 4);
}

#[test]
fn test_labels_align_with_cells() {
    let hm = HeatMap::new(vec![vec![0.5, 0.8], vec![0.2, 0.9]])
        .row_labels(vec!["R1".into(), "R2".into()])
        .col_labels(vec!["C1".into(), "C2".into()]);
    let rows = render_rows(&hm, 30, 3);
    assert_eq!(rows[0].trim_end(), "       C1C2");
    assert_eq!(rows[1].trim_end(), "    R1 ████");
    assert_eq!(rows[2].trim_end(), "    R2 ████");
}

#[test]
fn test_col_label_truncation() {
    let hm = HeatMap::new(vec![vec![0.5]])
        .cell_width(3)
        .col_labels(vec!["VeryLongLabel".into()]);
    let rows = render_rows(&hm, 30, 2);
    assert_eq!(rows[0].trim(), "Ver");
}

#[test]
fn test_row_label_truncation() {
    let hm = HeatMap::new(vec![vec![0.5]]).row_labels(vec!["VeryLongRowLabel".into()]);
    let rows = render_rows(&hm, 30, 1);
    assert!(rows[0].starts_with("VeryLo "), "{rows:?}");
    assert!(!rows[0].contains("VeryLon"));
}

#[test]
fn test_show_legend() {
    let hidden = render_rows(&HeatMap::new(vec![vec![0.0, 1.0]]), 40, 2).join("\n");
    assert!(!hidden.contains("Low"));

    let hm = HeatMap::new(vec![vec![0.0, 1.0]]).show_legend(true);
    // The legend pieces sit side by side at their own widths, so it fits
    // in exactly 4 + 10 + 5 + 13 columns
    let buffer = render(&hm, 32, 2);
    let legend = &rows(&buffer)[1];
    assert_eq!(legend, "Low ██████████ High  (0.0 - 1.0)");
    // Ten color swatches from the low to the high end of the scale
    assert_eq!(fg(&buffer, 4, 1), Some(ColorScale::BlueRed.color_at(0.0)));
    assert_eq!(fg(&buffer, 13, 1), Some(ColorScale::BlueRed.color_at(1.0)));
}

#[test]
fn test_lines_pack_from_the_top() {
    // A tall area does not spread the title, header and rows apart
    let hm = HeatMap::new(vec![vec![0.5], vec![0.6]])
        .title("T")
        .col_labels(vec!["C".into()]);
    let rows = render_rows(&hm, 10, 10);
    assert_eq!(rows[0].trim_end(), "T");
    assert_eq!(rows[1].trim_end(), "C");
    assert_eq!(rows[2].trim_end(), "██");
    assert_eq!(rows[3].trim_end(), "██");
    assert!(rows[4..].iter().all(|r| r.trim().is_empty()), "{rows:?}");
}

#[test]
fn test_rows_without_a_label_stay_aligned() {
    // Fewer row labels than rows: unlabeled rows keep the label column
    let hm = HeatMap::new(vec![vec![0.5], vec![0.6]])
        .row_labels(vec!["R1".into()])
        .col_labels(vec!["C".into()]);
    let rows = render_rows(&hm, 20, 3);
    assert_eq!(rows[0].trim_end(), "       C");
    assert_eq!(rows[1].trim_end(), "    R1 ██");
    assert_eq!(rows[2].trim_end(), "       ██");
}

#[test]
fn test_heatmap_zero_area_draws_nothing() {
    let hm = heatmap(vec![vec![0.0, 1.0], vec![0.5, 0.2]])
        .title("T")
        .col_labels(vec!["a".into(), "b".into()])
        .show_legend(true);
    assert_zero_areas_draw_nothing(&hm);
}

#[test]
fn test_heatmap_draws_the_same_when_offset() {
    let hm = heatmap(vec![vec![0.0, 1.0], vec![0.5, 0.2]])
        .title("T")
        .row_labels(vec!["r1".into(), "r2".into()])
        .col_labels(vec!["a".into(), "b".into()])
        .highlight(1, 0)
        .show_legend(true);
    assert_same_when_offset(hm.clone(), 40, 8);
    assert_same_when_offset(hm.show_values(true), 40, 8);
}
