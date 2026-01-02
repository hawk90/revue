//! Presentation Mode widget for terminal slideshows
//!
//! Create beautiful terminal-based presentations with slides,
//! transitions, and speaker notes.

use super::traits::{RenderContext, View, WidgetProps};
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::{impl_props_builders, impl_styled_view};

/// Slide transition effect
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Transition {
    /// No transition
    #[default]
    None,
    /// Fade in/out
    Fade,
    /// Slide from left
    SlideLeft,
    /// Slide from right
    SlideRight,
    /// Slide from bottom
    SlideUp,
    /// Zoom in
    ZoomIn,
}

/// Text alignment for slides
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SlideAlign {
    /// Left aligned
    Left,
    /// Center aligned
    #[default]
    Center,
    /// Right aligned
    Right,
}

/// A single slide
#[derive(Clone, Debug)]
pub struct Slide {
    /// Slide title
    pub title: String,
    /// Slide content (supports basic markdown)
    pub content: Vec<String>,
    /// Speaker notes (not displayed)
    pub notes: String,
    /// Background color
    pub bg: Option<Color>,
    /// Title color
    pub title_color: Color,
    /// Content color
    pub content_color: Color,
    /// Text alignment
    pub align: SlideAlign,
}

impl Slide {
    /// Create a new slide with title
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            content: Vec::new(),
            notes: String::new(),
            bg: None,
            title_color: Color::CYAN,
            content_color: Color::WHITE,
            align: SlideAlign::Center,
        }
    }

    /// Add content line
    pub fn line(mut self, text: impl Into<String>) -> Self {
        self.content.push(text.into());
        self
    }

    /// Add multiple content lines
    pub fn lines(mut self, lines: &[&str]) -> Self {
        for line in lines {
            self.content.push((*line).to_string());
        }
        self
    }

    /// Add bullet point
    pub fn bullet(mut self, text: impl Into<String>) -> Self {
        self.content.push(format!("  • {}", text.into()));
        self
    }

    /// Add numbered item
    pub fn numbered(mut self, num: usize, text: impl Into<String>) -> Self {
        self.content.push(format!("  {}. {}", num, text.into()));
        self
    }

    /// Add code block
    pub fn code(mut self, code: impl Into<String>) -> Self {
        self.content.push(String::new());
        for line in code.into().lines() {
            self.content.push(format!("    {}", line));
        }
        self.content.push(String::new());
        self
    }

    /// Set speaker notes
    pub fn notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = notes.into();
        self
    }

    /// Set background color
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    /// Set title color
    pub fn title_color(mut self, color: Color) -> Self {
        self.title_color = color;
        self
    }

    /// Set content color
    pub fn content_color(mut self, color: Color) -> Self {
        self.content_color = color;
        self
    }

    /// Set alignment
    pub fn align(mut self, align: SlideAlign) -> Self {
        self.align = align;
        self
    }
}

/// Presentation widget
///
/// # Example
///
/// ```rust,ignore
/// use revue::prelude::*;
///
/// let pres = Presentation::new()
///     .title("My Presentation")
///     .slide(Slide::new("Introduction")
///         .bullet("First point")
///         .bullet("Second point"))
///     .slide(Slide::new("Code Example")
///         .code("fn main() {\n    println!(\"Hello!\");\n}"));
///
/// // Navigate
/// pres.next();
/// pres.prev();
/// ```
pub struct Presentation {
    /// Presentation title
    title: String,
    /// Author name
    author: String,
    /// All slides
    slides: Vec<Slide>,
    /// Current slide index
    current: usize,
    /// Transition effect
    transition: Transition,
    /// Transition progress (0.0 to 1.0)
    transition_progress: f32,
    /// Show slide numbers
    show_numbers: bool,
    /// Show progress bar
    show_progress: bool,
    /// Timer (seconds)
    timer: Option<u64>,
    /// Background color
    bg: Color,
    /// Accent color
    accent: Color,
    /// Widget properties
    props: WidgetProps,
}

impl Presentation {
    /// Create a new presentation
    pub fn new() -> Self {
        Self {
            title: String::new(),
            author: String::new(),
            slides: Vec::new(),
            current: 0,
            transition: Transition::None,
            transition_progress: 1.0,
            show_numbers: true,
            show_progress: true,
            timer: None,
            bg: Color::rgb(20, 20, 30),
            accent: Color::CYAN,
            props: WidgetProps::new(),
        }
    }

    /// Set presentation title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Set author
    pub fn author(mut self, author: impl Into<String>) -> Self {
        self.author = author.into();
        self
    }

    /// Add a slide
    pub fn slide(mut self, slide: Slide) -> Self {
        self.slides.push(slide);
        self
    }

    /// Add multiple slides
    pub fn slides(mut self, slides: Vec<Slide>) -> Self {
        self.slides.extend(slides);
        self
    }

    /// Set transition effect
    pub fn transition(mut self, transition: Transition) -> Self {
        self.transition = transition;
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

    /// Set timer (in seconds)
    pub fn timer(mut self, seconds: u64) -> Self {
        self.timer = Some(seconds);
        self
    }

    /// Go to next slide
    pub fn next(&mut self) -> bool {
        if self.current < self.slides.len().saturating_sub(1) {
            self.current += 1;
            self.transition_progress = 0.0;
            true
        } else {
            false
        }
    }

    /// Go to previous slide
    pub fn prev(&mut self) -> bool {
        if self.current > 0 {
            self.current -= 1;
            self.transition_progress = 0.0;
            true
        } else {
            false
        }
    }

    /// Go to specific slide
    pub fn goto(&mut self, index: usize) {
        if index < self.slides.len() {
            self.current = index;
            self.transition_progress = 0.0;
        }
    }

    /// Go to first slide
    pub fn first(&mut self) {
        self.goto(0);
    }

    /// Go to last slide
    pub fn last(&mut self) {
        self.goto(self.slides.len().saturating_sub(1));
    }

    /// Get current slide index
    pub fn current_index(&self) -> usize {
        self.current
    }

    /// Get total slides
    pub fn slide_count(&self) -> usize {
        self.slides.len()
    }

    /// Get current slide
    pub fn current_slide(&self) -> Option<&Slide> {
        self.slides.get(self.current)
    }

    /// Get speaker notes for current slide
    pub fn current_notes(&self) -> Option<&str> {
        self.current_slide().map(|s| s.notes.as_str())
    }

    /// Update transition animation
    pub fn tick(&mut self, dt: f32) {
        if self.transition_progress < 1.0 {
            self.transition_progress = (self.transition_progress + dt * 3.0).min(1.0);
        }
    }

    /// Render title slide (slide 0 or empty presentation)
    fn render_title_slide(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let center_y = area.height / 2;

        // Title
        let title_y = center_y.saturating_sub(2);
        self.render_centered_text(ctx, &self.title, title_y, self.accent, Modifier::BOLD);

        // Author
        if !self.author.is_empty() {
            let author_y = center_y + 1;
            self.render_centered_text(
                ctx,
                &self.author,
                author_y,
                Color::rgb(150, 150, 150),
                Modifier::ITALIC,
            );
        }

        // Press key hint
        let hint = "Press → or Space to start";
        let hint_y = area.height - 2;
        self.render_centered_text(
            ctx,
            hint,
            hint_y,
            Color::rgb(100, 100, 100),
            Modifier::empty(),
        );
    }

    /// Render a content slide
    fn render_content_slide(&self, ctx: &mut RenderContext, slide: &Slide) {
        let area = ctx.area;

        // Background
        let bg = slide.bg.unwrap_or(self.bg);
        for y in 0..area.height {
            for x in 0..area.width {
                let mut cell = Cell::new(' ');
                cell.bg = Some(bg);
                ctx.buffer.set(area.x + x, area.y + y, cell);
            }
        }

        // Title (top center)
        let title_y = 2;
        self.render_centered_text(
            ctx,
            &slide.title,
            title_y,
            slide.title_color,
            Modifier::BOLD,
        );

        // Separator
        let sep_y = 4;
        let sep_len = slide.title.chars().count().min(area.width as usize - 4);
        let sep_start = (area.width as usize - sep_len) / 2;
        for i in 0..sep_len {
            let mut cell = Cell::new('─');
            cell.fg = Some(self.accent);
            ctx.buffer
                .set(area.x + sep_start as u16 + i as u16, area.y + sep_y, cell);
        }

        // Content
        let content_start_y = 6;
        for (i, line) in slide.content.iter().enumerate() {
            let y = area.y + content_start_y + i as u16;
            if y >= area.y + area.height - 3 {
                break;
            }

            match slide.align {
                SlideAlign::Left => {
                    for (j, ch) in line.chars().enumerate() {
                        if j as u16 + 2 >= area.width {
                            break;
                        }
                        let mut cell = Cell::new(ch);
                        cell.fg = Some(slide.content_color);
                        ctx.buffer.set(area.x + 2 + j as u16, y, cell);
                    }
                }
                SlideAlign::Center => {
                    self.render_centered_text(
                        ctx,
                        line,
                        y - area.y,
                        slide.content_color,
                        Modifier::empty(),
                    );
                }
                SlideAlign::Right => {
                    let line_len = line.chars().count();
                    let start_x = area.width.saturating_sub(line_len as u16 + 2);
                    for (j, ch) in line.chars().enumerate() {
                        let mut cell = Cell::new(ch);
                        cell.fg = Some(slide.content_color);
                        ctx.buffer.set(area.x + start_x + j as u16, y, cell);
                    }
                }
            }
        }
    }

    /// Render centered text
    fn render_centered_text(
        &self,
        ctx: &mut RenderContext,
        text: &str,
        y: u16,
        fg: Color,
        modifier: Modifier,
    ) {
        let area = ctx.area;
        let text_len = text.chars().count();
        let start_x = (area.width as usize).saturating_sub(text_len) / 2;

        for (i, ch) in text.chars().enumerate() {
            let x = area.x + start_x as u16 + i as u16;
            if x >= area.x + area.width {
                break;
            }
            let mut cell = Cell::new(ch);
            cell.fg = Some(fg);
            cell.modifier = modifier;
            ctx.buffer.set(x, area.y + y, cell);
        }
    }

    /// Render footer (slide numbers, progress)
    fn render_footer(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let footer_y = area.y + area.height - 1;

        // Slide numbers
        if self.show_numbers && !self.slides.is_empty() {
            let num_str = format!("{}/{}", self.current + 1, self.slides.len());
            let start_x = area.x + area.width - num_str.len() as u16 - 1;
            for (i, ch) in num_str.chars().enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(Color::rgb(100, 100, 100));
                ctx.buffer.set(start_x + i as u16, footer_y, cell);
            }
        }

        // Progress bar
        if self.show_progress && !self.slides.is_empty() {
            let bar_width = (area.width / 3).max(10);
            let progress = (self.current + 1) as f32 / self.slides.len() as f32;
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
    }
}

impl Default for Presentation {
    fn default() -> Self {
        Self::new()
    }
}

impl View for Presentation {
    crate::impl_view_meta!("Presentation");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;

        // Background
        for y in 0..area.height {
            for x in 0..area.width {
                let mut cell = Cell::new(' ');
                cell.bg = Some(self.bg);
                ctx.buffer.set(area.x + x, area.y + y, cell);
            }
        }

        // Render current slide
        if self.slides.is_empty() || self.current == 0 && !self.title.is_empty() {
            self.render_title_slide(ctx);
        } else if let Some(slide) = self.slides.get(self.current) {
            self.render_content_slide(ctx, slide);
        }

        // Footer
        self.render_footer(ctx);
    }
}

impl_styled_view!(Presentation);
impl_props_builders!(Presentation);

/// Create a new presentation
pub fn presentation() -> Presentation {
    Presentation::new()
}

/// Create a slide
pub fn slide(title: impl Into<String>) -> Slide {
    Slide::new(title)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    #[test]
    fn test_presentation_creation() {
        let pres = Presentation::new().title("Test").author("Author");
        assert_eq!(pres.slide_count(), 0);
    }

    #[test]
    fn test_slide_creation() {
        let s = Slide::new("Title")
            .bullet("Point 1")
            .bullet("Point 2")
            .code("let x = 1;");
        assert_eq!(s.title, "Title");
        assert_eq!(s.content.len(), 5); // 2 bullets + empty + code + empty
    }

    #[test]
    fn test_navigation() {
        let mut pres = Presentation::new()
            .slide(slide("Slide 1"))
            .slide(slide("Slide 2"))
            .slide(slide("Slide 3"));

        assert_eq!(pres.current_index(), 0);
        assert!(pres.next());
        assert_eq!(pres.current_index(), 1);
        assert!(pres.prev());
        assert_eq!(pres.current_index(), 0);
        assert!(!pres.prev()); // Can't go before 0
    }

    #[test]
    fn test_presentation_render() {
        let pres = Presentation::new()
            .title("Test Presentation")
            .slide(slide("Intro").bullet("Hello"));

        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        pres.render(&mut ctx);
    }
}
