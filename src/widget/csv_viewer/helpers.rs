//! Helper functions for creating CSV viewers

use super::core::CsvViewer;

/// Helper function to create a CSV viewer
pub fn csv_viewer() -> CsvViewer {
    CsvViewer::new()
}
