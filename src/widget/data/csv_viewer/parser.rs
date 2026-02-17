//! CSV parsing functionality

use super::types::Delimiter;

/// Maximum number of rows to prevent memory exhaustion
const MAX_CSV_ROWS: usize = 100_000;

/// Maximum number of columns to prevent memory exhaustion
const MAX_CSV_COLS: usize = 1_000;

/// Detect delimiter from content
pub fn detect_delimiter(content: &str, delimiter: Delimiter) -> char {
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
/// Returns at most MAX_CSV_ROWS rows with at most MAX_CSV_COLS columns each.
/// Excess rows and columns are silently dropped to prevent memory exhaustion.
pub fn parse_csv(content: &str, delimiter: char) -> Vec<Vec<String>> {
    let mut result = Vec::with_capacity(1024);
    let mut current_row = Vec::new();
    let mut current_field = String::new();
    let mut in_quotes = false;
    let mut chars = content.chars().peekable();
    let mut row_count = 0;

    while let Some(c) = chars.next() {
        // Check row limit
        if row_count >= MAX_CSV_ROWS {
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
            // Check column limit
            if current_row.len() >= MAX_CSV_COLS {
                // Skip remaining columns in this row
                while let Some(c) = chars.next() {
                    if c == '\n' {
                        break;
                    }
                }
                if !current_row.iter().all(|s| s.is_empty()) {
                    result.push(current_row);
                    row_count += 1;
                }
                current_row = Vec::new();
                current_field = String::new();
                continue;
            }
        } else if c == '\n' {
            current_row.push(current_field.trim().to_string());
            if !current_row.iter().all(|s| s.is_empty()) {
                result.push(std::mem::take(&mut current_row));
                row_count += 1;
            }
            current_field = String::new();
        } else if c != '\r' {
            current_field.push(c);
        }
    }

    // Handle last field/row
    if !current_field.is_empty() || !current_row.is_empty() {
        current_row.push(current_field.trim().to_string());
        if !current_row.iter().all(|s| s.is_empty()) && row_count < MAX_CSV_ROWS {
            result.push(current_row);
        }
    }

    result
}

/// Calculate optimal column widths
pub fn calculate_column_widths(data: &[Vec<String>]) -> Vec<u16> {
    let col_count = data.first().map(|r| r.len()).unwrap_or(0);
    let mut column_widths = vec![0; col_count];

    for row in data {
        for (col, cell) in row.iter().enumerate() {
            if col < column_widths.len() {
                // Clamp to u16::MAX to prevent truncation
                let width = cell.chars().count().min(u16::MAX as usize) as u16;
                column_widths[col] = column_widths[col].max(width);
            }
        }
    }

    // Cap widths at reasonable maximum
    for w in &mut column_widths {
        *w = (*w).clamp(3, 40);
    }

    column_widths
}
