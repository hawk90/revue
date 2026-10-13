//! PieChart frame tests: legend, title, background, tiny, zero and offset areas

use super::super::{
    assert_same_when_offset, assert_zero_areas_draw_nothing, render, render_rows, rows,
};
use super::halves;
use revue::style::Color;
use revue::widget::data::chart::{pie_chart, Legend, PieChart, PieLabelStyle};

#[test]
fn test_pie_legend() {
    let chart = || {
        PieChart::new()
            .slice("Alpha", 50.0)
            .slice("Beta", 30.0)
            .slice("Gamma", 20.0)
    };
    // Default: top right
    let rows = render_rows(&chart(), 40, 20);
    assert!(rows[2].ends_with("│■ Alpha│ "), "{:?}", rows[2]);
    assert!(rows[3].ends_with("│■ Beta │ "), "{:?}", rows[3]);
    assert!(rows[4].ends_with("│■ Gamma│ "), "{:?}", rows[4]);

    let bottom = render_rows(&chart().legend(Legend::bottom_center()), 40, 20);
    assert!(bottom[14..].join("\n").contains("■ Alpha"));

    let none = render_rows(&chart().no_legend(), 40, 20).join("\n");
    assert!(!none.contains('■'));
}

#[test]
fn test_pie_title() {
    let rows = render_rows(&PieChart::new().title("Test Chart").slice("A", 1.0), 30, 15);
    // Centered: (30 - 10) / 2 = 10
    assert_eq!(rows[0].trim_end(), format!("{}Test Chart", " ".repeat(10)));
}

#[test]
fn test_pie_bg() {
    let buffer = render(&halves().bg(Color::rgb(1, 2, 3)), 40, 20);
    assert_eq!(buffer.get(0, 0).unwrap().bg, Some(Color::rgb(1, 2, 3)));
}

#[test]
fn test_pie_tiny_area_renders_nothing() {
    let buffer = render(&PieChart::new().slice("A", 100.0), 2, 2);
    assert!(rows(&buffer).iter().all(|r| r.trim().is_empty()));
}

#[test]
fn test_pie_zero_area_draws_nothing() {
    let pie = halves().title("T").labels(PieLabelStyle::Label);
    assert_zero_areas_draw_nothing(&pie);
}

#[test]
fn test_pie_draws_the_same_when_offset() {
    let pie = pie_chart()
        .title("Share")
        .slice("a", 3.0)
        .slice("b", 2.0)
        .slice("c", 1.0)
        .explode(1)
        .labels(PieLabelStyle::Percent);
    assert_same_when_offset(pie.clone(), 40, 20);
    assert_same_when_offset(pie.donut(0.5), 40, 20);
}
