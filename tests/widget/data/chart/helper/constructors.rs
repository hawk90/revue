//! Chart constructor tests: chart(), line_chart() and scatter_plot()

use super::super::{count_char, render, render_rows};
use super::plot_line_cells;
use revue::widget::data::chart::{chart, line_chart, scatter_plot, Chart, Marker, Series};

#[test]
fn test_chart_new_default_and_helper_agree() {
    let a = render_rows(&Chart::new(), 40, 20);
    assert_eq!(a, render_rows(&Chart::default(), 40, 20));
    assert_eq!(a, render_rows(&chart(), 40, 20));
}

#[test]
fn test_line_chart_helper() {
    let a = render_rows(&line_chart(&[2.0, 4.0]), 40, 20);
    let b = render_rows(
        &Chart::new().series(Series::new("Data").data_y(&[2.0, 4.0]).line()),
        40,
        20,
    );
    assert_eq!(a, b);
    assert!(a.join("\n").contains("■ Data"));
    assert!(a.iter().any(|r| r.contains('╱')));
}

#[test]
fn test_line_chart_helper_empty_and_single() {
    assert_eq!(
        plot_line_cells(&render(&line_chart(&[]).no_legend(), 40, 20)),
        0
    );
    assert_eq!(
        plot_line_cells(&render(&line_chart(&[10.0]).no_legend(), 40, 20)),
        0
    );
}

#[test]
fn test_scatter_plot_helper() {
    let chart = scatter_plot(&[(1.0, 2.0), (3.0, 4.0), (5.0, 6.0)]).no_legend();
    let buffer = render(&chart, 40, 20);
    assert_eq!(count_char(&buffer, Marker::Dot.char()), 3);
    assert_eq!(plot_line_cells(&buffer), 0);

    let empty = render(&scatter_plot(&[]).no_legend(), 40, 20);
    assert_eq!(count_char(&empty, Marker::Dot.char()), 0);
}
