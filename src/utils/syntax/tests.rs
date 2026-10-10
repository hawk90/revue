//! Unit tests for the crate-private Language tables
//!
//! The public highlighting API is tested in tests/utils/syntax/mod.rs.

use super::*;

#[test]
fn test_language_keywords() {
    // Rust keywords should include fn, let, etc.
    let keywords = Language::Rust.keywords();
    assert!(keywords.contains(&"fn"));
    assert!(keywords.contains(&"let"));
    assert!(keywords.contains(&"mut"));

    // Python keywords
    let keywords = Language::Python.keywords();
    assert!(keywords.contains(&"def"));
    assert!(keywords.contains(&"class"));
}

#[test]
fn test_language_types() {
    // Rust types should include i32, String, etc.
    let types = Language::Rust.types();
    assert!(types.contains(&"i32"));
    assert!(types.contains(&"String"));
    assert!(types.contains(&"Vec"));

    // Python types
    let types = Language::Python.types();
    assert!(types.contains(&"int"));
    assert!(types.contains(&"str"));
}

#[test]
fn test_language_unknown_has_no_keywords() {
    let keywords = Language::Unknown.keywords();
    assert!(keywords.is_empty());
}

#[test]
fn test_language_comment_patterns() {
    // Rust uses // and /* */
    let (line, block) = Language::Rust.comment_patterns();
    assert_eq!(line, "//");
    assert_eq!(block, Some(("/*", "*/")));

    // Python uses # only
    let (line, block) = Language::Python.comment_patterns();
    assert_eq!(line, "#");
    assert_eq!(block, None);

    // SQL uses -- and /* */
    let (line, block) = Language::Sql.comment_patterns();
    assert_eq!(line, "--");
    assert_eq!(block, Some(("/*", "*/")));

    // HTML uses <!-- --> only
    let (line, block) = Language::Html.comment_patterns();
    assert_eq!(line, "");
    assert_eq!(block, Some(("<!--", "-->")));
}
