//! CSV text to rows

use super::types::Delimiter;

/// Maximum number of rows kept from parsed content, to bound memory use
pub(super) const MAX_CSV_ROWS: usize = 100_000;

/// Maximum number of columns kept per row, to bound memory use
pub(super) const MAX_CSV_COLS: usize = 1_000;

/// Detect delimiter from content
pub(super) fn detect_delimiter(content: &str, delimiter: Delimiter) -> char {
    if let Some(c) = delimiter.char() {
        return c;
    }

    // Count occurrences in first few lines
    let first_lines: String = content.lines().take(5).collect::<Vec<_>>().join("\n");

    let delimiters = [',', '\t', ';', '|'];
    let mut best = ',';
    let mut best_count = 0;

    for &d in &delimiters {
        let count = first_lines.matches(d).count();
        if count > best_count {
            best_count = count;
            best = d;
        }
    }

    best
}

/// Parse CSV with given delimiter
///
/// Returns at most [`MAX_CSV_ROWS`] rows with at most [`MAX_CSV_COLS`]
/// columns each. Excess rows and columns are dropped to bound memory use.
pub(super) fn parse_csv(content: &str, delimiter: char) -> Vec<Vec<String>> {
    let mut result = Vec::new();
    let mut current_row = Vec::new();
    let mut current_field = String::new();
    let mut in_quotes = false;
    let mut chars = content.chars().peekable();

    while let Some(c) = chars.next() {
        if result.len() >= MAX_CSV_ROWS {
            break;
        }

        if in_quotes {
            if c == '"' {
                if chars.peek() == Some(&'"') {
                    // Escaped quote
                    current_field.push('"');
                    chars.next();
                } else {
                    // End of quoted field
                    in_quotes = false;
                }
            } else {
                current_field.push(c);
            }
        } else if c == '"' {
            in_quotes = true;
        } else if c == delimiter {
            current_row.push(current_field.trim().to_string());
            current_field = String::new();
            if current_row.len() >= MAX_CSV_COLS {
                // Drop the rest of this line
                for c in chars.by_ref() {
                    if c == '\n' {
                        break;
                    }
                }
                if !current_row.iter().all(|s| s.is_empty()) {
                    result.push(current_row);
                }
                current_row = Vec::new();
            }
        } else if c == '\n' {
            current_row.push(current_field.trim().to_string());
            if !current_row.iter().all(|s| s.is_empty()) {
                result.push(current_row);
            }
            current_row = Vec::new();
            current_field = String::new();
        } else if c != '\r' {
            current_field.push(c);
        }
    }

    // Handle last field/row
    if !current_field.is_empty() || !current_row.is_empty() {
        current_row.push(current_field.trim().to_string());
        if !current_row.iter().all(|s| s.is_empty()) && result.len() < MAX_CSV_ROWS {
            result.push(current_row);
        }
    }

    result
}
