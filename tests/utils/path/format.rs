//! Tests for path formatting utilities

use revue::utils::display_width;
use revue::utils::path::{
    abbreviate_path, abbreviate_path_keep, relative_to, shorten_path, PathDisplay,
};
use std::path::Path;

// =========================================================================
// shorten_path tests
// =========================================================================

#[test]
fn test_shorten_path_no_shortening_needed() {
    let path = "/usr/local/bin";
    let result = shorten_path(path, 100);
    assert_eq!(result, "/usr/local/bin");
}

#[test]
fn test_shorten_path_exact_fit() {
    let path = "/usr/bin";
    let result = shorten_path(path, 7);
    // Function adds "..." prefix when approaching limit
    assert!(result.contains("bin"));
}

#[test]
fn test_shorten_path_preserves_filename() {
    let path = "/very/long/path/to/file.txt";
    let result = shorten_path(path, 20);
    // Should include filename
    assert!(result.contains("file.txt"));
}

#[test]
fn test_shorten_path_with_ellipsis() {
    let path = "/a/b/c/d/e/f/g/h/file.txt";
    let result = shorten_path(path, 20);
    // Should start with "..."
    assert!(result.starts_with("..."));
}

#[test]
fn test_shorten_path_single_component() {
    let path = "file.txt";
    let result = shorten_path(path, 20);
    assert_eq!(result, "file.txt");
}

#[test]
fn test_shorten_path_empty() {
    let path = "";
    let result = shorten_path(path, 10);
    assert_eq!(result, "");
}

#[test]
fn test_shorten_path_keeps_wide_path_that_fits_in_columns() {
    // 12 columns, 15 bytes
    assert_eq!(shorten_path("/홈/파일.txt", 12), "/홈/파일.txt");
}

#[test]
fn test_shorten_path_keeps_wide_filename_whole_after_ellipsis() {
    // The 8-column filename plus ".../" is 12 columns
    let short = shorten_path("/홈/문서/파일.txt", 12);
    assert_eq!(short, ".../파일.txt");
    assert_eq!(display_width(&short), 12);
}

#[test]
fn test_shorten_path_one_column_short_keeps_wide_filename_after_dots() {
    // 11 columns: ".../파일.txt" is one too many, "...파일.txt" fits exactly
    assert_eq!(shorten_path("/홈/파일.txt", 11), "...파일.txt");
}

#[test]
fn test_shorten_path_cuts_wide_filename_by_columns() {
    // 10 columns leave 7 for the filename: "파일.tx"
    assert_eq!(shorten_path("/홈/파일.txt", 10), "...파일.tx");
}

#[test]
fn test_shorten_path_does_not_split_a_wide_char_at_the_edge() {
    // 8 columns leave 5 for the filename; the third Hangul char would need 6
    let short = shorten_path("/a/파일파일", 8);
    assert_eq!(short, "...파일");
    assert_eq!(display_width(&short), 7);
}

#[test]
fn test_path_display_max_width_keeps_wide_path_that_fits_in_columns() {
    let display = PathDisplay::new().tilde(false);
    assert_eq!(
        display.clone().max_width(12).format("/홈/파일.txt"),
        "/홈/파일.txt"
    );
    assert_eq!(display.max_width(11).format("/홈/파일.txt"), "...파일.txt");
}

// =========================================================================
// abbreviate_path tests
// =========================================================================

#[test]
fn test_abbreviate_path_short_path() {
    let path = "/usr/bin";
    let result = abbreviate_path(path);
    // Path is too short to abbreviate
    assert_eq!(result, "/usr/bin");
}

#[test]
fn test_abbreviate_path_long_path() {
    let path = "/Users/john/Documents/Projects/rust/main.rs";
    let result = abbreviate_path(path);
    // Should abbreviate middle components
    assert!(result.contains("/U/"));
    assert!(result.contains("main.rs"));
}

#[test]
fn test_abbreviate_path_no_leading_slash() {
    let path = "Users/john/Documents/file.txt";
    let result = abbreviate_path(path);
    // Should work without leading slash
    assert!(result.contains("file.txt"));
}

// =========================================================================
// abbreviate_path_keep tests
// =========================================================================

#[test]
fn test_abbreviate_path_keep_zero() {
    let path = "/a/b/c/d/e";
    let result = abbreviate_path_keep(path, 0);
    // Should abbreviate all but last
    assert!(result.contains("/e"));
}

#[test]
fn test_abbreviate_path_keep_one() {
    let path = "/a/b/c/d/e";
    let result = abbreviate_path_keep(path, 1);
    // Should keep only last component
    assert!(result.ends_with("/e"));
}

#[test]
fn test_abbreviate_path_keep_two() {
    let path = "/a/b/c/d/e";
    let result = abbreviate_path_keep(path, 2);
    // Should keep last two components
    assert!(result.contains("/d/e"));
}

#[test]
fn test_abbreviate_path_keep_all() {
    let path = "/a/b/c";
    let result = abbreviate_path_keep(path, 10);
    // Should keep all components when keep is large enough
    assert_eq!(result, "/a/b/c");
}

#[test]
fn test_abbreviate_path_keep_with_leading_slash() {
    let path = "/usr/local/bin";
    let result = abbreviate_path_keep(path, 1);
    assert!(result.starts_with('/'));
}

// =========================================================================
// relative_to tests
// =========================================================================

#[test]
fn test_relative_to_direct_child() {
    let base = Path::new("/home/user");
    let path = Path::new("/home/user/documents");
    let result = relative_to(path, base);
    assert_eq!(result, "documents");
}

#[test]
fn test_relative_to_nested() {
    let base = Path::new("/home/user");
    let path = Path::new("/home/user/projects/rust/app");
    let result = relative_to(path, base);
    assert_eq!(result, "projects/rust/app");
}

#[test]
fn test_relative_to_same_path() {
    let base = Path::new("/home/user");
    let path = Path::new("/home/user");
    let result = relative_to(path, base);
    assert_eq!(result, "");
}

#[test]
fn test_relative_to_no_common_prefix() {
    let base = Path::new("/home/user");
    let path = Path::new("/var/log");
    let result = relative_to(path, base);
    // Should return full path when no common prefix
    assert_eq!(result, "/var/log");
}

#[test]
fn test_relative_to_partial_overlap() {
    let base = Path::new("/home/user1");
    let path = Path::new("/home/user2");
    let result = relative_to(path, base);
    // No true common prefix, should return full path
    assert_eq!(result, "/home/user2");
}
