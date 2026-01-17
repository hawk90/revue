//! Render context for widget rendering

use crate::dom::NodeState;
use crate::layout::Rect;
use crate::render::{Buffer, Cell};
use crate::style::{Color, Style};
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

use super::event::FocusStyle;

/// Progress bar rendering configuration
pub struct ProgressBarConfig {
    /// X position
    pub x: u16,
    /// Y position
    pub y: u16,
    /// Total width of the bar
    pub width: u16,
    /// Progress value from 0.0 to 1.0
    pub progress: f32,
    /// Character for filled portion (e.g., '█')
    pub filled_char: char,
    /// Character for empty portion (e.g., '░')
    pub empty_char: char,
    /// Foreground color
    pub fg: Color,
}

/// Render context passed to widgets
pub struct RenderContext<'a> {
    /// Buffer to render into
    pub buffer: &'a mut Buffer,
    /// Available area for rendering
    pub area: Rect,
    /// Computed style from CSS cascade
    pub style: Option<&'a Style>,
    /// Current widget state
    pub state: Option<&'a NodeState>,
    /// Transition values for animations (property name -> current value)
    transitions: Option<&'a std::collections::HashMap<String, f32>>,
}

impl<'a> RenderContext<'a> {
    /// Create a basic render context (without style/state)
    pub fn new(buffer: &'a mut Buffer, area: Rect) -> Self {
        Self {
            buffer,
            area,
            style: None,
            state: None,
            transitions: None,
        }
    }

    /// Create a render context with style
    pub fn with_style(buffer: &'a mut Buffer, area: Rect, style: &'a Style) -> Self {
        Self {
            buffer,
            area,
            style: Some(style),
            state: None,
            transitions: None,
        }
    }

    /// Create a full render context
    pub fn full(
        buffer: &'a mut Buffer,
        area: Rect,
        style: &'a Style,
        state: &'a NodeState,
    ) -> Self {
        Self {
            buffer,
            area,
            style: Some(style),
            state: Some(state),
            transitions: None,
        }
    }

    /// Set transition values for this render context
    pub fn with_transitions(
        mut self,
        transitions: &'a std::collections::HashMap<String, f32>,
    ) -> Self {
        self.transitions = Some(transitions);
        self
    }

    /// Get current transition value for a property
    pub fn transition(&self, property: &str) -> Option<f32> {
        self.transitions.and_then(|t| t.get(property).copied())
    }

    /// Get transition value with a default fallback
    pub fn transition_or(&self, property: &str, default: f32) -> f32 {
        self.transition(property).unwrap_or(default)
    }

    /// Check if focused
    pub fn is_focused(&self) -> bool {
        self.state.map(|s| s.focused).unwrap_or(false)
    }

    /// Check if hovered
    pub fn is_hovered(&self) -> bool {
        self.state.map(|s| s.hovered).unwrap_or(false)
    }

    /// Check if disabled
    pub fn is_disabled(&self) -> bool {
        self.state.map(|s| s.disabled).unwrap_or(false)
    }

    // =========================================================================
    // Drawing utilities
    // =========================================================================

    /// Helper: Draw text with custom cell styling, handling wide characters correctly.
    fn draw_text_with_style<F>(&mut self, x: u16, y: u16, text: &str, mut make_cell: F)
    where
        F: FnMut(char) -> Cell,
    {
        let mut offset = 0u16;
        for ch in text.chars() {
            let width = ch.width().unwrap_or(0) as u16;
            if width == 0 {
                continue;
            }
            self.buffer.set(x.saturating_add(offset), y, make_cell(ch));
            for i in 1..width {
                self.buffer
                    .set(x.saturating_add(offset + i), y, Cell::continuation());
            }
            offset = offset.saturating_add(width);
        }
    }

    /// Helper: Draw text clipped to max_width, handling wide characters correctly.
    fn draw_text_clipped_with_style<F>(
        &mut self,
        x: u16,
        y: u16,
        text: &str,
        max_width: u16,
        mut make_cell: F,
    ) where
        F: FnMut(char) -> Cell,
    {
        let mut offset = 0u16;
        for ch in text.chars() {
            let width = ch.width().unwrap_or(0) as u16;
            if width == 0 {
                continue;
            }
            if offset.saturating_add(width) > max_width {
                break;
            }
            self.buffer.set(x.saturating_add(offset), y, make_cell(ch));
            for i in 1..width {
                self.buffer
                    .set(x.saturating_add(offset + i), y, Cell::continuation());
            }
            offset = offset.saturating_add(width);
        }
    }

    /// Draw a single character at position
    #[inline]
    pub fn draw_char(&mut self, x: u16, y: u16, ch: char, fg: Color) {
        let cell = Cell::new(ch).fg(fg);
        self.buffer.set(x, y, cell);
    }

    /// Draw a character with background color
    #[inline]
    pub fn draw_char_bg(&mut self, x: u16, y: u16, ch: char, fg: Color, bg: Color) {
        let cell = Cell::new(ch).fg(fg).bg(bg);
        self.buffer.set(x, y, cell);
    }

    /// Draw a bold character
    #[inline]
    pub fn draw_char_bold(&mut self, x: u16, y: u16, ch: char, fg: Color) {
        let cell = Cell::new(ch).fg(fg).bold();
        self.buffer.set(x, y, cell);
    }

    /// Draw text at position
    pub fn draw_text(&mut self, x: u16, y: u16, text: &str, fg: Color) {
        self.draw_text_with_style(x, y, text, |ch| Cell::new(ch).fg(fg));
    }

    /// Draw text with background color
    pub fn draw_text_bg(&mut self, x: u16, y: u16, text: &str, fg: Color, bg: Color) {
        self.draw_text_with_style(x, y, text, |ch| Cell::new(ch).fg(fg).bg(bg));
    }

    /// Draw bold text
    pub fn draw_text_bold(&mut self, x: u16, y: u16, text: &str, fg: Color) {
        self.draw_text_with_style(x, y, text, |ch| Cell::new(ch).fg(fg).bold());
    }

    /// Draw a horizontal line
    pub fn draw_hline(&mut self, x: u16, y: u16, len: u16, ch: char, fg: Color) {
        for i in 0..len {
            self.draw_char(x + i, y, ch, fg);
        }
    }

    /// Draw a vertical line
    pub fn draw_vline(&mut self, x: u16, y: u16, len: u16, ch: char, fg: Color) {
        for i in 0..len {
            self.draw_char(x, y + i, ch, fg);
        }
    }

    /// Draw a box with rounded corners
    pub fn draw_box_rounded(&mut self, x: u16, y: u16, w: u16, h: u16, fg: Color) {
        if w < 2 || h < 2 {
            return;
        }
        self.draw_char(x, y, '╭', fg);
        self.draw_char(x + w - 1, y, '╮', fg);
        self.draw_char(x, y + h - 1, '╰', fg);
        self.draw_char(x + w - 1, y + h - 1, '╯', fg);
        self.draw_hline(x + 1, y, w - 2, '─', fg);
        self.draw_hline(x + 1, y + h - 1, w - 2, '─', fg);
        self.draw_vline(x, y + 1, h - 2, '│', fg);
        self.draw_vline(x + w - 1, y + 1, h - 2, '│', fg);
    }

    /// Draw a box without top border (for custom multi-color headers)
    pub fn draw_box_no_top(&mut self, x: u16, y: u16, w: u16, h: u16, fg: Color) {
        if w < 2 || h < 2 {
            return;
        }
        self.draw_char(x, y + h - 1, '╰', fg);
        self.draw_char(x + w - 1, y + h - 1, '╯', fg);
        self.draw_hline(x + 1, y + h - 1, w - 2, '─', fg);
        self.draw_vline(x, y + 1, h - 2, '│', fg);
        self.draw_vline(x + w - 1, y + 1, h - 2, '│', fg);
    }

    /// Draw a complete header line with corners for use with draw_box_no_top
    pub fn draw_header_line(
        &mut self,
        x: u16,
        y: u16,
        width: u16,
        parts: &[(&str, Color)],
        border_color: Color,
    ) {
        if width < 4 {
            return;
        }
        self.draw_text(x, y, "╭─", border_color);
        let mut pos = x + 2;
        for (text, color) in parts {
            self.draw_text(pos, y, text, *color);
            pos += text.width() as u16;
        }
        let end = x + width - 1;
        while pos < end {
            self.draw_char(pos, y, '─', border_color);
            pos += 1;
        }
        self.draw_char(end, y, '╮', border_color);
    }

    /// Draw a box with single border
    pub fn draw_box_single(&mut self, x: u16, y: u16, w: u16, h: u16, fg: Color) {
        if w < 2 || h < 2 {
            return;
        }
        self.draw_char(x, y, '┌', fg);
        self.draw_char(x + w - 1, y, '┐', fg);
        self.draw_char(x, y + h - 1, '└', fg);
        self.draw_char(x + w - 1, y + h - 1, '┘', fg);
        self.draw_hline(x + 1, y, w - 2, '─', fg);
        self.draw_hline(x + 1, y + h - 1, w - 2, '─', fg);
        self.draw_vline(x, y + 1, h - 2, '│', fg);
        self.draw_vline(x + w - 1, y + 1, h - 2, '│', fg);
    }

    /// Draw a box with double border
    pub fn draw_box_double(&mut self, x: u16, y: u16, w: u16, h: u16, fg: Color) {
        if w < 2 || h < 2 {
            return;
        }
        self.draw_char(x, y, '╔', fg);
        self.draw_char(x + w - 1, y, '╗', fg);
        self.draw_char(x, y + h - 1, '╚', fg);
        self.draw_char(x + w - 1, y + h - 1, '╝', fg);
        self.draw_hline(x + 1, y, w - 2, '═', fg);
        self.draw_hline(x + 1, y + h - 1, w - 2, '═', fg);
        self.draw_vline(x, y + 1, h - 2, '║', fg);
        self.draw_vline(x + w - 1, y + 1, h - 2, '║', fg);
    }

    /// Fill a rectangular area with a character
    pub fn fill(&mut self, x: u16, y: u16, w: u16, h: u16, ch: char, fg: Color) {
        for dy in 0..h {
            for dx in 0..w {
                self.draw_char(x + dx, y + dy, ch, fg);
            }
        }
    }

    /// Fill with background color
    pub fn fill_bg(&mut self, x: u16, y: u16, w: u16, h: u16, bg: Color) {
        for dy in 0..h {
            for dx in 0..w {
                let mut cell = Cell::new(' ');
                cell.bg = Some(bg);
                self.buffer.set(x + dx, y + dy, cell);
            }
        }
    }

    /// Clear area (fill with spaces)
    pub fn clear(&mut self, x: u16, y: u16, w: u16, h: u16) {
        for dy in 0..h {
            for dx in 0..w {
                self.buffer.set(x + dx, y + dy, Cell::empty());
            }
        }
    }

    // =========================================================================
    // Bounded/clipped drawing utilities
    // =========================================================================

    /// Draw text clipped to max_width (stops drawing at boundary)
    pub fn draw_text_clipped(&mut self, x: u16, y: u16, text: &str, fg: Color, max_width: u16) {
        self.draw_text_clipped_with_style(x, y, text, max_width, |ch| Cell::new(ch).fg(fg));
    }

    /// Draw bold text clipped to max_width
    pub fn draw_text_clipped_bold(
        &mut self,
        x: u16,
        y: u16,
        text: &str,
        fg: Color,
        max_width: u16,
    ) {
        self.draw_text_clipped_with_style(x, y, text, max_width, |ch| Cell::new(ch).fg(fg).bold());
    }

    /// Draw dimmed text
    pub fn draw_text_dim(&mut self, x: u16, y: u16, text: &str, fg: Color) {
        self.draw_text_with_style(x, y, text, |ch| Cell::new(ch).fg(fg).dim());
    }

    /// Draw italic text
    pub fn draw_text_italic(&mut self, x: u16, y: u16, text: &str, fg: Color) {
        self.draw_text_with_style(x, y, text, |ch| Cell::new(ch).fg(fg).italic());
    }

    /// Draw underlined text
    pub fn draw_text_underline(&mut self, x: u16, y: u16, text: &str, fg: Color) {
        self.draw_text_with_style(x, y, text, |ch| Cell::new(ch).fg(fg).underline());
    }

    /// Draw text centered within a given width
    pub fn draw_text_centered(&mut self, x: u16, y: u16, width: u16, text: &str, fg: Color) {
        let text_width = text.width() as u16;
        let start_x = if text_width >= width {
            x
        } else {
            x + (width - text_width) / 2
        };
        self.draw_text_clipped(start_x, y, text, fg, width);
    }

    /// Draw text right-aligned within a given width
    pub fn draw_text_right(&mut self, x: u16, y: u16, width: u16, text: &str, fg: Color) {
        let text_width = text.width() as u16;
        let start_x = if text_width >= width {
            x
        } else {
            x + width - text_width
        };
        self.draw_text_clipped(start_x, y, text, fg, width);
    }

    // =========================================================================
    // Box with title utilities
    // =========================================================================

    /// Draw a rounded box with a title on the top border
    pub fn draw_box_titled(&mut self, x: u16, y: u16, w: u16, h: u16, title: &str, fg: Color) {
        if w < 2 || h < 2 {
            return;
        }
        self.draw_char(x, y, '╭', fg);
        self.draw_char(x + w - 1, y, '╮', fg);
        self.draw_char(x, y + h - 1, '╰', fg);
        self.draw_char(x + w - 1, y + h - 1, '╯', fg);
        self.draw_top_border_with_title(x, y, w, title, '─', fg);
        self.draw_hline(x + 1, y + h - 1, w - 2, '─', fg);
        self.draw_vline(x, y + 1, h - 2, '│', fg);
        self.draw_vline(x + w - 1, y + 1, h - 2, '│', fg);
    }

    /// Draw a single-line box with a title
    pub fn draw_box_titled_single(
        &mut self,
        x: u16,
        y: u16,
        w: u16,
        h: u16,
        title: &str,
        fg: Color,
    ) {
        if w < 2 || h < 2 {
            return;
        }
        self.draw_char(x, y, '┌', fg);
        self.draw_char(x + w - 1, y, '┐', fg);
        self.draw_char(x, y + h - 1, '└', fg);
        self.draw_char(x + w - 1, y + h - 1, '┘', fg);
        self.draw_top_border_with_title(x, y, w, title, '─', fg);
        self.draw_hline(x + 1, y + h - 1, w - 2, '─', fg);
        self.draw_vline(x, y + 1, h - 2, '│', fg);
        self.draw_vline(x + w - 1, y + 1, h - 2, '│', fg);
    }

    /// Draw a double-line box with a title
    pub fn draw_box_titled_double(
        &mut self,
        x: u16,
        y: u16,
        w: u16,
        h: u16,
        title: &str,
        fg: Color,
    ) {
        if w < 2 || h < 2 {
            return;
        }
        self.draw_char(x, y, '╔', fg);
        self.draw_char(x + w - 1, y, '╗', fg);
        self.draw_char(x, y + h - 1, '╚', fg);
        self.draw_char(x + w - 1, y + h - 1, '╝', fg);
        self.draw_top_border_with_title(x, y, w, title, '═', fg);
        self.draw_hline(x + 1, y + h - 1, w - 2, '═', fg);
        self.draw_vline(x, y + 1, h - 2, '║', fg);
        self.draw_vline(x + w - 1, y + 1, h - 2, '║', fg);
    }

    /// Helper: Draw top border with embedded title using O(n) iterator
    fn draw_top_border_with_title(
        &mut self,
        x: u16,
        y: u16,
        w: u16,
        title: &str,
        border_char: char,
        fg: Color,
    ) {
        let title_start = 2u16;
        let border_end = w.saturating_sub(1);
        let mut title_chars = title.chars().peekable();
        let mut pos = 1u16;

        while pos < border_end {
            if pos >= title_start {
                if let Some(ch) = title_chars.next() {
                    let char_width = ch.width().unwrap_or(0) as u16;
                    if char_width == 0 {
                        continue;
                    }
                    if pos + char_width > border_end {
                        break;
                    }
                    self.draw_char(x + pos, y, ch, fg);
                    for i in 1..char_width {
                        self.buffer.set(x + pos + i, y, Cell::continuation());
                    }
                    pos += char_width;
                    continue;
                }
            }
            self.draw_char(x + pos, y, border_char, fg);
            pos += 1;
        }
    }

    // =========================================================================
    // Progress bar utilities
    // =========================================================================

    /// Draw a horizontal progress bar
    pub fn draw_progress_bar(&mut self, config: &ProgressBarConfig) {
        let progress = config.progress.clamp(0.0, 1.0);
        let filled = (config.width as f32 * progress).round() as u16;

        for i in 0..config.width {
            let ch = if i < filled {
                config.filled_char
            } else {
                config.empty_char
            };
            self.draw_char(config.x + i, config.y, ch, config.fg);
        }
    }

    /// Draw a progress bar with percentage label
    pub fn draw_progress_bar_labeled(
        &mut self,
        x: u16,
        y: u16,
        bar_width: u16,
        progress: f32,
        fg: Color,
    ) {
        let progress = progress.clamp(0.0, 1.0);
        let percent = (progress * 100.0).round() as u8;
        let label = format!("{:>3}%", percent);

        self.draw_text(x, y, &label, fg);
        let bar_x = x + 4;
        self.draw_char(bar_x, y, '[', fg);
        self.draw_progress_bar(&ProgressBarConfig {
            x: bar_x + 1,
            y,
            width: bar_width,
            progress,
            filled_char: '█',
            empty_char: '░',
            fg,
        });
        self.draw_char(bar_x + 1 + bar_width, y, ']', fg);
    }

    // =========================================================================
    // Segment-based drawing utilities
    // =========================================================================

    /// Draw multiple text segments with different colors on one line
    pub fn draw_segments(&mut self, x: u16, y: u16, segments: &[(&str, Color)]) -> u16 {
        let mut cx = x;
        for (text, color) in segments {
            self.draw_text(cx, y, text, *color);
            cx += text.width() as u16;
        }
        cx
    }

    /// Draw segments with a separator between them
    pub fn draw_segments_sep(
        &mut self,
        x: u16,
        y: u16,
        segments: &[(&str, Color)],
        sep: &str,
        sep_color: Color,
    ) -> u16 {
        let mut cx = x;
        for (i, (text, color)) in segments.iter().enumerate() {
            if i > 0 {
                self.draw_text(cx, y, sep, sep_color);
                cx += sep.width() as u16;
            }
            self.draw_text(cx, y, text, *color);
            cx += text.width() as u16;
        }
        cx
    }

    /// Draw key hints (key in bold color, action in dim)
    pub fn draw_key_hints(
        &mut self,
        x: u16,
        y: u16,
        hints: &[(&str, &str)],
        key_color: Color,
        action_color: Color,
    ) -> u16 {
        let mut cx = x;
        for (key, action) in hints {
            self.draw_text_bold(cx, y, key, key_color);
            cx += key.width() as u16 + 1;
            self.draw_text(cx, y, action, action_color);
            cx += action.width() as u16 + 2;
        }
        cx
    }

    /// Draw text with selection styling (bold + highlight color when selected)
    pub fn draw_text_selectable(
        &mut self,
        x: u16,
        y: u16,
        text: &str,
        selected: bool,
        normal_color: Color,
        selected_color: Color,
    ) {
        if selected {
            self.draw_text_bold(x, y, text, selected_color);
        } else {
            self.draw_text(x, y, text, normal_color);
        }
    }

    /// Get color based on value thresholds (for metrics)
    pub fn metric_color(
        value: u8,
        mid: u8,
        high: u8,
        low_color: Color,
        mid_color: Color,
        high_color: Color,
    ) -> Color {
        if value < mid {
            low_color
        } else if value < high {
            mid_color
        } else {
            high_color
        }
    }

    // =========================================================================
    // CSS Style Integration
    // =========================================================================

    /// Get foreground color from CSS style or use default
    pub fn css_color(&self, default: Color) -> Color {
        self.style
            .map(|s| {
                let c = s.visual.color;
                if c == Color::default() {
                    default
                } else {
                    c
                }
            })
            .unwrap_or(default)
    }

    /// Get background color from CSS style or use default
    pub fn css_background(&self, default: Color) -> Color {
        self.style
            .map(|s| {
                let c = s.visual.background;
                if c == Color::default() {
                    default
                } else {
                    c
                }
            })
            .unwrap_or(default)
    }

    /// Get border color from CSS style or use default
    pub fn css_border_color(&self, default: Color) -> Color {
        self.style
            .map(|s| {
                let c = s.visual.border_color;
                if c == Color::default() {
                    default
                } else {
                    c
                }
            })
            .unwrap_or(default)
    }

    /// Get opacity from CSS style (1.0 = fully opaque)
    pub fn css_opacity(&self) -> f32 {
        self.style.map(|s| s.visual.opacity).unwrap_or(1.0)
    }

    /// Check if visible according to CSS
    pub fn css_visible(&self) -> bool {
        self.style.map(|s| s.visual.visible).unwrap_or(true)
    }

    /// Get padding from CSS style
    pub fn css_padding(&self) -> crate::style::Spacing {
        self.style.map(|s| s.spacing.padding).unwrap_or_default()
    }

    /// Get margin from CSS style
    pub fn css_margin(&self) -> crate::style::Spacing {
        self.style.map(|s| s.spacing.margin).unwrap_or_default()
    }

    /// Get width from CSS style
    pub fn css_width(&self) -> crate::style::Size {
        self.style.map(|s| s.sizing.width).unwrap_or_default()
    }

    /// Get height from CSS style
    pub fn css_height(&self) -> crate::style::Size {
        self.style.map(|s| s.sizing.height).unwrap_or_default()
    }

    /// Get border style from CSS
    pub fn css_border_style(&self) -> crate::style::BorderStyle {
        self.style
            .map(|s| s.visual.border_style)
            .unwrap_or_default()
    }

    /// Get gap from CSS style (for flex/grid layouts)
    pub fn css_gap(&self) -> u16 {
        self.style.map(|s| s.layout.gap).unwrap_or(0)
    }

    // NOTE: Color resolution is handled by WidgetState::resolve_fg/resolve_bg/resolve_colors_interactive
    // Use self.state.resolve_colors_interactive(ctx.style, default_fg, default_bg) for widget color resolution

    // =========================================================================
    // Accessibility - Focus Indicators
    // =========================================================================

    /// Draw a focus ring around an area
    pub fn draw_focus_ring(
        &mut self,
        x: u16,
        y: u16,
        w: u16,
        h: u16,
        color: Color,
        style: FocusStyle,
    ) {
        if w < 2 || h < 2 {
            return;
        }

        let (h_char, v_char, tl, tr, bl, br) = match style {
            FocusStyle::Solid => ('─', '│', '┌', '┐', '└', '┘'),
            FocusStyle::Rounded => ('─', '│', '╭', '╮', '╰', '╯'),
            FocusStyle::Double => ('═', '║', '╔', '╗', '╚', '╝'),
            FocusStyle::Dotted => ('╌', '╎', '┌', '┐', '└', '┘'),
            FocusStyle::Bold => ('━', '┃', '┏', '┓', '┗', '┛'),
            FocusStyle::Ascii => ('-', '|', '+', '+', '+', '+'),
        };

        self.draw_char(x, y, tl, color);
        self.draw_char(x + w - 1, y, tr, color);
        self.draw_char(x, y + h - 1, bl, color);
        self.draw_char(x + w - 1, y + h - 1, br, color);
        self.draw_hline(x + 1, y, w - 2, h_char, color);
        self.draw_hline(x + 1, y + h - 1, w - 2, h_char, color);
        self.draw_vline(x, y + 1, h - 2, v_char, color);
        self.draw_vline(x + w - 1, y + 1, h - 2, v_char, color);
    }

    /// Draw a focus ring with automatic style based on context
    pub fn draw_focus_ring_auto(&mut self, x: u16, y: u16, w: u16, h: u16, color: Color) {
        self.draw_focus_ring(x, y, w, h, color, FocusStyle::Rounded);
    }

    /// Draw a focus underline (for inline elements)
    pub fn draw_focus_underline(&mut self, x: u16, y: u16, w: u16, color: Color) {
        for i in 0..w {
            let cell = Cell::new('▔').fg(color);
            self.buffer.set(x + i, y, cell);
        }
    }

    /// Draw a focus indicator at a specific position
    pub fn draw_focus_marker(&mut self, x: u16, y: u16, color: Color) {
        self.draw_char(x, y, '▶', color);
    }

    /// Draw a focus indicator on the left side of an item
    pub fn draw_focus_marker_left(&mut self, y: u16, color: Color) {
        if self.area.x > 0 {
            self.draw_char(self.area.x - 1, y, '▶', color);
        } else {
            self.draw_char(self.area.x, y, '▶', color);
        }
    }

    /// Invert colors in a region (for high contrast focus indication)
    pub fn invert_colors(&mut self, x: u16, y: u16, w: u16, h: u16) {
        for dy in 0..h {
            for dx in 0..w {
                if let Some(cell) = self.buffer.get_mut(x + dx, y + dy) {
                    let old_fg = cell.fg;
                    let old_bg = cell.bg;
                    cell.fg = old_bg;
                    cell.bg = old_fg;
                }
            }
        }
    }

    /// Add reverse video effect to indicate focus
    pub fn draw_focus_reverse(&mut self, x: u16, y: u16, w: u16, h: u16) {
        self.invert_colors(x, y, w, h);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::Style;
    use std::collections::HashMap;

    fn test_buffer() -> Buffer {
        Buffer::new(20, 10)
    }

    fn test_area() -> Rect {
        Rect::new(0, 0, 20, 10)
    }

    // =========================================================================
    // Constructor tests
    // =========================================================================

    #[test]
    fn test_render_context_new() {
        let mut buffer = test_buffer();
        let area = test_area();
        let ctx = RenderContext::new(&mut buffer, area);

        assert_eq!(ctx.area, area);
        assert!(ctx.style.is_none());
        assert!(ctx.state.is_none());
    }

    #[test]
    fn test_render_context_with_style() {
        let mut buffer = test_buffer();
        let area = test_area();
        let style = Style::default();
        let ctx = RenderContext::with_style(&mut buffer, area, &style);

        assert!(ctx.style.is_some());
        assert!(ctx.state.is_none());
    }

    #[test]
    fn test_render_context_full() {
        let mut buffer = test_buffer();
        let area = test_area();
        let style = Style::default();
        let state = NodeState::default();
        let ctx = RenderContext::full(&mut buffer, area, &style, &state);

        assert!(ctx.style.is_some());
        assert!(ctx.state.is_some());
    }

    #[test]
    fn test_render_context_with_transitions() {
        let mut buffer = test_buffer();
        let area = test_area();
        let mut transitions = HashMap::new();
        transitions.insert("opacity".to_string(), 0.5f32);

        let ctx = RenderContext::new(&mut buffer, area).with_transitions(&transitions);

        assert_eq!(ctx.transition("opacity"), Some(0.5));
        assert_eq!(ctx.transition("nonexistent"), None);
    }

    #[test]
    fn test_transition_or() {
        let mut buffer = test_buffer();
        let area = test_area();
        let mut transitions = HashMap::new();
        transitions.insert("opacity".to_string(), 0.5f32);

        let ctx = RenderContext::new(&mut buffer, area).with_transitions(&transitions);

        assert_eq!(ctx.transition_or("opacity", 1.0), 0.5);
        assert_eq!(ctx.transition_or("nonexistent", 1.0), 1.0);
    }

    // =========================================================================
    // State check tests
    // =========================================================================

    #[test]
    fn test_is_focused_no_state() {
        let mut buffer = test_buffer();
        let ctx = RenderContext::new(&mut buffer, test_area());
        assert!(!ctx.is_focused());
    }

    #[test]
    fn test_is_focused_with_state() {
        let mut buffer = test_buffer();
        let style = Style::default();
        let mut state = NodeState::default();
        state.focused = true;
        let ctx = RenderContext::full(&mut buffer, test_area(), &style, &state);
        assert!(ctx.is_focused());
    }

    #[test]
    fn test_is_hovered() {
        let mut buffer = test_buffer();
        let style = Style::default();
        let mut state = NodeState::default();
        state.hovered = true;
        let ctx = RenderContext::full(&mut buffer, test_area(), &style, &state);
        assert!(ctx.is_hovered());
    }

    #[test]
    fn test_is_disabled() {
        let mut buffer = test_buffer();
        let style = Style::default();
        let mut state = NodeState::default();
        state.disabled = true;
        let ctx = RenderContext::full(&mut buffer, test_area(), &style, &state);
        assert!(ctx.is_disabled());
    }

    // =========================================================================
    // Drawing tests
    // =========================================================================

    #[test]
    fn test_draw_char() {
        let mut buffer = test_buffer();
        let mut ctx = RenderContext::new(&mut buffer, test_area());
        let color = Color::rgb(255, 255, 255);

        ctx.draw_char(0, 0, 'A', color);

        let cell = buffer.get(0, 0).unwrap();
        assert_eq!(cell.symbol, 'A');
    }

    #[test]
    fn test_draw_char_bg() {
        let mut buffer = test_buffer();
        let mut ctx = RenderContext::new(&mut buffer, test_area());
        let fg = Color::rgb(255, 255, 255);
        let bg = Color::rgb(0, 0, 0);

        ctx.draw_char_bg(0, 0, 'X', fg, bg);

        let cell = buffer.get(0, 0).unwrap();
        assert_eq!(cell.symbol, 'X');
        assert_eq!(cell.bg, Some(bg));
    }

    #[test]
    fn test_draw_char_bold() {
        use crate::render::Modifier;

        let mut buffer = test_buffer();
        let mut ctx = RenderContext::new(&mut buffer, test_area());
        let color = Color::rgb(255, 255, 255);

        ctx.draw_char_bold(0, 0, 'B', color);

        let cell = buffer.get(0, 0).unwrap();
        assert_eq!(cell.symbol, 'B');
        assert!(cell.modifier.contains(Modifier::BOLD));
    }

    #[test]
    fn test_draw_text() {
        let mut buffer = test_buffer();
        let mut ctx = RenderContext::new(&mut buffer, test_area());
        let color = Color::rgb(255, 255, 255);

        ctx.draw_text(0, 0, "Hello", color);

        assert_eq!(buffer.get(0, 0).unwrap().symbol, 'H');
        assert_eq!(buffer.get(1, 0).unwrap().symbol, 'e');
        assert_eq!(buffer.get(2, 0).unwrap().symbol, 'l');
        assert_eq!(buffer.get(3, 0).unwrap().symbol, 'l');
        assert_eq!(buffer.get(4, 0).unwrap().symbol, 'o');
    }

    #[test]
    fn test_draw_text_bg() {
        let mut buffer = test_buffer();
        let mut ctx = RenderContext::new(&mut buffer, test_area());
        let fg = Color::rgb(255, 255, 255);
        let bg = Color::rgb(100, 100, 100);

        ctx.draw_text_bg(0, 0, "Hi", fg, bg);

        assert_eq!(buffer.get(0, 0).unwrap().bg, Some(bg));
        assert_eq!(buffer.get(1, 0).unwrap().bg, Some(bg));
    }

    #[test]
    fn test_draw_text_bold() {
        use crate::render::Modifier;

        let mut buffer = test_buffer();
        let mut ctx = RenderContext::new(&mut buffer, test_area());
        let color = Color::rgb(255, 255, 255);

        ctx.draw_text_bold(0, 0, "Bold", color);

        assert!(buffer.get(0, 0).unwrap().modifier.contains(Modifier::BOLD));
    }

    #[test]
    fn test_draw_hline() {
        let mut buffer = test_buffer();
        let mut ctx = RenderContext::new(&mut buffer, test_area());
        let color = Color::rgb(255, 255, 255);

        ctx.draw_hline(0, 0, 5, '-', color);

        for i in 0..5 {
            assert_eq!(buffer.get(i, 0).unwrap().symbol, '-');
        }
    }

    #[test]
    fn test_draw_vline() {
        let mut buffer = test_buffer();
        let mut ctx = RenderContext::new(&mut buffer, test_area());
        let color = Color::rgb(255, 255, 255);

        ctx.draw_vline(0, 0, 5, '|', color);

        for i in 0..5 {
            assert_eq!(buffer.get(0, i).unwrap().symbol, '|');
        }
    }

    #[test]
    fn test_draw_box_rounded() {
        let mut buffer = test_buffer();
        let mut ctx = RenderContext::new(&mut buffer, test_area());
        let color = Color::rgb(255, 255, 255);

        ctx.draw_box_rounded(0, 0, 5, 3, color);

        assert_eq!(buffer.get(0, 0).unwrap().symbol, '╭');
        assert_eq!(buffer.get(4, 0).unwrap().symbol, '╮');
        assert_eq!(buffer.get(0, 2).unwrap().symbol, '╰');
        assert_eq!(buffer.get(4, 2).unwrap().symbol, '╯');
    }

    #[test]
    fn test_draw_box_rounded_too_small() {
        let mut buffer = test_buffer();
        let mut ctx = RenderContext::new(&mut buffer, test_area());
        let color = Color::rgb(255, 255, 255);

        // Should not panic with small dimensions
        ctx.draw_box_rounded(0, 0, 1, 1, color);
    }

    #[test]
    fn test_draw_box_single() {
        let mut buffer = test_buffer();
        let mut ctx = RenderContext::new(&mut buffer, test_area());
        let color = Color::rgb(255, 255, 255);

        ctx.draw_box_single(0, 0, 5, 3, color);

        assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
        assert_eq!(buffer.get(4, 0).unwrap().symbol, '┐');
    }

    #[test]
    fn test_draw_box_double() {
        let mut buffer = test_buffer();
        let mut ctx = RenderContext::new(&mut buffer, test_area());
        let color = Color::rgb(255, 255, 255);

        ctx.draw_box_double(0, 0, 5, 3, color);

        assert_eq!(buffer.get(0, 0).unwrap().symbol, '╔');
        assert_eq!(buffer.get(4, 0).unwrap().symbol, '╗');
    }

    #[test]
    fn test_fill() {
        let mut buffer = test_buffer();
        let mut ctx = RenderContext::new(&mut buffer, test_area());
        let color = Color::rgb(255, 255, 255);

        ctx.fill(0, 0, 3, 2, '#', color);

        for y in 0..2 {
            for x in 0..3 {
                assert_eq!(buffer.get(x, y).unwrap().symbol, '#');
            }
        }
    }

    #[test]
    fn test_fill_bg() {
        let mut buffer = test_buffer();
        let mut ctx = RenderContext::new(&mut buffer, test_area());
        let bg = Color::rgb(100, 100, 100);

        ctx.fill_bg(0, 0, 3, 2, bg);

        for y in 0..2 {
            for x in 0..3 {
                assert_eq!(buffer.get(x, y).unwrap().bg, Some(bg));
            }
        }
    }

    #[test]
    fn test_clear() {
        let mut buffer = test_buffer();
        let mut ctx = RenderContext::new(&mut buffer, test_area());
        let color = Color::rgb(255, 255, 255);

        // First fill with content
        ctx.fill(0, 0, 3, 2, '#', color);
        // Then clear
        ctx.clear(0, 0, 3, 2);

        for y in 0..2 {
            for x in 0..3 {
                assert_eq!(buffer.get(x, y).unwrap().symbol, ' ');
            }
        }
    }

    #[test]
    fn test_draw_text_clipped() {
        let mut buffer = test_buffer();
        let mut ctx = RenderContext::new(&mut buffer, test_area());
        let color = Color::rgb(255, 255, 255);

        ctx.draw_text_clipped(0, 0, "Hello World", color, 5);

        assert_eq!(buffer.get(4, 0).unwrap().symbol, 'o');
        // Should not draw beyond max_width
        assert_eq!(buffer.get(5, 0).unwrap().symbol, ' ');
    }

    #[test]
    fn test_draw_text_centered() {
        let mut buffer = test_buffer();
        let mut ctx = RenderContext::new(&mut buffer, test_area());
        let color = Color::rgb(255, 255, 255);

        ctx.draw_text_centered(0, 0, 10, "Hi", color);

        // "Hi" is 2 chars, centered in 10 width = starts at position 4
        assert_eq!(buffer.get(4, 0).unwrap().symbol, 'H');
        assert_eq!(buffer.get(5, 0).unwrap().symbol, 'i');
    }

    #[test]
    fn test_draw_text_right() {
        let mut buffer = test_buffer();
        let mut ctx = RenderContext::new(&mut buffer, test_area());
        let color = Color::rgb(255, 255, 255);

        ctx.draw_text_right(0, 0, 10, "Hi", color);

        // "Hi" is 2 chars, right-aligned in 10 width = starts at position 8
        assert_eq!(buffer.get(8, 0).unwrap().symbol, 'H');
        assert_eq!(buffer.get(9, 0).unwrap().symbol, 'i');
    }

    #[test]
    fn test_draw_text_dim() {
        use crate::render::Modifier;

        let mut buffer = test_buffer();
        let mut ctx = RenderContext::new(&mut buffer, test_area());
        let color = Color::rgb(255, 255, 255);

        ctx.draw_text_dim(0, 0, "dim", color);

        assert!(buffer.get(0, 0).unwrap().modifier.contains(Modifier::DIM));
    }

    #[test]
    fn test_draw_text_italic() {
        use crate::render::Modifier;

        let mut buffer = test_buffer();
        let mut ctx = RenderContext::new(&mut buffer, test_area());
        let color = Color::rgb(255, 255, 255);

        ctx.draw_text_italic(0, 0, "italic", color);

        assert!(buffer
            .get(0, 0)
            .unwrap()
            .modifier
            .contains(Modifier::ITALIC));
    }

    #[test]
    fn test_draw_text_underline() {
        use crate::render::Modifier;

        let mut buffer = test_buffer();
        let mut ctx = RenderContext::new(&mut buffer, test_area());
        let color = Color::rgb(255, 255, 255);

        ctx.draw_text_underline(0, 0, "underline", color);

        assert!(buffer
            .get(0, 0)
            .unwrap()
            .modifier
            .contains(Modifier::UNDERLINE));
    }

    // =========================================================================
    // Progress bar tests
    // =========================================================================

    #[test]
    fn test_draw_progress_bar() {
        let mut buffer = test_buffer();
        let mut ctx = RenderContext::new(&mut buffer, test_area());

        let config = ProgressBarConfig {
            x: 0,
            y: 0,
            width: 10,
            progress: 0.5,
            filled_char: '█',
            empty_char: '░',
            fg: Color::rgb(255, 255, 255),
        };

        ctx.draw_progress_bar(&config);

        // 50% of 10 = 5 filled
        for i in 0..5 {
            assert_eq!(buffer.get(i, 0).unwrap().symbol, '█');
        }
        for i in 5..10 {
            assert_eq!(buffer.get(i, 0).unwrap().symbol, '░');
        }
    }

    #[test]
    fn test_draw_progress_bar_clamp() {
        let mut buffer = test_buffer();
        let mut ctx = RenderContext::new(&mut buffer, test_area());

        // Test progress > 1.0 is clamped
        let config = ProgressBarConfig {
            x: 0,
            y: 0,
            width: 10,
            progress: 1.5,
            filled_char: '█',
            empty_char: '░',
            fg: Color::rgb(255, 255, 255),
        };

        ctx.draw_progress_bar(&config);

        // Should be fully filled (clamped to 1.0)
        for i in 0..10 {
            assert_eq!(buffer.get(i, 0).unwrap().symbol, '█');
        }
    }

    // =========================================================================
    // Segment drawing tests
    // =========================================================================

    #[test]
    fn test_draw_segments() {
        let mut buffer = test_buffer();
        let mut ctx = RenderContext::new(&mut buffer, test_area());

        let c1 = Color::rgb(255, 0, 0);
        let c2 = Color::rgb(0, 255, 0);
        let segments: &[(&str, Color)] = &[("AB", c1), ("CD", c2)];

        let end_x = ctx.draw_segments(0, 0, segments);

        assert_eq!(end_x, 4);
        assert_eq!(buffer.get(0, 0).unwrap().symbol, 'A');
        assert_eq!(buffer.get(2, 0).unwrap().symbol, 'C');
    }

    #[test]
    fn test_draw_segments_sep() {
        let mut buffer = test_buffer();
        let mut ctx = RenderContext::new(&mut buffer, test_area());

        let c1 = Color::rgb(255, 0, 0);
        let c2 = Color::rgb(0, 255, 0);
        let sep_color = Color::rgb(128, 128, 128);
        let segments: &[(&str, Color)] = &[("A", c1), ("B", c2)];

        let end_x = ctx.draw_segments_sep(0, 0, segments, "|", sep_color);

        assert_eq!(end_x, 3); // "A" + "|" + "B"
        assert_eq!(buffer.get(1, 0).unwrap().symbol, '|');
    }

    #[test]
    fn test_draw_text_selectable_selected() {
        use crate::render::Modifier;

        let mut buffer = test_buffer();
        let normal = Color::rgb(200, 200, 200);
        let selected = Color::rgb(255, 255, 0);

        {
            let mut ctx = RenderContext::new(&mut buffer, test_area());
            ctx.draw_text_selectable(0, 0, "Item", true, normal, selected);
        }

        assert!(buffer.get(0, 0).unwrap().modifier.contains(Modifier::BOLD));
    }

    #[test]
    fn test_draw_text_selectable_not_selected() {
        use crate::render::Modifier;

        let mut buffer = test_buffer();
        let normal = Color::rgb(200, 200, 200);
        let selected = Color::rgb(255, 255, 0);

        {
            let mut ctx = RenderContext::new(&mut buffer, test_area());
            ctx.draw_text_selectable(0, 0, "Item", false, normal, selected);
        }

        assert!(!buffer.get(0, 0).unwrap().modifier.contains(Modifier::BOLD));
    }

    // =========================================================================
    // Metric color tests
    // =========================================================================

    #[test]
    fn test_metric_color() {
        let low = Color::rgb(0, 255, 0);
        let mid = Color::rgb(255, 255, 0);
        let high = Color::rgb(255, 0, 0);

        assert_eq!(RenderContext::metric_color(10, 50, 80, low, mid, high), low);
        assert_eq!(RenderContext::metric_color(60, 50, 80, low, mid, high), mid);
        assert_eq!(
            RenderContext::metric_color(90, 50, 80, low, mid, high),
            high
        );
    }

    // =========================================================================
    // CSS integration tests
    // =========================================================================

    #[test]
    fn test_css_color_no_style() {
        let mut buffer = test_buffer();
        let default = Color::rgb(255, 255, 255);
        let ctx = RenderContext::new(&mut buffer, test_area());

        assert_eq!(ctx.css_color(default), default);
    }

    #[test]
    fn test_css_opacity() {
        let mut buffer = test_buffer();
        let ctx = RenderContext::new(&mut buffer, test_area());

        assert_eq!(ctx.css_opacity(), 1.0);
    }

    #[test]
    fn test_css_visible() {
        let mut buffer = test_buffer();
        let ctx = RenderContext::new(&mut buffer, test_area());

        assert!(ctx.css_visible());
    }

    // =========================================================================
    // Focus ring tests
    // =========================================================================

    #[test]
    fn test_draw_focus_ring_solid() {
        let mut buffer = test_buffer();
        let mut ctx = RenderContext::new(&mut buffer, test_area());
        let color = Color::rgb(255, 255, 0);

        ctx.draw_focus_ring(0, 0, 5, 3, color, FocusStyle::Solid);

        assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
        assert_eq!(buffer.get(4, 0).unwrap().symbol, '┐');
    }

    #[test]
    fn test_draw_focus_ring_rounded() {
        let mut buffer = test_buffer();
        let mut ctx = RenderContext::new(&mut buffer, test_area());
        let color = Color::rgb(255, 255, 0);

        ctx.draw_focus_ring(0, 0, 5, 3, color, FocusStyle::Rounded);

        assert_eq!(buffer.get(0, 0).unwrap().symbol, '╭');
    }

    #[test]
    fn test_draw_focus_ring_double() {
        let mut buffer = test_buffer();
        let mut ctx = RenderContext::new(&mut buffer, test_area());
        let color = Color::rgb(255, 255, 0);

        ctx.draw_focus_ring(0, 0, 5, 3, color, FocusStyle::Double);

        assert_eq!(buffer.get(0, 0).unwrap().symbol, '╔');
    }

    #[test]
    fn test_draw_focus_ring_too_small() {
        let mut buffer = test_buffer();
        let mut ctx = RenderContext::new(&mut buffer, test_area());
        let color = Color::rgb(255, 255, 0);

        // Should not panic
        ctx.draw_focus_ring(0, 0, 1, 1, color, FocusStyle::Solid);
    }

    #[test]
    fn test_draw_focus_underline() {
        let mut buffer = test_buffer();
        let mut ctx = RenderContext::new(&mut buffer, test_area());
        let color = Color::rgb(255, 255, 0);

        ctx.draw_focus_underline(0, 0, 5, color);

        for i in 0..5 {
            assert_eq!(buffer.get(i, 0).unwrap().symbol, '▔');
        }
    }

    #[test]
    fn test_draw_focus_marker() {
        let mut buffer = test_buffer();
        let mut ctx = RenderContext::new(&mut buffer, test_area());
        let color = Color::rgb(255, 255, 0);

        ctx.draw_focus_marker(0, 0, color);

        assert_eq!(buffer.get(0, 0).unwrap().symbol, '▶');
    }

    #[test]
    fn test_invert_colors() {
        let mut buffer = test_buffer();
        let mut ctx = RenderContext::new(&mut buffer, test_area());
        let fg = Color::rgb(255, 255, 255);
        let bg = Color::rgb(0, 0, 0);

        ctx.draw_char_bg(0, 0, 'X', fg, bg);
        ctx.invert_colors(0, 0, 1, 1);

        let cell = buffer.get(0, 0).unwrap();
        assert_eq!(cell.fg, Some(bg));
        assert_eq!(cell.bg, Some(fg));
    }
}
