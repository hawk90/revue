//! Markdown Presentation widget
//!
//! A combined widget that renders markdown with slide support,
//! supporting both preview mode (scrollable) and presentation mode (one slide at a time).
//!
//! # Features
//!
//! - **Slide parsing**: Uses `---` (horizontal rule) as slide delimiter
//! - **Header sizing**: Uses Kitty Text Sizing Protocol (OSC 66) when available
//! - **Two viewing modes**: Preview (full scroll) and Slides (one at a time)
//! - **Navigation**: Arrow keys, vim keys, or programmatic control
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::prelude::*;
//!
//! let markdown = r#"
//! # Title Slide
//!
//! Welcome to my presentation!
//!
//! ---
//!
//! ## Slide 2
//!
//! - Point 1
//! - Point 2
//!
//! ---
//!
//! ## Conclusion
//!
//! Thank you!
//! "#;
//!
//! let mut pres = MarkdownPresentation::new(markdown);
//! pres.next_slide();
//! pres.toggle_mode();
//! ```

use super::bigtext::BigText;
use super::markdown::Markdown;
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::utils::figlet::FigletFont;
use crate::utils::text_sizing::is_supported as text_sizing_supported;
use crate::widget::slides::{SlideContent, SlideNav};
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// Viewing mode for markdown presentation
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ViewMode {
    /// Full scrollable preview of all content
    #[default]
    Preview,
    /// One slide at a time (presentation mode)
    Slides,
}

/// A widget for rendering markdown as a slideshow
///
/// Combines markdown rendering with slide navigation, supporting both
/// preview mode and presentation mode.
#[derive(Debug, Clone)]
pub struct MarkdownPresentation {
    /// Original markdown source
    source: String,
    /// Parsed slides
    nav: SlideNav,
    /// Current viewing mode
    mode: ViewMode,
    /// Scroll offset for preview mode
    scroll_offset: usize,
    /// Use text sizing protocol for headers
    use_text_sizing: bool,
    /// Figlet font for header fallback
    figlet_font: FigletFont,
    /// Background color
    bg: Color,
    /// Accent color (for headers, links, etc.)
    accent: Color,
    /// Show slide numbers in presentation mode
    show_numbers: bool,
    /// Show progress bar
    show_progress: bool,
    /// Heading color
    heading_fg: Color,
    /// Link color
    link_fg: Color,
    /// Code color
    code_fg: Color,
    /// Widget properties
    props: WidgetProps,
}

impl MarkdownPresentation {
    /// Create a new markdown presentation
    pub fn new(source: impl Into<String>) -> Self {
        let source = source.into();
        let nav = SlideNav::new(&source);

        Self {
            source,
            nav,
            mode: ViewMode::Preview,
            scroll_offset: 0,
            use_text_sizing: text_sizing_supported(),
            figlet_font: FigletFont::Block,
            bg: Color::rgb(20, 20, 30),
            accent: Color::CYAN,
            show_numbers: true,
            show_progress: true,
            heading_fg: Color::WHITE,
            link_fg: Color::CYAN,
            code_fg: Color::YELLOW,
            props: WidgetProps::new(),
        }
    }

    /// Create from pre-parsed slides
    pub fn from_slides(slides: Vec<SlideContent>) -> Self {
        let source = slides
            .iter()
            .map(|s| s.markdown().to_string())
            .collect::<Vec<_>>()
            .join("\n---\n");
        let nav = SlideNav::from_slides(slides);

        Self {
            source,
            nav,
            mode: ViewMode::Preview,
            scroll_offset: 0,
            use_text_sizing: text_sizing_supported(),
            figlet_font: FigletFont::Block,
            bg: Color::rgb(20, 20, 30),
            accent: Color::CYAN,
            show_numbers: true,
            show_progress: true,
            heading_fg: Color::WHITE,
            link_fg: Color::CYAN,
            code_fg: Color::YELLOW,
            props: WidgetProps::new(),
        }
    }

    /// Enable or disable text sizing protocol
    ///
    /// When enabled, uses Kitty's OSC 66 protocol for header rendering.
    /// When disabled (or unsupported), falls back to Figlet ASCII art.
    pub fn text_sizing(mut self, enable: bool) -> Self {
        self.use_text_sizing = enable && text_sizing_supported();
        self
    }

    /// Set the Figlet font for header fallback
    pub fn figlet_font(mut self, font: FigletFont) -> Self {
        self.figlet_font = font;
        self
    }

    /// Set background color
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = color;
        self
    }

    /// Set accent color
    pub fn accent(mut self, color: Color) -> Self {
        self.accent = color;
        self
    }

    /// Set heading color
    pub fn heading_fg(mut self, color: Color) -> Self {
        self.heading_fg = color;
        self
    }

    /// Set link color
    pub fn link_fg(mut self, color: Color) -> Self {
        self.link_fg = color;
        self
    }

    /// Set code color
    pub fn code_fg(mut self, color: Color) -> Self {
        self.code_fg = color;
        self
    }

    /// Show/hide slide numbers
    pub fn numbers(mut self, show: bool) -> Self {
        self.show_numbers = show;
        self
    }

    /// Show/hide progress bar
    pub fn progress(mut self, show: bool) -> Self {
        self.show_progress = show;
        self
    }

    /// Set the viewing mode
    pub fn mode(mut self, mode: ViewMode) -> Self {
        self.mode = mode;
        self
    }

    /// Get the current viewing mode
    pub fn current_mode(&self) -> ViewMode {
        self.mode
    }

    /// Toggle between preview and slide mode
    pub fn toggle_mode(&mut self) {
        self.mode = match self.mode {
            ViewMode::Preview => ViewMode::Slides,
            ViewMode::Slides => ViewMode::Preview,
        };
    }

    /// Go to the next slide
    ///
    /// Returns `true` if navigation succeeded.
    pub fn next_slide(&mut self) -> bool {
        self.nav.advance()
    }

    /// Go to the previous slide
    ///
    /// Returns `true` if navigation succeeded.
    pub fn prev_slide(&mut self) -> bool {
        self.nav.prev()
    }

    /// Go to a specific slide by index
    pub fn goto(&mut self, index: usize) {
        self.nav.goto(index);
    }

    /// Go to the first slide
    pub fn first(&mut self) {
        self.nav.first();
    }

    /// Go to the last slide
    pub fn last(&mut self) {
        self.nav.last();
    }

    /// Get the current slide index (0-based)
    pub fn current_index(&self) -> usize {
        self.nav.current_index()
    }

    /// Get the total number of slides
    pub fn slide_count(&self) -> usize {
        self.nav.slide_count()
    }

    /// Get the current slide
    pub fn current_slide(&self) -> Option<&SlideContent> {
        self.nav.current_slide()
    }

    /// Get the current slide's speaker notes
    pub fn current_notes(&self) -> Option<&str> {
        self.nav.current_slide().and_then(|s| s.notes())
    }

    /// Get the slide indicator string (e.g., "3/10")
    pub fn indicator(&self) -> String {
        self.nav.indicator()
    }

    /// Get the slide indicator with brackets (e.g., "[3/10]")
    pub fn indicator_bracketed(&self) -> String {
        self.nav.indicator_bracketed()
    }

    /// Get progress as a fraction (0.0 to 1.0)
    pub fn progress_value(&self) -> f32 {
        self.nav.progress()
    }

    /// Check if at the first slide
    pub fn is_first(&self) -> bool {
        self.nav.is_first()
    }

    /// Check if at the last slide
    pub fn is_last(&self) -> bool {
        self.nav.is_last()
    }

    /// Get all slides
    pub fn slides(&self) -> &[SlideContent] {
        self.nav.slides()
    }

    /// Get the original markdown source
    pub fn source(&self) -> &str {
        &self.source
    }

    /// Scroll up in preview mode
    pub fn scroll_up(&mut self, lines: usize) {
        self.scroll_offset = self.scroll_offset.saturating_sub(lines);
    }

    /// Scroll down in preview mode
    pub fn scroll_down(&mut self, lines: usize) {
        self.scroll_offset = self.scroll_offset.saturating_add(lines);
    }

    /// Reset scroll position
    pub fn scroll_to_top(&mut self) {
        self.scroll_offset = 0;
    }

    /// Reload from new source
    pub fn reload(&mut self, source: impl Into<String>) {
        self.source = source.into();
        self.nav = SlideNav::new(&self.source);
        self.scroll_offset = 0;
    }

    /// Render preview mode (full scrollable content)
    fn render_preview(&self, ctx: &mut RenderContext) {
        // Fill background
        self.fill_background(ctx);

        // Render full markdown using Markdown widget
        let md = Markdown::new(&self.source)
            .link_fg(self.link_fg)
            .code_fg(self.code_fg)
            .heading_fg(self.heading_fg);

        md.render(ctx);

        // Mode indicator in bottom right
        self.render_mode_indicator(ctx, "PREVIEW");
    }

    /// Render slide mode (one slide at a time)
    fn render_slide(&self, ctx: &mut RenderContext) {
        let area = ctx.area;

        // Fill background
        self.fill_background(ctx);

        if let Some(slide) = self.nav.current_slide() {
            // Render title with BigText if present
            let mut content_start_y = 0u16;

            if let Some(title) = slide.title() {
                let bt = BigText::new(title, 1)
                    .fg(self.heading_fg)
                    .figlet_font(self.figlet_font)
                    .force_figlet(!self.use_text_sizing);

                let title_height = bt.height();

                // Create sub-area for title
                let title_area = crate::layout::Rect::new(
                    area.x,
                    area.y + 1,
                    area.width,
                    title_height.min(area.height.saturating_sub(1)),
                );

                let mut title_ctx = RenderContext::new(ctx.buffer, title_area);
                bt.render(&mut title_ctx);

                content_start_y = title_height + 2;

                // Separator line
                let sep_y = area.y + content_start_y;
                if sep_y < area.y + area.height {
                    let sep_len = (area.width as usize).min(title.len() * 2).max(20);
                    let sep_start = (area.width as usize - sep_len) / 2;
                    for i in 0..sep_len {
                        let mut cell = Cell::new('─');
                        cell.fg = Some(self.accent);
                        ctx.buffer
                            .set(area.x + sep_start as u16 + i as u16, sep_y, cell);
                    }
                    content_start_y += 2;
                }
            }

            // Render content (markdown without the title)
            let content = self.strip_title(slide.markdown());
            if !content.trim().is_empty() {
                let content_area = crate::layout::Rect::new(
                    area.x + 2,
                    area.y + content_start_y,
                    area.width.saturating_sub(4),
                    area.height.saturating_sub(content_start_y + 2),
                );

                let md = Markdown::new(&content)
                    .link_fg(self.link_fg)
                    .code_fg(self.code_fg)
                    .heading_fg(self.heading_fg);

                let mut content_ctx = RenderContext::new(ctx.buffer, content_area);
                md.render(&mut content_ctx);
            }
        }

        // Footer
        self.render_footer(ctx);
    }

    /// Strip the first heading from markdown content
    fn strip_title(&self, markdown: &str) -> String {
        let mut lines = markdown.lines().peekable();
        let mut result = String::new();
        let mut skipped_title = false;

        while let Some(line) = lines.next() {
            // Skip first H1/H2 heading
            if !skipped_title
                && (line.trim_start().starts_with("# ") || line.trim_start().starts_with("## "))
            {
                skipped_title = true;
                // Skip any immediately following blank lines
                while lines.peek().is_some_and(|l| l.trim().is_empty()) {
                    lines.next();
                }
                continue;
            }
            result.push_str(line);
            result.push('\n');
        }

        result
    }

    /// Fill the background with the background color
    fn fill_background(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        for y in 0..area.height {
            for x in 0..area.width {
                let mut cell = Cell::new(' ');
                cell.bg = Some(self.bg);
                ctx.buffer.set(area.x + x, area.y + y, cell);
            }
        }
    }

    /// Render mode indicator
    fn render_mode_indicator(&self, ctx: &mut RenderContext, mode_text: &str) {
        let area = ctx.area;
        let text = format!(" {} ", mode_text);
        let start_x = area.x + area.width - text.len() as u16 - 1;
        let y = area.y + 1;

        for (i, ch) in text.chars().enumerate() {
            let mut cell = Cell::new(ch);
            cell.fg = Some(Color::BLACK);
            cell.bg = Some(self.accent);
            cell.modifier = Modifier::BOLD;
            ctx.buffer.set(start_x + i as u16, y, cell);
        }
    }

    /// Render footer with slide numbers and progress
    fn render_footer(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let footer_y = area.y + area.height - 1;

        // Slide numbers
        if self.show_numbers && self.nav.slide_count() > 0 {
            let num_str = self.nav.indicator();
            let start_x = area.x + area.width - num_str.len() as u16 - 1;
            for (i, ch) in num_str.chars().enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(Color::rgb(100, 100, 100));
                ctx.buffer.set(start_x + i as u16, footer_y, cell);
            }
        }

        // Progress bar
        if self.show_progress && self.nav.slide_count() > 0 {
            let bar_width = (area.width / 3).max(10);
            let progress = self.nav.progress();
            let filled = (bar_width as f32 * progress) as u16;

            for i in 0..bar_width {
                let ch = if i < filled { '━' } else { '─' };
                let mut cell = Cell::new(ch);
                cell.fg = Some(if i < filled {
                    self.accent
                } else {
                    Color::rgb(60, 60, 60)
                });
                ctx.buffer.set(area.x + 1 + i, footer_y, cell);
            }
        }

        // Mode indicator
        let mode_str = match self.mode {
            ViewMode::Preview => "[P]",
            ViewMode::Slides => "[S]",
        };
        let mode_x = area.x + area.width / 2 - 1;
        for (i, ch) in mode_str.chars().enumerate() {
            let mut cell = Cell::new(ch);
            cell.fg = Some(Color::rgb(80, 80, 80));
            ctx.buffer.set(mode_x + i as u16, footer_y, cell);
        }
    }
}

impl Default for MarkdownPresentation {
    fn default() -> Self {
        Self::new("")
    }
}

impl View for MarkdownPresentation {
    crate::impl_view_meta!("MarkdownPresentation");

    fn render(&self, ctx: &mut RenderContext) {
        if ctx.area.width == 0 || ctx.area.height == 0 {
            return;
        }

        match self.mode {
            ViewMode::Preview => self.render_preview(ctx),
            ViewMode::Slides => self.render_slide(ctx),
        }
    }
}

impl_styled_view!(MarkdownPresentation);
impl_props_builders!(MarkdownPresentation);

/// Create a new markdown presentation
pub fn markdown_presentation(source: impl Into<String>) -> MarkdownPresentation {
    MarkdownPresentation::new(source)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    #[test]
    fn test_creation() {
        let pres = MarkdownPresentation::new("# Hello\n\n---\n\n# World");
        assert_eq!(pres.slide_count(), 2);
        assert_eq!(pres.current_index(), 0);
    }

    #[test]
    fn test_navigation() {
        let mut pres = MarkdownPresentation::new("# A\n---\n# B\n---\n# C");

        assert_eq!(pres.current_index(), 0);
        assert!(pres.is_first());
        assert!(!pres.is_last());

        assert!(pres.next_slide());
        assert_eq!(pres.current_index(), 1);

        assert!(pres.next_slide());
        assert_eq!(pres.current_index(), 2);
        assert!(pres.is_last());

        assert!(!pres.next_slide()); // Can't go past last
        assert_eq!(pres.current_index(), 2);

        pres.first();
        assert_eq!(pres.current_index(), 0);

        pres.last();
        assert_eq!(pres.current_index(), 2);

        pres.goto(1);
        assert_eq!(pres.current_index(), 1);
    }

    #[test]
    fn test_mode_toggle() {
        let mut pres = MarkdownPresentation::new("# Test");

        assert_eq!(pres.current_mode(), ViewMode::Preview);

        pres.toggle_mode();
        assert_eq!(pres.current_mode(), ViewMode::Slides);

        pres.toggle_mode();
        assert_eq!(pres.current_mode(), ViewMode::Preview);
    }

    #[test]
    fn test_builder_pattern() {
        let pres = MarkdownPresentation::new("# Test")
            .bg(Color::BLACK)
            .accent(Color::GREEN)
            .heading_fg(Color::CYAN)
            .numbers(false)
            .progress(false)
            .mode(ViewMode::Slides);

        assert_eq!(pres.current_mode(), ViewMode::Slides);
        assert_eq!(pres.bg, Color::BLACK);
        assert_eq!(pres.accent, Color::GREEN);
    }

    #[test]
    fn test_indicator() {
        let pres = MarkdownPresentation::new("# A\n---\n# B\n---\n# C");
        assert_eq!(pres.indicator(), "1/3");
        assert_eq!(pres.indicator_bracketed(), "[1/3]");
    }

    #[test]
    fn test_render_preview() {
        let pres = MarkdownPresentation::new("# Hello\n\nWorld").mode(ViewMode::Preview);

        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        pres.render(&mut ctx);
        // Should not crash
    }

    #[test]
    fn test_render_slides() {
        let pres = MarkdownPresentation::new("# Slide 1\n\nContent\n---\n# Slide 2")
            .mode(ViewMode::Slides);

        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        pres.render(&mut ctx);
        // Should not crash
    }

    #[test]
    fn test_current_notes() {
        let md = "# Title\n\nContent\n\n<!-- notes: Speaker note here -->";
        let pres = MarkdownPresentation::new(md);

        assert_eq!(pres.current_notes(), Some("Speaker note here"));
    }

    #[test]
    fn test_reload() {
        let mut pres = MarkdownPresentation::new("# A\n---\n# B");
        assert_eq!(pres.slide_count(), 2);

        pres.next_slide();
        assert_eq!(pres.current_index(), 1);

        pres.reload("# X\n---\n# Y\n---\n# Z");
        assert_eq!(pres.slide_count(), 3);
        assert_eq!(pres.current_index(), 0); // Reset after reload
    }

    #[test]
    fn test_scroll() {
        let mut pres = MarkdownPresentation::new("# Test");

        assert_eq!(pres.scroll_offset, 0);

        pres.scroll_down(5);
        assert_eq!(pres.scroll_offset, 5);

        pres.scroll_up(2);
        assert_eq!(pres.scroll_offset, 3);

        pres.scroll_to_top();
        assert_eq!(pres.scroll_offset, 0);
    }

    #[test]
    fn test_progress() {
        let pres = MarkdownPresentation::new("# A\n---\n# B\n---\n# C\n---\n# D");
        assert!((pres.progress_value() - 0.25).abs() < 0.01);
    }

    #[test]
    fn test_strip_title() {
        let pres = MarkdownPresentation::new("");

        let content = "# Title\n\nContent here";
        let stripped = pres.strip_title(content);
        assert_eq!(stripped.trim(), "Content here");

        let content2 = "## Subtitle\n\nMore content";
        let stripped2 = pres.strip_title(content2);
        assert_eq!(stripped2.trim(), "More content");
    }

    // =========================================================================
    // ViewMode enum tests
    // =========================================================================

    #[test]
    fn test_view_mode_default() {
        let mode = ViewMode::default();
        assert_eq!(mode, ViewMode::Preview);
    }

    #[test]
    fn test_view_mode_clone() {
        let mode = ViewMode::Slides;
        let cloned = mode;
        assert_eq!(mode, cloned);
    }

    #[test]
    fn test_view_mode_copy() {
        let mode1 = ViewMode::Slides;
        let mode2 = mode1;
        assert_eq!(mode1, ViewMode::Slides);
        assert_eq!(mode2, ViewMode::Slides);
    }

    #[test]
    fn test_view_mode_partial_eq() {
        assert_eq!(ViewMode::Preview, ViewMode::Preview);
        assert_ne!(ViewMode::Preview, ViewMode::Slides);
    }

    // =========================================================================
    // MarkdownPresentation::from_slides tests
    // =========================================================================

    #[test]
    fn test_from_slides() {
        use crate::widget::slides::SlideContent;

        let slides = vec![
            SlideContent::new("# Slide 1"),
            SlideContent::new("# Slide 2"),
        ];

        let pres = MarkdownPresentation::from_slides(slides);
        assert_eq!(pres.slide_count(), 2);
        assert_eq!(pres.source(), "# Slide 1\n---\n# Slide 2");
    }

    #[test]
    fn test_from_slides_empty() {
        let pres = MarkdownPresentation::from_slides(vec![]);
        assert_eq!(pres.slide_count(), 0);
    }

    // =========================================================================
    // MarkdownPresentation::text_sizing tests
    // =========================================================================

    #[test]
    fn test_text_sizing() {
        let pres = MarkdownPresentation::new("# Test").text_sizing(true);
        // The actual value depends on terminal support
        // Just verify the method doesn't panic
    }

    #[test]
    fn test_text_sizing_disabled() {
        let pres = MarkdownPresentation::new("# Test").text_sizing(false);
        assert!(!pres.use_text_sizing);
    }

    // =========================================================================
    // MarkdownPresentation::figlet_font tests
    // =========================================================================

    #[test]
    fn test_figlet_font() {
        let pres = MarkdownPresentation::new("# Test").figlet_font(FigletFont::Small);
        assert_eq!(pres.figlet_font, FigletFont::Small);
    }

    // =========================================================================
    // MarkdownPresentation::link_fg tests
    // =========================================================================

    #[test]
    fn test_link_fg() {
        let pres = MarkdownPresentation::new("# Test").link_fg(Color::RED);
        assert_eq!(pres.link_fg, Color::RED);
    }

    // =========================================================================
    // MarkdownPresentation::code_fg tests
    // =========================================================================

    #[test]
    fn test_code_fg() {
        let pres = MarkdownPresentation::new("# Test").code_fg(Color::GREEN);
        assert_eq!(pres.code_fg, Color::GREEN);
    }

    // =========================================================================
    // MarkdownPresentation::current_slide tests
    // =========================================================================

    #[test]
    fn test_current_slide() {
        let pres = MarkdownPresentation::new("# A\n---\n# B");
        assert!(pres.current_slide().is_some());
        assert_eq!(pres.current_slide().unwrap().markdown(), "# A");
    }

    #[test]
    fn test_current_slide_empty() {
        let pres = MarkdownPresentation::new("");
        assert!(pres.current_slide().is_none());
    }

    // =========================================================================
    // MarkdownPresentation::slides tests
    // =========================================================================

    #[test]
    fn test_slides() {
        let pres = MarkdownPresentation::new("# A\n---\n# B\n---\n# C");
        assert_eq!(pres.slides().len(), 3);
    }

    #[test]
    fn test_slides_empty() {
        let pres = MarkdownPresentation::new("");
        assert_eq!(pres.slides().len(), 1); // Single slide with empty content
    }

    // =========================================================================
    // MarkdownPresentation::source tests
    // =========================================================================

    #[test]
    fn test_source() {
        let markdown = "# Hello\n\nWorld";
        let pres = MarkdownPresentation::new(markdown);
        assert_eq!(pres.source(), markdown);
    }

    // =========================================================================
    // MarkdownPresentation::prev_slide tests
    // =========================================================================

    #[test]
    fn test_prev_slide() {
        let mut pres = MarkdownPresentation::new("# A\n---\n# B\n---\n# C");
        pres.last();
        assert_eq!(pres.current_index(), 2);

        assert!(pres.prev_slide());
        assert_eq!(pres.current_index(), 1);

        assert!(pres.prev_slide());
        assert_eq!(pres.current_index(), 0);

        assert!(!pres.prev_slide()); // Can't go before first
        assert_eq!(pres.current_index(), 0);
    }

    // =========================================================================
    // Helper function tests
    // =========================================================================

    #[test]
    fn test_markdown_presentation_helper() {
        let pres = markdown_presentation("# Test");
        assert_eq!(pres.source(), "# Test");
    }

    // =========================================================================
    // MarkdownPresentation Default tests
    // =========================================================================

    #[test]
    fn test_default() {
        let pres = MarkdownPresentation::default();
        assert_eq!(pres.source(), "");
        assert_eq!(pres.current_mode(), ViewMode::Preview);
    }

    // =========================================================================
    // Edge case tests
    // =========================================================================

    #[test]
    fn test_scroll_edge_cases() {
        let mut pres = MarkdownPresentation::new("# Test");

        pres.scroll_up(10); // Should not go negative
        assert_eq!(pres.scroll_offset, 0);

        pres.scroll_down(usize::MAX); // Should saturate
        assert!(pres.scroll_offset > 0);
    }

    #[test]
    fn test_goto_bounds() {
        let mut pres = MarkdownPresentation::new("# A\n---\n# B\n---\n# C");

        pres.goto(10); // Out of bounds
        assert_eq!(pres.current_index(), 3); // Goes to last

        pres.goto(0);
        assert_eq!(pres.current_index(), 0);
    }

    #[test]
    fn test_mode_setter() {
        let pres = MarkdownPresentation::new("# Test").mode(ViewMode::Slides);
        assert_eq!(pres.current_mode(), ViewMode::Slides);
    }

    // =========================================================================
    // ViewMode::Slides tests
    // =========================================================================

    #[test]
    fn test_view_mode_slides_rendering() {
        let pres = MarkdownPresentation::new("# Slide 1\n\n---\n# Slide 2").mode(ViewMode::Slides);
        assert_eq!(pres.current_mode(), ViewMode::Slides);
    }

    // =========================================================================
    // Clone tests
    // =========================================================================

    #[test]
    fn test_clone() {
        let pres1 = MarkdownPresentation::new("# Test")
            .bg(Color::BLACK)
            .accent(Color::RED)
            .mode(ViewMode::Slides);
        let pres2 = pres1.clone();

        assert_eq!(pres1.source(), pres2.source());
        assert_eq!(pres1.current_mode(), pres2.current_mode());
    }

    // =========================================================================
    // Debug trait tests
    // =========================================================================

    #[test]
    fn test_view_mode_debug() {
        let mode = ViewMode::Slides;
        let debug_str = format!("{:?}", mode);
        assert!(debug_str.contains("Slides"));
    }

    #[test]
    fn test_markdown_presentation_debug() {
        let pres = MarkdownPresentation::new("# Test");
        let debug_str = format!("{:?}", pres);
        assert!(debug_str.contains("MarkdownPresentation"));
    }

    // =========================================================================
    // Combined builder tests
    // =========================================================================

    #[test]
    fn test_combined_builder() {
        let pres = MarkdownPresentation::new("# Test")
            .bg(Color::rgb(10, 10, 20))
            .accent(Color::MAGENTA)
            .heading_fg(Color::WHITE)
            .link_fg(Color::CYAN)
            .code_fg(Color::YELLOW)
            .numbers(true)
            .progress(true)
            .mode(ViewMode::Slides);

        assert_eq!(pres.current_mode(), ViewMode::Slides);
        assert!(pres.show_numbers);
        assert!(pres.show_progress);
    }

    // =========================================================================
    // Strip title edge cases
    // =========================================================================

    #[test]
    fn test_strip_title_no_heading() {
        let pres = MarkdownPresentation::new("");
        let content = "Just some content\n\nNo heading";
        let stripped = pres.strip_title(content);
        assert_eq!(stripped.trim(), "Just some content\n\nNo heading");
    }

    #[test]
    fn test_strip_title_only_heading() {
        let pres = MarkdownPresentation::new("");
        let content = "# Title Only";
        let stripped = pres.strip_title(content);
        assert!(stripped.trim().is_empty());
    }
}
