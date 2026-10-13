//! ScatterSeries tests

use revue::style::Color;
use revue::widget::data::chart::{Marker, ScatterSeries};

#[test]
fn test_scatter_series() {
    let series = ScatterSeries::new("Test")
        .points(&[(1.0, 2.0), (3.0, 4.0), (5.0, 6.0)])
        .color(Color::RED)
        .marker(Marker::Star);

    assert_eq!(series.name, "Test");
    assert_eq!(series.data.len(), 3);
    assert_eq!(series.color, Some(Color::RED));
    assert_eq!(series.marker, Marker::Star);
}
