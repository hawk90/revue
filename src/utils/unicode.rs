//! Unicode width utilities
//!
//! Provides accurate display width calculation for Unicode strings,
//! handling CJK characters, emojis, and combining characters.
//!
//! # Example
//!
//! ```
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
/// Returns the number of terminal columns needed to display the character,
/// from Unicode's East Asian Width as the `unicode-width` crate gives it -
/// the width terminals use:
/// - 0 for combining marks, zero-width characters, the conjoining Hangul
///   vowels and finals, and control characters
/// - 2 for wide characters: CJK, Hangul syllables, fullwidth forms and
///   emoji shown as emoji by default (😀, ✅, ⚡)
/// - 1 for everything else, including the symbols in Miscellaneous Symbols
///   and Dingbats that are drawn as text (★ ✓ ☐ ➜)
///
/// This is the one width revue draws, measures and lays out text with
/// (#1026). It counts one character at a time: a sequence the terminal
/// joins into one glyph (a flag, an emoji with a skin tone or a ZWJ, a
/// character with VS16) counts as the sum of its characters.
pub fn char_width(c: char) -> usize {
    match c {
        // unicode-width measures these as they are drawn in running text
        // (two Khmer signs spelled out, a visible joiner); terminals
        // (wcwidth) give a Lo or Po character one column and a
        // nonspacing mark none (#1269)
        '\u{17A4}' | '\u{17D8}' => 1,
        '\u{2D7F}' => 0,
        // `None` is a control character, which takes no cell. A cell holds
        // at most two columns.
        _ => unicode_width::UnicodeWidthChar::width(c)
            .unwrap_or(0)
            .min(2),
    }
}

/// The tab width of text in which a tab takes no column: labels and titles,
/// and one-line fields, whose text never holds a tab (`EditState` turns it
/// into a space)
pub(crate) const NO_TABS: usize = 0;

/// Columns `ch` takes when it starts at column `col`: a tab reaches the next
/// multiple of `tab` (no column when `tab` is [`NO_TABS`]), any other char
/// its [`char_width`]. The one tab rule the editors draw, measure and move
/// by (#1378).
pub(crate) fn glyph_width(ch: char, col: usize, tab: usize) -> usize {
    match (ch, tab) {
        ('\t', NO_TABS) => 0,
        ('\t', tab) => tab - col % tab,
        _ => char_width(ch),
    }
}

/// Get the display width of a string
///
/// Returns the total number of terminal columns needed to display the string.
///
/// # Example
///
/// ```
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
/// ```
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
/// ```
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
/// The rows [`crate::text::wrap_words`] gives (#1474): broken at spaces,
/// after hyphens and between CJK characters, a word wider than the width
/// broken across rows, each `\n` starting a new row. Spaces inside a row
/// are kept, and nothing is dropped but the whitespace at a break. A
/// `max_width` of 0 is taken as 1, so no text is lost.
pub fn wrap_to_width(s: &str, max_width: usize) -> Vec<String> {
    crate::text::wrap_words(s, max_width)
}
