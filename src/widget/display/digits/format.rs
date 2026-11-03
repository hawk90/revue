//! The displayed string: zero or blank padding and thousands separators

use super::Digits;

impl Digits {
    /// Build the display string with formatting
    ///
    /// `min_width` counts the value's characters before grouping. Zero
    /// padding puts zeros after any sign and groups them with the digits;
    /// blank padding gives the same width, with blanks in front of the
    /// grouped number, so the two right-align (#1066).
    pub fn format_value(&self) -> String {
        let zero_padded = self.group(&self.zero_padded());
        if self.leading_zeros != Some(false) {
            return zero_padded;
        }
        let shown = self.group(&self.value);
        let width = zero_padded.chars().count();
        let len = shown.chars().count();
        format!("{}{}", " ".repeat(width.saturating_sub(len)), shown)
    }

    /// The value padded with zeros after any sign to `min_width` characters
    fn zero_padded(&self) -> String {
        let value = &self.value;
        let len = value.chars().count();
        match self.min_width {
            Some(width) if len < width => {
                let pad = "0".repeat(width - len);
                match value.strip_prefix('-') {
                    Some(digits) => format!("-{}{}", pad, digits),
                    None => format!("{}{}", pad, value),
                }
            }
            _ => value.clone(),
        }
    }

    /// `s` with thousands separators when one is set and `s` is a number
    /// (a sign, digits and maybe a fraction); anything else, such as a
    /// clock or a version, is left whole
    fn group(&self, s: &str) -> String {
        match self.separator {
            Some(sep) if is_plain_number(s) => self.add_thousands_separator(s, sep),
            _ => s.to_string(),
        }
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

/// Whether `s` is `-`, digits, and maybe `.` and digits
fn is_plain_number(s: &str) -> bool {
    let num = s.strip_prefix('-').unwrap_or(s);
    let (int, frac) = match num.split_once('.') {
        Some((int, frac)) => (int, Some(frac)),
        None => (num, None),
    };
    !int.is_empty()
        && int.bytes().all(|b| b.is_ascii_digit())
        && frac.is_none_or(|f| !f.is_empty() && f.bytes().all(|b| b.is_ascii_digit()))
}
