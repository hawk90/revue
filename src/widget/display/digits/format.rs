//! The displayed string: zero or blank padding and thousands separators

use super::Digits;

impl Digits {
    /// Build the display string with formatting
    pub fn format_value(&self) -> String {
        let mut result = self.value.clone();

        if self.leading_zeros == Some(false) {
            // Pad with blanks in front of the formatted number
            if let Some(sep) = self.separator {
                result = self.add_thousands_separator(&result, sep);
            }
            if let Some(width) = self.min_width {
                let len = result.chars().count();
                if len < width {
                    result = format!("{}{}", " ".repeat(width - len), result);
                }
            }
            return result;
        }

        // Apply minimum width with leading zeros, after any sign
        if let Some(width) = self.min_width {
            if result.len() < width {
                let pad = "0".repeat(width - result.len());
                result = match result.strip_prefix('-') {
                    Some(digits) => format!("-{}{}", pad, digits),
                    None => format!("{}{}", pad, result),
                };
            }
        }

        // Apply thousands separator
        if let Some(sep) = self.separator {
            result = self.add_thousands_separator(&result, sep);
        }

        result
    }

    fn add_thousands_separator(&self, s: &str, sep: char) -> String {
        // Handle negative numbers
        let (sign, num) = if let Some(rest) = s.strip_prefix('-') {
            ("-", rest)
        } else {
            ("", s)
        };

        // Split on decimal point
        let parts: Vec<&str> = num.split('.').collect();
        let integer_part = parts[0];
        let decimal_part = parts.get(1);

        // Add separators to integer part
        let mut result = String::new();
        for (i, c) in integer_part.chars().rev().enumerate() {
            if i > 0 && i % 3 == 0 {
                result.push(sep);
            }
            result.push(c);
        }
        let integer_with_sep: String = result.chars().rev().collect();

        // Reconstruct
        match decimal_part {
            Some(dec) => format!("{}{}.{}", sign, integer_with_sep, dec),
            None => format!("{}{}", sign, integer_with_sep),
        }
    }
}
