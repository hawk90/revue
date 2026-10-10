//! `utils::text`: character/byte indexing, padding, truncation, wrapping
//! and progress bars

use revue::utils::text::{
    byte_to_char_index, center, char_count, char_slice, char_to_byte_index,
    char_to_byte_index_with_char, display_width, insert_at_char, pad_left, pad_right, progress_bar,
    progress_bar_precise, remove_char_at, remove_char_range, repeat_char, split_fixed_width,
    truncate, truncate_start, wrap_text,
};

// =============================================================================
// char_to_byte_index
// =============================================================================

#[test]
fn char_to_byte_index_ascii() {
    {
        let s = "hello";
        assert_eq!(char_to_byte_index(s, 0), 0);
        assert_eq!(char_to_byte_index(s, 4), 4);
    }
    {
        assert_eq!(char_to_byte_index("hello", 0), 0);
        assert_eq!(char_to_byte_index("hello", 2), 2);
        assert_eq!(char_to_byte_index("hello", 5), 5);
    }
}
#[test]
fn char_to_byte_index_multibyte() {
    // 'é' is 2 bytes in UTF-8
    let s = "héllo";
    assert_eq!(char_to_byte_index(s, 0), 0); // 'h'
    assert_eq!(char_to_byte_index(s, 1), 1); // 'é' starts at byte 1
    assert_eq!(char_to_byte_index(s, 2), 3); // 'l' starts at byte 3
}
#[test]
fn char_to_byte_index_korean() {
    {
        // Each Korean char is 3 bytes
        let s = "안녕하세요";
        assert_eq!(char_to_byte_index(s, 0), 0);
        assert_eq!(char_to_byte_index(s, 1), 3);
        assert_eq!(char_to_byte_index(s, 2), 6);
    }
    {
        // Each Korean char is 3 bytes in UTF-8
        let s = "안녕하세요";
        assert_eq!(char_to_byte_index(s, 0), 0);
        assert_eq!(char_to_byte_index(s, 1), 3);
        assert_eq!(char_to_byte_index(s, 2), 6);
    }
}
#[test]
fn char_to_byte_index_emoji() {
    // '😀' is 4 bytes
    let s = "a😀b";
    assert_eq!(char_to_byte_index(s, 0), 0); // 'a'
    assert_eq!(char_to_byte_index(s, 1), 1); // '😀'
    assert_eq!(char_to_byte_index(s, 2), 5); // 'b'
}
#[test]
fn char_to_byte_index_empty() {
    {
        assert_eq!(char_to_byte_index("", 0), 0);
        assert_eq!(char_to_byte_index("", 5), 0);
    }
    {
        assert_eq!(char_to_byte_index("", 0), 0);
    }
}
#[test]
fn char_to_byte_index_out_of_bounds() {
    {
        let s = "abc";
        assert_eq!(char_to_byte_index(s, 10), s.len());
    }
    {
        assert_eq!(char_to_byte_index("abc", 10), 3);
    }
}

// =============================================================================
// char_to_byte_index_with_char
// =============================================================================

#[test]
fn char_to_byte_index_with_char_valid() {
    let s = "hello";
    assert_eq!(char_to_byte_index_with_char(s, 0), (0, Some('h')));
    assert_eq!(char_to_byte_index_with_char(s, 4), (4, Some('o')));
}
#[test]
fn char_to_byte_index_with_char_oob() {
    let s = "hi";
    assert_eq!(char_to_byte_index_with_char(s, 5), (s.len(), None));
}
#[test]
fn char_to_byte_index_with_char_korean() {
    {
        let s = "가나";
        let (idx, ch) = char_to_byte_index_with_char(s, 1);
        assert_eq!(idx, 3);
        assert_eq!(ch, Some('나'));
    }
    {
        let (idx, ch) = char_to_byte_index_with_char("가나다", 1);
        assert_eq!(idx, 3);
        assert_eq!(ch, Some('나'));
    }
}

// =============================================================================
// byte_to_char_index
// =============================================================================

#[test]
fn byte_to_char_index_ascii() {
    {
        let s = "hello";
        assert_eq!(byte_to_char_index(s, 0), 0);
        assert_eq!(byte_to_char_index(s, 3), 3);
    }
    {
        assert_eq!(byte_to_char_index("hello", 0), 0);
        assert_eq!(byte_to_char_index("hello", 3), 3);
    }
}
#[test]
fn byte_to_char_index_multibyte() {
    let s = "héllo"; // 'é' = 2 bytes → bytes [0,1,2,3,4,5]
    assert_eq!(byte_to_char_index(s, 0), 0); // 'h'
    assert_eq!(byte_to_char_index(s, 1), 1); // start of 'é'
    assert_eq!(byte_to_char_index(s, 3), 2); // 'l'
}
#[test]
fn byte_to_char_index_past_end() {
    {
        let s = "abc";
        assert_eq!(byte_to_char_index(s, 100), 3);
    }
    {
        assert_eq!(byte_to_char_index("ab", 100), 2);
    }
}
#[test]
fn byte_to_char_index_empty() {
    assert_eq!(byte_to_char_index("", 0), 0);
}
#[test]
fn byte_to_char_index_mid_char_boundary() {
    {
        // 'é' occupies bytes 1..3; byte 2 is mid-character
        let s = "héllo";
        // Should snap to previous valid boundary
        let result = byte_to_char_index(s, 2);
        assert_eq!(result, 1); // char index of 'é'
    }
    {
        // byte index 1 is mid-character for Korean
        let s = "가나";
        let result = byte_to_char_index(s, 1);
        assert_eq!(result, 0); // rounds down to previous boundary
    }
}

// =============================================================================
// char_count
// =============================================================================

#[test]
fn char_count_ascii() {
    assert_eq!(char_count("hello"), 5);
}
#[test]
fn char_count_korean() {
    assert_eq!(char_count("안녕"), 2);
}
#[test]
fn char_count_empty() {
    assert_eq!(char_count(""), 0);
}
#[test]
fn char_count_mixed() {
    assert_eq!(char_count("a😀b"), 3);
}

// =============================================================================
// char_slice
// =============================================================================

#[test]
fn char_slice_ascii() {
    {
        assert_eq!(char_slice("hello", 1, 4), "ell");
    }
    {
        assert_eq!(char_slice("hello world", 0, 5), "hello");
        assert_eq!(char_slice("hello world", 6, 11), "world");
    }
}
#[test]
fn char_slice_korean() {
    {
        assert_eq!(char_slice("안녕하세요", 1, 3), "녕하");
    }
    {
        assert_eq!(char_slice("안녕하세요", 0, 2), "안녕");
        assert_eq!(char_slice("안녕하세요", 2, 5), "하세요");
    }
}
#[test]
fn char_slice_full() {
    assert_eq!(char_slice("abc", 0, 3), "abc");
}
#[test]
fn char_slice_empty_range() {
    {
        assert_eq!(char_slice("abc", 2, 2), "");
    }
    {
        assert_eq!(char_slice("hello", 3, 3), "");
        assert_eq!(char_slice("hello", 5, 3), "");
    }
}
#[test]
fn char_slice_start_past_end() {
    assert_eq!(char_slice("abc", 5, 10), "");
}
#[test]
fn char_slice_reversed_range() {
    assert_eq!(char_slice("abc", 3, 1), "");
}
#[test]
fn char_slice_end_beyond_len() {
    assert_eq!(char_slice("abc", 1, 100), "bc");
}

// =============================================================================
// insert_at_char
// =============================================================================

#[test]
fn insert_at_char_beginning() {
    let mut s = String::from("world");
    let cursor = insert_at_char(&mut s, 0, "hello ");
    assert_eq!(s, "hello world");
    assert_eq!(cursor, 6);
}
#[test]
fn insert_at_char_middle() {
    let mut s = String::from("hllo");
    let cursor = insert_at_char(&mut s, 1, "e");
    assert_eq!(s, "hello");
    assert_eq!(cursor, 2);
}
#[test]
fn insert_at_char_end() {
    let mut s = String::from("hello");
    let cursor = insert_at_char(&mut s, 5, "!");
    assert_eq!(s, "hello!");
    assert_eq!(cursor, 6);
}
#[test]
fn insert_at_char_multibyte() {
    let mut s = String::from("안하세요");
    let cursor = insert_at_char(&mut s, 1, "녕");
    assert_eq!(s, "안녕하세요");
    assert_eq!(cursor, 2);
}

// =============================================================================
// remove_char_at
// =============================================================================

#[test]
fn remove_char_at_beginning() {
    let mut s = String::from("hello");
    let removed = remove_char_at(&mut s, 0);
    assert_eq!(removed, Some('h'));
    assert_eq!(s, "ello");
}
#[test]
fn remove_char_at_middle() {
    let mut s = String::from("hello");
    let removed = remove_char_at(&mut s, 2);
    assert_eq!(removed, Some('l'));
    assert_eq!(s, "helo");
}
#[test]
fn remove_char_at_end() {
    let mut s = String::from("hello");
    let removed = remove_char_at(&mut s, 4);
    assert_eq!(removed, Some('o'));
    assert_eq!(s, "hell");
}
#[test]
fn remove_char_at_oob() {
    let mut s = String::from("hello");
    let removed = remove_char_at(&mut s, 10);
    assert_eq!(removed, None);
    assert_eq!(s, "hello");
}
#[test]
fn remove_char_at_multibyte() {
    let mut s = String::from("안녕");
    let removed = remove_char_at(&mut s, 0);
    assert_eq!(removed, Some('안'));
    assert_eq!(s, "녕");
}

// =============================================================================
// remove_char_range
// =============================================================================

#[test]
fn remove_char_range_middle() {
    let mut s = String::from("hello world");
    remove_char_range(&mut s, 5, 11);
    assert_eq!(s, "hello");
}
#[test]
fn remove_char_range_beginning() {
    let mut s = String::from("hello");
    remove_char_range(&mut s, 0, 3);
    assert_eq!(s, "lo");
}
#[test]
fn remove_char_range_noop_equal() {
    let mut s = String::from("hello");
    remove_char_range(&mut s, 2, 2);
    assert_eq!(s, "hello");
}
#[test]
fn remove_char_range_noop_reversed() {
    let mut s = String::from("hello");
    remove_char_range(&mut s, 4, 1);
    assert_eq!(s, "hello");
}
#[test]
fn remove_char_range_korean() {
    {
        let mut s = String::from("안녕하세요");
        remove_char_range(&mut s, 1, 4);
        assert_eq!(s, "안요");
    }
    {
        let mut s = String::from("안녕하세요");
        remove_char_range(&mut s, 2, 4);
        assert_eq!(s, "안녕요");
    }
}

// =============================================================================
// truncate
// =============================================================================

#[test]
fn truncate_no_change() {
    assert_eq!(truncate("hello", 10), "hello");
}
#[test]
fn truncate_exact_fit() {
    assert_eq!(truncate("hello", 5), "hello");
}
#[test]
fn truncate_with_ellipsis() {
    let result = truncate("hello world", 8);
    assert_eq!(result, "hello w…");
}
#[test]
fn truncate_width_1() {
    assert_eq!(truncate("hello", 1), "…");
}
#[test]
fn truncate_width_0() {
    assert_eq!(truncate("hello", 0), "");
}
#[test]
fn truncate_empty_input() {
    assert_eq!(truncate("", 5), "");
}

// =============================================================================
// truncate_start
// =============================================================================

#[test]
fn truncate_start_no_change() {
    assert_eq!(truncate_start("hello", 10), "hello");
}
#[test]
fn truncate_start_exact_fit() {
    assert_eq!(truncate_start("hello", 5), "hello");
}
#[test]
fn truncate_start_with_ellipsis() {
    let result = truncate_start("hello world", 8);
    assert_eq!(result, "…o world");
}
#[test]
fn truncate_start_width_1() {
    assert_eq!(truncate_start("hello", 1), "…");
}
#[test]
fn truncate_start_width_0() {
    assert_eq!(truncate_start("hello", 0), "…");
}
#[test]
fn truncate_start_empty_input() {
    assert_eq!(truncate_start("", 5), "");
}

// =============================================================================
// center
// =============================================================================

#[test]
fn center_shorter_text() {
    let result = center("hi", 10);
    assert_eq!(result.len(), 10);
    assert_eq!(result, "    hi    ");
}
#[test]
fn center_odd_padding() {
    let result = center("hi", 7);
    // 5 padding chars: 2 left + 3 right (or vice versa)
    assert_eq!(result.chars().count(), 7);
    assert!(result.contains("hi"));
}
#[test]
fn center_text_wider_than_width() {
    let result = center("long text", 3);
    assert_eq!(result, "long text");
}
#[test]
fn center_exact_width() {
    let result = center("abc", 3);
    assert_eq!(result, "abc");
}

// =============================================================================
// pad_left
// =============================================================================

#[test]
fn pad_left_shorter_text() {
    let result = pad_left("hi", 6);
    assert_eq!(result, "    hi");
}
#[test]
fn pad_left_text_wider() {
    let result = pad_left("hello", 3);
    assert_eq!(result, "hello");
}
#[test]
fn pad_left_exact_width() {
    let result = pad_left("abc", 3);
    assert_eq!(result, "abc");
}

// =============================================================================
// pad_right
// =============================================================================

#[test]
fn pad_right_shorter_text() {
    let result = pad_right("hi", 6);
    assert_eq!(result, "hi    ");
}
#[test]
fn pad_right_text_wider() {
    let result = pad_right("hello", 3);
    assert_eq!(result, "hello");
}
#[test]
fn pad_right_exact_width() {
    let result = pad_right("abc", 3);
    assert_eq!(result, "abc");
}

// =============================================================================
// wrap_text
// =============================================================================

#[test]
fn wrap_text_empty() {
    {
        assert_eq!(wrap_text("", 10), Vec::<String>::new());
    }
    {
        assert_eq!(wrap_text("", 10), Vec::<String>::new());
        assert_eq!(wrap_text("hello", 0), Vec::<String>::new());
    }
}
#[test]
fn wrap_text_width_zero() {
    assert_eq!(wrap_text("hello", 0), Vec::<String>::new());
}
#[test]
fn wrap_text_fits_in_one_line() {
    assert_eq!(wrap_text("hello world", 20), vec!["hello world"]);
}
#[test]
fn wrap_text_wraps_at_word_boundary() {
    let result = wrap_text("hello world foo", 11);
    assert_eq!(result, vec!["hello world", "foo"]);
}
#[test]
fn wrap_text_long_word_splits() {
    let result = wrap_text("abcdefghij", 5);
    assert_eq!(result, vec!["abcde", "fghij"]);
}
#[test]
fn wrap_text_preserves_empty_paragraphs() {
    let result = wrap_text("a\n\nb", 10);
    assert_eq!(result, vec!["a", "", "b"]);
}
#[test]
fn wrap_text_multiple_words_per_line() {
    let result = wrap_text("a b c d e f", 5);
    // "a b c" fits in 5, "d e f" fits in 5
    assert_eq!(result, vec!["a b c", "d e f"]);
}

// =============================================================================
// split_fixed_width
// =============================================================================

#[test]
fn split_fixed_width_normal() {
    let result = split_fixed_width("abcdef", 3);
    assert_eq!(result, vec!["abc", "def"]);
}
#[test]
fn split_fixed_width_uneven() {
    let result = split_fixed_width("abcde", 3);
    assert_eq!(result, vec!["abc", "de"]);
}
#[test]
fn split_fixed_width_empty() {
    {
        let result = split_fixed_width("", 5);
        assert_eq!(result, vec![""]);
    }
    {
        assert_eq!(split_fixed_width("abc", 0), Vec::<String>::new());
    }
}
#[test]
fn split_fixed_width_zero_width() {
    let result = split_fixed_width("abc", 0);
    assert_eq!(result, Vec::<String>::new());
}
#[test]
fn split_fixed_width_width_larger_than_text() {
    let result = split_fixed_width("hi", 10);
    assert_eq!(result, vec!["hi"]);
}

// =============================================================================
// display_width
// =============================================================================

#[test]
fn display_width_ascii() {
    assert_eq!(display_width("hello"), 5);
}
#[test]
fn display_width_empty() {
    assert_eq!(display_width(""), 0);
}
#[test]
fn display_width_cjk() {
    // text.rs display_width should now delegate to unicode.rs
    assert_eq!(display_width("한글"), 4); // 2 chars × 2 width
    assert_eq!(display_width("Hello한글"), 9); // 5 + 4
}
#[test]
fn display_width_emoji() {
    assert_eq!(display_width("🎉"), 2);
}
#[test]
fn truncate_cjk() {
    // truncate should respect display width, not char count
    let result = truncate("안녕하세요", 5);
    // "안녕" = 4 width, "하" would make 6 > 5, so truncate with ellipsis
    assert_eq!(display_width(&result), 5); // "안녕…" = 4 + 1
}
#[test]
fn center_cjk() {
    let result = center("한", 6); // "한" is width 2, center in 6
    assert_eq!(display_width(&result), 6);
    assert!(result.contains("한"));
}
#[test]
fn pad_right_cjk() {
    let result = pad_right("한글", 8); // width 4, pad to 8
    assert_eq!(display_width(&result), 8);
    assert!(result.starts_with("한글"));
}
#[test]
fn pad_left_cjk() {
    let result = pad_left("한글", 8); // width 4, pad to 8
    assert_eq!(display_width(&result), 8);
    assert!(result.ends_with("한글"));
}
#[test]
fn wrap_text_cjk() {
    let result = wrap_text("가나다라마바사", 5);
    // Each char is width 2, so max 2 chars per line (width 4), "마" starts new line
    assert!(result.len() >= 2);
    for line in &result {
        assert!(display_width(line) <= 5);
    }
}

// =============================================================================
// repeat_char
// =============================================================================

#[test]
fn repeat_char_basic() {
    {
        assert_eq!(repeat_char('x', 3), "xxx");
    }
    {
        assert_eq!(repeat_char('x', 3), "xxx");
        assert_eq!(repeat_char('가', 2), "가가");
        assert_eq!(repeat_char('a', 0), "");
    }
}
#[test]
fn repeat_char_zero() {
    assert_eq!(repeat_char('x', 0), "");
}

// =============================================================================
// progress_bar
// =============================================================================

#[test]
fn progress_bar_zero() {
    let result = progress_bar(0.0, 10);
    assert_eq!(result.chars().count(), 10);
    assert!(result.chars().all(|c| c == '░'));
}
#[test]
fn progress_bar_full() {
    {
        let result = progress_bar(1.0, 10);
        assert_eq!(result.chars().count(), 10);
        assert!(result.chars().all(|c| c == '█'));
    }
    {
        let bar = progress_bar(1.0, 10);
        assert_eq!(bar, "██████████");
    }
}
#[test]
fn progress_bar_half() {
    {
        let result = progress_bar(0.5, 10);
        assert_eq!(result.chars().count(), 10);
        let filled: usize = result.chars().filter(|&c| c == '█').count();
        assert_eq!(filled, 5);
    }
    {
        let bar = progress_bar(0.5, 10);
        assert_eq!(bar, "█████░░░░░");
    }
}
#[test]
fn progress_bar_clamps_above_1() {
    let result = progress_bar(1.5, 5);
    assert!(result.chars().all(|c| c == '█'));
}
#[test]
fn progress_bar_clamps_below_0() {
    let result = progress_bar(-0.5, 5);
    assert!(result.chars().all(|c| c == '░'));
}

// =============================================================================
// progress_bar_precise
// =============================================================================

#[test]
fn progress_bar_precise_zero() {
    let result = progress_bar_precise(0.0, 10);
    assert_eq!(result.chars().count(), 10);
}
#[test]
fn progress_bar_precise_full() {
    {
        let result = progress_bar_precise(1.0, 10);
        assert_eq!(result.chars().count(), 10);
        assert!(result.chars().all(|c| c == '█'));
    }
    {
        let bar = progress_bar_precise(1.0, 10);
        assert!(bar.contains('█'));
    }
}
#[test]
fn progress_bar_precise_half() {
    let result = progress_bar_precise(0.5, 10);
    assert_eq!(result.chars().count(), 10);
}
#[test]
fn progress_bar_precise_partial_block() {
    // 0.1 with width 10 = 1 full block worth, which is 8 eighths
    let result = progress_bar_precise(0.1, 10);
    assert_eq!(result.chars().count(), 10);
}

// =============================================================================
// More cases
// =============================================================================

#[test]
fn char_to_byte_index_mixed() {
    let s = "a한b";
    assert_eq!(char_to_byte_index(s, 0), 0); // 'a'
    assert_eq!(char_to_byte_index(s, 1), 1); // '한'
    assert_eq!(char_to_byte_index(s, 2), 4); // 'b'
}

// =============================================================================
// char_to_byte_index_with_char
// =============================================================================

#[test]
fn char_to_byte_index_with_char_basic() {
    let (idx, ch) = char_to_byte_index_with_char("hello", 1);
    assert_eq!(idx, 1);
    assert_eq!(ch, Some('e'));
}

#[test]
fn char_to_byte_index_with_char_out_of_bounds() {
    let (idx, ch) = char_to_byte_index_with_char("abc", 5);
    assert_eq!(idx, 3);
    assert_eq!(ch, None);
}

#[test]
fn byte_to_char_index_korean() {
    let s = "가나다";
    assert_eq!(byte_to_char_index(s, 0), 0);
    assert_eq!(byte_to_char_index(s, 3), 1);
    assert_eq!(byte_to_char_index(s, 6), 2);
}

// =============================================================================
// char_count
// =============================================================================

#[test]
fn char_count_basic() {
    assert_eq!(char_count("hello"), 5);
    assert_eq!(char_count("안녕"), 2);
    assert_eq!(char_count(""), 0);
}

#[test]
fn char_slice_out_of_bounds_start() {
    assert_eq!(char_slice("hi", 10, 20), "");
}

// =============================================================================
// insert_at_char
// =============================================================================

#[test]
fn insert_at_char_ascii() {
    let mut s = String::from("hllo");
    let pos = insert_at_char(&mut s, 1, "e");
    assert_eq!(s, "hello");
    assert_eq!(pos, 2);
}

#[test]
fn insert_at_char_korean() {
    let mut s = String::from("안하세요");
    let pos = insert_at_char(&mut s, 1, "녕");
    assert_eq!(s, "안녕하세요");
    assert_eq!(pos, 2);
}

#[test]
fn insert_at_char_at_end() {
    let mut s = String::from("abc");
    let pos = insert_at_char(&mut s, 3, "d");
    assert_eq!(s, "abcd");
    assert_eq!(pos, 4);
}

// =============================================================================
// remove_char_at
// =============================================================================

#[test]
fn remove_char_at_ascii() {
    let mut s = String::from("hello");
    let ch = remove_char_at(&mut s, 1);
    assert_eq!(ch, Some('e'));
    assert_eq!(s, "hllo");
}

#[test]
fn remove_char_at_korean() {
    let mut s = String::from("가나다");
    let ch = remove_char_at(&mut s, 1);
    assert_eq!(ch, Some('나'));
    assert_eq!(s, "가다");
}

#[test]
fn remove_char_at_out_of_bounds() {
    let mut s = String::from("abc");
    let ch = remove_char_at(&mut s, 10);
    assert_eq!(ch, None);
    assert_eq!(s, "abc");
}

// =============================================================================
// remove_char_range
// =============================================================================

#[test]
fn remove_char_range_basic() {
    let mut s = String::from("hello world");
    remove_char_range(&mut s, 5, 11);
    assert_eq!(s, "hello");
}

#[test]
fn remove_char_range_noop() {
    let mut s = String::from("abc");
    remove_char_range(&mut s, 3, 1); // start >= end
    assert_eq!(s, "abc");
}

// =============================================================================
// truncate
// =============================================================================

#[test]
fn truncate_short_text() {
    assert_eq!(truncate("hi", 10), "hi");
}

// =============================================================================
// truncate_start
// =============================================================================

#[test]
fn truncate_start_short_text() {
    assert_eq!(truncate_start("hi", 10), "hi");
}

#[test]
fn truncate_start_long_text() {
    let result = truncate_start("abcdefghij", 5);
    assert!(result.starts_with('…'));
    assert!(result.len() <= 10); // reasonable length
}

#[test]
fn truncate_start_width_one() {
    assert_eq!(truncate_start("abcdef", 1), "…");
}

// =============================================================================
// pad_left / pad_right / center (delegate to unicode utils)
// =============================================================================

#[test]
fn pad_left_basic() {
    let result = pad_left("hi", 5);
    assert_eq!(display_width(&result), 5);
    assert!(result.ends_with("hi"));
}

#[test]
fn pad_right_basic() {
    let result = pad_right("hi", 5);
    assert_eq!(display_width(&result), 5);
    assert!(result.starts_with("hi"));
}

// =============================================================================
// wrap_text
// =============================================================================

#[test]
fn wrap_text_basic() {
    let lines = wrap_text("hello world", 5);
    assert!(lines.len() >= 2);
}

#[test]
fn wrap_text_with_newlines() {
    let lines = wrap_text("a\nb\nc", 10);
    assert_eq!(lines.len(), 3);
    assert_eq!(lines[0], "a");
    assert_eq!(lines[1], "b");
    assert_eq!(lines[2], "c");
}

// =============================================================================
// split_fixed_width
// =============================================================================

#[test]
fn split_fixed_width_basic() {
    let chunks = split_fixed_width("abcdef", 2);
    assert_eq!(chunks, vec!["ab", "cd", "ef"]);
}

#[test]
fn split_fixed_width_larger_than_text() {
    let chunks = split_fixed_width("hi", 10);
    assert_eq!(chunks, vec!["hi"]);
}

#[test]
fn display_width_korean() {
    // Korean characters are typically double-width
    assert_eq!(display_width("가"), 2);
    assert_eq!(display_width("안녕"), 4);
}

#[test]
fn progress_bar_empty() {
    let bar = progress_bar(0.0, 10);
    assert_eq!(bar, "░░░░░░░░░░");
}

#[test]
fn progress_bar_clamps() {
    let over = progress_bar(2.0, 5);
    let under = progress_bar(-1.0, 5);
    assert_eq!(over, "█████");
    assert_eq!(under, "░░░░░");
}

#[test]
fn progress_bar_precise_empty() {
    let bar = progress_bar_precise(0.0, 10);
    assert!(!bar.contains('█'));
}

#[test]
fn progress_bar_precise_partial() {
    // Should use partial block characters for partial fill
    let bar = progress_bar_precise(0.3, 10);
    assert!(!bar.is_empty());
}
