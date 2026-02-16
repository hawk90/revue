/// Orientation for charts (bar, histogram, box plot)
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ChartOrientation {
    /// Vertical orientation (default for most charts)
    #[default]
    Vertical,
    /// Horizontal orientation
    Horizontal,
}

// KEEP HERE: All tests for ChartOrientation are public API tests
// Since enum variants and their behavior are part of the public API,
// we've extracted all tests to the separate test file.
