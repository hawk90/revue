//! Tests for chart widget helper public APIs

use revue::widget::data::chart::{Chart, Axis, LegendPosition, Series, ChartType, LineStyle};
use revue::widget::data::chart::chart_common::{Marker, AxisFormat};
use revue::style::Color;

#[test]
fn test_chart_new() {
    let chart = Chart::new();
    assert_eq!(chart.series.len(), 0);
    assert!(chart.title.is_none());
}

#[test]
fn test_chart_builder_pattern() {
    let chart = Chart::new()
        .title("Test Chart")
        .series(Series::new("Series 1").data(vec![(1.0, 2.0), (3.0, 4.0)]))
        .series(Series::new("Series 2").data(vec![(1.0, 1.0), (2.0, 2.0)]))
        .x_axis(Axis::new())
        .y_axis(Axis::new())
        .legend(LegendPosition::TopRight)
        .bg(Color::rgb(50, 50, 50))
        .border(Color::rgb(255, 255, 255));

    assert_eq!(chart.series.len(), 2);
    assert_eq!(chart.title, Some("Test Chart".to_string()));
    assert_eq!(chart.legend, LegendPosition::TopRight);
    assert_eq!(chart.bg_color, Some(Color::rgb(50, 50, 50)));
    assert_eq!(chart.border_color, Some(Color::rgb(255, 255, 255)));
}

#[test]
fn test_chart_series_single() {
    let series = Series::new("Test Series")
        .data(vec![(1.0, 2.0), (2.0, 4.0), (3.0, 6.0)])
        .line()
        .color(Color::BLUE)
        .marker(Marker::Dot)
        .fill_color(Color::rgba(0, 0, 255, 128));

    assert_eq!(series.name, "Test Series");
    assert_eq!(series.data, vec![(1.0, 2.0), (2.0, 4.0), (3.0, 6.0)]);
    assert_eq!(series.chart_type, ChartType::Line);
    assert_eq!(series.line_style, LineStyle::Solid);
    assert_eq!(series.color, Color::BLUE);
    assert_eq!(series.marker, Marker::Dot);
    assert_eq!(series.fill_color, Some(Color::rgba(0, 0, 255, 128)));
}

#[test]
fn test_chart_series_multiple() {
    let chart = Chart::new()
        .series_vec(vec![
            Series::new("Series 1").data_y(vec![1.0, 2.0, 3.0]),
            Series::new("Series 2").data_y(vec![2.0, 4.0, 6.0]),
        ]);

    assert_eq!(chart.series.len(), 2);
    assert_eq!(chart.series[0].name, "Series 1");
    assert_eq!(chart.series[1].name, "Series 2");
}

#[test]
fn test_chart_axis() {
    let x_axis = Axis::new()
        .min(0.0)
        .max(10.0)
        .title("X Axis")
        .ticks(5)
        .color(Color::RED)
        .grid(true);

    let y_axis = Axis::new()
        .min(0.0)
        .max(100.0)
        .title("Y Axis")
        .ticks(10)
        .color(Color::GREEN)
        .grid(false);

    assert_eq!(x_axis.min, Some(0.0));
    assert_eq!(x_axis.max, Some(10.0));
    assert_eq!(x_axis.title, Some("X Axis".to_string()));
    assert_eq!(x_axis.ticks, 5);
    assert_eq!(x_axis.color, Color::RED);
    assert_eq!(x_axis.grid, true);

    assert_eq!(y_axis.min, Some(0.0));
    assert_eq!(y_axis.max, Some(100.0));
    assert_eq!(y_axis.title, Some("Y Axis".to_string()));
    assert_eq!(y_axis.ticks, 10);
    assert_eq!(y_axis.color, Color::GREEN);
    assert_eq!(y_axis.grid, false);
}

#[test]
fn test_chart_legend() {
    let chart = Chart::new()
        .legend(LegendPosition::TopLeft)
        .no_legend();

    assert_eq!(chart.legend, LegendPosition::None);
}

#[test]
fn test_chart_colors() {
    let chart = Chart::new()
        .bg(Color::BLACK)
        .border(Color::WHITE);

    assert_eq!(chart.bg_color, Some(Color::BLACK));
    assert_eq!(chart.border_color, Some(Color::WHITE));
}

#[test]
fn test_chart_braille_mode() {
    let chart = Chart::new().braille();
    assert!(chart.braille_mode);
}

#[test]
fn test_chart_marker_types() {
    let dot = Marker::Dot;
    let square = Marker::Square;
    let plus = Marker::Plus;
    let cross = Marker::Cross;
    let none = Marker::None;

    assert_eq!(dot.char(), '·');
    assert_eq!(square.char(), '■');
    assert_eq!(plus.char(), '+');
    assert_eq!(cross.char(), '×');
    assert_eq!(none.char(), ' ');
}

#[test]
fn test_axis_format() {
    let auto_format = AxisFormat::Auto;
    let int_format = AxisFormat::Integer;
    let fixed_format = AxisFormat::Fixed(2);
    let percent_format = AxisFormat::Percent;
    let custom_format = AxisFormat::Custom("Value: {}".to_string());

    // Test string formatting (these are tested in private tests, but we test the public types)
    let chart = Chart::new();
    assert_eq!(chart.format_label(5.0, &int_format), "5");
    assert_eq!(chart.format_label(5.67, &int_format), "6");
    assert_eq!(chart.format_label(0.5, &percent_format), "50%");
    assert_eq!(chart.format_label(42.0, &custom_format), "Value: 42");
}

#[test]
fn test_chart_clone() {
    let chart1 = Chart::new().title("Test").bg(Color::RED);
    let chart2 = chart1.clone();

    assert_eq!(chart1.title, chart2.title);
    assert_eq!(chart1.bg_color, chart2.bg_color);
}

#[test]
fn test_chart_default() {
    let chart: Chart = Default::default();
    let default_chart = Chart::new();

    assert_eq!(chart.title, default_chart.title);
    assert_eq!(chart.series.len(), default_chart.series.len());
    assert_eq!(chart.legend, default_chart.legend);
    assert_eq!(chart.bg_color, default_chart.bg_color);
    assert_eq!(chart.border_color, default_chart.border_color);
}