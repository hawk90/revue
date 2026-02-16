//! Unicode width utilities tests

use revue::utils::unicode::{
    center_to_width, char_width, display_width, pad_to_width, right_align_to_width, split_at_width,
    truncate_to_width, truncate_with_ellipsis, truncate_with_suffix, wrap_to_width,
};

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
