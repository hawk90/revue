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
        if self.revealing {
            return self.value.clone();
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
                    self.value.clone()
                } else {
                    let mask_count = len - n;
                    let mut result = String::with_capacity(len);
                    result.extend(std::iter::repeat_n(self.mask_char, mask_count));
                    result.push_str(&self.value[self.byte_at(len - n)..]);
                    result
                }
            }
            MaskStyle::ShowFirst(n) => {
                if len <= n {
                    self.value.clone()
                } else {
                    let mut result = String::with_capacity(len);
                    result.push_str(&self.value[..self.byte_at(n)]);
                    result.extend(std::iter::repeat_n(self.mask_char, len - n));
                    result
                }
            }
            MaskStyle::Peek => {
                if self.peek_countdown > 0 && self.cursor > 0 && self.cursor <= len {
                    // Show the last typed character
                    // Use char_indices for O(n) instead of O(n²) with .chars().nth()
                    let last_char = self
                        .value
                        .char_indices()
                        .nth(self.cursor - 1)
                        .map(|(_, c)| c)
                        .unwrap_or(' ');
                    let mut result = String::with_capacity(len);
                    result.extend(std::iter::repeat_n(self.mask_char, self.cursor - 1));
                    result.push(last_char);
                    result.extend(std::iter::repeat_n(self.mask_char, len - self.cursor));
                    result
                } else {
                    std::iter::repeat_n(self.mask_char, len).collect()
                }
            }
            MaskStyle::Hidden => String::new(),
        }
    }
}
