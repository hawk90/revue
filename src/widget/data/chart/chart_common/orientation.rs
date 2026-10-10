/// Orientation for charts (bar, histogram, box plot)
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ChartOrientation {
    /// Vertical orientation (default for most charts)
    #[default]
    Vertical,
    /// Horizontal orientation
    Horizontal,
}
