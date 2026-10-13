//! Chart frame tests: layout, title, border, background and the View impl

use super::super::{count_char, render, render_rows, rows};
use super::{diagonal, plain, sym};
use revue::style::Color;
use revue::widget::data::chart::{Axis, Chart, LegendPosition, LineStyle, Marker, Series};
use revue::widget::View;

#[test]
fn test_chart_view_meta() {
    assert_eq!(Chart::new().meta().widget_type, "Chart");
}

#[test]
fn test_chart_title() {
    let rows = render_rows(&plain().title("Test Chart"), 40, 20);
    // Centered: (40 - 10) / 2 = 15
    assert_eq!(rows[0].trim_end(), format!("{}Test Chart", " ".repeat(15)));
    let owned = render_rows(&plain().title(String::from("Test Chart")), 40, 20);
    assert_eq!(rows, owned);
}

#[test]
fn test_chart_title_shifts_plot_down() {
    // With a title the plot is rows 1..18 and the X labels stay on row 18
    let buffer = render(&plain().title("T").series(diagonal()), 40, 20);
    assert_eq!(sym(&buffer, 8, 18), '0');
    assert_eq!(sym(&buffer, 8, 17), '╱');
    assert_eq!(sym(&buffer, 39, 2), '╱');
}

#[test]
fn test_chart_too_small_renders_nothing() {
    let chart = Chart::new().series(diagonal());
    for (w, h) in [(9, 20), (40, 4), (10, 5)] {
        let buffer = render(&chart, w, h);
        assert!(rows(&buffer).iter().all(|r| r.trim().is_empty()), "{w}x{h}");
    }
}

#[test]
fn test_chart_bg() {
    let buffer = render(&plain().bg(Color::BLUE), 40, 20);
    assert_eq!(buffer.get(0, 0).unwrap().bg, Some(Color::BLUE));
    assert_eq!(buffer.get(39, 19).unwrap().bg, Some(Color::BLUE));
}

#[test]
fn test_chart_border() {
    let buffer = render(&plain().border(Color::GREEN).series(diagonal()), 40, 20);
    assert_eq!(sym(&buffer, 0, 0), '┌');
    assert_eq!(sym(&buffer, 39, 0), '┐');
    assert_eq!(sym(&buffer, 0, 19), '└');
    assert_eq!(sym(&buffer, 39, 19), '┘');
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::GREEN));
    // Plot moves in by one cell: x=9..39, y=1..17, labels on row 17
    assert_eq!(sym(&buffer, 9, 17), '0');
    assert_eq!(sym(&buffer, 9, 16), '╱');
}

#[test]
fn test_chart_full_featured() {
    let chart = Chart::new()
        .title("Full Featured Chart")
        .series(
            Series::new("Temperature")
                .data_y(&[20.0, 22.0, 25.0, 23.0, 21.0])
                .color(Color::RED)
                .marker(Marker::Circle),
        )
        .series(
            Series::new("Humidity")
                .data_y(&[60.0, 65.0, 70.0, 68.0, 62.0])
                .color(Color::BLUE)
                .line_style(LineStyle::Dashed)
                .marker(Marker::Dot),
        )
        .x_axis(Axis::new().bounds(0.0, 6.0))
        .y_axis(Axis::new().bounds(0.0, 100.0))
        .legend(LegendPosition::TopRight)
        .bg(Color::BLACK)
        .border(Color::WHITE);
    let buffer = render(&chart, 80, 40);
    let text = rows(&buffer).join("\n");
    assert!(text.contains("Full Featured Chart"));
    assert!(text.contains("■ Temperature"));
    assert!(text.contains("■ Humidity"));
    assert_eq!(count_char(&buffer, '○'), 5);
    assert_eq!(count_char(&buffer, '•'), 5);
}

#[test]
fn test_chart_large_area() {
    let s = diagonal().marker(Marker::Star);
    let buffer = render(&plain().series(s), 200, 100);
    assert_eq!(count_char(&buffer, '★'), 2);
}
