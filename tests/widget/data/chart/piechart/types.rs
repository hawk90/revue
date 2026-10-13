//! PieStyle, PieLabelStyle and PieSlice tests

use revue::style::Color;
use revue::widget::data::chart::{PieLabelStyle, PieSlice, PieStyle};

#[test]
fn test_pie_style_and_label_defaults() {
    assert_eq!(PieStyle::default(), PieStyle::Pie);
    assert_eq!(PieLabelStyle::default(), PieLabelStyle::None);
}

#[test]
fn test_pie_slice_struct() {
    let slice = PieSlice::new("Test", 42.0);
    assert_eq!(slice.label, "Test");
    assert_eq!(slice.value, 42.0);
    assert!(slice.color.is_none());

    let slice = PieSlice::with_color("Colored", 100.0, Color::BLUE);
    assert_eq!(slice.label, "Colored");
    assert_eq!(slice.color, Some(Color::BLUE));
}
