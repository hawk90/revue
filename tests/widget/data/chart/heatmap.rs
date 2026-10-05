//! HeatMap widget tests
//!
//! This is the surviving copy of the in-source heatmap tests
//! (src/widget/data/chart/heatmap/tests.rs). Assertions that read private
//! fields go through normalize()/color_for()/render_cell() or the rendered
//! buffer instead.
//!
//! HeatMap lays itself out with stacks, which in 2.x give every child an
//! equal share of the area. Cell-layout tests therefore render into an area
//! that exactly fits the cells (columns x cell_width by rows x cell_height),
//! where an equal share is the cell's own size.

use super::{render, render_rows, rows};
use revue::render::{Buffer, Modifier};
use revue::style::Color;
use revue::widget::data::chart::{contribution_map, heatmap, CellDisplay, ColorScale, HeatMap};

fn fg(buffer: &Buffer, x: u16, y: u16) -> Option<Color> {
    buffer.get(x, y).unwrap().fg
}

fn blank(buffer: &Buffer) -> bool {
    rows(buffer).iter().all(|r| r.trim().is_empty())
}

// ==================== Construction ====================

#[test]
fn test_heatmap_new() {
    let hm = HeatMap::new(vec![vec![0.0, 0.5, 1.0], vec![0.2, 0.4, 0.8]]);
    // Bounds come from the data
    assert_eq!(hm.normalize(0.0), 0.0);
    assert_eq!(hm.normalize(1.0), 1.0);

    // Default cells: two-column blocks colored by value
    let buffer = render(&hm, 6, 2);
    let rows = rows(&buffer);
    assert_eq!(rows[0].trim_end(), "██████");
    assert_eq!(rows[1].trim_end(), "██████");
    assert_eq!(fg(&buffer, 0, 0), Some(hm.color_for(0.0)));
    assert_eq!(fg(&buffer, 4, 0), Some(hm.color_for(1.0)));
    assert_eq!(fg(&buffer, 2, 1), Some(hm.color_for(0.4)));
}

#[test]
fn test_heatmap_helper_matches_new() {
    let data = vec![vec![0.5, 0.1]];
    assert_eq!(
        render_rows(&heatmap(data.clone()), 10, 2),
        render_rows(&HeatMap::new(data), 10, 2)
    );
}

#[test]
fn test_heatmap_from_flat() {
    let hm = HeatMap::from_flat(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 2, 3);
    assert_eq!(hm.normalize(1.0), 0.0);
    assert_eq!(hm.normalize(6.0), 1.0);
    let buffer = render(&hm, 6, 2);
    assert_eq!(rows(&buffer)[0].trim_end(), "██████");
    assert_eq!(rows(&buffer)[1].trim_end(), "██████");
    // (row 0, col 0) holds 1.0 and (row 1, col 2) holds 6.0
    assert_eq!(fg(&buffer, 0, 0), Some(hm.color_for(1.0)));
    assert_eq!(fg(&buffer, 4, 1), Some(hm.color_for(6.0)));
}

#[test]
fn test_from_flat_partial_data() {
    // Data shorter than rows * cols: the second row is empty
    let hm = HeatMap::from_flat(&[1.0, 2.0, 3.0], 2, 3);
    let rows = render_rows(&hm, 6, 2);
    assert_eq!(rows[0].trim_end(), "██████");
    assert!(rows[1].trim().is_empty());
}

#[test]
fn test_heatmap_empty_data() {
    let hm = HeatMap::new(vec![]);
    // Default bounds 0..1
    assert_eq!(hm.normalize(0.0), 0.0);
    assert_eq!(hm.normalize(1.0), 1.0);
    assert!(blank(&render(&hm, 10, 3)));
}

#[test]
fn test_heatmap_empty_rows() {
    let hm = HeatMap::new(vec![vec![], vec![]]);
    assert!(blank(&render(&hm, 10, 3)));
}

#[test]
fn test_heatmap_clone_and_debug() {
    let hm = HeatMap::new(vec![vec![0.5, 1.0]])
        .title("Test")
        .color_scale(ColorScale::Viridis);
    let cloned = hm.clone();
    assert_eq!(render_rows(&cloned, 10, 3), render_rows(&hm, 10, 3));
    assert!(format!("{:?}", hm).contains("HeatMap"));
}

// ==================== Normalization ====================

#[test]
fn test_normalization() {
    let hm = HeatMap::new(vec![vec![10.0, 20.0, 30.0]]);
    assert_eq!(hm.normalize(10.0), 0.0);
    assert_eq!(hm.normalize(20.0), 0.5);
    assert_eq!(hm.normalize(30.0), 1.0);
}

#[test]
fn test_normalize_range() {
    let hm = HeatMap::new(vec![vec![0.0, 100.0]]);
    assert_eq!(hm.normalize(0.0), 0.0);
    assert_eq!(hm.normalize(25.0), 0.25);
    assert_eq!(hm.normalize(50.0), 0.5);
    assert_eq!(hm.normalize(100.0), 1.0);
}

#[test]
fn test_normalize_same_min_max() {
    let hm = HeatMap::new(vec![vec![5.0, 5.0, 5.0]]);
    assert_eq!(hm.normalize(5.0), 0.5);
}

#[test]
fn test_heatmap_negative_values() {
    let hm = HeatMap::new(vec![vec![-10.0, 0.0, 10.0]]);
    assert_eq!(hm.normalize(-10.0), 0.0);
    assert_eq!(hm.normalize(0.0), 0.5);
    assert_eq!(hm.normalize(10.0), 1.0);
}

#[test]
fn test_custom_bounds() {
    let hm = HeatMap::new(vec![vec![5.0]]).bounds(-10.0, 10.0);
    assert_eq!(hm.normalize(-10.0), 0.0);
    assert_eq!(hm.normalize(10.0), 1.0);
    assert_eq!(hm.normalize(5.0), 0.75);
}

// ==================== ColorScale ====================

#[test]
fn test_color_scale_default() {
    assert_eq!(ColorScale::default(), ColorScale::BlueRed);
    // and HeatMap uses it by default
    let hm = HeatMap::new(vec![vec![0.0, 1.0]]);
    assert_eq!(hm.color_for(0.0), ColorScale::BlueRed.color_at(0.0));
}

#[test]
fn test_color_scale_blue_red() {
    let scale = ColorScale::BlueRed;
    assert_eq!(scale.color_at(0.0).b, 255);
    assert_eq!(scale.color_at(1.0).r, 255);

    // Lower half: blue to white
    let low = scale.color_at(0.25);
    assert_eq!(low.b, 255);
    assert!(low.r > 0 && low.r < 255);
    // Upper half: white to red
    let high = scale.color_at(0.75);
    assert_eq!(high.r, 255);
    assert!(high.b < 255 && high.b > 0);
}

#[test]
fn test_color_scale_green_buckets() {
    let scale = ColorScale::Green;
    assert_eq!(scale.color_at(0.0), Color::rgb(22, 27, 34));
    assert_eq!(scale.color_at(0.15), Color::rgb(14, 68, 41));
    assert_eq!(scale.color_at(0.35), Color::rgb(0, 109, 50));
    assert_eq!(scale.color_at(0.60), Color::rgb(38, 166, 65));
    assert_eq!(scale.color_at(0.90), Color::rgb(57, 211, 83));
    assert_eq!(scale.color_at(1.0), Color::rgb(57, 211, 83));
}

#[test]
fn test_color_scale_viridis() {
    let low = ColorScale::Viridis.color_at(0.0);
    let high = ColorScale::Viridis.color_at(1.0);
    // Starts purple-ish, ends yellow-ish
    assert!(low.b > low.r);
    assert!(high.g > high.b);
}

#[test]
fn test_color_scale_plasma() {
    let scale = ColorScale::Plasma;
    let low = scale.color_at(0.0);
    let mid = scale.color_at(0.5);
    let high = scale.color_at(1.0);
    assert!(low.r < 50);
    assert!(high.r > 200);
    assert!(mid.r > low.r && mid.r < high.r);
}

#[test]
fn test_color_scale_gray() {
    let scale = ColorScale::Gray;
    assert_eq!(scale.color_at(0.0), Color::rgb(0, 0, 0));
    assert_eq!(scale.color_at(1.0), Color::rgb(255, 255, 255));
    let mid = scale.color_at(0.5);
    assert_eq!(mid.r, mid.g);
    assert_eq!(mid.g, mid.b);
}

#[test]
fn test_color_scale_red_yellow_green() {
    let scale = ColorScale::RedYellowGreen;
    assert_eq!(scale.color_at(0.0), Color::rgb(255, 0, 0));
    assert_eq!(scale.color_at(0.5), Color::rgb(255, 255, 0));
    assert_eq!(scale.color_at(1.0), Color::rgb(0, 255, 0));
}

#[test]
fn test_color_scale_custom_returns_white() {
    // Custom only means something together with custom_colors()
    assert_eq!(ColorScale::Custom.color_at(0.5), Color::WHITE);
}

#[test]
fn test_color_scale_value_clamping() {
    assert_eq!(ColorScale::Gray.color_at(-1.0), Color::rgb(0, 0, 0));
    assert_eq!(ColorScale::Gray.color_at(2.0), Color::rgb(255, 255, 255));
}

// ==================== color_for ====================

#[test]
fn test_color_scale_builder() {
    let hm = HeatMap::new(vec![vec![0.0, 1.0]]).color_scale(ColorScale::Gray);
    assert_eq!(hm.color_for(0.0), Color::rgb(0, 0, 0));
    assert_eq!(hm.color_for(1.0), Color::rgb(255, 255, 255));
    let buffer = render(&hm, 4, 1);
    assert_eq!(fg(&buffer, 0, 0), Some(Color::rgb(0, 0, 0)));
    assert_eq!(fg(&buffer, 2, 0), Some(Color::rgb(255, 255, 255)));
}

#[test]
fn test_custom_colors() {
    let hm = HeatMap::new(vec![vec![0.0, 1.0]]).custom_colors(Color::BLUE, Color::RED);
    assert_eq!(hm.color_for(0.0), Color::BLUE);
    assert_eq!(hm.color_for(1.0), Color::RED);
    let buffer = render(&hm, 4, 1);
    assert_eq!(fg(&buffer, 0, 0), Some(Color::BLUE));
    assert_eq!(fg(&buffer, 2, 0), Some(Color::RED));
}

#[test]
fn test_custom_colors_interpolate() {
    let hm = HeatMap::new(vec![vec![0.0, 1.0]])
        .custom_colors(Color::rgb(0, 0, 0), Color::rgb(255, 255, 255));
    assert_eq!(hm.color_for(0.0), Color::rgb(0, 0, 0));
    assert_eq!(hm.color_for(1.0), Color::rgb(255, 255, 255));
    assert_eq!(hm.color_for(0.5), Color::rgb(127, 127, 127));
}

// ==================== CellDisplay and cell sizes ====================

#[test]
fn test_cell_display_default() {
    assert_eq!(CellDisplay::default(), CellDisplay::Block);
}

#[test]
fn test_render_cell_block() {
    let hm = HeatMap::new(vec![vec![0.5]])
        .cell_display(CellDisplay::Block)
        .cell_width(3);
    assert_eq!(hm.render_cell(0.5), "███");
}

#[test]
fn test_render_cell_half_block() {
    let hm = HeatMap::new(vec![vec![0.5]]).cell_display(CellDisplay::HalfBlock);
    assert_eq!(hm.render_cell(0.5), "▀▀");
    assert_eq!(render_rows(&hm, 4, 1)[0], "▀▀  ");
}

#[test]
fn test_render_cell_value_display() {
    let hm = HeatMap::new(vec![vec![0.5]])
        .cell_display(CellDisplay::Value)
        .cell_width(4)
        .value_decimals(1);
    assert_eq!(hm.render_cell(0.5), " 0.5");
}

#[test]
fn test_render_cell_custom() {
    let hm = HeatMap::new(vec![vec![0.5]])
        .cell_display(CellDisplay::Custom)
        .cell_width(2);
    assert_eq!(hm.render_cell(0.5), "■■");
}

#[test]
fn test_cell_size() {
    let hm = HeatMap::new(vec![vec![0.5]]).cell_size(5, 3);
    let rows = render_rows(&hm, 5, 3);
    assert_eq!(rows, ["█████", "█████", "█████"]);
}

#[test]
fn test_cell_width_and_height() {
    let hm = HeatMap::new(vec![vec![0.5]]).cell_width(8).cell_height(2);
    let rows = render_rows(&hm, 8, 2);
    assert_eq!(rows, ["████████", "████████"]);
}

// ==================== Values ====================

#[test]
fn test_show_values_widens_cells_to_four() {
    let hm = HeatMap::new(vec![vec![0.5]])
        .cell_width(2)
        .show_values(true);
    assert_eq!(hm.render_cell(0.5), " 0.5");
}

#[test]
fn test_show_values_keeps_larger_width() {
    let hm = HeatMap::new(vec![vec![0.5]])
        .cell_width(10)
        .show_values(true);
    assert_eq!(hm.render_cell(0.5), format!("{:>10}", "0.5"));
}

#[test]
fn test_show_values_overrides_display_and_decimals() {
    let hm = HeatMap::new(vec![vec![0.5]])
        .cell_display(CellDisplay::Block)
        .show_values(true)
        .value_decimals(3);
    assert_eq!(hm.render_cell(0.5), "0.500");
}

#[test]
fn test_show_values_contrast() {
    // Dark cells get white text, bright cells black text
    let hm = HeatMap::new(vec![vec![0.0, 1.0]])
        .color_scale(ColorScale::Gray)
        .show_values(true);
    let buffer = render(&hm, 8, 1);
    assert_eq!(rows(&buffer)[0], " 0.0 1.0");
    let dark = buffer.get(1, 0).unwrap();
    assert_eq!(dark.bg, Some(Color::rgb(0, 0, 0)));
    assert_eq!(dark.fg, Some(Color::WHITE));
    let bright = buffer.get(5, 0).unwrap();
    assert_eq!(bright.bg, Some(Color::rgb(255, 255, 255)));
    assert_eq!(bright.fg, Some(Color::BLACK));
}

// ==================== Title, labels, legend, highlight ====================

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
fn test_highlight() {
    let hm = HeatMap::new(vec![vec![0.5, 0.6], vec![0.7, 0.8]]).highlight(1, 0);
    let buffer = render(&hm, 4, 2);
    let bold = |x, y| buffer.get(x, y).unwrap().modifier.contains(Modifier::BOLD);
    assert!(bold(0, 1) && bold(1, 1));
    assert!(!bold(2, 1));
    assert!(!bold(0, 0));
}

// ==================== Constructors ====================

#[test]
fn test_correlation_matrix() {
    let data = vec![
        vec![1.0, 0.5, -0.5],
        vec![0.5, 1.0, 0.3],
        vec![-0.5, 0.3, 1.0],
    ];
    let labels = vec!["A".into(), "B".into(), "C".into()];
    let hm = HeatMap::correlation_matrix(&data, labels);
    // Fixed -1..1 bounds on the BlueRed scale
    assert_eq!(hm.normalize(-1.0), 0.0);
    assert_eq!(hm.normalize(0.0), 0.5);
    assert_eq!(hm.color_for(1.0), ColorScale::BlueRed.color_at(1.0));
    // Labels and values are shown
    let rows = render_rows(&hm, 40, 4);
    for label in ["A", "B", "C"] {
        assert!(rows[0].contains(label), "{rows:?}");
    }
    assert!(rows[1].starts_with("     A "), "{rows:?}");
    assert!(
        rows[1].contains("1.0") && rows[1].contains("0.5"),
        "{rows:?}"
    );
    assert!(rows[1].contains("-0.5"), "{rows:?}");
    assert!(rows[3].starts_with("     C "), "{rows:?}");
}

#[test]
fn test_contribution_map() {
    let contributions: Vec<u32> = (0..364).map(|i| i % 10).collect();
    for hm in [
        HeatMap::contribution_map(&contributions),
        contribution_map(&contributions),
    ] {
        // 7 days x 52 weeks of 2-column cells on the GitHub green scale
        let buffer = render(&hm, 104, 7);
        for row in rows(&buffer) {
            assert_eq!(row.chars().filter(|&c| c == '█').count(), 104);
        }
        // Day 0 of week 0 has no contributions, day 1 of week 0 has 1 of 9
        assert_eq!(fg(&buffer, 0, 0), Some(ColorScale::Green.color_at(0.0)));
        assert_eq!(
            fg(&buffer, 0, 1),
            Some(ColorScale::Green.color_at(1.0 / 9.0))
        );
    }
}
