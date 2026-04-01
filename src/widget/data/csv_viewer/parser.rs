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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_delimiter_comma() {
        let csv = "a,b,c\n1,2,3";
        assert_eq!(detect_delimiter(csv, Delimiter::Auto), ',');
    }

    #[test]
    fn test_detect_delimiter_tab() {
        let csv = "a\tb\tc\n1\t2\t3";
        assert_eq!(detect_delimiter(csv, Delimiter::Auto), '\t');
    }

    #[test]
    fn test_detect_delimiter_explicit() {
        let csv = "a,b,c";
        assert_eq!(detect_delimiter(csv, Delimiter::Semicolon), ';');
        assert_eq!(detect_delimiter(csv, Delimiter::Pipe), '|');
    }

    #[test]
    fn test_parse_csv_simple() {
        let csv = "a,b,c\n1,2,3\n4,5,6";
        let result = parse_csv(csv, ',');
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], vec!["a", "b", "c"]);
        assert_eq!(result[1], vec!["1", "2", "3"]);
        assert_eq!(result[2], vec!["4", "5", "6"]);
    }

    #[test]
    fn test_parse_csv_quoted_fields() {
        let csv = r#""hello","world""#;
        let result = parse_csv(csv, ',');
        assert_eq!(result[0], vec!["hello", "world"]);
    }

    #[test]
    fn test_parse_csv_escaped_quotes() {
        let csv = r#""he said ""hi""",b"#;
        let result = parse_csv(csv, ',');
        assert_eq!(result[0][0], r#"he said "hi""#);
    }

    #[test]
    fn test_parse_csv_quoted_delimiter() {
        let csv = r#""a,b",c"#;
        let result = parse_csv(csv, ',');
        assert_eq!(result[0], vec!["a,b", "c"]);
    }

    #[test]
    fn test_parse_csv_empty() {
        let result = parse_csv("", ',');
        assert!(result.is_empty());
    }

    #[test]
    fn test_parse_csv_whitespace_trimmed() {
        let csv = " a , b , c ";
        let result = parse_csv(csv, ',');
        assert_eq!(result[0], vec!["a", "b", "c"]);
    }

    #[test]
    fn test_parse_csv_crlf() {
        let csv = "a,b\r\n1,2\r\n";
        let result = parse_csv(csv, ',');
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_calculate_column_widths() {
        let data = vec![
            vec!["Name".to_string(), "Age".to_string()],
            vec!["Alice".to_string(), "30".to_string()],
            vec!["Bob".to_string(), "25".to_string()],
        ];
        let widths = calculate_column_widths(&data);
        assert_eq!(widths.len(), 2);
        assert!(widths[0] >= 5); // "Alice" = 5 chars
        assert!(widths[1] >= 3); // "Age" = 3 chars
    }

    #[test]
    fn test_calculate_column_widths_empty() {
        let widths = calculate_column_widths(&[]);
        assert!(widths.is_empty());
    }
}

/// Calculate optimal column widths
pub fn calculate_column_widths(data: &[Vec<String>]) -> Vec<u16> {
    let col_count = data.first().map(|r| r.len()).unwrap_or(0);
    let mut column_widths = vec![0; col_count];

    for row in data {
        for (col, cell) in row.iter().enumerate() {
            if col < column_widths.len() {
                // Use display width for proper CJK/emoji column sizing
                let width = crate::utils::display_width(cell).min(u16::MAX as usize) as u16;
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
