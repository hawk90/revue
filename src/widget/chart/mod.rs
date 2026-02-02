//! Chart widgets - Data visualization components
//!
//! Widgets for displaying data in various chart formats.

pub mod barchart;
pub mod boxplot;
pub mod candlechart;
pub mod chart_common;
pub mod chart_render;
pub mod chart_stats;
pub mod heatmap;
pub mod helper;
pub mod histogram;
pub mod piechart;
pub mod scatterchart;
pub mod sparkline;
pub mod tests;
pub mod timeseries;
pub mod types;
pub mod waveline;

// Re-exports for convenience
// Some chart_common types are re-exported for public API but not used internally
#[allow(unused_imports)]
pub use chart_common::{
    Axis, AxisFormat, ChartGrid, ChartOrientation, ColorScheme, GridStyle, Legend,
    LegendOrientation, LegendPosition, Marker,
};

pub use barchart::{barchart, BarChart, BarOrientation};
pub use boxplot::{boxplot, BoxGroup, BoxPlot, BoxStats, WhiskerStyle};
pub use candlechart::{candle_chart, ohlc_chart, Candle, CandleChart, ChartStyle as CandleStyle};
pub use heatmap::{contribution_map, heatmap, CellDisplay, ColorScale, HeatMap};
pub use helper::{chart, line_chart, scatter_plot, Chart};
pub use histogram::{histogram, BinConfig, Histogram, HistogramBin};
pub use piechart::{donut_chart, pie_chart, PieChart, PieLabelStyle, PieSlice, PieStyle};
pub use scatterchart::{bubble_chart, scatter_chart, ScatterChart, ScatterSeries};
pub use sparkline::{sparkline, Sparkline, SparklineStyle};
pub use timeseries::{
    cpu_chart, memory_chart, network_chart, time_series, time_series_with_data, MarkerStyle,
    TimeFormat, TimeLineStyle, TimeMarker, TimePoint, TimeRange, TimeSeries, TimeSeriesData,
};
pub use types::{ChartType, LineStyle, Series};
pub use waveline::{
    area_wave, audio_waveform, sawtooth_wave, signal_wave, sine_wave, spectrum, square_wave,
    waveline, Interpolation, WaveStyle, Waveline,
};
