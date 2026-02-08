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

// Most tests moved to tests/widget_tests.rs
// Tests below access private fields and must stay inline

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spinner_new() {
        let s = Spinner::new();
        assert_eq!(s.frame(), 0);
        assert_eq!(s.style, SpinnerStyle::Dots);
    }

    // =========================================================================
    // Spinner::style tests
    // =========================================================================

    #[test]
    fn test_spinner_style_line() {
        let s = Spinner::new().style(SpinnerStyle::Line);
        assert_eq!(s.style, SpinnerStyle::Line);
    }

    #[test]
    fn test_spinner_style_circle() {
        let s = Spinner::new().style(SpinnerStyle::Circle);
        assert_eq!(s.style, SpinnerStyle::Circle);
    }

    #[test]
    fn test_spinner_style_arrow() {
        let s = Spinner::new().style(SpinnerStyle::Arrow);
        assert_eq!(s.style, SpinnerStyle::Arrow);
    }

    #[test]
    fn test_spinner_style_box() {
        let s = Spinner::new().style(SpinnerStyle::Box);
        assert_eq!(s.style, SpinnerStyle::Box);
    }

    #[test]
    fn test_spinner_style_bounce() {
        let s = Spinner::new().style(SpinnerStyle::Bounce);
        assert_eq!(s.style, SpinnerStyle::Bounce);
    }

    // =========================================================================
    // Spinner::label tests
    // =========================================================================

    #[test]
    fn test_spinner_label() {
        let s = Spinner::new().label("Loading...");
        assert_eq!(s.label, Some("Loading...".to_string()));
    }

    #[test]
    fn test_spinner_label_with_string() {
        let s = Spinner::new().label("Please wait".to_string());
        assert_eq!(s.label, Some("Please wait".to_string()));
    }

    // =========================================================================
    // Spinner::fg tests
    // =========================================================================

    #[test]
    fn test_spinner_fg() {
        let s = Spinner::new().fg(Color::RED);
        assert_eq!(s.fg, Some(Color::RED));
    }

    #[test]
    fn test_spinner_fg_none() {
        let s = Spinner::new();
        assert_eq!(s.fg, Some(Color::CYAN)); // Default is CYAN
    }

    // =========================================================================
    // Spinner::tick tests
    // =========================================================================

    #[test]
    fn test_spinner_tick() {
        let mut s = Spinner::new();
        assert_eq!(s.frame(), 0);
        s.tick();
        assert_eq!(s.frame(), 1);
    }

    #[test]
    fn test_spinner_tick_wrap() {
        let mut s = Spinner::new().style(SpinnerStyle::Line);
        // Line has 4 frames: |, /, -, \
        s.set_frame(3);
        s.tick();
        assert_eq!(s.frame(), 0); // Should wrap
    }

    // =========================================================================
    // Spinner::reset tests
    // =========================================================================

    #[test]
    fn test_spinner_reset() {
        let mut s = Spinner::new();
        s.set_frame(5);
        s.reset();
        assert_eq!(s.frame(), 0);
    }

    // =========================================================================
    // Spinner::set_frame tests
    // =========================================================================

    #[test]
    fn test_set_frame_valid() {
        let mut s = Spinner::new();
        s.set_frame(2);
        assert_eq!(s.frame(), 2);
    }

    #[test]
    fn test_set_frame_wraps() {
        let mut s = Spinner::new().style(SpinnerStyle::Box);
        // Box has 4 frames
        s.set_frame(10);
        assert_eq!(s.frame(), 10 % 4); // Should wrap
    }

    // =========================================================================
    // Spinner::frame tests
    // =========================================================================

    #[test]
    fn test_frame_after_new() {
        let s = Spinner::new();
        assert_eq!(s.frame(), 0);
    }

    #[test]
    fn test_frame_after_style_change() {
        let s = Spinner::new().style(SpinnerStyle::Circle);
        assert_eq!(s.frame(), 0);
    }

    // =========================================================================
    // Helper function tests
    // =========================================================================

    #[test]
    fn test_spinner_helper() {
        let s = spinner();
        assert_eq!(s.frame(), 0);
    }

    // =========================================================================
    // SpinnerStyle enum trait tests
    // =========================================================================

    #[test]
    fn test_spinner_style_default() {
        assert_eq!(SpinnerStyle::default(), SpinnerStyle::Dots);
    }

    #[test]
    fn test_spinner_style_clone() {
        let style = SpinnerStyle::Arrow;
        let cloned = style;
        assert_eq!(style, cloned);
    }

    #[test]
    fn test_spinner_style_copy() {
        let s1 = SpinnerStyle::Circle;
        let s2 = s1;
        assert_eq!(s1, SpinnerStyle::Circle);
        assert_eq!(s2, SpinnerStyle::Circle);
    }

    #[test]
    fn test_spinner_style_partial_eq() {
        assert_eq!(SpinnerStyle::Dots, SpinnerStyle::Dots);
        assert_ne!(SpinnerStyle::Dots, SpinnerStyle::Line);
    }

    // =========================================================================
    // Spinner Default tests
    // =========================================================================

    #[test]
    fn test_spinner_default() {
        let s = Spinner::default();
        assert_eq!(s.frame(), 0);
    }

    // =========================================================================
    // Builder chain tests
    // =========================================================================

    #[test]
    fn test_builder_chain() {
        let s = Spinner::new()
            .style(SpinnerStyle::Arrow)
            .label("Wait")
            .fg(Color::YELLOW);

        assert_eq!(s.style, SpinnerStyle::Arrow);
        assert_eq!(s.label, Some("Wait".to_string()));
        assert_eq!(s.fg, Some(Color::YELLOW));
    }

    // =========================================================================
    // Frame animation tests
    // =========================================================================

    #[test]
    fn test_multiple_ticks() {
        let mut s = Spinner::new().style(SpinnerStyle::Line);
        for i in 0..10 {
            s.tick();
            assert_eq!(s.frame(), (i + 1) % 4);
        }
    }

    #[test]
    fn test_tick_then_reset() {
        let mut s = Spinner::new();
        s.tick();
        s.tick();
        assert!(s.frame() > 0);
        s.reset();
        assert_eq!(s.frame(), 0);
    }

    // =========================================================================
    // Edge case tests
    // =========================================================================

    #[test]
    fn test_spinner_zero_frame() {
        let mut s = Spinner::new();
        s.reset();
        s.set_frame(0);
        assert_eq!(s.frame(), 0);
    }

    #[test]
    fn test_spinner_large_frame() {
        let mut s = Spinner::new();
        s.set_frame(1000);
        // Should wrap based on style's frame count
        let _ = s.frame();
    }

    #[test]
    fn test_empty_label() {
        let s = Spinner::new().label("");
        assert_eq!(s.label, Some("".to_string()));
    }

    #[test]
    fn test_unicode_label() {
        let s = Spinner::new().label("Загрузка...");
        assert_eq!(s.label, Some("Загрузка...".to_string()));
    }

    #[test]
    fn test_emoji_label() {
        let s = Spinner::new().label("⏳ Loading");
        assert_eq!(s.label, Some("⏳ Loading".to_string()));
    }

    // =========================================================================
    // SpinnerStyle frames test
    // =========================================================================

    #[test]
    fn test_spinner_style_dots_frames() {
        let frames = SpinnerStyle::Dots.frames();
        assert_eq!(frames.len(), 10);
        assert_eq!(frames[0], "⠋");
    }

    #[test]
    fn test_spinner_style_line_frames() {
        let frames = SpinnerStyle::Line.frames();
        assert_eq!(frames.len(), 4);
        assert_eq!(frames[0], "|");
    }

    #[test]
    fn test_spinner_style_circle_frames() {
        let frames = SpinnerStyle::Circle.frames();
        assert_eq!(frames.len(), 4);
        assert_eq!(frames[0], "◐");
    }

    #[test]
    fn test_spinner_style_arrow_frames() {
        let frames = SpinnerStyle::Arrow.frames();
        assert_eq!(frames.len(), 8);
        assert_eq!(frames[0], "←");
    }

    #[test]
    fn test_spinner_style_box_frames() {
        let frames = SpinnerStyle::Box.frames();
        assert_eq!(frames.len(), 4);
        assert_eq!(frames[0], "▖");
    }

    #[test]
    fn test_spinner_style_bounce_frames() {
        let frames = SpinnerStyle::Bounce.frames();
        assert_eq!(frames.len(), 4);
        assert_eq!(frames[0], "⠁");
    }

    // =========================================================================
    // Render tests
    // =========================================================================

    #[test]
    fn test_spinner_render_with_valid_area() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(20, 5);
        let area = Rect::new(0, 0, 20, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let spinner = Spinner::new();
        spinner.render(&mut ctx);

        // Check that spinner character was rendered
        let cell = buffer.get(0, 0).unwrap();
        assert_eq!(cell.symbol, '⠋'); // Default Dots style first frame
    }

    #[test]
    fn test_spinner_render_with_zero_width() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(0, 5);
        let area = Rect::new(0, 0, 0, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let spinner = Spinner::new();
        spinner.render(&mut ctx); // Should not crash

        // Buffer should be empty
        assert_eq!(buffer.get(0, 0), None);
    }

    #[test]
    fn test_spinner_render_with_zero_height() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(20, 0);
        let area = Rect::new(0, 0, 20, 0);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let spinner = Spinner::new();
        spinner.render(&mut ctx); // Should not crash

        // Buffer should be empty
        assert_eq!(buffer.get(0, 0), None);
    }

    #[test]
    fn test_spinner_render_with_label() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(20, 5);
        let area = Rect::new(0, 0, 20, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let spinner = Spinner::new().label("Loading");
        spinner.render(&mut ctx);

        // Check spinner character
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '⠋');

        // Check label (starts at x=2)
        assert_eq!(buffer.get(2, 0).unwrap().symbol, 'L');
        assert_eq!(buffer.get(3, 0).unwrap().symbol, 'o');
        assert_eq!(buffer.get(4, 0).unwrap().symbol, 'a');
        assert_eq!(buffer.get(5, 0).unwrap().symbol, 'd');
        assert_eq!(buffer.get(6, 0).unwrap().symbol, 'i');
        assert_eq!(buffer.get(7, 0).unwrap().symbol, 'n');
        assert_eq!(buffer.get(8, 0).unwrap().symbol, 'g');
    }

    #[test]
    fn test_spinner_render_with_long_label_truncated() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(10, 5);
        let area = Rect::new(0, 0, 10, 5);

        let spinner = Spinner::new().label("This is a very long label");
        {
            let mut ctx = RenderContext::new(&mut buffer, area);
            spinner.render(&mut ctx);
        }

        // Check spinner character
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '⠋');

        // Check that label is truncated at boundary
        // Label starts at x=2, so we have 8 characters (x=2 to x=9)
        // "This is " is the truncated label
        assert_eq!(buffer.get(2, 0).unwrap().symbol, 'T');
        assert_eq!(buffer.get(9, 0).unwrap().symbol, ' '); // Space at end of "This is "
        assert_eq!(buffer.get(10, 0), None); // Out of bounds
    }

    #[test]
    fn test_spinner_render_with_custom_color() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(20, 5);
        let area = Rect::new(0, 0, 20, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let spinner = Spinner::new().fg(Color::RED);
        spinner.render(&mut ctx);

        // Check that color is applied
        let cell = buffer.get(0, 0).unwrap();
        assert_eq!(cell.fg, Some(Color::RED));
    }

    #[test]
    fn test_spinner_render_with_different_frame() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(20, 5);
        let area = Rect::new(0, 0, 20, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut spinner = Spinner::new();
        spinner.set_frame(1); // Second frame
        spinner.render(&mut ctx);

        // Check that second frame character is rendered
        let cell = buffer.get(0, 0).unwrap();
        assert_eq!(cell.symbol, '⠙'); // Dots style second frame
    }

    #[test]
    fn test_spinner_render_line_style() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(20, 5);
        let area = Rect::new(0, 0, 20, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let spinner = Spinner::new().style(SpinnerStyle::Line);
        spinner.render(&mut ctx);

        // Check that Line style first frame is rendered
        let cell = buffer.get(0, 0).unwrap();
        assert_eq!(cell.symbol, '|'); // Line style first frame
    }

    #[test]
    fn test_spinner_render_circle_style() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(20, 5);
        let area = Rect::new(0, 0, 20, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let spinner = Spinner::new().style(SpinnerStyle::Circle);
        spinner.render(&mut ctx);

        // Check that Circle style first frame is rendered
        let cell = buffer.get(0, 0).unwrap();
        assert_eq!(cell.symbol, '◐'); // Circle style first frame
    }

    #[test]
    fn test_spinner_render_arrow_style() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(20, 5);
        let area = Rect::new(0, 0, 20, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let spinner = Spinner::new().style(SpinnerStyle::Arrow);
        spinner.render(&mut ctx);

        // Check that Arrow style first frame is rendered
        let cell = buffer.get(0, 0).unwrap();
        assert_eq!(cell.symbol, '←'); // Arrow style first frame
    }

    #[test]
    fn test_spinner_render_box_style() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(20, 5);
        let area = Rect::new(0, 0, 20, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let spinner = Spinner::new().style(SpinnerStyle::Box);
        spinner.render(&mut ctx);

        // Check that Box style first frame is rendered
        let cell = buffer.get(0, 0).unwrap();
        assert_eq!(cell.symbol, '▖'); // Box style first frame
    }

    #[test]
    fn test_spinner_render_bounce_style() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(20, 5);
        let area = Rect::new(0, 0, 20, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let spinner = Spinner::new().style(SpinnerStyle::Bounce);
        spinner.render(&mut ctx);

        // Check that Bounce style first frame is rendered
        let cell = buffer.get(0, 0).unwrap();
        assert_eq!(cell.symbol, '⠁'); // Bounce style first frame
    }

    #[test]
    fn test_spinner_render_at_different_position() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(5, 3, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let spinner = Spinner::new();
        spinner.render(&mut ctx);

        // Check that spinner is rendered at correct position
        let cell = buffer.get(5, 3).unwrap();
        assert_eq!(cell.symbol, '⠋');
    }

    #[test]
    fn test_spinner_render_with_label_at_position() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(10, 5, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let spinner = Spinner::new().label("Test");
        spinner.render(&mut ctx);

        // Check spinner at position
        assert_eq!(buffer.get(10, 5).unwrap().symbol, '⠋');

        // Check label starts at x+2
        assert_eq!(buffer.get(12, 5).unwrap().symbol, 'T');
        assert_eq!(buffer.get(13, 5).unwrap().symbol, 'e');
        assert_eq!(buffer.get(14, 5).unwrap().symbol, 's');
        assert_eq!(buffer.get(15, 5).unwrap().symbol, 't');
    }

    #[test]
    fn test_spinner_render_animation_tick() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(20, 5);
        let area = Rect::new(0, 0, 20, 5);

        let mut spinner = Spinner::new();

        // Render first frame
        {
            let mut ctx = RenderContext::new(&mut buffer, area);
            spinner.render(&mut ctx);
        }
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '⠋');

        // Tick and render again
        spinner.tick();
        {
            let mut ctx = RenderContext::new(&mut buffer, area);
            spinner.render(&mut ctx);
        }
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '⠙');
    }

    #[test]
    fn test_spinner_render_with_unicode_label() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(20, 5);
        let area = Rect::new(0, 0, 20, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let spinner = Spinner::new().label("Загрузка");
        spinner.render(&mut ctx);

        // Check spinner
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '⠋');

        // Check unicode label
        assert_eq!(buffer.get(2, 0).unwrap().symbol, 'З');
        assert_eq!(buffer.get(3, 0).unwrap().symbol, 'а');
    }

    #[test]
    fn test_spinner_render_with_emoji_label() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(20, 5);
        let area = Rect::new(0, 0, 20, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let spinner = Spinner::new().label("⏳ Load");
        spinner.render(&mut ctx);

        // Check spinner
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '⠋');

        // Check emoji in label
        assert_eq!(buffer.get(2, 0).unwrap().symbol, '⏳');
        assert_eq!(buffer.get(4, 0).unwrap().symbol, 'L'); // After space
    }

    #[test]
    fn test_spinner_render_all_styles_first_frame() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let expected = [
            (SpinnerStyle::Dots, '⠋'),
            (SpinnerStyle::Line, '|'),
            (SpinnerStyle::Circle, '◐'),
            (SpinnerStyle::Arrow, '←'),
            (SpinnerStyle::Box, '▖'),
            (SpinnerStyle::Bounce, '⠁'),
        ];

        for (style, expected_char) in expected {
            let mut buffer = Buffer::new(20, 5);
            let area = Rect::new(0, 0, 20, 5);
            let mut ctx = RenderContext::new(&mut buffer, area);

            let spinner = Spinner::new().style(style);
            spinner.render(&mut ctx);

            let cell = buffer.get(0, 0).unwrap();
            assert_eq!(cell.symbol, expected_char, "Failed for style {:?}", style);
        }
    }

    #[test]
    fn test_spinner_render_multiple_frames() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(20, 5);
        let area = Rect::new(0, 0, 20, 5);

        let mut spinner = Spinner::new().style(SpinnerStyle::Line);
        let expected_frames = ['|', '/', '-', '\\'];

        for (i, &expected_char) in expected_frames.iter().enumerate() {
            {
                let mut ctx = RenderContext::new(&mut buffer, area);
                spinner.render(&mut ctx);
            }
            let cell = buffer.get(0, 0).unwrap();
            assert_eq!(cell.symbol, expected_char, "Frame {} failed", i);
            spinner.tick();
        }
    }

    #[test]
    fn test_spinner_render_no_color() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        let mut buffer = Buffer::new(20, 5);
        let area = Rect::new(0, 0, 20, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut spinner = Spinner::new();
        spinner.fg = None; // Explicitly set no color
        spinner.render(&mut ctx);

        // Check that spinner renders without color
        let cell = buffer.get(0, 0).unwrap();
        assert_eq!(cell.symbol, '⠋');
        assert_eq!(cell.fg, None);
    }

    #[test]
    fn test_spinner_render_small_area_with_label() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::traits::RenderContext;

        // Area only has room for spinner + 1 char of label
        let mut buffer = Buffer::new(3, 1);
        let area = Rect::new(0, 0, 3, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let spinner = Spinner::new().label("LongLabel");
        spinner.render(&mut ctx);

        // Spinner should render
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '⠋');

        // Only first char of label should fit (at x=2)
        assert_eq!(buffer.get(2, 0).unwrap().symbol, 'L');

        // Rest should be truncated
        assert_eq!(buffer.get(3, 0), None);
    }
}
