//! Masking the value for display: the mask styles and the peek

use super::{MaskStyle, MaskedInput};

impl MaskedInput {
    /// Get masked display string
    ///
    /// This method is optimized to minimize string allocations by:
    /// - Pre-allocating strings with known capacity
    /// - Avoiding repeated `.to_string().repeat()` calls
    /// - Using `extend` with char iterators instead of format!
    pub fn masked_display(&self) -> String {
        let value = self.get_value();
        if self.revealing {
            return value.to_string();
        }

        let len = self.char_count();
        if len == 0 {
            return String::new();
        }

        match self.mask_style {
            MaskStyle::Full => {
                // Pre-allocate with exact capacity
                std::iter::repeat_n(self.mask_char, len).collect()
            }
            MaskStyle::ShowLast(n) => {
                if len <= n {
                    value.to_string()
                } else {
                    let mask_count = len - n;
                    let mut result = String::with_capacity(len);
                    result.extend(std::iter::repeat_n(self.mask_char, mask_count));
                    result.push_str(&value[self.byte_at(len - n)..]);
                    result
                }
            }
            MaskStyle::ShowFirst(n) => {
                if len <= n {
                    value.to_string()
                } else {
                    let mut result = String::with_capacity(len);
                    result.push_str(&value[..self.byte_at(n)]);
                    result.extend(std::iter::repeat_n(self.mask_char, len - n));
                    result
                }
            }
            MaskStyle::Peek => {
                let cursor = self.get_cursor();
                if self.peek_countdown > 0 && cursor > 0 && cursor <= len {
                    // Show the last typed character
                    let last_char = value.chars().nth(cursor - 1).unwrap_or(' ');
                    let mut result = String::with_capacity(len);
                    result.extend(std::iter::repeat_n(self.mask_char, cursor - 1));
                    result.push(last_char);
                    result.extend(std::iter::repeat_n(self.mask_char, len - cursor));
                    result
                } else {
                    std::iter::repeat_n(self.mask_char, len).collect()
                }
            }
            MaskStyle::Hidden => String::new(),
        }
    }
}
