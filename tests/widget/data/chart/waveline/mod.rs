//! Waveline widget tests
//!
//! With the default baseline (0.5) a value v is drawn at
//! y = (1 - (0.5 + v / 2)) * (height - 1): in a 5-row area 1.0 is row 0,
//! 0.0 is row 2 and -1.0 is row 4.

mod builders;
mod generators;
mod scale;
mod shapes;
mod types;
mod view;

use super::rows;
use revue::render::Buffer;

pub(super) fn sym(buffer: &Buffer, x: u16, y: u16) -> char {
    buffer.get(x, y).unwrap().symbol
}

/// Row of the '●' in each column (None if the column is empty).
pub(super) fn dot_rows(buffer: &Buffer) -> Vec<Option<u16>> {
    (0..buffer.width())
        .map(|x| (0..buffer.height()).find(|&y| sym(buffer, x, y) == '●'))
        .collect()
}

pub(super) fn blank(buffer: &Buffer) -> bool {
    rows(buffer).iter().all(|r| r.trim().is_empty())
}

mod snapshots {

    use revue::testing::{Pilot, TestApp, TestConfig};

    #[test]
    fn test_waveline_basic() {
        use revue::widget::Waveline;

        let data: Vec<f64> = (0..50)
            .map(|i| (i as f64 * 0.2).sin() * 0.4 + 0.5)
            .collect();
        let view = Waveline::new(data);

        let config = TestConfig::with_size(60, 10);
        let mut app = TestApp::with_config(view, config);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("waveline_basic");
    }

    #[test]
    fn test_waveline_filled() {
        use revue::style::Color;
        use revue::widget::{WaveStyle, Waveline};

        let data: Vec<f64> = (0..40)
            .map(|i| (i as f64 * 0.15).sin() * 0.3 + 0.5)
            .collect();
        let view = Waveline::new(data)
            .style(WaveStyle::Filled)
            .color(Color::GREEN);

        let config = TestConfig::with_size(50, 8);
        let mut app = TestApp::with_config(view, config);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("waveline_filled");
    }

    #[test]
    fn test_waveline_mirrored() {
        use revue::style::Color;
        use revue::widget::{WaveStyle, Waveline};

        let data: Vec<f64> = (0..60)
            .map(|i| (i as f64 * 0.1).sin() * 0.5 + 0.5)
            .collect();
        let view = Waveline::new(data)
            .style(WaveStyle::Mirrored)
            .color(Color::CYAN)
            .show_baseline(true);

        let config = TestConfig::with_size(70, 12);
        let mut app = TestApp::with_config(view, config);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("waveline_mirrored");
    }
}
