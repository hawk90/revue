//! Sparkline widget tests extracted from chart/sparkline.rs

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::data::chart::sparkline::{Sparkline, SparklineStyle};
use revue::style::Color;

#[test]
fn test_sparkline_new() {
    let sl = Sparkline::new(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    assert_eq!(sl.data.len(), 5);
}

#[test]
fn test_sparkline_empty() {
    let sl = Sparkline::empty();
    assert!(sl.data.is_empty());
}

#[test]
fn test_sparkline_builder() {
    let sl = Sparkline::new(vec![1.0, 2.0])
        .max(10.0)
        .min(0.0)
        .style(SparklineStyle::Ascii)
        .fg(Color::GREEN)
        .bg(Color::BLACK)
        .show_bounds(true);

    assert_eq!(sl.max, Some(10.0));
    assert_eq!(sl.min, Some(0.0));
    assert_eq!(sl.style, SparklineStyle::Ascii);
    assert_eq!(sl.fg, Some(Color::GREEN));
    assert_eq!(sl.bg, Some(Color::BLACK));
    assert!(sl.show_bounds);
}

#[test]
fn test_sparkline_push() {
    let mut sl = Sparkline::new(vec![1.0, 2.0]);
    sl.push(3.0);
    assert_eq!(sl.data.len(), 3);
    assert_eq!(sl.data[2], 3.0);
}

#[test]
fn test_sparkline_push_shift() {
    let mut sl = Sparkline::new(vec![1.0, 2.0, 3.0]);
    sl.push_shift(4.0, 3);
    assert_eq!(sl.data.len(), 3);
    assert_eq!(sl.data, vec![2.0, 3.0, 4.0]);
}

#[test]
fn test_sparkline_clear() {
    let mut sl = Sparkline::new(vec![1.0, 2.0, 3.0]);
    sl.clear();
    assert!(sl.data.is_empty());
}

#[test]
fn test_sparkline_calc_bounds() {
    let sl = Sparkline::new(vec![1.0, 5.0, 3.0]);
    let (min, max) = sl.calc_bounds();
    assert_eq!(min, 0.0); // Default min is 0
    assert_eq!(max, 5.0);
}

#[test]
fn test_sparkline_calc_bounds_custom() {
    let sl = Sparkline::new(vec![1.0, 5.0, 3.0]).min(-10.0).max(10.0);
    let (min, max) = sl.calc_bounds();
    assert_eq!(min, -10.0);
    assert_eq!(max, 10.0);
}

#[test]
fn test_sparkline_value_to_index() {
    let sl = Sparkline::new(vec![0.0, 100.0]);

    // With bounds 0-100
    assert_eq!(sl.value_to_index(0.0, 0.0, 100.0), 0);
    assert_eq!(sl.value_to_index(100.0, 0.0, 100.0), 7);
    assert_eq!(sl.value_to_index(50.0, 0.0, 100.0), 4);
}

#[test]
fn test_sparkline_styles() {
    let block = SparklineStyle::Block;
    assert_eq!(block.chars().len(), 8);
    assert_eq!(block.chars()[0], '▁');
    assert_eq!(block.chars()[7], '█');

    let ascii = SparklineStyle::Ascii;
    assert_eq!(ascii.chars()[0], '_');
    assert_eq!(ascii.chars()[7], '@');
}

#[test]
fn test_sparkline_render() {
    let sl = Sparkline::new(vec![1.0, 4.0, 2.0, 8.0, 5.0, 3.0]);
    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(1, 1, 15, 1);
    let mut ctx = revue::render::RenderContext::new(&mut buffer, area);

    sl.render(&mut ctx);
    // Should render sparkline bars
}

#[test]
fn test_sparkline_render_with_bounds() {
    let sl = Sparkline::new(vec![1.0, 5.0, 3.0]).show_bounds(true);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(1, 1, 25, 1);
    let mut ctx = revue::render::RenderContext::new(&mut buffer, area);

    sl.render(&mut ctx);
    // Should render with bounds label
}

#[test]
fn test_sparkline_helper() {
    let sl = revue::widget::data::chart::sparkline::sparkline(vec![1.0, 2.0, 3.0]);
    assert_eq!(sl.data.len(), 3);
}

#[test]
fn test_sparkline_get_data() {
    let sl = Sparkline::new(vec![1.0, 2.0, 3.0]);
    assert_eq!(sl.get_data(), &[1.0, 2.0, 3.0]);
}

#[test]
fn test_sparkline_flat_data() {
    // All same values
    let sl = Sparkline::new(vec![5.0, 5.0, 5.0, 5.0]);
    let (min, max) = sl.calc_bounds();
    // Should handle flat data gracefully
    assert!(max > min);
}

// =========================================================================
// SparklineStyle enum tests
// =========================================================================

#[test]
fn test_sparkline_style_default() {
    assert_eq!(SparklineStyle::default(), SparklineStyle::Block);
}

#[test]
fn test_sparkline_style_clone() {
    let style = SparklineStyle::Braille;
    assert_eq!(style, style.clone());
}

#[test]
fn test_sparkline_style_copy() {
    let style1 = SparklineStyle::Ascii;
    let style2 = style1;
    assert_eq!(style1, SparklineStyle::Ascii);
    assert_eq!(style2, SparklineStyle::Ascii);
}

#[test]
fn test_sparkline_style_equality() {
    assert_eq!(SparklineStyle::Block, SparklineStyle::Block);
    assert_eq!(SparklineStyle::Braille, SparklineStyle::Braille);
    assert_ne!(SparklineStyle::Block, SparklineStyle::Ascii);
}

#[test]
fn test_sparkline_style_debug() {
    let debug_str = format!("{:?}", SparklineStyle::Block);
    assert!(debug_str.contains("Block"));
}

#[test]
fn test_sparkline_style_chars_block() {
    let chars = SparklineStyle::Block.chars();
    assert_eq!(chars.len(), 8);
    assert_eq!(chars[0], '▁');
    assert_eq!(chars[7], '█');
}

#[test]
fn test_sparkline_style_chars_braille() {
    let chars = SparklineStyle::Braille.chars();
    assert_eq!(chars.len(), 8);
    assert_eq!(chars[0], '⠀');
}

#[test]
fn test_sparkline_style_chars_ascii() {
    let chars = SparklineStyle::Ascii.chars();
    assert_eq!(chars.len(), 8);
    assert_eq!(chars[0], '_');
    assert_eq!(chars[7], '@');
}

// =========================================================================
// Sparkline struct tests
// =========================================================================

#[test]
fn test_sparkline_default() {
    let sl = Sparkline::default();
    assert!(sl.data.is_empty());
}

#[test]
fn test_sparkline_clone() {
    let sl1 = Sparkline::new(vec![1.0, 2.0, 3.0]);
    let sl2 = sl1.clone();
    assert_eq!(sl1.data, sl2.data);
}

#[test]
fn test_sparkline_data_builder() {
    let sl = Sparkline::empty().data(vec![1.0, 2.0, 3.0]);
    assert_eq!(sl.data.len(), 3);
}

#[test]
fn test_sparkline_data_from_iterator() {
    let data = vec![1.0, 2.0, 3.0];
    let sl = Sparkline::new(data.iter().copied());
    assert_eq!(sl.data.len(), 3);
}

#[test]
fn test_sparkline_negative_values() {
    let sl = Sparkline::new(vec![-5.0, 0.0, 5.0]);
    let (min, max) = sl.calc_bounds();
    assert_eq!(min, -5.0);
    assert_eq!(max, 5.0);
}

#[test]
fn test_sparkline_all_zeros() {
    let sl = Sparkline::new(vec![0.0, 0.0, 0.0]);
    let index = sl.value_to_index(0.0, -1.0, 1.0);
    // Should handle zero range gracefully
    assert!(index <= 7);
}

#[test]
fn test_sparkline_clamp_to_range() {
    let sl = Sparkline::new(vec![0.0, 100.0]);
    // Value above max should clamp to 7
    assert_eq!(sl.value_to_index(200.0, 0.0, 100.0), 7);
    // Value below min should clamp to 0
    assert_eq!(sl.value_to_index(-50.0, 0.0, 100.0), 0);
}

#[test]
fn test_sparkline_single_value() {
    let sl = Sparkline::new(vec![42.0]);
    let (min, max) = sl.calc_bounds();
    // Should create a range even with one value
    assert!(max > min);
}

#[test]
fn test_sparkline_push_shift_exact_limit() {
    let mut sl = Sparkline::new(vec![1.0, 2.0, 3.0]);
    sl.push_shift(4.0, 3);
    assert_eq!(sl.data.len(), 3);
    assert_eq!(sl.data, vec![2.0, 3.0, 4.0]);
}

#[test]
fn test_sparkline_push_shift_under_limit() {
    let mut sl = Sparkline::new(vec![1.0, 2.0]);
    sl.push_shift(3.0, 5);
    assert_eq!(sl.data.len(), 3);
    assert_eq!(sl.data, vec![1.0, 2.0, 3.0]);
}

#[test]
fn test_sparkline_data_mutator() {
    let mut sl = Sparkline::empty();
    sl.data = vec![5.0, 4.0, 3.0];
    assert_eq!(sl.get_data(), &[5.0, 4.0, 3.0]);
}

#[test]
fn test_sparkline_show_bounds_builder() {
    let sl = Sparkline::new(vec![1.0]).show_bounds(true);
    assert!(sl.show_bounds);
}

#[test]
fn test_sparkline_builder_chain() {
    let sl = Sparkline::new(vec![1.0, 2.0, 3.0])
        .max(10.0)
        .min(0.0)
        .style(SparklineStyle::Braille)
        .fg(Color::RED)
        .bg(Color::WHITE)
        .show_bounds(true);

    assert_eq!(sl.max, Some(10.0));
    assert_eq!(sl.min, Some(0.0));
    assert_eq!(sl.style, SparklineStyle::Braille);
    assert_eq!(sl.fg, Some(Color::RED));
    assert_eq!(sl.bg, Some(Color::WHITE));
    assert!(sl.show_bounds);
}

#[test]
fn test_sparkline_single_element() {
    let sl = Sparkline::new(vec![7.5]);
    assert_eq!(sl.data.len(), 1);
    assert_eq!(sl.data[0], 7.5);
}

#[test]
fn test_sparkline_large_dataset() {
    let data: Vec<f64> = (0..1000).map(|i| i as f64).collect();
    let sl = Sparkline::new(data);
    assert_eq!(sl.data.len(), 1000);
}

#[test]
fn test_sparkline_nan_handling() {
    let sl = Sparkline::new(vec![1.0, f64::NAN, 3.0]);
    // NaN should be handled in calculations
    let (min, _max) = sl.calc_bounds();
    // min/max should skip NaN
    assert!(min.is_finite());
}

#[test]
fn test_sparkline_infinity_handling() {
    let sl = Sparkline::new(vec![1.0, f64::INFINITY, 3.0]);
    let (min, max) = sl.calc_bounds();
    // calc_bounds uses data_min.min(0.0), so min is 0.0 even when data_min is 1.0
    assert_eq!(min, 0.0);
    assert!(max.is_infinite());
}