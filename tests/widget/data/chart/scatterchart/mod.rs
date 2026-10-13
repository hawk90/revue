//! ScatterChart tests (src/widget/data/chart/scatterchart), one file per
//! src file. They check what the chart draws.

mod marks;
mod scale;
mod series;
mod view;

use revue::widget::data::chart::{ChartGrid, ScatterChart};

/// A chart with no grid and no legend, so only points and labels are drawn.
pub(super) fn bare() -> ScatterChart {
    ScatterChart::new().grid(ChartGrid::new()).no_legend()
}
