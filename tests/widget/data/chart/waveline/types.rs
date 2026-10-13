//! WaveStyle and Interpolation tests

use revue::widget::data::chart::{Interpolation, WaveStyle};

#[test]
fn test_enum_defaults() {
    assert_eq!(WaveStyle::default(), WaveStyle::Line);
    assert_eq!(Interpolation::default(), Interpolation::Linear);
}
