//! Shared chart frame tests: title, background and plot area

use super::super::rows;
use super::draw;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::data::chart::chart_render::{
    calculate_chart_area, fill_background, render_title,
};
use revue::widget::traits::RenderContext;

#[test]
fn test_render_title() {
    let mut offset = 0;
    let buffer = draw(30, 10, |ctx, area| {
        offset = render_title(ctx, area, Some("Test Title"), Color::YELLOW);
    });
    assert_eq!(offset, 1);
    // Centered: (30 - 10) / 2 = 10
    assert_eq!(rows(&buffer)[0], format!("{:10}Test Title{:10}", "", ""));
    assert_eq!(buffer.get(10, 0).unwrap().fg, Some(Color::YELLOW));
}

#[test]
fn test_render_title_none() {
    let mut offset = 1;
    let buffer = draw(30, 10, |ctx, area| {
        offset = render_title(ctx, area, None, Color::WHITE);
    });
    assert_eq!(offset, 0);
    assert!(rows(&buffer).iter().all(|r| r.trim().is_empty()));
}

#[test]
fn test_render_title_long() {
    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let offset = render_title(&mut ctx, area, Some("Very Long Title"), Color::WHITE);
    assert_eq!(offset, 1);
}

#[test]
fn test_fill_background() {
    let buffer = draw(10, 5, |ctx, _| {
        fill_background(ctx, Rect::new(2, 1, 3, 2), Color::BLUE);
    });
    for y in 0..5 {
        for x in 0..10 {
            let inside = (2..5).contains(&x) && (1..3).contains(&y);
            let bg = buffer.get(x, y).unwrap().bg;
            assert_eq!(bg == Some(Color::BLUE), inside, "({x}, {y})");
        }
    }
}

#[test]
fn test_fill_background_zero_area() {
    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 0, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    fill_background(&mut ctx, area, Color::RED);
    // Should not crash
}

#[test]
fn test_calculate_chart_area() {
    let area = Rect::new(0, 0, 100, 50);
    assert_eq!(
        calculate_chart_area(area, true, 8, 2),
        Rect::new(8, 1, 91, 46)
    );
    assert_eq!(
        calculate_chart_area(area, false, 8, 2),
        Rect::new(8, 0, 91, 47)
    );
    // Too small: sizes saturate at zero
    let tiny = calculate_chart_area(Rect::new(0, 0, 5, 2), true, 8, 2);
    assert_eq!((tiny.width, tiny.height), (0, 0));
}

#[test]
fn test_calculate_chart_area_no_labels() {
    let area = Rect::new(0, 0, 100, 50);
    let chart_area = calculate_chart_area(area, false, 0, 0);
    assert_eq!(chart_area.x, 0);
    assert_eq!(chart_area.y, 0);
    assert_eq!(chart_area.width, 99);
    assert_eq!(chart_area.height, 49);
}
