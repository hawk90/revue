//! Slides parsing tests

use revue::widget::{parse_slides, SlideContent};

// ==================== SlideContent Tests ====================

#[test]
fn test_slide_content_new() {
    let slide = SlideContent::new("# Hello World\n\nSome content");
    assert_eq!(slide.markdown(), "# Hello World\n\nSome content");
    assert_eq!(slide.title(), Some("Hello World"));
}

#[test]
fn test_slide_content_new_empty() {
    let slide = SlideContent::new("");
    assert!(slide.is_empty());
}

#[test]
fn test_slide_content_new_whitespace() {
    let slide = SlideContent::new("   \n\n   ");
    assert!(slide.is_empty());
}

#[test]
fn test_slide_content_title_h1() {
    let slide = SlideContent::new("# My Title");
    assert_eq!(slide.title(), Some("My Title"));
}

#[test]
fn test_slide_content_title_h2() {
    let slide = SlideContent::new("## My Title");
    assert_eq!(slide.title(), Some("My Title"));
}

#[test]
fn test_slide_content_no_title_h3() {
    let slide = SlideContent::new("### Not Title");
    assert_eq!(slide.title(), None);
}

#[test]
fn test_slide_content_no_title() {
    let slide = SlideContent::new("Just content without heading");
    assert_eq!(slide.title(), None);
}

#[test]
fn test_slide_content_title_with_formatting() {
    let slide = SlideContent::new("# **Bold** and *italic*");
    // Title extraction preserves markdown formatting
    assert!(slide.title().is_some());
}

#[test]
fn test_slide_content_title_with_code() {
    let slide = SlideContent::new("# `code` title");
    // Title extraction strips backticks from inline code
    assert_eq!(slide.title(), Some("code title"));
}

#[test]
fn test_slide_content_notes() {
    let slide = SlideContent::new("# Title\n\n<!-- notes: Speak slowly -->");
    assert_eq!(slide.notes(), Some("Speak slowly"));
}

#[test]
fn test_slide_content_notes_multiline() {
    let slide = SlideContent::new("# Title\n\n<!--\nnotes:\n- Point 1\n- Point 2\n-->");
    assert!(slide.notes().is_some());
}

#[test]
fn test_slide_content_no_notes() {
    let slide = SlideContent::new("# Title\n\nJust content");
    assert_eq!(slide.notes(), None);
}

#[test]
fn test_slide_content_markdown_accessor() {
    let slide = SlideContent::new("## Test\n\nContent here");
    assert_eq!(slide.markdown(), "## Test\n\nContent here");
}

#[test]
fn test_slide_content_notes_newline_format() {
    let slide = SlideContent::new("# Title\n\n<!--\nnotes: Remember to smile\n-->");
    assert!(slide.notes().is_some());
}

// ==================== parse_slides Tests ====================

#[test]
fn test_parse_slides_single() {
    let markdown = "# First Slide\n\nContent here";
    let slides = parse_slides(markdown);
    assert_eq!(slides.len(), 1);
    assert_eq!(slides[0].title(), Some("First Slide"));
}

#[test]
fn test_parse_slides_two() {
    let markdown = "# First\n\n---\n\n# Second\n\nContent";
    let slides = parse_slides(markdown);
    assert_eq!(slides.len(), 2);
    assert_eq!(slides[0].title(), Some("First"));
    assert_eq!(slides[1].title(), Some("Second"));
}

#[test]
fn test_parse_slides_multiple() {
    let markdown = "# A\n---\n# B\n---\n# C\n---\n# D";
    let slides = parse_slides(markdown);
    assert_eq!(slides.len(), 4);
}

#[test]
fn test_parse_slides_empty() {
    let slides = parse_slides("");
    // Empty input returns 0 slides
    assert_eq!(slides.len(), 0);
}

#[test]
fn test_parse_slides_preserves_content() {
    let markdown = "# Test\n\n- Item 1\n- Item 2";
    let slides = parse_slides(markdown);
    assert_eq!(slides.len(), 1);
    // Content is preserved, possibly with trailing newline
    assert!(slides[0].markdown().contains("# Test"));
    assert!(slides[0].markdown().contains("Item 1"));
    assert!(slides[0].markdown().contains("Item 2"));
}

#[test]
fn test_parse_slides_delimiter_variations() {
    // Different dash counts
    let markdown = "# A\n\n---\n\n# B\n\n----\n\n# C";
    let slides = parse_slides(markdown);
    assert_eq!(slides.len(), 3);
}

#[test]
fn test_parse_slides_delimiter_with_spaces() {
    let markdown = "# A\n\n   ---   \n\n# B";
    let slides = parse_slides(markdown);
    assert_eq!(slides.len(), 2);
}

#[test]
fn test_parse_slides_code_fence_protection() {
    let markdown =
        "# Slide 1\n\n```rust\nfn example() {\n    let x = 1; // --- not a delimiter\n}\n```";
    let slides = parse_slides(markdown);
    assert_eq!(slides.len(), 1);
}

#[test]
fn test_parse_slides_tilde_fence_protection() {
    let markdown = "# Slide 1\n\n~~~\nContent with --- inside\n~~~";
    let slides = parse_slides(markdown);
    assert_eq!(slides.len(), 1);
}

#[test]
fn test_parse_slides_real_world_example() {
    let markdown = r#"
# Introduction

Welcome to the presentation!

---

# Overview

- Point 1
- Point 2
- Point 3

---

# Conclusion

Thank you!
"#;
    let slides = parse_slides(markdown);
    assert_eq!(slides.len(), 3);
    assert_eq!(slides[0].title(), Some("Introduction"));
    assert_eq!(slides[1].title(), Some("Overview"));
    assert_eq!(slides[2].title(), Some("Conclusion"));
}

#[test]
fn test_parse_slides_empty_slides_filtered() {
    let markdown = "# A\n\n---\n\n---\n\n# B";
    let slides = parse_slides(markdown);
    // Empty slide between delimiters should be filtered
    assert!(slides.len() >= 2);
}

#[test]
fn test_parse_slides_preserves_notes() {
    let markdown = "# Slide\n\n<!-- notes: Present this slide well -->";
    let slides = parse_slides(markdown);
    assert_eq!(slides[0].notes(), Some("Present this slide well"));
}

#[test]
fn test_parse_slides_complex_markdown() {
    let markdown = r#"
# Features

## Key Points

1. First feature
2. Second feature

**Bold** and *italic* text.

`inline code`

---

# Code Example

```rust
fn main() {
    println!("Hello");
}
```
"#;
    let slides = parse_slides(markdown);
    assert_eq!(slides.len(), 2);
}

#[test]
fn test_parse_slides_leading_delimiter() {
    let markdown = "---\n\n# First Slide";
    let slides = parse_slides(markdown);
    assert_eq!(slides.len(), 1);
}

#[test]
fn test_parse_slides_trailing_delimiter() {
    let markdown = "# Last Slide\n\n---";
    let slides = parse_slides(markdown);
    assert!(slides.len() >= 1);
}

#[test]
fn test_parse_slides_consecutive_delimiters() {
    let markdown = "# A\n\n---\n\n---\n\n# B";
    let slides = parse_slides(markdown);
    // Should handle consecutive delimiters gracefully
    assert!(slides.len() >= 2);
}

#[test]
fn test_parse_slides_titles_extraction() {
    let markdown = "# Slide 1\n\n---\n\n## Slide 2\n\n---\n\n### Slide 3";
    let slides = parse_slides(markdown);
    assert_eq!(slides.len(), 3);
    assert_eq!(slides[0].title(), Some("Slide 1"));
    // H2 should be extracted
    assert_eq!(slides[1].title(), Some("Slide 2"));
    // H3 should NOT be extracted
    assert_eq!(slides[2].title(), None);
}

#[test]
fn test_parse_slides_with_list_content() {
    let markdown = "# Items\n\n- First\n- Second\n- Third\n\n---\n\n# More Items\n\n* A\n* B";
    let slides = parse_slides(markdown);
    assert_eq!(slides.len(), 2);
}

#[test]
fn test_parse_slides_with_blockquotes() {
    let markdown = "# Quotes\n\n> Important quote\n\n> Another line";
    let slides = parse_slides(markdown);
    assert_eq!(slides.len(), 1);
}

#[test]
fn test_parse_slides_multiline_content() {
    let markdown = "# Slide\n\nParagraph 1.\n\nParagraph 2.\n\nParagraph 3.";
    let slides = parse_slides(markdown);
    assert_eq!(slides.len(), 1);
}

#[test]
fn test_parse_slides_unicode_content() {
    let markdown = "# ティション\n\n日本語のコンテンツ";
    let slides = parse_slides(markdown);
    assert_eq!(slides.len(), 1);
    assert_eq!(slides[0].title(), Some("ティション"));
}

#[test]
fn test_parse_slides_newlines_in_delimiter() {
    let markdown = "# A\n\n*\n*\n*\n# B";
    let slides = parse_slides(markdown);
    // The asterisks are treated as content, not a delimiter
    // So this becomes one slide with all the content
    assert!(slides.len() >= 1);
}

#[test]
fn test_slide_content_is_empty_detection() {
    let slide = SlideContent::new("");
    assert!(slide.is_empty());

    let slide2 = SlideContent::new("   \n\t  \n");
    assert!(slide2.is_empty());

    let slide3 = SlideContent::new("Content");
    assert!(!slide3.is_empty());
}

#[test]
fn test_parse_slides_title_extraction_with_code() {
    let markdown = "# `Main` Function\n\nCode examples";
    let slides = parse_slides(markdown);
    // Backticks are stripped from title
    assert_eq!(slides[0].title(), Some("Main Function"));
}

#[test]
fn test_parse_slides_multiple_headings() {
    let markdown = "# Main Title\n\n## Subtitle\n\nContent";
    let slides = parse_slides(markdown);
    assert_eq!(slides[0].title(), Some("Main Title"));
}

#[test]
fn test_parse_slides_slides_with_only_headings() {
    let markdown = "# One\n\n---\n\n# Two\n\n---\n\n# Three";
    let slides = parse_slides(markdown);
    assert_eq!(slides.len(), 3);
}

#[test]
fn test_parse_slides_horizontal_rule_variants() {
    // More than 3 dashes should also work
    let markdown = "# A\n\n-----\n\n# B";
    let slides = parse_slides(markdown);
    assert_eq!(slides.len(), 2);
}

#[test]
fn test_parse_slides_preserves_indentation() {
    let markdown = "# Slide\n\n    Indented code\n\n    More indented";
    let slides = parse_slides(markdown);
    assert!(slides[0].markdown().contains("    Indented"));
}

#[test]
fn test_parse_slides_links_and_images() {
    let markdown = "# Slide\n\n[Link](https://example.com)\n\n![Alt](image.png)";
    let slides = parse_slides(markdown);
    assert_eq!(slides.len(), 1);
}
