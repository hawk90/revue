//! BarChart widget tests

use super::{count_char, render, render_rows, rows};
use revue::style::Color;
use revue::widget::data::chart::barchart::Bar;
use revue::widget::data::chart::{barchart, BarChart, BarOrientation};

fn bar(n: usize) -> String {
    "█".repeat(n)
}

// =========================================================================
// Bar
// =========================================================================

#[test]
fn test_bar_new() {
    let bar = Bar::new("Test", 10.0);
    assert_eq!(bar.label, "Test");
    assert_eq!(bar.value, 10.0);
    assert!(bar.color.is_none());
}

#[test]
fn test_bar_with_color() {
    let bar = Bar::new("Test", 42.0).color(Color::YELLOW);
    assert_eq!(bar.label, "Test");
    assert_eq!(bar.value, 42.0);
    assert_eq!(bar.color, Some(Color::YELLOW));
}

// =========================================================================
// BarOrientation
// =========================================================================

#[test]
fn test_bar_orientation_default() {
    assert_eq!(BarOrientation::default(), BarOrientation::Horizontal);
}

#[test]
fn test_bar_orientation_copy_and_eq() {
    let o1 = BarOrientation::Vertical;
    let o2 = o1;
    assert_eq!(o1, o2);
    assert_ne!(BarOrientation::Horizontal, BarOrientation::Vertical);
}

// =========================================================================
// Construction
// =========================================================================

#[test]
fn test_barchart_new_is_empty() {
    let buffer = render(&BarChart::new(), 20, 10);
    assert!(rows(&buffer).iter().all(|r| r.trim().is_empty()));
}

#[test]
fn test_barchart_helper_matches_new() {
    let a = render_rows(&barchart().bar("Test", 42.0), 30, 3);
    let b = render_rows(&BarChart::new().bar("Test", 42.0), 30, 3);
    assert_eq!(a, b);
    assert!(a[0].starts_with("Test █"));
}

#[test]
fn test_barchart_default_matches_new() {
    let a = render_rows(&BarChart::default().bar("A", 1.0), 30, 3);
    let b = render_rows(&BarChart::new().bar("A", 1.0), 30, 3);
    assert_eq!(a, b);
}

// =========================================================================
// Horizontal rendering (default)
// =========================================================================

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
fn test_barchart_bar_and_data_agree() {
    let by_bar = BarChart::new()
        .bar("Sales", 150.0)
        .bar("Revenue", 200.0)
        .bar("Profit", 75.0);
    let by_data =
        BarChart::new().data(vec![("Sales", 150.0), ("Revenue", 200.0), ("Profit", 75.0)]);
    let rows = render_rows(&by_bar, 60, 6);
    assert_eq!(rows, render_rows(&by_data, 60, 6));

    // Labels are right-aligned to the widest label, in insertion order
    assert!(rows[0].starts_with("  Sales █"));
    assert!(rows[2].starts_with("Revenue █"));
    assert!(rows[4].starts_with(" Profit █"));
}

#[test]
fn test_barchart_auto_max_is_largest_value() {
    // Without max(), the largest bar fills the whole bar area (29 cells)
    let rows = render_rows(&BarChart::new().bar("A", 10.0).bar("B", 50.0), 40, 3);
    assert!(rows[2].contains(&bar(29)));
    assert!(!rows[2].contains(&bar(30)));
    // 10 / 50 * 29 = 5.8
    assert!(rows[0].contains(&format!(" {} ", bar(5))));
    assert!(!rows[0].contains(&bar(6)));
}

#[test]
fn test_barchart_explicit_max() {
    let rows = render_rows(&BarChart::new().bar("A", 50.0).max(200.0), 40, 1);
    // 50 / 200 * 29 = 7.25
    assert_eq!(rows[0].trim_end(), format!("A {} 50.0", bar(7)));
}

#[test]
fn test_barchart_all_zero_values() {
    // Max falls back to 1.0, so zero bars draw nothing
    let chart = BarChart::new().bar("A", 0.0).bar("B", 0.0);
    let buffer = render(&chart, 40, 3);
    assert_eq!(count_char(&buffer, '█'), 0);
    assert_eq!(rows(&buffer)[0].trim_end(), "A  0.0");
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

#[test]
fn test_barchart_zero_area() {
    let chart = BarChart::new().bar("A", 50.0);
    let buffer = render(&chart, 20, 0);
    assert_eq!(buffer.height(), 0);
    let buffer = render(&chart.vertical(), 0, 5);
    assert_eq!(buffer.width(), 0);
}

// =========================================================================
// Vertical rendering
// =========================================================================

#[test]
fn test_barchart_orientation_builders() {
    let data = [("A", 1.0)];
    let v1 = render_rows(&BarChart::new().data(data).vertical(), 10, 5);
    let v2 = render_rows(
        &BarChart::new()
            .data(data)
            .orientation(BarOrientation::Vertical),
        10,
        5,
    );
    let h1 = render_rows(&BarChart::new().data(data).vertical().horizontal(), 10, 5);
    let h2 = render_rows(&BarChart::new().data(data), 10, 5);
    assert_eq!(v1, v2);
    assert_eq!(h1, h2);
    assert_ne!(v1, h1);
    // Vertical: label sits on the bottom row
    assert!(v1[4].starts_with('A'));
}

#[test]
fn test_barchart_vertical_without_values() {
    let chart = BarChart::new()
        .bar("A", 100.0)
        .vertical()
        .show_values(false)
        .bar_width(2);
    let rows = render_rows(&chart, 10, 5);
    // 4 bar rows, 1 label row
    for row in &rows[..4] {
        assert_eq!(row.trim_end(), "██");
    }
    assert_eq!(rows[4].trim_end(), "A");
}

#[test]
fn test_barchart_vertical_long_labels_truncated() {
    let chart = BarChart::new()
        .bar("VeryLongLabel", 50.0)
        .vertical()
        .bar_width(5)
        .max(100.0);
    let rows = render_rows(&chart, 10, 10);
    assert_eq!(rows[9].trim_end(), "VeryL");
}

#[test]
fn test_barchart_vertical_bars_beyond_width_are_dropped() {
    let chart = BarChart::new()
        .bar("A", 1.0)
        .bar("B", 1.0)
        .bar("C", 1.0)
        .vertical()
        .bar_width(3);
    // A at 0..3, B at 4..7, C would need 8..11
    let rows = render_rows(&chart, 10, 5);
    assert_eq!(rows[4].trim_end(), "A   B");
}
