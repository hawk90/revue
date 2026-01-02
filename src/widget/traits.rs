//! Widget traits

use crate::render::{Buffer, Cell};
use crate::layout::Rect;
use crate::dom::{WidgetMeta, NodeState};
use crate::style::{Style, Color};
use std::time::{Duration, Instant};

// =============================================================================
// Timeout Utility
// =============================================================================

/// A simple timeout tracker for auto-clearing messages or timed events.
///
/// # Example
/// ```rust,ignore
/// use revue::widget::traits::Timeout;
/// use std::time::Duration;
///
/// let mut msg_timeout = Timeout::new(Duration::from_secs(3));
///
/// // Set a message
/// msg_timeout.set("Operation complete".to_string());
///
/// // In your tick handler
/// if msg_timeout.is_expired() {
///     msg_timeout.clear();
/// }
///
/// // Get current value
/// if let Some(msg) = msg_timeout.get() {
///     println!("{}", msg);
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Timeout<T> {
    value: Option<T>,
    set_time: Option<Instant>,
    duration: Duration,
}

impl<T> Timeout<T> {
    /// Create a new timeout tracker with the specified duration.
    pub fn new(duration: Duration) -> Self {
        Self {
            value: None,
            set_time: None,
            duration,
        }
    }

    /// Create a timeout with seconds.
    pub fn secs(secs: u64) -> Self {
        Self::new(Duration::from_secs(secs))
    }

    /// Create a timeout with milliseconds.
    pub fn millis(millis: u64) -> Self {
        Self::new(Duration::from_millis(millis))
    }

    /// Set a value and start the timeout timer.
    pub fn set(&mut self, value: T) {
        self.value = Some(value);
        self.set_time = Some(Instant::now());
    }

    /// Clear the value and timer.
    pub fn clear(&mut self) {
        self.value = None;
        self.set_time = None;
    }

    /// Check if the timeout has expired.
    pub fn is_expired(&self) -> bool {
        self.set_time.map(|t| t.elapsed() > self.duration).unwrap_or(false)
    }

    /// Get a reference to the current value if set.
    pub fn get(&self) -> Option<&T> {
        self.value.as_ref()
    }

    /// Check if value is set (regardless of expiration).
    pub fn is_set(&self) -> bool {
        self.value.is_some()
    }

    /// Poll and auto-clear if expired. Returns true if cleared.
    pub fn poll(&mut self) -> bool {
        if self.is_expired() {
            self.clear();
            true
        } else {
            false
        }
    }

    /// Get remaining time before expiration, or None if not set or expired.
    pub fn remaining(&self) -> Option<Duration> {
        self.set_time.and_then(|t| {
            let elapsed = t.elapsed();
            if elapsed < self.duration {
                Some(self.duration - elapsed)
            } else {
                None
            }
        })
    }
}

impl<T> Default for Timeout<T> {
    fn default() -> Self {
        Self::secs(3)
    }
}

// =============================================================================
// Common Icons (Nerd Font & Unicode)
// =============================================================================

/// Common icons (Unicode & Nerd Font)
pub mod icons {
    // Status indicators
    /// Filled circle - online/active
    pub const ONLINE: char = '●';
    /// Empty circle - offline/inactive
    pub const OFFLINE: char = '○';
    /// Dotted circle - unknown/loading
    pub const UNKNOWN: char = '◌';

    // Mount/folder indicators
    /// Filled diamond - mounted
    pub const MOUNTED: char = '◆';
    /// Empty diamond - unmounted
    pub const UNMOUNTED: char = '◇';
    /// Folder (Nerd Font: nf-fa-folder)
    pub const FOLDER: char = '\u{f07b}';
    /// Folder open (Nerd Font: nf-fa-folder_open)
    pub const FOLDER_OPEN: char = '\u{f07c}';

    // Connection types (Nerd Font)
    /// Terminal (nf-oct-terminal)
    pub const TERMINAL: char = '\u{f489}';
    /// Server (nf-fa-server)
    pub const SERVER: char = '\u{f233}';
    /// Network (nf-md-network)
    pub const NETWORK: char = '\u{f0318}';
    /// Cloud (nf-fa-cloud)
    pub const CLOUD: char = '\u{f0c2}';
    /// Database (nf-fa-database)
    pub const DATABASE: char = '\u{f1c0}';

    // Common actions
    /// Check mark
    pub const CHECK: char = '✓';
    /// Cross mark
    pub const CROSS: char = '✗';
    /// Warning triangle
    pub const WARNING: char = '⚠';
    /// Info circle
    pub const INFO: char = 'ℹ';
    /// Arrow right
    pub const ARROW_RIGHT: char = '→';
    /// Arrow left
    pub const ARROW_LEFT: char = '←';
    /// Arrow up
    pub const ARROW_UP: char = '↑';
    /// Arrow down
    pub const ARROW_DOWN: char = '↓';

    // Progress/metrics
    /// Filled block for progress bars
    pub const BLOCK_FULL: char = '█';
    /// Light shade block
    pub const BLOCK_LIGHT: char = '░';
    /// Medium shade block
    pub const BLOCK_MEDIUM: char = '▒';
    /// Dark shade block
    pub const BLOCK_DARK: char = '▓';

    // Separators
    /// Vertical bar separator
    pub const SEP_VERT: char = '│';
    /// Bullet point
    pub const BULLET: char = '•';
    /// Double angle right
    pub const CHEVRON_RIGHT: char = '»';
    /// Double angle left
    pub const CHEVRON_LEFT: char = '«';
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
    pub fn full(buffer: &'a mut Buffer, area: Rect, style: &'a Style, state: &'a NodeState) -> Self {
        Self {
            buffer,
            area,
            style: Some(style),
            state: Some(state),
            transitions: None,
        }
    }

    /// Set transition values for this render context
    pub fn with_transitions(mut self, transitions: &'a std::collections::HashMap<String, f32>) -> Self {
        self.transitions = Some(transitions);
        self
    }

    /// Get current transition value for a property
    ///
    /// Returns the interpolated value during animation, or None if no transition is active.
    /// Use this to animate properties like opacity, position offsets, etc.
    ///
    /// # Example
    /// ```ignore
    /// let opacity = ctx.transition("opacity").unwrap_or(1.0);
    /// ```
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
        for (i, ch) in text.chars().enumerate() {
            let cell = Cell::new(ch).fg(fg);
            self.buffer.set(x + i as u16, y, cell);
        }
    }

    /// Draw text with background color
    pub fn draw_text_bg(&mut self, x: u16, y: u16, text: &str, fg: Color, bg: Color) {
        for (i, ch) in text.chars().enumerate() {
            let cell = Cell::new(ch).fg(fg).bg(bg);
            self.buffer.set(x + i as u16, y, cell);
        }
    }

    /// Draw bold text
    pub fn draw_text_bold(&mut self, x: u16, y: u16, text: &str, fg: Color) {
        for (i, ch) in text.chars().enumerate() {
            let cell = Cell::new(ch).fg(fg).bold();
            self.buffer.set(x + i as u16, y, cell);
        }
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
        // Corners
        self.draw_char(x, y, '╭', fg);
        self.draw_char(x + w - 1, y, '╮', fg);
        self.draw_char(x, y + h - 1, '╰', fg);
        self.draw_char(x + w - 1, y + h - 1, '╯', fg);
        // Horizontal lines
        self.draw_hline(x + 1, y, w - 2, '─', fg);
        self.draw_hline(x + 1, y + h - 1, w - 2, '─', fg);
        // Vertical lines
        self.draw_vline(x, y + 1, h - 2, '│', fg);
        self.draw_vline(x + w - 1, y + 1, h - 2, '│', fg);
    }

    /// Draw a box without top border (for custom multi-color headers)
    ///
    /// Use this when you want to draw your own header line with multiple colors.
    /// The caller should draw the complete top line including corners.
    pub fn draw_box_no_top(&mut self, x: u16, y: u16, w: u16, h: u16, fg: Color) {
        if w < 2 || h < 2 {
            return;
        }
        // Bottom corners
        self.draw_char(x, y + h - 1, '╰', fg);
        self.draw_char(x + w - 1, y + h - 1, '╯', fg);
        // Bottom border
        self.draw_hline(x + 1, y + h - 1, w - 2, '─', fg);
        // Vertical lines (from y+1 to bottom-1)
        self.draw_vline(x, y + 1, h - 2, '│', fg);
        self.draw_vline(x + w - 1, y + 1, h - 2, '│', fg);
    }

    /// Draw a complete header line with corners for use with draw_box_no_top
    ///
    /// Draws: `╭─ [content parts with colors] ─────╮`
    ///
    /// # Arguments
    /// * `x`, `y` - Position of header line
    /// * `width` - Total width including corners
    /// * `parts` - Content parts as (text, color) pairs
    /// * `border_color` - Color for corners and fill dashes
    ///
    /// # Example
    /// ```rust,ignore
    /// ctx.draw_box_no_top(x, y, width, height, BLUE);
    /// ctx.draw_header_line(x, y, width, &[
    ///     (" 5 jobs ", BLUE),
    ///     ("│", FG_DIM),
    ///     (" ✓ 3 ", GREEN),
    /// ], BLUE);
    /// ```
    pub fn draw_header_line(&mut self, x: u16, y: u16, width: u16, parts: &[(&str, Color)], border_color: Color) {
        if width < 4 {
            return;
        }

        // Draw left corner
        self.draw_text(x, y, "╭─", border_color);
        let mut pos = x + 2;

        // Draw content parts
        for (text, color) in parts {
            self.draw_text(pos, y, text, *color);
            pos += text.chars().count() as u16;
        }

        // Fill remaining with ─ and close with ╮
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
        // Corners
        self.draw_char(x, y, '┌', fg);
        self.draw_char(x + w - 1, y, '┐', fg);
        self.draw_char(x, y + h - 1, '└', fg);
        self.draw_char(x + w - 1, y + h - 1, '┘', fg);
        // Horizontal lines
        self.draw_hline(x + 1, y, w - 2, '─', fg);
        self.draw_hline(x + 1, y + h - 1, w - 2, '─', fg);
        // Vertical lines
        self.draw_vline(x, y + 1, h - 2, '│', fg);
        self.draw_vline(x + w - 1, y + 1, h - 2, '│', fg);
    }

    /// Draw a box with double border
    pub fn draw_box_double(&mut self, x: u16, y: u16, w: u16, h: u16, fg: Color) {
        if w < 2 || h < 2 {
            return;
        }
        // Corners
        self.draw_char(x, y, '╔', fg);
        self.draw_char(x + w - 1, y, '╗', fg);
        self.draw_char(x, y + h - 1, '╚', fg);
        self.draw_char(x + w - 1, y + h - 1, '╝', fg);
        // Horizontal lines
        self.draw_hline(x + 1, y, w - 2, '═', fg);
        self.draw_hline(x + 1, y + h - 1, w - 2, '═', fg);
        // Vertical lines
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
        for (i, ch) in text.chars().enumerate() {
            if (i as u16) >= max_width {
                break;
            }
            let cell = Cell::new(ch).fg(fg);
            self.buffer.set(x + i as u16, y, cell);
        }
    }

    /// Draw bold text clipped to max_width
    pub fn draw_text_clipped_bold(&mut self, x: u16, y: u16, text: &str, fg: Color, max_width: u16) {
        for (i, ch) in text.chars().enumerate() {
            if (i as u16) >= max_width {
                break;
            }
            let cell = Cell::new(ch).fg(fg).bold();
            self.buffer.set(x + i as u16, y, cell);
        }
    }

    /// Draw dimmed text
    pub fn draw_text_dim(&mut self, x: u16, y: u16, text: &str, fg: Color) {
        for (i, ch) in text.chars().enumerate() {
            let cell = Cell::new(ch).fg(fg).dim();
            self.buffer.set(x + i as u16, y, cell);
        }
    }

    /// Draw italic text
    pub fn draw_text_italic(&mut self, x: u16, y: u16, text: &str, fg: Color) {
        for (i, ch) in text.chars().enumerate() {
            let cell = Cell::new(ch).fg(fg).italic();
            self.buffer.set(x + i as u16, y, cell);
        }
    }

    /// Draw underlined text
    pub fn draw_text_underline(&mut self, x: u16, y: u16, text: &str, fg: Color) {
        for (i, ch) in text.chars().enumerate() {
            let cell = Cell::new(ch).fg(fg).underline();
            self.buffer.set(x + i as u16, y, cell);
        }
    }

    /// Draw text centered within a given width
    pub fn draw_text_centered(&mut self, x: u16, y: u16, width: u16, text: &str, fg: Color) {
        let text_len = text.chars().count() as u16;
        let start_x = if text_len >= width {
            x
        } else {
            x + (width - text_len) / 2
        };
        self.draw_text_clipped(start_x, y, text, fg, width);
    }

    /// Draw text right-aligned within a given width
    pub fn draw_text_right(&mut self, x: u16, y: u16, width: u16, text: &str, fg: Color) {
        let text_len = text.chars().count() as u16;
        let start_x = if text_len >= width {
            x
        } else {
            x + width - text_len
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
        // Corners
        self.draw_char(x, y, '╭', fg);
        self.draw_char(x + w - 1, y, '╮', fg);
        self.draw_char(x, y + h - 1, '╰', fg);
        self.draw_char(x + w - 1, y + h - 1, '╯', fg);

        // Top border with title
        let title_start = 2u16;
        let title_len = title.chars().count() as u16;
        let title_end = title_start + title_len;

        for i in 1..(w - 1) {
            if i >= title_start && i < title_end {
                let ch = title.chars().nth((i - title_start) as usize).unwrap_or('─');
                self.draw_char(x + i, y, ch, fg);
            } else {
                self.draw_char(x + i, y, '─', fg);
            }
        }

        // Bottom border
        self.draw_hline(x + 1, y + h - 1, w - 2, '─', fg);
        // Vertical lines
        self.draw_vline(x, y + 1, h - 2, '│', fg);
        self.draw_vline(x + w - 1, y + 1, h - 2, '│', fg);
    }

    /// Draw a single-line box with a title
    pub fn draw_box_titled_single(&mut self, x: u16, y: u16, w: u16, h: u16, title: &str, fg: Color) {
        if w < 2 || h < 2 {
            return;
        }
        // Corners
        self.draw_char(x, y, '┌', fg);
        self.draw_char(x + w - 1, y, '┐', fg);
        self.draw_char(x, y + h - 1, '└', fg);
        self.draw_char(x + w - 1, y + h - 1, '┘', fg);

        // Top border with title
        let title_start = 2u16;
        let title_len = title.chars().count() as u16;
        let title_end = title_start + title_len;

        for i in 1..(w - 1) {
            if i >= title_start && i < title_end {
                let ch = title.chars().nth((i - title_start) as usize).unwrap_or('─');
                self.draw_char(x + i, y, ch, fg);
            } else {
                self.draw_char(x + i, y, '─', fg);
            }
        }

        // Bottom border
        self.draw_hline(x + 1, y + h - 1, w - 2, '─', fg);
        // Vertical lines
        self.draw_vline(x, y + 1, h - 2, '│', fg);
        self.draw_vline(x + w - 1, y + 1, h - 2, '│', fg);
    }

    /// Draw a double-line box with a title
    pub fn draw_box_titled_double(&mut self, x: u16, y: u16, w: u16, h: u16, title: &str, fg: Color) {
        if w < 2 || h < 2 {
            return;
        }
        // Corners
        self.draw_char(x, y, '╔', fg);
        self.draw_char(x + w - 1, y, '╗', fg);
        self.draw_char(x, y + h - 1, '╚', fg);
        self.draw_char(x + w - 1, y + h - 1, '╝', fg);

        // Top border with title
        let title_start = 2u16;
        let title_len = title.chars().count() as u16;
        let title_end = title_start + title_len;

        for i in 1..(w - 1) {
            if i >= title_start && i < title_end {
                let ch = title.chars().nth((i - title_start) as usize).unwrap_or('═');
                self.draw_char(x + i, y, ch, fg);
            } else {
                self.draw_char(x + i, y, '═', fg);
            }
        }

        // Bottom border
        self.draw_hline(x + 1, y + h - 1, w - 2, '═', fg);
        // Vertical lines
        self.draw_vline(x, y + 1, h - 2, '║', fg);
        self.draw_vline(x + w - 1, y + 1, h - 2, '║', fg);
    }

    // =========================================================================
    // Progress bar utilities
    // =========================================================================

    /// Draw a horizontal progress bar
    ///
    /// # Arguments
    /// * `x`, `y` - Position
    /// * `width` - Total width of the bar
    /// * `progress` - Progress value from 0.0 to 1.0
    /// * `filled_char` - Character for filled portion (e.g., '█')
    /// * `empty_char` - Character for empty portion (e.g., '░')
    /// * `fg` - Foreground color
    pub fn draw_progress_bar(
        &mut self,
        x: u16,
        y: u16,
        width: u16,
        progress: f32,
        filled_char: char,
        empty_char: char,
        fg: Color,
    ) {
        let progress = progress.clamp(0.0, 1.0);
        let filled = (width as f32 * progress).round() as u16;

        for i in 0..width {
            let ch = if i < filled { filled_char } else { empty_char };
            self.draw_char(x + i, y, ch, fg);
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

        // Draw label first
        self.draw_text(x, y, &label, fg);

        // Draw bar after label
        let bar_x = x + 4;
        self.draw_char(bar_x, y, '[', fg);
        self.draw_progress_bar(bar_x + 1, y, bar_width, progress, '█', '░', fg);
        self.draw_char(bar_x + 1 + bar_width, y, ']', fg);
    }

    // =========================================================================
    // Segment-based drawing utilities
    // =========================================================================

    /// Draw multiple text segments with different colors on one line
    ///
    /// Returns the ending x position for chaining.
    ///
    /// # Example
    /// ```ignore
    /// ctx.draw_segments(x, y, &[
    ///     ("●", Color::GREEN),
    ///     (" 3/5 ", Color::WHITE),
    ///     ("mounted", Color::CYAN),
    /// ]);
    /// ```
    pub fn draw_segments(&mut self, x: u16, y: u16, segments: &[(&str, Color)]) -> u16 {
        let mut cx = x;
        for (text, color) in segments {
            self.draw_text(cx, y, text, *color);
            cx += text.chars().count() as u16;
        }
        cx
    }

    /// Draw segments with a separator between them
    pub fn draw_segments_sep(&mut self, x: u16, y: u16, segments: &[(&str, Color)], sep: &str, sep_color: Color) -> u16 {
        let mut cx = x;
        for (i, (text, color)) in segments.iter().enumerate() {
            if i > 0 {
                self.draw_text(cx, y, sep, sep_color);
                cx += sep.chars().count() as u16;
            }
            self.draw_text(cx, y, text, *color);
            cx += text.chars().count() as u16;
        }
        cx
    }

    /// Draw key hints (key in bold color, action in dim)
    ///
    /// # Example
    /// ```ignore
    /// ctx.draw_key_hints(x, y, &[("j/k", "Move"), ("m", "Mount")], Color::CYAN, Color::GRAY);
    /// ```
    pub fn draw_key_hints(&mut self, x: u16, y: u16, hints: &[(&str, &str)], key_color: Color, action_color: Color) -> u16 {
        let mut cx = x;
        for (key, action) in hints {
            self.draw_text_bold(cx, y, key, key_color);
            cx += key.chars().count() as u16 + 1;
            self.draw_text(cx, y, action, action_color);
            cx += action.chars().count() as u16 + 2;
        }
        cx
    }

    /// Draw text with selection styling (bold + highlight color when selected)
    pub fn draw_text_selectable(&mut self, x: u16, y: u16, text: &str, selected: bool, normal_color: Color, selected_color: Color) {
        if selected {
            self.draw_text_bold(x, y, text, selected_color);
        } else {
            self.draw_text(x, y, text, normal_color);
        }
    }

    /// Get color based on value thresholds (for metrics)
    ///
    /// Returns low_color if value < mid, mid_color if value < high, else high_color
    pub fn metric_color(value: u8, mid: u8, high: u8, low_color: Color, mid_color: Color, high_color: Color) -> Color {
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
    ///
    /// This is the primary method for widgets to get their foreground color
    /// when CSS styling is enabled. It checks the computed CSS style first,
    /// falling back to the provided default.
    ///
    /// # Example
    /// ```ignore
    /// let fg = ctx.css_color(Color::WHITE);
    /// ```
    pub fn css_color(&self, default: Color) -> Color {
        self.style
            .map(|s| {
                let c = s.visual.color;
                if c == Color::default() { default } else { c }
            })
            .unwrap_or(default)
    }

    /// Get background color from CSS style or use default
    pub fn css_background(&self, default: Color) -> Color {
        self.style
            .map(|s| {
                let c = s.visual.background;
                if c == Color::default() { default } else { c }
            })
            .unwrap_or(default)
    }

    /// Get border color from CSS style or use default
    pub fn css_border_color(&self, default: Color) -> Color {
        self.style
            .map(|s| {
                let c = s.visual.border_color;
                if c == Color::default() { default } else { c }
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
        self.style.map(|s| s.visual.border_style).unwrap_or_default()
    }

    /// Get gap from CSS style (for flex/grid layouts)
    pub fn css_gap(&self) -> u16 {
        self.style.map(|s| s.layout.gap).unwrap_or(0)
    }

    /// Resolve effective foreground color considering CSS, widget state, and disabled state
    ///
    /// Priority order:
    /// 1. Disabled state (returns DISABLED_FG)
    /// 2. Widget state override (if provided)
    /// 3. CSS computed style
    /// 4. Default fallback
    ///
    /// # Example
    /// ```ignore
    /// let fg = ctx.resolve_fg(widget.state.effective_fg_opt(), Color::WHITE);
    /// ```
    pub fn resolve_fg(&self, widget_override: Option<Color>, default: Color) -> Color {
        // Check disabled from node state
        if self.is_disabled() {
            return DISABLED_FG;
        }

        // Widget-level override takes priority
        if let Some(color) = widget_override {
            return color;
        }

        // CSS style
        self.css_color(default)
    }

    /// Resolve effective background color considering CSS, widget state, and disabled state
    pub fn resolve_bg(&self, widget_override: Option<Color>, default: Color) -> Color {
        if self.is_disabled() {
            return DISABLED_BG;
        }

        if let Some(color) = widget_override {
            return color;
        }

        self.css_background(default)
    }

    // =========================================================================
    // Accessibility - Focus Indicators
    // =========================================================================

    /// Draw a focus ring around an area
    ///
    /// This draws a visible focus indicator around the given bounds.
    /// Use this when the widget has focus to provide visual feedback.
    ///
    /// # Arguments
    /// * `x`, `y` - Top-left corner of the focused area
    /// * `w`, `h` - Width and height of the focused area
    /// * `color` - Color of the focus ring
    /// * `style` - Style of the focus ring
    ///
    /// # Example
    /// ```ignore
    /// if ctx.is_focused() {
    ///     ctx.draw_focus_ring(x, y, w, h, Color::CYAN, FocusStyle::Dotted);
    /// }
    /// ```
    pub fn draw_focus_ring(&mut self, x: u16, y: u16, w: u16, h: u16, color: Color, style: FocusStyle) {
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

        // Corners
        self.draw_char(x, y, tl, color);
        self.draw_char(x + w - 1, y, tr, color);
        self.draw_char(x, y + h - 1, bl, color);
        self.draw_char(x + w - 1, y + h - 1, br, color);

        // Horizontal lines
        self.draw_hline(x + 1, y, w - 2, h_char, color);
        self.draw_hline(x + 1, y + h - 1, w - 2, h_char, color);

        // Vertical lines
        self.draw_vline(x, y + 1, h - 2, v_char, color);
        self.draw_vline(x + w - 1, y + 1, h - 2, v_char, color);
    }

    /// Draw a focus ring with automatic style based on context
    ///
    /// Uses bold style for high-contrast mode, rounded for normal mode.
    pub fn draw_focus_ring_auto(&mut self, x: u16, y: u16, w: u16, h: u16, color: Color) {
        // Default to rounded style - high contrast check would be done elsewhere
        self.draw_focus_ring(x, y, w, h, color, FocusStyle::Rounded);
    }

    /// Draw a focus underline (for inline elements)
    ///
    /// Draws an underline under the focused element.
    pub fn draw_focus_underline(&mut self, x: u16, y: u16, w: u16, color: Color) {
        for i in 0..w {
            let cell = Cell::new('▔').fg(color);
            self.buffer.set(x + i, y, cell);
        }
    }

    /// Draw a focus indicator at a specific position
    ///
    /// Draws a small marker (like ▶) to indicate focus.
    pub fn draw_focus_marker(&mut self, x: u16, y: u16, color: Color) {
        self.draw_char(x, y, '▶', color);
    }

    /// Draw a focus indicator on the left side of an item
    ///
    /// Common pattern for list items - shows a marker when focused.
    pub fn draw_focus_marker_left(&mut self, y: u16, color: Color) {
        if self.area.x > 0 {
            self.draw_char(self.area.x - 1, y, '▶', color);
        } else {
            self.draw_char(self.area.x, y, '▶', color);
        }
    }

    /// Invert colors in a region (for high contrast focus indication)
    ///
    /// Swaps foreground and background colors in the specified area.
    pub fn invert_colors(&mut self, x: u16, y: u16, w: u16, h: u16) {
        for dy in 0..h {
            for dx in 0..w {
                if let Some(cell) = self.buffer.get_mut(x + dx, y + dy) {
                    // Swap fg and bg
                    let old_fg = cell.fg;
                    let old_bg = cell.bg;
                    cell.fg = old_bg;
                    cell.bg = old_fg;
                }
            }
        }
    }

    /// Add reverse video effect to indicate focus
    ///
    /// Applies reverse video (swapped fg/bg colors) to the area.
    pub fn draw_focus_reverse(&mut self, x: u16, y: u16, w: u16, h: u16) {
        self.invert_colors(x, y, w, h);
    }
}

/// Style of focus indicator
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FocusStyle {
    /// Single line border (┌─┐)
    #[default]
    Solid,
    /// Rounded corners (╭─╮)
    Rounded,
    /// Double line border (╔═╗)
    Double,
    /// Dotted line border (┌╌┐)
    Dotted,
    /// Bold line border (┏━┓)
    Bold,
    /// ASCII compatible (+--+)
    Ascii,
}

// =============================================================================
// EventResult - Result of event handling
// =============================================================================

/// Result of handling an event
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EventResult {
    /// Event was not handled, should propagate to parent
    #[default]
    Ignored,
    /// Event was consumed, stop propagation
    Consumed,
    /// Event was consumed and widget needs re-render
    ConsumedAndRender,
}

impl EventResult {
    /// Check if the event was handled (consumed)
    #[inline]
    pub fn is_consumed(&self) -> bool {
        !matches!(self, EventResult::Ignored)
    }

    /// Check if re-render is needed
    #[inline]
    pub fn needs_render(&self) -> bool {
        matches!(self, EventResult::ConsumedAndRender)
    }

    /// Combine with another result (takes the "most impactful")
    pub fn or(self, other: EventResult) -> EventResult {
        match (self, other) {
            (EventResult::ConsumedAndRender, _) | (_, EventResult::ConsumedAndRender) => {
                EventResult::ConsumedAndRender
            }
            (EventResult::Consumed, _) | (_, EventResult::Consumed) => EventResult::Consumed,
            _ => EventResult::Ignored,
        }
    }
}

impl From<bool> for EventResult {
    /// Convert from bool: true = ConsumedAndRender, false = Ignored
    fn from(handled: bool) -> Self {
        if handled {
            EventResult::ConsumedAndRender
        } else {
            EventResult::Ignored
        }
    }
}

/// The core trait for all renderable components
pub trait View {
    /// Render the view
    fn render(&self, ctx: &mut RenderContext);

    /// Get widget type name (for CSS type selectors)
    fn widget_type(&self) -> &'static str {
        std::any::type_name::<Self>().rsplit("::").next().unwrap_or("Unknown")
    }

    /// Get element ID (for CSS #id selectors)
    fn id(&self) -> Option<&str> {
        None
    }

    /// Get CSS classes (for CSS .class selectors)
    fn classes(&self) -> &[String] {
        &[]
    }

    /// Get child views (for container widgets)
    ///
    /// Container widgets (Stack, Grid, etc.) should override this to expose
    /// their children, enabling the DOM builder to traverse the full widget tree.
    fn children(&self) -> &[Box<dyn View>] {
        &[]
    }

    /// Get widget metadata for DOM
    fn meta(&self) -> WidgetMeta {
        let mut meta = WidgetMeta::new(self.widget_type());
        if let Some(id) = self.id() {
            meta.id = Some(id.to_string());
        }
        for class in self.classes() {
            meta.classes.insert(class.clone());
        }
        meta
    }
}

/// Implement View for `Box<dyn View>` to allow boxed views to be used as children
impl View for Box<dyn View> {
    fn render(&self, ctx: &mut RenderContext) {
        (**self).render(ctx);
    }

    fn widget_type(&self) -> &'static str {
        (**self).widget_type()
    }

    fn id(&self) -> Option<&str> {
        (**self).id()
    }

    fn classes(&self) -> &[String] {
        (**self).classes()
    }

    fn children(&self) -> &[Box<dyn View>] {
        (**self).children()
    }

    fn meta(&self) -> WidgetMeta {
        (**self).meta()
    }
}

/// Trait for interactive widgets that handle events
///
/// This trait extends View with keyboard and mouse handling capabilities.
/// Widgets that need to respond to user input should implement this trait.
///
/// # Example
///
/// ```rust,ignore
/// use revue::widget::{View, Interactive, EventResult, RenderContext};
/// use revue::event::{KeyEvent, MouseEvent, Key};
/// use revue::layout::Rect;
///
/// struct Counter {
///     value: i32,
/// }
///
/// impl View for Counter {
///     fn render(&self, ctx: &mut RenderContext) {
///         // render implementation
///     }
/// }
///
/// impl Interactive for Counter {
///     fn handle_key(&mut self, event: &KeyEvent) -> EventResult {
///         match event.key {
///             Key::Up => { self.value += 1; EventResult::ConsumedAndRender }
///             Key::Down => { self.value -= 1; EventResult::ConsumedAndRender }
///             _ => EventResult::Ignored
///         }
///     }
/// }
/// ```
pub trait Interactive: View {
    /// Handle keyboard event
    ///
    /// Returns `EventResult::Consumed` or `EventResult::ConsumedAndRender` if
    /// the event was handled, `EventResult::Ignored` to let it propagate.
    ///
    /// # Arguments
    /// * `event` - The keyboard event with key and modifiers
    fn handle_key(&mut self, event: &crate::event::KeyEvent) -> EventResult {
        // Default: delegate to simple key handler if exists
        let _ = event;
        EventResult::Ignored
    }

    /// Handle mouse event
    ///
    /// Returns `EventResult` indicating if event was handled.
    ///
    /// # Arguments
    /// * `event` - The mouse event with position and kind
    /// * `area` - The widget's rendered area for hit testing
    fn handle_mouse(&mut self, event: &crate::event::MouseEvent, area: crate::layout::Rect) -> EventResult {
        let _ = (event, area);
        EventResult::Ignored
    }

    /// Check if the widget can receive focus
    fn focusable(&self) -> bool {
        true
    }

    /// Called when the widget receives focus
    fn on_focus(&mut self) {}

    /// Called when the widget loses focus
    fn on_blur(&mut self) {}
}

/// Trait for widgets that support drag-and-drop
///
/// Implement this trait to make a widget draggable or a drop target.
///
/// # Example
///
/// ```rust,ignore
/// use revue::widget::{View, Draggable, RenderContext};
/// use revue::event::drag::{DragData, DropResult};
/// use revue::layout::Rect;
///
/// struct DraggableItem {
///     label: String,
///     index: usize,
/// }
///
/// impl View for DraggableItem {
///     fn render(&self, ctx: &mut RenderContext) {
///         // render implementation
///     }
/// }
///
/// impl Draggable for DraggableItem {
///     fn can_drag(&self) -> bool {
///         true
///     }
///
///     fn drag_data(&self) -> Option<DragData> {
///         Some(DragData::list_item(self.index, &self.label))
///     }
///
///     fn drag_preview(&self) -> Option<String> {
///         Some(format!("📦 {}", self.label))
///     }
/// }
/// ```
pub trait Draggable: View {
    /// Check if this widget can be dragged
    fn can_drag(&self) -> bool {
        false
    }

    /// Get the drag data when a drag starts
    ///
    /// Return None to cancel the drag.
    fn drag_data(&self) -> Option<crate::event::drag::DragData> {
        None
    }

    /// Get a text preview for the drag operation
    ///
    /// This is shown near the cursor during drag.
    fn drag_preview(&self) -> Option<String> {
        None
    }

    /// Called when drag starts
    fn on_drag_start(&mut self) {}

    /// Called when drag ends (regardless of outcome)
    fn on_drag_end(&mut self, _result: crate::event::drag::DropResult) {}

    /// Check if this widget accepts drops
    fn can_drop(&self) -> bool {
        false
    }

    /// Get the types this widget accepts for drops
    fn accepted_types(&self) -> &[&'static str] {
        &[]
    }

    /// Check if this widget can accept specific drag data
    fn can_accept(&self, data: &crate::event::drag::DragData) -> bool {
        let types = self.accepted_types();
        types.is_empty() || types.contains(&data.type_id)
    }

    /// Called when a drag enters this widget's bounds
    fn on_drag_enter(&mut self, _data: &crate::event::drag::DragData) {}

    /// Called when a drag leaves this widget's bounds
    fn on_drag_leave(&mut self) {}

    /// Called when a drop occurs on this widget
    ///
    /// Return true if the drop was accepted, false to reject.
    fn on_drop(&mut self, _data: crate::event::drag::DragData) -> bool {
        false
    }

    /// Get the drop zone bounds for this widget
    ///
    /// Override this if the drop zone differs from the render area.
    fn drop_bounds(&self, area: Rect) -> Rect {
        area
    }
}

/// Extended View trait with styling support
pub trait StyledView: View {
    /// Set element ID
    fn set_id(&mut self, id: impl Into<String>);

    /// Add a CSS class
    fn add_class(&mut self, class: impl Into<String>);

    /// Remove a CSS class
    fn remove_class(&mut self, class: &str);

    /// Toggle a CSS class
    fn toggle_class(&mut self, class: &str);

    /// Check if has class
    fn has_class(&self, class: &str) -> bool;
}

/// Common widget properties for styling
#[derive(Debug, Clone, Default)]
pub struct WidgetProps {
    /// Element ID
    pub id: Option<String>,
    /// CSS classes
    pub classes: Vec<String>,
    /// Inline style override
    pub inline_style: Option<Style>,
}

// =============================================================================
// WidgetState - Common interactive state for widgets
// =============================================================================

/// Common interactive state shared by most widgets.
///
/// This struct provides a unified way to handle focus, disabled, pressed,
/// hovered states and color customization. Widgets can embed this struct
/// to reduce code duplication.
///
/// # Example
///
/// ```rust,ignore
/// use revue::widget::traits::WidgetState;
/// use revue::style::Color;
///
/// struct MyWidget {
///     label: String,
///     state: WidgetState,
/// }
///
/// impl MyWidget {
///     fn new(label: impl Into<String>) -> Self {
///         Self {
///             label: label.into(),
///             state: WidgetState::new(),
///         }
///     }
///
///     // Delegate builder methods
///     pub fn focused(mut self, focused: bool) -> Self {
///         self.state = self.state.focused(focused);
///         self
///     }
///
///     pub fn disabled(mut self, disabled: bool) -> Self {
///         self.state = self.state.disabled(disabled);
///         self
///     }
///
///     pub fn fg(mut self, color: Color) -> Self {
///         self.state = self.state.fg(color);
///         self
///     }
/// }
/// ```
#[derive(Debug, Clone, Default)]
pub struct WidgetState {
    /// Whether the widget is focused
    pub focused: bool,
    /// Whether the widget is disabled (non-interactive)
    pub disabled: bool,
    /// Whether the widget is currently pressed (mouse down)
    pub pressed: bool,
    /// Whether the mouse is hovering over the widget
    pub hovered: bool,
    /// Custom foreground color
    pub fg: Option<Color>,
    /// Custom background color
    pub bg: Option<Color>,
}

/// Default disabled foreground color
pub const DISABLED_FG: Color = Color { r: 100, g: 100, b: 100, a: 255 };
/// Default disabled background color
pub const DISABLED_BG: Color = Color { r: 50, g: 50, b: 50, a: 255 };

impl WidgetState {
    /// Create a new default widget state
    pub fn new() -> Self {
        Self::default()
    }

    // =========================================================================
    // Builder methods
    // =========================================================================

    /// Set focused state (builder)
    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    /// Set disabled state (builder)
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set pressed state (builder)
    pub fn pressed(mut self, pressed: bool) -> Self {
        self.pressed = pressed;
        self
    }

    /// Set hovered state (builder)
    pub fn hovered(mut self, hovered: bool) -> Self {
        self.hovered = hovered;
        self
    }

    /// Set foreground color (builder)
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Set background color (builder)
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    // =========================================================================
    // Getters
    // =========================================================================

    /// Check if widget is focused
    #[inline]
    pub fn is_focused(&self) -> bool {
        self.focused
    }

    /// Check if widget is disabled
    #[inline]
    pub fn is_disabled(&self) -> bool {
        self.disabled
    }

    /// Check if widget is pressed
    #[inline]
    pub fn is_pressed(&self) -> bool {
        self.pressed
    }

    /// Check if widget is hovered
    #[inline]
    pub fn is_hovered(&self) -> bool {
        self.hovered
    }

    /// Check if widget is interactive (focused or hovered, not disabled)
    #[inline]
    pub fn is_interactive(&self) -> bool {
        !self.disabled && (self.focused || self.hovered)
    }

    // =========================================================================
    // Mutable setters
    // =========================================================================

    /// Set focused state
    pub fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }

    /// Set disabled state
    pub fn set_disabled(&mut self, disabled: bool) {
        self.disabled = disabled;
    }

    /// Set pressed state
    pub fn set_pressed(&mut self, pressed: bool) {
        self.pressed = pressed;
    }

    /// Set hovered state
    pub fn set_hovered(&mut self, hovered: bool) {
        self.hovered = hovered;
    }

    /// Set foreground color
    pub fn set_fg(&mut self, color: Option<Color>) {
        self.fg = color;
    }

    /// Set background color
    pub fn set_bg(&mut self, color: Option<Color>) {
        self.bg = color;
    }

    // =========================================================================
    // Effective colors (handles disabled state)
    // =========================================================================

    /// Get effective foreground color, using disabled color if disabled
    pub fn effective_fg(&self, default: Color) -> Color {
        if self.disabled {
            DISABLED_FG
        } else {
            self.fg.unwrap_or(default)
        }
    }

    /// Get effective background color, using disabled color if disabled
    pub fn effective_bg(&self, default: Color) -> Color {
        if self.disabled {
            DISABLED_BG
        } else {
            self.bg.unwrap_or(default)
        }
    }

    /// Get effective foreground color, returning None if no color set
    pub fn effective_fg_opt(&self) -> Option<Color> {
        if self.disabled {
            Some(DISABLED_FG)
        } else {
            self.fg
        }
    }

    /// Get effective background color, returning None if no color set
    pub fn effective_bg_opt(&self) -> Option<Color> {
        if self.disabled {
            Some(DISABLED_BG)
        } else {
            self.bg
        }
    }

    // =========================================================================
    // CSS Color Resolution
    // =========================================================================

    /// Resolve foreground color with CSS cascade priority
    ///
    /// Priority order:
    /// 1. Disabled state (DISABLED_FG)
    /// 2. Widget inline override (via .fg())
    /// 3. CSS computed style from context
    /// 4. Default color
    ///
    /// # Example
    /// ```rust,ignore
    /// let fg = state.resolve_fg(ctx.style, Color::WHITE);
    /// ```
    pub fn resolve_fg(&self, css_style: Option<&Style>, default: Color) -> Color {
        if self.disabled {
            return DISABLED_FG;
        }

        // Widget inline override takes priority
        if let Some(fg) = self.fg {
            return fg;
        }

        // Try CSS color
        if let Some(style) = css_style {
            let c = style.visual.color;
            if c != Color::default() {
                return c;
            }
        }

        default
    }

    /// Resolve background color with CSS cascade priority
    ///
    /// Priority order:
    /// 1. Disabled state (DISABLED_BG)
    /// 2. Widget inline override (via .bg())
    /// 3. CSS computed style from context
    /// 4. Default color
    ///
    /// # Example
    /// ```rust,ignore
    /// let bg = state.resolve_bg(ctx.style, Color::rgb(60, 60, 60));
    /// ```
    pub fn resolve_bg(&self, css_style: Option<&Style>, default: Color) -> Color {
        if self.disabled {
            return DISABLED_BG;
        }

        // Widget inline override takes priority
        if let Some(bg) = self.bg {
            return bg;
        }

        // Try CSS background
        if let Some(style) = css_style {
            let c = style.visual.background;
            if c != Color::default() {
                return c;
            }
        }

        default
    }

    /// Resolve both fg and bg colors with CSS cascade priority
    ///
    /// Convenience method that resolves both colors at once.
    ///
    /// # Example
    /// ```rust,ignore
    /// let (fg, bg) = state.resolve_colors(ctx.style, Color::WHITE, Color::rgb(60, 60, 60));
    /// ```
    pub fn resolve_colors(
        &self,
        css_style: Option<&Style>,
        default_fg: Color,
        default_bg: Color,
    ) -> (Color, Color) {
        (
            self.resolve_fg(css_style, default_fg),
            self.resolve_bg(css_style, default_bg),
        )
    }

    /// Resolve colors with CSS cascade and apply interaction effects
    ///
    /// This is the most common use case: resolve colors from CSS cascade,
    /// then apply pressed/hover/focus effects to the background.
    ///
    /// Priority order for each color:
    /// 1. Disabled state (grayed out)
    /// 2. Widget inline override
    /// 3. CSS computed style
    /// 4. Default color
    ///
    /// After resolution, applies interaction effects to background:
    /// - Pressed: darken by 30
    /// - Hover/Focus: lighten by 40
    ///
    /// # Example
    /// ```rust,ignore
    /// let (fg, bg) = state.resolve_colors_interactive(
    ///     ctx.style,
    ///     Color::WHITE,
    ///     Color::rgb(37, 99, 235),
    /// );
    /// ```
    pub fn resolve_colors_interactive(
        &self,
        css_style: Option<&Style>,
        default_fg: Color,
        default_bg: Color,
    ) -> (Color, Color) {
        if self.disabled {
            return (DISABLED_FG, DISABLED_BG);
        }

        let fg = self.resolve_fg(css_style, default_fg);
        let bg = self.resolve_bg(css_style, default_bg);

        // Apply interaction effects
        let bg = bg.with_interaction(self.pressed, self.hovered, self.focused);

        (fg, bg)
    }

    // =========================================================================
    // State change helpers
    // =========================================================================

    /// Reset all transient states (pressed, hovered) but keep focused/disabled
    pub fn reset_transient(&mut self) {
        self.pressed = false;
        self.hovered = false;
    }

    /// Check if any visual state changed compared to another state
    pub fn visual_changed(&self, other: &WidgetState) -> bool {
        self.focused != other.focused
            || self.disabled != other.disabled
            || self.pressed != other.pressed
            || self.hovered != other.hovered
    }
}

// =============================================================================
// CSS Integration Macros
// =============================================================================

/// Implement StyledView trait for a widget that has a `props: WidgetProps` field.
///
/// This macro eliminates boilerplate by automatically implementing all
/// StyledView methods using the widget's `props` field.
///
/// # Example
/// ```rust,ignore
/// struct MyWidget {
///     label: String,
///     props: WidgetProps,
/// }
///
/// impl_styled_view!(MyWidget);
/// ```
#[macro_export]
macro_rules! impl_styled_view {
    ($widget:ty) => {
        impl $crate::widget::StyledView for $widget {
            fn set_id(&mut self, id: impl Into<String>) {
                self.props.id = Some(id.into());
            }

            fn add_class(&mut self, class: impl Into<String>) {
                let class_str = class.into();
                if !self.props.classes.contains(&class_str) {
                    self.props.classes.push(class_str);
                }
            }

            fn remove_class(&mut self, class: &str) {
                self.props.classes.retain(|c| c != class);
            }

            fn toggle_class(&mut self, class: &str) {
                if self.props.classes.iter().any(|c| c == class) {
                    self.props.classes.retain(|c| c != class);
                } else {
                    self.props.classes.push(class.to_string());
                }
            }

            fn has_class(&self, class: &str) -> bool {
                self.props.classes.iter().any(|c| c == class)
            }
        }
    };
}

/// Implement View::id() and View::meta() for a widget with `props: WidgetProps`.
///
/// # Arguments
/// * `$widget` - The widget type
/// * `$name` - The widget type name as a string literal (e.g., "Button")
///
/// # Example
/// ```rust,ignore
/// struct MyWidget {
///     props: WidgetProps,
/// }
///
/// impl View for MyWidget {
///     fn render(&self, ctx: &mut RenderContext) { /* ... */ }
///     impl_view_meta!(MyWidget, "MyWidget");
/// }
/// ```
#[macro_export]
macro_rules! impl_view_meta {
    ($name:expr) => {
        fn id(&self) -> Option<&str> {
            self.props.id.as_deref()
        }

        fn classes(&self) -> &[String] {
            &self.props.classes
        }

        fn meta(&self) -> $crate::dom::WidgetMeta {
            let mut meta = $crate::dom::WidgetMeta::new($name);
            if let Some(ref id) = self.props.id {
                meta.id = Some(id.clone());
            }
            for class in &self.props.classes {
                meta.classes.insert(class.clone());
            }
            meta
        }
    };
}

impl WidgetProps {
    /// Create new empty props
    pub fn new() -> Self {
        Self::default()
    }

    /// Set ID
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Add class
    pub fn class(mut self, class: impl Into<String>) -> Self {
        let class_str = class.into();
        if !self.classes.contains(&class_str) {
            self.classes.push(class_str);
        }
        self
    }

    /// Add multiple classes
    pub fn classes<I, S>(mut self, classes: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        for class in classes {
            let class_str = class.into();
            if !self.classes.contains(&class_str) {
                self.classes.push(class_str);
            }
        }
        self
    }

    /// Set inline style
    pub fn style(mut self, style: Style) -> Self {
        self.inline_style = Some(style);
        self
    }

    /// Get classes as slice
    pub fn classes_vec(&self) -> Vec<String> {
        self.classes.iter().cloned().collect()
    }
}

/// A rendered element
#[derive(Default)]
pub enum Element {
    /// Empty element
    #[default]
    Empty,
    /// View element
    View(Box<dyn View>),
}

// =============================================================================
// Builder Macros
// =============================================================================

/// Generate builder methods for widgets with `state: WidgetState` field.
///
/// This macro generates the following methods:
/// - `focused(self, bool) -> Self` - Set focused state
/// - `disabled(self, bool) -> Self` - Set disabled state
/// - `fg(self, Color) -> Self` - Set foreground color
/// - `bg(self, Color) -> Self` - Set background color
/// - `is_focused(&self) -> bool` - Check if focused
/// - `is_disabled(&self) -> bool` - Check if disabled
/// - `set_focused(&mut self, bool)` - Mutably set focused state
///
/// # Example
/// ```rust,ignore
/// struct MyWidget {
///     state: WidgetState,
///     props: WidgetProps,
/// }
///
/// impl_state_builders!(MyWidget);
/// ```
#[macro_export]
macro_rules! impl_state_builders {
    ($widget:ty) => {
        impl $widget {
            /// Set focused state
            pub fn focused(mut self, focused: bool) -> Self {
                self.state.focused = focused;
                self
            }

            /// Set disabled state
            pub fn disabled(mut self, disabled: bool) -> Self {
                self.state.disabled = disabled;
                self
            }

            /// Set foreground color
            pub fn fg(mut self, color: $crate::style::Color) -> Self {
                self.state.fg = Some(color);
                self
            }

            /// Set background color
            pub fn bg(mut self, color: $crate::style::Color) -> Self {
                self.state.bg = Some(color);
                self
            }

            /// Check if widget is focused
            pub fn is_focused(&self) -> bool {
                self.state.focused
            }

            /// Check if widget is disabled
            pub fn is_disabled(&self) -> bool {
                self.state.disabled
            }

            /// Set focused state (mutable)
            pub fn set_focused(&mut self, focused: bool) {
                self.state.focused = focused;
            }
        }
    };
}

/// Generate builder methods for widgets with `props: WidgetProps` field.
///
/// This macro generates the following methods:
/// - `element_id(self, impl Into<String>) -> Self` - Set CSS element ID
/// - `class(self, impl Into<String>) -> Self` - Add a CSS class
/// - `classes(self, IntoIterator<Item=S>) -> Self` - Add multiple CSS classes
///
/// # Example
/// ```rust,ignore
/// struct MyWidget {
///     props: WidgetProps,
/// }
///
/// impl_props_builders!(MyWidget);
/// ```
#[macro_export]
macro_rules! impl_props_builders {
    ($widget:ty) => {
        impl $widget {
            /// Set element ID for CSS selector (#id)
            pub fn element_id(mut self, id: impl Into<String>) -> Self {
                self.props.id = Some(id.into());
                self
            }

            /// Add a CSS class
            pub fn class(mut self, class: impl Into<String>) -> Self {
                let class_str = class.into();
                if !self.props.classes.contains(&class_str) {
                    self.props.classes.push(class_str);
                }
                self
            }

            /// Add multiple CSS classes
            pub fn classes<I, S>(mut self, classes: I) -> Self
            where
                I: IntoIterator<Item = S>,
                S: Into<String>,
            {
                for class in classes {
                    let class_str = class.into();
                    if !self.props.classes.contains(&class_str) {
                        self.props.classes.push(class_str);
                    }
                }
                self
            }
        }
    };
}

/// Generate all common builder methods for widgets with both `state: WidgetState`
/// and `props: WidgetProps` fields.
///
/// This is a convenience macro that combines `impl_state_builders!` and
/// `impl_props_builders!`.
///
/// Generated methods:
/// - State: `focused`, `disabled`, `fg`, `bg`, `is_focused`, `is_disabled`, `set_focused`
/// - Props: `element_id`, `class`, `classes`
///
/// # Example
/// ```rust,ignore
/// struct MyWidget {
///     label: String,
///     state: WidgetState,
///     props: WidgetProps,
/// }
///
/// impl MyWidget {
///     pub fn new(label: impl Into<String>) -> Self {
///         Self {
///             label: label.into(),
///             state: WidgetState::new(),
///             props: WidgetProps::new(),
///         }
///     }
/// }
///
/// // Generates: focused, disabled, fg, bg, is_focused, is_disabled,
/// //            set_focused, element_id, class, classes
/// impl_widget_builders!(MyWidget);
/// ```
#[macro_export]
macro_rules! impl_widget_builders {
    ($widget:ty) => {
        $crate::impl_state_builders!($widget);
        $crate::impl_props_builders!($widget);
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_result_default() {
        let result = EventResult::default();
        assert!(!result.is_consumed());
        assert!(!result.needs_render());
    }

    #[test]
    fn test_event_result_consumed() {
        let consumed = EventResult::Consumed;
        assert!(consumed.is_consumed());
        assert!(!consumed.needs_render());
    }

    #[test]
    fn test_event_result_consumed_and_render() {
        let result = EventResult::ConsumedAndRender;
        assert!(result.is_consumed());
        assert!(result.needs_render());
    }

    #[test]
    fn test_event_result_from_bool() {
        let handled: EventResult = true.into();
        assert_eq!(handled, EventResult::ConsumedAndRender);

        let ignored: EventResult = false.into();
        assert_eq!(ignored, EventResult::Ignored);
    }

    #[test]
    fn test_event_result_or() {
        // ConsumedAndRender wins
        assert_eq!(
            EventResult::Ignored.or(EventResult::ConsumedAndRender),
            EventResult::ConsumedAndRender
        );
        assert_eq!(
            EventResult::ConsumedAndRender.or(EventResult::Ignored),
            EventResult::ConsumedAndRender
        );

        // Consumed wins over Ignored
        assert_eq!(
            EventResult::Ignored.or(EventResult::Consumed),
            EventResult::Consumed
        );

        // Both Ignored = Ignored
        assert_eq!(
            EventResult::Ignored.or(EventResult::Ignored),
            EventResult::Ignored
        );
    }

    #[test]
    fn test_widget_state_new() {
        let state = WidgetState::new();
        assert!(!state.is_focused());
        assert!(!state.is_disabled());
        assert!(!state.is_pressed());
        assert!(!state.is_hovered());
        assert!(!state.is_interactive());
    }

    #[test]
    fn test_widget_state_builder() {
        let state = WidgetState::new()
            .focused(true)
            .disabled(false)
            .fg(Color::RED)
            .bg(Color::BLUE);

        assert!(state.is_focused());
        assert!(!state.is_disabled());
        assert_eq!(state.fg, Some(Color::RED));
        assert_eq!(state.bg, Some(Color::BLUE));
    }

    #[test]
    fn test_widget_state_effective_colors() {
        let default_color = Color::rgb(128, 128, 128);

        let normal = WidgetState::new().fg(Color::WHITE);
        assert_eq!(normal.effective_fg(default_color), Color::WHITE);

        let disabled = WidgetState::new().fg(Color::WHITE).disabled(true);
        assert_eq!(disabled.effective_fg(default_color), DISABLED_FG);
    }

    #[test]
    fn test_widget_state_reset_transient() {
        let mut state = WidgetState::new()
            .focused(true)
            .disabled(false)
            .pressed(true)
            .hovered(true);

        state.reset_transient();

        assert!(state.focused); // kept
        assert!(!state.disabled); // kept
        assert!(!state.pressed); // reset
        assert!(!state.hovered); // reset
    }

    #[test]
    fn test_widget_classes_exposure() {
        use crate::widget::Text;

        // Create a Text widget with classes
        let widget = Text::new("Test")
            .class("btn")
            .class("primary");

        // Verify classes are accessible via View trait
        let classes = View::classes(&widget);
        assert_eq!(classes.len(), 2);
        assert!(classes.contains(&"btn".to_string()));
        assert!(classes.contains(&"primary".to_string()));

        // Verify meta() also includes classes
        let meta = widget.meta();
        assert!(meta.classes.contains("btn"));
        assert!(meta.classes.contains("primary"));
    }
}
