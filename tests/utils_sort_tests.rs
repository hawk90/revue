//! Tests for natural sorting utilities
//!
//! Extracted from src/utils/sort.rs

use revue::utils::sort::{natural_cmp, natural_sort, NaturalKey};
use std::cmp::Ordering;

#[test]
fn test_natural_cmp_basic() {
    assert_eq!(natural_cmp("a", "b"), Ordering::Less);
    assert_eq!(natural_cmp("b", "a"), Ordering::Greater);
    assert_eq!(natural_cmp("a", "a"), Ordering::Equal);
}

#[test]
fn test_natural_cmp_numbers() {
    assert_eq!(natural_cmp("file1", "file2"), Ordering::Less);
    assert_eq!(natural_cmp("file2", "file10"), Ordering::Less);
    assert_eq!(natural_cmp("file10", "file2"), Ordering::Greater);
    assert_eq!(natural_cmp("file10", "file10"), Ordering::Equal);
}

#[test]
fn test_natural_cmp_mixed() {
    assert_eq!(natural_cmp("a1b2", "a1b10"), Ordering::Less);
    assert_eq!(natural_cmp("a10b2", "a2b10"), Ordering::Greater);
    assert_eq!(natural_cmp("item1", "item1a"), Ordering::Less);
}

#[test]
fn test_natural_cmp_case_insensitive() {
    assert_eq!(natural_cmp("File1", "file2"), Ordering::Less);
    assert_eq!(natural_cmp("FILE10", "file2"), Ordering::Greater);
    assert_eq!(natural_cmp("ABC", "abc"), Ordering::Equal);
}

#[test]
fn test_natural_cmp_empty() {
    assert_eq!(natural_cmp("", ""), Ordering::Equal);
    assert_eq!(natural_cmp("", "a"), Ordering::Less);
    assert_eq!(natural_cmp("a", ""), Ordering::Greater);
}

#[test]
fn test_natural_cmp_only_numbers() {
    assert_eq!(natural_cmp("1", "2"), Ordering::Less);
    assert_eq!(natural_cmp("10", "2"), Ordering::Greater);
    assert_eq!(natural_cmp("100", "20"), Ordering::Greater);
}

#[test]
fn test_natural_sort() {
    let mut files = vec!["file10.txt", "file2.txt", "file1.txt", "file20.txt"];
    natural_sort(&mut files);
    assert_eq!(
        files,
        vec!["file1.txt", "file2.txt", "file10.txt", "file20.txt"]
    );
}

#[test]
fn test_natural_sort_mixed() {
    let mut items = vec!["z1", "a10", "a2", "z10", "a1"];
    natural_sort(&mut items);
    assert_eq!(items, vec!["a1", "a2", "a10", "z1", "z10"]);
}

#[test]
fn test_natural_key() {
    let key1 = NaturalKey::new("file2");
    let key2 = NaturalKey::new("file10");
    assert!(key1 < key2);
}

#[test]
fn test_real_world_filenames() {
    let mut files = vec![
        "chapter1.md",
        "chapter10.md",
        "chapter2.md",
        "chapter11.md",
        "intro.md",
        "chapter3.md",
    ];
    natural_sort(&mut files);
    assert_eq!(
        files,
        vec![
            "chapter1.md",
            "chapter2.md",
            "chapter3.md",
            "chapter10.md",
            "chapter11.md",
            "intro.md",
        ]
    );
}

#[test]
fn test_version_numbers() {
    let mut versions = vec!["v1.10.0", "v1.2.0", "v1.9.0", "v2.0.0", "v1.1.0"];
    natural_sort(&mut versions);
    assert_eq!(
        versions,
        vec!["v1.1.0", "v1.2.0", "v1.9.0", "v1.10.0", "v2.0.0"]
    );
}

#[test]
fn test_leading_zeros() {
    let mut items = vec!["item001", "item01", "item1", "item10"];
    natural_sort(&mut items);
    // Items with same numeric value (001=01=1) are equal, stable sort preserves order
    assert_eq!(items, vec!["item001", "item01", "item1", "item10"]);
}
