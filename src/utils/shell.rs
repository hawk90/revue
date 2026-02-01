//! Shell-safe string escaping for command invocation
//!
//! This module provides utilities for safely escaping strings that will be
//! passed to shell commands, preventing command injection vulnerabilities.

/// Escape a string for safe use in AppleScript quoted strings
///
/// AppleScript strings use backslash escapes. This function escapes:
/// - Double quotes (")
/// - Backslashes (\)
/// - Line feeds (\n) as \n
/// - Carriage returns (\r) as \r
/// - Tabs (\t) as \t
///
/// # Example
/// ```
/// use revue::utils::shell::escape_applescript;
/// assert_eq!(escape_applescript("Hello \"World\""), r#"Hello \"World\""#);
/// assert_eq!(escape_applescript("foo\\bar"), r#"foo\\bar"#);
/// ```
pub fn escape_applescript(s: &str) -> String {
    let mut result = String::with_capacity(s.len() * 2);
    for c in s.chars() {
        match c {
            '\\' => result.push_str("\\\\"),
            '"' => result.push_str("\\\""),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            _ => result.push(c),
        }
    }
    result
}

/// Escape a string for safe use in PowerShell single-quoted strings
///
/// PowerShell single-quoted strings only escape single quotes by doubling them.
/// This is the safest approach for PowerShell as single-quoted strings don't
/// interpret any other escape sequences.
///
/// # Example
/// ```text
/// use revue::utils::shell::escape_powershell;
/// assert_eq!(escape_powershell("Hello 'World''), "Hello ''World''");
/// ```
pub fn escape_powershell(s: &str) -> String {
    s.replace('\'', "''")
}

/// Sanitize a string by removing potentially dangerous characters
///
/// This is a fallback for when escaping is not feasible. It removes:
/// - Control characters (except newline, tab, carriage return)
/// - Backslashes
/// - Quotes (both single and double)
/// - Dollar signs (variable expansion in shells)
/// - Backticks (command substitution in shells)
/// - Pipe and other shell metacharacters
///
/// # Example
/// ```
/// use revue::utils::shell::sanitize_string;
/// assert_eq!(sanitize_string("foo; rm -rf /"), "foo rm -rf /");
/// ```
pub fn sanitize_string(s: &str) -> String {
    // Shell metacharacters to remove
    const SHELL_META: &[char] = &[
        '\\', '"', '\'', '$', '`', ';', '|', '&', '(', ')', '<', '>', '[', ']', '{', '}', '*', '?',
        '!', '#', '%', '~',
    ];

    s.chars()
        .filter(|&c| {
            // Filter out shell metacharacters and control characters
            // Allow: alphanumeric, basic punctuation, spaces, newlines, tabs
            // Allow: Unicode printable characters
            !SHELL_META.contains(&c)
                && (c.is_ascii_alphanumeric()
                    || c.is_ascii_whitespace()
                    || matches!(
                        c,
                        ',' | '.' | '-' | '_' | '=' | '+' | '/' | '@' | '\u{80}'..='\u{FFFF}'
                    ))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
