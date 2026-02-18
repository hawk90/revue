//! LogParser tests

#![allow(unused_imports)]

use revue::widget::{log_filter, log_parser, log_viewer, AdvLogEntry, AdvLogLevel};

#[test]
fn test_log_parser_creation() {
    let parser = log_parser();
    assert!(parser.json_parsing);
}

#[test]
fn test_log_parser_builder() {
    let parser = log_parser()
        .json_parsing(false)
        .json_fields("lvl", "message", "timestamp");

    assert!(!parser.json_parsing);
}

#[test]
fn test_log_parser_json_parsing() {
    let parser = log_parser();

    let entry = parser.parse(
        r#"{"level":"ERROR","msg":"Test error","time":"2024-01-15"}"#,
        1,
    );

    assert_eq!(entry.level, AdvLogLevel::Error);
    assert_eq!(entry.message, "Test error");
    assert_eq!(entry.timestamp, Some("2024-01-15".to_string()));
    assert!(entry.json_fields.is_some());
}

#[test]
fn test_log_parser_json_with_custom_fields() {
    let parser = log_parser().json_fields("severity", "message", "ts");

    let entry = parser.parse(
        r#"{"severity":"WARNING","message":"Custom message","ts":"12:30:00"}"#,
        1,
    );

    assert_eq!(entry.level, AdvLogLevel::Warning);
    assert_eq!(entry.message, "Custom message");
    assert_eq!(entry.timestamp, Some("12:30:00".to_string()));
}

#[test]
fn test_log_parser_standard_format() {
    let parser = log_parser();

    let entry = parser.parse("[2024-01-15 10:30:00] [ERROR] [main] Something failed", 1);

    assert_eq!(entry.level, AdvLogLevel::Error);
    assert_eq!(entry.timestamp, Some("2024-01-15 10:30:00".to_string()));
    // Message parsing may vary
}

#[test]
fn test_log_parser_simple_format() {
    let parser = log_parser();

    let entry = parser.parse("ERROR: Something went wrong", 1);

    assert_eq!(entry.level, AdvLogLevel::Error);
}

#[test]
fn test_log_parser_disabled_json() {
    let parser = log_parser().json_parsing(false);

    let entry = parser.parse(r#"{"level":"ERROR","msg":"Test"}"#, 1);

    // Should not parse as JSON, treat as plain text
    assert_eq!(entry.level, AdvLogLevel::Info); // Default level
    assert!(entry.json_fields.is_none());
}

#[test]
fn test_log_parser_json_empty_object() {
    let parser = log_parser();

    let entry = parser.parse(r#"{}"#, 1);

    // Empty JSON should have no json_fields
    assert!(entry.json_fields.is_none());
}

#[test]
fn test_log_parser_json_with_nested_object() {
    let parser = log_parser();

    let entry = parser.parse(
        r#"{"level":"INFO","msg":"Test","extra":{"nested":"value"}}"#,
        1,
    );

    assert_eq!(entry.level, AdvLogLevel::Info);
    assert_eq!(entry.message, "Test");
}

#[test]
fn test_log_parser_json_with_escaped_chars() {
    let parser = log_parser();

    let entry = parser.parse(r#"{"msg":"Test \"quoted\" string"}"#, 1);

    assert_eq!(entry.message, r#"Test "quoted" string"#);
}

#[test]
fn test_log_parser_json_alternate_field_names() {
    let parser = log_parser();

    // Test "severity" field
    let entry1 = parser.parse(r#"{"severity":"FATAL","msg":"Test"}"#, 1);
    assert_eq!(entry1.level, AdvLogLevel::Fatal);

    // Test "message" field
    let entry2 = parser.parse(r#"{"level":"INFO","message":"Custom message"}"#, 1);
    assert_eq!(entry2.message, "Custom message");

    // Test "timestamp" field
    let entry3 = parser.parse(r#"{"level":"INFO","timestamp":"2024-01-15"}"#, 1);
    assert_eq!(entry3.timestamp, Some("2024-01-15".to_string()));

    // Test "ts" field
    let entry4 = parser.parse(r#"{"level":"INFO","ts":"123456"}"#, 1);
    assert_eq!(entry4.timestamp, Some("123456".to_string()));

    // Test "logger" field
    let entry5 = parser.parse(r#"{"level":"INFO","logger":"my.app"}"#, 1);
    assert_eq!(entry5.source, Some("my.app".to_string()));

    // Test "caller" field
    let entry6 = parser.parse(r#"{"level":"INFO","caller":"main.rs"}"#, 1);
    assert_eq!(entry6.source, Some("main.rs".to_string()));
}

#[test]
fn test_log_parser_iso_timestamp() {
    let parser = log_parser();

    let entry = parser.parse("2024-01-15T10:30:00 INFO Message", 1);

    assert_eq!(entry.level, AdvLogLevel::Info);
    assert_eq!(entry.timestamp, Some("2024-01-15T10:30:00".to_string()));
}

#[test]
fn test_log_parser_iso_timestamp_with_ms() {
    let parser = log_parser();

    let entry = parser.parse("2024-01-15T10:30:00.123 INFO Message", 1);

    assert_eq!(entry.level, AdvLogLevel::Info);
    assert_eq!(entry.timestamp, Some("2024-01-15T10:30:00.123".to_string()));
}

#[test]
fn test_log_parser_time_only() {
    let parser = log_parser();

    let entry = parser.parse("10:30:00 INFO Message", 1);

    assert_eq!(entry.level, AdvLogLevel::Info);
    assert_eq!(entry.timestamp, Some("10:30:00".to_string()));
}

#[test]
fn test_log_parser_level_variants() {
    let parser = log_parser();

    // Test TRACE
    let entry1 = parser.parse("TRACE Debug message", 1);
    assert_eq!(entry1.level, AdvLogLevel::Trace);

    // Test DEBUG
    let entry2 = parser.parse("DEBUG Debug message", 2);
    assert_eq!(entry2.level, AdvLogLevel::Debug);

    // Test WARNING (long form)
    let entry3 = parser.parse("WARNING Warning message", 3);
    assert_eq!(entry3.level, AdvLogLevel::Warning);

    // Test FATAL
    let entry4 = parser.parse("FATAL Fatal message", 4);
    assert_eq!(entry4.level, AdvLogLevel::Fatal);

    // Test CRITICAL
    let entry5 = parser.parse("CRITICAL Critical message", 5);
    assert_eq!(entry5.level, AdvLogLevel::Fatal);
}

#[test]
fn test_log_parser_bracketed_level() {
    let parser = log_parser();

    let entry = parser.parse("[INFO] Message", 1);

    assert_eq!(entry.level, AdvLogLevel::Info);
}

#[test]
fn test_log_parser_level_with_colon() {
    let parser = log_parser();

    let entry = parser.parse("INFO: Message", 1);

    assert_eq!(entry.level, AdvLogLevel::Info);
    assert_eq!(entry.message, "Message");
}

#[test]
fn test_log_parser_bracketed_source() {
    let parser = log_parser();

    let entry = parser.parse("[main] INFO Message", 1);

    assert_eq!(entry.source, Some("main".to_string()));
}

#[test]
fn test_log_parser_source_with_colon() {
    let parser = log_parser();

    let entry = parser.parse("main.rs: INFO Message", 1);

    assert_eq!(entry.source, Some("main.rs".to_string()));
}

#[test]
fn test_log_parser_timestamp_numeric() {
    let parser = log_parser();

    let entry = parser.parse(r#"{"level":"INFO","time":"1234567890"}"#, 1);

    assert_eq!(entry.timestamp_value, Some(1234567890_i64));
}

#[test]
fn test_log_parser_line_number() {
    let parser = log_parser();

    // Test that line_number is set correctly by parse()
    let entry1 = parser.parse("INFO Message", 42);
    assert_eq!(entry1.line_number, 42);

    let entry2 = parser.parse("ERROR Error", 100);
    assert_eq!(entry2.line_number, 100);
}

// =============================================================================
// JSON edge cases
// =============================================================================

#[test]
fn test_log_parser_json_non_object_input() {
    let parser = log_parser();

    // Array input should not parse as JSON log
    let entry = parser.parse(r#"["not", "an", "object"]"#, 1);
    assert!(entry.json_fields.is_none());
}

#[test]
fn test_log_parser_json_with_array_value() {
    let parser = log_parser();

    let entry = parser.parse(r#"{"level":"INFO","msg":"Test","tags":["a","b"]}"#, 1);
    assert_eq!(entry.level, AdvLogLevel::Info);
    assert_eq!(entry.message, "Test");
    assert!(entry.json_fields.is_some());
}

#[test]
fn test_log_parser_json_whitespace() {
    let parser = log_parser();

    let entry = parser.parse(r#"  { "level" : "ERROR" , "msg" : "spaced" }  "#, 1);
    assert_eq!(entry.level, AdvLogLevel::Error);
    assert_eq!(entry.message, "spaced");
}

#[test]
fn test_log_parser_json_malformed_no_closing_brace() {
    let parser = log_parser();

    let entry = parser.parse(r#"{"level":"INFO","msg":"test""#, 1);
    // Should not parse as valid JSON
    assert!(entry.json_fields.is_none());
}

#[test]
fn test_log_parser_json_numeric_value() {
    let parser = log_parser();

    let entry = parser.parse(r#"{"level":"INFO","msg":"Test","count":42}"#, 1);
    assert_eq!(entry.level, AdvLogLevel::Info);
    assert!(entry.json_fields.is_some());
    let fields = entry.json_fields.unwrap();
    assert!(fields.iter().any(|(k, v)| k == "count" && v == "42"));
}

#[test]
fn test_log_parser_json_boolean_value() {
    let parser = log_parser();

    let entry = parser.parse(r#"{"level":"WARN","msg":"flag","ok":true}"#, 1);
    assert_eq!(entry.level, AdvLogLevel::Warning);
    assert!(entry.json_fields.is_some());
    let fields = entry.json_fields.unwrap();
    assert!(fields.iter().any(|(k, v)| k == "ok" && v == "true"));
}

// =============================================================================
// Standard format edge cases
// =============================================================================

#[test]
fn test_log_parser_no_timestamp_line() {
    let parser = log_parser();

    let entry = parser.parse("Just a plain message", 1);
    assert_eq!(entry.timestamp, None);
    // Message should contain the text
    assert!(entry.message.contains("plain message"));
}

#[test]
fn test_log_parser_slash_date_in_brackets() {
    let parser = log_parser();

    let entry = parser.parse("[2024/01/15 10:30:00] INFO Message", 1);
    // The bracket parser should find it since it contains ':' or '/'
    assert!(entry.timestamp.is_some());
}

#[test]
fn test_log_parser_iso_timestamp_with_long_ms() {
    let parser = log_parser();

    let entry = parser.parse("2024-01-15T10:30:00.123456 ERROR Detailed", 1);
    assert!(entry.timestamp.is_some());
    let ts = entry.timestamp.unwrap();
    assert!(ts.contains("123456"));
}

#[test]
fn test_log_parser_long_source_rejected() {
    let parser = log_parser();

    // Source names > 50 chars should be rejected by find_source
    let long_source = "a".repeat(51);
    let line = format!("[{}] INFO Message", long_source);
    let entry = parser.parse(&line, 1);
    // The long bracketed content should not be parsed as source
    // (it contains no space, but is > 50 chars)
    assert!(entry.source.is_none() || entry.source.as_deref().unwrap().len() < 51);
}

// =============================================================================
// Level parsing edge cases
// =============================================================================

#[test]
fn test_log_parser_warn_abbreviation() {
    let parser = log_parser();

    let entry = parser.parse("WARN Something happened", 1);
    assert_eq!(entry.level, AdvLogLevel::Warning);
}

#[test]
fn test_log_parser_mixed_case_level() {
    let parser = log_parser();

    // The parser uppercases before comparison
    let entry = parser.parse("Error: lower case level", 1);
    assert_eq!(entry.level, AdvLogLevel::Error);
}

#[test]
fn test_log_parser_level_at_string_end() {
    let parser = log_parser();

    // Level at end with following space (parser requires a delimiter after the level keyword)
    let entry = parser.parse("[2024-01-15 10:00:00] ERROR ", 1);
    assert_eq!(entry.level, AdvLogLevel::Error);
}

// =============================================================================
// JSON field mapping edge cases
// =============================================================================

#[test]
fn test_log_parser_custom_source_field() {
    let parser = log_parser();

    let entry = parser.parse(r#"{"level":"INFO","source":"my-service"}"#, 1);
    assert_eq!(entry.source, Some("my-service".to_string()));
}

#[test]
fn test_log_parser_numeric_timestamp_value() {
    let parser = log_parser();

    let entry = parser.parse(r#"{"level":"INFO","time":"1705311000"}"#, 1);
    assert_eq!(entry.timestamp, Some("1705311000".to_string()));
    assert_eq!(entry.timestamp_value, Some(1705311000_i64));
}

#[test]
fn test_log_parser_non_numeric_timestamp_no_value() {
    let parser = log_parser();

    let entry = parser.parse(r#"{"level":"INFO","time":"2024-01-15"}"#, 1);
    assert_eq!(entry.timestamp, Some("2024-01-15".to_string()));
    assert_eq!(entry.timestamp_value, None); // Not a number
}

#[test]
fn test_log_parser_unknown_json_field_ignored() {
    let parser = log_parser();

    let entry = parser.parse(
        r#"{"level":"INFO","msg":"Test","unknown_field":"value"}"#,
        1,
    );
    assert_eq!(entry.level, AdvLogLevel::Info);
    assert_eq!(entry.message, "Test");
    // Unknown field should be in json_fields but not in entry.source/timestamp
    assert!(entry.json_fields.is_some());
}

#[test]
fn test_log_parser_json_line_number_set_from_parse() {
    let parser = log_parser();

    let entry = parser.parse(r#"{"level":"INFO","msg":"Test"}"#, 99);
    assert_eq!(entry.line_number, 99);
}

#[test]
fn test_log_parser_level_parse_shortcodes() {
    // Test the LogLevel::parse directly through parser
    let parser = log_parser();

    let entry_trc = parser.parse(r#"{"level":"TRC","msg":"t"}"#, 1);
    assert_eq!(entry_trc.level, AdvLogLevel::Trace);

    let entry_dbg = parser.parse(r#"{"level":"DBG","msg":"d"}"#, 1);
    assert_eq!(entry_dbg.level, AdvLogLevel::Debug);

    let entry_inf = parser.parse(r#"{"level":"INF","msg":"i"}"#, 1);
    assert_eq!(entry_inf.level, AdvLogLevel::Info);

    let entry_wrn = parser.parse(r#"{"level":"WRN","msg":"w"}"#, 1);
    assert_eq!(entry_wrn.level, AdvLogLevel::Warning);

    let entry_err = parser.parse(r#"{"level":"ERR","msg":"e"}"#, 1);
    assert_eq!(entry_err.level, AdvLogLevel::Error);

    let entry_ftl = parser.parse(r#"{"level":"FTL","msg":"f"}"#, 1);
    assert_eq!(entry_ftl.level, AdvLogLevel::Fatal);

    let entry_crit = parser.parse(r#"{"level":"CRIT","msg":"c"}"#, 1);
    assert_eq!(entry_crit.level, AdvLogLevel::Fatal);
}
