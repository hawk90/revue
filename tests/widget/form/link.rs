//! Markdown output of links and image references at the edges
//!
//! new/with_title/to_markdown (with and without a title) are covered by
//! the in-source link and image_ref tests.

use revue::widget::{ImageRef, MarkdownLink as Link};

#[test]
fn test_link_to_markdown_with_special_chars() {
    let link = Link::new("Click (here)", "https://example.com?param=value");
    assert_eq!(
        link.to_markdown(),
        "[Click (here)](https://example.com?param=value)"
    );
}

#[test]
fn test_link_to_markdown_empty_text() {
    let link = Link::new("", "https://example.com");
    assert_eq!(link.to_markdown(), "[](https://example.com)");
}

#[test]
fn test_link_to_markdown_empty_url() {
    let link = Link::new("Text", "");
    assert_eq!(link.to_markdown(), "[Text]()");
}

#[test]
fn test_link_accepts_owned_strings() {
    let link = Link::new(String::from("Text"), String::from("https://example.com"))
        .with_title(String::from("Title"));
    assert_eq!(link.to_markdown(), "[Text](https://example.com \"Title\")");
}

#[test]
fn test_image_ref_to_markdown_with_special_chars() {
    let img = ImageRef::new("A (great) photo", "path/to/image.png");
    assert_eq!(img.to_markdown(), "![A (great) photo](path/to/image.png)");
}

#[test]
fn test_image_ref_to_markdown_empty_alt() {
    let img = ImageRef::new("", "image.png");
    assert_eq!(img.to_markdown(), "![](image.png)");
}

#[test]
fn test_image_ref_to_markdown_empty_src() {
    let img = ImageRef::new("Alt", "");
    assert_eq!(img.to_markdown(), "![Alt]()");
}

#[test]
fn test_image_ref_accepts_owned_strings() {
    let img = ImageRef::new(String::from("Photo"), String::from("photo.jpg"))
        .with_title(String::from("A photo"));
    assert_eq!(img.to_markdown(), "![Photo](photo.jpg \"A photo\")");
}
