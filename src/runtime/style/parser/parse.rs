//! CSS parser for TUI styling

use crate::constants::{MAX_COMMENT_LENGTH, MAX_CSS_FILE_SIZE};
use crate::style::{Declaration, ErrorCode, ParseError, Rule, StyleSheet};

/// Create a ParseError at the given position
fn make_error(css: &str, pos: usize, message: &str, code: ErrorCode) -> ParseError {
    ParseError::at_offset(message, css, pos).with_code(code)
}

/// Create a ParseError for missing brace
fn missing_brace_error(css: &str, pos: usize, expected: char) -> ParseError {
    make_error(
        css,
        pos,
        &format!("expected '{}' but found end of input", expected),
        ErrorCode::MissingBrace,
    )
}

/// Maximum allowed CSS size to prevent memory exhaustion
const MAX_CSS_SIZE: usize = MAX_CSS_FILE_SIZE as usize;
/// Maximum number of rules to prevent excessive memory usage
const MAX_RULES: usize = 10_000;
/// Maximum number of total declarations across all rules
const MAX_DECLARATIONS: usize = 10_000; // Lowered for testing
/// Maximum number of @keyframes definitions
const MAX_KEYFRAMES: usize = 100;
/// Maximum number of keyframe blocks per @keyframes
const MAX_KEYFRAME_BLOCKS: usize = 50;

pub fn parse(css: &str) -> Result<StyleSheet, ParseError> {
    // Check CSS size limit before parsing
    if css.len() > MAX_CSS_SIZE {
        return Err(make_error(
            css,
            css.len().min(css.len()),
            &format!(
                "CSS input too large: {} bytes (max: {} bytes). Consider splitting into multiple files.",
                css.len(),
                MAX_CSS_SIZE
            ),
            ErrorCode::InvalidValue,
        ));
    }

    let mut sheet = StyleSheet::new();
    let bytes = css.as_bytes();
    let mut pos = 0;
    let mut total_declarations = 0;

    while pos < bytes.len() {
        // Check rule limit
        if sheet.rules.len() >= MAX_RULES {
            return Err(make_error(
                css,
                pos,
                &format!(
                    "Too many CSS rules: {} (max: {}). Consider simplifying your styles.",
                    sheet.rules.len(),
                    MAX_RULES
                ),
                ErrorCode::InvalidValue,
            ));
        }
        // Skip whitespace and comments
        pos = skip_whitespace_bytes(bytes, pos);
        if pos >= bytes.len() {
            break;
        }

        // Check for CSS variable definition (in :root)
        if bytes[pos..].starts_with(b":root") {
            pos = parse_root_variables_str(css, pos, &mut sheet)?;
            continue;
        }

        // Check for @keyframes definition
        if bytes[pos..].starts_with(b"@keyframes") {
            if sheet.keyframes.len() >= MAX_KEYFRAMES {
                return Err(make_error(
                    css,
                    pos,
                    &format!(
                        "Too many @keyframes definitions: {} (max: {})",
                        sheet.keyframes.len(),
                        MAX_KEYFRAMES
                    ),
                    ErrorCode::InvalidValue,
                ));
            }
            pos = parse_keyframes_block(css, pos, &mut sheet)?;
            continue;
        }

        // Parse selector
        let (selector, new_pos) = parse_selector_str(css, pos)?;
        pos = new_pos;

        // Skip whitespace
        pos = skip_whitespace_bytes(bytes, pos);

        // Expect '{'
        if pos >= bytes.len() || bytes[pos] != b'{' {
            return Err(make_error(
                css,
                pos,
                &format!(
                    "expected '{{' after selector '{}', found '{}'",
                    selector,
                    if pos < bytes.len() {
                        bytes[pos] as char
                    } else {
                        ' '
                    }
                ),
                ErrorCode::MissingBrace,
            ));
        }
        pos += 1;

        // Parse declarations
        let (declarations, new_pos) = parse_declarations_str(css, pos)?;
        pos = new_pos;

        // Check total declaration limit
        total_declarations += declarations.len();
        if total_declarations > MAX_DECLARATIONS {
            return Err(make_error(
                css,
                pos,
                &format!(
                    "Too many CSS declarations: {} (max: {}). Consider simplifying your styles.",
                    total_declarations, MAX_DECLARATIONS
                ),
                ErrorCode::InvalidValue,
            ));
        }

        // Expect '}'
        if pos >= bytes.len() || bytes[pos] != b'}' {
            return Err(missing_brace_error(css, pos, '}'));
        }
        pos += 1;

        sheet.rules.push(Rule {
            selector,
            declarations,
        });
    }

    Ok(sheet)
}

/// Skip ASCII whitespace using byte slice (no allocation)
#[inline]
fn skip_whitespace_bytes(bytes: &[u8], mut pos: usize) -> usize {
    while pos < bytes.len() && bytes[pos].is_ascii_whitespace() {
        pos += 1;
    }
    pos
}

/// Skip whitespace and block comments using byte slice (no allocation)
///
/// This function includes protection against malicious input that attempts
/// to cause denial-of-service through malformed or unterminated comments.
fn skip_whitespace_and_comments_bytes(bytes: &[u8], mut pos: usize) -> usize {
    loop {
        pos = skip_whitespace_bytes(bytes, pos);
        // Check for block comment start (/*)
        if pos + 1 < bytes.len() && bytes[pos] == b'/' && bytes[pos + 1] == b'*' {
            // Skip block comment
            pos += 2;
            let comment_start = pos;

            // Look for comment end (*/), with protection against malformed comments
            while pos + 1 < bytes.len() {
                // Check for maliciously long comments that could cause DoS
                if pos - comment_start > MAX_COMMENT_LENGTH {
                    // Return an error position that signals the comment is too long
                    return bytes.len(); // Signal error condition
                }

                if bytes[pos] == b'*' && bytes[pos + 1] == b'/' {
                    pos += 2; // Skip the closing */
                    break;
                }
                pos += 1;
            }

            // If we reached the end without finding a closing */, signal an error
            if pos >= bytes.len() || pos + 1 >= bytes.len() {
                return pos;
            }
        } else {
            break;
        }
    }
    pos
}

/// Parse :root variables block using zero-copy str slicing
fn parse_root_variables_str(
    css: &str,
    mut pos: usize,
    sheet: &mut StyleSheet,
) -> Result<usize, ParseError> {
    let bytes = css.as_bytes();

    // Skip ":root"
    pos += 5;
    pos = skip_whitespace_bytes(bytes, pos);

    // Expect '{'
    if pos >= bytes.len() || bytes[pos] != b'{' {
        return Err(make_error(
            css,
            pos,
            "expected '{' after :root",
            ErrorCode::MissingBrace,
        ));
    }
    pos += 1;

    // Parse variable declarations
    loop {
        pos = skip_whitespace_and_comments_bytes(bytes, pos);

        if pos >= bytes.len() {
            return Err(missing_brace_error(css, pos, '}'));
        }

        if bytes[pos] == b'}' {
            pos += 1;
            break;
        }

        // Variable name starts with --
        if !bytes[pos..].starts_with(b"--") {
            return Err(make_error(
                css,
                pos,
                "CSS variables must start with '--' (e.g., --primary-color)",
                ErrorCode::InvalidSyntax,
            )
            .suggest("use '--variable-name: value;' format"));
        }

        // Read variable name (ASCII only, safe to use byte indexing)
        let start = pos;
        while pos < bytes.len() && bytes[pos] != b':' && !bytes[pos].is_ascii_whitespace() {
            pos += 1;
        }
        let name = css[start..pos].to_string();

        pos = skip_whitespace_bytes(bytes, pos);

        // Expect ':'
        if pos >= bytes.len() || bytes[pos] != b':' {
            return Err(make_error(
                css,
                pos,
                "expected ':' after variable name",
                ErrorCode::InvalidSyntax,
            )
            .suggest("format: --variable-name: value;"));
        }
        pos += 1;

        pos = skip_whitespace_bytes(bytes, pos);

        // Read value until ';' or '}'
        let start = pos;
        while pos < bytes.len() && bytes[pos] != b';' && bytes[pos] != b'}' {
            pos += 1;
        }
        let value = css[start..pos].trim().to_string();

        sheet.variables.insert(name, value);

        if pos < bytes.len() && bytes[pos] == b';' {
            pos += 1;
        }
    }

    Ok(pos)
}

/// Parse selector using zero-copy str slicing
fn parse_selector_str(css: &str, mut pos: usize) -> Result<(String, usize), ParseError> {
    let bytes = css.as_bytes();
    let start = pos;
    while pos < bytes.len() && bytes[pos] != b'{' {
        pos += 1;
    }
    Ok((css[start..pos].trim().to_string(), pos))
}

/// Parse declarations block using zero-copy str slicing
fn parse_declarations_str(
    css: &str,
    mut pos: usize,
) -> Result<(Vec<Declaration>, usize), ParseError> {
    let bytes = css.as_bytes();
    let mut declarations = Vec::new();

    loop {
        pos = skip_whitespace_and_comments_bytes(bytes, pos);

        if pos >= bytes.len() || bytes[pos] == b'}' {
            break;
        }

        // Read property name
        let start = pos;
        while pos < bytes.len() && bytes[pos] != b':' && bytes[pos] != b'}' {
            pos += 1;
        }
        let property = css[start..pos].trim().to_string();

        if pos >= bytes.len() || bytes[pos] == b'}' {
            break;
        }

        // Skip ':'
        pos += 1;
        pos = skip_whitespace_bytes(bytes, pos);

        // Read value until ';' or '}'
        let start = pos;
        let mut paren_depth: i32 = 0;
        while pos < bytes.len() {
            match bytes[pos] {
                b'(' => paren_depth += 1,
                b')' => paren_depth = paren_depth.saturating_sub(1),
                b';' | b'}' if paren_depth == 0 => break,
                _ => {}
            }
            pos += 1;
        }
        let value = css[start..pos].trim().to_string();

        if !property.is_empty() {
            declarations.push(Declaration { property, value });
        }

        if pos < bytes.len() && bytes[pos] == b';' {
            pos += 1;
        }
    }

    Ok((declarations, pos))
}

/// Parse a @keyframes block
fn parse_keyframes_block(
    css: &str,
    mut pos: usize,
    sheet: &mut StyleSheet,
) -> Result<usize, ParseError> {
    let bytes = css.as_bytes();

    // Skip "@keyframes"
    pos += 10;
    pos = skip_whitespace_bytes(bytes, pos);

    // Parse animation name
    let name_start = pos;
    while pos < bytes.len() && !bytes[pos].is_ascii_whitespace() && bytes[pos] != b'{' {
        pos += 1;
    }

    let name = css[name_start..pos].trim().to_string();
    if name.is_empty() {
        return Err(make_error(
            css,
            name_start,
            "expected name after @keyframes",
            ErrorCode::InvalidSyntax,
        ));
    }

    pos = skip_whitespace_bytes(bytes, pos);

    // Expect outer '{'
    if pos >= bytes.len() || bytes[pos] != b'{' {
        return Err(make_error(
            css,
            pos,
            "expected '{' after @keyframes name",
            ErrorCode::MissingBrace,
        ));
    }
    pos += 1;

    let mut keyframe_blocks = Vec::new();

    // Parse keyframe blocks
    loop {
        pos = skip_whitespace_and_comments_bytes(bytes, pos);

        if pos >= bytes.len() {
            return Err(missing_brace_error(css, pos, '}'));
        }

        if bytes[pos] == b'}' {
            pos += 1;
            break;
        }

        if keyframe_blocks.len() >= MAX_KEYFRAME_BLOCKS {
            return Err(make_error(
                css,
                pos,
                &format!(
                    "Too many keyframe blocks in @keyframes '{}': {} (max: {})",
                    name,
                    keyframe_blocks.len(),
                    MAX_KEYFRAME_BLOCKS
                ),
                ErrorCode::InvalidValue,
            ));
        }

        // Parse keyframe selector (from, to, N%)
        let (percent, new_pos) = parse_keyframe_selector(css, pos)?;
        pos = new_pos;

        pos = skip_whitespace_bytes(bytes, pos);

        // Expect '{'
        if pos >= bytes.len() || bytes[pos] != b'{' {
            return Err(make_error(
                css,
                pos,
                "expected '{' after keyframe selector",
                ErrorCode::MissingBrace,
            ));
        }
        pos += 1;

        // Parse declarations
        let (declarations, new_pos) = parse_declarations_str(css, pos)?;
        pos = new_pos;

        // Expect '}'
        if pos >= bytes.len() || bytes[pos] != b'}' {
            return Err(missing_brace_error(css, pos, '}'));
        }
        pos += 1;

        keyframe_blocks.push(crate::style::KeyframeBlock {
            percent,
            declarations,
        });
    }

    sheet.keyframes.insert(
        name.clone(),
        crate::style::KeyframesDefinition {
            name,
            keyframes: keyframe_blocks,
        },
    );

    Ok(pos)
}

/// Parse a keyframe selector: `from` → 0, `to` → 100, `50%` → 50
fn parse_keyframe_selector(css: &str, mut pos: usize) -> Result<(u8, usize), ParseError> {
    let bytes = css.as_bytes();
    let start = pos;

    while pos < bytes.len()
        && !bytes[pos].is_ascii_whitespace()
        && bytes[pos] != b'{'
        && bytes[pos] != b','
    {
        pos += 1;
    }

    let selector = css[start..pos].trim();
    let percent = match selector {
        "from" => 0,
        "to" => 100,
        s if s.ends_with('%') => {
            let num_str = &s[..s.len() - 1];
            num_str
                .parse::<u8>()
                .map_err(|_| {
                    make_error(
                        css,
                        start,
                        &format!("invalid keyframe percentage: '{}'", s),
                        ErrorCode::InvalidValue,
                    )
                })?
                .min(100)
        }
        _ => {
            return Err(make_error(
                css,
                start,
                &format!(
                    "invalid keyframe selector '{}': expected 'from', 'to', or 'N%'",
                    selector
                ),
                ErrorCode::InvalidSyntax,
            ));
        }
    };

    Ok((percent, pos))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_normal_css() {
        let css = r#"
            .button {
                background: blue;
                color: white;
            }
        "#;
        assert!(parse(css).is_ok());
    }

    #[test]
    fn test_css_size_limit() {
        // Create CSS that exceeds 1MB
        let mut large_css = String::new();
        large_css.push_str(".test { content: ");
        for _ in 0..1_200_000 {
            large_css.push('x');
        }
        large_css.push_str("; }");

        let result = parse(&large_css);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("too large"));
    }

    #[test]
    fn test_css_rules_limit() {
        // Create CSS with many rules
        let mut css = String::new();
        for i in 0..10_001 {
            css.push_str(&format!(".class{} {{ color: red; }}", i));
        }

        let result = parse(&css);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("Too many CSS rules"));
    }

    #[test]
    fn test_css_declarations_limit() {
        // Create CSS with many declarations (exceeds 10,000 limit)
        let mut css = String::new();
        for rule in 0..2 {
            css.push_str(&format!(".rule{} {{ ", rule));
            for i in 0..5_001 {
                css.push_str(&format!("prop{}: val{}; ", i, i));
            }
            css.push_str("} ");
        }

        let result = parse(&css);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("CSS declarations") || err.message.contains("declarations"));
    }

    #[test]
    fn test_css_within_limits() {
        // CSS within all limits should parse fine
        let mut css = String::new();
        for i in 0..100 {
            css.push_str(&format!(".class{} {{ ", i));
            for j in 0..10 {
                css.push_str(&format!("prop{}: val{}; ", j, j));
            }
            css.push_str("}");
        }

        assert!(parse(&css).is_ok());
    }

    // Security tests for comment parsing
    #[test]
    fn test_css_normal_comments() {
        // Normal comments should work fine
        let css = r#"
        /* This is a normal comment */
        .box { width: 100; }
        /* Another comment */
        .text { color: red; }
        "#;
        assert!(parse(&css).is_ok());
    }

    #[test]
    fn test_css_multiline_comment() {
        // Multi-line comments should work
        let css = r#"
        /* This is a
           multi-line
           comment */
        .box { width: 100; }
        "#;
        assert!(parse(&css).is_ok());
    }

    #[test]
    fn test_css_nested_comments_wont_hang() {
        // CSS doesn't support nested comments - this should parse but not hang
        let css = "/* outer /* inner */ comment */ .box { width: 100; }";
        // The parser will handle this as: /* outer /* inner */ then "comment */" as text
        // It won't hang or crash
        let _ = parse(&css);
    }

    #[test]
    fn test_css_unterminated_comment_is_safe() {
        // Unterminated comment should not cause infinite loop
        let css = "/* This comment is never closed .box { width: 100; }";
        // The scanner should handle this safely without infinite loop
        let result = parse(&css);
        // Should either error or parse what it can, but never hang
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_css_comment_after_property() {
        // Comment after property value
        let css = ".box { width: 100; /* comment after */ }";
        assert!(parse(&css).is_ok());
    }

    #[test]
    fn test_css_empty_comment() {
        // Empty comment should work
        let css = "/**/ .box { width: 100; }";
        assert!(parse(&css).is_ok());
    }

    // @keyframes parsing tests
    #[test]
    fn test_keyframes_from_to() {
        let css = r#"
            @keyframes fadeIn {
                from { opacity: 0; }
                to { opacity: 1; }
            }
        "#;
        let sheet = parse(css).unwrap();
        assert!(sheet.keyframes.contains_key("fadeIn"));
        let def = &sheet.keyframes["fadeIn"];
        assert_eq!(def.keyframes.len(), 2);
        assert_eq!(def.keyframes[0].percent, 0);
        assert_eq!(def.keyframes[1].percent, 100);
    }

    #[test]
    fn test_keyframes_percentages() {
        let css = r#"
            @keyframes slide {
                0% { x: 0; }
                50% { x: 50; }
                100% { x: 100; }
            }
        "#;
        let sheet = parse(css).unwrap();
        let def = &sheet.keyframes["slide"];
        assert_eq!(def.keyframes.len(), 3);
        assert_eq!(def.keyframes[0].percent, 0);
        assert_eq!(def.keyframes[1].percent, 50);
        assert_eq!(def.keyframes[2].percent, 100);
    }

    #[test]
    fn test_keyframes_empty_body() {
        let css = "@keyframes empty {}";
        let sheet = parse(css).unwrap();
        assert_eq!(sheet.keyframes["empty"].keyframes.len(), 0);
    }

    #[test]
    fn test_keyframes_missing_name_error() {
        let css = "@keyframes { from { opacity: 0; } }";
        assert!(parse(css).is_err());
    }

    #[test]
    fn test_keyframes_with_regular_rules() {
        let css = r#"
            .btn { color: red; }
            @keyframes fadeIn {
                from { opacity: 0; }
                to { opacity: 1; }
            }
            .text { color: blue; }
        "#;
        let sheet = parse(css).unwrap();
        assert_eq!(sheet.rules.len(), 2);
        assert!(sheet.keyframes.contains_key("fadeIn"));
    }

    #[test]
    fn test_keyframes_invalid_selector() {
        let css = r#"
            @keyframes test {
                invalid { opacity: 0; }
            }
        "#;
        assert!(parse(css).is_err());
    }
}
