//! Tests for figlet rendering

#[cfg(test)]
mod tests {
    use super::super::api::*;
    use super::super::types::FigletFont;

    #[test]
    fn test_figlet_block() {
        let result = figlet("HI");
        assert!(result.contains("██╗"));
        assert!(result.lines().count() == 6);
    }

    #[test]
    fn test_figlet_small() {
        let result = figlet_with_font("AB", FigletFont::Small);
        assert!(result.lines().count() == 5);
    }

    #[test]
    fn test_figlet_mini() {
        let result = figlet_with_font("XY", FigletFont::Mini);
        assert!(result.lines().count() == 3);
    }

    #[test]
    fn test_font_height() {
        assert_eq!(font_height(FigletFont::Block), 6);
        assert_eq!(font_height(FigletFont::Small), 5);
        assert_eq!(font_height(FigletFont::Mini), 3);
    }

    #[test]
    fn test_figlet_lines() {
        let lines = figlet_lines("A", FigletFont::Block);
        assert_eq!(lines.len(), 6);
    }

    #[test]
    fn test_figlet_space() {
        let result = figlet("A B");
        // Should contain space between A and B
        assert!(result.contains("   "));
    }

    #[test]
    fn test_figlet_numbers() {
        let result = figlet("123");
        assert!(result.lines().count() == 6);
    }

    #[test]
    fn test_figlet_special_chars() {
        let result = figlet("!?.");
        assert!(result.lines().count() == 6);
    }

    #[test]
    fn test_figlet_font_default() {
        let font = FigletFont::default();
        assert_eq!(font, FigletFont::Block);
    }

    #[test]
    fn test_figlet_font_clone() {
        let font = FigletFont::Slant;
        let cloned = font.clone();
        assert_eq!(font, cloned);
    }

    #[test]
    fn test_figlet_font_debug() {
        let font = FigletFont::Banner;
        let debug = format!("{:?}", font);
        assert!(debug.contains("Banner"));
    }

    #[test]
    fn test_figlet_font_eq() {
        assert_eq!(FigletFont::Block, FigletFont::Block);
        assert_ne!(FigletFont::Block, FigletFont::Slant);
    }

    #[test]
    fn test_font_height_all_fonts() {
        assert_eq!(font_height(FigletFont::Block), 6);
        assert_eq!(font_height(FigletFont::Slant), 6);
        assert_eq!(font_height(FigletFont::Banner), 7);
        assert_eq!(font_height(FigletFont::Small), 5);
        assert_eq!(font_height(FigletFont::Mini), 3);
    }

    #[test]
    fn test_figlet_slant() {
        let result = figlet_with_font("AB", FigletFont::Slant);
        assert!(result.lines().count() == 6);
    }

    #[test]
    fn test_figlet_banner() {
        let result = figlet_with_font("HELLO", FigletFont::Banner);
        assert!(result.lines().count() == 7);
        // Banner uses # characters
        assert!(result.contains('#'));
    }

    #[test]
    fn test_figlet_all_letters_block() {
        for ch in 'A'..='Z' {
            let result = figlet(&ch.to_string());
            assert_eq!(
                result.lines().count(),
                6,
                "Letter {} should have 6 lines",
                ch
            );
        }
    }

    #[test]
    fn test_figlet_all_digits_block() {
        for ch in '0'..='9' {
            let result = figlet(&ch.to_string());
            assert_eq!(
                result.lines().count(),
                6,
                "Digit {} should have 6 lines",
                ch
            );
        }
    }

    #[test]
    fn test_figlet_lowercase_conversion() {
        // Lowercase should be converted to uppercase
        let lower = figlet("hello");
        let upper = figlet("HELLO");
        assert_eq!(lower, upper);
    }

    #[test]
    fn test_figlet_empty_string() {
        let result = figlet("");
        // Empty string produces lines joined by newlines but with empty content
        // The join creates 5 separators between 6 empty strings, resulting in 5 lines when split
        assert!(result.lines().count() <= 6);
    }

    #[test]
    fn test_figlet_unknown_char() {
        // Unknown char should use fallback glyph
        let result = figlet("€");
        assert_eq!(result.lines().count(), 6);
        assert!(result.contains('▄') || result.contains('█') || result.contains('▀'));
    }

    #[test]
    fn test_figlet_mini_letters() {
        for ch in 'A'..='Z' {
            let result = figlet_with_font(&ch.to_string(), FigletFont::Mini);
            assert_eq!(
                result.lines().count(),
                3,
                "Mini letter {} should have 3 lines",
                ch
            );
        }
    }

    #[test]
    fn test_figlet_mini_digits() {
        for ch in '0'..='9' {
            let result = figlet_with_font(&ch.to_string(), FigletFont::Mini);
            assert_eq!(
                result.lines().count(),
                3,
                "Mini digit {} should have 3 lines",
                ch
            );
        }
    }

    #[test]
    fn test_figlet_small_letters() {
        for ch in 'A'..='Z' {
            let result = figlet_with_font(&ch.to_string(), FigletFont::Small);
            assert_eq!(
                result.lines().count(),
                5,
                "Small letter {} should have 5 lines",
                ch
            );
        }
    }

    #[test]
    fn test_figlet_lines_conversion() {
        let lines = figlet_lines("TEST", FigletFont::Block);
        assert_eq!(lines.len(), 6);
        assert!(lines.iter().all(|l| l.is_ascii() == false)); // Unicode chars
    }

    #[test]
    fn test_figlet_lines_mini() {
        let lines = figlet_lines("HI", FigletFont::Mini);
        assert_eq!(lines.len(), 3);
    }

    #[test]
    fn test_figlet_slant_space() {
        let result = figlet_with_font("A B", FigletFont::Slant);
        assert!(result.lines().count() == 6);
    }

    #[test]
    fn test_figlet_slant_unknown_fallback() {
        // Slant font falls back to block for unknown chars
        let result = figlet_with_font("@", FigletFont::Slant);
        assert!(result.lines().count() == 6);
    }

    #[test]
    fn test_figlet_banner_space() {
        let result = figlet_with_font("A B", FigletFont::Banner);
        assert!(result.lines().count() == 7);
    }

    #[test]
    fn test_figlet_banner_fallback() {
        // Banner falls back to block (converted to 7 lines) for unknown chars
        let result = figlet_with_font("@", FigletFont::Banner);
        assert!(result.lines().count() == 7);
    }

    #[test]
    fn test_block_char_punctuation() {
        let result = figlet("!?.,:-_+=/#@");
        assert_eq!(result.lines().count(), 6);
    }

    #[test]
    fn test_small_char_unknown() {
        let result = figlet_with_font("€", FigletFont::Small);
        assert_eq!(result.lines().count(), 5);
    }

    #[test]
    fn test_mini_char_special() {
        let result = figlet_with_font("!?.-", FigletFont::Mini);
        assert_eq!(result.lines().count(), 3);
    }

    #[test]
    fn test_figlet_multiword() {
        let result = figlet("HELLO WORLD");
        assert_eq!(result.lines().count(), 6);
        // Should be wider due to space
        let first_line = result.lines().next().unwrap();
        assert!(first_line.len() > 20);
    }

    #[test]
    fn test_figlet_with_font_routes_correctly() {
        // Each font should produce different output
        let block = figlet_with_font("A", FigletFont::Block);
        let small = figlet_with_font("A", FigletFont::Small);
        let mini = figlet_with_font("A", FigletFont::Mini);

        assert_ne!(block.lines().count(), small.lines().count());
        assert_ne!(small.lines().count(), mini.lines().count());
    }

    #[test]
    fn test_figlet_copy_trait() {
        let font = FigletFont::Block;
        let copied = font; // Copy, not move
        assert_eq!(font, copied);
    }
}
