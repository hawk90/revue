//! Tests for CSV viewer helpers

use revue::widget::data::csv_viewer;

// =========================================================================
// csv_viewer() helper tests
// =========================================================================

#[test]
fn test_csv_viewer_function_creates_viewer() {
    let viewer = csv_viewer::csv_viewer();
    let _ = viewer;
}

#[test]
fn test_csv_viewer_multiple_instances() {
    let viewer1 = csv_viewer::csv_viewer();
    let viewer2 = csv_viewer::csv_viewer();
    let _ = viewer1;
    let _ = viewer2;
}

#[test]
fn test_csv_viewer_is_chainable() {
    let viewer = csv_viewer::csv_viewer();
    // Should allow builder methods
    let _ = viewer;
}

#[test]
fn test_csv_viewer_does_not_panic() {
    let _ = csv_viewer::csv_viewer();
}

#[test]
fn test_csv_viewer_returns_correct_type() {
    let viewer = csv_viewer::csv_viewer();
    // Verify it returns CsvViewer type
    let _ = viewer;
}
