//! Histogram builder tests: bin configuration, data replacement and the constructor function

use super::super::render_rows;
use super::{bar_count, three_bins};
use revue::widget::data::chart::{histogram, BinConfig, Histogram};

#[test]
fn test_histogram_bin_configs() {
    let data: Vec<f64> = (0..100).map(|x| x as f64).collect();
    // Sturges' rule: 8 bins for 100 values
    assert_eq!(bar_count(Histogram::new(&data)), 8);
    assert_eq!(bar_count(Histogram::new(&data).bin_count(10)), 10);
    assert_eq!(bar_count(Histogram::new(&data).bin_width(10.0)), 10);
    assert_eq!(
        bar_count(Histogram::new(&data).bins(BinConfig::Count(15))),
        15
    );
}

#[test]
fn test_histogram_data_recomputes_bins() {
    let replaced = Histogram::new(&[100.0, 200.0])
        .bins(BinConfig::Edges(vec![0.0, 10.0, 20.0, 30.0]))
        .data(&[1.0, 2.0, 3.0, 11.0, 21.0, 22.0]);
    assert_eq!(
        render_rows(&replaced, 40, 20),
        render_rows(&three_bins(), 40, 20)
    );
}

#[test]
fn test_histogram_helper_matches_new() {
    let data = [1.0, 2.0, 3.0, 5.0, 8.0];
    assert_eq!(
        render_rows(&histogram(&data), 40, 20),
        render_rows(&Histogram::new(&data), 40, 20)
    );
}
