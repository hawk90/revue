//! Tests for chart widget

#[test]
fn test_chart_new() {
    let c = Chart::new();
    assert!(c.title.is_none());
    assert!(c.series.is_empty());
}

#[test]
fn test_series_builder() {
    let s = Series::new("Test")
        .data(vec![(0.0, 1.0), (1.0, 2.0)])
        .color(Color::RED)
        .marker(Marker::Dot);

    assert_eq!(s.name, "Test");
    assert_eq!(s.data.len(), 2);
    assert_eq!(s.color, Color::RED);
}

#[test]
fn test_series_data_y() {
    let s = Series::new("Test").data_y(&[1.0, 2.0, 3.0]);
    assert_eq!(s.data, vec![(0.0, 1.0), (1.0, 2.0), (2.0, 3.0)]);
}

#[test]
fn test_chart_bounds() {
    let c = Chart::new().series(Series::new("A").data(vec![(0.0, 0.0), (10.0, 100.0)]));

    let bounds = c.compute_bounds();
    assert_eq!(bounds.0, 0.0); // x_min
    assert_eq!(bounds.1, 10.0); // x_max
}

#[test]
fn test_axis_builder() {
    let axis = Axis::new()
        .title("Value")
        .bounds(0.0, 100.0)
        .ticks(10)
        .grid(true);

    assert_eq!(axis.title, Some("Value".to_string()));
    assert_eq!(axis.min, Some(0.0));
    assert_eq!(axis.max, Some(100.0));
    assert_eq!(axis.ticks, 10);
    assert!(axis.grid);
}

#[test]
fn test_chart_render() {
    let mut buffer = Buffer::new(60, 20);
    let area = Rect::new(0, 0, 60, 20);
    let mut ctx = crate::traits::RenderContext::new(&mut buffer, area);

    let c = Chart::new().title("Test Chart").series(
        Series::new("Data")
            .data_y(&[1.0, 4.0, 2.0, 5.0, 3.0])
            .line(),
    );

    c.render(&mut ctx);
    // Basic smoke test - chart renders without panic
}

#[test]
fn test_quick_line_chart() {
    let c = super::line_chart(&[1.0, 2.0, 3.0, 2.0, 1.0]);
    assert_eq!(c.series.len(), 1);
    assert_eq!(c.series[0].data.len(), 5);
}

#[test]
fn test_quick_scatter_plot() {
    let c = super::scatter_plot(&[(0.0, 1.0), (1.0, 2.0), (2.0, 1.5)]);
    assert_eq!(c.series.len(), 1);
    assert!(matches!(c.series[0].chart_type, ChartType::Scatter));
}

#[test]
fn test_multiple_series() {
    let c = Chart::new()
        .series(Series::new("A").data_y(&[1.0, 2.0, 3.0]).color(Color::RED))
        .series(Series::new("B").data_y(&[3.0, 2.0, 1.0]).color(Color::BLUE));

    assert_eq!(c.series.len(), 2);
}

#[test]
fn test_area_chart() {
    let s = Series::new("Area")
        .data_y(&[1.0, 3.0, 2.0])
        .area(Color::CYAN);

    assert!(matches!(s.chart_type, ChartType::Area));
    assert_eq!(s.fill_color, Some(Color::CYAN));
}

#[test]
fn test_step_chart() {
    let s = Series::new("Step").data_y(&[1.0, 2.0, 3.0]).step();
    assert!(matches!(s.chart_type, ChartType::StepAfter));
}

#[test]
fn test_marker_chars() {
    assert_eq!(Marker::Dot.char(), '•');
    assert_eq!(Marker::Circle.char(), '○');
    assert_eq!(Marker::Square.char(), '□');
    assert_eq!(Marker::Diamond.char(), '◇');
    assert_eq!(Marker::Cross.char(), '+');
}

#[test]
fn test_format_labels() {
    let c = Chart::new();

    assert_eq!(
        c.format_label(5.0, &crate::widget::chart_common::AxisFormat::Integer),
        "5"
    );
    assert_eq!(
        c.format_label(5.0, &crate::widget::chart_common::AxisFormat::Fixed(2)),
        "5.00"
    );
    assert_eq!(
        c.format_label(0.5, &crate::widget::chart_common::AxisFormat::Percent),
        "50%"
    );
}

#[test]
fn test_legend_positions() {
    use crate::widget::chart_common::LegendPosition;
    let c = Chart::new()
        .series(Series::new("Test").data_y(&[1.0, 2.0]))
        .legend(LegendPosition::BottomLeft);

    assert!(matches!(c.legend, LegendPosition::BottomLeft));
}

#[test]
fn test_chart_with_all_options() {
    let mut buffer = Buffer::new(80, 30);
    let area = Rect::new(0, 0, 80, 30);
    let mut ctx = crate::traits::RenderContext::new(&mut buffer, area);

    let c = Chart::new()
        .title("Full Chart")
        .border(Color::WHITE)
        .bg(Color::rgb(20, 20, 20))
        .x_axis(Axis::new().title("Time").ticks(10))
        .y_axis(Axis::new().title("Value").bounds(0.0, 100.0))
        .legend(crate::widget::chart_common::LegendPosition::TopRight)
        .series(
            Series::new("Line")
                .data_y(&[10.0, 30.0, 20.0, 50.0, 40.0, 60.0])
                .color(Color::GREEN)
                .marker(Marker::Circle),
        )
        .series(
            Series::new("Area")
                .data_y(&[5.0, 15.0, 10.0, 25.0, 20.0, 30.0])
                .area(Color::CYAN),
        );

    c.render(&mut ctx);
    // Smoke test passes
}

#[test]
fn test_compute_bounds_empty_data() {
    let c = Chart::new();
    let (x_min, x_max, y_min, y_max) = c.compute_bounds();
    // Default bounds for empty data
    assert_eq!(x_min, 0.0);
    assert_eq!(x_max, 1.0);
    assert!(y_min < y_max); // With padding
}

#[test]
fn test_compute_bounds_single_point() {
    let c = Chart::new().series(Series::new("Single").data(vec![(5.0, 10.0)]));
    let (x_min, x_max, y_min, y_max) = c.compute_bounds();
    // Should create range around single point
    assert!(x_min < x_max);
    assert!(y_min < y_max);
}

#[test]
fn test_compute_bounds_zero_range() {
    let c = Chart::new().series(Series::new("Flat").data_y(&[5.0, 5.0, 5.0]));
    let (_, _, y_min, y_max) = c.compute_bounds();
    // Should create range around flat line
    assert!(y_min < y_max);
}

#[test]
fn test_compute_bounds_nan_values() {
    let c = Chart::new().series(Series::new("WithNaN").data(vec![
        (1.0, 2.0),
        (f64::NAN, 3.0),
        (3.0, f64::INFINITY),
    ]));
    let (x_min, x_max, y_min, y_max) = c.compute_bounds();
    // Should ignore NaN/Infinity and use valid data
    assert!(x_min.is_finite());
    assert!(x_max.is_finite());
    assert!(y_min.is_finite());
    assert!(y_max.is_finite());
}
