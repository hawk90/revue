//! Sample wave generator tests: sine, square and sawtooth

use revue::widget::data::chart::{sawtooth_wave, sine_wave, square_wave};

#[test]
fn test_sine_wave() {
    let data = sine_wave(4, 1.0, 1.0);
    let expected = [0.0, 1.0, 0.0, -1.0];
    assert_eq!(data.len(), 4);
    for (v, e) in data.iter().zip(expected) {
        assert!((v - e).abs() < 1e-9, "{data:?}");
    }
    // Frequency 2 completes a period every 4 of 8 samples
    let data = sine_wave(8, 2.0, 1.0);
    assert!((data[1] - 1.0).abs() < 1e-9 && (data[5] - 1.0).abs() < 1e-9);
    // Amplitude scales
    let data = sine_wave(100, 1.0, 0.5);
    assert!(data.iter().all(|&v| (-0.5..=0.5).contains(&v)));
    assert!(data.iter().any(|&v| (v - 0.5).abs() < 1e-9));
}

#[test]
fn test_square_wave() {
    assert_eq!(square_wave(4, 1.0, 1.0), vec![1.0, 1.0, -1.0, -1.0]);
    assert_eq!(square_wave(4, 2.0, 0.5), vec![0.5, -0.5, 0.5, -0.5]);
}

#[test]
fn test_sawtooth_wave() {
    assert_eq!(sawtooth_wave(4, 1.0, 1.0), vec![-1.0, -0.5, 0.0, 0.5]);
    assert_eq!(sawtooth_wave(2, 1.0, 2.0), vec![-2.0, 0.0]);
}
