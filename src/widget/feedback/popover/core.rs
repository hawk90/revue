use super::types::{PopoverArrow, PopoverPosition, PopoverStyle, PopoverTrigger};
use crate::event::Key;
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::widget::traits::{RenderContext, View, WidgetProps, WidgetState};

/// A popover widget for interactive overlays
///
/// Popovers are positioned relative to an anchor point and can contain
/// interactive content. They support various triggers and can be configured
/// to close on escape or click outside.
pub struct Popover {
    /// Content text (for simple popovers)
    pub(crate) content: String,
    /// Anchor point (x, y)
    pub(crate) anchor: (u16, u16),
    /// Position relative to anchor
    pub(crate) position: PopoverPosition,
    /// Trigger type
    pub(crate) trigger: PopoverTrigger,
    /// Visual style
    pub(crate) popover_style: PopoverStyle,
    /// Arrow style
    pub(crate) arrow: PopoverArrow,
    /// Whether popover is open
    pub(crate) open: bool,
    /// Close on escape key
    pub(crate) close_on_escape: bool,
    /// Close when clicking outside
    pub(crate) close_on_click_outside: bool,
    /// Title (optional)
    pub(crate) title: Option<String>,
    /// Max width (0 = auto)
    pub(crate) max_width: u16,
    /// Custom border color
    pub(crate) border_color: Option<Color>,
    /// Widget state
    pub(crate) state: WidgetState,
    /// Widget properties
    pub(crate) props: WidgetProps,
}

impl Popover {
    /// Create a new popover with content
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            anchor: (0, 0),
            position: PopoverPosition::Bottom,
            trigger: PopoverTrigger::Click,
            popover_style: PopoverStyle::Default,
            arrow: PopoverArrow::None,
            open: false,
            close_on_escape: true,
            close_on_click_outside: true,
            title: None,
            max_width: 40,
            border_color: None,
            state: WidgetState::new(),
            props: WidgetProps::new(),
        }
    }

    /// Set the content text
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = content.into();
        self
    }

    /// Set the anchor point
    pub fn anchor(mut self, x: u16, y: u16) -> Self {
        self.anchor = (x, y);
        self
    }

    /// Set the position relative to anchor
    pub fn position(mut self, position: PopoverPosition) -> Self {
        self.position = position;
        self
    }

    /// Set the trigger type
    pub fn trigger(mut self, trigger: PopoverTrigger) -> Self {
        self.trigger = trigger;
        self
    }

    /// Set the visual style
    pub fn popover_style(mut self, style: PopoverStyle) -> Self {
        self.popover_style = style;
        self
    }

    /// Set the arrow style
    pub fn arrow(mut self, arrow: PopoverArrow) -> Self {
        self.arrow = arrow;
        self
    }

    /// Set whether the popover is open
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// Set whether to close on escape key
    pub fn close_on_escape(mut self, close: bool) -> Self {
        self.close_on_escape = close;
        self
    }

    /// Set whether to close on click outside
    pub fn close_on_click_outside(mut self, close: bool) -> Self {
        self.close_on_click_outside = close;
        self
    }

    /// Set an optional title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set max width
    pub fn max_width(mut self, width: u16) -> Self {
        self.max_width = width;
        self
    }

    /// Set border color
    pub fn border_color(mut self, color: Color) -> Self {
        self.border_color = Some(color);
        self
    }

    // State management

    /// Open the popover
    pub fn show(&mut self) {
        self.open = true;
    }

    /// Close the popover
    pub fn hide(&mut self) {
        self.open = false;
    }

    /// Toggle the popover
    pub fn toggle(&mut self) {
        self.open = !self.open;
    }

    /// Check if the popover is open
    pub fn is_open(&self) -> bool {
        self.open
    }

    /// Set anchor position
    pub fn set_anchor(&mut self, x: u16, y: u16) {
        self.anchor = (x, y);
    }

    /// Handle keyboard input
    ///
    /// Returns `true` if the key was handled.
    pub fn handle_key(&mut self, key: &Key) -> bool {
        if !self.open {
            return false;
        }

        if self.close_on_escape && matches!(key, Key::Escape) {
            self.hide();
            return true;
        }

        false
    }

    /// Handle click at position
    ///
    /// Returns `true` if the click was handled.
    pub fn handle_click(&mut self, x: u16, y: u16, area_width: u16, area_height: u16) -> bool {
        if !self.open {
            // Check if click is on anchor (toggle open)
            if matches!(self.trigger, PopoverTrigger::Click)
                && x == self.anchor.0
                && y == self.anchor.1
            {
                self.show();
                return true;
            }
            return false;
        }

        // Check if click is inside popover
        let (popup_x, popup_y, _) = self.calculate_position(area_width, area_height);
        let (popup_w, popup_h) = self.calculate_dimensions();

        let inside = x >= popup_x && x < popup_x + popup_w && y >= popup_y && y < popup_y + popup_h;

        if inside {
            // Click inside popover - could be handled by content
            true
        } else if self.close_on_click_outside {
            self.hide();
            true
        } else {
            false
        }
    }

    /// Word wrap content
    fn wrap_content(&self) -> Vec<String> {
        let max_width = if self.max_width > 0 {
            self.max_width as usize
        } else {
            40
        };

        let mut lines = Vec::new();
        for line in self.content.lines() {
            if line.len() <= max_width {
                lines.push(line.to_string());
            } else {
                let mut current_line = String::new();
                for word in line.split_whitespace() {
                    if current_line.is_empty() {
                        current_line = word.to_string();
                    } else if current_line.len() + 1 + word.len() <= max_width {
                        current_line.push(' ');
                        current_line.push_str(word);
                    } else {
                        lines.push(current_line);
                        current_line = word.to_string();
                    }
                }
                if !current_line.is_empty() {
                    lines.push(current_line);
                }
            }
        }

        if lines.is_empty() {
            lines.push(String::new());
        }

        lines
    }

    /// Calculate popover dimensions
    fn calculate_dimensions(&self) -> (u16, u16) {
        let lines = self.wrap_content();
        let has_border = self.popover_style.border_chars().is_some();
        let has_title = self.title.is_some();

        let content_width = lines.iter().map(|l| l.len()).max().unwrap_or(0) as u16;
        let title_width = self.title.as_ref().map(|t| t.len() as u16 + 2).unwrap_or(0);
        let text_width = content_width.max(title_width);

        let width = text_width + if has_border { 4 } else { 2 };
        let height = lines.len() as u16
            + if has_border { 2 } else { 0 }
            + if has_title && has_border { 1 } else { 0 };

        (width.max(10), height.max(3))
    }

    /// Calculate position based on anchor and available space
    fn calculate_position(&self, area_width: u16, area_height: u16) -> (u16, u16, PopoverPosition) {
        let (popup_w, popup_h) = self.calculate_dimensions();
        let (anchor_x, anchor_y) = self.anchor;
        let arrow_offset: u16 = if matches!(self.arrow, PopoverArrow::None) {
            0
        } else {
            1
        };

        let (x, y, position) = match self.position {
            PopoverPosition::Auto => {
                // Auto-detect best position based on available space
                let space_above = anchor_y;
                let space_below = area_height.saturating_sub(anchor_y + 1);
                let space_left = anchor_x;
                let space_right = area_width.saturating_sub(anchor_x + 1);

                let pos = if space_below >= popup_h + arrow_offset {
                    PopoverPosition::Bottom
                } else if space_above >= popup_h + arrow_offset {
                    PopoverPosition::Top
                } else if space_right >= popup_w + arrow_offset {
                    PopoverPosition::Right
                } else if space_left >= popup_w + arrow_offset {
                    PopoverPosition::Left
                } else {
                    PopoverPosition::Bottom
                };

                let (x, y) = match pos {
                    PopoverPosition::Top => {
                        let x = anchor_x.saturating_sub(popup_w / 2);
                        let y = anchor_y.saturating_sub(popup_h + arrow_offset);
                        (x, y)
                    }
                    PopoverPosition::Bottom => {
                        let x = anchor_x.saturating_sub(popup_w / 2);
                        let y = anchor_y + 1 + arrow_offset;
                        (x, y)
                    }
                    PopoverPosition::Left => {
                        let x = anchor_x.saturating_sub(popup_w + arrow_offset);
                        let y = anchor_y.saturating_sub(popup_h / 2);
                        (x, y)
                    }
                    PopoverPosition::Right => {
                        let x = anchor_x + 1 + arrow_offset;
                        let y = anchor_y.saturating_sub(popup_h / 2);
                        (x, y)
                    }
                    _ => unreachable!("Auto position should be resolved before this match"),
                };
                (x, y, pos)
            }
            PopoverPosition::Top => {
                let x = anchor_x.saturating_sub(popup_w / 2);
                let y = anchor_y.saturating_sub(popup_h + arrow_offset);
                (x, y, PopoverPosition::Top)
            }
            PopoverPosition::Bottom => {
                let x = anchor_x.saturating_sub(popup_w / 2);
                let y = anchor_y + 1 + arrow_offset;
                (x, y, PopoverPosition::Bottom)
            }
            PopoverPosition::Left => {
                let x = anchor_x.saturating_sub(popup_w + arrow_offset);
                let y = anchor_y.saturating_sub(popup_h / 2);
                (x, y, PopoverPosition::Left)
            }
            PopoverPosition::Right => {
                let x = anchor_x + 1 + arrow_offset;
                let y = anchor_y.saturating_sub(popup_h / 2);
                (x, y, PopoverPosition::Right)
            }
        };

        let x = x.min(area_width.saturating_sub(popup_w));
        let y = y.min(area_height.saturating_sub(popup_h));

        (x, y, position)
    }
}

impl Default for Popover {
    fn default() -> Self {
        Self::new("")
    }
}

impl View for Popover {
    crate::impl_view_meta!("Popover");

    fn render(&self, ctx: &mut RenderContext) {
        if !self.open {
            return;
        }

        let area = ctx.area;
        let (popup_w, popup_h) = self.calculate_dimensions();
        let (popup_x, popup_y, actual_position) = self.calculate_position(area.width, area.height);

        // Get colors
        let (default_fg, default_bg, default_border) = self.popover_style.colors();
        let fg = self.state.fg.unwrap_or(default_fg);
        let bg = self.state.bg.unwrap_or(default_bg);
        let border_fg = self.border_color.unwrap_or(default_border);

        // Draw shadow for elevated style
        if matches!(self.popover_style, PopoverStyle::Elevated) {
            for dy in 1..=popup_h {
                for dx in 1..=popup_w {
                    let x = popup_x + dx;
                    let y = popup_y + dy;
                    if x < area.width && y < area.height {
                        let mut cell = Cell::new(' ');
                        cell.bg = Some(Color::rgb(15, 15, 15));
                        ctx.buffer.set(x, y, cell);
                    }
                }
            }
        }

        // Draw background
        for dy in 0..popup_h {
            for dx in 0..popup_w {
                let x = popup_x + dx;
                let y = popup_y + dy;
                if x < area.width && y < area.height {
                    let mut cell = Cell::new(' ');
                    cell.bg = Some(bg);
                    ctx.buffer.set(x, y, cell);
                }
            }
        }

        // Draw border
        let content_start_x;
        let content_start_y;

        if let Some(border) = self.popover_style.border_chars() {
            content_start_x = popup_x + 2;
            content_start_y = popup_y + 1;

            // Top border
            if popup_y < area.height {
                let mut tl = Cell::new(border.top_left);
                tl.fg = Some(border_fg);
                tl.bg = Some(bg);
                ctx.buffer.set(popup_x, popup_y, tl);

                for dx in 1..popup_w - 1 {
                    let mut h = Cell::new(border.horizontal);
                    h.fg = Some(border_fg);
                    h.bg = Some(bg);
                    ctx.buffer.set(popup_x + dx, popup_y, h);
                }

                let mut tr = Cell::new(border.top_right);
                tr.fg = Some(border_fg);
                tr.bg = Some(bg);
                ctx.buffer.set(popup_x + popup_w - 1, popup_y, tr);
            }

            // Title if present
            if let Some(ref title) = self.title {
                let title_x = popup_x + 2;
                let title_y = popup_y + 1;
                for (i, ch) in title.chars().enumerate() {
                    let x = title_x + i as u16;
                    if x < popup_x + popup_w - 2 && x < area.width {
                        let mut cell = Cell::new(ch);
                        cell.fg = Some(fg);
                        cell.bg = Some(bg);
                        cell.modifier |= Modifier::BOLD;
                        ctx.buffer.set(x, title_y, cell);
                    }
                }
            }

            // Side borders
            for dy in 1..popup_h - 1 {
                let y = popup_y + dy;
                if y < area.height {
                    let mut left = Cell::new(border.vertical);
                    left.fg = Some(border_fg);
                    left.bg = Some(bg);
                    ctx.buffer.set(popup_x, y, left);

                    let mut right = Cell::new(border.vertical);
                    right.fg = Some(border_fg);
                    right.bg = Some(bg);
                    ctx.buffer.set(popup_x + popup_w - 1, y, right);
                }
            }

            // Bottom border
            let bottom_y = popup_y + popup_h - 1;
            if bottom_y < area.height {
                let mut bl = Cell::new(border.bottom_left);
                bl.fg = Some(border_fg);
                bl.bg = Some(bg);
                ctx.buffer.set(popup_x, bottom_y, bl);

                for dx in 1..popup_w - 1 {
                    let mut h = Cell::new(border.horizontal);
                    h.fg = Some(border_fg);
                    h.bg = Some(bg);
                    ctx.buffer.set(popup_x + dx, bottom_y, h);
                }

                let mut br = Cell::new(border.bottom_right);
                br.fg = Some(border_fg);
                br.bg = Some(bg);
                ctx.buffer.set(popup_x + popup_w - 1, bottom_y, br);
            }
        } else {
            content_start_x = popup_x + 1;
            content_start_y = popup_y;
        }

        // Draw content
        let lines = self.wrap_content();
        let text_y_offset = if self.title.is_some() && self.popover_style.border_chars().is_some() {
            1
        } else {
            0
        };

        for (i, line) in lines.iter().enumerate() {
            let y = content_start_y + text_y_offset + i as u16;
            if y >= area.height || y >= popup_y + popup_h - 1 {
                break;
            }

            for (j, ch) in line.chars().enumerate() {
                let x = content_start_x + j as u16;
                if x < popup_x + popup_w - 1 && x < area.width {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(fg);
                    cell.bg = Some(bg);
                    ctx.buffer.set(x, y, cell);
                }
            }
        }

        // Draw arrow
        if !matches!(self.arrow, PopoverArrow::None) {
            let arrow_char = self.arrow.chars(actual_position);
            let (arrow_x, arrow_y) = match actual_position {
                PopoverPosition::Top => (self.anchor.0, popup_y + popup_h),
                PopoverPosition::Bottom => (self.anchor.0, popup_y.saturating_sub(1)),
                PopoverPosition::Left => (popup_x + popup_w, self.anchor.1),
                PopoverPosition::Right => (popup_x.saturating_sub(1), self.anchor.1),
                // Auto is already resolved to a concrete position in calculate_position
                // This case should never be reached, but use Bottom as fallback
                PopoverPosition::Auto => (self.anchor.0, popup_y.saturating_sub(1)),
            };

            if arrow_x < area.width && arrow_y < area.height {
                let mut cell = Cell::new(arrow_char);
                cell.fg = Some(border_fg);
                ctx.buffer.set(arrow_x, arrow_y, cell);
            }
        }
    }
}

crate::impl_styled_view!(Popover);
