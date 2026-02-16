//! Pie chart tests extracted from chart/piechart.rs

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::data::chart::piechart::{PieChart, PieSlice, PieStyle, PieLabelStyle, Legend, LegendPosition};
use revue::widget::traits::RenderContext;

#[test]
fn test_pie_chart_new() {
    let chart = PieChart::new();
    assert!(chart.slices.is_empty());
    assert_eq!(chart.style, PieStyle::Pie);
    assert_eq!(chart.start_angle, -90.0);
}

#[test]
fn test_pie_chart_slices() {
    let chart = PieChart::new()
        .slice("A", 30.0)
        .slice("B", 50.0)
        .slice("C", 20.0);

    assert_eq!(chart.slices.len(), 3);
    assert_eq!(chart.total(), 100.0);
}

#[test]
fn test_pie_chart_slice_angles() {
    let chart = PieChart::new()
        .slice("A", 25.0)
        .slice("B", 25.0)
        .slice("C", 25.0)
        .slice("D", 25.0);

    assert_eq!(chart.slice_angle(25.0), 90.0);
}

#[test]
fn test_pie_chart_colors() {
    let chart = PieChart::new()
        .slice("A", 30.0)
        .slice_colored("B", 50.0, Color::RED);

    // First slice uses palette
    let color0 = chart.slice_color(0);
    assert_ne!(color0.r, 0);

    // Second slice uses custom color
    let color1 = chart.slice_color(1);
    assert_eq!(color1.r, 255);
}

#[test]
fn test_donut_chart() {
    let chart = PieChart::new().donut(0.6);

    assert_eq!(chart.style, PieStyle::Donut);
    assert_eq!(chart.donut_ratio, 0.6);
}

#[test]
fn test_pie_chart_explode() {
    let chart = PieChart::new().slice("A", 30.0).slice("B", 50.0).explode(0);

    assert_eq!(chart.explode, Some(0));
}

#[test]
fn test_pie_chart_labels() {
    let chart = PieChart::new().labels(PieLabelStyle::Percent);
    assert_eq!(chart.labels, PieLabelStyle::Percent);
}

#[test]
fn test_pie_chart_legend() {
    let chart = PieChart::new().legend(Legend::bottom_left());
    assert_eq!(chart.legend.position, LegendPosition::BottomLeft);

    let chart = PieChart::new().no_legend();
    assert!(!chart.legend.is_visible());
}

#[test]
fn test_pie_chart_builder_chain() {
    let chart = PieChart::new()
        .title("Sales")
        .slice("Product A", 100.0)
        .slice("Product B", 200.0)
        .slice("Product C", 150.0)
        .donut(0.4)
        .labels(PieLabelStyle::LabelPercent)
        .legend(Legend::right())
        .explode(1)
        .start_angle(0.0);

    assert_eq!(chart.title, Some("Sales".to_string()));
    assert_eq!(chart.slices.len(), 3);
    assert_eq!(chart.style, PieStyle::Donut);
    assert_eq!(chart.donut_ratio, 0.4);
    assert_eq!(chart.labels, PieLabelStyle::LabelPercent);
    assert_eq!(chart.explode, Some(1));
    assert_eq!(chart.start_angle, 0.0);
}

#[test]
fn test_pie_helpers() {
    let chart = pie_chart();
    assert_eq!(chart.style, PieStyle::Pie);

    let chart = donut_chart();
    assert_eq!(chart.style, PieStyle::Donut);
    assert_eq!(chart.donut_ratio, 0.5);
}

#[test]
fn test_pie_slice_struct() {
    let slice = PieSlice::new("Test", 42.0);
    assert_eq!(slice.label, "Test");
    assert_eq!(slice.value, 42.0);
    assert!(slice.color.is_none());

    let slice = PieSlice::with_color("Colored", 100.0, Color::BLUE);
    assert!(slice.color.is_some());
}

// ========== Render Tests ==========

#[test]
fn test_pie_chart_render_basic() {
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::widget::traits::RenderContext;

    let mut buffer = Buffer::new(30, 15);
    let area = Rect::new(0, 0, 30, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let chart = PieChart::new().slice("A", 50.0).slice("B", 50.0);

    chart.render(&mut ctx);

    // Verify something was rendered (not all spaces)
    let mut has_content = false;
    for y in 0..15 {
        for x in 0..30 {
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
fn test_pie_chart_render_with_title() {
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::widget::traits::RenderContext;

    let mut buffer = Buffer::new(30, 15);
    let area = Rect::new(0, 0, 30, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let chart = PieChart::new().title("Test Chart").slice("A", 100.0);

    chart.render(&mut ctx);

    // Title should be rendered at the top
    let mut title_found = false;
    for x in 0..30 {
        if let Some(cell) = buffer.get(x, 0) {
            if cell.symbol == 'T' {
                title_found = true;
                break;
            }
        }
    }
    assert!(title_found);
}

#[test]
fn test_pie_chart_render_donut() {
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::widget::traits::RenderContext;

    let mut buffer = Buffer::new(30, 15);
    let area = Rect::new(0, 0, 30, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let chart = donut_chart().slice("A", 50.0).slice("B", 50.0);

    chart.render(&mut ctx);

    // Verify donut renders (has content)
    let mut has_content = false;
    for y in 0..15 {
        for x in 0..30 {
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
fn test_pie_chart_render_small_area() {
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::widget::traits::RenderContext;

    // Very small area - should handle gracefully
    let mut buffer = Buffer::new(5, 3);
    let area = Rect::new(0, 0, 5, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let chart = PieChart::new().slice("A", 100.0);

    // Should not panic
    chart.render(&mut ctx);
}

#[test]
fn test_pie_chart_render_with_legend() {
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::widget::traits::RenderContext;

    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let chart = PieChart::new()
        .slice("Alpha", 50.0)
        .slice("Beta", 30.0)
        .slice("Gamma", 20.0)
        .legend(Legend::bottom_center());

    chart.render(&mut ctx);

    // Verify legend area has content (look for legend markers)
    let mut legend_found = false;
    for y in 15..20 {
        for x in 0..40 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '■' || cell.symbol == '●' {
                    legend_found = true;
                    break;
                }
            }
        }
    }
    assert!(legend_found);
}

#[test]
fn test_pie_chart_render_empty() {
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::widget::traits::RenderContext;

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // Empty chart - should not panic
    let chart = PieChart::new();
    chart.render(&mut ctx);
}

#[test]
fn test_pie_chart_render_labels() {
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::widget::traits::RenderContext;

    let mut buffer = Buffer::new(50, 25);
    let area = Rect::new(0, 0, 50, 25);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let chart = PieChart::new()
        .slice("A", 50.0)
        .slice("B", 50.0)
        .labels(PieLabelStyle::Percent);

    chart.render(&mut ctx);

    // Check if percentage labels are rendered (look for %)
    let mut percent_found = false;
    for y in 0..25 {
        for x in 0..50 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '%' {
                    percent_found = true;
                    break;
                }
            }
        }
    }
    assert!(percent_found);
}