//! Histogram frame tests: title, background, grid, empty data, small, zero and offset areas

use super::super::{
    assert_same_when_offset, assert_zero_areas_draw_nothing, render, render_rows, rows,
};
use super::three_bins;
use revue::style::Color;
use revue::widget::data::chart::{ChartGrid, Histogram};
use revue::widget::traits::View;

#[test]
fn test_histogram_title() {
    let rows = render_rows(&three_bins().title("Distribution"), 40, 20);
    // Centered: (40 - 12) / 2 = 14; the plot moves down one row
    assert_eq!(
        rows[0].trim_end(),
        format!("{}Distribution", " ".repeat(14))
    );
}

#[test]
fn test_histogram_bg() {
    let buffer = render(&three_bins().bg(Color::BLUE), 40, 20);
    assert_eq!(buffer.get(39, 0).unwrap().bg, Some(Color::BLUE));
}

#[test]
fn test_histogram_render_with_grid() {
    use revue::layout::Rect;
    use revue::render::Buffer;
    use revue::widget::traits::RenderContext;

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
fn test_histogram_empty_renders_nothing() {
    let buffer = render(&Histogram::new(&[]), 40, 20);
    assert!(rows(&buffer).iter().all(|r| r.trim().is_empty()));
}

#[test]
fn test_histogram_too_small_renders_nothing() {
    for (w, h) in [(14, 20), (40, 4)] {
        let buffer = render(&three_bins(), w, h);
        assert!(rows(&buffer).iter().all(|r| r.trim().is_empty()), "{w}x{h}");
    }
}

#[test]
fn test_histogram_zero_area_draws_nothing() {
    assert_zero_areas_draw_nothing(&three_bins().title("T").show_stats(true));
}

#[test]
fn test_histogram_draws_the_same_when_offset() {
    let hist = three_bins().title("Distribution").show_stats(true);
    assert_same_when_offset(hist.clone(), 40, 20);
    assert_same_when_offset(hist.horizontal(), 40, 20);
}
