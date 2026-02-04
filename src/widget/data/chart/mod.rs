//! Chart widgets - Data visualization components
//!
//! This module provides comprehensive chart and graph widgets for visualizing data.
//! Includes statistical charts, time series, sparklines, and more.
//!
//! # Chart Categories
//!
//! ## Statistical Charts
//!
//! | Widget | Description | Constructor |
//! |--------|-------------|-------------|
//! | [`BarChart`] | Bar chart with categories | [`barchart()`] |
//! | [`Histogram`] | Frequency distribution | [`histogram()`] |
//! | [`BoxPlot`] | Box-and-whisker plot | [`boxplot()`] |
//! | [`ScatterChart`] | Scatter/bubble plot | [`scatter_chart()`] |
//! | [`HeatMap`] | Heat map visualization | [`heatmap()`] |
//!
//! ## Time Series
//!
//! | Widget | Description | Constructor |
//! |--------|-------------|-------------|
//! | [`TimeSeries`] | Time series chart | [`time_series()`] |
//! | [`Sparkline`] | Inline mini chart | [`sparkline()`] |
//! | [`Waveline`] | Wave/audio waveform | [`waveline()`] |
//!
//! ## Advanced Charts
//!
//! | Widget | Description | Constructor |
//! |--------|-------------|-------------|
//! | [`Chart`] | Generic line/area chart | [`chart()`], [`line_chart()`] |
//! | [`PieChart`] | Pie/donut chart | [`pie_chart()`], [`donut_chart()`] |
//! | [`CandleChart`] | Candlestick/OHLC chart | [`candle_chart()`] |
//!
//! # Quick Start
//!
//! ## Bar Chart
//!
//! ```rust,ignore
//! use revue::prelude::*;
//!
//! let data = vec![
//!     ("Jan", 100),
//!     ("Feb", 150),
//!     ("Mar", 200),
//! ];
//!
//! barchart()
//!     .data(data)
//!     .width(40)
//!     .height(10);
//! ```
//!
//! ## Line Chart
//!
//! ```rust,ignore
//! use revue::prelude::*;
//!
//! let data = vec![
//!     (1, 10),
//!     (2, 25),
//!     (3, 15),
//!     (4, 30),
//! ];
//!
//! line_chart()
//!     .series(data)
//!     .width(50)
//!     .height(15);
//! ```
//!
//! ## Sparkline
//!
//! ```rust,ignore
//! use revue::prelude::*;
//!
//! let data = vec![10, 25, 15, 30, 20, 35, 25];
//!
//! sparkline()
//!     .data(data)
//!     .width(20)
//!     .height(3);
//! ```
//!
//! ## Heat Map
//!
//! ```rust,ignore
//! use revue::prelude::*;
//!
//! let data = vec![
//!     (0, 0, 10),
//!     (0, 1, 20),
//!     (1, 0, 30),
//!     (1, 1, 40),
//! ];
//!
//! heatmap()
//!     .data(data)
//!     .cell_width(4)
//!     .cell_height(2);
//! ```
//!
//! # Common Features
//!
//! All charts support:
//!
//! - **Custom colors** - Set color schemes and palettes
//! - **Axes** - Configure X and Y axes with labels
//! - **Grid lines** - Show/hide grid lines
//! - **Legends** - Add legends with customizable position
//! - **Labels** - Add labels to data points
//! - **Orientation** - Horizontal or vertical layouts

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
