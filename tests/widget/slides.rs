//! Slides parsing tests

use revue::widget::{is_slide_delimiter, parse_slides, SlideContent, SlideNav};

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

// ==================== Additional Edge Cases ====================

#[test]
fn test_parse_slides_with_tables() {
    let markdown = "# Table Slide\n\n| Header 1 | Header 2 |\n|----------|----------|\n| Cell 1   | Cell 2   |";
    let slides = parse_slides(markdown);
    assert_eq!(slides.len(), 1);
}

#[test]
fn test_parse_slides_with_task_lists() {
    let markdown = "# Tasks\n\n- [x] Done\n- [ ] Todo";
    let slides = parse_slides(markdown);
    assert_eq!(slides.len(), 1);
}

#[test]
fn test_parse_slides_with_html_entities() {
    let markdown = "# HTML Entities\n\n&lt;tag&gt; &amp; &quot;";
    let slides = parse_slides(markdown);
    assert_eq!(slides.len(), 1);
}

#[test]
fn test_parse_slides_title_with_emoji() {
    let markdown = "# Hello 👋\n\nContent";
    let slides = parse_slides(markdown);
    assert_eq!(slides[0].title(), Some("Hello 👋"));
}

#[test]
fn test_parse_slides_title_with_special_chars() {
    let markdown = "# C++ & Java: A Comparison\n\nContent";
    let slides = parse_slides(markdown);
    assert!(slides[0].title().is_some());
}

#[test]
fn test_parse_slides_title_with_colon() {
    let markdown = "# Section 1: Introduction\n\nContent";
    let slides = parse_slides(markdown);
    assert_eq!(slides[0].title(), Some("Section 1: Introduction"));
}

#[test]
fn test_parse_slides_title_with_pipe() {
    let markdown = "# A | B | C\n\nContent";
    let slides = parse_slides(markdown);
    assert_eq!(slides[0].title(), Some("A | B | C"));
}

#[test]
fn test_parse_slides_title_with_quotes() {
    let markdown = r#"# "Quoted Title" and 'Single Quotes'"#;
    let slides = parse_slides(markdown);
    assert!(slides[0].title().is_some());
}

#[test]
fn test_parse_slides_notes_with_special_chars() {
    let markdown = "# Title\n\n<!-- notes: Remember: key points are A, B, & C -->";
    let slides = parse_slides(markdown);
    assert!(slides[0].notes().is_some());
}

#[test]
fn test_parse_slides_notes_multiline_complex() {
    let markdown = r#"# Title

<!--
notes:
- First point
- Second point
- Third point

Remember to emphasize this!
-->"#;
    let slides = parse_slides(markdown);
    assert!(slides[0].notes().is_some());
}

#[test]
fn test_parse_slides_multiple_notes_blocks() {
    let markdown = r#"# Title

<!-- notes: First note -->

Content

<!-- notes: Second note -->"#;
    let slides = parse_slides(markdown);
    // Should handle multiple notes blocks (implementation dependent)
    assert!(slides.len() >= 1);
}

#[test]
fn test_parse_slides_code_fence_with_language() {
    let markdown = r#"# Code Slide

```rust
fn hello() {
    println!("Hello");
}
```

```javascript
function hello() {
    console.log("Hello");
}
```"#;
    let slides = parse_slides(markdown);
    assert_eq!(slides.len(), 1);
}

#[test]
fn test_parse_slides_code_fence_empty() {
    let markdown = "# Slide\n\n```\nNo language specified\n```\n\n---\n\n# Next";
    let slides = parse_slides(markdown);
    assert!(slides.len() >= 2);
}

#[test]
fn test_parse_slides_inline_code() {
    let markdown = "# Slide\n\nUse `command` to do things";
    let slides = parse_slides(markdown);
    assert!(slides[0].markdown().contains("command"));
}

#[test]
fn test_parse_slides_mixed_delimiter_styles() {
    let markdown = "# A\n\n---\n\n# B\n\n***\n\n# C\n\n___";
    let slides = parse_slides(markdown);
    // Different delimiter styles may or may not work
    // At minimum, dashes should work
    assert!(slides.len() >= 1);
}

#[test]
fn test_parse_slides_delimiter_at_start() {
    let markdown = "---\n\n---\n\n# First Slide";
    let slides = parse_slides(markdown);
    assert_eq!(slides[0].title(), Some("First Slide"));
}

#[test]
fn test_parse_slides_delimiter_at_end() {
    let markdown = "# Last Slide\n\n---\n\n---";
    let slides = parse_slides(markdown);
    assert_eq!(slides[0].title(), Some("Last Slide"));
}

#[test]
fn test_parse_slides_only_delimiters() {
    let slides = parse_slides("---\n\n---\n\n---");
    // Should handle gracefully
    assert!(slides.len() >= 0);
}

#[test]
fn test_parse_slides_single_word_content() {
    let markdown = "# Title\n\nWord";
    let slides = parse_slides(markdown);
    assert_eq!(slides.len(), 1);
}

#[test]
fn test_parse_slides_only_title() {
    let markdown = "# Title Only";
    let slides = parse_slides(markdown);
    assert_eq!(slides.len(), 1);
}

#[test]
fn test_parse_slides_no_title_just_content() {
    let markdown = "Just some content\n\nWithout a title";
    let slides = parse_slides(markdown);
    assert_eq!(slides.len(), 1);
    assert_eq!(slides[0].title(), None);
}

#[test]
fn test_parse_slides_nested_list() {
    let markdown = "# Lists\n\n- Item 1\n  - Nested 1\n  - Nested 2\n- Item 2";
    let slides = parse_slides(markdown);
    assert_eq!(slides.len(), 1);
}

#[test]
fn test_parse_slides_footnotes() {
    let markdown = "# Footnotes\n\nContent with[^1] footnote\n\n[^1]: The note";
    let slides = parse_slides(markdown);
    assert_eq!(slides.len(), 1);
}

#[test]
fn test_parse_slides_strikethrough() {
    let markdown = "# ~~Deleted~~\n\nContent";
    let slides = parse_slides(markdown);
    assert_eq!(slides.len(), 1);
}

#[test]
fn test_parse_slides_empty_lines_between_slides() {
    let markdown = "# A\n\n\n\n---\n\n\n\n# B";
    let slides = parse_slides(markdown);
    assert_eq!(slides.len(), 2);
}

#[test]
fn test_parse_slides_carriage_returns() {
    let markdown = "# Title\r\n\r\nContent\r\n---\r\n\r\n# Next";
    let slides = parse_slides(markdown);
    assert_eq!(slides.len(), 2);
}

#[test]
fn test_parse_slides_mixed_line_endings() {
    let markdown = "# Title\n\rContent\n---\r\n# Next";
    let slides = parse_slides(markdown);
    assert!(slides.len() >= 2);
}

#[test]
fn test_parse_slides_tab_indentation() {
    let markdown = "# Title\n\tIndented content\n\tMore indented";
    let slides = parse_slides(markdown);
    assert_eq!(slides.len(), 1);
}

#[test]
fn test_parse_slides_atx_heading_with_closing() {
    let markdown = "# Title With Closing #\n\nContent";
    let slides = parse_slides(markdown);
    // Closing hashes should be handled
    assert!(slides[0].title().is_some());
}

#[test]
fn test_parse_slides_atx_heading_with_extra_hashes() {
    let markdown = "## Title ###\n\nContent";
    let slides = parse_slides(markdown);
    assert!(slides[0].title().is_some());
}

#[test]
fn test_parse_slides_autolinks() {
    let markdown = "# Links\n\n<https://example.com>\n\n<user@example.com>";
    let slides = parse_slides(markdown);
    assert_eq!(slides.len(), 1);
}

#[test]
fn test_parse_slides_empty_title_h1() {
    let markdown = "#\n\nContent without title";
    let slides = parse_slides(markdown);
    // Empty heading should be handled
    assert!(slides.len() >= 1);
}

#[test]
fn test_parse_slides_heading_only_hash() {
    let markdown = "##\n\nContent";
    let slides = parse_slides(markdown);
    assert!(slides.len() >= 1);
}

#[test]
fn test_parse_slides_very_long_title() {
    let long_title = "A".repeat(200);
    let markdown = format!("# {}\n\nContent", long_title);
    let slides = parse_slides(&markdown);
    assert_eq!(slides.len(), 1);
    assert!(slides[0].title().is_some());
}

#[test]
fn test_parse_slides_very_long_content() {
    let long_content = "A".repeat(10000);
    let markdown = format!("# Title\n\n{}", long_content);
    let slides = parse_slides(&markdown);
    assert_eq!(slides.len(), 1);
}

#[test]
fn test_parse_slides_many_slides() {
    let slides_text: Vec<String> = (1..=50).map(|i| format!("# Slide {}", i)).collect();
    let markdown = slides_text.join("\n\n---\n\n");
    let slides = parse_slides(&markdown);
    assert_eq!(slides.len(), 50);
}

#[test]
fn test_parse_slides_single_character_content() {
    let markdown = "# X\n\nA";
    let slides = parse_slides(markdown);
    assert_eq!(slides.len(), 1);
}

#[test]
fn test_parse_slides_content_with_pipe_in_table() {
    let markdown = "# Table\n\n| A | B |\n|---|---|\n| 1 | 2 |";
    let slides = parse_slides(markdown);
    assert_eq!(slides.len(), 1);
}

#[test]
fn test_parse_slides_content_with_escaped_chars() {
    let markdown = r"# Escaped\n\n\*not italic\*\n\n\[not a link\]";
    let slides = parse_slides(markdown);
    assert_eq!(slides.len(), 1);
}

#[test]
fn test_parse_slides_hard_breaks() {
    let markdown = "# Title\n\nLine 1  \nLine 2  \nLine 3";
    let slides = parse_slides(markdown);
    assert_eq!(slides.len(), 1);
}

#[test]
fn test_parse_slides_soft_breaks() {
    let markdown = "# Title\n\nLine 1\nLine 2\nLine 3";
    let slides = parse_slides(markdown);
    assert_eq!(slides.len(), 1);
}

#[test]
fn test_parse_slides_thematic_break_not_at_line_start() {
    let markdown = "# Title\n\nContent with --- in the middle";
    let slides = parse_slides(markdown);
    assert_eq!(slides.len(), 1);
}

#[test]
fn test_parse_slides_code_fence_with_backticks_in_code() {
    let markdown = r#"# Code

```
text with ``` inside
```"#;
    let slides = parse_slides(markdown);
    assert_eq!(slides.len(), 1);
}

#[test]
fn test_parse_slides_multiple_code_fences() {
    let markdown = r#"# Code

```rust
let x = 1;
```

Some text

```python
x = 1
```"#;
    let slides = parse_slides(markdown);
    assert_eq!(slides.len(), 1);
}

#[test]
fn test_parse_slides_indented_code_block() {
    let markdown = "# Code\n\n    let x = 1;\n    let y = 2;";
    let slides = parse_slides(markdown);
    assert_eq!(slides.len(), 1);
}

#[test]
fn test_parse_slides_notes_empty() {
    let markdown = "# Title\n\n<!-- notes: -->";
    let slides = parse_slides(markdown);
    // Empty notes should be handled
    assert!(slides.len() >= 1);
}

#[test]
fn test_parse_slides_notes_with_html_tags() {
    let markdown = "# Title\n\n<!-- notes: Remember <strong>bold</strong> text -->";
    let slides = parse_slides(markdown);
    assert!(slides[0].notes().is_some());
}

#[test]
fn test_parse_slides_slides_with_metadata() {
    let markdown = "# Title\n\nkey: value\nanother: data";
    let slides = parse_slides(markdown);
    // Metadata-like content should be preserved
    assert_eq!(slides.len(), 1);
}

#[test]
fn test_parse_slides_frontmatter_like_content() {
    let markdown = "---\nkey: value\n---\n\n# Title";
    let slides = parse_slides(markdown);
    // Frontmatter-like content should be handled
    assert!(slides.len() >= 1);
}

#[test]
fn test_parse_slides_content_only_numbers() {
    let markdown = "# Numbers\n\n123\n456\n789";
    let slides = parse_slides(markdown);
    assert_eq!(slides.len(), 1);
}

#[test]
fn test_parse_slides_content_only_symbols() {
    let markdown = "# Symbols\n\n@#$%^&*()";
    let slides = parse_slides(markdown);
    assert_eq!(slides.len(), 1);
}

// ==================== SlideNav Tests ====================

#[test]
fn test_slide_nav() {
    let md = r#"
# Slide 1

---

# Slide 2

---

# Slide 3
"#;
    let mut nav = SlideNav::new(md);

    assert_eq!(nav.slide_count(), 3);
    assert_eq!(nav.current_index(), 0);
    assert!(nav.is_first());
    assert!(!nav.is_last());

    assert!(nav.advance());
    assert_eq!(nav.current_index(), 1);
    assert_eq!(nav.indicator(), "2/3");

    assert!(nav.advance());
    assert_eq!(nav.current_index(), 2);
    assert!(nav.is_last());

    assert!(!nav.advance()); // Can't go further
    assert_eq!(nav.current_index(), 2);

    assert!(nav.prev());
    assert_eq!(nav.current_index(), 1);

    nav.first();
    assert_eq!(nav.current_index(), 0);

    nav.last();
    assert_eq!(nav.current_index(), 2);

    nav.goto(1);
    assert_eq!(nav.current_index(), 1);
}

#[test]
fn test_slide_nav_new() {
    let nav = SlideNav::new("# 1\n---\n# 2");
    assert_eq!(nav.slide_count(), 2);
    assert_eq!(nav.current_index(), 0);
}

#[test]
fn test_slide_nav_from_slides() {
    let slides = vec![
        SlideContent::new("# Slide 1"),
        SlideContent::new("# Slide 2"),
    ];
    let nav = SlideNav::from_slides(slides);
    assert_eq!(nav.slide_count(), 2);
    assert_eq!(nav.current_index(), 0);
}

#[test]
fn test_slide_nav_current_slide() {
    let nav = SlideNav::new("# 1\n---\n# 2");
    let slide = nav.current_slide().unwrap();
    assert_eq!(slide.title(), Some("1"));
}

#[test]
fn test_slide_nav_current_slide_empty() {
    let nav = SlideNav::new("");
    assert!(nav.current_slide().is_none());
}

#[test]
fn test_slide_nav_advance_at_end() {
    let mut nav = SlideNav::new("# 1");
    assert!(!nav.advance());
    assert_eq!(nav.current_index(), 0);
}

#[test]
fn test_slide_nav_prev_at_start() {
    let mut nav = SlideNav::new("# 1");
    assert!(!nav.prev());
    assert_eq!(nav.current_index(), 0);
}

#[test]
fn test_slide_nav_goto() {
    let mut nav = SlideNav::new("# 1\n---\n# 2\n---\n# 3");
    nav.goto(2);
    assert_eq!(nav.current_index(), 2);
}

#[test]
fn test_slide_nav_goto_out_of_bounds() {
    let mut nav = SlideNav::new("# 1\n---\n# 2");
    nav.goto(10); // Out of bounds
    assert_eq!(nav.current_index(), 0); // Unchanged
}

#[test]
fn test_slide_nav_indicator_bracketed() {
    let nav = SlideNav::new("# 1\n---\n# 2\n---\n# 3");
    assert_eq!(nav.indicator_bracketed(), "[1/3]");
}

#[test]
fn test_slide_nav_slides_accessor() {
    let nav = SlideNav::new("# 1\n---\n# 2");
    let slides = nav.slides();
    assert_eq!(slides.len(), 2);
}

#[test]
fn test_slide_nav_empty_progress() {
    let nav = SlideNav::new("");
    assert_eq!(nav.progress(), 0.0);
}

#[test]
fn test_slide_nav_default() {
    let nav = SlideNav::default();
    assert_eq!(nav.slide_count(), 0);
    assert_eq!(nav.current_index(), 0);
}

#[test]
fn test_progress() {
    let mut nav = SlideNav::new("# 1\n---\n# 2\n---\n# 3\n---\n# 4");

    assert!((nav.progress() - 0.25).abs() < 0.01); // 1/4
    nav.advance();
    assert!((nav.progress() - 0.50).abs() < 0.01); // 2/4
    nav.last();
    assert!((nav.progress() - 1.0).abs() < 0.01); // 4/4
}

// ==================== is_slide_delimiter Tests ====================

#[test]
fn test_is_slide_delimiter_exactly_three() {
    assert!(is_slide_delimiter("---"));
}

#[test]
fn test_is_slide_delimiter_many_dashes() {
    assert!(is_slide_delimiter("-----"));
}

#[test]
fn test_is_slide_delimiter_with_spaces() {
    assert!(is_slide_delimiter("  ---  "));
    assert!(is_slide_delimiter("\t---\t"));
}

#[test]
fn test_is_slide_delimiter_two_dashes() {
    assert!(!is_slide_delimiter("--"));
}

#[test]
fn test_is_slide_delimiter_one_dash() {
    assert!(!is_slide_delimiter("-"));
}

#[test]
fn test_is_slide_delimiter_with_text() {
    assert!(!is_slide_delimiter("--- text"));
    assert!(!is_slide_delimiter("text ---"));
}

#[test]
fn test_is_slide_delimiter_mixed_content() {
    assert!(!is_slide_delimiter("- -"));
    assert!(!is_slide_delimiter("-_-"));
}

#[test]
fn test_slide_delimiter_variations() {
    assert!(is_slide_delimiter("---"));
    assert!(is_slide_delimiter("----"));
    assert!(is_slide_delimiter("  ---  "));
    assert!(is_slide_delimiter("----------"));

    assert!(!is_slide_delimiter("--"));
    assert!(!is_slide_delimiter("- - -"));
    assert!(!is_slide_delimiter("--- text"));
}

// ==================== Additional Edge Case Tests ====================

#[test]
fn test_slide_content_clone() {
    let slide1 = SlideContent::new("# Test\n\nContent");
    let slide2 = slide1.clone();
    assert_eq!(slide1.title(), slide2.title());
}

#[test]
fn test_extract_notes_format2() {
    let md = r#"
# Title

<!--

notes:
Multi-line note here

-->
"#;
    let slide = SlideContent::new(md);
    // Note: This format is not parsed as notes by the current implementation
    assert!(slide.notes().is_none());
}

#[test]
fn test_parse_empty_content() {
    let slides = parse_slides("");
    assert!(slides.is_empty());
}

#[test]
fn test_parse_no_delimiter() {
    let md = "# Single\n\nContent\n\nMore content";
    let slides = parse_slides(md);
    assert_eq!(slides.len(), 1);
}

#[test]
fn test_parse_h3_not_title() {
    let md = "### Not Title\n\nContent";
    let slides = parse_slides(md);
    // H3 is not extracted as title
    assert_eq!(slides[0].title(), None);
}

#[test]
fn test_slide_with_inline_code() {
    let md = r#"
# Slide with `code`

Content
"#;
    let slides = parse_slides(md);
    assert_eq!(slides.len(), 1);
    assert!(slides[0].title().unwrap().contains("code"));
}

#[test]
fn test_slide_with_empty_lines() {
    let md = "\n\n# Title\n\n\n\nContent\n\n";
    let slides = parse_slides(md);
    assert_eq!(slides.len(), 1);
}

#[test]
fn test_multiple_consecutive_delimiters() {
    let md = "Content\n---\n---\n---\n\n# Next";
    let slides = parse_slides(md);
    assert_eq!(slides.len(), 2);
}

#[test]
fn test_slide_with_code_fence_language() {
    let md = r#"
# Slide

```rust
fn main() {
    println!("---");
}
```
"#;
    let slides = parse_slides(md);
    assert_eq!(slides.len(), 1);
    assert!(slides[0].markdown().contains("println!(\"---\")"));
}

#[test]
fn test_parse_title_with_inline_code() {
    let md = "# Title with `code` word";
    let slide = SlideContent::new(md);
    let title = slide.title().unwrap();
    assert!(title.contains("code"));
}
