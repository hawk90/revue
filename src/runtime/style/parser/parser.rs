//! Core CSS parsing functions

use crate::style::{ErrorCode, ParseError};
use std::collections::HashMap;

use super::helpers::{make_error, missing_brace_error};
use super::scanner::{skip_whitespace_and_comments_bytes, skip_whitespace_bytes};
use super::types::{Declaration, Rule, StyleSheet};

/// Parse :root variables block using zero-copy str slicing
pub fn parse_root_variables_str(
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
pub fn parse_selector_str(css: &str, mut pos: usize) -> Result<(String, usize), ParseError> {
    let bytes = css.as_bytes();
    let start = pos;
    while pos < bytes.len() && bytes[pos] != b'{' {
        pos += 1;
    }
    Ok((css[start..pos].trim().to_string(), pos))
}

/// Parse declarations block using zero-copy str slicing
pub fn parse_declarations_str(
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

/// Parse CSS text into a StyleSheet
///
/// This parser uses zero-copy `&str` slicing for efficiency, avoiding `Vec<char>` allocation.
pub fn parse(css: &str) -> Result<StyleSheet, ParseError> {
    let mut sheet = StyleSheet::new();
    let bytes = css.as_bytes();
    let mut pos = 0;

    while pos < bytes.len() {
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
