//! PieChart label tests

use super::super::render_rows;
use super::halves;
use revue::widget::data::chart::PieLabelStyle;

#[test]
fn test_pie_labels() {
    // 80x20: radius 18 around (40, 10). Labels sit at 1.3x the radius on
    // each slice's middle angle: A (right) at x=63, B (left) ending at x=16
    let text = |style| render_rows(&halves().labels(style), 80, 20);
    let percent = text(PieLabelStyle::Percent);
    let at = |x: usize| percent[10].chars().skip(x).take(3).collect::<String>();
    assert_eq!(at(63), "50%", "{percent:#?}");
    assert_eq!(at(13), "50%", "{percent:#?}");
    assert_eq!(text(PieLabelStyle::Value)[10].matches("50.0").count(), 2);
    let label = &text(PieLabelStyle::Label)[10];
    assert_eq!(label.matches('A').count(), 1);
    assert_eq!(label.matches('B').count(), 1);
    let both = &text(PieLabelStyle::LabelPercent)[10];
    assert!(both.contains("A (50%)"), "{both}");
    assert!(both.contains("B (50%)"), "{both}");
}

#[test]
fn test_pie_labels_none_draws_no_text() {
    let rows = render_rows(&halves(), 80, 20);
    assert!(rows.iter().all(|r| r.chars().all(|c| c == ' ' || c == '█')));
}
