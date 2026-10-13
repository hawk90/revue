//! ColorScale and CellDisplay tests

use revue::style::Color;
use revue::widget::data::chart::{CellDisplay, ColorScale, HeatMap};

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

#[test]
fn test_cell_display_default() {
    assert_eq!(CellDisplay::default(), CellDisplay::Block);
}
