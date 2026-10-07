//! Gauge drawing styles and label positions

/// Gauge style
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum GaugeStyle {
    /// Horizontal bar with segments
    #[default]
    Bar,
    /// Battery indicator
    Battery,
    /// Thermometer style
    Thermometer,
    /// Circular/arc (text-based)
    Arc,
    /// Percentage circle
    Circle,
    /// Vertical bar
    Vertical,
    /// Segmented blocks
    Segments,
    /// Dot indicator
    Dots,
}

/// Gauge label position
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LabelPosition {
    /// No label
    None,
    /// Inside the gauge
    #[default]
    Inside,
    /// Left of gauge
    Left,
    /// Right of gauge
    Right,
    /// Above gauge
    Above,
    /// Below gauge
    Below,
}
