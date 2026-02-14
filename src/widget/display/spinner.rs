//! Spinner widget for loading states

use crate::render::Cell;
use crate::style::Color;
use crate::widget::traits::{RenderContext, View, WidgetProps};
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
    /// Get the animation frames for this spinner style
    pub fn frames(&self) -> &'static [&'static str] {
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
