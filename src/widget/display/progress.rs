//! Progress bar widget

use crate::render::Cell;
use crate::style::Color;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// Progress bar style
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ProgressStyle {
    /// Block style: ████░░░░
    #[default]
    Block,
    /// Line style: ━━━━────
    Line,
    /// ASCII: ####----
    Ascii,
    /// Braille: ⣿⣿⣿⣿⡀
    Braille,
}

/// A progress bar widget
pub struct Progress {
    progress: f32, // 0.0 to 1.0
    style: ProgressStyle,
    filled_fg: Option<Color>,
    filled_bg: Option<Color>,
    empty_fg: Option<Color>,
    empty_bg: Option<Color>,
    show_percentage: bool,
    props: WidgetProps,
}

impl Progress {
    /// Create a new progress bar
    pub fn new(progress: f32) -> Self {
        Self {
            progress: progress.clamp(0.0, 1.0),
            style: ProgressStyle::default(),
            filled_fg: Some(Color::GREEN),
            filled_bg: None,
            empty_fg: Some(Color::rgb(64, 64, 64)),
            empty_bg: None,
            show_percentage: false,
            props: WidgetProps::new(),
        }
    }

    /// Set progress (0.0 to 1.0)
    pub fn progress(mut self, progress: f32) -> Self {
        self.progress = progress.clamp(0.0, 1.0);
        self
    }

    /// Set style
    pub fn style(mut self, style: ProgressStyle) -> Self {
        self.style = style;
        self
    }

    /// Set filled bar color
    pub fn filled_color(mut self, color: Color) -> Self {
        self.filled_fg = Some(color);
        self
    }

    /// Set empty bar color
    pub fn empty_color(mut self, color: Color) -> Self {
        self.empty_fg = Some(color);
        self
    }

    /// Show percentage text
    pub fn show_percentage(mut self, show: bool) -> Self {
        self.show_percentage = show;
        self
    }

    /// Get current progress value (0.0 to 1.0)
    pub fn value(&self) -> f32 {
        self.progress
    }

    /// Set progress value
    pub fn set_progress(&mut self, progress: f32) {
        self.progress = progress.clamp(0.0, 1.0);
    }

    fn get_chars(&self) -> (char, char) {
        match self.style {
            ProgressStyle::Block => ('█', '░'),
            ProgressStyle::Line => ('━', '─'),
            ProgressStyle::Ascii => ('#', '-'),
            ProgressStyle::Braille => ('⣿', '⡀'),
        }
    }
}

impl Default for Progress {
    fn default() -> Self {
        Self::new(0.0)
    }
}

impl View for Progress {
    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width == 0 || area.height == 0 {
            return;
        }

        let (filled_char, empty_char) = self.get_chars();

        // Calculate bar width (reserve space for percentage if shown)
        let bar_width = if self.show_percentage {
            area.width.saturating_sub(5) // " 100%"
        } else {
            area.width
        };

        let filled_width = (bar_width as f32 * self.progress).round() as u16;

        // Draw filled portion
        for x in 0..filled_width {
            if x >= bar_width {
                break;
            }
            let mut cell = Cell::new(filled_char);
            cell.fg = self.filled_fg;
            cell.bg = self.filled_bg;
            ctx.buffer.set(area.x + x, area.y, cell);
        }

        // Draw empty portion
        for x in filled_width..bar_width {
            let mut cell = Cell::new(empty_char);
            cell.fg = self.empty_fg;
            cell.bg = self.empty_bg;
            ctx.buffer.set(area.x + x, area.y, cell);
        }

        // Draw percentage if enabled
        if self.show_percentage {
            let pct = format!("{:3.0}%", self.progress * 100.0);
            let start_x = area.x + bar_width + 1;
            let max_width = (area.x + area.width).saturating_sub(start_x);
            ctx.draw_text_clipped(start_x, area.y, &pct, Color::WHITE, max_width);
        }
    }

    crate::impl_view_meta!("Progress");
}

/// Helper function to create a progress bar
pub fn progress(value: f32) -> Progress {
    Progress::new(value)
}

impl_styled_view!(Progress);
impl_props_builders!(Progress);
