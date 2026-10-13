//! Chart mark tests: lines, chart types, line styles, markers and Braille

use super::super::{count_char, render, render_rows, rows};
use super::{diagonal, flat, is_braille, plain, plot_line_cells, sym};
use revue::style::Color;
use revue::widget::data::chart::{Axis, Chart, ChartType, LineStyle, Marker, Series};

#[test]
fn test_chart_line_series() {
    // (0,0) -> (8,17), (10,10) -> (39,1); Y is padded to -0.5..10.5
    let buffer = render(&plain().series(diagonal()), 40, 20);
    assert_eq!(sym(&buffer, 8, 17), '╱');
    assert_eq!(sym(&buffer, 39, 1), '╱');
    // Bresenham visits max(dx, dy) + 1 cells
    assert_eq!(plot_line_cells(&buffer), 32);
    // Series default color
    assert_eq!(buffer.get(8, 17).unwrap().fg, Some(Color::WHITE));
}

#[test]
fn test_chart_descending_line() {
    let s = Series::new("D").data(vec![(0.0, 10.0), (10.0, 0.0)]);
    let buffer = render(&plain().series(s), 40, 20);
    assert_eq!(sym(&buffer, 8, 1), '╲');
    assert_eq!(sym(&buffer, 39, 17), '╲');
}

#[test]
fn test_chart_series_colors() {
    let chart = plain().series(diagonal().color(Color::RED)).series(
        Series::new("B")
            .data(vec![(0.0, 10.0), (10.0, 0.0)])
            .color(Color::BLUE),
    );
    let buffer = render(&chart, 40, 20);
    assert_eq!(buffer.get(8, 17).unwrap().fg, Some(Color::RED));
    assert_eq!(buffer.get(8, 1).unwrap().fg, Some(Color::BLUE));
}

#[test]
fn test_chart_empty_series_draws_nothing() {
    let chart = plain().series(Series::new("Empty")).series_vec(vec![]);
    assert_eq!(plot_line_cells(&render(&chart, 40, 20)), 0);
}

#[test]
fn test_chart_type_scatter_draws_no_lines() {
    let s = diagonal().chart_type(ChartType::Scatter);
    assert_eq!(plot_line_cells(&render(&plain().series(s), 40, 20)), 0);
}

#[test]
fn test_chart_type_area_fills_under_line() {
    let without_fill = render(
        &plain().series(diagonal().chart_type(ChartType::Area)),
        40,
        20,
    );
    assert_eq!(count_char(&without_fill, '░'), 0);
    assert_eq!(plot_line_cells(&without_fill), 32);

    let filled = render(&plain().series(diagonal().area(Color::GREEN)), 40, 20);
    let shades = count_char(&filled, '░') + count_char(&filled, '▒') + count_char(&filled, '▓');
    assert!(shades > 100, "{shades}");
    // The bottom-right corner is under the line; the top-left is not
    assert_ne!(sym(&filled, 38, 17), ' ');
    assert_eq!(sym(&filled, 9, 2), ' ');
    assert_eq!(filled.get(38, 17).unwrap().fg, Some(Color::GREEN));
}

#[test]
fn test_chart_type_step_after() {
    // Horizontal along the first point's row, then up at the second x
    let buffer = render(&plain().series(diagonal().step()), 40, 20);
    for x in 8..39 {
        assert_eq!(sym(&buffer, x, 17), '─', "x={x}");
    }
    for y in 1..17 {
        assert_eq!(sym(&buffer, 39, y), '│', "y={y}");
    }
}

#[test]
fn test_chart_type_step_before() {
    // Up at the first x, then horizontal along the second point's row
    let s = diagonal().chart_type(ChartType::StepBefore);
    let buffer = render(&plain().series(s), 40, 20);
    for y in 2..18 {
        assert_eq!(sym(&buffer, 8, y), '│', "y={y}");
    }
    for x in 9..40 {
        assert_eq!(sym(&buffer, x, 1), '─', "x={x}");
    }
}

#[test]
fn test_chart_line_styles() {
    // One 32-cell segment (the dash pattern restarts on every segment)
    let two_points = || Series::new("Flat").data(vec![(1.0, 5.0), (3.0, 5.0)]);
    let count = |style: LineStyle| {
        let buffer = render(&plain().series(two_points().line_style(style)), 40, 20);
        count_char(&buffer, '─')
    };
    assert_eq!(count(LineStyle::Solid), 32);
    // Dashed: 3 on, 3 off over 32 steps
    assert_eq!(count(LineStyle::Dashed), 17);
    // Dotted: every other step
    assert_eq!(count(LineStyle::Dotted), 16);
    assert_eq!(count(LineStyle::None), 0);
}

#[test]
fn test_chart_markers() {
    for marker in [Marker::Dot, Marker::Circle, Marker::Star, Marker::Square] {
        let s = flat().marker(marker).line_style(LineStyle::None);
        let buffer = render(&plain().series(s), 40, 20);
        assert_eq!(count_char(&buffer, marker.char()), 3, "{marker:?}");
    }
    // Marker::None draws nothing
    let s = flat().marker(Marker::None).line_style(LineStyle::None);
    let rows = rows(&render(&plain().series(s), 40, 20));
    assert!(rows[..18].iter().all(|r| r[8..].trim().is_empty()));
}

#[test]
fn test_chart_markers_drawn_over_lines() {
    let buffer = render(&plain().series(diagonal().marker(Marker::Circle)), 40, 20);
    assert_eq!(sym(&buffer, 8, 17), '○');
    assert_eq!(sym(&buffer, 39, 1), '○');
    assert_eq!(plot_line_cells(&buffer), 30);
}

#[test]
fn test_chart_braille_changes_rendering() {
    let normal = render_rows(&plain().series(diagonal()), 40, 20);
    let braille = render_rows(&plain().braille().series(diagonal()), 40, 20);
    assert_ne!(normal, braille);
}

#[test]
fn test_chart_braille_draws_lines_with_dots() {
    let buffer = render(
        &plain().braille().series(diagonal().color(Color::CYAN)),
        40,
        20,
    );
    // No box-drawing line cells; the line is made of Braille dots instead
    assert_eq!(plot_line_cells(&buffer), 0);
    // On a 64x72 dot grid the line starts at the bottom-left dot of the plot
    // (column 8, row 17) and climbs to the top-right cell, touching every row
    // on the way up (row 0 is the top padding)
    assert_eq!(sym(&buffer, 8, 17), '\u{2801}');
    assert!(is_braille(sym(&buffer, 39, 1)));
    for y in 1..18 {
        assert!(
            (8..40).any(|x| is_braille(sym(&buffer, x, y))),
            "row {y}: {:?}",
            rows(&buffer)[y as usize]
        );
    }
    assert_eq!(buffer.get(8, 17).unwrap().fg, Some(Color::CYAN));
}

#[test]
fn test_chart_braille_keeps_markers_and_clips() {
    // Markers stay whole-cell characters, and data past fixed bounds is
    // clipped at the plot edge as in cell mode
    let s = Series::new("Data")
        .data(vec![(-5.0, -5.0), (5.0, 5.0), (20.0, 20.0)])
        .marker(Marker::Star);
    let chart = plain()
        .braille()
        .series(s)
        .x_axis(Axis::new().bounds(0.0, 10.0).grid(false))
        .y_axis(Axis::new().bounds(0.0, 10.0).grid(false));
    let buffer = render(&chart, 40, 20);
    assert_eq!(count_char(&buffer, '★'), 1);
    assert_eq!(sym(&buffer, 23, 9), '★');
    assert!(is_braille(sym(&buffer, 8, 17)));
    assert!(is_braille(sym(&buffer, 39, 0)));
    assert_eq!(plot_line_cells(&buffer), 0);
}

#[test]
fn test_chart_mixed_series() {
    let chart = Chart::new()
        .title("Complex Test")
        .series(Series::new("Empty").data(vec![]))
        .series(Series::new("With NaN").data(vec![(1.0, 2.0), (f64::NAN, 3.0), (3.0, 4.0)]))
        .series(Series::new("Normal").data(vec![(1.0, 5.0), (2.0, 6.0), (3.0, 7.0)]));
    let text = render_rows(&chart, 60, 30).join("\n");
    assert!(text.contains("■ Empty"));
    assert!(text.contains("■ With NaN"));
    assert!(text.contains("■ Normal"));
}
