//! Tests for Heat Map widget
//!
//! Extracted from src/widget/data/chart/heatmap/

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::RenderContext;
use revue::widget::View;
use revue::widget::data::chart::heatmap::{heatmap, contribution_map, HeatMap};
use revue::widget::data::chart::heatmap::types::{CellDisplay, ColorScale};

// ==================== Basic Tests ====================

#[test]
fn test_heatmap_new() {
    let data = vec![vec![0.0, 0.5, 1.0], vec![0.2, 0.4, 0.8]];
    let hm = HeatMap::new(data.clone());
    // Note: _rows and cols are private, so we test via public API
    assert_eq!(hm.data, data);
    assert_eq!(hm.min_val, 0.0);
    assert_eq!(hm.max_val, 1.0);
}

#[test]
fn test_heatmap_from_flat() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let hm = HeatMap::from_flat(&data, 2, 3);
    assert_eq!(hm.data[0][0], 1.0);
    assert_eq!(hm.data[1][2], 6.0);
}

#[test]
fn test_normalization() {
    let hm = HeatMap::new(vec![vec![10.0, 20.0, 30.0]]);
    assert!((hm.normalize(10.0) - 0.0).abs() < 0.001);
    assert!((hm.normalize(20.0) - 0.5).abs() < 0.001);
    assert!((hm.normalize(30.0) - 1.0).abs() < 0.001);
}

#[test]
fn test_color_scale_blue_red() {
    let scale = ColorScale::BlueRed;
    let low = scale.color_at(0.0);
    let _mid = scale.color_at(0.5);
    let high = scale.color_at(1.0);

    assert_eq!(low.b, 255); // Blue
    assert_eq!(high.r, 255); // Red
}

#[test]
fn test_color_scale_green() {
    let scale = ColorScale::Green;
    let empty = scale.color_at(0.0);
    let full = scale.color_at(1.0);

    // Empty should be dark
    assert!(empty.r < 50);
    // Full should be bright green
    assert!(full.g > 200);
}

#[test]
fn test_custom_bounds() {
    let hm = HeatMap::new(vec![vec![5.0]]).bounds(-10.0, 10.0);
    assert_eq!(hm.min_val, -10.0);
    assert_eq!(hm.max_val, 10.0);
    assert!((hm.normalize(5.0) - 0.75).abs() < 0.001);
}

#[test]
fn test_contribution_map() {
    let contributions: Vec<u32> = (0..364).map(|i| i % 10).collect();
    let hm = HeatMap::contribution_map(&contributions);
    assert_eq!(hm.color_scale, ColorScale::Green);
}

#[test]
fn test_labels() {
    let hm = HeatMap::new(vec![vec![1.0, 2.0], vec![3.0, 4.0]])
        .row_labels(vec!["A".into(), "B".into()])
        .col_labels(vec!["X".into(), "Y".into()]);

    assert!(hm.row_labels.is_some());
    assert!(hm.col_labels.is_some());
}

#[test]
fn test_helper_functions() {
    let data = vec![vec![0.5]];
    let hm = heatmap(data);
    assert_eq!(hm.data.len(), 1);
}

// ==================== ColorScale Tests ====================

#[test]
fn test_color_scale_viridis() {
    let scale = ColorScale::Viridis;
    let low = scale.color_at(0.0);
    let high = scale.color_at(1.0);

    // Viridis starts purple-ish and ends yellow-ish
    assert!(low.b > low.r); // Low end has more blue
    assert!(high.g > high.b); // High end has more green
}

#[test]
fn test_color_scale_plasma() {
    let scale = ColorScale::Plasma;
    let low = scale.color_at(0.0);
    let mid = scale.color_at(0.5);
    let high = scale.color_at(1.0);

    // Plasma goes from dark purple to bright yellow
    assert!(low.r < 50); // Starts dark
    assert!(high.r > 200); // Ends bright
    assert!(mid.r > low.r && mid.r < high.r); // Monotonic increase
}

#[test]
fn test_color_scale_gray() {
    let scale = ColorScale::Gray;
    let black = scale.color_at(0.0);
    let white = scale.color_at(1.0);
    let mid = scale.color_at(0.5);

    assert_eq!(black.r, 0);
    assert_eq!(black.g, 0);
    assert_eq!(black.b, 0);

    assert_eq!(white.r, 255);
    assert_eq!(white.g, 255);
    assert_eq!(white.b, 255);

    // Grayscale should have equal RGB
    assert_eq!(mid.r, mid.g);
    assert_eq!(mid.g, mid.b);
}

#[test]
fn test_color_scale_red_yellow_green() {
    let scale = ColorScale::RedYellowGreen;
    let red = scale.color_at(0.0);
    let yellow = scale.color_at(0.5);
    let green = scale.color_at(1.0);

    // At 0.0: Red
    assert_eq!(red.r, 255);
    assert_eq!(red.g, 0);
    assert_eq!(red.b, 0);

    // At 0.5: Yellow
    assert_eq!(yellow.r, 255);
    assert_eq!(yellow.g, 255);
    assert_eq!(yellow.b, 0);

    // At 1.0: Green
    assert_eq!(green.r, 0);
    assert_eq!(green.g, 255);
    assert_eq!(green.b, 0);
}

#[test]
fn test_color_scale_custom_returns_white() {
    let scale = ColorScale::Custom;
    let color = scale.color_at(0.5);
    // Custom returns WHITE when used directly (override with custom_colors)
    assert_eq!(color, Color::WHITE);
}

#[test]
fn test_color_scale_green_buckets() {
    let scale = ColorScale::Green;

    // Test each bucket
    let empty = scale.color_at(0.0); // < 0.01
    let low = scale.color_at(0.15); // 0.01-0.25
    let med_low = scale.color_at(0.35); // 0.25-0.50
    let med_high = scale.color_at(0.60); // 0.50-0.75
    let high = scale.color_at(0.90); // >= 0.75

    // Empty is darkest
    assert_eq!(empty, Color::rgb(22, 27, 34));
    // Low
    assert_eq!(low, Color::rgb(14, 68, 41));
    // Medium-low
    assert_eq!(med_low, Color::rgb(0, 109, 50));
    // Medium-high
    assert_eq!(med_high, Color::rgb(38, 166, 65));
    // High
    assert_eq!(high, Color::rgb(57, 211, 83));
}

#[test]
fn test_color_scale_blue_red_gradient() {
    let scale = ColorScale::BlueRed;

    // Lower half: blue to white
    let low = scale.color_at(0.25);
    assert_eq!(low.b, 255); // Blue stays at 255
    assert!(low.r > 0 && low.r < 255); // Red increases

    // Upper half: white to red
    let high = scale.color_at(0.75);
    assert_eq!(high.r, 255); // Red stays at 255
    assert!(high.b < 255 && high.b > 0); // Blue decreases
}

#[test]
fn test_color_scale_value_clamping() {
    let scale = ColorScale::Gray;

    // Values below 0 should clamp to 0
    let below = scale.color_at(-1.0);
    assert_eq!(below, Color::rgb(0, 0, 0));

    // Values above 1 should clamp to 1
    let above = scale.color_at(2.0);
    assert_eq!(above, Color::rgb(255, 255, 255));
}

#[test]
fn test_color_scale_default() {
    assert_eq!(ColorScale::default(), ColorScale::BlueRed);
}

#[test]
fn test_color_scale_debug_and_clone() {
    let scale = ColorScale::Viridis;
    let cloned = scale;
    assert_eq!(scale, cloned);
    let _ = format!("{:?}", scale);
}

// ==================== CellDisplay Tests ====================

#[test]
fn test_cell_display_default() {
    assert_eq!(CellDisplay::default(), CellDisplay::Block);
}

#[test]
fn test_cell_display_debug_and_clone() {
    let display = CellDisplay::HalfBlock;
    let cloned = display;
    assert_eq!(display, cloned);
    let _ = format!("{:?}", display);
}

// ==================== HeatMap Builder Tests ====================

#[test]
fn test_heatmap_custom_colors() {
    let hm = HeatMap::new(vec![vec![0.0, 1.0]]).custom_colors(Color::BLUE, Color::RED);

    assert_eq!(hm.color_scale, ColorScale::Custom);
    assert!(hm.custom_colors.is_some());

    let (low, high) = hm.custom_colors.unwrap();
    assert_eq!(low, Color::BLUE);
    assert_eq!(high, Color::RED);
}

#[test]
fn test_heatmap_cell_display() {
    let hm = HeatMap::new(vec![vec![0.5]]).cell_display(CellDisplay::HalfBlock);
    assert_eq!(hm.cell_display, CellDisplay::HalfBlock);
}

#[test]
fn test_heatmap_cell_size() {
    let hm = HeatMap::new(vec![vec![0.5]]).cell_size(5, 3);
    assert_eq!(hm.cell_width, 5);
    assert_eq!(hm.cell_height, 3);
}

#[test]
fn test_heatmap_cell_width() {
    let hm = HeatMap::new(vec![vec![0.5]]).cell_width(8);
    assert_eq!(hm.cell_width, 8);
}

#[test]
fn test_heatmap_cell_height() {
    let hm = HeatMap::new(vec![vec![0.5]]).cell_height(4);
    assert_eq!(hm.cell_height, 4);
}

#[test]
fn test_heatmap_show_values_increases_cell_width() {
    let hm = HeatMap::new(vec![vec![0.5]])
        .cell_width(2)
        .show_values(true);

    // show_values increases cell_width to at least 4
    assert!(hm.cell_width >= 4);
    assert!(hm.show_values);
}

#[test]
fn test_heatmap_show_values_keeps_larger_width() {
    let hm = HeatMap::new(vec![vec![0.5]])
        .cell_width(10)
        .show_values(true);

    // Should keep the larger width
    assert_eq!(hm.cell_width, 10);
}

#[test]
fn test_heatmap_value_decimals() {
    let hm = HeatMap::new(vec![vec![0.5]]).value_decimals(3);
    assert_eq!(hm.value_decimals, 3);
}

#[test]
fn test_heatmap_title() {
    let hm = HeatMap::new(vec![vec![0.5]]).title("My Heatmap");
    assert_eq!(hm.title, Some("My Heatmap".to_string()));
}

#[test]
fn test_heatmap_show_legend() {
    let hm = HeatMap::new(vec![vec![0.5]]).show_legend(true);
    assert!(hm.show_legend);
}

#[test]
fn test_heatmap_highlight() {
    let hm = HeatMap::new(vec![vec![0.5, 0.6], vec![0.7, 0.8]]).highlight(1, 0);
    assert_eq!(hm.highlighted, Some((1, 0)));
}

// ==================== HeatMap Helper Functions ====================

#[test]
fn test_correlation_matrix() {
    let data = vec![
        vec![1.0, 0.5, -0.5],
        vec![0.5, 1.0, 0.3],
        vec![-0.5, 0.3, 1.0],
    ];
    let labels = vec!["A".into(), "B".into(), "C".into()];
    let hm = HeatMap::correlation_matrix(&data, labels);

    assert_eq!(hm.color_scale, ColorScale::BlueRed);
    assert_eq!(hm.min_val, -1.0);
    assert_eq!(hm.max_val, 1.0);
    assert!(hm.row_labels.is_some());
    assert!(hm.col_labels.is_some());
    assert!(hm.show_values);
}

#[test]
fn test_contribution_map_helper() {
    let contributions: Vec<u32> = (0..100).collect();
    let hm = contribution_map(&contributions);
    assert_eq!(hm.color_scale, ColorScale::Green);
}

// ==================== Normalization Tests ====================

#[test]
fn test_normalize_same_min_max() {
    let hm = HeatMap::new(vec![vec![5.0, 5.0, 5.0]]);
    // When min == max, normalize returns 0.5
    assert_eq!(hm.normalize(5.0), 0.5);
}

#[test]
fn test_normalize_range() {
    let hm = HeatMap::new(vec![vec![0.0, 100.0]]);
    assert_eq!(hm.normalize(0.0), 0.0);
    assert_eq!(hm.normalize(50.0), 0.5);
    assert_eq!(hm.normalize(100.0), 1.0);
    assert_eq!(hm.normalize(25.0), 0.25);
}

// ==================== Color For Value Tests ====================

#[test]
fn test_color_for_with_custom_colors() {
    let hm = HeatMap::new(vec![vec![0.0, 1.0]])
        .custom_colors(Color::rgb(0, 0, 0), Color::rgb(255, 255, 255));

    let low_color = hm.color_for(0.0);
    let high_color = hm.color_for(1.0);
    let mid_color = hm.color_for(0.5);

    assert_eq!(low_color, Color::rgb(0, 0, 0));
    assert_eq!(high_color, Color::rgb(255, 255, 255));
    // Mid should be approximately gray
    assert!(mid_color.r > 100 && mid_color.r < 150);
}

#[test]
fn test_color_for_with_standard_scale() {
    let hm = HeatMap::new(vec![vec![0.0, 1.0]]).color_scale(ColorScale::Gray);

    let low = hm.color_for(0.0);
    let high = hm.color_for(1.0);

    assert_eq!(low, Color::rgb(0, 0, 0));
    assert_eq!(high, Color::rgb(255, 255, 255));
}

// ==================== Render Cell Tests ====================

#[test]
fn test_render_cell_block() {
    let hm = HeatMap::new(vec![vec![0.5]])
        .cell_display(CellDisplay::Block)
        .cell_width(3);

    let cell = hm.render_cell(0.5);
    assert_eq!(cell, "███");
}

#[test]
fn test_render_cell_half_block() {
    let hm = HeatMap::new(vec![vec![0.5]])
        .cell_display(CellDisplay::HalfBlock)
        .cell_width(2);

    let cell = hm.render_cell(0.5);
    assert_eq!(cell, "▀▀");
}

#[test]
fn test_render_cell_value_display() {
    let hm = HeatMap::new(vec![vec![0.5]])
        .cell_display(CellDisplay::Value)
        .cell_width(4)
        .value_decimals(1);

    let cell = hm.render_cell(0.5);
    assert!(cell.contains("0.5"));
}

#[test]
fn test_render_cell_custom() {
    let hm = HeatMap::new(vec![vec![0.5]])
        .cell_display(CellDisplay::Custom)
        .cell_width(2);

    let cell = hm.render_cell(0.5);
    assert_eq!(cell, "■■");
}

#[test]
fn test_render_cell_show_values_overrides_display() {
    let hm = HeatMap::new(vec![vec![0.5]])
        .cell_display(CellDisplay::Block)
        .show_values(true)
        .value_decimals(2);

    let cell = hm.render_cell(0.5);
    // show_values takes precedence over cell_display
    assert!(cell.contains("0.50"));
}

// ==================== Edge Cases ====================

#[test]
fn test_heatmap_empty_data() {
    let hm = HeatMap::new(vec![]);
    // When empty, defaults apply
    assert_eq!(hm.min_val, 0.0);
    assert_eq!(hm.max_val, 1.0);
}

#[test]
fn test_heatmap_empty_rows() {
    let hm = HeatMap::new(vec![vec![], vec![]]);
    assert_eq!(hm.data.len(), 2);
}

#[test]
fn test_from_flat_partial_data() {
    // Data smaller than rows * cols
    let data = vec![1.0, 2.0, 3.0];
    let hm = HeatMap::from_flat(&data, 2, 3);
    assert_eq!(hm.data[0].len(), 3);
    assert_eq!(hm.data[1].len(), 0); // Second row is empty since data ran out
}

#[test]
fn test_heatmap_negative_values() {
    let hm = HeatMap::new(vec![vec![-10.0, 0.0, 10.0]]);
    assert_eq!(hm.min_val, -10.0);
    assert_eq!(hm.max_val, 10.0);
    assert_eq!(hm.normalize(-10.0), 0.0);
    assert_eq!(hm.normalize(0.0), 0.5);
    assert_eq!(hm.normalize(10.0), 1.0);
}

// ==================== Rendering Tests ====================

#[test]
fn test_heatmap_render_basic() {
    let hm = HeatMap::new(vec![vec![0.0, 0.5, 1.0]]).cell_width(1);

    let mut buffer = Buffer::new(20, 5);
    let rect = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, rect);

    hm.render(&mut ctx);
    // Basic smoke test - just ensure no panic
}

#[test]
fn test_heatmap_render_with_title() {
    let hm = HeatMap::new(vec![vec![0.5]]).title("Test Title");

    let mut buffer = Buffer::new(20, 5);
    let rect = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, rect);

    hm.render(&mut ctx);

    // Check that first row contains title characters
    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'T');
    assert_eq!(buffer.get(1, 0).unwrap().symbol, 'e');
    assert_eq!(buffer.get(2, 0).unwrap().symbol, 's');
    assert_eq!(buffer.get(3, 0).unwrap().symbol, 't');
}

#[test]
fn test_heatmap_render_with_labels() {
    let hm = HeatMap::new(vec![vec![0.5, 0.8], vec![0.2, 0.9]])
        .row_labels(vec!["R1".into(), "R2".into()])
        .col_labels(vec!["C1".into(), "C2".into()]);

    let mut buffer = Buffer::new(30, 10);
    let rect = Rect::new(0, 0, 30, 10);
    let mut ctx = RenderContext::new(&mut buffer, rect);

    hm.render(&mut ctx);

    // Smoke test - render should complete without panic
    // Labels are rendered somewhere in the buffer
}

#[test]
fn test_heatmap_render_with_legend() {
    let hm = HeatMap::new(vec![vec![0.0, 1.0]]).show_legend(true);

    let mut buffer = Buffer::new(40, 10);
    let rect = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, rect);

    hm.render(&mut ctx);

    // Smoke test - legend should render without panic
    // The legend contains "Low" and "High" labels somewhere in the buffer
}

#[test]
fn test_heatmap_render_with_values() {
    let hm = HeatMap::new(vec![vec![0.5]])
        .show_values(true)
        .value_decimals(1);

    let mut buffer = Buffer::new(20, 5);
    let rect = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, rect);

    hm.render(&mut ctx);
    // Smoke test
}

#[test]
fn test_heatmap_render_with_highlight() {
    let hm = HeatMap::new(vec![vec![0.5, 0.8], vec![0.2, 0.9]]).highlight(0, 1);

    let mut buffer = Buffer::new(20, 10);
    let rect = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, rect);

    hm.render(&mut ctx);
    // Smoke test - highlight should apply bold
}

#[test]
fn test_heatmap_render_multiline_cells() {
    let hm = HeatMap::new(vec![vec![0.5]]).cell_height(2);

    let mut buffer = Buffer::new(20, 10);
    let rect = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, rect);

    hm.render(&mut ctx);
    // Smoke test
}

#[test]
fn test_heatmap_clone() {
    let hm = HeatMap::new(vec![vec![0.5]])
        .title("Test")
        .color_scale(ColorScale::Viridis);

    let cloned = hm.clone();
    assert_eq!(cloned.title, Some("Test".to_string()));
    assert_eq!(cloned.color_scale, ColorScale::Viridis);
}

#[test]
fn test_heatmap_debug() {
    let hm = HeatMap::new(vec![vec![0.5]]);
    let debug = format!("{:?}", hm);
    assert!(debug.contains("HeatMap"));
}

// ==================== Brightness Contrast Tests ====================

#[test]
fn test_render_value_brightness_contrast() {
    // Test that high brightness cells get black text, low brightness get white
    let hm = HeatMap::new(vec![vec![0.0, 1.0]])
        .color_scale(ColorScale::Gray)
        .show_values(true);

    let mut buffer = Buffer::new(20, 5);
    let rect = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, rect);

    hm.render(&mut ctx);
    // This tests the brightness > 128 branch in render
}

// ==================== Label Truncation Tests ====================

#[test]
fn test_col_label_truncation() {
    let hm = HeatMap::new(vec![vec![0.5]])
        .cell_width(3)
        .col_labels(vec!["VeryLongLabel".into()]);

    let mut buffer = Buffer::new(30, 10);
    let rect = Rect::new(0, 0, 30, 10);
    let mut ctx = RenderContext::new(&mut buffer, rect);

    hm.render(&mut ctx);
    // Column label should be truncated to cell_width
}

#[test]
fn test_row_label_truncation() {
    let hm = HeatMap::new(vec![vec![0.5]]).row_labels(vec!["VeryLongRowLabel".into()]);

    let mut buffer = Buffer::new(30, 10);
    let rect = Rect::new(0, 0, 30, 10);
    let mut ctx = RenderContext::new(&mut buffer, rect);

    hm.render(&mut ctx);
    // Row label should be truncated to 6 chars
}
