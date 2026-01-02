//! Spinner widget for loading states

use super::traits::{RenderContext, View, WidgetProps};
use crate::render::Cell;
use crate::style::Color;
use crate::{impl_props_builders, impl_styled_view};

/// Spinner animation style
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SpinnerStyle {
    /// Dots: ⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏
    #[default]
    Dots,
    /// Line: |/-\
    Line,
    /// Circle: ◐◓◑◒
    Circle,
    /// Arrow: ←↖↑↗→↘↓↙
    Arrow,
    /// Box: ▖▘▝▗
    Box,
    /// Bounce: ⠁⠂⠄⠂
    Bounce,
}

impl SpinnerStyle {
    fn frames(&self) -> &'static [&'static str] {
        match self {
            SpinnerStyle::Dots => &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
            SpinnerStyle::Line => &["|", "/", "-", "\\"],
            SpinnerStyle::Circle => &["◐", "◓", "◑", "◒"],
            SpinnerStyle::Arrow => &["←", "↖", "↑", "↗", "→", "↘", "↓", "↙"],
            SpinnerStyle::Box => &["▖", "▘", "▝", "▗"],
            SpinnerStyle::Bounce => &["⠁", "⠂", "⠄", "⠂"],
        }
    }
}

/// A spinner widget for loading states
pub struct Spinner {
    style: SpinnerStyle,
    frame: usize,
    label: Option<String>,
    fg: Option<Color>,
    props: WidgetProps,
}

impl Spinner {
    /// Create a new spinner
    pub fn new() -> Self {
        Self {
            style: SpinnerStyle::default(),
            frame: 0,
            label: None,
            fg: Some(Color::CYAN),
            props: WidgetProps::new(),
        }
    }

    /// Set spinner style
    pub fn style(mut self, style: SpinnerStyle) -> Self {
        self.style = style;
        self
    }

    /// Set label text (shown after spinner)
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set spinner color
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Get current frame index
    pub fn frame(&self) -> usize {
        self.frame
    }

    /// Advance to next frame
    pub fn tick(&mut self) {
        let frames = self.style.frames();
        self.frame = (self.frame + 1) % frames.len();
    }

    /// Reset to first frame
    pub fn reset(&mut self) {
        self.frame = 0;
    }

    /// Set specific frame
    pub fn set_frame(&mut self, frame: usize) {
        let frames = self.style.frames();
        self.frame = frame % frames.len();
    }
}

impl Default for Spinner {
    fn default() -> Self {
        Self::new()
    }
}

impl View for Spinner {
    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width == 0 || area.height == 0 {
            return;
        }

        let frames = self.style.frames();
        let current_frame = frames[self.frame % frames.len()];

        // Render spinner character (get first char from frame string)
        if let Some(ch) = current_frame.chars().next() {
            let mut cell = Cell::new(ch);
            cell.fg = self.fg;
            ctx.buffer.set(area.x, area.y, cell);
        }

        // Render label if present
        if let Some(ref label) = self.label {
            let mut x = area.x + 2; // Space after spinner
            for ch in label.chars() {
                if x >= area.x + area.width {
                    break;
                }
                let cell = Cell::new(ch);
                ctx.buffer.set(x, area.y, cell);
                x += 1;
            }
        }
    }

    crate::impl_view_meta!("Spinner");
}

impl_styled_view!(Spinner);
impl_props_builders!(Spinner);

/// Helper function to create a spinner
pub fn spinner() -> Spinner {
    Spinner::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    #[test]
    fn test_spinner_new() {
        let s = Spinner::new();
        assert_eq!(s.frame(), 0);
        assert_eq!(s.style, SpinnerStyle::Dots);
    }

    #[test]
    fn test_spinner_tick() {
        let mut s = Spinner::new();
        assert_eq!(s.frame(), 0);
        s.tick();
        assert_eq!(s.frame(), 1);
        s.tick();
        assert_eq!(s.frame(), 2);
    }

    #[test]
    fn test_spinner_wrap() {
        let mut s = Spinner::new().style(SpinnerStyle::Line);
        // Line has 4 frames
        s.set_frame(3);
        s.tick();
        assert_eq!(s.frame(), 0); // Wraps back
    }

    #[test]
    fn test_spinner_render() {
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let s = Spinner::new(); // Dots style, frame 0
        s.render(&mut ctx);

        assert_eq!(buffer.get(0, 0).unwrap().symbol, '⠋');
    }

    #[test]
    fn test_spinner_with_label() {
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let s = Spinner::new().label("Loading...");
        s.render(&mut ctx);

        assert_eq!(buffer.get(0, 0).unwrap().symbol, '⠋');
        assert_eq!(buffer.get(2, 0).unwrap().symbol, 'L');
        assert_eq!(buffer.get(3, 0).unwrap().symbol, 'o');
    }

    #[test]
    fn test_spinner_style_line() {
        let mut buffer = Buffer::new(5, 1);
        let area = Rect::new(0, 0, 5, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let s = Spinner::new().style(SpinnerStyle::Line);
        s.render(&mut ctx);
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '|');
    }

    #[test]
    fn test_spinner_style_circle() {
        let mut buffer = Buffer::new(5, 1);
        let area = Rect::new(0, 0, 5, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let s = Spinner::new().style(SpinnerStyle::Circle);
        s.render(&mut ctx);
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '◐');
    }

    #[test]
    fn test_spinner_reset() {
        let mut s = Spinner::new();
        s.tick();
        s.tick();
        assert_eq!(s.frame(), 2);
        s.reset();
        assert_eq!(s.frame(), 0);
    }
}
