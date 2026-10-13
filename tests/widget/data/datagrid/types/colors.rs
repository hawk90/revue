//! DataGrid GridColors tests

use revue::style::Color;
use revue::widget::data::datagrid::GridColors;

#[test]
fn test_grid_colors_new() {
    let colors = GridColors::new();
    assert_eq!(colors.header_bg, Color::rgb(60, 60, 80));
    assert_eq!(colors.header_fg, Color::WHITE);
}

#[test]
fn test_grid_colors_dark() {
    let colors = GridColors::dark();
    assert_eq!(colors.header_bg, Color::rgb(60, 60, 80));
    assert_eq!(colors.header_fg, Color::WHITE);
}

#[test]
fn test_grid_colors_light() {
    let colors = GridColors::light();
    assert_eq!(colors.header_bg, Color::rgb(220, 220, 230));
    assert_eq!(colors.header_fg, Color::BLACK);
    assert_eq!(colors.row_bg, Color::rgb(255, 255, 255));
}

#[test]
fn test_grid_colors_debug_clone() {
    let colors = GridColors::default();
    let cloned = colors.clone();
    assert_eq!(colors.header_bg, cloned.header_bg);
    let _ = format!("{:?}", colors);
}

#[test]
fn test_grid_colors_default() {
    let colors = GridColors::default();
    assert_eq!(colors.header_fg, Color::WHITE);
    assert_eq!(colors.selected_fg, Color::WHITE);
}
