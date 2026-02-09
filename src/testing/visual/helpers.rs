//! Helper functions for visual testing

/// Parse hex color string like "#rrggbb"
pub fn parse_hex_color(s: &str) -> Option<(u8, u8, u8)> {
    let s = s.trim().trim_start_matches('#');
    if s.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&s[0..2], 16).ok()?;
    let g = u8::from_str_radix(&s[2..4], 16).ok()?;
    let b = u8::from_str_radix(&s[4..6], 16).ok()?;
    Some((r, g, b))
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // parse_hex_color() helper tests
    // =========================================================================

    #[test]
    fn test_parse_hex_color_valid_with_hash() {
        assert_eq!(parse_hex_color("#ff0000"), Some((255, 0, 0)));
        assert_eq!(parse_hex_color("#00ff00"), Some((0, 255, 0)));
        assert_eq!(parse_hex_color("#0000ff"), Some((0, 0, 255)));
    }

    #[test]
    fn test_parse_hex_color_valid_without_hash() {
        assert_eq!(parse_hex_color("ff0000"), Some((255, 0, 0)));
        assert_eq!(parse_hex_color("00ff00"), Some((0, 255, 0)));
        assert_eq!(parse_hex_color("0000ff"), Some((0, 0, 255)));
    }

    #[test]
    fn test_parse_hex_color_black() {
        assert_eq!(parse_hex_color("#000000"), Some((0, 0, 0)));
        assert_eq!(parse_hex_color("000000"), Some((0, 0, 0)));
    }

    #[test]
    fn test_parse_hex_color_white() {
        assert_eq!(parse_hex_color("#ffffff"), Some((255, 255, 255)));
        assert_eq!(parse_hex_color("FFFFFF"), Some((255, 255, 255)));
    }

    #[test]
    fn test_parse_hex_color_gray() {
        assert_eq!(parse_hex_color("#808080"), Some((128, 128, 128)));
        assert_eq!(parse_hex_color("808080"), Some((128, 128, 128)));
    }

    #[test]
    fn test_parse_hex_color_mixed_case() {
        assert_eq!(parse_hex_color("#AbCdEf"), Some((171, 205, 239)));
        assert_eq!(parse_hex_color("aBcDeF"), Some((171, 205, 239)));
    }

    #[test]
    fn test_parse_hex_color_too_short() {
        assert_eq!(parse_hex_color("#fff"), None);
        assert_eq!(parse_hex_color("fff"), None);
        assert_eq!(parse_hex_color("#00000"), None);
        assert_eq!(parse_hex_color(""), None);
    }

    #[test]
    fn test_parse_hex_color_too_long() {
        assert_eq!(parse_hex_color("#ffffff00"), None); // 8 chars
        assert_eq!(parse_hex_color("ffffffffffff"), None); // 12 chars
    }

    #[test]
    fn test_parse_hex_color_invalid_chars() {
        assert_eq!(parse_hex_color("#gggggg"), None);
        assert_eq!(parse_hex_color("#zzzzzz"), None);
        assert_eq!(parse_hex_color("#xyzabc"), None);
        assert_eq!(parse_hex_color("!@#$%^"), None);
    }

    #[test]
    fn test_parse_hex_color_with_whitespace() {
        assert_eq!(parse_hex_color(" #ff0000"), Some((255, 0, 0)));
        assert_eq!(parse_hex_color("  #00ff00  "), Some((0, 255, 0)));
    }

    #[test]
    fn test_parse_hex_color_partial_invalid() {
        assert_eq!(parse_hex_color("#ff000"), None); // 5 chars
        assert_eq!(parse_hex_color("#ff00000"), None); // 7 chars
        assert_eq!(parse_hex_color("#ff00g0"), None); // invalid char
    }

    #[test]
    fn test_parse_hex_color_web_colors() {
        // Common web colors
        assert_eq!(parse_hex_color("#f00"), None); // Short form not supported
        assert_eq!(parse_hex_color("#ff0000"), Some((255, 0, 0))); // Red
        assert_eq!(parse_hex_color("#00ff00"), Some((0, 255, 0))); // Green
        assert_eq!(parse_hex_color("#0000ff"), Some((0, 0, 255))); // Blue
        assert_eq!(parse_hex_color("#ffff00"), Some((255, 255, 0))); // Yellow
        assert_eq!(parse_hex_color("#ff00ff"), Some((255, 0, 255))); // Magenta
        assert_eq!(parse_hex_color("#00ffff"), Some((0, 255, 255))); // Cyan
    }

    #[test]
    fn test_parse_hex_color_with_special_chars_prefix() {
        assert_eq!(parse_hex_color("##ff0000"), Some((255, 0, 0))); // Double hash
        assert_eq!(parse_hex_color("###ff0000"), Some((255, 0, 0))); // Triple hash
    }

    #[test]
    fn test_parse_hex_color_min_max_values() {
        assert_eq!(parse_hex_color("#000000"), Some((0, 0, 0))); // Minimum
        assert_eq!(parse_hex_color("#ffffff"), Some((255, 255, 255))); // Maximum
    }

    #[test]
    fn test_parse_hex_color_all_digits() {
        assert_eq!(parse_hex_color("#123456"), Some((18, 52, 86)));
        assert_eq!(parse_hex_color("#987654"), Some((152, 118, 84)));
    }

    #[test]
    fn test_parse_hex_color_repeated_patterns() {
        assert_eq!(parse_hex_color("#aaaaaa"), Some((170, 170, 170)));
        assert_eq!(parse_hex_color("#bbbbbb"), Some((187, 187, 187)));
        assert_eq!(parse_hex_color("#cccccc"), Some((204, 204, 204)));
    }

    #[test]
    fn test_parse_hex_color_only_hash() {
        assert_eq!(parse_hex_color("#"), None);
    }

    #[test]
    fn test_parse_hex_color_hash_only() {
        assert_eq!(parse_hex_color("##"), None);
        assert_eq!(parse_hex_color("###"), None);
    }

    #[test]
    fn test_parse_hex_color_binary_like() {
        assert_eq!(parse_hex_color("#010101"), Some((1, 1, 1)));
        assert_eq!(parse_hex_color("#101010"), Some((16, 16, 16)));
    }
}
