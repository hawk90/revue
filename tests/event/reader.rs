//! EventReader tests

use revue::event::EventReader;
use std::time::Duration;

#[test]
fn test_event_reader_creation() {
    let reader = EventReader::new(Duration::from_millis(100));
    // Can't easily test tick_rate without accessing private fields
    // Just verify it creates without panic
    let _ = reader;
}

#[test]
fn test_event_reader_default() {
    let reader = EventReader::default();
    // Verify default creates without panic
    let _ = reader;
}
