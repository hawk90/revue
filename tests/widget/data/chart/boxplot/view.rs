//! BoxPlot frame tests: title, background, orientation, empty, small, zero and offset areas

use super::super::{assert_same_when_offset, assert_zero_areas_draw_nothing, render, rows};
use revue::style::Color;
use revue::widget::data::chart::{boxplot, BoxPlot};

#[test]
fn test_boxplot_new_renders_nothing() {
    let buffer = render(&BoxPlot::new(), 40, 20);
    assert!(rows(&buffer).iter().all(|r| r.trim().is_empty()));
    let buffer = render(&boxplot(), 40, 20);
    assert!(rows(&buffer).iter().all(|r| r.trim().is_empty()));
}

#[test]
fn test_boxplot_too_small_area_renders_nothing() {
    let bp = BoxPlot::new().group("D", &[1.0, 2.0, 3.0]);
    let buffer = render(&bp, 10, 5);
    assert!(rows(&buffer).iter().all(|r| r.trim().is_empty()));
}

#[test]
fn test_boxplot_title() {
    let bp = BoxPlot::new()
        .title("Distribution")
        .group("A", &[1.0, 2.0, 3.0])
        .group("B", &[4.0, 5.0, 6.0]);
    let rows = rows(&render(&bp, 40, 20));
    // Centered on the first row: (40 - 12) / 2 = 14
    assert_eq!(
        rows[0].trim_end(),
        format!("{}Distribution", " ".repeat(14))
    );
    assert_eq!(
        rows.iter().map(|r| r.matches('┌').count()).sum::<usize>(),
        2
    );
}

#[test]
fn test_boxplot_bg() {
    let bp = BoxPlot::new().group("A", &[1.0, 2.0, 3.0]).bg(Color::BLUE);
    let buffer = render(&bp, 40, 20);
    assert_eq!(buffer.get(0, 0).unwrap().bg, Some(Color::BLUE));
    assert_eq!(buffer.get(39, 19).unwrap().bg, Some(Color::BLUE));
}

#[test]
fn test_boxplot_horizontal_differs_from_vertical() {
    let data = [1.0, 2.0, 3.0, 4.0, 5.0];
    let v = rows(&render(
        &BoxPlot::new().group("Data", &data).vertical(),
        40,
        20,
    ));
    let h = rows(&render(
        &BoxPlot::new().group("Data", &data).horizontal(),
        40,
        20,
    ));
    assert_ne!(v, h);
}

#[test]
fn test_boxplot_zero_area_draws_nothing() {
    let bp = BoxPlot::new().title("T").group("A", &[1.0, 2.0, 3.0]);
    assert_zero_areas_draw_nothing(&bp);
    assert_zero_areas_draw_nothing(&bp.horizontal());
}

#[test]
fn test_boxplot_draws_the_same_when_offset() {
    let mut data: Vec<f64> = (0..20).map(|x| x as f64).collect();
    data.push(100.0);
    let bp = BoxPlot::new()
        .title("Distribution")
        .group("A", &data)
        .group("B", &[3.0, 4.0, 5.0, 6.0, 7.0])
        .notched(true);
    assert_same_when_offset(bp.clone(), 40, 20);
    assert_same_when_offset(bp.horizontal(), 40, 20);
}
