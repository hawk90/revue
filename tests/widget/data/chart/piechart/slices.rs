//! PieChart slice tests: angles, areas, colors, donut hole, explode and shape

use super::super::{count_char, render, rows};
use super::{cells, filled, halves};
use proptest::prelude::*;
use revue::style::Color;
use revue::widget::data::chart::{pie_chart, ColorScheme, PieChart};

#[test]
fn test_pie_default_start_angle_is_top() {
    // From the top, clockwise: A takes the right half, B the left
    let buffer = render(&halves(), 40, 20);
    let a = cells(&buffer, Color::RED);
    let b = cells(&buffer, Color::BLUE);
    assert!(!a.is_empty() && !b.is_empty());
    assert!(a.iter().all(|&(x, _)| x >= 20), "{a:?}");
    assert!(b.iter().all(|&(x, _)| x <= 20), "{b:?}");
}

#[test]
fn test_pie_start_angle() {
    // From the right (0 degrees), clockwise: A takes the bottom half
    let buffer = render(&halves().start_angle(0.0), 40, 20);
    let a = cells(&buffer, Color::RED);
    let b = cells(&buffer, Color::BLUE);
    assert!(a.iter().all(|&(_, y)| y >= 10), "{a:?}");
    assert!(b.iter().all(|&(_, y)| y <= 10), "{b:?}");
}

#[test]
fn test_pie_slice_areas_follow_values() {
    let chart = PieChart::new()
        .no_legend()
        .slice_colored("A", 25.0, Color::RED)
        .slice_colored("B", 75.0, Color::BLUE);
    let buffer = render(&chart, 40, 20);
    let a = cells(&buffer, Color::RED).len() as f64;
    let b = cells(&buffer, Color::BLUE).len() as f64;
    let share = a / (a + b);
    assert!((0.2..0.3).contains(&share), "{share}");
}

#[test]
fn test_pie_palette_colors() {
    let chart = PieChart::new()
        .no_legend()
        .slice("A", 50.0)
        .slice_colored("B", 50.0, Color::RED);
    let buffer = render(&chart, 40, 20);
    let palette = ColorScheme::default_palette();
    assert!(!cells(&buffer, palette.get(0)).is_empty());
    assert!(!cells(&buffer, Color::RED).is_empty());
    assert!(cells(&buffer, palette.get(1)).is_empty());

    let custom = PieChart::new()
        .no_legend()
        .colors(ColorScheme::new(vec![Color::GREEN, Color::YELLOW]))
        .slice("A", 50.0)
        .slice("B", 50.0);
    let buffer = render(&custom, 40, 20);
    assert!(!cells(&buffer, Color::GREEN).is_empty());
    assert!(!cells(&buffer, Color::YELLOW).is_empty());
}

#[test]
fn test_pie_is_filled_donut_has_a_hole() {
    let pie = render(&halves(), 40, 20);
    assert_eq!(pie.get(20, 10).unwrap().symbol, '█');

    let donut = render(&halves().donut(0.6), 40, 20);
    assert_eq!(donut.get(20, 10).unwrap().symbol, ' ');
    assert!(count_char(&donut, '█') < count_char(&pie, '█'));
}

#[test]
fn test_pie_explode_moves_slice_out() {
    // A's middle points right (0 degrees), so exploding it moves its left
    // edge (the center line) to the right
    let min_x = |chart: &PieChart| {
        let buffer = render(chart, 40, 20);
        cells(&buffer, Color::RED)
            .iter()
            .map(|&(x, _)| x)
            .min()
            .unwrap()
    };
    let plain = min_x(&halves());
    let exploded = min_x(&halves().explode(0));
    let further = min_x(&halves().explode(0).explode_distance(0.5));
    assert_eq!(plain, 20);
    assert!(exploded > plain, "{exploded} > {plain}");
    assert!(further > exploded, "{further} > {exploded}");
    // Exploding B instead leaves A in place
    assert_eq!(min_x(&halves().explode(1)), plain);
}

#[test]
fn test_pie_is_round_on_terminal_cells() {
    // Radius 18: 18 columns either side of x=20 and 9 rows either side of
    // y=10 (cells are about twice as tall as wide)
    let buffer = render(&halves(), 40, 20);
    let mut all = cells(&buffer, Color::RED);
    all.extend(cells(&buffer, Color::BLUE));
    let ys: Vec<u16> = all.iter().map(|&(_, y)| y).collect();
    let xs: Vec<u16> = all.iter().map(|&(x, _)| x).collect();
    assert_eq!(*ys.iter().min().unwrap(), 1);
    assert_eq!(*ys.iter().max().unwrap(), 19);
    assert_eq!(*xs.iter().min().unwrap(), 2);
    assert_eq!(*xs.iter().max().unwrap(), 38);
}

#[test]
fn test_pie_empty_and_zero_total_render_nothing() {
    for chart in [PieChart::new(), PieChart::new().no_legend().slice("A", 0.0)] {
        let buffer = render(&chart, 20, 10);
        assert!(rows(&buffer).iter().all(|r| r.trim().is_empty()));
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    /// Any positive slices together cover exactly the disk of a one-slice
    /// pie of the same size: no gaps between slices and nothing outside.
    #[test]
    fn test_pie_slices_cover_the_disk(
        values in prop::collection::vec(0.01f64..100.0, 1..7),
        w in 10u16..60,
        h in 6u16..30,
        start in -360.0f64..360.0,
    ) {
        let mut pie = pie_chart().no_legend().start_angle(start);
        for (i, &v) in values.iter().enumerate() {
            pie = pie.slice(format!("s{i}"), v);
        }
        let disk = pie_chart().no_legend().start_angle(start).slice("all", 1.0);
        prop_assert_eq!(filled(&render(&pie, w, h)), filled(&render(&disk, w, h)));
    }
}
