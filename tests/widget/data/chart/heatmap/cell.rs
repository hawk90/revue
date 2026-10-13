//! HeatMap cell tests: normalization, colors, cell text and size, values and highlight

use super::super::{render, render_rows, rows};
use super::fg;
use proptest::prelude::*;
use revue::render::Modifier;
use revue::style::Color;
use revue::widget::data::chart::{heatmap, CellDisplay, ColorScale, HeatMap};

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

#[test]
fn test_highlight() {
    let hm = HeatMap::new(vec![vec![0.5, 0.6], vec![0.7, 0.8]]).highlight(1, 0);
    let buffer = render(&hm, 4, 2);
    let bold = |x, y| buffer.get(x, y).unwrap().modifier.contains(Modifier::BOLD);
    assert!(bold(0, 1) && bold(1, 1));
    assert!(!bold(2, 1));
    assert!(!bold(0, 0));
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    /// Every cell of any grid is drawn at its row and column, in the color
    /// `color_for` gives its value.
    #[test]
    fn test_heatmap_cells_match_color_for(
        grid in (1usize..6, 1usize..7).prop_flat_map(|(r, c)| {
            prop::collection::vec(prop::collection::vec(-1e3f64..1e3, c), r)
        }),
    ) {
        let hm = heatmap(grid.clone());
        let (rows_n, cols_n) = (grid.len() as u16, grid[0].len() as u16);
        let buffer = render(&hm, cols_n * 2 + 3, rows_n + 2);
        for (r, row) in grid.iter().enumerate() {
            for (c, &value) in row.iter().enumerate() {
                for dx in 0..2 {
                    let (x, y) = (c as u16 * 2 + dx, r as u16);
                    prop_assert_eq!(buffer.get(x, y).unwrap().symbol, '█');
                    prop_assert_eq!(fg(&buffer, x, y), Some(hm.color_for(value)), "({}, {})", x, y);
                }
            }
        }
        // Nothing to the right of or below the grid
        for y in 0..rows_n + 2 {
            for x in 0..cols_n * 2 + 3 {
                if x >= cols_n * 2 || y >= rows_n {
                    prop_assert_eq!(buffer.get(x, y).unwrap().symbol, ' ');
                }
            }
        }
    }
}
