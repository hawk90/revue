//! Log viewer unit tests not covered by the integration tests
//!
//! Everything else lives in tests/widget/log_viewer_tests.rs and tests/log_viewer/.

use super::*;

#[test]
fn test_timestamp_format_default() {
    assert_eq!(TimestampFormat::default(), TimestampFormat::Iso8601);
}

// Timestamp sniffing looks at the first 8 and 19 bytes of a line; a line whose
// multibyte character straddles either cut must not panic.
#[test]
fn test_parse_does_not_slice_inside_a_multibyte_char() {
    let parser = LogParser::new();
    for line in [
        "한글 로그 메시지",            // byte 8 inside '로', byte 19 inside '지'
        "1234567한 message",           // byte 8 inside '한'
        "2024-01-01 12:00:한 message", // byte 19 inside '한'
        "👍👍👍",
    ] {
        let entry = parser.parse(line, 1);
        assert_eq!(entry.timestamp, None, "{line:?}");
    }
}

#[test]
fn test_parse_still_detects_ascii_timestamps() {
    let parser = LogParser::new();
    let entry = parser.parse("12:34:56 INFO 한글", 1);
    assert_eq!(entry.timestamp.as_deref(), Some("12:34:56"));
    let entry = parser.parse("2024-01-01 12:34:56 INFO 한글", 1);
    assert_eq!(entry.timestamp.as_deref(), Some("2024-01-01 12:34:56"));
}

#[test]
fn test_load_hangul_lines() {
    let mut viewer = LogViewer::new();
    viewer.load("가나다라마바사\n안녕하세요 세계");
    assert_eq!(viewer.len(), 2);
}
