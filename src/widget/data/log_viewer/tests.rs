//! Log viewer unit tests not covered by the integration tests
//!
//! Everything else lives in tests/widget/log_viewer_tests.rs and tests/log_viewer/.

use super::*;

#[test]
fn test_timestamp_format_default() {
    assert_eq!(TimestampFormat::default(), TimestampFormat::Iso8601);
}
