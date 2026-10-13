//! Bar and BarOrientation tests

use revue::style::Color;
use revue::widget::data::chart::barchart::Bar;
use revue::widget::data::chart::BarOrientation;

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
