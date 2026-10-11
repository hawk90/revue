//! Paint-level checks for the last of the data widgets and charts.
//!
//! Each paints something a single rule cannot name: a selected grid row, a CPU
//! threshold turning red, a series palette, a waveform whose color is computed
//! per sample. So `color` reaches what is left - the ordinary cell, the process
//! row, the axis label, the chart's own title.

use revue::prelude::*;
use revue::style::Color;
use revue::testing::PipelineHarness;
use revue::widget::{DataGrid, GridColumn, TimeSeries, TimeSeriesData, Waveline};

/// A color none of these widgets paints on its own.
///
/// Picking a sentinel out of the widget's own palette is how a paint test goes
/// vacuous - `Color::RED` passed against a reverted `ColorPicker` in #643.
const INK: Color = Color {
    r: 17,
    g: 34,
    b: 51,
    a: 255,
};

fn draw<V: View>(css: &str, view: &V) -> PipelineHarness {
    let mut h = PipelineHarness::with_css(css, 60, 14).dom_from_render(true);
    h.draw(view);
    h
}

fn any_fg(h: &PipelineHarness, want: Color) -> bool {
    let buffer = h.buffer();
    (0..buffer.height())
        .any(|y| (0..buffer.width()).any(|x| buffer.get(x, y).and_then(|c| c.fg) == Some(want)))
}

macro_rules! wrap {
    ($name:ident, $build:expr) => {
        struct $name;
        impl View for $name {
            fn render(&self, ctx: &mut RenderContext) {
                vstack().child($build).render(ctx);
            }
            fn widget_type(&self) -> &'static str {
                stringify!($name)
            }
            fn id(&self) -> Option<&str> {
                Some("root")
            }
        }
    };
}

wrap!(
    DataGridView,
    DataGrid::new()
        .column(GridColumn::new("name", "Name"))
        .column(GridColumn::new("qty", "Qty"))
        .data(vec![
            vec!["widget".into(), "12".into()],
            vec!["gadget".into(), "7".into()],
        ])
        .element_id("w")
);

wrap!(
    TimeSeriesView,
    TimeSeries::new()
        .title("latency")
        .series(TimeSeriesData::new("p50").points(vec![(0, 1.0), (1, 4.0), (2, 2.0), (3, 5.0),]))
        .element_id("w")
);

wrap!(
    WavelineView,
    Waveline::new(vec![0.1, 0.7, -0.3, 0.9, -0.6])
        .label("signal")
        .element_id("w")
);

#[test]
fn color_reaches_a_data_grids_cell() {
    let h = draw("#w { color: #112233; }", &DataGridView);
    assert!(any_fg(&h, INK), "`color` did not reach a DataGrid cell");
}

#[test]
fn color_reaches_a_time_series_chrome() {
    let h = draw("#w { color: #112233; }", &TimeSeriesView);
    assert!(any_fg(&h, INK), "`color` did not reach TimeSeries' labels");
}

/// The series color carries the data. Flattening every series to one `color`
/// leaves a chart that shows nothing, so it stays put.
#[test]
fn a_time_series_keeps_its_series_color() {
    let h = draw("#w { color: #112233; }", &TimeSeriesView);
    assert!(
        any_fg(&h, Color::CYAN),
        "`color` flattened the series it should not address"
    );
}

#[test]
fn color_reaches_a_wavelines_label() {
    let h = draw("#w { color: #112233; }", &WavelineView);
    assert!(any_fg(&h, INK), "`color` did not reach Waveline's label");
}

/// `ProcessMonitor` reads the real system, so its fixture has to refresh before
/// there are any rows to paint. An empty list would make the assertion vacuous,
/// so that is checked first and fails as a fixture problem, not a wiring one.
#[cfg(feature = "sysinfo")]
#[test]
fn color_reaches_a_process_row() {
    use revue::widget::ProcessMonitor;

    struct V(ProcessMonitor);
    impl View for V {
        fn render(&self, ctx: &mut RenderContext) {
            let area = ctx.area;
            ctx.render_child(&self.0, area);
        }
        fn widget_type(&self) -> &'static str {
            "V"
        }
        fn id(&self) -> Option<&str> {
            Some("root")
        }
    }

    let mut monitor = ProcessMonitor::new().element_id("w");
    monitor.refresh();
    assert!(
        monitor.process_count() > 0,
        "fixture problem: the system reported no processes"
    );

    let view = V(monitor);
    let h = draw("#w { color: #112233; }", &view);
    assert!(any_fg(&h, INK), "`color` did not reach a process row");
}
