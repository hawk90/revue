//! Unicode width utilities
//!
//! Provides accurate display width calculation for Unicode strings,
//! handling CJK characters, emojis, and combining characters.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::utils::{display_width, truncate_to_width};
//!
//! assert_eq!(display_width("hello"), 5);
//! assert_eq!(display_width("안녕"), 4);  // Korean: 2 chars × 2 width
//! assert_eq!(display_width("🎉"), 2);    // Emoji: 2 width
//!
//! let truncated = truncate_to_width("Hello, 世界!", 9);
//! assert_eq!(truncated, "Hello, 世");
//! ```

/// Get the display width of a character
///
/// Returns the number of terminal columns needed to display the character:
/// - 0 for combining characters and zero-width characters
/// - 1 for most ASCII and Latin characters
/// - 2 for CJK characters, emojis, and other wide characters
pub fn char_width(c: char) -> usize {
    let cp = c as u32;

    // Zero-width characters
    if is_zero_width(c) {
        return 0;
    }

    // Control characters
    if cp < 0x20 || (0x7F..0xA0).contains(&cp) {
        return 0;
    }

    // ASCII printable
    if cp < 0x7F {
        return 1;
    }

    // Wide characters (CJK, emojis, etc.)
    if is_wide_char(c) {
        return 2;
    }

    // Default to 1 for other characters
    1
}

/// Check if character is zero-width
fn is_zero_width(c: char) -> bool {
    let cp = c as u32;

    // Combining Diacritical Marks
    if (0x0300..=0x036F).contains(&cp) {
        return true;
    }

    // Combining Diacritical Marks Extended
    if (0x1AB0..=0x1AFF).contains(&cp) {
        return true;
    }

    // Combining Diacritical Marks Supplement
    if (0x1DC0..=0x1DFF).contains(&cp) {
        return true;
    }

    // Combining Diacritical Marks for Symbols
    if (0x20D0..=0x20FF).contains(&cp) {
        return true;
    }

    // Combining Half Marks
    if (0xFE20..=0xFE2F).contains(&cp) {
        return true;
    }

    // Zero Width Space, Zero Width Non-Joiner, Zero Width Joiner
    if cp == 0x200B || cp == 0x200C || cp == 0x200D {
        return true;
    }

    // Variation selectors
    if (0xFE00..=0xFE0F).contains(&cp) || (0xE0100..=0xE01EF).contains(&cp) {
        return true;
    }

    false
}

/// Check if character is wide (takes 2 columns)
fn is_wide_char(c: char) -> bool {
    let cp = c as u32;

    // CJK Unified Ideographs and related
    if (0x4E00..=0x9FFF).contains(&cp) {
        // CJK Unified Ideographs
        return true;
    }
    if (0x3400..=0x4DBF).contains(&cp) {
        // CJK Unified Ideographs Extension A
        return true;
    }
    if (0x20000..=0x2A6DF).contains(&cp) {
        // CJK Unified Ideographs Extension B
        return true;
    }
    if (0x2A700..=0x2B73F).contains(&cp) {
        // CJK Unified Ideographs Extension C
        return true;
    }
    if (0x2B740..=0x2B81F).contains(&cp) {
        // CJK Unified Ideographs Extension D
        return true;
    }

    // CJK Compatibility Ideographs
    if (0xF900..=0xFAFF).contains(&cp) {
        return true;
    }

    // Hangul (Korean)
    if (0xAC00..=0xD7AF).contains(&cp) {
        // Hangul Syllables
        return true;
    }
    if (0x1100..=0x11FF).contains(&cp) {
        // Hangul Jamo
        return true;
    }

    // Japanese
    if (0x3040..=0x309F).contains(&cp) {
        // Hiragana
        return true;
    }
    if (0x30A0..=0x30FF).contains(&cp) {
        // Katakana
        return true;
    }

    // Full-width characters
    if (0xFF00..=0xFFEF).contains(&cp) {
        // Halfwidth forms are not wide
        if (0xFF61..=0xFFDC).contains(&cp) || (0xFFE8..=0xFFEE).contains(&cp) {
            return false;
        }
        return true;
    }

    // Emojis (most common ranges)
    if (0x1F300..=0x1F9FF).contains(&cp) {
        // Miscellaneous Symbols and Pictographs, Emoticons, etc.
        return true;
    }
    if (0x1FA00..=0x1FAFF).contains(&cp) {
        // Chess, Extended-A
        return true;
    }
    if (0x2600..=0x26FF).contains(&cp) {
        // Miscellaneous Symbols
        return true;
    }
    if (0x2700..=0x27BF).contains(&cp) {
        // Dingbats
        return true;
    }

    // Box Drawing and Block Elements (typically wide in some terminals)
    // Actually these are usually width 1, so we don't include them

    false
}

/// Get the display width of a string
///
/// Returns the total number of terminal columns needed to display the string.
///
/// # Example
///
/// ```rust,ignore
/// use revue::utils::display_width;
///
/// assert_eq!(display_width("hello"), 5);
/// assert_eq!(display_width("世界"), 4);
/// assert_eq!(display_width("café"), 4);  // e with combining accent
/// ```
pub fn display_width(s: &str) -> usize {
    s.chars().map(char_width).sum()
}

/// Truncate a string to fit within a given display width
///
/// Returns a string slice that fits within the specified width.
/// If truncation occurs, the result may be shorter than `max_width`
/// to avoid splitting a wide character.
///
/// # Safety
/// Ensures the returned slice is always at a valid UTF-8 boundary.
///
/// # Example
///
/// ```rust,ignore
/// use revue::utils::truncate_to_width;
///
/// assert_eq!(truncate_to_width("Hello, World!", 5), "Hello");
/// assert_eq!(truncate_to_width("안녕하세요", 5), "안녕");  // 4 width, can't fit 6
/// ```
pub fn truncate_to_width(s: &str, max_width: usize) -> &str {
    let mut width = 0;
    let mut end_idx = 0;

    for (i, c) in s.char_indices() {
        let cw = char_width(c);
        if width + cw > max_width {
            break;
        }
        width += cw;
        end_idx = i + c.len_utf8();
    }

    // Ensure end_idx is at a valid UTF-8 boundary
    if end_idx == 0 {
        return "";
    }
    if end_idx >= s.len() {
        return s;
    }
    if !s.is_char_boundary(end_idx) {
        // Find the previous valid boundary
        for (i, _) in s.char_indices() {
            if i >= end_idx {
                break;
            }
            end_idx = i;
        }
    }

    &s[..end_idx]
}

/// Truncate a string and add ellipsis if needed
///
/// If the string is truncated, appends "…" (or custom suffix).
/// The result will fit within `max_width` including the ellipsis.
///
/// # Example
///
/// ```rust,ignore
/// use revue::utils::truncate_with_ellipsis;
///
/// assert_eq!(truncate_with_ellipsis("Hello, World!", 8), "Hello, …");
/// assert_eq!(truncate_with_ellipsis("Hi", 8), "Hi");
/// ```
pub fn truncate_with_ellipsis(s: &str, max_width: usize) -> String {
    truncate_with_suffix(s, max_width, "…")
}

/// Truncate a string and add a custom suffix if needed
pub fn truncate_with_suffix(s: &str, max_width: usize, suffix: &str) -> String {
    let width = display_width(s);
    if width <= max_width {
        return s.to_string();
    }

    let suffix_width = display_width(suffix);
    if max_width <= suffix_width {
        return truncate_to_width(suffix, max_width).to_string();
    }

    let content_width = max_width - suffix_width;
    let truncated = truncate_to_width(s, content_width);
    format!("{}{}", truncated, suffix)
}

/// Pad a string to a specific display width
///
/// Adds spaces to reach the target width. If the string is already
/// wider than the target, returns it unchanged.
pub fn pad_to_width(s: &str, target_width: usize) -> String {
    let width = display_width(s);
    if width >= target_width {
        s.to_string()
    } else {
        format!("{}{}", s, " ".repeat(target_width - width))
    }
}

/// Center a string within a specific display width
pub fn center_to_width(s: &str, target_width: usize) -> String {
    let width = display_width(s);
    if width >= target_width {
        return s.to_string();
    }

    let padding = target_width - width;
    let left = padding / 2;
    let right = padding - left;
    format!("{}{}{}", " ".repeat(left), s, " ".repeat(right))
}

/// Right-align a string within a specific display width
pub fn right_align_to_width(s: &str, target_width: usize) -> String {
    let width = display_width(s);
    if width >= target_width {
        return s.to_string();
    }

    format!("{}{}", " ".repeat(target_width - width), s)
}

/// Split a string at a specific display width position
///
/// Returns (left, right) where left has the specified width.
pub fn split_at_width(s: &str, width: usize) -> (&str, &str) {
    let left = truncate_to_width(s, width);
    let right = &s[left.len()..];
    (left, right)
}

/// Wrap text to a specific display width
///
/// Wraps text at word boundaries when possible.
pub fn wrap_to_width(s: &str, max_width: usize) -> Vec<String> {
    if max_width == 0 {
        return vec![];
    }

    let mut lines = Vec::new();
    let mut current_line = String::new();
    let mut current_width = 0;

    for word in s.split_whitespace() {
        let word_width = display_width(word);

        if current_width == 0 {
            // First word on line
            if word_width <= max_width {
                current_line = word.to_string();
                current_width = word_width;
            } else {
                // Word is too long, need to break it
                let mut remaining = word;
                while !remaining.is_empty() {
                    let (chunk, rest) = split_at_width(remaining, max_width);
                    if chunk.is_empty() {
                        // Can't fit even a single character, skip to avoid infinite loop
                        break;
                    }
                    lines.push(chunk.to_string());
                    if rest.is_empty() {
                        break;
                    }
                    remaining = rest;
                }
            }
        } else if current_width + 1 + word_width <= max_width {
            // Word fits on current line
            current_line.push(' ');
            current_line.push_str(word);
            current_width += 1 + word_width;
        } else {
            // Need to wrap
            lines.push(current_line);
            if word_width <= max_width {
                current_line = word.to_string();
                current_width = word_width;
            } else {
                // Word is too long, need to break it
                current_line = String::new();
                current_width = 0;
                let mut remaining = word;
                while !remaining.is_empty() {
                    let (chunk, rest) = split_at_width(remaining, max_width);
                    if chunk.is_empty() {
                        // Can't fit even a single character (e.g., wide char in narrow width)
                        // Skip this word to avoid infinite loop
                        break;
                    }
                    if rest.is_empty() {
                        current_line = chunk.to_string();
                        current_width = display_width(chunk);
                    } else {
                        lines.push(chunk.to_string());
                    }
                    remaining = rest;
                }
            }
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii_width() {
        assert_eq!(display_width("hello"), 5);
        assert_eq!(display_width("Hello, World!"), 13);
    }

    #[test]
    fn test_cjk_width() {
        assert_eq!(display_width("世界"), 4);
        assert_eq!(display_width("안녕"), 4);
        assert_eq!(display_width("こんにちは"), 10);
    }

    #[test]
    fn test_mixed_width() {
        assert_eq!(display_width("Hello世界"), 9); // 5 + 4
        assert_eq!(display_width("a한b글c"), 7); // 1 + 2 + 1 + 2 + 1
    }

    #[test]
    fn test_emoji_width() {
        assert_eq!(display_width("🎉"), 2);
        assert_eq!(display_width("👍"), 2);
        assert_eq!(display_width("Hello 🎉"), 8); // 6 + 2
    }

    #[test]
    fn test_truncate_ascii() {
        assert_eq!(truncate_to_width("Hello, World!", 5), "Hello");
        assert_eq!(truncate_to_width("Hello", 10), "Hello");
    }

    #[test]
    fn test_truncate_cjk() {
        // "안녕하세요" = 10 width (5 chars × 2)
        assert_eq!(truncate_to_width("안녕하세요", 4), "안녕");
        assert_eq!(truncate_to_width("안녕하세요", 5), "안녕"); // Can't fit half of 하
        assert_eq!(truncate_to_width("안녕하세요", 6), "안녕하");
    }

    #[test]
    fn test_truncate_mixed() {
        // "Hello世界" = 9 width
        assert_eq!(truncate_to_width("Hello世界", 7), "Hello世");
        assert_eq!(truncate_to_width("Hello世界", 6), "Hello"); // Can't fit 世 (width 2)
    }

    #[test]
    fn test_truncate_with_ellipsis() {
        assert_eq!(truncate_with_ellipsis("Hello, World!", 8), "Hello, …");
        assert_eq!(truncate_with_ellipsis("Hi", 8), "Hi");
        assert_eq!(truncate_with_ellipsis("안녕하세요", 5), "안녕…");
    }

    #[test]
    fn test_pad_to_width() {
        assert_eq!(pad_to_width("Hi", 5), "Hi   ");
        assert_eq!(pad_to_width("Hello", 3), "Hello");
        assert_eq!(pad_to_width("안녕", 6), "안녕  ");
    }

    #[test]
    fn test_center_to_width() {
        assert_eq!(center_to_width("Hi", 6), "  Hi  ");
        assert_eq!(center_to_width("Hi", 5), " Hi  ");
    }

    #[test]
    fn test_right_align_to_width() {
        assert_eq!(right_align_to_width("Hi", 5), "   Hi");
        assert_eq!(right_align_to_width("안녕", 6), "  안녕");
    }

    #[test]
    fn test_wrap_to_width() {
        let lines = wrap_to_width("Hello World", 6);
        assert_eq!(lines, vec!["Hello", "World"]);

        let lines = wrap_to_width("안녕 세계", 6);
        assert_eq!(lines, vec!["안녕", "세계"]);
    }

    #[test]
    fn test_char_width() {
        assert_eq!(char_width('a'), 1);
        assert_eq!(char_width('가'), 2);
        assert_eq!(char_width('あ'), 2);
        assert_eq!(char_width('漢'), 2);
    }

    #[test]
    fn test_zero_width_chars() {
        // Combining character shouldn't add width
        assert_eq!(char_width('\u{0301}'), 0); // Combining acute accent
    }

    #[test]
    fn test_fullwidth_chars() {
        // Full-width ASCII characters
        assert_eq!(display_width("ＡＢＣ"), 6); // Full-width ABC
        assert_eq!(char_width('Ａ'), 2);
    }

    // =========================================================================
    // Edge Case Tests - UTF-8 Boundary Safety
    // =========================================================================

    #[test]
    fn test_truncate_empty_string() {
        assert_eq!(truncate_to_width("", 10), "");
        assert_eq!(truncate_to_width("", 0), "");
    }

    #[test]
    fn test_truncate_zero_width() {
        assert_eq!(truncate_to_width("hello", 0), "");
        assert_eq!(truncate_to_width("안녕", 0), "");
    }

    #[test]
    fn test_truncate_single_wide_char() {
        // Can't fit even one wide character
        assert_eq!(truncate_to_width("안", 1), "");
        // Can fit exactly one wide character
        assert_eq!(truncate_to_width("안", 2), "안");
    }

    #[test]
    fn test_truncate_wider_than_available() {
        assert_eq!(truncate_to_width("hi", 100), "hi");
        assert_eq!(truncate_to_width("안녕", 100), "안녕");
    }

    #[test]
    fn test_split_at_width_edge_cases() {
        // Empty string
        let (left, right) = split_at_width("", 5);
        assert_eq!(left, "");
        assert_eq!(right, "");

        // Zero width
        let (left, right) = split_at_width("hello", 0);
        assert_eq!(left, "");
        assert_eq!(right, "hello");

        // Width larger than string
        let (left, right) = split_at_width("hi", 100);
        assert_eq!(left, "hi");
        assert_eq!(right, "");
    }

    #[test]
    fn test_split_at_width_wide_chars() {
        let (left, _right) = split_at_width("안녕하세요", 4);
        assert_eq!(left, "안녕");
        assert_eq!(display_width(left), 4);
    }

    #[test]
    fn test_truncate_with_suffix_edge_cases() {
        // Empty string
        assert_eq!(truncate_with_suffix("", 5, "…"), "");

        // Zero max width - can't fit anything, returns empty
        assert_eq!(truncate_with_suffix("hello", 0, "…"), "");

        // String already fits
        assert_eq!(truncate_with_suffix("hi", 5, "…"), "hi");

        // Max width exactly fits suffix
        assert_eq!(truncate_with_suffix("hello world", 1, "…"), "…");
    }

    #[test]
    fn test_pad_to_width_edge_cases() {
        // Empty string
        assert_eq!(pad_to_width("", 5), "     ");

        // Zero width
        assert_eq!(pad_to_width("hello", 0), "hello");

        // Already at target width
        assert_eq!(pad_to_width("abc", 3), "abc");
    }

    #[test]
    fn test_center_to_width_edge_cases() {
        // Empty string
        assert_eq!(center_to_width("", 5), "     ");

        // Zero width
        assert_eq!(center_to_width("hello", 0), "hello");

        // Odd padding
        assert_eq!(center_to_width("x", 4), " x  ");
    }

    #[test]
    fn test_right_align_to_width_edge_cases() {
        // Empty string
        assert_eq!(right_align_to_width("", 5), "     ");

        // Zero width
        assert_eq!(right_align_to_width("hello", 0), "hello");
    }

    #[test]
    fn test_wrap_to_width_edge_cases() {
        // Empty string
        assert!(wrap_to_width("", 10).is_empty());

        // Zero width
        assert!(wrap_to_width("hello", 0).is_empty());

        // Single character wider than max
        let lines = wrap_to_width("안", 1);
        assert!(lines.is_empty()); // Can't fit
    }

    #[test]
    fn test_display_width_edge_cases() {
        // Empty string
        assert_eq!(display_width(""), 0);

        // Control characters
        assert_eq!(display_width("\n\t"), 0);

        // Zero-width characters
        assert_eq!(display_width("a\u{0301}"), 1); // a + combining acute = 1 width
    }

    #[test]
    fn test_char_width_control_chars() {
        assert_eq!(char_width('\0'), 0);
        assert_eq!(char_width('\n'), 0);
        assert_eq!(char_width('\t'), 0);
        assert_eq!(char_width('\r'), 0);
    }

    #[test]
    fn test_char_width_ascii_printable() {
        for c in '!'..='~' {
            assert_eq!(char_width(c), 1, "Character '{}' should have width 1", c);
        }
    }

    #[test]
    fn test_truncate_preserves_valid_utf8() {
        // Test that truncated strings are always valid UTF-8
        let s = "Hello世界안녕🎉";

        for width in 0..=20 {
            let truncated = truncate_to_width(s, width);
            // Verify it's valid UTF-8 by trying to count chars
            let _ = truncated.chars().count();
        }
    }

    #[test]
    fn test_split_preserves_valid_utf8() {
        // Test that both parts of split are valid UTF-8
        let s = "Hello世界안녕🎉";

        for width in 0..=20 {
            let (left, right) = split_at_width(s, width);
            // Verify both are valid UTF-8
            let _ = left.chars().count();
            let _ = right.chars().count();
        }
    }
}
