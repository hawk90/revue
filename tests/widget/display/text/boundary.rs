//! Boundary condition tests for text operations
//!
//! Tests edge cases for text handling including:
//! - Empty strings
//! - Unicode characters
//! - Very long strings
//! - Special characters

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::{RenderContext, View};
use revue::widget::Text;

/// Every cell of `buffer` is still a blank
fn blank(buffer: &Buffer) -> bool {
    (0..buffer.height())
        .all(|y| (0..buffer.width()).all(|x| buffer.get(x, y).unwrap().symbol == ' '))
}

/// Row `y` of `buffer` as text
fn row(buffer: &Buffer, y: u16) -> String {
    (0..buffer.width())
        .map(|x| buffer.get(x, y).unwrap().symbol)
        .collect()
}

/// Test empty string handling
mod empty_strings {
    use super::*;

    #[test]
    fn test_text_widget_with_empty_string() {
        let text = Text::new("");
        assert_eq!(text.content(), "");
    }

    #[test]
    fn test_render_empty_string() {
        let text = Text::new("");
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        text.render(&mut ctx);
        assert!(blank(&buffer));
    }
}

/// Test single character strings
mod single_character {
    use super::*;

    #[test]
    fn test_text_widget_single_char() {
        let text = Text::new("A");
        assert_eq!(text.content(), "A");
    }

    #[test]
    fn test_text_widget_single_space() {
        let text = Text::new(" ");
        assert_eq!(text.content(), " ");
    }

    #[test]
    fn test_render_single_char() {
        let text = Text::new("X");
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        text.render(&mut ctx);

        // Should render the character
        let cell = buffer.get(0, 0);
        assert!(cell.is_some());
        assert_eq!(cell.unwrap().symbol, 'X');
    }

    #[test]
    fn test_render_single_newline() {
        let text = Text::new("\n");
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        text.render(&mut ctx);
        // The newline only moves to the next row; nothing is drawn
        assert!(blank(&buffer));
    }
}

/// Test Unicode characters
mod unicode_characters {
    use super::*;

    #[test]
    fn test_text_with_emoji() {
        let text = Text::new("Hello 👋");
        assert_eq!(text.content(), "Hello 👋");
    }

    #[test]
    fn test_text_with_korean() {
        let text = Text::new("안녕하세요");
        assert_eq!(text.content(), "안녕하세요");
    }

    #[test]
    fn test_text_with_japanese() {
        let text = Text::new("こんにちは");
        assert_eq!(text.content(), "こんにちは");
    }

    #[test]
    fn test_text_with_chinese() {
        let text = Text::new("你好");
        assert_eq!(text.content(), "你好");
    }

    #[test]
    fn test_text_with_arabic() {
        let text = Text::new("مرحبا");
        assert_eq!(text.content(), "مرحبا");
    }

    #[test]
    fn test_text_with_cyrillic() {
        let text = Text::new("Привет");
        assert_eq!(text.content(), "Привет");
    }

    #[test]
    fn test_text_with_mixed_scripts() {
        let text = Text::new("Hello 안녕 こんにちは");
        assert_eq!(text.content(), "Hello 안녕 こんにちは");
    }

    #[test]
    fn test_render_unicode_text() {
        let text = Text::new("안녕");
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        text.render(&mut ctx);
        // Each syllable takes two columns: the glyph and its continuation
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '안');
        assert!(buffer.get(1, 0).unwrap().is_continuation());
        assert_eq!(buffer.get(2, 0).unwrap().symbol, '녕');
        assert!(buffer.get(3, 0).unwrap().is_continuation());
        assert_eq!(buffer.get(4, 0).unwrap().symbol, ' ');
    }

    #[test]
    fn test_render_emoji() {
        let text = Text::new("🎉");
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        text.render(&mut ctx);
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '🎉');
        assert!(buffer.get(1, 0).unwrap().is_continuation());
        assert_eq!(buffer.get(2, 0).unwrap().symbol, ' ');
    }
}

/// Test special characters
mod special_characters {
    use super::*;

    #[test]
    fn test_text_with_newlines() {
        let text = Text::new("Line 1\nLine 2\nLine 3");
        assert_eq!(text.content(), "Line 1\nLine 2\nLine 3");
    }

    #[test]
    fn test_text_with_tabs() {
        let text = Text::new("Column 1\tColumn 2");
        assert_eq!(text.content(), "Column 1\tColumn 2");
    }

    #[test]
    fn test_text_with_null_byte() {
        // Null byte in string
        let text = Text::new("Hello\0World");
        assert_eq!(text.content(), "Hello\0World");
    }

    #[test]
    fn test_text_with_carriage_return() {
        let text = Text::new("Line 1\r\nLine 2");
        assert_eq!(text.content(), "Line 1\r\nLine 2");
    }

    #[test]
    fn test_text_with_backspaces_raw() {
        // Use raw string literal for backspace
        let text = Text::new(r"Hello\b\bWorld");
        assert_eq!(text.content(), r"Hello\b\bWorld");
    }

    #[test]
    fn test_text_with_bom() {
        // Byte order mark
        let text = Text::new("\u{FEFF}Hello");
        assert_eq!(text.content(), "\u{FEFF}Hello");
    }
}

/// Test very long strings
mod long_strings {
    use super::*;

    #[test]
    fn test_text_widget_long_string() {
        let long_text = "A".repeat(10000);
        let text = Text::new(&long_text);
        assert_eq!(text.content().len(), 10000);
    }

    #[test]
    fn test_render_long_string() {
        let long_text = "X".repeat(1000);
        let text = Text::new(&long_text);
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        text.render(&mut ctx);
        // Clipped at the right edge; it does not wrap onto the next row
        assert_eq!(row(&buffer, 0), "XXXXXXXXXX");
        assert_eq!(row(&buffer, 1), "          ");
    }

    #[test]
    fn test_text_with_many_newlines() {
        let many_newlines = "\n".repeat(1000);
        let text = Text::new(&many_newlines);
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        text.render(&mut ctx);
        assert!(blank(&buffer));
    }
}

/// Test whitespace-only strings
mod whitespace_strings {
    use super::*;

    #[test]
    fn test_text_all_spaces() {
        let text = Text::new("     ");
        assert_eq!(text.content(), "     ");
    }

    #[test]
    fn test_text_all_tabs() {
        let text = Text::new("\t\t\t");
        assert_eq!(text.content(), "\t\t\t");
    }

    #[test]
    fn test_text_all_newlines() {
        let text = Text::new("\n\n\n");
        assert_eq!(text.content(), "\n\n\n");
    }

    #[test]
    fn test_text_mixed_whitespace() {
        let text = Text::new(" \t\n \r ");
        assert_eq!(text.content(), " \t\n \r ");
    }

    #[test]
    fn test_render_whitespace() {
        let text = Text::new("   ");
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        text.render(&mut ctx);
        // Spaces are measured but draw as blanks
        assert_eq!(text.measure(10, 10), Some((3, 1)));
        assert!(blank(&buffer));
    }
}

/// Test text truncation and clipping
mod text_truncation {
    use super::*;

    #[test]
    fn test_text_wider_than_buffer() {
        let text = Text::new("This is a very long string that exceeds buffer width");
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        text.render(&mut ctx);
        assert_eq!(row(&buffer, 0), "This is a ");
        assert_eq!(row(&buffer, 1), "          ");
        assert_eq!(text.measure(10, 10), Some((10, 1)));
    }

    #[test]
    fn test_text_taller_than_buffer() {
        let text = Text::new("Line 1\nLine 2\nLine 3\nLine 4\nLine 5\nLine 6\nLine 7\nLine 8\nLine 9\nLine 10\nLine 11\nLine 12");
        let mut buffer = Buffer::new(10, 12);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        text.render(&mut ctx);
        // Rows past the area's height are clipped
        assert_eq!(row(&buffer, 0), "Line 1    ");
        assert_eq!(row(&buffer, 9), "Line 10   ");
        assert_eq!(row(&buffer, 10), "          ");
        assert_eq!(row(&buffer, 11), "          ");
    }

    #[test]
    fn test_text_in_zero_width_buffer() {
        let text = Text::new("Hello");
        let mut buffer = Buffer::new(5, 10);
        let area = Rect::new(0, 0, 0, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        text.render(&mut ctx);
        assert!(blank(&buffer));
        assert_eq!(text.measure(0, 10), Some((0, 1)));
    }
}
