//! Diff property tests: a diff, applied to the old text, gives the new one
//!
//! Inputs over 10 000 lines or chars take the simplified large-input path,
//! so these cover it as well as the full LCS.

use proptest::prelude::*;
use revue::utils::diff::{diff_chars, diff_lines, DiffChange, DiffOp};

/// The (old, new) sides of a line diff
fn line_sides(changes: &[DiffChange]) -> (Vec<String>, Vec<String>) {
    let side = |keep: DiffOp| {
        changes
            .iter()
            .filter(|c| c.op == DiffOp::Equal || c.op == keep)
            .map(|c| c.text.clone())
            .collect()
    };
    (side(DiffOp::Delete), side(DiffOp::Insert))
}

/// The (old, new) sides of a char diff
fn char_sides(changes: &[DiffChange]) -> (String, String) {
    let side = |keep: DiffOp| {
        changes
            .iter()
            .filter(|c| c.op == DiffOp::Equal || c.op == keep)
            .map(|c| c.text.as_str())
            .collect()
    };
    (side(DiffOp::Delete), side(DiffOp::Insert))
}

fn lines(s: &str) -> Vec<String> {
    s.lines().map(String::from).collect()
}

/// A text of `len` short lines (or chars) over a small alphabet, so lines
/// repeat and match in many places
fn text(len: std::ops::Range<usize>) -> impl Strategy<Value = Vec<u8>> {
    prop::collection::vec(0u8..6, len)
}

fn as_lines(v: &[u8]) -> String {
    v.iter().map(|b| format!("l{b}\n")).collect()
}

fn as_chars(v: &[u8]) -> String {
    v.iter().map(|b| char::from(b'a' + b)).collect()
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(32))]

    #[test]
    fn diff_lines_reconstructs_small(old in text(0..60), new in text(0..60)) {
        let (old, new) = (as_lines(&old), as_lines(&new));
        let (o, n) = line_sides(&diff_lines(&old, &new));
        prop_assert_eq!(o, lines(&old));
        prop_assert_eq!(n, lines(&new));
    }

    #[test]
    fn diff_chars_reconstructs_small(old in text(0..60), new in text(0..60)) {
        let (old, new) = (as_chars(&old), as_chars(&new));
        let (o, n) = char_sides(&diff_chars(&old, &new));
        prop_assert_eq!(o, old);
        prop_assert_eq!(n, new);
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(8))]

    #[test]
    fn diff_lines_reconstructs_large(old in text(10_001..10_050), new in text(10_001..10_050)) {
        let (old, new) = (as_lines(&old), as_lines(&new));
        let (o, n) = line_sides(&diff_lines(&old, &new));
        prop_assert_eq!(o, lines(&old));
        prop_assert_eq!(n, lines(&new));
    }

    #[test]
    fn diff_chars_reconstructs_large(old in text(10_001..10_050), new in text(10_001..10_050)) {
        let (old, new) = (as_chars(&old), as_chars(&new));
        let (o, n) = char_sides(&diff_chars(&old, &new));
        prop_assert_eq!(o, old);
        prop_assert_eq!(n, new);
    }
}

/// Two swapped lines in a large input: the old pairing matched them
/// crosswise, which put the new-side index backwards
#[test]
fn diff_lines_large_swapped_lines() {
    let filler: String = (0..10_000).map(|i| format!("f{i}\n")).collect();
    let old = format!("b\na\n{filler}");
    let new = format!("a\nb\n{filler}");
    let (o, n) = line_sides(&diff_lines(&old, &new));
    assert_eq!(o, lines(&old));
    assert_eq!(n, lines(&new));
}
