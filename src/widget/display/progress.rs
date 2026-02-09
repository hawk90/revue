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

// Most tests moved to tests/widget_tests.rs
// Tests below access private fields and must stay inline

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // Progress::new tests
    // =========================================================================

    #[test]
    fn test_progress_new_zero() {
        let p = Progress::new(0.0);
        assert_eq!(p.value(), 0.0);
        assert_eq!(p.style, ProgressStyle::Block);
        assert_eq!(p.filled_fg, Some(Color::GREEN));
        assert_eq!(p.empty_fg, Some(Color::rgb(64, 64, 64)));
        assert_eq!(p.show_percentage, false);
    }

    #[test]
    fn test_progress_new_half() {
        let p = Progress::new(0.5);
        assert_eq!(p.value(), 0.5);
    }

    #[test]
    fn test_progress_new_full() {
        let p = Progress::new(1.0);
        assert_eq!(p.value(), 1.0);
    }

    #[test]
    fn test_progress_new_clamps_above() {
        let p = Progress::new(1.5);
        assert_eq!(p.value(), 1.0);
    }

    #[test]
    fn test_progress_new_clamps_below() {
        let p = Progress::new(-0.5);
        assert_eq!(p.value(), 0.0);
    }

    // =========================================================================
    // Progress::progress tests
    // =========================================================================

    #[test]
    fn test_progress_builder() {
        let p = Progress::new(0.0).progress(0.75);
        assert_eq!(p.value(), 0.75);
    }

    #[test]
    fn test_progress_builder_clamps() {
        let p = Progress::new(0.0).progress(2.0);
        assert_eq!(p.value(), 1.0);
    }

    #[test]
    fn test_progress_builder_clamps_negative() {
        let p = Progress::new(0.0).progress(-1.0);
        assert_eq!(p.value(), 0.0);
    }

    // =========================================================================
    // Progress::style tests
    // =========================================================================

    #[test]
    fn test_style_block() {
        let p = Progress::new(0.5).style(ProgressStyle::Block);
        assert_eq!(p.style, ProgressStyle::Block);
    }

    #[test]
    fn test_style_line() {
        let p = Progress::new(0.5).style(ProgressStyle::Line);
        assert_eq!(p.style, ProgressStyle::Line);
    }

    #[test]
    fn test_style_ascii() {
        let p = Progress::new(0.5).style(ProgressStyle::Ascii);
        assert_eq!(p.style, ProgressStyle::Ascii);
    }

    #[test]
    fn test_style_braille() {
        let p = Progress::new(0.5).style(ProgressStyle::Braille);
        assert_eq!(p.style, ProgressStyle::Braille);
    }

    // =========================================================================
    // Progress::filled_color tests
    // =========================================================================

    #[test]
    fn test_filled_color() {
        let p = Progress::new(0.5).filled_color(Color::BLUE);
        assert_eq!(p.filled_fg, Some(Color::BLUE));
    }

    #[test]
    fn test_filled_color_red() {
        let p = Progress::new(0.5).filled_color(Color::RED);
        assert_eq!(p.filled_fg, Some(Color::RED));
    }

    #[test]
    fn test_filled_color_yellow() {
        let p = Progress::new(0.5).filled_color(Color::YELLOW);
        assert_eq!(p.filled_fg, Some(Color::YELLOW));
    }

    // =========================================================================
    // Progress::empty_color tests
    // =========================================================================

    #[test]
    fn test_empty_color() {
        let p = Progress::new(0.5).empty_color(Color::WHITE);
        assert_eq!(p.empty_fg, Some(Color::WHITE));
    }

    #[test]
    fn test_empty_color_default() {
        let p = Progress::new(0.5);
        assert_eq!(p.empty_fg, Some(Color::rgb(64, 64, 64)));
    }

    // =========================================================================
    // Progress::show_percentage tests
    // =========================================================================

    #[test]
    fn test_show_percentage_true() {
        let p = Progress::new(0.5).show_percentage(true);
        assert_eq!(p.show_percentage, true);
    }

    #[test]
    fn test_show_percentage_false() {
        let p = Progress::new(0.5).show_percentage(false);
        assert_eq!(p.show_percentage, false);
    }

    #[test]
    fn test_show_percentage_default() {
        let p = Progress::new(0.5);
        assert_eq!(p.show_percentage, false);
    }

    // =========================================================================
    // Progress::value tests
    // =========================================================================

    #[test]
    fn test_value_zero() {
        let p = Progress::new(0.0);
        assert_eq!(p.value(), 0.0);
    }

    #[test]
    fn test_value_quarter() {
        let p = Progress::new(0.25);
        assert_eq!(p.value(), 0.25);
    }

    #[test]
    fn test_value_half() {
        let p = Progress::new(0.5);
        assert_eq!(p.value(), 0.5);
    }

    #[test]
    fn test_value_three_quarters() {
        let p = Progress::new(0.75);
        assert_eq!(p.value(), 0.75);
    }

    #[test]
    fn test_value_full() {
        let p = Progress::new(1.0);
        assert_eq!(p.value(), 1.0);
    }

    // =========================================================================
    // Progress::set_progress tests
    // =========================================================================

    #[test]
    fn test_set_progress() {
        let mut p = Progress::new(0.0);
        p.set_progress(0.5);
        assert_eq!(p.value(), 0.5);
    }

    #[test]
    fn test_set_progress_clamps_above() {
        let mut p = Progress::new(0.0);
        p.set_progress(1.5);
        assert_eq!(p.value(), 1.0);
    }

    #[test]
    fn test_set_progress_clamps_below() {
        let mut p = Progress::new(0.0);
        p.set_progress(-0.5);
        assert_eq!(p.value(), 0.0);
    }

    #[test]
    fn test_set_progress_multiple_times() {
        let mut p = Progress::new(0.0);
        p.set_progress(0.3);
        assert_eq!(p.value(), 0.3);
        p.set_progress(0.6);
        assert_eq!(p.value(), 0.6);
        p.set_progress(0.9);
        assert_eq!(p.value(), 0.9);
    }

    // =========================================================================
    // Progress::get_chars tests (tested via style)
    // =========================================================================

    #[test]
    fn test_get_chars_block() {
        let p = Progress::new(0.5).style(ProgressStyle::Block);
        let (filled, empty) = p.get_chars();
        assert_eq!(filled, '█');
        assert_eq!(empty, '░');
    }

    #[test]
    fn test_get_chars_line() {
        let p = Progress::new(0.5).style(ProgressStyle::Line);
        let (filled, empty) = p.get_chars();
        assert_eq!(filled, '━');
        assert_eq!(empty, '─');
    }

    #[test]
    fn test_get_chars_ascii() {
        let p = Progress::new(0.5).style(ProgressStyle::Ascii);
        let (filled, empty) = p.get_chars();
        assert_eq!(filled, '#');
        assert_eq!(empty, '-');
    }

    #[test]
    fn test_get_chars_braille() {
        let p = Progress::new(0.5).style(ProgressStyle::Braille);
        let (filled, empty) = p.get_chars();
        assert_eq!(filled, '⣿');
        assert_eq!(empty, '⡀');
    }

    // =========================================================================
    // Default trait tests
    // =========================================================================

    #[test]
    fn test_progress_default() {
        let p = Progress::default();
        assert_eq!(p.value(), 0.0);
        assert_eq!(p.style, ProgressStyle::Block);
    }

    // =========================================================================
    // Helper function tests
    // =========================================================================

    #[test]
    fn test_progress_helper() {
        let p = progress(0.5);
        assert_eq!(p.value(), 0.5);
    }

    #[test]
    fn test_progress_helper_zero() {
        let p = progress(0.0);
        assert_eq!(p.value(), 0.0);
    }

    #[test]
    fn test_progress_helper_full() {
        let p = progress(1.0);
        assert_eq!(p.value(), 1.0);
    }

    // =========================================================================
    // Builder chain tests
    // =========================================================================

    #[test]
    fn test_builder_chain_full() {
        let p = Progress::new(0.5)
            .style(ProgressStyle::Line)
            .filled_color(Color::BLUE)
            .empty_color(Color::rgb(128, 128, 128))
            .show_percentage(true);

        assert_eq!(p.value(), 0.5);
        assert_eq!(p.style, ProgressStyle::Line);
        assert_eq!(p.filled_fg, Some(Color::BLUE));
        assert_eq!(p.empty_fg, Some(Color::rgb(128, 128, 128)));
        assert_eq!(p.show_percentage, true);
    }

    #[test]
    fn test_builder_chain_with_progress_update() {
        let p = Progress::new(0.25)
            .progress(0.75)
            .style(ProgressStyle::Braille);

        assert_eq!(p.value(), 0.75);
        assert_eq!(p.style, ProgressStyle::Braille);
    }

    // =========================================================================
    // ProgressStyle enum trait tests
    // =========================================================================

    #[test]
    fn test_progress_style_default() {
        assert_eq!(ProgressStyle::default(), ProgressStyle::Block);
    }

    #[test]
    fn test_progress_style_clone() {
        let style = ProgressStyle::Line;
        let cloned = style;
        assert_eq!(style, cloned);
    }

    #[test]
    fn test_progress_style_copy() {
        let s1 = ProgressStyle::Ascii;
        let s2 = s1;
        assert_eq!(s1, ProgressStyle::Ascii);
        assert_eq!(s2, ProgressStyle::Ascii);
    }

    #[test]
    fn test_progress_style_partial_eq() {
        assert_eq!(ProgressStyle::Block, ProgressStyle::Block);
        assert_ne!(ProgressStyle::Block, ProgressStyle::Line);
        assert_ne!(ProgressStyle::Line, ProgressStyle::Ascii);
        assert_ne!(ProgressStyle::Ascii, ProgressStyle::Braille);
    }

    // =========================================================================
    // Edge case tests
    // =========================================================================

    #[test]
    fn test_very_small_progress() {
        let p = Progress::new(0.001);
        assert_eq!(p.value(), 0.001);
    }

    #[test]
    fn test_very_large_progress_clamped() {
        let p = Progress::new(999.0);
        assert_eq!(p.value(), 1.0);
    }

    #[test]
    fn test_negative_progress_clamped() {
        let p = Progress::new(-999.0);
        assert_eq!(p.value(), 0.0);
    }

    #[test]
    fn test_exact_boundary_zero() {
        let p = Progress::new(0.0);
        assert_eq!(p.value(), 0.0);
    }

    #[test]
    fn test_exact_boundary_one() {
        let p = Progress::new(1.0);
        assert_eq!(p.value(), 1.0);
    }

    #[test]
    fn test_set_then_builder() {
        let mut p = Progress::new(0.0);
        p.set_progress(0.6);
        assert_eq!(p.value(), 0.6);

        let p2 = p.progress(0.8);
        assert_eq!(p2.value(), 0.8);
    }

    // =========================================================================
    // Color variation tests
    // =========================================================================

    #[test]
    fn test_multiple_color_settings() {
        let p = Progress::new(0.5)
            .filled_color(Color::CYAN)
            .empty_color(Color::MAGENTA);

        assert_eq!(p.filled_fg, Some(Color::CYAN));
        assert_eq!(p.empty_fg, Some(Color::MAGENTA));
    }

    #[test]
    fn test_default_colors_after_new() {
        let p = Progress::new(0.5);
        assert_eq!(p.filled_fg, Some(Color::GREEN));
        assert_eq!(p.filled_bg, None);
        assert_eq!(p.empty_fg, Some(Color::rgb(64, 64, 64)));
        assert_eq!(p.empty_bg, None);
    }

    // =========================================================================
    // Style variation tests
    // =========================================================================

    #[test]
    fn test_all_styles_distinct() {
        let block = Progress::new(0.5).style(ProgressStyle::Block);
        let line = Progress::new(0.5).style(ProgressStyle::Line);
        let ascii = Progress::new(0.5).style(ProgressStyle::Ascii);
        let braille = Progress::new(0.5).style(ProgressStyle::Braille);

        assert_ne!(block.style, line.style);
        assert_ne!(line.style, ascii.style);
        assert_ne!(ascii.style, braille.style);
    }
}
