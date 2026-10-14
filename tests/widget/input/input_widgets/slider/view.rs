//! Slider on the screen: placement, each style's glyphs and colors, the
//! disabled and focused looks, the vertical value, and the fill

use proptest::prelude::*;
use revue::layout::Rect;
use revue::render::{Buffer, Modifier};
use revue::style::Color;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{slider, Slider, SliderStyle};

use super::render_rows;
use crate::traits::placement::{assert_same_when_offset, assert_zero_areas_draw_nothing};

const FILL: Color = Color::GREEN;
const TRACK: Color = Color::BLUE;
const KNOB: Color = Color::RED;

fn render(s: &Slider, w: u16, h: u16) -> Buffer {
    let mut buffer = Buffer::new(w, h);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, w, h));
    s.render(&mut ctx);
    buffer
}

/// A 5-cell track at 40%, in the test colors, with no value.
fn track(style: SliderStyle) -> Slider {
    slider()
        .style(style)
        .length(5)
        .value(40.0)
        .show_value(false)
        .fill_color(FILL)
        .track_color(TRACK)
        .knob_color(KNOB)
}

fn fgs(buffer: &Buffer, y: u16) -> Vec<Option<Color>> {
    (0..buffer.width())
        .map(|x| buffer.get(x, y).unwrap().fg)
        .collect()
}

#[test]
fn test_slider_zero_areas_draw_nothing() {
    let s = slider().label("Vol").ticks(3).value(50.0);
    assert_zero_areas_draw_nothing(&s);
    assert_zero_areas_draw_nothing(&s.vertical());
}

#[test]
fn test_slider_same_when_offset() {
    let s = slider().label("Vol").length(6).ticks(3).value(50.0);
    assert_same_when_offset(s.clone(), 14, 2);
    assert_same_when_offset(s.vertical().focused(true), 5, 6);
}

#[test]
fn test_slider_horizontal_styles_glyphs_and_colors() {
    let (f, t, k) = (Some(FILL), Some(TRACK), Some(KNOB));
    // 40% of the last cell's index (4) rounds to cell 2
    for (style, glyphs, colors) in [
        (SliderStyle::Block, "███░░", [f, f, f, t, t]),
        (SliderStyle::Dots, "●●●○○", [f, f, f, t, t]),
        (SliderStyle::Line, "━━●━━", [f, f, k, t, t]),
        // Thin draws no fill
        (SliderStyle::Thin, "──┃──", [t, t, k, t, t]),
        // Gradient ramps up to the value: cell 0 is the lightest
        (SliderStyle::Gradient, "░▒█░░", [f, f, f, t, t]),
    ] {
        let buffer = render(&track(style), 5, 1);
        assert_eq!(render_rows(&track(style), 5, 1)[0], glyphs, "{style:?}");
        assert_eq!(fgs(&buffer, 0), colors, "{style:?}");
    }
}

#[test]
fn test_slider_vertical_styles_glyphs_and_colors() {
    let (f, t, k) = (Some(FILL), Some(TRACK), Some(KNOB));
    // Top to bottom; the fill grows from the bottom
    for (style, glyphs, colors) in [
        (
            SliderStyle::Block,
            ["░", "░", "█", "█", "█"],
            [t, t, f, f, f],
        ),
        (
            SliderStyle::Line,
            ["│", "│", "●", "│", "│"],
            [t, t, k, f, f],
        ),
        (
            SliderStyle::Dots,
            ["○", "○", "●", "●", "●"],
            [t, t, f, f, f],
        ),
    ] {
        let s = track(style).vertical();
        let buffer = render(&s, 1, 5);
        assert_eq!(render_rows(&s, 1, 5), glyphs, "{style:?}");
        let column: Vec<_> = (0..5).map(|y| buffer.get(0, y).unwrap().fg).collect();
        assert_eq!(column, colors, "{style:?}");
    }
}

#[test]
fn test_slider_disabled_dims_the_label_and_the_track() {
    let s = track(SliderStyle::Line).label("L").disabled(true);
    let buffer = render(&s, 7, 1);
    assert_eq!(render_rows(&s, 7, 1)[0], "L ━━●━━");
    let label = buffer.get(0, 0).unwrap().fg;
    assert!(label.is_some());
    assert_ne!(label, Some(Color::WHITE));
    // Fill, knob and track alike: one gray, none of the set colors
    let cells: Vec<_> = (2..7).map(|x| buffer.get(x, 0).unwrap().fg).collect();
    assert!(cells.iter().all(|&c| c == cells[0]), "{cells:?}");
    assert!(![Some(FILL), Some(TRACK), Some(KNOB)].contains(&cells[0]));

    let enabled = render(&track(SliderStyle::Line).label("L"), 7, 1);
    assert_eq!(enabled.get(0, 0).unwrap().fg, Some(Color::WHITE));
}

#[test]
fn test_slider_focused_value_is_cyan_and_bold() {
    let s = slider().length(3).value(50.0);
    let buffer = render(&s, 6, 1);
    assert_eq!(render_rows(&s, 6, 1)[0], "██░ 50");
    let cell = buffer.get(4, 0).unwrap();
    assert_eq!(cell.fg, Some(Color::WHITE));
    assert!(!cell.modifier.contains(Modifier::BOLD));

    let buffer = render(&s.focused(true), 6, 1);
    for x in 4..6 {
        let cell = buffer.get(x, 0).unwrap();
        assert_eq!(cell.fg, Some(Color::CYAN), "{x}");
        assert!(cell.modifier.contains(Modifier::BOLD), "{x}");
    }
}

#[test]
fn test_slider_value_clipped_at_the_right_edge() {
    let s = slider().length(3).value(100.0);
    assert_eq!(render_rows(&s, 6, 1)[0], "███ 10");
}

#[test]
fn test_slider_vertical_value_beside_the_middle_row() {
    let s = slider().vertical().length(5).value(40.0);
    assert_eq!(
        render_rows(&s, 5, 5),
        ["░    ", "░    ", "█ 40 ", "█    ", "█    "]
    );
    // Too narrow for the gap and a digit: no value
    assert_eq!(render_rows(&s, 2, 5)[2], "█ ");
}

#[test]
fn test_slider_ticks_follow_the_track() {
    let s = slider().label("ab").length(9).ticks(5).show_value(false);
    let rows = render_rows(&s, 12, 2);
    // The track starts after "ab "; ticks at its ends and every two cells
    assert_eq!(rows[0], format!("ab █{}", "░".repeat(8)));
    assert_eq!(rows[1], "   ┴ ┴ ┴ ┴ ┴");
    // No second row, no ticks
    assert_eq!(render_rows(&s, 12, 1).len(), 1);
}

#[test]
fn test_slider_flat_range_draws_the_minimum() {
    let s = slider().range(5.0, 5.0).value(5.0).length(4);
    assert_eq!(render_rows(&s, 8, 1)[0], "█░░░ 5.0");
}

fn style() -> impl Strategy<Value = SliderStyle> {
    prop_oneof![
        Just(SliderStyle::Block),
        Just(SliderStyle::Line),
        Just(SliderStyle::Thin),
        Just(SliderStyle::Gradient),
        Just(SliderStyle::Dots),
    ]
}

proptest! {
    /// The painted cells fit the measured size, for any style, value,
    /// length and orientation. (ASCII labels: wide ones move the ticks,
    /// #1010.)
    #[test]
    fn prop_slider_draws_within_its_measure(
        style in style(),
        vertical in any::<bool>(),
        value in 0.0f64..=100.0,
        length in 0u16..30,
        label in proptest::option::of("[a-z]{0,6}"),
        ticks in proptest::option::of(2u16..8),
        show_value in any::<bool>(),
    ) {
        let mut s = slider().style(style).value(value).length(length).show_value(show_value);
        if vertical {
            s = s.vertical();
        }
        if let Some(l) = label {
            s = s.label(l);
        }
        if let Some(n) = ticks {
            s = s.ticks(n);
        }
        let (w, h) = s.measure(60, 40).unwrap();
        let buffer = render(&s, 60, 40);
        for y in 0..40 {
            for x in 0..60 {
                let cell = buffer.get(x, y).unwrap();
                if x >= w || y >= h {
                    prop_assert_eq!(cell.symbol, ' ', "({}, {}) outside {}x{}", x, y, w, h);
                }
            }
        }
    }

    /// A larger value never fills fewer cells, and the minimum and the
    /// maximum fill the first and every cell.
    #[test]
    fn prop_slider_fill_grows_with_the_value(
        style in prop_oneof![Just(SliderStyle::Block), Just(SliderStyle::Dots)],
        vertical in any::<bool>(),
        a in 0.0f64..=100.0,
        b in 0.0f64..=100.0,
        length in 3u16..30,
    ) {
        let filled = |v: f64| {
            let mut s = slider().style(style).value(v).length(length).show_value(false).fill_color(FILL);
            if vertical {
                s = s.vertical();
            }
            let buffer = render(&s, 30, 30);
            (0..30)
                .flat_map(|y| (0..30).map(move |x| (x, y)))
                .filter(|&(x, y)| buffer.get(x, y).unwrap().fg == Some(FILL))
                .count()
        };
        let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
        prop_assert!(filled(lo) <= filled(hi));
        prop_assert_eq!(filled(0.0), 1);
        prop_assert_eq!(filled(100.0), length as usize);
    }
}
