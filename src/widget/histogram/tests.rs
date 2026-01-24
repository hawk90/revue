use super::*;

#[test]
fn test_histogram_new() {
    let hist = Histogram::new(&[1.0, 2.0, 3.0, 4.0, 5.0]);
    assert_eq!(hist.data.len(), 5);
    assert!(!hist.bins.is_empty());
}

#[test]
fn test_histogram_empty() {
    let hist = Histogram::new(&[]);
    assert!(hist.data.is_empty());
    assert!(hist.bins.is_empty());
}

#[test]
fn test_histogram_bins_auto() {
    let data: Vec<f64> = (0..100).map(|x| x as f64).collect();
    let hist = Histogram::new(&data);
    assert!(!hist.bins.is_empty());
}

#[test]
fn test_histogram_bins_count() {
    let data: Vec<f64> = (0..100).map(|x| x as f64).collect();
    let hist = Histogram::new(&data).bin_count(10);
    assert_eq!(hist.bins.len(), 10);
}

#[test]
fn test_histogram_bins_width() {
    let data: Vec<f64> = (0..100).map(|x| x as f64).collect();
    let hist = Histogram::new(&data).bin_width(10.0);
    assert_eq!(hist.bins.len(), 10);
}

#[test]
fn test_histogram_mean() {
    let hist = Histogram::new(&[1.0, 2.0, 3.0, 4.0, 5.0]);
    assert_eq!(hist.mean(), Some(3.0));
}

#[test]
fn test_histogram_median() {
    let hist = Histogram::new(&[1.0, 2.0, 3.0, 4.0, 5.0]);
    assert_eq!(hist.median(), Some(3.0));

    let hist = Histogram::new(&[1.0, 2.0, 3.0, 4.0]);
    assert_eq!(hist.median(), Some(2.5));
}

#[test]
fn test_histogram_density() {
    let hist = Histogram::new(&[1.0, 2.0, 3.0]).density(true);
    assert!(hist.density);
}

#[test]
fn test_histogram_cumulative() {
    let hist = Histogram::new(&[1.0, 2.0, 3.0]).cumulative(true);
    assert!(hist.cumulative);
}

#[test]
fn test_histogram_show_stats() {
    let hist = Histogram::new(&[1.0, 2.0, 3.0]).show_stats(true);
    assert!(hist.show_stats);
}

#[test]
fn test_histogram_builder() {
    let hist = Histogram::new(&[1.0, 2.0, 3.0])
        .title("Distribution")
        .bin_count(5)
        .color(Color::GREEN)
        .density(true)
        .show_stats(true)
        .x_axis(Axis::new().title("Value"))
        .y_axis(Axis::new().title("Density"));

    assert_eq!(hist.title, Some("Distribution".to_string()));
    assert!(hist.density);
    assert!(hist.show_stats);
}

#[test]
fn test_histogram_helper() {
    let hist = histogram(&[1.0, 2.0, 3.0]);
    assert_eq!(hist.data.len(), 3);
}

#[test]
fn test_histogram_orientation() {
    let hist = Histogram::new(&[1.0]).horizontal();
    assert_eq!(hist.orientation, ChartOrientation::Horizontal);

    let hist = Histogram::new(&[1.0]).vertical();
    assert_eq!(hist.orientation, ChartOrientation::Vertical);
}

// ========== Render Tests ==========

#[test]
fn test_histogram_render_basic() {
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::widget::traits::RenderContext;

    let data: Vec<f64> = (0..50)
        .map(|x| (x as f64) + (x as f64).sin() * 5.0)
        .collect();
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let hist = Histogram::new(&data).bin_count(10);
    hist.render(&mut ctx);

    // Verify bars are rendered (look for block characters)
    let mut has_bars = false;
    for y in 0..20 {
        for x in 0..40 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '█' || cell.symbol == '▓' || cell.symbol == '▒' {
                    has_bars = true;
                    break;
                }
            }
        }
    }
    assert!(has_bars);
}

#[test]
fn test_histogram_render_with_title() {
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::widget::traits::RenderContext;

    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let hist = Histogram::new(&[1.0, 2.0, 3.0, 4.0, 5.0]).title("Test Distribution");
    hist.render(&mut ctx);

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
fn test_histogram_render_with_stats() {
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::widget::traits::RenderContext;

    let mut buffer = Buffer::new(50, 25);
    let area = Rect::new(0, 0, 50, 25);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let data: Vec<f64> = (0..100).map(|x| x as f64).collect();
    let hist = Histogram::new(&data).show_stats(true).bin_count(10);
    hist.render(&mut ctx);

    // Should render without panic and have content
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
fn test_histogram_render_density() {
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::widget::traits::RenderContext;

    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let data: Vec<f64> = (0..50).map(|x| x as f64).collect();
    let hist = Histogram::new(&data).density(true);
    hist.render(&mut ctx);

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
fn test_histogram_render_cumulative() {
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::widget::traits::RenderContext;

    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let data: Vec<f64> = (0..50).map(|x| x as f64).collect();
    let hist = Histogram::new(&data).cumulative(true);
    hist.render(&mut ctx);

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
fn test_histogram_render_small_area() {
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::widget::traits::RenderContext;

    let mut buffer = Buffer::new(10, 3);
    let area = Rect::new(0, 0, 10, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let hist = Histogram::new(&[1.0, 2.0, 3.0]);
    // Should not panic on small area
    hist.render(&mut ctx);
}

#[test]
fn test_histogram_render_empty() {
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::widget::traits::RenderContext;

    let mut buffer = Buffer::new(30, 15);
    let area = Rect::new(0, 0, 30, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // Empty data
    let hist = Histogram::new(&[]);
    hist.render(&mut ctx);
}

#[test]
fn test_histogram_render_with_grid() {
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::widget::traits::RenderContext;

    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let data: Vec<f64> = (0..50).map(|x| x as f64).collect();
    let hist = Histogram::new(&data).grid(ChartGrid::both());
    hist.render(&mut ctx);

    // Should have grid lines
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
fn test_histogram_render_custom_bins() {
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::widget::traits::RenderContext;

    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let data: Vec<f64> = (0..100).map(|x| x as f64).collect();
    let hist = Histogram::new(&data).bins(BinConfig::Edges(vec![0.0, 25.0, 50.0, 75.0, 100.0]));
    hist.render(&mut ctx);

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
