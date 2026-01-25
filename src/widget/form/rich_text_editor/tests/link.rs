//! Tests for Link

use super::*;

#[test]
fn test_link_new() {
    let link = Link::new("Example", "https://example.com");
    assert_eq!(link.text, "Example");
    assert_eq!(link.url, "https://example.com");
    assert!(link.title.is_none());
}

#[test]
fn test_link_with_title() {
    let link = Link::new("Example", "https://example.com").with_title("My Title");
    assert_eq!(link.title, Some("My Title".to_string()));
}

#[test]
fn test_link_to_markdown() {
    let link = Link::new("Example", "https://example.com");
    assert_eq!(link.to_markdown(), "[Example](https://example.com)");
}

#[test]
fn test_link_to_markdown_with_title() {
    let link = Link::new("Example", "https://example.com").with_title("Title");
    assert_eq!(
        link.to_markdown(),
        "[Example](https://example.com \"Title\")"
    );
}
