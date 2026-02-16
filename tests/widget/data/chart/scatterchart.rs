//! Scatter Chart widget public API tests extracted from scatterchart.rs

use crate::layout::Rect;
use crate::render::Buffer;
use crate::style::Color;
use crate::widget::traits::RenderContext;

pub use crate::widget::data::chart::scatterchart::{ScatterChart, ScatterSeries, Marker};
pub use revue::widget::data::chart::chart_common::{Axis, LegendPosition};
pub use revue::widget::data::chart::chart_common::ChartGrid;

#[test]
fn test_scatter_chart_new() {
    let chart = ScatterChart::new();
    assert!(chart.series.is_empty());
}

#[test]
fn test_scatter_series() {
    let series = ScatterSeries::new("Test")
        .points(&[(1.0, 2.0), (3.0, 4.0), (5.0, 6.0)])
        .color(Color::RED)
        .marker(Marker::Star);

    assert_eq!(series.name, "Test");
    assert_eq!(series.data.len(), 3);
    assert_eq!(series.color, Some(Color::RED));
    assert_eq!(series.marker, Marker::Star);
}

#[test]
fn test_scatter_chart_series() {
    let chart = ScatterChart::new()
        .series(ScatterSeries::new("A").points(&[(1.0, 1.0)]))
        .series(ScatterSeries::new("B").points(&[(2.0, 2.0)]));

    assert_eq!(chart.series.len(), 2);
}

#[test]
fn test_scatter_chart_bounds() {
    let chart = ScatterChart::new()
        .series(ScatterSeries::new("Test").points(&[(0.0, 0.0), (10.0, 20.0)]));

    let (x_min, x_max, y_min, y_max) = chart.compute_bounds();
    assert!(x_min < 0.0);
    assert!(x_max > 10.0);
    assert!(y_min < 0.0);
    assert!(y_max > 20.0);
}

#[test]
fn test_scatter_chart_axis_override() {
    let chart = ScatterChart::new()
        .series(ScatterSeries::new("Test").points(&[(5.0, 5.0)]))
        .x_axis(Axis::new().bounds(0.0, 100.0))
        .y_axis(Axis::new().bounds(0.0, 50.0));

    let (x_min, x_max, y_min, y_max) = chart.compute_bounds();
    assert!(x_min <= 0.0);
    assert!(x_max >= 100.0);
    assert!(y_min <= 0.0);
    assert!(y_max >= 50.0);
}

#[test]
fn test_bubble_chart() {
    let series = ScatterSeries::new("Bubbles")
        .points(&[(1.0, 1.0), (2.0, 2.0)])
        .sizes(vec![10.0, 50.0]);

    assert!(series.sizes.is_some());
    assert_eq!(series.sizes.as_ref().unwrap().len(), 2);
}

#[test]
fn test_scatter_chart_legend() {
    let chart = ScatterChart::new().legend(LegendPosition::BottomLeft);
    assert_eq!(chart.legend.position, LegendPosition::BottomLeft);

    let chart = ScatterChart::new().no_legend();
    assert!(!chart.legend.is_visible());
}

#[test]
fn test_scatter_chart_grid() {
    let chart = ScatterChart::new().grid(ChartGrid::y_only());
    assert!(!chart.grid.x);
    assert!(chart.grid.y);
}

#[test]
fn test_scatter_chart_builder() {
    let chart = ScatterChart::new()
        .title("Scatter Plot")
        .series(ScatterSeries::new("Data").points(&[(1.0, 1.0)]))
        .x_axis(Axis::new().title("X"))
        .y_axis(Axis::new().title("Y"))
        .legend(LegendPosition::top_right())
        .grid(ChartGrid::both());

    assert_eq!(chart.title, Some("Scatter Plot".to_string()));
    assert_eq!(chart.series.len(), 1);
    assert!(chart.grid.x);
    assert!(chart.grid.y);
}

#[test]
fn test_scatter_helpers() {
    let chart = scatter_chart();
    assert!(chart.series.is_empty());

    let chart = bubble_chart();
    assert!(chart.series.is_empty());
}

#[test]
fn test_scatter_chart_render_basic() {
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let chart = ScatterChart::new().series(ScatterSeries::new("Data").points(&[
        (1.0, 2.0),
        (3.0, 4.0),
        (5.0, 3.0),
        (7.0, 6.0),
    ]));

    chart.render(&mut ctx);

    // Verify something was rendered
    let mut has_content = false;
    for y in 0..20 {
        for x in 0..40 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol != ' ' {
                    has_content = true;
                    break;
                }
            }
        }
    }
    assert!(has_content);
}

#[test]
fn test_scatter_chart_render_with_title() {
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let chart = ScatterChart::new()
        .title("My Scatter")
        .series(ScatterSeries::new("D").points(&[(1.0, 1.0)]));

    chart.render(&mut ctx);

    // Title should be rendered
    let mut title_found = false;
    for x in 0..40 {
        if let Some(cell) = buffer.get(x, 0) {
            if cell.symbol == 'M' {
                title_found = true;
                break;
            }
        }
    }
    assert!(title_found);
}

#[test]
fn test_scatter_chart_render_multiple_series() {
    let mut buffer = Buffer::new(50, 25);
    let area = Rect::new(0, 0, 50, 25);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let chart = ScatterChart::new()
        .series(ScatterSeries::new("Series A").points(&[(1.0, 1.0), (2.0, 2.0)]))
        .series(ScatterSeries::new("Series B").points(&[(3.0, 3.0), (4.0, 4.0)]))
        .legend(LegendPosition::top_right());

    chart.render(&mut ctx);

    // Should render without panic
    let mut has_content = false;
    for y in 0..25 {
        for x in 0..50 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol != ' ' {
                    has_content = true;
                    break;
                }
            }
        }
    }
    assert!(has_content);
}

#[test]
fn test_scatter_chart_render_with_grid() {
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let chart = ScatterChart::new()
        .series(ScatterSeries::new("D").points(&[(5.0, 5.0)]))
        .grid(ChartGrid::both());

    chart.render(&mut ctx);

    // Look for grid characters
    let mut grid_found = false;
    for y in 0..20 {
        for x in 0..40 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '─' || cell.symbol == '│' || cell.symbol == '┼' {
                    grid_found = true;
                    break;
                }
            }
        }
    }
    assert!(grid_found);
}

#[test]
fn test_scatter_chart_render_small_area() {
    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let chart = ScatterChart::new().series(ScatterSeries::new("D").points(&[(1.0, 1.0)]));

    // Should not panic on small area
    chart.render(&mut ctx);
}

#[test]
fn test_scatter_chart_render_empty() {
    let mut buffer = Buffer::new(30, 15);
    let area = Rect::new(0, 0, 30, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // Empty chart
    let chart = ScatterChart::new();
    chart.render(&mut ctx);
}

#[test]
fn test_scatter_chart_render_with_markers() {
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let chart = ScatterChart::new().series(
        ScatterSeries::new("Stars")
            .points(&[(5.0, 5.0), (10.0, 10.0)])
            .marker(Marker::Star),
    );

    chart.render(&mut ctx);

    // Look for star markers
    let mut marker_found = false;
    for y in 0..20 {
        for x in 0..40 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '★' {
                    marker_found = true;
                    break;
                }
            }
        }
    }
    assert!(marker_found);
}

#[test]
fn test_scatter_chart_render_bubble() {
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let chart = bubble_chart().series(
        ScatterSeries::new("Bubbles")
            .points(&[(5.0, 5.0), (10.0, 10.0)])
            .sizes(vec![1.0, 5.0]),
    );

    chart.render(&mut ctx);

    // Should render without panic
    let mut has_content = false;
    for y in 0..20 {
        for x in 0..40 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol != ' ' {
                    has_content = true;
                    break;
                }
            }
        }
    }
    assert!(has_content);
}