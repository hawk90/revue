//! Slide parsing utilities for markdown presentations
//!
//! This module provides functionality to parse markdown content into slides,
//! using `---` as the slide delimiter (Slidev/Marp style).
//!
//! # Example
//!
//! ```rust
//! use revue::widget::slides::{parse_slides, SlideContent};
//!
//! let markdown = "# First Slide\n\nWelcome!\n\n---\n\n# Second Slide\n\n- Point 1\n- Point 2";
//!
//! let slides = parse_slides(markdown);
//! assert_eq!(slides.len(), 2);
//! assert_eq!(slides[0].title(), Some("First Slide"));
//! ```

use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};

/// Content of a single slide
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlideContent {
    /// Raw markdown content of the slide
    markdown: String,
    /// Extracted title (first H1 or H2)
    title: Option<String>,
    /// Speaker notes (from HTML comments)
    notes: Option<String>,
}

impl SlideContent {
    /// Create a new slide from markdown content
    pub fn new(markdown: impl Into<String>) -> Self {
        let markdown = markdown.into();
        let title = Self::extract_title(&markdown);
        let notes = Self::extract_notes(&markdown);

        Self {
            markdown,
            title,
            notes,
        }
    }

    /// Get the raw markdown content
    pub fn markdown(&self) -> &str {
        &self.markdown
    }

    /// Get the slide title (extracted from first H1/H2)
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    /// Get speaker notes (extracted from HTML comments)
    pub fn notes(&self) -> Option<&str> {
        self.notes.as_deref()
    }

    /// Check if the slide is empty
    pub fn is_empty(&self) -> bool {
        self.markdown.trim().is_empty()
    }

    /// Extract title from markdown (first H1 or H2 heading)
    fn extract_title(markdown: &str) -> Option<String> {
        let options = Options::empty();
        let parser = Parser::new_ext(markdown, options);

        let mut in_heading = false;
        let mut heading_level = 0u8;
        let mut title_text = String::new();

        for event in parser {
            match event {
                Event::Start(Tag::Heading { level, .. }) => {
                    let level_num = match level {
                        HeadingLevel::H1 => 1,
                        HeadingLevel::H2 => 2,
                        HeadingLevel::H3 => 3,
                        HeadingLevel::H4 => 4,
                        HeadingLevel::H5 => 5,
                        HeadingLevel::H6 => 6,
                    };
                    // Only capture H1 or H2 as title
                    if level_num <= 2 {
                        in_heading = true;
                        heading_level = level_num;
                        title_text.clear();
                    }
                }
                Event::End(TagEnd::Heading(_)) => {
                    if in_heading && heading_level <= 2 && !title_text.is_empty() {
                        return Some(title_text);
                    }
                    in_heading = false;
                }
                Event::Text(text) if in_heading => {
                    title_text.push_str(&text);
                }
                Event::Code(code) if in_heading => {
                    title_text.push_str(&code);
                }
                _ => {}
            }
        }

        None
    }

    /// Extract speaker notes from HTML comments
    ///
    /// Notes are expected in format: `<!-- notes: Your notes here -->`
    /// or multi-line:
    /// ```html
    /// <!--
    /// notes:
    /// - Point 1
    /// - Point 2
    /// -->
    /// ```
    fn extract_notes(markdown: &str) -> Option<String> {
        // Simple regex-free approach: find <!-- notes: ... -->
        let lower = markdown.to_lowercase();

        if let Some(start_idx) = lower.find("<!-- notes:") {
            let content_start = start_idx + "<!-- notes:".len();
            if let Some(end_offset) = markdown[content_start..].find("-->") {
                let notes = markdown[content_start..content_start + end_offset].trim();
                if !notes.is_empty() {
                    return Some(notes.to_string());
                }
            }
        }

        // Also try <!-- notes\n format
        if let Some(start_idx) = lower.find("<!--\nnotes:") {
            let content_start = start_idx + "<!--\nnotes:".len();
            if let Some(end_offset) = markdown[content_start..].find("-->") {
                let notes = markdown[content_start..content_start + end_offset].trim();
                if !notes.is_empty() {
                    return Some(notes.to_string());
                }
            }
        }

        None
    }
}

/// Parse markdown content into slides
///
/// Slides are separated by `---` (horizontal rule) on its own line.
/// Code blocks containing `---` are properly handled and won't split.
///
/// # Arguments
/// * `source` - The markdown source to parse
///
/// # Returns
/// A vector of `SlideContent`, one for each slide
pub fn parse_slides(source: &str) -> Vec<SlideContent> {
    let mut slides = Vec::new();
    let mut current = String::new();
    let mut in_code_block = false;
    let mut code_fence = String::new();

    for line in source.lines() {
        let trimmed = line.trim_start();

        // Track code blocks to avoid splitting on --- inside them
        if !in_code_block {
            if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
                in_code_block = true;
                code_fence = if trimmed.starts_with("```") {
                    "```".to_string()
                } else {
                    "~~~".to_string()
                };
            }
        } else if trimmed.starts_with(&code_fence) {
            in_code_block = false;
            code_fence.clear();
        }

        // Check for slide delimiter (only outside code blocks)
        if !in_code_block && is_slide_delimiter(line) {
            // Save current slide if not empty
            if !current.trim().is_empty() {
                slides.push(SlideContent::new(current.clone()));
            }
            current.clear();
            continue;
        }

        // Add line to current slide
        current.push_str(line);
        current.push('\n');
    }

    // Don't forget the last slide
    if !current.trim().is_empty() {
        slides.push(SlideContent::new(current));
    }

    slides
}

/// Check if a line is a slide delimiter
///
/// A slide delimiter is `---` (3 or more dashes) on its own line,
/// optionally surrounded by whitespace.
fn is_slide_delimiter(line: &str) -> bool {
    let trimmed = line.trim();

    // Must be only dashes (3 or more)
    if trimmed.len() < 3 {
        return false;
    }

    trimmed.chars().all(|c| c == '-')
}

/// State for slide navigation
#[derive(Debug, Clone, Default)]
pub struct SlideNav {
    /// All slides
    slides: Vec<SlideContent>,
    /// Current slide index
    current: usize,
}

impl SlideNav {
    /// Create a new slide navigator from markdown source
    pub fn new(source: &str) -> Self {
        let slides = parse_slides(source);
        Self { slides, current: 0 }
    }

    /// Create from pre-parsed slides
    pub fn from_slides(slides: Vec<SlideContent>) -> Self {
        Self { slides, current: 0 }
    }

    /// Get the current slide index (0-based)
    pub fn current_index(&self) -> usize {
        self.current
    }

    /// Get total number of slides
    pub fn slide_count(&self) -> usize {
        self.slides.len()
    }

    /// Get the current slide
    pub fn current_slide(&self) -> Option<&SlideContent> {
        self.slides.get(self.current)
    }

    /// Go to the next slide
    ///
    /// Returns `true` if navigation succeeded, `false` if already at last slide.
    pub fn advance(&mut self) -> bool {
        if self.current < self.slides.len().saturating_sub(1) {
            self.current += 1;
            true
        } else {
            false
        }
    }

    /// Go to the previous slide
    ///
    /// Returns `true` if navigation succeeded, `false` if already at first slide.
    pub fn prev(&mut self) -> bool {
        if self.current > 0 {
            self.current -= 1;
            true
        } else {
            false
        }
    }

    /// Go to a specific slide by index
    pub fn goto(&mut self, index: usize) {
        if index < self.slides.len() {
            self.current = index;
        }
    }

    /// Go to the first slide
    pub fn first(&mut self) {
        self.current = 0;
    }

    /// Go to the last slide
    pub fn last(&mut self) {
        self.current = self.slides.len().saturating_sub(1);
    }

    /// Get the slide indicator string (e.g., "3/10")
    pub fn indicator(&self) -> String {
        format!("{}/{}", self.current + 1, self.slides.len())
    }

    /// Get the slide indicator with brackets (e.g., "[3/10]")
    pub fn indicator_bracketed(&self) -> String {
        format!("[{}/{}]", self.current + 1, self.slides.len())
    }

    /// Check if at the first slide
    pub fn is_first(&self) -> bool {
        self.current == 0
    }

    /// Check if at the last slide
    pub fn is_last(&self) -> bool {
        self.current >= self.slides.len().saturating_sub(1)
    }

    /// Get all slides
    pub fn slides(&self) -> &[SlideContent] {
        &self.slides
    }

    /// Get progress as a fraction (0.0 to 1.0)
    pub fn progress(&self) -> f32 {
        if self.slides.is_empty() {
            return 0.0;
        }
        (self.current + 1) as f32 / self.slides.len() as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_slide() {
        let md = "# Hello\n\nWorld";
        let slides = parse_slides(md);

        assert_eq!(slides.len(), 1);
        assert_eq!(slides[0].title(), Some("Hello"));
    }

    #[test]
    fn test_parse_multiple_slides() {
        let md = r#"
# Slide 1

Content 1

---

# Slide 2

Content 2

---

# Slide 3

Content 3
"#;
        let slides = parse_slides(md);

        assert_eq!(slides.len(), 3);
        assert_eq!(slides[0].title(), Some("Slide 1"));
        assert_eq!(slides[1].title(), Some("Slide 2"));
        assert_eq!(slides[2].title(), Some("Slide 3"));
    }

    #[test]
    fn test_code_block_not_split() {
        let md = r#"
# Slide 1

```
---
This is not a slide break
---
```

Still slide 1

---

# Slide 2
"#;
        let slides = parse_slides(md);

        assert_eq!(slides.len(), 2);
        assert!(slides[0].markdown().contains("This is not a slide break"));
    }

    #[test]
    fn test_tilde_code_block() {
        let md = r#"
# Slide 1

~~~yaml
---
key: value
~~~

---

# Slide 2
"#;
        let slides = parse_slides(md);

        assert_eq!(slides.len(), 2);
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

    #[test]
    fn test_extract_title_h1() {
        let md = "# Main Title\n\nContent";
        let slide = SlideContent::new(md);
        assert_eq!(slide.title(), Some("Main Title"));
    }

    #[test]
    fn test_extract_title_h2() {
        let md = "## Subtitle\n\nContent";
        let slide = SlideContent::new(md);
        assert_eq!(slide.title(), Some("Subtitle"));
    }

    #[test]
    fn test_no_title() {
        let md = "Just some content\n\nMore content";
        let slide = SlideContent::new(md);
        assert_eq!(slide.title(), None);
    }

    #[test]
    fn test_extract_notes() {
        let md = r#"
# Title

Content

<!-- notes: This is a speaker note -->
"#;
        let slide = SlideContent::new(md);
        assert_eq!(slide.notes(), Some("This is a speaker note"));
    }

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
    fn test_empty_slides() {
        let md = "";
        let slides = parse_slides(md);
        assert!(slides.is_empty());
    }

    #[test]
    fn test_only_delimiters() {
        let md = "---\n---\n---";
        let slides = parse_slides(md);
        assert!(slides.is_empty());
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

    // =========================================================================
    // SlideContent struct tests
    // =========================================================================

    #[test]
    fn test_slide_content_new() {
        let slide = SlideContent::new("# Test\n\nContent");
        assert_eq!(slide.title(), Some("Test"));
        assert!(!slide.is_empty());
    }

    #[test]
    fn test_slide_content_empty() {
        let slide = SlideContent::new("");
        assert!(slide.is_empty());
    }

    #[test]
    fn test_slide_content_markdown_accessor() {
        let slide = SlideContent::new("# Title\n\nContent");
        assert!(slide.markdown().contains("Title"));
        assert!(slide.markdown().contains("Content"));
    }

    #[test]
    fn test_slide_content_no_notes() {
        let slide = SlideContent::new("# Title");
        assert!(slide.notes().is_none());
    }

    #[test]
    fn test_slide_content_clone() {
        let slide1 = SlideContent::new("# Test\n\nContent");
        let slide2 = slide1.clone();
        assert_eq!(slide1.title(), slide2.title());
    }

    #[test]
    fn test_slide_content_multi_line_notes() {
        let md = r#"
# Title

Content

<!--
notes:
- Point 1
- Point 2
-->
"#;
        let slide = SlideContent::new(md);
        assert!(slide.notes().is_some());
        let notes = slide.notes().unwrap();
        assert!(notes.contains("Point 1") || notes.contains("point 1"));
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
        assert!(slide.notes().is_some());
    }

    // =========================================================================
    // parse_slides function tests
    // =========================================================================

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
    fn test_parse_leading_delimiter() {
        let md = "---\n\n# First Slide";
        let slides = parse_slides(md);
        assert_eq!(slides.len(), 1);
        assert_eq!(slides[0].title(), Some("First Slide"));
    }

    #[test]
    fn test_parse_trailing_delimiter() {
        let md = "# Last Slide\n\n---";
        let slides = parse_slides(md);
        assert_eq!(slides.len(), 1);
    }

    #[test]
    fn test_parse_preserves_content() {
        let md = "# Title\n\n- Item 1\n- Item 2";
        let slides = parse_slides(md);
        assert!(slides[0].markdown().contains("Item 1"));
    }

    #[test]
    fn test_parse_h2_title() {
        let md = "## Subtitle\n\nContent";
        let slides = parse_slides(md);
        assert_eq!(slides[0].title(), Some("Subtitle"));
    }

    #[test]
    fn test_parse_h3_not_title() {
        let md = "### Not Title\n\nContent";
        let slides = parse_slides(md);
        // H3 is not extracted as title
        assert_eq!(slides[0].title(), None);
    }

    // =========================================================================
    // SlideNav struct tests
    // =========================================================================

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

    // =========================================================================
    // is_slide_delimiter tests
    // =========================================================================

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

    // =========================================================================
    // Edge case tests
    // =========================================================================

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
}
