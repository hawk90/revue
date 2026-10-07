//! Sample data generators for waveline demos: sine, square and sawtooth waves

/// Generate sine wave data
pub fn sine_wave(samples: usize, frequency: f64, amplitude: f64) -> Vec<f64> {
    (0..samples)
        .map(|i| {
            let t = i as f64 / samples as f64 * std::f64::consts::PI * 2.0 * frequency;
            t.sin() * amplitude
        })
        .collect()
}

/// Generate square wave data
pub fn square_wave(samples: usize, frequency: f64, amplitude: f64) -> Vec<f64> {
    (0..samples)
        .map(|i| {
            let t = i as f64 / samples as f64 * frequency;
            if t.fract() < 0.5 {
                amplitude
            } else {
                -amplitude
            }
        })
        .collect()
}

/// Generate sawtooth wave data
pub fn sawtooth_wave(samples: usize, frequency: f64, amplitude: f64) -> Vec<f64> {
    (0..samples)
        .map(|i| {
            let t = i as f64 / samples as f64 * frequency;
            (t.fract() * 2.0 - 1.0) * amplitude
        })
        .collect()
}
