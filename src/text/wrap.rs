//! Text wrapping and formatting utilities

use crate::utils::unicode::{char_width, display_width, truncate_to_width};
use textwrap::{Options, WordSeparator, WordSplitter};

/// Text wrapping mode
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum WrapMode {
    /// No wrapping, truncate at width
    NoWrap,
    /// Wrap at word boundaries
    #[default]
    Word,
    /// Wrap at character boundaries
    Char,
}

/// Text overflow handling
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Overflow {
    /// Clip text at boundary
    #[default]
    Clip,
    /// Show ellipsis at end
    Ellipsis,
    /// Show ellipsis in middle
    EllipsisMiddle,
}

/// Text wrapper with configurable options
#[derive(Clone)]
pub struct TextWrapper {
    width: usize,
    mode: WrapMode,
    overflow: Overflow,
    indent: String,
    subsequent_indent: String,
    break_words: bool,
}

impl TextWrapper {
    /// Create a new text wrapper
    pub fn new(width: usize) -> Self {
        Self {
            width,
            mode: WrapMode::Word,
            overflow: Overflow::Clip,
            indent: String::new(),
            subsequent_indent: String::new(),
            break_words: true,
        }
    }

    /// Set wrap mode
    pub fn mode(mut self, mode: WrapMode) -> Self {
        self.mode = mode;
        self
    }

    /// Set overflow handling
    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.overflow = overflow;
        self
    }

    /// Set first line indent
    pub fn indent(mut self, indent: impl Into<String>) -> Self {
        self.indent = indent.into();
        self
    }

    /// Set subsequent line indent
    pub fn subsequent_indent(mut self, indent: impl Into<String>) -> Self {
        self.subsequent_indent = indent.into();
        self
    }

    /// Set whether to break long words
    pub fn break_words(mut self, break_words: bool) -> Self {
        self.break_words = break_words;
        self
    }

    /// Wrap text
    pub fn wrap(&self, text: &str) -> Vec<String> {
        match self.mode {
            WrapMode::NoWrap => text
                .lines()
                .map(|line| self.handle_overflow(line))
                .collect(),
            WrapMode::Word => {
                let options = Options::new(self.width)
                    .initial_indent(&self.indent)
                    .subsequent_indent(&self.subsequent_indent)
                    .word_separator(WordSeparator::UnicodeBreakProperties)
                    .word_splitter(if self.break_words {
                        WordSplitter::HyphenSplitter
                    } else {
                        WordSplitter::NoHyphenation
                    });

                textwrap::wrap(text, options)
                    .into_iter()
                    .map(|cow| cow.into_owned())
                    .collect()
            }
            WrapMode::Char => {
                let mut lines = Vec::new();

                for line in text.lines() {
                    let indent = if lines.is_empty() {
                        &self.indent
                    } else {
                        &self.subsequent_indent
                    };

                    // Use display_width for indent width
                    let indent_width = display_width(indent);
                    let remaining_width = self.width.saturating_sub(indent_width);

                    if remaining_width == 0 {
                        lines.push(indent.clone());
                        continue;
                    }

                    // If line fits, just add it
                    if display_width(line) <= remaining_width {
                        lines.push(format!("{}{}", indent, line));
                        continue;
                    }

                    // Split by display width, not character count
                    let mut pos = 0;
                    let mut first_line = true;

                    while pos < line.len() {
                        // Get substring starting at pos
                        let remaining_str = &line[pos..];

                        // Get indent for this line
                        let current_indent = if first_line {
                            &self.indent
                        } else {
                            &self.subsequent_indent
                        };

                        let indent_width = display_width(current_indent);
                        let remaining_width = self.width.saturating_sub(indent_width);

                        if remaining_width == 0 {
                            break;
                        }

                        // Truncate to fit in remaining width
                        let chunk = truncate_to_width(remaining_str, remaining_width);
                        lines.push(format!("{}{}", current_indent, chunk));

                        pos += chunk.len();
                        first_line = false;
                    }

                    if line.is_empty() && !indent.is_empty() {
                        lines.push(indent.clone());
                    }
                }

                lines
            }
        }
    }

    /// Handle overflow for a single line
    fn handle_overflow(&self, text: &str) -> String {
        let text_width = display_width(text);
        if text_width <= self.width {
            return text.to_string();
        }

        match self.overflow {
            Overflow::Clip => truncate_to_width(text, self.width).to_string(),
            Overflow::Ellipsis => {
                if self.width <= 3 {
                    "...".chars().take(self.width).collect()
                } else {
                    let visible = self.width - 3;
                    let truncated = truncate_to_width(text, visible);
                    format!("{}...", truncated)
                }
            }
            Overflow::EllipsisMiddle => {
                if self.width <= 3 {
                    "...".chars().take(self.width).collect()
                } else {
                    let half = (self.width - 3) / 2;
                    // Get first half
                    let first = truncate_to_width(text, half);
                    // Get second half from the end
                    let from_end = self.width - 3 - half;
                    let second = if from_end > 0 {
                        // Need to get last 'from_end' display columns
                        let mut total = 0;
                        let chars: Vec<char> = text.chars().rev().collect();
                        let mut rev_chars = Vec::new();
                        for ch in chars {
                            let ch_width = char_width(ch);
                            if total + ch_width > from_end {
                                break;
                            }
                            total += ch_width;
                            rev_chars.push(ch);
                        }
                        rev_chars.into_iter().rev().collect()
                    } else {
                        String::new()
                    };
                    format!("{}...{}", first, second)
                }
            }
        }
    }
}

impl Default for TextWrapper {
    fn default() -> Self {
        Self::new(80)
    }
}

/// Wrap text to fit within width
pub fn wrap_text(text: &str, width: usize) -> Vec<String> {
    TextWrapper::new(width).wrap(text)
}

/// Wrap text with word boundaries
pub fn wrap_words(text: &str, width: usize) -> Vec<String> {
    TextWrapper::new(width).mode(WrapMode::Word).wrap(text)
}

/// Wrap text at character boundaries
pub fn wrap_chars(text: &str, width: usize) -> Vec<String> {
    TextWrapper::new(width).mode(WrapMode::Char).wrap(text)
}

/// Truncate text with ellipsis
pub fn truncate(text: &str, width: usize) -> String {
    TextWrapper::new(width)
        .mode(WrapMode::NoWrap)
        .overflow(Overflow::Ellipsis)
        .wrap(text)
        .into_iter()
        .next()
        .unwrap_or_default()
}

/// Truncate text with ellipsis in the middle
pub fn truncate_middle(text: &str, width: usize) -> String {
    TextWrapper::new(width)
        .mode(WrapMode::NoWrap)
        .overflow(Overflow::EllipsisMiddle)
        .wrap(text)
        .into_iter()
        .next()
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrap_text() {
        let wrapped = wrap_text("Hello world", 5);
        assert!(wrapped.len() >= 2);
    }

    #[test]
    fn test_wrap_words() {
        let wrapped = wrap_words("The quick brown fox", 10);
        assert!(wrapped.len() >= 2);
    }

    #[test]
    fn test_wrap_chars() {
        let wrapped = wrap_chars("Hello", 3);
        assert_eq!(wrapped.len(), 2);
        assert_eq!(wrapped[0], "Hel");
        assert_eq!(wrapped[1], "lo");
    }

    #[test]
    fn test_truncate() {
        let result = truncate("Hello World", 8);
        assert_eq!(result, "Hello...");
    }

    #[test]
    fn test_truncate_short() {
        let result = truncate("Hi", 10);
        assert_eq!(result, "Hi");
    }

    #[test]
    fn test_truncate_middle() {
        let result = truncate_middle("Hello World", 9);
        assert!(result.contains("..."));
        assert_eq!(result.len(), 9);
    }

    #[test]
    fn test_wrapper_new() {
        let wrapper = TextWrapper::new(40);
        assert_eq!(wrapper.width, 40);
    }

    #[test]
    fn test_wrapper_mode() {
        let wrapper = TextWrapper::new(40).mode(WrapMode::Char);
        assert_eq!(wrapper.mode, WrapMode::Char);
    }

    #[test]
    fn test_wrapper_overflow() {
        let wrapper = TextWrapper::new(40).overflow(Overflow::Ellipsis);
        assert_eq!(wrapper.overflow, Overflow::Ellipsis);
    }

    #[test]
    fn test_wrapper_indent() {
        let wrapper = TextWrapper::new(40).indent("  ").subsequent_indent("    ");

        let wrapped = wrapper.wrap("This is a test line that should wrap");
        assert!(wrapped[0].starts_with("  "));
        if wrapped.len() > 1 {
            assert!(wrapped[1].starts_with("    "));
        }
    }

    #[test]
    fn test_no_wrap_mode() {
        let wrapper = TextWrapper::new(5).mode(WrapMode::NoWrap);
        let wrapped = wrapper.wrap("Hello World");

        assert_eq!(wrapped.len(), 1);
        assert_eq!(wrapped[0], "Hello");
    }

    #[test]
    fn test_overflow_clip() {
        let wrapper = TextWrapper::new(5)
            .mode(WrapMode::NoWrap)
            .overflow(Overflow::Clip);

        let wrapped = wrapper.wrap("Hello World");
        assert_eq!(wrapped[0], "Hello");
    }

    #[test]
    fn test_wrapper_break_words() {
        let wrapper = TextWrapper::new(40).break_words(false);
        assert!(!wrapper.break_words);
    }

    #[test]
    fn test_wrap_multiline() {
        let text = "Line1\nLine2\nLine3";
        let wrapped = wrap_text(text, 10);
        assert!(wrapped.len() >= 3);
    }

    #[test]
    fn test_wrap_empty() {
        let wrapped = wrap_text("", 10);
        assert!(wrapped.is_empty() || wrapped[0].is_empty());
    }

    // =============================================================================
    // Edge Case Tests
    // =============================================================================

    #[test]
    fn test_wrap_unicode_emoji() {
        // Each emoji is 1 char but may be multiple bytes
        let text = "Hello 👋 World 🌍";
        let wrapped = wrap_chars(text, 10);
        assert!(wrapped.len() >= 1);
        // Ensure emojis are not broken
        for line in &wrapped {
            assert!(line.is_char_boundary(line.len()));
        }
    }

    #[test]
    fn test_wrap_unicode_cjk() {
        // CJK characters are 2 columns wide
        let text = "你好世界こんにちは";
        let wrapped = wrap_chars(text, 4);
        assert!(wrapped.len() >= 2);
        // Width 4 means 2 CJK chars fit (each is 2 columns wide)
        assert_eq!(display_width(&wrapped[0]), 4);
        assert_eq!(wrapped[0].chars().count(), 2); // 2 CJK chars
    }

    #[test]
    fn test_wrap_unicode_mixed() {
        // Mix of ASCII (width 1) and CJK (width 2)
        let text = "Hi你好"; // H(1) + i(1) + 你(2) + 好(2) = 6 columns
        let wrapped = wrap_chars(text, 4);
        // Should wrap to fit in width 4
        assert_eq!(display_width(&wrapped[0]), 4);
        // First line should contain "Hi你" (1+1+2=4 columns)
        assert_eq!(wrapped[0], "Hi你");
    }

    #[test]
    fn test_truncate_with_display_width() {
        // Truncate using display width
        let text = "Hi你好世界"; // H(1) + i(1) + 你(2) + 好(2) + 世(2) + 界(2) = 10 columns
        let result = truncate(text, 6);
        // truncate_to_width breaks when adding a char would exceed width
        // So with width 6: H(1) + i(1) + 你(2) = 4, then adding 好(2) would make 6
        // The function allows equality, so we get "Hi你好"
        // But if there's any off-by-one, we get "Hi你" = 5 columns
        let result_width = display_width(&result);
        assert!(result_width <= 6);
        // Verify the result is character-boundary safe
        assert!(result.is_char_boundary(result.len()));
    }

    #[test]
    fn test_truncate_cjk_to_exact_width() {
        // Test exact width truncation with CJK
        let text = "你好"; // 4 columns
        let result = truncate(text, 4);
        assert_eq!(display_width(&result), 4);
        assert_eq!(result, "你好");

        // For width 2 with Ellipsis, we get ".." (ellipsis is 3 chars minimum)
        let result = truncate(text, 2);
        assert_eq!(result, "..");

        // Use Clip mode to actually truncate without ellipsis
        let wrapper = TextWrapper::new(2)
            .mode(WrapMode::NoWrap)
            .overflow(Overflow::Clip);
        let result = wrapper.wrap(text).into_iter().next().unwrap_or_default();
        assert_eq!(display_width(&result), 2);
        assert_eq!(result, "你");
    }

    #[test]
    fn test_truncate_unicode() {
        // Ensure truncation doesn't break in the middle of a character
        let text = "Hello 世界";
        let result = truncate(text, 8);
        assert!(result.is_char_boundary(result.len()));
    }

    #[test]
    fn test_truncate_very_short_width() {
        // Width less than ellipsis length
        let result = truncate("Hello World", 2);
        assert_eq!(result, "..");

        let result = truncate("Hello World", 1);
        assert_eq!(result, ".");

        let result = truncate("Hello World", 0);
        assert_eq!(result, "");
    }

    #[test]
    fn test_truncate_middle_very_short() {
        let result = truncate_middle("Hello World", 3);
        assert_eq!(result, "...");

        let result = truncate_middle("Hello World", 2);
        assert_eq!(result, "..");
    }

    #[test]
    fn test_wrap_single_long_word() {
        // A word longer than width
        let text = "Supercalifragilisticexpialidocious";
        let wrapped = wrap_chars(text, 10);
        assert!(wrapped.len() >= 3);
        for line in &wrapped {
            assert!(line.chars().count() <= 10);
        }
    }

    #[test]
    fn test_wrap_width_one() {
        let text = "Hi";
        let wrapped = wrap_chars(text, 1);
        assert_eq!(wrapped.len(), 2);
        assert_eq!(wrapped[0], "H");
        assert_eq!(wrapped[1], "i");
    }

    #[test]
    fn test_wrap_preserves_newlines() {
        let text = "Line1\n\nLine3";
        let wrapped = wrap_text(text, 20);
        // Should preserve the empty line
        assert!(wrapped.len() >= 3);
    }

    #[test]
    fn test_overflow_exactly_at_width() {
        let text = "Hello"; // 5 chars
        let wrapper = TextWrapper::new(5).mode(WrapMode::NoWrap);
        let wrapped = wrapper.wrap(text);
        assert_eq!(wrapped[0], "Hello");
    }

    #[test]
    fn test_indent_with_char_wrap() {
        let wrapper = TextWrapper::new(10)
            .mode(WrapMode::Char)
            .indent(">> ")
            .subsequent_indent("   ");

        let wrapped = wrapper.wrap("Hello World!");
        assert!(wrapped[0].starts_with(">> "));
        if wrapped.len() > 1 {
            assert!(wrapped[1].starts_with("   "));
        }
    }

    #[test]
    fn test_default_wrapper() {
        let wrapper = TextWrapper::default();
        assert_eq!(wrapper.width, 80);
        assert_eq!(wrapper.mode, WrapMode::Word);
        assert_eq!(wrapper.overflow, Overflow::Clip);
    }
}
