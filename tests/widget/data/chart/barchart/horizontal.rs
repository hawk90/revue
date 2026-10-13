//! Horizontal BarChart tests: labels, bars, values, widths, gaps and colors

use super::super::{render, render_rows};
use super::bar;
use proptest::prelude::*;
use revue::style::Color;
use revue::widget::data::chart::BarChart;

#[test]
fn test_barchart_render_horizontal() {
    // label width 1, bar area = 40 - (1 + 2 + 8 value chars) = 29
    let chart = BarChart::new().bar("A", 50.0).bar("B", 100.0).max(100.0);
    let rows = render_rows(&chart, 40, 5);
    assert_eq!(rows[0].trim_end(), format!("A {} 50.0", bar(14)));
    assert!(rows[1].trim().is_empty(), "default gap is one row");
    assert_eq!(rows[2].trim_end(), format!("B {} 100.0", bar(29)));
}

#[test]
fn test_barchart_negative_values_use_magnitude() {
    let chart = BarChart::new().bar("A", -50.0).bar("B", 100.0);
    let rows = render_rows(&chart, 40, 3);
    assert_eq!(rows[0].trim_end(), format!("A {} -50.0", bar(14)));
    assert!(rows[2].contains(&bar(29)));
}

#[test]
fn test_barchart_bar_width_rows() {
    let chart = BarChart::new().bar("A", 100.0).bar("B", 100.0).bar_width(2);
    let rows = render_rows(&chart, 40, 6);
    for (y, row) in rows.iter().enumerate() {
        // rows 0-1: A, row 2: gap, rows 3-4: B
        assert_eq!(row.contains('█'), y != 2 && y != 5, "row {y}: {row:?}");
    }
    assert!(rows[0].starts_with('A'));
    assert!(rows[1].starts_with(' '));
    assert!(rows[3].starts_with('B'));
}

#[test]
fn test_barchart_bar_width_zero_clamps_to_one() {
    let zero = render_rows(
        &BarChart::new().bar("A", 1.0).bar("B", 1.0).bar_width(0),
        20,
        4,
    );
    let one = render_rows(
        &BarChart::new().bar("A", 1.0).bar("B", 1.0).bar_width(1),
        20,
        4,
    );
    assert_eq!(zero, one);
    assert!(zero[0].contains('█'));
}

#[test]
fn test_barchart_gap() {
    let chart = BarChart::new().bar("A", 1.0).bar("B", 1.0).gap(5);
    let rows = render_rows(&chart, 20, 8);
    assert!(rows[0].starts_with('A'));
    assert!(rows[6].starts_with('B'));
    assert!(rows[1..6].iter().all(|r| r.trim().is_empty()));
}

#[test]
fn test_barchart_gap_zero() {
    let chart = BarChart::new().bar("A", 1.0).bar("B", 1.0).gap(0);
    let rows = render_rows(&chart, 20, 2);
    assert!(rows[0].starts_with('A'));
    assert!(rows[1].starts_with('B'));
}

#[test]
fn test_barchart_show_values_false() {
    let shown = render_rows(&BarChart::new().bar("A", 50.0), 40, 1);
    assert!(shown[0].contains("50.0"));

    let hidden = render_rows(&BarChart::new().bar("A", 50.0).show_values(false), 40, 1);
    assert!(!hidden[0].contains('5'));
    // Without the value column the bar area grows to 40 - (1 + 2) = 37
    assert_eq!(hidden[0].trim_end(), format!("A {}", bar(37)));
}

#[test]
fn test_barchart_show_values_toggle() {
    let chart = BarChart::new()
        .bar("A", 50.0)
        .show_values(false)
        .show_values(true);
    assert!(render_rows(&chart, 40, 1)[0].contains("50.0"));
}

#[test]
fn test_barchart_fg_and_bar_colors() {
    let chart = BarChart::new()
        .fg(Color::YELLOW)
        .bar("A", 10.0)
        .bar_colored("B", 10.0, Color::RED);
    let buffer = render(&chart, 30, 3);
    assert_eq!(buffer.get(2, 0).unwrap().fg, Some(Color::YELLOW));
    assert_eq!(buffer.get(2, 2).unwrap().fg, Some(Color::RED));
}

#[test]
fn test_barchart_default_fg_is_cyan() {
    let buffer = render(&BarChart::new().bar("A", 10.0), 30, 1);
    assert_eq!(buffer.get(2, 0).unwrap().fg, Some(Color::CYAN));
}

#[test]
fn test_barchart_label_width() {
    let chart = BarChart::new()
        .bar("A", 100.0)
        .label_width(10)
        .show_values(false);
    let rows = render_rows(&chart, 40, 1);
    // Label right-aligned in 10 columns, bar starts at column 11 and fills
    // 40 - (10 + 2) = 28 cells
    assert_eq!(rows[0].trim_end(), format!("{:>10} {}", "A", bar(28)));
}

#[test]
fn test_barchart_long_labels_truncated() {
    // Label width is capped at a third of the area width (10 / 3 = 3)
    let chart = BarChart::new().bar("Very Long Label", 50.0).max(100.0);
    let rows = render_rows(&chart, 10, 1);
    assert!(rows[0].starts_with("Ver "));
    assert!(!rows[0].contains("Very"));
}

#[test]
fn test_barchart_rows_beyond_area_are_dropped() {
    let chart = BarChart::new().bar("A", 1.0).bar("B", 1.0).bar("C", 1.0);
    let rows = render_rows(&chart, 20, 3);
    assert!(rows[0].starts_with('A'));
    assert!(rows[2].starts_with('B'));
    assert!(!rows.iter().any(|r| r.contains('C')));
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    /// Horizontal bars are as long as their magnitude allows: never past
    /// the area, and a larger magnitude never draws a shorter bar.
    #[test]
    fn test_barchart_bar_lengths_follow_the_values(
        values in prop::collection::vec(-1e6f64..1e6, 1..10),
        w in 12u16..80,
    ) {
        let chart = BarChart::new()
            .data(values.iter().map(|&v| ("x", v)))
            .gap(0)
            .show_values(false);
        let rows = render_rows(&chart, w, values.len() as u16);
        let lengths: Vec<usize> = rows.iter().map(|r| r.chars().filter(|&c| c == '█').count()).collect();
        for (i, a) in values.iter().enumerate() {
            prop_assert!(lengths[i] < w as usize);
            for (j, b) in values.iter().enumerate() {
                if a.abs() > b.abs() {
                    prop_assert!(lengths[i] >= lengths[j], "{a} drew {} and {b} drew {}", lengths[i], lengths[j]);
                }
            }
        }
    }
}
