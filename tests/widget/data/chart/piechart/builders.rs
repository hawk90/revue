//! PieChart builder tests: slices, donut constructors and ratio

use super::super::render_rows;
use super::halves;
use revue::widget::data::chart::{donut_chart, pie_chart, PieChart, PieStyle};

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
