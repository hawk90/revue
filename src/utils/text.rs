//! Text manipulation utilities
//!
//! Common text processing functions used across widgets.

// =============================================================================
// Character Index Utilities (UTF-8 safe)
// =============================================================================

/// Get byte index from character index in a string
///
/// This is essential for UTF-8 safe string manipulation where you need
/// to work with character positions but String operations require byte indices.
///
/// # Arguments
/// * `s` - The string to index into
/// * `char_idx` - The character index (0-based)
///
/// # Returns
/// The byte index corresponding to the character index, or string length if out of bounds
///
/// # Example
/// ```ignore
/// let s = "héllo";
/// assert_eq!(char_to_byte_index(s, 0), 0); // 'h'
/// assert_eq!(char_to_byte_index(s, 1), 1); // 'é' starts at byte 1
/// assert_eq!(char_to_byte_index(s, 2), 3); // 'l' starts at byte 3 (é is 2 bytes)
/// ```
#[inline]
pub fn char_to_byte_index(s: &str, char_idx: usize) -> usize {
    s.char_indices()
        .nth(char_idx)
        .map(|(i, _)| i)
        .unwrap_or(s.len())
}

/// Get byte index from character index, also returning the character at that position
///
/// # Returns
/// Tuple of `(byte_index, Option<char>)` where char is None if index is out of bounds
#[inline]
pub fn char_to_byte_index_with_char(s: &str, char_idx: usize) -> (usize, Option<char>) {
    s.char_indices()
        .nth(char_idx)
        .map(|(i, c)| (i, Some(c)))
        .unwrap_or((s.len(), None))
}

/// Get character index from byte index
///
/// # Arguments
/// * `s` - The string
/// * `byte_idx` - The byte index
///
/// # Returns
/// The character index, or character count if byte_idx is at or past end
///
/// # Safety
/// This function safely handles invalid byte indices by clamping to string bounds
/// and validating UTF-8 boundaries.
#[inline]
pub fn byte_to_char_index(s: &str, byte_idx: usize) -> usize {
    // Clamp to valid range
    let byte_idx = byte_idx.min(s.len());

    // Use is_char_boundary to safely handle invalid byte positions
    // If not at a valid boundary, find the nearest valid boundary
    let safe_idx = if s.is_char_boundary(byte_idx) {
        byte_idx
    } else {
        // Find the previous valid character boundary by scanning backwards
        let mut safe = 0;
        for (i, _) in s.char_indices() {
            if i <= byte_idx && s.is_char_boundary(i) {
                safe = i;
            } else if i > byte_idx {
                break;
            }
        }
        safe
    };

    s[..safe_idx].chars().count()
}

/// Count characters in a string (more explicit than .chars().count())
#[inline]
pub fn char_count(s: &str) -> usize {
    s.chars().count()
}

/// Get a substring by character indices (not byte indices)
///
/// # Arguments
/// * `s` - The string to slice
/// * `start` - Start character index (inclusive)
/// * `end` - End character index (exclusive)
///
/// # Returns
/// The substring, or empty string if indices are invalid
///
/// # Safety
/// This function safely handles out-of-bounds indices and ensures
/// all byte indices are at valid UTF-8 character boundaries.
pub fn char_slice(s: &str, start: usize, end: usize) -> &str {
    if start >= end || start >= char_count(s) {
        return "";
    }

    let start_byte = char_to_byte_index(s, start);
    let end_byte = char_to_byte_index(s, end).min(s.len());

    // Ensure both indices are at valid UTF-8 boundaries
    if !s.is_char_boundary(start_byte) || !s.is_char_boundary(end_byte) {
        return "";
    }

    &s[start_byte..end_byte]
}

/// Insert a string at a character position
///
/// # Arguments
/// * `s` - The string to modify
/// * `char_idx` - Character position to insert at
/// * `insert` - String to insert
///
/// # Returns
/// New cursor position (char_idx + inserted char count)
pub fn insert_at_char(s: &mut String, char_idx: usize, insert: &str) -> usize {
    let byte_idx = char_to_byte_index(s, char_idx);
    s.insert_str(byte_idx, insert);
    char_idx + insert.chars().count()
}

/// Remove a character at a character position
///
/// # Arguments
/// * `s` - The string to modify
/// * `char_idx` - Character position to remove
///
/// # Returns
/// The removed character, or None if index was out of bounds
pub fn remove_char_at(s: &mut String, char_idx: usize) -> Option<char> {
    let (byte_idx, maybe_char) = char_to_byte_index_with_char(s, char_idx);
    if let Some(ch) = maybe_char {
        s.drain(byte_idx..byte_idx + ch.len_utf8());
        Some(ch)
    } else {
        None
    }
}

/// Remove a range of characters (start..end in character indices)
///
/// # Arguments
/// * `s` - The string to modify
/// * `start` - Start character index (inclusive)
/// * `end` - End character index (exclusive)
pub fn remove_char_range(s: &mut String, start: usize, end: usize) {
    if start >= end {
        return;
    }
    let start_byte = char_to_byte_index(s, start);
    let end_byte = char_to_byte_index(s, end);
    s.drain(start_byte..end_byte);
}

/// Truncate text to fit within max_width, adding ellipsis if needed
///
/// # Arguments
/// * `text` - Text to truncate
/// * `max_width` - Maximum character width
///
/// # Returns
/// Truncated string with ellipsis if truncation occurred
///
/// # Example
/// ```ignore
/// let short = truncate("Hello World", 8);
/// assert_eq!(short, "Hello…");
/// ```
pub fn truncate(text: &str, max_width: usize) -> String {
    let char_count = text.chars().count();

    if char_count <= max_width {
        // Fast path: no truncation needed
        // Pre-allocate exact capacity to avoid reallocation
        let mut result = String::with_capacity(text.len());
        result.push_str(text);
        result
    } else if max_width <= 1 {
        // Edge case: just the ellipsis
        String::from("…")
    } else {
        // Pre-allocate capacity for truncated text + ellipsis
        // Estimate: (max_width - 1) chars * 3 bytes/char (UTF-8 worst case) + 3 bytes for ellipsis
        let capacity = (max_width.saturating_sub(1)) * 3 + 3;
        let mut result = String::with_capacity(capacity);

        for (i, c) in text.chars().enumerate() {
            if i >= max_width.saturating_sub(1) {
                break;
            }
            result.push(c);
        }
        result.push('…');
        result
    }
}

/// Truncate text from the start, adding ellipsis at beginning
///
/// # Example
/// ```ignore
/// let short = truncate_start("/home/user/documents/file.txt", 20);
/// assert_eq!(short, "…ments/file.txt");
/// ```
pub fn truncate_start(text: &str, max_width: usize) -> String {
    let char_count = text.chars().count();

    if char_count <= max_width {
        // Fast path: no truncation needed
        let mut result = String::with_capacity(text.len());
        result.push_str(text);
        result
    } else if max_width <= 1 {
        // Edge case: just the ellipsis
        String::from("…")
    } else {
        // Pre-allocate capacity
        let keep = max_width.saturating_sub(1);
        let capacity = keep * 3 + 3; // Estimate for UTF-8 + ellipsis
        let mut result = String::with_capacity(capacity);

        result.push('…');

        let skip = char_count - keep;
        for (i, c) in text.chars().enumerate() {
            if i >= skip {
                result.push(c);
            }
        }

        result
    }
}

/// Center text within given width
///
/// # Arguments
/// * `text` - Text to center
/// * `width` - Total width to center within
///
/// # Returns
/// Centered string padded with spaces
pub fn center(text: &str, width: usize) -> String {
    let text_len = text.chars().count();
    if text_len >= width {
        // Fast path: no padding needed
        let mut result = String::with_capacity(text.len());
        result.push_str(text);
        result
    } else {
        // Pre-allocate exact capacity
        let padding = width - text_len;
        let left_pad = padding / 2;
        let right_pad = padding - left_pad;
        let capacity = text.len() + left_pad + right_pad;

        let mut result = String::with_capacity(capacity);
        for _ in 0..left_pad {
            result.push(' ');
        }
        result.push_str(text);
        for _ in 0..right_pad {
            result.push(' ');
        }
        result
    }
}

/// Pad text on the left to reach target width
pub fn pad_left(text: &str, width: usize) -> String {
    let text_len = text.chars().count();
    if text_len >= width {
        // Fast path: no padding needed
        let mut result = String::with_capacity(text.len());
        result.push_str(text);
        result
    } else {
        // Pre-allocate exact capacity
        let padding = width - text_len;
        let capacity = text.len() + padding;

        let mut result = String::with_capacity(capacity);
        for _ in 0..padding {
            result.push(' ');
        }
        result.push_str(text);
        result
    }
}

/// Pad text on the right to reach target width
pub fn pad_right(text: &str, width: usize) -> String {
    let text_len = text.chars().count();
    if text_len >= width {
        // Fast path: no padding needed
        let mut result = String::with_capacity(text.len());
        result.push_str(text);
        result
    } else {
        // Pre-allocate exact capacity
        let padding = width - text_len;
        let capacity = text.len() + padding;

        let mut result = String::with_capacity(capacity);
        result.push_str(text);
        for _ in 0..padding {
            result.push(' ');
        }
        result
    }
}

/// Wrap text to fit within max_width
///
/// # Arguments
/// * `text` - Text to wrap
/// * `max_width` - Maximum line width
///
/// # Returns
/// Vector of lines, each fitting within max_width
pub fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    if max_width == 0 || text.is_empty() {
        return vec![];
    }

    let mut lines = Vec::new();

    for paragraph in text.lines() {
        if paragraph.is_empty() {
            lines.push(String::new());
            continue;
        }

        let words: Vec<&str> = paragraph.split_whitespace().collect();
        if words.is_empty() {
            lines.push(String::new());
            continue;
        }

        let mut current_line = String::new();

        for word in words {
            let word_len = word.chars().count();

            // If word is longer than max_width, split it
            if word_len > max_width {
                // Flush current line if not empty
                if !current_line.is_empty() {
                    lines.push(current_line);
                    current_line = String::new();
                }

                // Split long word
                let mut chars = word.chars().peekable();
                while chars.peek().is_some() {
                    let chunk: String = chars.by_ref().take(max_width).collect();
                    if chars.peek().is_some() {
                        lines.push(chunk);
                    } else {
                        current_line = chunk;
                    }
                }
                continue;
            }

            if current_line.is_empty() {
                current_line = word.to_string();
            } else if current_line.chars().count() + 1 + word_len <= max_width {
                current_line.push(' ');
                current_line.push_str(word);
            } else {
                lines.push(current_line);
                current_line = word.to_string();
            }
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }
    }

    lines
}

/// Split text into fixed-width chunks (for display in columns)
pub fn split_fixed_width(text: &str, width: usize) -> Vec<String> {
    if width == 0 {
        return vec![];
    }

    let mut chunks = Vec::new();
    let mut chars = text.chars().peekable();

    while chars.peek().is_some() {
        let chunk: String = chars.by_ref().take(width).collect();
        chunks.push(chunk);
    }

    if chunks.is_empty() {
        chunks.push(String::new());
    }

    chunks
}

/// Get display width of a string (accounting for wide characters)
///
/// Note: This is a simplified version that counts chars.
/// For proper Unicode width handling, consider using unicode-width crate.
pub fn display_width(text: &str) -> usize {
    text.chars().count()
}

/// Repeat a character to create a string
pub fn repeat_char(ch: char, count: usize) -> String {
    std::iter::repeat_n(ch, count).collect()
}

/// Create a horizontal bar using block characters
pub fn progress_bar(value: f64, width: usize) -> String {
    let value = value.clamp(0.0, 1.0);
    let filled = (value * width as f64).round() as usize;
    let empty = width.saturating_sub(filled);

    // Pre-allocate exact capacity (each char is 1-3 bytes, but 3 is safe)
    let capacity = width * 3;
    let mut result = String::with_capacity(capacity);

    for _ in 0..filled {
        result.push('█');
    }
    for _ in 0..empty {
        result.push('░');
    }
    result
}

/// Create a horizontal bar with partial fill character
pub fn progress_bar_precise(value: f64, width: usize) -> String {
    let value = value.clamp(0.0, 1.0);
    let total_eighths = (value * width as f64 * 8.0).round() as usize;
    let full_blocks = total_eighths / 8;
    let remainder = total_eighths % 8;

    let partial = match remainder {
        0 => "",
        1 => "▏",
        2 => "▎",
        3 => "▍",
        4 => "▌",
        5 => "▋",
        6 => "▊",
        7 => "▉",
        _ => "█",
    };

    let empty = width
        .saturating_sub(full_blocks)
        .saturating_sub(if remainder > 0 { 1 } else { 0 });

    // Pre-allocate capacity
    let capacity = (full_blocks + partial.len() + empty) * 3;
    let mut result = String::with_capacity(capacity);

    for _ in 0..full_blocks {
        result.push('█');
    }
    result.push_str(partial);
    for _ in 0..empty {
        result.push(' ');
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate() {
        assert_eq!(truncate("Hello", 10), "Hello");
        assert_eq!(truncate("Hello World", 8), "Hello W…");
        assert_eq!(truncate("Hi", 1), "…");
        assert_eq!(truncate("", 5), "");
    }

    #[test]
    fn test_truncate_start() {
        assert_eq!(truncate_start("Hello", 10), "Hello");
        assert_eq!(truncate_start("Hello World", 8), "…o World");
        assert_eq!(truncate_start("/home/user/file.txt", 10), "…/file.txt");
    }

    #[test]
    fn test_center() {
        assert_eq!(center("Hi", 6), "  Hi  ");
        assert_eq!(center("Hello", 5), "Hello");
        assert_eq!(center("Hi", 5), " Hi  ");
    }

    #[test]
    fn test_pad_left() {
        assert_eq!(pad_left("42", 5), "   42");
        assert_eq!(pad_left("12345", 5), "12345");
        assert_eq!(pad_left("123456", 5), "123456");
    }

    #[test]
    fn test_pad_right() {
        assert_eq!(pad_right("Hi", 5), "Hi   ");
        assert_eq!(pad_right("Hello", 5), "Hello");
    }

    #[test]
    fn test_wrap_text() {
        let lines = wrap_text("Hello World", 5);
        assert_eq!(lines, vec!["Hello", "World"]);

        let lines = wrap_text("Short", 10);
        assert_eq!(lines, vec!["Short"]);

        let lines = wrap_text("A very long word", 5);
        assert_eq!(lines.len(), 4);
    }

    #[test]
    fn test_wrap_text_empty() {
        let lines = wrap_text("", 10);
        assert!(lines.is_empty());

        let lines = wrap_text("test", 0);
        assert!(lines.is_empty());
    }

    #[test]
    fn test_wrap_text_multiline() {
        let lines = wrap_text("Line1\nLine2", 10);
        assert_eq!(lines, vec!["Line1", "Line2"]);
    }

    #[test]
    fn test_split_fixed_width() {
        let chunks = split_fixed_width("HelloWorld", 3);
        assert_eq!(chunks, vec!["Hel", "loW", "orl", "d"]);
    }

    #[test]
    fn test_repeat_char() {
        assert_eq!(repeat_char('─', 5), "─────");
        assert_eq!(repeat_char('X', 0), "");
    }

    #[test]
    fn test_progress_bar() {
        assert_eq!(progress_bar(0.5, 10), "█████░░░░░");
        assert_eq!(progress_bar(1.0, 5), "█████");
        assert_eq!(progress_bar(0.0, 5), "░░░░░");
    }

    #[test]
    fn test_progress_bar_precise() {
        let bar = progress_bar_precise(0.5, 10);
        assert!(bar.contains('█'));
    }

    #[test]
    fn test_display_width() {
        assert_eq!(display_width("Hello"), 5);
        assert_eq!(display_width(""), 0);
    }

    // =========================================================================
    // Character Index Utility Tests
    // =========================================================================

    #[test]
    fn test_char_to_byte_index_ascii() {
        let s = "hello";
        assert_eq!(char_to_byte_index(s, 0), 0);
        assert_eq!(char_to_byte_index(s, 2), 2);
        assert_eq!(char_to_byte_index(s, 5), 5);
        assert_eq!(char_to_byte_index(s, 10), 5); // Out of bounds
    }

    #[test]
    fn test_char_to_byte_index_unicode() {
        let s = "héllo"; // 'é' is 2 bytes in UTF-8
        assert_eq!(char_to_byte_index(s, 0), 0); // 'h'
        assert_eq!(char_to_byte_index(s, 1), 1); // 'é' starts at byte 1
        assert_eq!(char_to_byte_index(s, 2), 3); // 'l' starts at byte 3
        assert_eq!(char_to_byte_index(s, 3), 4); // second 'l'
        assert_eq!(char_to_byte_index(s, 4), 5); // 'o'
    }

    #[test]
    fn test_char_to_byte_index_emoji() {
        let s = "a🎉b"; // emoji is 4 bytes
        assert_eq!(char_to_byte_index(s, 0), 0); // 'a'
        assert_eq!(char_to_byte_index(s, 1), 1); // emoji starts at byte 1
        assert_eq!(char_to_byte_index(s, 2), 5); // 'b' starts at byte 5
    }

    #[test]
    fn test_byte_to_char_index() {
        let s = "héllo";
        assert_eq!(byte_to_char_index(s, 0), 0);
        assert_eq!(byte_to_char_index(s, 1), 1);
        assert_eq!(byte_to_char_index(s, 3), 2);
        assert_eq!(byte_to_char_index(s, 6), 5);
    }

    #[test]
    fn test_char_count() {
        assert_eq!(char_count("hello"), 5);
        assert_eq!(char_count("héllo"), 5); // 5 chars, 6 bytes
        assert_eq!(char_count("🎉"), 1); // 1 char, 4 bytes
        assert_eq!(char_count(""), 0);
    }

    #[test]
    fn test_char_slice() {
        let s = "héllo world";
        assert_eq!(char_slice(s, 0, 5), "héllo");
        assert_eq!(char_slice(s, 6, 11), "world");
        assert_eq!(char_slice(s, 1, 4), "éll");
        assert_eq!(char_slice(s, 5, 5), ""); // Empty range
        assert_eq!(char_slice(s, 5, 3), ""); // Invalid range
    }

    #[test]
    fn test_insert_at_char() {
        let mut s = String::from("hllo");
        let new_pos = insert_at_char(&mut s, 1, "e");
        assert_eq!(s, "hello");
        assert_eq!(new_pos, 2);

        let mut s = String::from("héllo");
        let new_pos = insert_at_char(&mut s, 2, "XX");
        assert_eq!(s, "héXXllo");
        assert_eq!(new_pos, 4);
    }

    #[test]
    fn test_remove_char_at() {
        let mut s = String::from("hello");
        let removed = remove_char_at(&mut s, 1);
        assert_eq!(s, "hllo");
        assert_eq!(removed, Some('e'));

        let mut s = String::from("héllo");
        let removed = remove_char_at(&mut s, 1);
        assert_eq!(s, "hllo");
        assert_eq!(removed, Some('é'));

        let mut s = String::from("hello");
        let removed = remove_char_at(&mut s, 10);
        assert_eq!(s, "hello");
        assert_eq!(removed, None);
    }

    #[test]
    fn test_remove_char_range() {
        let mut s = String::from("hello world");
        remove_char_range(&mut s, 5, 11);
        assert_eq!(s, "hello");

        let mut s = String::from("héllo");
        remove_char_range(&mut s, 1, 3);
        assert_eq!(s, "hlo");

        let mut s = String::from("hello");
        remove_char_range(&mut s, 3, 3); // Empty range
        assert_eq!(s, "hello");
    }

    // =========================================================================
    // Edge Case Tests - UTF-8 Boundary Safety
    // =========================================================================

    #[test]
    fn test_byte_to_char_index_misaligned() {
        // Test with byte index in the middle of a UTF-8 character
        let s = "héllo"; // é is 2 bytes at positions 1-2
                         // byte_idx=2 is in the middle of 'é', should not panic
        let count = byte_to_char_index(s, 2);
        assert!(count <= 2); // Should be 0, 1, or 2 (safe handling)
    }

    #[test]
    fn test_byte_to_char_index_out_of_bounds() {
        let s = "hello";
        assert_eq!(byte_to_char_index(s, 100), 5); // Returns char count
        assert_eq!(byte_to_char_index(s, 5), 5);
    }

    #[test]
    fn test_byte_to_char_index_empty_string() {
        let s = "";
        assert_eq!(byte_to_char_index(s, 0), 0);
        assert_eq!(byte_to_char_index(s, 10), 0);
    }

    #[test]
    fn test_char_slice_boundary_safety() {
        let s = "héllo world";

        // Safe slices
        assert_eq!(char_slice(s, 0, 5), "héllo");
        assert_eq!(char_slice(s, 6, 11), "world");

        // Edge cases
        assert_eq!(char_slice(s, 0, 0), ""); // Empty range
        assert_eq!(char_slice(s, 5, 3), ""); // Invalid range
        assert_eq!(char_slice(s, 100, 110), ""); // Out of bounds
    }

    #[test]
    fn test_char_slice_unicode() {
        let s = "Hello 世界 World"; // Chinese characters are 3 bytes each

        // Slice including Unicode
        let result = char_slice(s, 6, 8); // Should get the two Chinese chars
        assert_eq!(result, "世界");

        // Empty edge cases
        assert_eq!(char_slice(s, 100, 110), "");
        assert_eq!(char_slice(s, 5, 3), "");
    }

    #[test]
    fn test_char_slice_emoji() {
        let s = "a🎉b🎉c"; // Emojis are 4 bytes each

        assert_eq!(char_slice(s, 1, 2), "🎉");
        assert_eq!(char_slice(s, 0, 5), "a🎉b🎉c");
        assert_eq!(char_slice(s, 0, 0), "");
        assert_eq!(char_slice(s, 10, 20), "");
    }

    #[test]
    fn test_char_to_byte_index_edge_cases() {
        let s = "hello";

        // Out of bounds should return string length
        assert_eq!(char_to_byte_index(s, 100), 5);
        assert_eq!(char_to_byte_index(s, 5), 5);
        assert_eq!(char_to_byte_index(s, 0), 0);
    }

    #[test]
    fn test_char_to_byte_index_empty_string() {
        let s = "";
        assert_eq!(char_to_byte_index(s, 0), 0);
        assert_eq!(char_to_byte_index(s, 10), 0);
    }

    #[test]
    fn test_insert_at_char_edge_cases() {
        let mut s = String::from("hello");

        // Insert at end
        let pos = insert_at_char(&mut s, 5, "!");
        assert_eq!(s, "hello!");
        assert_eq!(pos, 6);

        // Insert at beginning
        let mut s = String::from("hello");
        let pos = insert_at_char(&mut s, 0, ">");
        assert_eq!(s, ">hello");
        assert_eq!(pos, 1);
    }

    #[test]
    fn test_remove_char_at_edge_cases() {
        let mut s = String::from("a");

        // Remove only character
        let removed = remove_char_at(&mut s, 0);
        assert_eq!(s, "");
        assert_eq!(removed, Some('a'));

        // Try to remove from empty string
        let removed = remove_char_at(&mut s, 0);
        assert_eq!(removed, None);

        // Out of bounds
        let mut s = String::from("hello");
        let removed = remove_char_at(&mut s, 100);
        assert_eq!(s, "hello");
        assert_eq!(removed, None);
    }

    #[test]
    fn test_truncate_edge_cases() {
        assert_eq!(truncate("", 5), "");
        assert_eq!(truncate("hi", 0), "…");
        assert_eq!(truncate("a", 1), "a");
    }

    #[test]
    fn test_truncate_start_edge_cases() {
        assert_eq!(truncate_start("", 5), "");
        assert_eq!(truncate_start("hi", 0), "…");
        assert_eq!(truncate_start("a", 1), "a");
    }

    #[test]
    fn test_center_edge_cases() {
        assert_eq!(center("", 5), "     ");
        assert_eq!(center("x", 0), "x");
        assert_eq!(center("hello", 5), "hello");
    }

    #[test]
    fn test_wrap_text_very_long_word() {
        // Word much longer than max_width
        let lines = wrap_text("abcdefghij", 3);
        assert_eq!(lines, vec!["abc", "def", "ghi", "j"]);
    }

    #[test]
    fn test_wrap_text_preserve_newlines() {
        let lines = wrap_text("Line1\n\nLine2", 10);
        assert_eq!(lines, vec!["Line1", "", "Line2"]);
    }

    #[test]
    fn test_split_fixed_width_edge_cases() {
        assert_eq!(split_fixed_width("", 5), vec![""]);
        assert_eq!(split_fixed_width("a", 0), Vec::<String>::new());
        assert_eq!(split_fixed_width("abc", 10), vec!["abc"]);
    }

    #[test]
    fn test_progress_bar_edge_cases() {
        assert_eq!(progress_bar(-1.0, 5), "░░░░░"); // Negative clamped to 0
        assert_eq!(progress_bar(2.0, 5), "█████"); // >1 clamped to 1
        assert_eq!(progress_bar(0.5, 0), ""); // Zero width
    }

    #[test]
    fn test_char_to_byte_index_with_char_edge_cases() {
        let s = "hello";

        // Out of bounds
        let (byte_idx, ch) = char_to_byte_index_with_char(s, 100);
        assert_eq!(byte_idx, 5);
        assert_eq!(ch, None);

        let (byte_idx, ch) = char_to_byte_index_with_char(s, 4);
        assert_eq!(byte_idx, 4);
        assert_eq!(ch, Some('o'));
    }

    #[test]
    fn test_empty_string_operations() {
        let mut s = String::from("");

        // All these should handle empty string safely
        assert_eq!(char_slice(&s, 0, 0), "");
        assert_eq!(char_to_byte_index(&s, 0), 0);
        assert_eq!(byte_to_char_index(&s, 0), 0);
        assert_eq!(char_count(&s), 0);

        let removed = remove_char_at(&mut s, 0);
        assert_eq!(removed, None);

        let pos = insert_at_char(&mut s, 0, "a");
        assert_eq!(s, "a");
        assert_eq!(pos, 1);
    }
}
