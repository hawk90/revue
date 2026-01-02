//! Progress bar widget

use super::traits::{RenderContext, View, WidgetProps};
use crate::render::Cell;
use crate::style::Color;
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
            for (i, ch) in pct.chars().enumerate() {
                if start_x + i as u16 >= area.x + area.width {
                    break;
                }
                let cell = Cell::new(ch);
                ctx.buffer.set(start_x + i as u16, area.y, cell);
            }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    #[test]
    fn test_progress_new() {
        let p = Progress::new(0.5);
        assert!((p.value() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_progress_clamp() {
        let p1 = Progress::new(-0.5);
        assert!((p1.value() - 0.0).abs() < f32::EPSILON);

        let p2 = Progress::new(1.5);
        assert!((p2.value() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_progress_render_half() {
        let mut buffer = Buffer::new(10, 1);
        let area = Rect::new(0, 0, 10, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let p = Progress::new(0.5);
        p.render(&mut ctx);

        // First 5 should be filled
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '█');
        assert_eq!(buffer.get(4, 0).unwrap().symbol, '█');
        // Last 5 should be empty
        assert_eq!(buffer.get(5, 0).unwrap().symbol, '░');
        assert_eq!(buffer.get(9, 0).unwrap().symbol, '░');
    }

    #[test]
    fn test_progress_render_full() {
        let mut buffer = Buffer::new(10, 1);
        let area = Rect::new(0, 0, 10, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let p = Progress::new(1.0);
        p.render(&mut ctx);

        for x in 0..10 {
            assert_eq!(buffer.get(x, 0).unwrap().symbol, '█');
        }
    }

    #[test]
    fn test_progress_render_empty() {
        let mut buffer = Buffer::new(10, 1);
        let area = Rect::new(0, 0, 10, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let p = Progress::new(0.0);
        p.render(&mut ctx);

        for x in 0..10 {
            assert_eq!(buffer.get(x, 0).unwrap().symbol, '░');
        }
    }

    #[test]
    fn test_progress_ascii_style() {
        let mut buffer = Buffer::new(10, 1);
        let area = Rect::new(0, 0, 10, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let p = Progress::new(0.5).style(ProgressStyle::Ascii);
        p.render(&mut ctx);

        assert_eq!(buffer.get(0, 0).unwrap().symbol, '#');
        assert_eq!(buffer.get(5, 0).unwrap().symbol, '-');
    }

    #[test]
    fn test_progress_with_percentage() {
        let mut buffer = Buffer::new(15, 1);
        let area = Rect::new(0, 0, 15, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let p = Progress::new(0.5).show_percentage(true);
        p.render(&mut ctx);

        // Bar is 10 chars (15 - 5 for " 50%"), percentage starts at 11
        // Format is " 50%" (space-padded to 3 digits + %)
        assert_eq!(buffer.get(11, 0).unwrap().symbol, ' ');
        assert_eq!(buffer.get(12, 0).unwrap().symbol, '5');
        assert_eq!(buffer.get(13, 0).unwrap().symbol, '0');
        assert_eq!(buffer.get(14, 0).unwrap().symbol, '%');
    }

    #[test]
    fn test_progress_set() {
        let mut p = Progress::new(0.0);
        p.set_progress(0.75);
        assert!((p.value() - 0.75).abs() < f32::EPSILON);
    }
}
