//! Chart widget public API tests extracted from helper.rs

use crate::layout::Rect;
use crate::render::Buffer;
use crate::style::Color;
use crate::widget::data::chart::types::{ChartType, LineStyle, Series};
use crate::widget::data::chart::chart_common::{Axis, AxisFormat, LegendPosition, Marker};
use crate::widget::traits::RenderContext;
use crate::widget::View;

pub use crate::widget::data::chart::helper::Chart;

// =========================================================================
// Chart::new tests
// =========================================================================

#[test]
fn test_chart_new() {
    let chart = Chart::new();
    assert!(chart.title.is_none());
    assert!(chart.series.is_empty());
    assert_eq!(chart.legend, LegendPosition::TopRight);
    assert!(chart.bg_color.is_none());
    assert!(chart.border_color.is_none());
    assert!(!chart.braille_mode);
}

// =========================================================================
// Chart::title tests
// =========================================================================

#[test]
fn test_chart_title() {
    let chart = Chart::new().title("My Chart");
    assert_eq!(chart.title, Some("My Chart".to_string()));
}

#[test]
fn test_chart_title_from_string() {
    let chart = Chart::new().title(String::from("Owned String"));
    assert_eq!(chart.title, Some("Owned String".to_string()));
}

#[test]
fn test_chart_title_from_str() {
    let chart = Chart::new().title("Str Title");
    assert_eq!(chart.title, Some("Str Title".to_string()));
}

// =========================================================================
// Chart::series tests
// =========================================================================

#[test]
fn test_chart_series() {
    let series = Series::new("Test").data(vec![(1.0, 2.0), (3.0, 4.0)]);
    let chart = Chart::new().series(series.clone());
    assert_eq!(chart.series.len(), 1);
    assert_eq!(chart.series[0].name, "Test");
}

#[test]
fn test_chart_series_multiple() {
    let chart = Chart::new()
        .series(Series::new("A").data(vec![(1.0, 2.0)]))
        .series(Series::new("B").data(vec![(3.0, 4.0)]));
    assert_eq!(chart.series.len(), 2);
}

#[test]
fn test_chart_series_vec() {
    let series = vec![
        Series::new("A").data(vec![(1.0, 2.0)]),
        Series::new("B").data(vec![(3.0, 4.0)]),
    ];
    let chart = Chart::new().series_vec(series);
    assert_eq!(chart.series.len(), 2);
}

#[test]
fn test_chart_series_vec_empty() {
    let series: Vec<Series> = vec![];
    let chart = Chart::new().series_vec(series);
    assert_eq!(chart.series.len(), 0);
}

// =========================================================================
// Chart::x_axis / y_axis tests
// =========================================================================

#[test]
fn test_chart_x_axis() {
    let axis = Axis::new().title("X Axis");
    let chart = Chart::new().x_axis(axis.clone());
    assert_eq!(chart.x_axis.title, Some("X Axis".to_string()));
}

#[test]
fn test_chart_y_axis() {
    let axis = Axis::new().title("Y Axis");
    let chart = Chart::new().y_axis(axis.clone());
    assert_eq!(chart.y_axis.title, Some("Y Axis".to_string()));
}

#[test]
fn test_chart_both_axes() {
    let chart = Chart::new()
        .x_axis(Axis::new().title("X"))
        .y_axis(Axis::new().title("Y"));
    assert_eq!(chart.x_axis.title, Some("X".to_string()));
    assert_eq!(chart.y_axis.title, Some("Y".to_string()));
}

// =========================================================================
// Chart::legend tests
// =========================================================================

#[test]
fn test_chart_legend() {
    let chart = Chart::new().legend(LegendPosition::BottomLeft);
    assert_eq!(chart.legend, LegendPosition::BottomLeft);
}

#[test]
fn test_chart_legend_top_right() {
    let chart = Chart::new().legend(LegendPosition::TopRight);
    assert_eq!(chart.legend, LegendPosition::TopRight);
}

#[test]
fn test_chart_no_legend() {
    let chart = Chart::new().no_legend();
    assert_eq!(chart.legend, LegendPosition::None);
}

#[test]
fn test_chart_legend_all_positions() {
    let positions = vec![
        LegendPosition::TopLeft,
        LegendPosition::TopCenter,
        LegendPosition::TopRight,
        LegendPosition::BottomLeft,
        LegendPosition::BottomCenter,
        LegendPosition::BottomRight,
        LegendPosition::Left,
        LegendPosition::Right,
        LegendPosition::None,
    ];

    for pos in positions {
        let chart = Chart::new().legend(pos);
        assert_eq!(chart.legend, pos);
    }
}

// =========================================================================
// Chart::bg tests
// =========================================================================

#[test]
fn test_chart_bg() {
    let chart = Chart::new().bg(Color::BLUE);
    assert_eq!(chart.bg_color, Some(Color::BLUE));
}

#[test]
fn test_chart_bg_multiple() {
    let chart = Chart::new().bg(Color::RED).bg(Color::GREEN);
    assert_eq!(chart.bg_color, Some(Color::GREEN));
}

// =========================================================================
// Chart::border tests
// =========================================================================

#[test]
fn test_chart_border() {
    let chart = Chart::new().border(Color::WHITE);
    assert_eq!(chart.border_color, Some(Color::WHITE));
}

#[test]
fn test_chart_border_multiple() {
    let chart = Chart::new().border(Color::YELLOW).border(Color::CYAN);
    assert_eq!(chart.border_color, Some(Color::CYAN));
}

// =========================================================================
// Chart::braille tests
// =========================================================================

#[test]
fn test_chart_braille() {
    let chart = Chart::new().braille();
    assert!(chart.braille_mode);
}

#[test]
fn test_chart_braille_default_false() {
    let chart = Chart::new();
    assert!(!chart.braille_mode);
}

// =========================================================================
// Chart::Default tests
// =========================================================================

#[test]
fn test_chart_default() {
    let chart = Chart::default();
    assert!(chart.series.is_empty());
    assert!(chart.title.is_none());
    assert_eq!(chart.legend, LegendPosition::TopRight);
}

// =========================================================================
// Chart::render tests
// =========================================================================

#[test]
fn test_chart_render_basic() {
    let chart = Chart::new().series(Series::new("Test").data(vec![(1.0, 2.0), (3.0, 4.0)]));

    let mut buffer = Buffer::new(40, 20);
    let rect = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, rect);

    chart.render(&mut ctx);
    // Should render without panic
}

#[test]
fn test_chart_render_with_title() {
    let chart = Chart::new()
        .title("Test Chart")
        .series(Series::new("Test").data(vec![(1.0, 2.0)]));

    let mut buffer = Buffer::new(40, 20);
    let rect = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, rect);

    chart.render(&mut ctx);

    // Title should be rendered centered at top
    // "Test Chart" is 10 chars, in a 40-char wide buffer, centered at position 15
    assert_eq!(buffer.get(15, 0).unwrap().symbol, 'T');
    assert_eq!(buffer.get(16, 0).unwrap().symbol, 'e');
}

#[test]
fn test_chart_render_with_background() {
    let chart = Chart::new()
        .bg(Color::BLUE)
        .series(Series::new("Test").data(vec![(1.0, 2.0)]));

    let mut buffer = Buffer::new(40, 20);
    let rect = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, rect);

    chart.render(&mut ctx);
    // Background should be set
}

#[test]
fn test_chart_render_with_border() {
    let chart = Chart::new()
        .border(Color::WHITE)
        .series(Series::new("Test").data(vec![(1.0, 2.0)]));

    let mut buffer = Buffer::new(40, 20);
    let rect = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, rect);

    chart.render(&mut ctx);

    // Border corners should be rendered
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
    assert_eq!(buffer.get(39, 0).unwrap().symbol, '┐');
    assert_eq!(buffer.get(0, 19).unwrap().symbol, '└');
    assert_eq!(buffer.get(39, 19).unwrap().symbol, '┘');
}

#[test]
fn test_chart_render_too_small() {
    let chart = Chart::new().series(Series::new("Test").data(vec![(1.0, 2.0)]));

    let mut buffer = Buffer::new(5, 2);
    let rect = Rect::new(0, 0, 5, 2);
    let mut ctx = RenderContext::new(&mut buffer, rect);

    chart.render(&mut ctx);
    // Should return early without rendering
}

#[test]
fn test_chart_render_empty_series() {
    let chart = Chart::new();

    let mut buffer = Buffer::new(40, 20);
    let rect = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, rect);

    chart.render(&mut ctx);
    // Should render axes even without data
}

#[test]
fn test_chart_render_with_legend() {
    let chart = Chart::new()
        .series(Series::new("Series A").data(vec![(1.0, 2.0)]))
        .series(Series::new("Series B").data(vec![(3.0, 4.0)]));

    let mut buffer = Buffer::new(40, 20);
    let rect = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, rect);

    chart.render(&mut ctx);
    // Legend should be rendered by default
}

#[test]
fn test_chart_render_no_legend() {
    let chart = Chart::new()
        .no_legend()
        .series(Series::new("Test").data(vec![(1.0, 2.0)]));

    let mut buffer = Buffer::new(40, 20);
    let rect = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, rect);

    chart.render(&mut ctx);
    // Legend should not be rendered
}

#[test]
fn test_chart_render_line_chart() {
    let chart = Chart::new().series(
        Series::new("Test")
            .data(vec![(1.0, 2.0), (3.0, 4.0), (5.0, 6.0)])
            .line(),
    );

    let mut buffer = Buffer::new(40, 20);
    let rect = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, rect);

    chart.render(&mut ctx);
    // Should render lines
}

#[test]
fn test_chart_render_area_chart() {
    let chart = Chart::new().series(
        Series::new("Test")
            .data(vec![(1.0, 2.0), (3.0, 4.0), (5.0, 6.0)])
            .area(Color::BLUE),
    );

    let mut buffer = Buffer::new(40, 20);
    let rect = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, rect);

    chart.render(&mut ctx);
    // Should render area fill
}

#[test]
fn test_chart_render_scatter_chart() {
    let chart = Chart::new().series(
        Series::new("Test")
            .data(vec![(1.0, 2.0), (3.0, 4.0), (5.0, 6.0)])
            .scatter(),
    );

    let mut buffer = Buffer::new(40, 20);
    let rect = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, rect);

    chart.render(&mut ctx);
    // Should render markers
}

#[test]
fn test_chart_render_step() {
    let chart = Chart::new().series(
        Series::new("Test")
            .data(vec![(1.0, 2.0), (3.0, 4.0), (5.0, 6.0)])
            .step(),
    );

    let mut buffer = Buffer::new(40, 20);
    let rect = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, rect);

    chart.render(&mut ctx);
    // Should render step lines
}

#[test]
fn test_chart_render_step_before_variant() {
    let chart = Chart::new().series(
        Series::new("Test")
            .data(vec![(1.0, 2.0), (3.0, 4.0), (5.0, 6.0)])
            .chart_type(ChartType::StepBefore),
    );

    let mut buffer = Buffer::new(40, 20);
    let rect = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, rect);

    chart.render(&mut ctx);
    // Should render step lines
}

#[test]
fn test_chart_render_with_axis_titles() {
    let chart = Chart::new()
        .x_axis(Axis::new().title("X Axis"))
        .y_axis(Axis::new().title("Y Axis"))
        .series(Series::new("Test").data(vec![(1.0, 2.0)]));

    let mut buffer = Buffer::new(40, 20);
    let rect = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, rect);

    chart.render(&mut ctx);
    // Should render axis titles
}

#[test]
fn test_chart_render_with_grid() {
    let chart = Chart::new().series(Series::new("Test").data(vec![(1.0, 2.0), (10.0, 20.0)]));

    let mut buffer = Buffer::new(40, 20);
    let rect = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, rect);

    chart.render(&mut ctx);
    // Should render grid lines
}

#[test]
fn test_chart_render_with_markers() {
    let chart = Chart::new().series(
        Series::new("Test")
            .data(vec![(1.0, 2.0), (3.0, 4.0)])
            .marker(Marker::Circle),
    );

    let mut buffer = Buffer::new(40, 20);
    let rect = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, rect);

    chart.render(&mut ctx);
    // Should render markers
}

// =========================================================================
// Helper function tests
// =========================================================================

#[test]
fn test_chart_helper() {
    let chart = chart();
    assert!(chart.series.is_empty());
}

#[test]
fn test_line_chart_helper() {
    let data = vec![1.0, 2.0, 3.0, 4.0];
    let chart = line_chart(&data);
    assert_eq!(chart.series.len(), 1);
    assert_eq!(chart.series[0].name, "Data");
    // Verify it's a line chart by checking line style
    assert_eq!(chart.series[0].line_style, LineStyle::Solid);
}

#[test]
fn test_line_chart_helper_empty() {
    let data: Vec<f64> = vec![];
    let chart = line_chart(&data);
    assert_eq!(chart.series.len(), 1);
}

#[test]
fn test_scatter_plot_helper() {
    let data = vec![(1.0, 2.0), (3.0, 4.0), (5.0, 6.0)];
    let chart = scatter_plot(&data);
    assert_eq!(chart.series.len(), 1);
    assert_eq!(chart.series[0].name, "Data");
}

#[test]
fn test_scatter_plot_helper_empty() {
    let data: Vec<(f64, f64)> = vec![];
    let chart = scatter_plot(&data);
    assert_eq!(chart.series.len(), 1);
}

// =========================================================================
// Integration tests
// =========================================================================

#[test]
fn test_chart_complex_render() {
    let chart = Chart::new()
        .title("Complex Chart")
        .x_axis(Axis::new().title("Time").min(0.0).max(10.0))
        .y_axis(Axis::new().title("Value").min(0.0).max(100.0))
        .legend(LegendPosition::BottomRight)
        .border(Color::WHITE)
        .bg(Color::BLACK)
        .series(
            Series::new("Series 1")
                .data(vec![(1.0, 20.0), (5.0, 50.0), (9.0, 80.0)])
                .color(Color::RED)
                .marker(Marker::Circle)
                .line(),
        )
        .series(
            Series::new("Series 2")
                .data(vec![(1.0, 30.0), (5.0, 60.0), (9.0, 90.0)])
                .color(Color::BLUE)
                .marker(Marker::Square)
                .line(),
        );

    let mut buffer = Buffer::new(60, 30);
    let rect = Rect::new(0, 0, 60, 30);
    let mut ctx = RenderContext::new(&mut buffer, rect);

    chart.render(&mut ctx);
    // Complex chart should render without panic
}

#[test]
fn test_chart_view_meta() {
    let chart = Chart::new();
    // View impl_view_meta! macro creates meta() method
    let meta = chart.meta();
    assert_eq!(meta.widget_type, "Chart");
}

// =========================================================================
// ChartType enum tests (derived traits)
// =========================================================================

#[test]
fn test_chart_type_clone() {
    let ct1 = ChartType::Line;
    let ct2 = ct1.clone();
    assert_eq!(ct1, ct2);
}

#[test]
fn test_chart_type_copy() {
    let ct1 = ChartType::Area;
    let ct2 = ct1;
    assert_eq!(ct2, ChartType::Area);
}

#[test]
fn test_chart_type_partial_eq() {
    assert_eq!(ChartType::Line, ChartType::Line);
    assert_eq!(ChartType::Scatter, ChartType::Scatter);
    assert_ne!(ChartType::Line, ChartType::Area);
}

// =========================================================================
// LineStyle enum tests (derived traits)
// =========================================================================

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
    assert_eq!(LineStyle::Dashed, LineStyle::Dashed);
    assert_ne!(LineStyle::Solid, LineStyle::Dotted);
}