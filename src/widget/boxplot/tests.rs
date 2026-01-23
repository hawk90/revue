//! Tests for boxplot module

use super::super::chart_stats::percentile;
use super::*;

#[test]
fn test_boxplot_new() {
    let bp = BoxPlot::new();
    assert!(bp.groups.is_empty());
}

#[test]
fn test_boxplot_group() {
    let bp = BoxPlot::new()
        .group("A", &[1.0, 2.0, 3.0, 4.0, 5.0])
        .group("B", &[2.0, 3.0, 4.0, 5.0, 6.0]);

    assert_eq!(bp.groups.len(), 2);
}

#[test]
fn test_boxstats_from_data() {
    let data = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    let stats = BoxStats::from_data(&data, WhiskerStyle::MinMax).unwrap();

    assert_eq!(stats.min, 1.0);
    assert_eq!(stats.max, 10.0);
    assert_eq!(stats.median, 5.5);
}

#[test]
fn test_boxstats_quartiles() {
    let data = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let stats = BoxStats::from_data(&data, WhiskerStyle::MinMax).unwrap();

    assert!(stats.q1 >= 2.0 && stats.q1 <= 3.0);
    assert!(stats.q3 >= 6.0 && stats.q3 <= 7.0);
}

#[test]
fn test_boxstats_outliers() {
    let data = [1.0, 2.0, 3.0, 4.0, 5.0, 100.0]; // 100 is an outlier
    let stats = BoxStats::from_data(&data, WhiskerStyle::IQR).unwrap();

    assert!(!stats.outliers.is_empty());
    assert!(stats.outliers.contains(&100.0));
}

#[test]
fn test_boxplot_orientation() {
    let bp = BoxPlot::new().horizontal();
    assert_eq!(bp.orientation, ChartOrientation::Horizontal);

    let bp = BoxPlot::new().vertical();
    assert_eq!(bp.orientation, ChartOrientation::Vertical);
}

#[test]
fn test_boxplot_whisker_style() {
    let bp = BoxPlot::new().whisker_style(WhiskerStyle::MinMax);
    assert_eq!(bp.whisker_style, WhiskerStyle::MinMax);

    let bp = BoxPlot::new().whisker_style(WhiskerStyle::Percentile);
    assert_eq!(bp.whisker_style, WhiskerStyle::Percentile);
}

#[test]
fn test_boxplot_notched() {
    let bp = BoxPlot::new().notched(true);
    assert!(bp.notched);
}

#[test]
fn test_boxplot_show_outliers() {
    let bp = BoxPlot::new().show_outliers(false);
    assert!(!bp.show_outliers);
}

#[test]
fn test_boxplot_box_width() {
    let bp = BoxPlot::new().box_width(0.8);
    assert_eq!(bp.box_width, 0.8);
}

#[test]
fn test_boxplot_builder() {
    let bp = BoxPlot::new()
        .title("Distribution")
        .group("A", &[1.0, 2.0, 3.0])
        .group("B", &[4.0, 5.0, 6.0])
        .value_axis(Axis::new().title("Value"))
        .whisker_style(WhiskerStyle::IQR)
        .show_outliers(true)
        .notched(false);

    assert_eq!(bp.title, Some("Distribution".to_string()));
    assert_eq!(bp.groups.len(), 2);
}

#[test]
fn test_boxplot_helper() {
    let bp = boxplot();
    assert!(bp.groups.is_empty());
}

#[test]
fn test_percentile() {
    let sorted = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    assert_eq!(percentile(&sorted, 0.0), 1.0);
    assert_eq!(percentile(&sorted, 50.0), 3.0);
    assert_eq!(percentile(&sorted, 100.0), 5.0);
}

#[test]
fn test_box_group() {
    let group = BoxGroup::new("Test", &[1.0, 2.0, 3.0]);
    assert_eq!(group.label, "Test");
    assert_eq!(group.data.len(), 3);

    let group = group.color(Color::RED);
    assert_eq!(group.color, Some(Color::RED));
}

// ========== Render Tests ==========

#[test]
fn test_boxplot_render_basic() {
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::widget::traits::RenderContext;

    let data: Vec<f64> = (0..50).map(|x| x as f64).collect();
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let bp = BoxPlot::new().group("Data", &data);
    bp.render(&mut ctx);

    // Verify box elements are rendered
    let mut has_box = false;
    for y in 0..20 {
        for x in 0..40 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '┌'
                    || cell.symbol == '┐'
                    || cell.symbol == '└'
                    || cell.symbol == '┘'
                    || cell.symbol == '│'
                    || cell.symbol == '─'
                {
                    has_box = true;
                    break;
                }
            }
        }
    }
    assert!(has_box);
}

#[test]
fn test_boxplot_render_multiple_groups() {
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::widget::traits::RenderContext;

    let mut buffer = Buffer::new(60, 25);
    let area = Rect::new(0, 0, 60, 25);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let bp = BoxPlot::new()
        .group("Group A", &[1.0, 2.0, 3.0, 4.0, 5.0])
        .group("Group B", &[3.0, 4.0, 5.0, 6.0, 7.0])
        .group("Group C", &[5.0, 6.0, 7.0, 8.0, 9.0]);

    bp.render(&mut ctx);

    // Should render without panic
    let mut has_content = false;
    for y in 0..25 {
        for x in 0..60 {
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
fn test_boxplot_render_with_title() {
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::widget::traits::RenderContext;

    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let bp = BoxPlot::new()
        .title("Test BoxPlot")
        .group("D", &[1.0, 2.0, 3.0, 4.0, 5.0]);

    bp.render(&mut ctx);

    // Title should be rendered
    let mut title_found = false;
    for x in 0..40 {
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
fn test_boxplot_render_with_outliers() {
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::widget::traits::RenderContext;

    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // Data with outliers
    let mut data: Vec<f64> = (0..20).map(|x| x as f64).collect();
    data.push(100.0); // Outlier
    data.push(-50.0); // Outlier

    let bp = BoxPlot::new()
        .group("Data", &data)
        .show_outliers(true)
        .whisker_style(WhiskerStyle::IQR);

    bp.render(&mut ctx);

    // Should render without panic and show outliers
    let mut has_outliers = false;
    for y in 0..20 {
        for x in 0..40 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '○' {
                    has_outliers = true;
                    break;
                }
            }
        }
    }
    assert!(has_outliers);
}

#[test]
fn test_boxplot_render_horizontal() {
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::widget::traits::RenderContext;

    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let bp = BoxPlot::new()
        .group("Data", &[1.0, 2.0, 3.0, 4.0, 5.0])
        .horizontal();

    bp.render(&mut ctx);

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

#[test]
fn test_boxplot_render_small_area() {
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::widget::traits::RenderContext;

    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let bp = BoxPlot::new().group("D", &[1.0, 2.0, 3.0]);

    // Should not panic on small area
    bp.render(&mut ctx);
}

#[test]
fn test_boxplot_render_empty() {
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::widget::traits::RenderContext;

    let mut buffer = Buffer::new(30, 15);
    let area = Rect::new(0, 0, 30, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // Empty boxplot
    let bp = BoxPlot::new();
    bp.render(&mut ctx);
}

#[test]
fn test_boxplot_render_notched() {
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::widget::traits::RenderContext;

    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let data: Vec<f64> = (0..30).map(|x| x as f64).collect();
    let bp = BoxPlot::new().group("Data", &data).notched(true);

    bp.render(&mut ctx);

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

#[test]
fn test_boxplot_render_minmax_whiskers() {
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::widget::traits::RenderContext;

    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let data: Vec<f64> = (0..20).map(|x| x as f64).collect();
    let bp = BoxPlot::new()
        .group("Data", &data)
        .whisker_style(WhiskerStyle::MinMax);

    bp.render(&mut ctx);

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
