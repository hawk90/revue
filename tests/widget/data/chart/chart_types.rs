//! Chart type tests extracted from src/widget/data/chart/types.rs
//!
//! This file contains tests for chart type definitions:
//! - ChartType enum (Line, Scatter, Area, StepAfter, StepBefore)
//! - LineStyle enum (Solid, Dashed, Dotted, None)
//! - Series struct and builder methods

use revue::style::Color;
use revue::widget::data::chart::chart_common::Marker;
use revue::widget::data::chart::types::{ChartType, LineStyle, Series};

// =========================================================================
// Series::new tests
// =========================================================================

#[test]
fn test_series_new() {
    let series = Series::new("Test Series");
    assert_eq!(series.name, "Test Series");
    assert!(series.data.is_empty());
    assert_eq!(series.chart_type, ChartType::Line);
    assert_eq!(series.color, Color::WHITE);
    assert_eq!(series.line_style, LineStyle::Solid);
    assert_eq!(series.marker, Marker::None);
    assert!(series.fill_color.is_none());
}

#[test]
fn test_series_new_string() {
    let series = Series::new(String::from("Owned Name"));
    assert_eq!(series.name, "Owned Name");
}

// =========================================================================
// Series::data tests
// =========================================================================

#[test]
fn test_series_data() {
    let data = vec![(1.0, 2.0), (3.0, 4.0), (5.0, 6.0)];
    let series = Series::new("Test").data(data.clone());
    assert_eq!(series.data, data);
}

#[test]
fn test_series_data_empty() {
    let series = Series::new("Test").data(vec![]);
    assert!(series.data.is_empty());
}

// =========================================================================
// Series::data_y tests
// =========================================================================

#[test]
fn test_series_data_y() {
    let ys = vec![1.0, 2.0, 3.0, 4.0];
    let series = Series::new("Test").data_y(&ys);
    assert_eq!(series.data.len(), 4);
    assert_eq!(series.data[0], (0.0, 1.0));
    assert_eq!(series.data[1], (1.0, 2.0));
    assert_eq!(series.data[2], (2.0, 3.0));
    assert_eq!(series.data[3], (3.0, 4.0));
}

#[test]
fn test_series_data_y_empty() {
    let series = Series::new("Test").data_y(&[]);
    assert!(series.data.is_empty());
}

// =========================================================================
// Series::chart_type tests
// =========================================================================

#[test]
fn test_series_chart_type_line() {
    let series = Series::new("Test").chart_type(ChartType::Line);
    assert_eq!(series.chart_type, ChartType::Line);
}

#[test]
fn test_series_chart_type_scatter() {
    let series = Series::new("Test").chart_type(ChartType::Scatter);
    assert_eq!(series.chart_type, ChartType::Scatter);
}

#[test]
fn test_series_chart_type_area() {
    let series = Series::new("Test").chart_type(ChartType::Area);
    assert_eq!(series.chart_type, ChartType::Area);
}

#[test]
fn test_series_chart_type_step_after() {
    let series = Series::new("Test").chart_type(ChartType::StepAfter);
    assert_eq!(series.chart_type, ChartType::StepAfter);
}

#[test]
fn test_series_chart_type_step_before() {
    let series = Series::new("Test").chart_type(ChartType::StepBefore);
    assert_eq!(series.chart_type, ChartType::StepBefore);
}

// =========================================================================
// Series::color tests
// =========================================================================

#[test]
fn test_series_color() {
    let series = Series::new("Test").color(Color::RED);
    assert_eq!(series.color, Color::RED);
}

#[test]
fn test_series_color_rgb() {
    let series = Series::new("Test").color(Color::rgb(128, 64, 32));
    assert_eq!(series.color.r, 128);
    assert_eq!(series.color.g, 64);
    assert_eq!(series.color.b, 32);
}

// =========================================================================
// Series::line_style tests
// =========================================================================

#[test]
fn test_series_line_style_solid() {
    let series = Series::new("Test").line_style(LineStyle::Solid);
    assert_eq!(series.line_style, LineStyle::Solid);
}

#[test]
fn test_series_line_style_dashed() {
    let series = Series::new("Test").line_style(LineStyle::Dashed);
    assert_eq!(series.line_style, LineStyle::Dashed);
}

#[test]
fn test_series_line_style_dotted() {
    let series = Series::new("Test").line_style(LineStyle::Dotted);
    assert_eq!(series.line_style, LineStyle::Dotted);
}

#[test]
fn test_series_line_style_none() {
    let series = Series::new("Test").line_style(LineStyle::None);
    assert_eq!(series.line_style, LineStyle::None);
}

// =========================================================================
// Series::marker tests
// =========================================================================

#[test]
fn test_series_marker_dot() {
    let series = Series::new("Test").marker(Marker::Dot);
    assert_eq!(series.marker, Marker::Dot);
}

#[test]
fn test_series_marker_circle() {
    let series = Series::new("Test").marker(Marker::Circle);
    assert_eq!(series.marker, Marker::Circle);
}

#[test]
fn test_series_marker_star() {
    let series = Series::new("Test").marker(Marker::Star);
    assert_eq!(series.marker, Marker::Star);
}

#[test]
fn test_series_marker_square() {
    let series = Series::new("Test").marker(Marker::Square);
    assert_eq!(series.marker, Marker::Square);
}

#[test]
fn test_series_marker_none() {
    let series = Series::new("Test").marker(Marker::None);
    assert_eq!(series.marker, Marker::None);
}

// =========================================================================
// Series::fill tests
// =========================================================================

#[test]
fn test_series_fill() {
    let series = Series::new("Test").fill(Color::BLUE);
    assert_eq!(series.fill_color, Some(Color::BLUE));
    assert_eq!(series.chart_type, ChartType::Area);
}

// =========================================================================
// Series::scatter tests
// =========================================================================

#[test]
fn test_series_scatter() {
    let series = Series::new("Test").scatter();
    assert_eq!(series.chart_type, ChartType::Scatter);
    assert_eq!(series.line_style, LineStyle::None);
    assert_eq!(series.marker, Marker::Dot);
}

#[test]
fn test_series_scatter_preserves_custom_marker() {
    let series = Series::new("Test").marker(Marker::Star).scatter();
    assert_eq!(series.marker, Marker::Star);
}

// =========================================================================
// Series::line tests
// =========================================================================

#[test]
fn test_series_line() {
    let series = Series::new("Test").line();
    assert_eq!(series.chart_type, ChartType::Line);
    assert_eq!(series.line_style, LineStyle::Solid);
}

// =========================================================================
// Series::area tests
// =========================================================================

#[test]
fn test_series_area() {
    let series = Series::new("Test").area(Color::GREEN);
    assert_eq!(series.chart_type, ChartType::Area);
    assert_eq!(series.fill_color, Some(Color::GREEN));
}

// =========================================================================
// Series::step tests
// =========================================================================

#[test]
fn test_series_step() {
    let series = Series::new("Test").step();
    assert_eq!(series.chart_type, ChartType::StepAfter);
}

// =========================================================================
// Series Clone trait
// =========================================================================

#[test]
fn test_series_clone() {
    let series1 = Series::new("Test")
        .data(vec![(1.0, 2.0)])
        .color(Color::RED)
        .line_style(LineStyle::Dashed);

    let series2 = series1.clone();
    assert_eq!(series1.name, series2.name);
    assert_eq!(series1.data, series2.data);
    assert_eq!(series1.color, series2.color);
    assert_eq!(series1.line_style, series2.line_style);
}

// =========================================================================
// Series Debug trait
// =========================================================================

#[test]
fn test_series_debug() {
    let series = Series::new("Debug Test");
    let debug_str = format!("{:?}", series);
    assert!(debug_str.contains("Debug Test"));
}

// =========================================================================
// Series builder chain tests
// =========================================================================

#[test]
fn test_series_builder_chain() {
    let data = vec![(1.0, 10.0), (2.0, 20.0), (3.0, 15.0)];
    let series = Series::new("Chain Test")
        .data(data.clone())
        .chart_type(ChartType::Area)
        .color(Color::CYAN)
        .line_style(LineStyle::Dotted)
        .fill(Color::BLUE);

    assert_eq!(series.name, "Chain Test");
    assert_eq!(series.data, data);
    assert_eq!(series.chart_type, ChartType::Area);
    assert_eq!(series.color, Color::CYAN);
    assert_eq!(series.line_style, LineStyle::Dotted);
    assert_eq!(series.fill_color, Some(Color::BLUE));
}

#[test]
fn test_series_scatter_chain() {
    let series = Series::new("Scatter Data")
        .data(vec![(1.0, 5.0), (2.0, 7.0)])
        .color(Color::YELLOW)
        .scatter();

    assert_eq!(series.chart_type, ChartType::Scatter);
    assert_eq!(series.line_style, LineStyle::None);
    assert_eq!(series.color, Color::YELLOW);
}

#[test]
fn test_series_line_chain() {
    let series = Series::new("Line Data")
        .data_y(&[1.0, 2.0, 3.0])
        .color(Color::GREEN)
        .line();

    assert_eq!(series.chart_type, ChartType::Line);
    assert_eq!(series.line_style, LineStyle::Solid);
    assert_eq!(series.data.len(), 3);
}

// =========================================================================
// ChartType enum tests
// =========================================================================

#[test]
fn test_chart_type_default() {
    assert_eq!(ChartType::default(), ChartType::Line);
}

#[test]
fn test_chart_type_clone() {
    let ct1 = ChartType::Area;
    let ct2 = ct1.clone();
    assert_eq!(ct1, ct2);
}

#[test]
fn test_chart_type_copy() {
    let ct1 = ChartType::StepAfter;
    let ct2 = ct1;
    assert_eq!(ct2, ChartType::StepAfter);
}

#[test]
fn test_chart_type_partial_eq() {
    assert_eq!(ChartType::Line, ChartType::Line);
    assert_eq!(ChartType::Scatter, ChartType::Scatter);
    assert_ne!(ChartType::Line, ChartType::Scatter);
}

#[test]
fn test_chart_type_debug() {
    let debug_str = format!("{:?}", ChartType::Area);
    assert!(debug_str.contains("Area"));
}

// =========================================================================
// LineStyle enum tests
// =========================================================================

#[test]
fn test_line_style_default() {
    assert_eq!(LineStyle::default(), LineStyle::Solid);
}

#[test]
fn test_line_style_clone() {
    let ls1 = LineStyle::Dashed;
    let ls2 = ls1.clone();
    assert_eq!(ls1, ls2);
}

#[test]
fn test_line_style_copy() {
    let ls1 = LineStyle::Dotted;
    let ls2 = ls1;
    assert_eq!(ls2, LineStyle::Dotted);
}

#[test]
fn test_line_style_partial_eq() {
    assert_eq!(LineStyle::Solid, LineStyle::Solid);
    assert_eq!(LineStyle::None, LineStyle::None);
    assert_ne!(LineStyle::Solid, LineStyle::Dashed);
}

#[test]
fn test_line_style_debug() {
    let debug_str = format!("{:?}", LineStyle::Dotted);
    assert!(debug_str.contains("Dotted"));
}

// =========================================================================
// Integration tests with different chart types
// =========================================================================

#[test]
fn test_series_as_multiple_types() {
    let data = vec![(1.0, 2.0), (3.0, 4.0)];

    let line_series = Series::new("Line").data(data.clone()).line();
    assert_eq!(line_series.chart_type, ChartType::Line);

    let scatter_series = Series::new("Scatter").data(data.clone()).scatter();
    assert_eq!(scatter_series.chart_type, ChartType::Scatter);

    let area_series = Series::new("Area").data(data.clone()).area(Color::BLUE);
    assert_eq!(area_series.chart_type, ChartType::Area);

    let step_series = Series::new("Step").data(data).step();
    assert_eq!(step_series.chart_type, ChartType::StepAfter);
}

#[test]
fn test_series_with_all_options() {
    let series = Series::new("Full Featured")
        .data(vec![(0.0, 1.0), (1.0, 2.0), (2.0, 1.5)])
        .chart_type(ChartType::Area)
        .color(Color::rgb(255, 128, 0))
        .line_style(LineStyle::Solid)
        .fill(Color::rgb(255, 128, 0));

    assert_eq!(series.name, "Full Featured");
    assert_eq!(series.data.len(), 3);
    assert_eq!(series.chart_type, ChartType::Area);
    assert_eq!(series.color.r, 255);
    assert_eq!(series.line_style, LineStyle::Solid);
    assert_eq!(series.fill_color, Some(Color::rgb(255, 128, 0)));
}

#[test]
fn test_series_empty_data_with_styling() {
    let series = Series::new("Empty")
        .color(Color::RED)
        .line_style(LineStyle::Dashed)
        .scatter();

    assert!(series.data.is_empty());
    assert_eq!(series.color, Color::RED);
    assert_eq!(series.line_style, LineStyle::None); // scatter() sets this
    assert_eq!(series.chart_type, ChartType::Scatter);
}

#[test]
fn test_series_data_y_with_f64_values() {
    let series = Series::new("Y Values").data_y(&[10.5, 20.3, 30.7, 40.1]);
    assert_eq!(series.data.len(), 4);
    assert_eq!(series.data[0].1, 10.5);
    assert_eq!(series.data[1].1, 20.3);
    assert_eq!(series.data[2].1, 30.7);
    assert_eq!(series.data[3].1, 40.1);
}

#[test]
fn test_series_with_negative_values() {
    let series = Series::new("Negative").data(vec![(-1.0, -2.0), (-3.0, -4.0)]);
    assert_eq!(series.data[0], (-1.0, -2.0));
    assert_eq!(series.data[1], (-3.0, -4.0));
}

#[test]
fn test_series_marker_variants() {
    let dot = Series::new("Dot").marker(Marker::Dot);
    assert_eq!(dot.marker, Marker::Dot);

    let circle = Series::new("Circle").marker(Marker::Circle);
    assert_eq!(circle.marker, Marker::Circle);

    let star = Series::new("Star").marker(Marker::Star);
    assert_eq!(star.marker, Marker::Star);

    let square = Series::new("Square").marker(Marker::Square);
    assert_eq!(square.marker, Marker::Square);

    let none = Series::new("None").marker(Marker::None);
    assert_eq!(none.marker, Marker::None);
}

#[test]
fn test_series_area_with_fill() {
    let series = Series::new("Area").area(Color::MAGENTA);
    assert_eq!(series.chart_type, ChartType::Area);
    assert_eq!(series.fill_color, Some(Color::MAGENTA));
}

#[test]
fn test_series_step_before() {
    let series = Series::new("Step").chart_type(ChartType::StepBefore);
    assert_eq!(series.chart_type, ChartType::StepBefore);
}
