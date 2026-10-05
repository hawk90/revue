//! Sparkline widget tests

use super::{render, render_rows};
use revue::style::Color;
use revue::widget::data::chart::{sparkline, Sparkline, SparklineStyle};

/// The sparkline's single row as text.
fn line(sl: &Sparkline, width: u16) -> String {
    render_rows(sl, width, 1).remove(0)
}

// =========================================================================
// Data
// =========================================================================

#[test]
fn test_sparkline_new_and_get_data() {
    let sl = Sparkline::new(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    assert_eq!(sl.get_data(), &[1.0, 2.0, 3.0, 4.0, 5.0]);
}

#[test]
fn test_sparkline_empty_and_default() {
    assert!(Sparkline::empty().get_data().is_empty());
    assert!(Sparkline::default().get_data().is_empty());
}

#[test]
fn test_sparkline_data_builder() {
    let sl = Sparkline::empty().data(vec![1.0, 2.0, 3.0]);
    assert_eq!(sl.get_data(), &[1.0, 2.0, 3.0]);
    // Replaces, does not append
    let sl = sl.data((0..2).map(|i| i as f64 * 10.0));
    assert_eq!(sl.get_data(), &[0.0, 10.0]);
}

#[test]
fn test_sparkline_helper_and_iterator() {
    let sl = sparkline([1.0, 2.0, 3.0].iter().copied());
    assert_eq!(sl.get_data(), &[1.0, 2.0, 3.0]);
}

#[test]
fn test_sparkline_push() {
    let mut sl = Sparkline::new(vec![1.0, 2.0]);
    sl.push(3.0);
    assert_eq!(sl.get_data(), &[1.0, 2.0, 3.0]);
}

#[test]
fn test_sparkline_push_shift() {
    let mut sl = Sparkline::new(vec![1.0, 2.0, 3.0]);
    sl.push_shift(4.0, 3);
    assert_eq!(sl.get_data(), &[2.0, 3.0, 4.0]);

    // Drops as many as needed to get back to max_len
    let mut sl = Sparkline::new(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    sl.push_shift(6.0, 2);
    assert_eq!(sl.get_data(), &[5.0, 6.0]);

    // Under the limit nothing is dropped
    let mut sl = Sparkline::new(vec![1.0, 2.0]);
    sl.push_shift(3.0, 5);
    assert_eq!(sl.get_data(), &[1.0, 2.0, 3.0]);
}

#[test]
fn test_sparkline_clear() {
    let mut sl = Sparkline::new(vec![1.0, 2.0, 3.0]);
    sl.clear();
    assert!(sl.get_data().is_empty());
}

#[test]
fn test_sparkline_clone() {
    let sl1 = Sparkline::new(vec![1.0, 2.0, 3.0]).style(SparklineStyle::Ascii);
    let sl2 = sl1.clone();
    assert_eq!(sl1.get_data(), sl2.get_data());
    assert_eq!(line(&sl1, 5), line(&sl2, 5));
}

// =========================================================================
// Rendering
// =========================================================================

#[test]
fn test_sparkline_render_scales_from_zero() {
    // Lower bound defaults to min(data, 0); 50 of 100 rounds to level 4
    assert_eq!(line(&Sparkline::new(vec![0.0, 50.0, 100.0]), 3), "▁▅█");
    assert_eq!(line(&Sparkline::new(vec![1.0, 5.0, 3.0]), 3), "▂█▅");
}

#[test]
fn test_sparkline_pads_short_data() {
    let sl = Sparkline::new(vec![0.0, 50.0, 100.0]);
    let buffer = render(&sl, 5, 1);
    assert_eq!(super::rows(&buffer)[0], "▁▅█▁▁");
    // Padding is dimmed, data uses the default fg
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::CYAN));
    assert_eq!(buffer.get(4, 0).unwrap().fg, Some(Color::rgb(50, 50, 50)));
}

#[test]
fn test_sparkline_shows_latest_data_when_too_long() {
    let data: Vec<f64> = (0..1000).map(|i| i as f64).collect();
    let sl = Sparkline::new(data);
    // The last 3 values (997..999) are all near the max
    assert_eq!(line(&sl, 3), "███");
}

#[test]
fn test_sparkline_custom_bounds() {
    let sl = Sparkline::new(vec![1.0, 5.0, 3.0]).min(-10.0).max(10.0);
    assert_eq!(line(&sl, 3), "▅▆▆");
}

#[test]
fn test_sparkline_values_outside_bounds_are_clamped() {
    let sl = Sparkline::new(vec![200.0, -50.0, 50.0]).min(0.0).max(100.0);
    assert_eq!(line(&sl, 3), "█▁▅");
}

#[test]
fn test_sparkline_flat_data() {
    // Lower bound is 0, so equal positive values are full height
    assert_eq!(line(&Sparkline::new(vec![5.0, 5.0, 5.0]), 3), "███");
    // All zeros: the range is widened to -1..1, putting 0 in the middle
    assert_eq!(line(&Sparkline::new(vec![0.0, 0.0]), 2), "▅▅");
}

#[test]
fn test_sparkline_negative_values() {
    assert_eq!(line(&Sparkline::new(vec![-10.0, 0.0, 10.0]), 3), "▁▅█");
}

#[test]
fn test_sparkline_nan_is_lowest() {
    // NaN is ignored for the bounds and drawn as the lowest level
    assert_eq!(line(&Sparkline::new(vec![1.0, f64::NAN, 3.0]), 3), "▃▁█");
}

#[test]
fn test_sparkline_styles() {
    let data = vec![0.0, 50.0, 100.0];
    assert_eq!(SparklineStyle::default(), SparklineStyle::Block);
    assert_eq!(line(&Sparkline::new(data.clone()), 3), "▁▅█");
    let ascii = Sparkline::new(data.clone()).style(SparklineStyle::Ascii);
    assert_eq!(line(&ascii, 3), "_+@");
    let braille = Sparkline::new(data).style(SparklineStyle::Braille);
    assert_eq!(line(&braille, 3), "⠀⣿⣿");
}

#[test]
fn test_sparkline_all_levels() {
    let data: Vec<f64> = (0..8).map(|i| i as f64).collect();
    let line_of = |style| line(&Sparkline::new(data.clone()).style(style), 8);
    assert_eq!(line_of(SparklineStyle::Block), "▁▂▃▄▅▆▇█");
    assert_eq!(line_of(SparklineStyle::Ascii), "_.-=+*#@");
    assert_eq!(line_of(SparklineStyle::Braille), "⠀⣀⣤⣶⣿⣿⣿⣿");
}

#[test]
fn test_sparkline_colors() {
    let sl = Sparkline::new(vec![1.0, 2.0])
        .fg(Color::GREEN)
        .bg(Color::BLACK);
    let buffer = render(&sl, 3, 1);
    for x in 0..2 {
        assert_eq!(buffer.get(x, 0).unwrap().fg, Some(Color::GREEN));
        assert_eq!(buffer.get(x, 0).unwrap().bg, Some(Color::BLACK));
    }
    // Padding keeps the bg
    assert_eq!(buffer.get(2, 0).unwrap().bg, Some(Color::BLACK));
}

#[test]
fn test_sparkline_show_bounds() {
    let sl = Sparkline::new(vec![1.0, 5.0, 3.0]).show_bounds(true);
    assert_eq!(line(&sl, 5), "5 ▂█▅");
    // The prefix takes columns away from the data, which keeps the latest
    assert_eq!(line(&sl, 4), "5 █▅");
    // Width is enough for the label only
    assert_eq!(line(&sl, 1), "5");
    // Custom max shows in the label
    assert_eq!(line(&sl.max(100.0), 6), "100 ▁▁");
}

#[test]
fn test_sparkline_only_first_row() {
    let rows = render_rows(&Sparkline::new(vec![1.0, 2.0]), 4, 3);
    assert_eq!(rows[0], "▅█▁▁");
    assert!(rows[1..].iter().all(|r| r.trim().is_empty()));
}

#[test]
fn test_sparkline_zero_area() {
    let sl = Sparkline::new(vec![1.0, 2.0]);
    assert_eq!(render(&sl, 0, 1).width(), 0);
    assert_eq!(render(&sl, 4, 0).height(), 0);
}
