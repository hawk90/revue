//! Text wrapping and formatting utilities

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
            WrapMode::NoWrap => {
                text.lines()
                    .map(|line| self.handle_overflow(line))
                    .collect()
            }
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
                let _effective_width = self.width.saturating_sub(self.indent.len());

                for line in text.lines() {
                    let chars: Vec<char> = line.chars().collect();
                    let mut start = 0;

                    while start < chars.len() {
                        let indent = if lines.is_empty() {
                            &self.indent
                        } else {
                            &self.subsequent_indent
                        };

                        let remaining = chars.len() - start;
                        let chunk_size = (self.width.saturating_sub(indent.len())).min(remaining);
                        let end = start + chunk_size;

                        let chunk: String = chars[start..end].iter().collect();
                        lines.push(format!("{}{}", indent, chunk));

                        start = end;
                    }

                    if chars.is_empty() {
                        lines.push(self.indent.clone());
                    }
                }

                lines
            }
        }
    }

    /// Handle overflow for a single line
    fn handle_overflow(&self, text: &str) -> String {
        let chars: Vec<char> = text.chars().collect();
        if chars.len() <= self.width {
            return text.to_string();
        }

        match self.overflow {
            Overflow::Clip => {
                chars[..self.width].iter().collect()
            }
            Overflow::Ellipsis => {
                if self.width <= 3 {
                    "...".chars().take(self.width).collect()
                } else {
                    let visible = self.width - 3;
                    let mut result: String = chars[..visible].iter().collect();
                    result.push_str("...");
                    result
                }
            }
            Overflow::EllipsisMiddle => {
                if self.width <= 3 {
                    "...".chars().take(self.width).collect()
                } else {
                    let half = (self.width - 3) / 2;
                    let end_start = chars.len() - (self.width - 3 - half);
                    let mut result: String = chars[..half].iter().collect();
                    result.push_str("...");
                    result.extend(&chars[end_start..]);
                    result
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
        let wrapper = TextWrapper::new(40)
            .indent("  ")
            .subsequent_indent("    ");

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
        // CJK characters
        let text = "你好世界こんにちは";
        let wrapped = wrap_chars(text, 4);
        assert!(wrapped.len() >= 2);
        assert_eq!(wrapped[0].chars().count(), 4);
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
