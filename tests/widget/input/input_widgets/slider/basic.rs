//! Slider construction and rendering

use super::render_rows;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::RenderContext;
use revue::widget::{
    percentage_slider, slider, slider_range, volume_slider, Slider, SliderStyle, View,
};

#[test]
fn test_slider_new_starts_at_min_of_0_to_100() {
    let mut s = slider();
    assert_eq!(s.get_value(), 0.0);
    s.set_max();
    assert_eq!(s.get_value(), 100.0);
    assert_eq!(Slider::default().get_value(), 0.0);
}

#[test]
fn test_slider_value_builder() {
    assert_eq!(slider().value(42.0).get_value(), 42.0);
}

#[test]
fn test_slider_range_helper() {
    let mut s = slider_range(-10.0, 10.0);
    // The old value (0) is inside the new range, so it is kept
    assert_eq!(s.get_value(), 0.0);
    s.set_min();
    assert_eq!(s.get_value(), -10.0);
    s.set_max();
    assert_eq!(s.get_value(), 10.0);
}

#[test]
fn test_slider_widget_type() {
    assert_eq!(slider().widget_type(), "Slider");
}

#[test]
fn test_slider_renders_block_track_and_value() {
    let s = slider().value(50.0);
    let rows = render_rows(&s, 30, 1);
    // Default length 20: half filled (knob position rounds 9.5 up to 10)
    assert_eq!(
        rows[0],
        format!("{}{} 50       ", "█".repeat(11), "░".repeat(9))
    );
}

#[test]
fn test_slider_renders_line_style_knob() {
    let s = slider()
        .value(0.0)
        .length(5)
        .style(SliderStyle::Line)
        .show_value(false);
    assert_eq!(render_rows(&s, 5, 1)[0], "●━━━━");
}

#[test]
fn test_slider_renders_dots_style() {
    let s = slider()
        .value(100.0)
        .length(4)
        .style(SliderStyle::Dots)
        .show_value(false);
    assert_eq!(render_rows(&s, 4, 1)[0], "●●●●");
}

#[test]
fn test_slider_renders_label_before_track() {
    let s = slider().label("Vol").length(4).show_value(false);
    assert_eq!(render_rows(&s, 10, 1)[0], "Vol █░░░  ");
}

#[test]
fn test_slider_value_format() {
    let s = slider().value(25.0).length(3).value_format("{pct}%");
    assert_eq!(render_rows(&s, 8, 1)[0], "██░ 25% ");
}

#[test]
fn test_slider_small_range_shows_one_decimal() {
    let s = slider_range(0.0, 1.0).value(0.5).length(3);
    assert!(render_rows(&s, 8, 1)[0].contains("0.5"));
}

#[test]
fn test_slider_ticks_render_on_second_row() {
    let s = slider().length(5).ticks(3).show_value(false);
    let rows = render_rows(&s, 5, 2);
    assert_eq!(rows[1], "┴ ┴ ┴");
}

#[test]
fn test_slider_vertical_fills_from_bottom() {
    let s = slider()
        .vertical()
        .length(4)
        .value(100.0 / 3.0)
        .show_value(false);
    let rows = render_rows(&s, 1, 4);
    assert_eq!(rows, vec!["░", "░", "█", "█"]);
}

#[test]
fn test_slider_label_wider_than_area_renders_without_track() {
    // The label consumes the whole width, leaving no room for a track.
    let s = slider().label("A long label");
    let rows = render_rows(&s, 6, 1);
    assert_eq!(rows[0], "A long");
}

#[test]
fn test_slider_length_has_a_minimum_of_three() {
    let s = slider().length(0).value(7.0);
    assert_eq!(render_rows(&s, 6, 1)[0], "█░░ 7 ");
}

#[test]
fn test_slider_vertical_in_zero_height_area_draws_nothing() {
    let s = slider().vertical();
    let mut buffer = Buffer::new(3, 2);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 3, 0));
    s.render(&mut ctx);
    for y in 0..2 {
        for x in 0..3 {
            assert_eq!(buffer.get(x, y).unwrap().symbol, ' ');
        }
    }
}

#[test]
fn test_percentage_slider_formats_with_percent_sign() {
    let s = percentage_slider().value(30.0).length(3);
    assert_eq!(render_rows(&s, 10, 1)[0], "██░ 30.0% ");
}

#[test]
fn test_volume_slider_has_vol_label() {
    let s = volume_slider().length(3).show_value(false);
    assert_eq!(render_rows(&s, 7, 1)[0], "Vol █░░");
}
