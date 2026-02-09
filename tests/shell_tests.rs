//! Integration tests for shell utilities
//!
//! Extracted from src/utils/shell.rs

use revue::utils::shell::{escape_applescript, escape_powershell, sanitize_string};

#[test]
fn test_escape_applescript_quotes() {
    assert_eq!(escape_applescript("Hello \"World\""), r#"Hello \"World\""#);
}

#[test]
fn test_escape_applescript_backslash() {
    assert_eq!(escape_applescript("foo\\bar"), r#"foo\\bar"#);
}

#[test]
fn test_escape_applescript_newline() {
    assert_eq!(escape_applescript("line1\nline2"), r#"line1\nline2"#);
}

#[test]
fn test_escape_applescript_carriage_return() {
    assert_eq!(escape_applescript("line1\rline2"), r#"line1\rline2"#);
}

#[test]
fn test_escape_applescript_tab() {
    assert_eq!(escape_applescript("col1\tcol2"), r#"col1\tcol2"#);
}

#[test]
fn test_escape_applescript_combined() {
    assert_eq!(escape_applescript("foo\"bar\\baz\n"), r#"foo\"bar\\baz\n"#);
}

#[test]
fn test_escape_applescript_empty() {
    assert_eq!(escape_applescript(""), "");
}

#[test]
fn test_escape_applescript_no_escaping_needed() {
    assert_eq!(escape_applescript("Hello World"), "Hello World");
}

#[test]
fn test_escape_powershell_single_quote() {
    assert_eq!(escape_powershell("Hello 'World'"), "Hello ''World''");
}

#[test]
fn test_escape_powershell_multiple_quotes() {
    assert_eq!(escape_powershell("'a''b'"), "''a''''b''");
}

#[test]
fn test_escape_powershell_empty() {
    assert_eq!(escape_powershell(""), "");
}

#[test]
fn test_escape_powershell_no_escaping_needed() {
    assert_eq!(escape_powershell("Hello World"), "Hello World");
}

#[test]
fn test_sanitize_string_removes_semicolon() {
    assert_eq!(sanitize_string("foo; bar"), "foo bar");
}

#[test]
fn test_sanitize_string_removes_backticks() {
    assert_eq!(sanitize_string("foo`bar"), "foobar");
}

#[test]
fn test_sanitize_string_removes_pipe() {
    assert_eq!(sanitize_string("foo|bar"), "foobar");
}

#[test]
fn test_sanitize_string_preserves_newlines() {
    assert_eq!(sanitize_string("line1\nline2"), "line1\nline2");
}

#[test]
fn test_sanitize_string_preserves_tabs() {
    assert_eq!(sanitize_string("col1\tcol2"), "col1\tcol2");
}

#[test]
fn test_sanitize_string_removes_control_chars() {
    assert_eq!(sanitize_string("foo\x00bar"), "foobar");
}

#[test]
fn test_sanitize_string_unicode() {
    assert_eq!(sanitize_string("Hello 世界"), "Hello 世界");
}

#[test]
fn test_sanitize_string_empty() {
    assert_eq!(sanitize_string(""), "");
}
