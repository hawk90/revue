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
#[doc(hidden)]
pub fn is_slide_delimiter(line: &str) -> bool {
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
