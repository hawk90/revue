//! PieChart widget tests
//!
//! Geometry used below: a 40x20 chart without title is centered on
//! (20, 10). Slices are drawn clockwise from `start_angle` (-90 = top).

use super::{count_char, render, render_rows, rows};
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::data::chart::{
    donut_chart, pie_chart, ColorScheme, Legend, PieChart, PieLabelStyle, PieSlice, PieStyle,
};

/// Positions of '█' cells drawn in `color`.
fn cells(buffer: &Buffer, color: Color) -> Vec<(u16, u16)> {
    let mut out = Vec::new();
    for y in 0..buffer.height() {
        for x in 0..buffer.width() {
            let cell = buffer.get(x, y).unwrap();
            if cell.symbol == '█' && cell.fg == Some(color) {
                out.push((x, y));
            }
        }
    }
    out
}

fn halves() -> PieChart {
    PieChart::new()
        .no_legend()
        .slice_colored("A", 50.0, Color::RED)
        .slice_colored("B", 50.0, Color::BLUE)
}

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
fn test_pie_slices_iterator_matches_slice() {
    let a = render_rows(&PieChart::new().slice("A", 30.0).slice("B", 70.0), 40, 20);
    let b = render_rows(
        &PieChart::new().slices(vec![("A", 30.0), ("B", 70.0)]),
        40,
        20,
    );
    assert_eq!(a, b);
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
fn test_donut_constructors_agree() {
    let slices = |c: PieChart| c.slice("A", 1.0).slice("B", 2.0);
    let a = render_rows(&slices(donut_chart()), 40, 20);
    assert_eq!(a, render_rows(&slices(PieChart::new().donut(0.5)), 40, 20));
    // style(Donut) picks the same default ratio
    assert_eq!(
        a,
        render_rows(&slices(PieChart::new().style(PieStyle::Donut)), 40, 20)
    );
    assert_ne!(a, render_rows(&slices(pie_chart()), 40, 20));
    assert_eq!(
        render_rows(&slices(pie_chart()), 40, 20),
        render_rows(&slices(PieChart::new()), 40, 20)
    );
}

#[test]
fn test_donut_ratio_is_clamped() {
    let max = render_rows(&halves().donut(0.9), 40, 20);
    assert_eq!(max, render_rows(&halves().donut(5.0), 40, 20));
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
fn test_pie_labels() {
    // 80x20: radius 18 around (40, 10). Labels sit at 1.3x the radius on
    // each slice's middle angle: A (right) at x=63, B (left) ending at x=16
    let text = |style| render_rows(&halves().labels(style), 80, 20);
    let percent = text(PieLabelStyle::Percent);
    let at = |x: usize| percent[10].chars().skip(x).take(3).collect::<String>();
    assert_eq!(at(63), "50%", "{percent:#?}");
    assert_eq!(at(13), "50%", "{percent:#?}");
    assert_eq!(text(PieLabelStyle::Value)[10].matches("50.0").count(), 2);
    let label = &text(PieLabelStyle::Label)[10];
    assert_eq!(label.matches('A').count(), 1);
    assert_eq!(label.matches('B').count(), 1);
    let both = &text(PieLabelStyle::LabelPercent)[10];
    assert!(both.contains("A (50%)"), "{both}");
    assert!(both.contains("B (50%)"), "{both}");
}

#[test]
fn test_pie_labels_none_draws_no_text() {
    let rows = render_rows(&halves(), 80, 20);
    assert!(rows.iter().all(|r| r.chars().all(|c| c == ' ' || c == '█')));
}

#[test]
fn test_pie_legend() {
    let chart = || {
        PieChart::new()
            .slice("Alpha", 50.0)
            .slice("Beta", 30.0)
            .slice("Gamma", 20.0)
    };
    // Default: top right
    let rows = render_rows(&chart(), 40, 20);
    assert!(rows[2].ends_with("│■ Alpha│ "), "{:?}", rows[2]);
    assert!(rows[3].ends_with("│■ Beta │ "), "{:?}", rows[3]);
    assert!(rows[4].ends_with("│■ Gamma│ "), "{:?}", rows[4]);

    let bottom = render_rows(&chart().legend(Legend::bottom_center()), 40, 20);
    assert!(bottom[14..].join("\n").contains("■ Alpha"));

    let none = render_rows(&chart().no_legend(), 40, 20).join("\n");
    assert!(!none.contains('■'));
}

#[test]
fn test_pie_title() {
    let rows = render_rows(&PieChart::new().title("Test Chart").slice("A", 1.0), 30, 15);
    // Centered: (30 - 10) / 2 = 10
    assert_eq!(rows[0].trim_end(), format!("{}Test Chart", " ".repeat(10)));
}

#[test]
fn test_pie_bg() {
    let buffer = render(&halves().bg(Color::rgb(1, 2, 3)), 40, 20);
    assert_eq!(buffer.get(0, 0).unwrap().bg, Some(Color::rgb(1, 2, 3)));
}

#[test]
fn test_pie_empty_and_zero_total_render_nothing() {
    for chart in [PieChart::new(), PieChart::new().no_legend().slice("A", 0.0)] {
        let buffer = render(&chart, 20, 10);
        assert!(rows(&buffer).iter().all(|r| r.trim().is_empty()));
    }
}

#[test]
fn test_pie_tiny_area_renders_nothing() {
    let buffer = render(&PieChart::new().slice("A", 100.0), 2, 2);
    assert!(rows(&buffer).iter().all(|r| r.trim().is_empty()));
}
