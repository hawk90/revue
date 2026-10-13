//! Waveline sampling tests: interpolation, amplitude, color and gradient

use super::super::render;
use super::dot_rows;
use revue::style::Color;
use revue::widget::data::chart::{waveline, Interpolation};

#[test]
fn test_interpolation_linear_and_step() {
    let linear = render(&waveline(vec![-1.0, 1.0]), 5, 5);
    assert_eq!(
        dot_rows(&linear),
        vec![Some(4), Some(3), Some(2), Some(1), Some(0)]
    );
    let step = render(
        &waveline(vec![-1.0, 1.0]).interpolation(Interpolation::Step),
        5,
        5,
    );
    assert_eq!(
        dot_rows(&step),
        vec![Some(4), Some(4), Some(4), Some(4), Some(0)]
    );
}

#[test]
fn test_interpolation_curves_pass_through_points() {
    // 13 columns over 4 points: samples land exactly on x = 0, 4, 8, 12
    let data = vec![-1.0, 1.0, -1.0, 1.0];
    let linear = render(&waveline(data.clone()), 13, 9);
    for method in [Interpolation::CatmullRom, Interpolation::Bezier] {
        let curve = render(&waveline(data.clone()).interpolation(method), 13, 9);
        let dots = dot_rows(&curve);
        assert_eq!(dots[0], Some(8), "{method:?}");
        assert_eq!(dots[4], Some(0), "{method:?}");
        assert_eq!(dots[8], Some(8), "{method:?}");
        assert_eq!(dots[12], Some(0), "{method:?}");
        assert_ne!(dots, dot_rows(&linear), "{method:?}");
    }
}

#[test]
fn test_waveline_color_and_gradient() {
    let buffer = render(&waveline(vec![1.0]).color(Color::RED), 2, 5);
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::RED));

    // The gradient runs from the start color at the bottom to the end at the top
    let start = Color::rgb(0, 0, 0);
    let end = Color::rgb(200, 100, 50);
    let wave = waveline(vec![-1.0, 1.0]).gradient(start, end);
    let buffer = render(&wave, 2, 5);
    assert_eq!(buffer.get(0, 4).unwrap().fg, Some(start));
    assert_eq!(buffer.get(1, 0).unwrap().fg, Some(end));
}

#[test]
fn test_waveline_amplitude() {
    // 0.5 sits at row 1; doubled it reaches the top; values are clamped
    let base = render(&waveline(vec![0.5]), 3, 5);
    assert_eq!(dot_rows(&base), vec![Some(1); 3]);
    let doubled = render(&waveline(vec![0.5]).amplitude(2.0), 3, 5);
    assert_eq!(dot_rows(&doubled), vec![Some(0); 3]);
    let huge = render(&waveline(vec![0.5]).amplitude(100.0), 3, 5);
    assert_eq!(dot_rows(&huge), vec![Some(0); 3]);
}
