//! Tests for text manipulation utilities (src/utils/text.rs)

use revue::utils::{
    byte_to_char_index, char_count, char_slice, char_to_byte_index,
    char_to_byte_index_with_char, insert_at_char, pad_left, pad_right, remove_char_at,
    remove_char_range, truncate, truncate_start, wrap_text,
};
use revue::utils::text::{
    display_width, progress_bar, progress_bar_precise, repeat_char, split_fixed_width,
};

// =============================================================================
// char_to_byte_index
// =============================================================================

#[test]
fn char_to_byte_index_ascii() {
    assert_eq!(char_to_byte_index("hello", 0), 0);
    assert_eq!(char_to_byte_index("hello", 2), 2);
    assert_eq!(char_to_byte_index("hello", 5), 5);
}

#[test]
fn char_to_byte_index_korean() {
    // Each Korean char is 3 bytes in UTF-8
    let s = "안녕하세요";
    assert_eq!(char_to_byte_index(s, 0), 0);
    assert_eq!(char_to_byte_index(s, 1), 3);
    assert_eq!(char_to_byte_index(s, 2), 6);
}

#[test]
fn char_to_byte_index_mixed() {
    let s = "a한b";
    assert_eq!(char_to_byte_index(s, 0), 0); // 'a'
    assert_eq!(char_to_byte_index(s, 1), 1); // '한'
    assert_eq!(char_to_byte_index(s, 2), 4); // 'b'
}

#[test]
fn char_to_byte_index_out_of_bounds() {
    assert_eq!(char_to_byte_index("abc", 10), 3);
}

#[test]
fn char_to_byte_index_empty() {
    assert_eq!(char_to_byte_index("", 0), 0);
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
fn char_to_byte_index_with_char_korean() {
    let (idx, ch) = char_to_byte_index_with_char("가나다", 1);
    assert_eq!(idx, 3);
    assert_eq!(ch, Some('나'));
}

#[test]
fn char_to_byte_index_with_char_out_of_bounds() {
    let (idx, ch) = char_to_byte_index_with_char("abc", 5);
    assert_eq!(idx, 3);
    assert_eq!(ch, None);
}

// =============================================================================
// byte_to_char_index
// =============================================================================

#[test]
fn byte_to_char_index_ascii() {
    assert_eq!(byte_to_char_index("hello", 0), 0);
    assert_eq!(byte_to_char_index("hello", 3), 3);
}

#[test]
fn byte_to_char_index_korean() {
    let s = "가나다";
    assert_eq!(byte_to_char_index(s, 0), 0);
    assert_eq!(byte_to_char_index(s, 3), 1);
    assert_eq!(byte_to_char_index(s, 6), 2);
}

#[test]
fn byte_to_char_index_mid_char_boundary() {
    // byte index 1 is mid-character for Korean
    let s = "가나";
    let result = byte_to_char_index(s, 1);
    assert_eq!(result, 0); // rounds down to previous boundary
}

#[test]
fn byte_to_char_index_past_end() {
    assert_eq!(byte_to_char_index("ab", 100), 2);
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

// =============================================================================
// char_slice
// =============================================================================

#[test]
fn char_slice_ascii() {
    assert_eq!(char_slice("hello world", 0, 5), "hello");
    assert_eq!(char_slice("hello world", 6, 11), "world");
}

#[test]
fn char_slice_korean() {
    assert_eq!(char_slice("안녕하세요", 0, 2), "안녕");
    assert_eq!(char_slice("안녕하세요", 2, 5), "하세요");
}

#[test]
fn char_slice_empty_range() {
    assert_eq!(char_slice("hello", 3, 3), "");
    assert_eq!(char_slice("hello", 5, 3), "");
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
fn remove_char_range_korean() {
    let mut s = String::from("안녕하세요");
    remove_char_range(&mut s, 2, 4);
    assert_eq!(s, "안녕요");
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

#[test]
fn truncate_exact_fit() {
    assert_eq!(truncate("hello", 5), "hello");
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
fn wrap_text_empty() {
    assert_eq!(wrap_text("", 10), Vec::<String>::new());
    assert_eq!(wrap_text("hello", 0), Vec::<String>::new());
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
fn split_fixed_width_empty() {
    assert_eq!(split_fixed_width("abc", 0), Vec::<String>::new());
}

#[test]
fn split_fixed_width_larger_than_text() {
    let chunks = split_fixed_width("hi", 10);
    assert_eq!(chunks, vec!["hi"]);
}

// =============================================================================
// display_width
// =============================================================================

#[test]
fn display_width_ascii() {
    assert_eq!(display_width("hello"), 5);
}

#[test]
fn display_width_korean() {
    // Korean characters are typically double-width
    assert_eq!(display_width("가"), 2);
    assert_eq!(display_width("안녕"), 4);
}

#[test]
fn display_width_empty() {
    assert_eq!(display_width(""), 0);
}

// =============================================================================
// repeat_char
// =============================================================================

#[test]
fn repeat_char_basic() {
    assert_eq!(repeat_char('x', 3), "xxx");
    assert_eq!(repeat_char('가', 2), "가가");
    assert_eq!(repeat_char('a', 0), "");
}

// =============================================================================
// progress_bar
// =============================================================================

#[test]
fn progress_bar_full() {
    let bar = progress_bar(1.0, 10);
    assert_eq!(bar, "██████████");
}

#[test]
fn progress_bar_empty() {
    let bar = progress_bar(0.0, 10);
    assert_eq!(bar, "░░░░░░░░░░");
}

#[test]
fn progress_bar_half() {
    let bar = progress_bar(0.5, 10);
    assert_eq!(bar, "█████░░░░░");
}

#[test]
fn progress_bar_clamps() {
    let over = progress_bar(2.0, 5);
    let under = progress_bar(-1.0, 5);
    assert_eq!(over, "█████");
    assert_eq!(under, "░░░░░");
}

// =============================================================================
// progress_bar_precise
// =============================================================================

#[test]
fn progress_bar_precise_full() {
    let bar = progress_bar_precise(1.0, 10);
    assert!(bar.contains('█'));
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
