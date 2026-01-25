//! Tests for ImageRef

use super::*;

#[test]
fn test_image_ref_new() {
    let img = ImageRef::new("Alt text", "/path/to/image.png");
    assert_eq!(img.alt, "Alt text");
    assert_eq!(img.src, "/path/to/image.png");
    assert!(img.title.is_none());
}

#[test]
fn test_image_ref_with_title() {
    let img = ImageRef::new("Alt", "img.png").with_title("Image Title");
    assert_eq!(img.title, Some("Image Title".to_string()));
}

#[test]
fn test_image_ref_to_markdown() {
    let img = ImageRef::new("Alt", "img.png");
    assert_eq!(img.to_markdown(), "![Alt](img.png)");
}

#[test]
fn test_image_ref_to_markdown_with_title() {
    let img = ImageRef::new("Alt", "img.png").with_title("Title");
    assert_eq!(img.to_markdown(), "![Alt](img.png \"Title\")");
}
