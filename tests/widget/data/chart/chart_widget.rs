//! Chart (line/area/step/scatter) widget tests
//!
//! Layout used below: a 40x20 chart without border or title has its plot
//! area at x=8..40, y=0..18, Y labels in columns 0..8 and X labels on
//! row 18.

use super::{count_char, render, render_rows, rows};
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::data::chart::{
    chart, line_chart, scatter_plot, Axis, AxisFormat, Chart, ChartType, LegendPosition, LineStyle,
    Marker, Series,
};
use revue::widget::View;

/// A chart with no legend and no grid lines.
fn plain() -> Chart {
    Chart::new()
        .no_legend()
        .x_axis(Axis::new().grid(false))
        .y_axis(Axis::new().grid(false))
}

fn diagonal() -> Series {
    Series::new("Data").data(vec![(0.0, 0.0), (10.0, 10.0)])
}

fn flat() -> Series {
    Series::new("Flat").data(vec![(1.0, 5.0), (2.0, 5.0), (3.0, 5.0)])
}

fn sym(buffer: &Buffer, x: u16, y: u16) -> char {
    buffer.get(x, y).unwrap().symbol
}

/// Line-drawing cells inside the plot area of a 40x20 chart.
fn plot_line_cells(buffer: &Buffer) -> usize {
    let mut n = 0;
    for y in 0..18 {
        for x in 8..40 {
            if matches!(sym(buffer, x, y), '─' | '│' | '╱' | '╲') {
                n += 1;
            }
        }
    }
    n
}

// =========================================================================
// Construction
// =========================================================================

#[test]
fn test_chart_new_default_and_helper_agree() {
    let a = render_rows(&Chart::new(), 40, 20);
    assert_eq!(a, render_rows(&Chart::default(), 40, 20));
    assert_eq!(a, render_rows(&chart(), 40, 20));
}

#[test]
fn test_chart_empty_uses_unit_bounds() {
    let buffer = render(&plain(), 40, 20);
    // X runs 0..1; Y runs 0..1 padded by 5%
    assert_eq!(sym(&buffer, 8, 18), '0');
    assert_eq!(sym(&buffer, 39, 18), '1');
    let rows = rows(&buffer);
    assert!(rows[17].starts_with("  -0.05"), "{:?}", rows[17]);
    assert!(rows[0].starts_with("   1.05"), "{:?}", rows[0]);
}

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

// =========================================================================
// Series and lines
// =========================================================================

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
fn test_chart_identical_values_draw_flat_line() {
    // A zero Y range is widened to +-0.5 around the value: row 9
    let buffer = render(&plain().series(flat()), 40, 20);
    for x in 8..40 {
        assert_eq!(sym(&buffer, x, 9), '─', "x={x}");
    }
    assert_eq!(plot_line_cells(&buffer), 32);
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
fn test_chart_single_point_marker_is_centered() {
    let s = Series::new("P")
        .data(vec![(5.0, 10.0)])
        .marker(Marker::Star);
    let buffer = render(&plain().series(s), 40, 20);
    assert_eq!(count_char(&buffer, '★'), 1);
    assert_eq!(sym(&buffer, 23, 9), '★');
    assert_eq!(plot_line_cells(&buffer), 0);
}

#[test]
fn test_chart_nan_points_are_skipped() {
    let s = Series::new("Data")
        .data(vec![
            (1.0, 2.0),
            (f64::NAN, 3.0),
            (3.0, f64::NAN),
            (4.0, 5.0),
        ])
        .marker(Marker::Star);
    let buffer = render(&plain().series(s), 40, 20);
    // Only the two finite points get markers: (1,2) -> (8,17), (4,5) -> (39,1)
    assert_eq!(count_char(&buffer, '★'), 2);
    assert_eq!(sym(&buffer, 8, 17), '★');
    assert_eq!(sym(&buffer, 39, 1), '★');
    // ... joined by a single rising line
    assert!(rows(&buffer).iter().all(|r| !r.contains('╲')));
}

#[test]
fn test_chart_infinite_points_are_skipped() {
    let s = Series::new("Data")
        .data(vec![
            (1.0, 2.0),
            (f64::INFINITY, 3.0),
            (3.0, f64::NEG_INFINITY),
            (4.0, 5.0),
        ])
        .marker(Marker::Star);
    let buffer = render(&plain().series(s), 40, 20);
    assert_eq!(count_char(&buffer, '★'), 2);
    assert_eq!(sym(&buffer, 8, 17), '★');
    assert_eq!(sym(&buffer, 39, 1), '★');
}

#[test]
#[ignore = "BUG: Chart overflows u16 (panics in debug) mapping points outside fixed axis bounds"]
fn test_chart_points_outside_axis_bounds_are_clipped() {
    // Fixed bounds 0..10 with data reaching past both ends
    let s = Series::new("Data")
        .data(vec![(-5.0, -5.0), (5.0, 5.0), (20.0, 20.0)])
        .marker(Marker::Star);
    let chart = plain()
        .series(s)
        .x_axis(Axis::new().bounds(0.0, 10.0).grid(false))
        .y_axis(Axis::new().bounds(0.0, 10.0).grid(false));
    let buffer = render(&chart, 40, 20);
    // Only (5, 5) is inside the plot
    assert_eq!(count_char(&buffer, '★'), 1);
}

// =========================================================================
// Axes
// =========================================================================

#[test]
fn test_chart_axis_bounds_override() {
    // Both bounds fixed, so no padding: (5, 10) -> x = 8 + 5/20*31, y = 17 - 10/30*17
    let chart = plain()
        .series(
            Series::new("P")
                .data(vec![(5.0, 10.0)])
                .marker(Marker::Star),
        )
        .x_axis(Axis::new().bounds(0.0, 20.0).grid(false))
        .y_axis(Axis::new().bounds(0.0, 30.0).grid(false));
    let buffer = render(&chart, 40, 20);
    assert_eq!(sym(&buffer, 15, 12), '★');
    let rows = rows(&buffer);
    assert!(rows[17].starts_with("      0"), "{:?}", rows[17]);
    assert!(rows[0].starts_with("     30"), "{:?}", rows[0]);
    assert!(rows[18].trim_end().ends_with("20"), "{:?}", rows[18]);
}

#[test]
fn test_chart_negative_values_labels() {
    let s = Series::new("D").data(vec![(-5.0, -3.0), (-2.0, -1.0), (0.0, 0.0)]);
    let rows = render_rows(&plain().series(s), 40, 20);
    // Y -3..0 padded to -3.15..0.15
    assert!(rows[17].starts_with("  -3.15"), "{:?}", rows[17]);
    assert!(rows[0].starts_with("   0.15"), "{:?}", rows[0]);
    // X labels are centered on their tick: "-5" starts one column left of the plot
    assert!(rows[18][7..].starts_with("-5"), "{:?}", rows[18]);
}

#[test]
fn test_chart_large_values_use_exponent_labels() {
    let s = Series::new("D").data(vec![(1000.0, 2000.0), (3000.0, 4000.0)]);
    let rows = render_rows(&plain().series(s), 40, 20);
    assert!(rows[17].starts_with("  1.9e3"), "{:?}", rows[17]);
    assert!(rows[0].starts_with("  4.1e3"), "{:?}", rows[0]);
}

#[test]
fn test_chart_small_values_use_exponent_labels() {
    let s = Series::new("D").data(vec![(0.001, 0.002), (0.003, 0.004)]);
    let rows = render_rows(&plain().series(s), 40, 20);
    assert!(rows[17].starts_with(" 1.9e-3"), "{:?}", rows[17]);
    assert!(rows[0].starts_with(" 4.1e-3"), "{:?}", rows[0]);
}

#[test]
fn test_chart_axis_formats() {
    let s = Series::new("D").data(vec![(0.0, 0.0), (1.0, 1.0)]);
    let chart = plain()
        .series(s)
        .x_axis(
            Axis::new()
                .bounds(0.0, 1.0)
                .grid(false)
                .format(AxisFormat::Percent),
        )
        .y_axis(
            Axis::new()
                .bounds(0.0, 1.0)
                .grid(false)
                .format(AxisFormat::Custom("v{}".to_string())),
        );
    let rows = render_rows(&chart, 40, 20);
    assert!(rows[17].starts_with("     v0"), "{:?}", rows[17]);
    assert!(rows[0].starts_with("     v1"), "{:?}", rows[0]);
    assert!(rows[18][7..].starts_with("0%"), "{:?}", rows[18]);
    // "100%" is centered on the last column, so its '%' falls off the edge
    assert!(rows[18].trim_end().ends_with("100"), "{:?}", rows[18]);

    let fixed = plain()
        .series(Series::new("D").data(vec![(0.0, 0.0), (1.0, 1.0)]))
        .y_axis(
            Axis::new()
                .bounds(0.0, 1.0)
                .grid(false)
                .format(AxisFormat::Fixed(3)),
        );
    let rows = render_rows(&fixed, 40, 20);
    assert!(rows[17].starts_with("  0.000"), "{:?}", rows[17]);

    let integer = plain()
        .series(Series::new("D").data(vec![(0.0, 0.0), (1.0, 1.0)]))
        .y_axis(
            Axis::new()
                .bounds(0.0, 5.67)
                .grid(false)
                .format(AxisFormat::Integer),
        );
    let rows = render_rows(&integer, 40, 20);
    assert!(rows[0].starts_with("      6"), "{:?}", rows[0]);
}

#[test]
fn test_chart_grid_lines() {
    let chart = Chart::new().no_legend().series(diagonal());
    let buffer = render(&chart, 40, 20);
    // Default axes draw 4 inner grid lines each way
    assert!(count_char(&buffer, '┄') > 0);
    assert!(count_char(&buffer, '┊') > 0);

    let buffer = render(&plain().series(diagonal()), 40, 20);
    assert_eq!(count_char(&buffer, '┄'), 0);
    assert_eq!(count_char(&buffer, '┊'), 0);
}

#[test]
fn test_chart_axis_titles() {
    let chart = plain()
        .x_axis(Axis::new().title("X Axis").grid(false))
        .y_axis(Axis::new().title("Y Axis").grid(false))
        .series(Series::new("Test").data(vec![(1.0, 2.0)]));
    let buffer = render(&chart, 40, 20);
    let rows = rows(&buffer);
    // X title centered under the plot on row 19
    assert_eq!(rows[19].trim(), "X Axis");
    // Y title written vertically in column 0, centered on the plot height
    let column: String = (6..12).map(|y| sym(&buffer, 0, y)).collect();
    assert_eq!(column, "Y Axis");
}

// =========================================================================
// Chart types and styles
// =========================================================================

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

// =========================================================================
// Legend
// =========================================================================

#[test]
fn test_chart_legend_lists_series_in_order() {
    let chart = Chart::new()
        .series(Series::new("Data 1").data(vec![(1.0, 2.0)]))
        .series_vec(vec![
            Series::new("Data 2").data(vec![(1.0, 3.0)]),
            Series::new("Data 3").data(vec![(1.0, 4.0)]),
        ]);
    let rows = render_rows(&chart, 40, 20);
    // Default TopRight: box starts on row 1, entries on rows 2..5
    assert!(rows[2].contains("■ Data 1"), "{:?}", rows[2]);
    assert!(rows[3].contains("■ Data 2"), "{:?}", rows[3]);
    assert!(rows[4].contains("■ Data 3"), "{:?}", rows[4]);
}

#[test]
fn test_chart_legend_positions() {
    // Legend for "Data" is 8 wide and 3 tall
    let corner = |pos: LegendPosition| {
        let chart = plain()
            .legend(pos)
            .series(Series::new("Data").data(vec![(1.0, 2.0)]));
        let buffer = render(&chart, 40, 20);
        let mut found = None;
        for y in 0..20 {
            for x in 0..40 {
                if sym(&buffer, x, y) == '┌' {
                    found = Some((x, y));
                }
            }
        }
        found
    };
    assert_eq!(corner(LegendPosition::TopLeft), Some((9, 1)));
    assert_eq!(corner(LegendPosition::TopRight), Some((31, 1)));
    assert_eq!(corner(LegendPosition::BottomLeft), Some((9, 14)));
    assert_eq!(corner(LegendPosition::BottomRight), Some((31, 14)));
    assert_eq!(corner(LegendPosition::None), None);
}

#[test]
fn test_chart_no_legend() {
    let chart = Chart::new()
        .legend(LegendPosition::TopLeft)
        .no_legend()
        .series(Series::new("Data").data(vec![(1.0, 2.0)]));
    let buffer = render(&chart, 40, 20);
    assert_eq!(count_char(&buffer, '■'), 0);
    assert!(!rows(&buffer).join("").contains("Data"));
}

#[test]
fn test_chart_legend_wider_than_plot() {
    let chart = Chart::new()
        .series(Series::new("A very long series name that does not fit").data(vec![(1.0, 2.0)]));
    let buffer = render(&chart, 20, 10);
    assert_eq!(count_char(&buffer, '■'), 1);
}

#[test]
fn test_chart_legend_taller_than_plot() {
    let mut chart = Chart::new();
    for i in 0..10 {
        chart = chart.series(Series::new(format!("S{i}")).data(vec![(1.0, i as f64)]));
    }
    let buffer = render(&chart, 30, 8);
    assert!(count_char(&buffer, '■') >= 1);
}

// =========================================================================
// Background and border
// =========================================================================

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
#[ignore = "BUG: Chart::braille() is stored but never used when rendering"]
fn test_chart_braille_changes_rendering() {
    let normal = render_rows(&plain().series(diagonal()), 40, 20);
    let braille = render_rows(&plain().braille().series(diagonal()), 40, 20);
    assert_ne!(normal, braille);
}

// =========================================================================
// Helpers
// =========================================================================

#[test]
fn test_line_chart_helper() {
    let a = render_rows(&line_chart(&[2.0, 4.0]), 40, 20);
    let b = render_rows(
        &Chart::new().series(Series::new("Data").data_y(&[2.0, 4.0]).line()),
        40,
        20,
    );
    assert_eq!(a, b);
    assert!(a.join("\n").contains("■ Data"));
    assert!(a.iter().any(|r| r.contains('╱')));
}

#[test]
fn test_line_chart_helper_empty_and_single() {
    assert_eq!(
        plot_line_cells(&render(&line_chart(&[]).no_legend(), 40, 20)),
        0
    );
    assert_eq!(
        plot_line_cells(&render(&line_chart(&[10.0]).no_legend(), 40, 20)),
        0
    );
}

#[test]
fn test_scatter_plot_helper() {
    let chart = scatter_plot(&[(1.0, 2.0), (3.0, 4.0), (5.0, 6.0)]).no_legend();
    let buffer = render(&chart, 40, 20);
    assert_eq!(count_char(&buffer, Marker::Dot.char()), 3);
    assert_eq!(plot_line_cells(&buffer), 0);

    let empty = render(&scatter_plot(&[]).no_legend(), 40, 20);
    assert_eq!(count_char(&empty, Marker::Dot.char()), 0);
}

// =========================================================================
// Larger scenarios
// =========================================================================

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
