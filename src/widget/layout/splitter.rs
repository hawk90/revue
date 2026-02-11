//! Splitter widget for resizable split panels
//!
//! Allows dividing an area into resizable panes with draggable dividers.

use crate::event::Key;
use crate::layout::Rect;
use crate::render::Cell;
use crate::style::Color;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// Split orientation
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SplitOrientation {
    /// Split horizontally (panes side by side)
    #[default]
    Horizontal,
    /// Split vertically (panes stacked)
    Vertical,
}

/// Splitter style
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SplitterStyle {
    /// Single line
    #[default]
    Line,
    /// Double line
    Double,
    /// Thick line
    Thick,
    /// No visible splitter
    Hidden,
}

impl SplitterStyle {
    fn char(&self, orientation: SplitOrientation) -> char {
        match (self, orientation) {
            (SplitterStyle::Line, SplitOrientation::Horizontal) => '│',
            (SplitterStyle::Line, SplitOrientation::Vertical) => '─',
            (SplitterStyle::Double, SplitOrientation::Horizontal) => '║',
            (SplitterStyle::Double, SplitOrientation::Vertical) => '═',
            (SplitterStyle::Thick, SplitOrientation::Horizontal) => '┃',
            (SplitterStyle::Thick, SplitOrientation::Vertical) => '━',
            (SplitterStyle::Hidden, _) => ' ',
        }
    }
}

/// A pane in the splitter
pub struct Pane {
    /// Pane identifier
    pub id: String,
    /// Minimum size (percentage or absolute)
    pub min_size: u16,
    /// Maximum size (0 = unlimited)
    pub max_size: u16,
    /// Initial size ratio (0.0 - 1.0)
    pub ratio: f32,
    /// Whether pane is collapsible
    pub collapsible: bool,
    /// Whether pane is collapsed
    pub collapsed: bool,
}

impl Pane {
    /// Create a new pane
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            min_size: 5,
            max_size: 0,
            ratio: 0.5,
            collapsible: false,
            collapsed: false,
        }
    }

    /// Set minimum size
    pub fn min_size(mut self, size: u16) -> Self {
        self.min_size = size;
        self
    }

    /// Set maximum size
    pub fn max_size(mut self, size: u16) -> Self {
        self.max_size = size;
        self
    }

    /// Set initial ratio
    pub fn ratio(mut self, ratio: f32) -> Self {
        self.ratio = ratio.clamp(0.0, 1.0);
        self
    }

    /// Make collapsible
    pub fn collapsible(mut self) -> Self {
        self.collapsible = true;
        self
    }

    /// Toggle collapsed state
    pub fn toggle_collapse(&mut self) {
        if self.collapsible {
            self.collapsed = !self.collapsed;
        }
    }
}

/// Splitter widget
pub struct Splitter {
    /// Panes
    panes: Vec<Pane>,
    /// Orientation
    orientation: SplitOrientation,
    /// Style
    style: SplitterStyle,
    /// Splitter color
    color: Color,
    /// Active splitter color
    active_color: Color,
    /// Currently active divider (for resizing)
    active_divider: Option<usize>,
    /// Focused pane index
    focused_pane: usize,
    /// Splitter width
    splitter_width: u16,
    /// Widget props for CSS integration
    props: WidgetProps,
}

impl Splitter {
    /// Create a new splitter
    pub fn new() -> Self {
        Self {
            panes: Vec::new(),
            orientation: SplitOrientation::Horizontal,
            style: SplitterStyle::Line,
            color: Color::rgb(80, 80, 80),
            active_color: Color::CYAN,
            active_divider: None,
            focused_pane: 0,
            splitter_width: 1,
            props: WidgetProps::new(),
        }
    }

    /// Set orientation
    pub fn orientation(mut self, orientation: SplitOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Set horizontal orientation
    pub fn horizontal(mut self) -> Self {
        self.orientation = SplitOrientation::Horizontal;
        self
    }

    /// Set vertical orientation
    pub fn vertical(mut self) -> Self {
        self.orientation = SplitOrientation::Vertical;
        self
    }

    /// Add a pane
    pub fn pane(mut self, pane: Pane) -> Self {
        self.panes.push(pane);
        self
    }

    /// Add panes
    pub fn panes(mut self, panes: Vec<Pane>) -> Self {
        self.panes.extend(panes);
        self
    }

    /// Set style
    pub fn style(mut self, style: SplitterStyle) -> Self {
        self.style = style;
        self
    }

    /// Set splitter color
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Set active color
    pub fn active_color(mut self, color: Color) -> Self {
        self.active_color = color;
        self
    }

    /// Get pane areas
    pub fn pane_areas(&self, area: Rect) -> Vec<(String, Rect)> {
        let mut areas = Vec::new();
        let visible_panes: Vec<_> = self.panes.iter().filter(|p| !p.collapsed).collect();

        if visible_panes.is_empty() {
            return areas;
        }

        let total_splitter_width =
            (visible_panes.len().saturating_sub(1)) as u16 * self.splitter_width;
        let available = match self.orientation {
            SplitOrientation::Horizontal => area.width.saturating_sub(total_splitter_width),
            SplitOrientation::Vertical => area.height.saturating_sub(total_splitter_width),
        };

        // Normalize ratios
        let total_ratio: f32 = visible_panes.iter().map(|p| p.ratio).sum();
        let mut offset = 0u16;

        for (i, pane) in visible_panes.iter().enumerate() {
            let ratio = if total_ratio > 0.0 {
                pane.ratio / total_ratio
            } else {
                1.0 / visible_panes.len() as f32
            };
            let size = (available as f32 * ratio).clamp(0.0, available as f32);
            let mut size = size as u16;

            // Apply constraints
            size = size.max(pane.min_size);
            if pane.max_size > 0 {
                size = size.min(pane.max_size);
            }

            // Last pane takes remaining space
            if i == visible_panes.len() - 1 {
                size = available.saturating_sub(offset);
            }

            let pane_area = match self.orientation {
                SplitOrientation::Horizontal => {
                    Rect::new(area.x + offset, area.y, size, area.height)
                }
                SplitOrientation::Vertical => Rect::new(area.x, area.y + offset, area.width, size),
            };

            areas.push((pane.id.clone(), pane_area));
            offset += size + self.splitter_width;
        }

        areas
    }

    /// Get focused pane id
    pub fn focused(&self) -> Option<&str> {
        self.panes.get(self.focused_pane).map(|p| p.id.as_str())
    }

    /// Focus next pane
    pub fn focus_next(&mut self) {
        let visible: Vec<_> = self
            .panes
            .iter()
            .enumerate()
            .filter(|(_, p)| !p.collapsed)
            .map(|(i, _)| i)
            .collect();

        if let Some(pos) = visible.iter().position(|&i| i == self.focused_pane) {
            let next = (pos + 1) % visible.len();
            self.focused_pane = visible[next];
        }
    }

    /// Focus previous pane
    pub fn focus_prev(&mut self) {
        let visible: Vec<_> = self
            .panes
            .iter()
            .enumerate()
            .filter(|(_, p)| !p.collapsed)
            .map(|(i, _)| i)
            .collect();

        if let Some(pos) = visible.iter().position(|&i| i == self.focused_pane) {
            let prev = if pos == 0 { visible.len() - 1 } else { pos - 1 };
            self.focused_pane = visible[prev];
        }
    }

    /// Start resizing divider
    pub fn start_resize(&mut self, divider: usize) {
        if divider < self.panes.len() - 1 {
            self.active_divider = Some(divider);
        }
    }

    /// Stop resizing
    pub fn stop_resize(&mut self) {
        self.active_divider = None;
    }

    /// Resize by delta
    pub fn resize(&mut self, delta: i16) {
        if let Some(divider) = self.active_divider {
            if divider < self.panes.len() - 1 {
                let current_ratio = self.panes[divider].ratio;
                let next_ratio = self.panes[divider + 1].ratio;

                let change = delta as f32 * 0.01;
                let new_current = (current_ratio + change).clamp(0.1, 0.9);
                let new_next = (next_ratio - change).clamp(0.1, 0.9);

                self.panes[divider].ratio = new_current;
                self.panes[divider + 1].ratio = new_next;
            }
        }
    }

    /// Toggle collapse for pane
    pub fn toggle_pane(&mut self, index: usize) {
        if let Some(pane) = self.panes.get_mut(index) {
            pane.toggle_collapse();
        }
    }

    /// Handle key input
    pub fn handle_key(&mut self, key: &Key) -> bool {
        match key {
            Key::Tab => {
                self.focus_next();
                true
            }
            Key::Left | Key::Char('h') if self.active_divider.is_some() => {
                self.resize(-5);
                true
            }
            Key::Right | Key::Char('l') if self.active_divider.is_some() => {
                self.resize(5);
                true
            }
            Key::Up | Key::Char('k') if self.active_divider.is_some() => {
                self.resize(-5);
                true
            }
            Key::Down | Key::Char('j') if self.active_divider.is_some() => {
                self.resize(5);
                true
            }
            Key::Enter if self.active_divider.is_some() => {
                self.stop_resize();
                true
            }
            Key::Escape if self.active_divider.is_some() => {
                self.stop_resize();
                true
            }
            _ => false,
        }
    }
}

impl Default for Splitter {
    fn default() -> Self {
        Self::new()
    }
}

impl View for Splitter {
    crate::impl_view_meta!("Splitter");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let areas = self.pane_areas(area);

        // Draw splitters between panes
        for (i, (_, pane_area)) in areas.iter().enumerate().take(areas.len().saturating_sub(1)) {
            let is_active = self.active_divider == Some(i);
            let color = if is_active {
                self.active_color
            } else {
                self.color
            };
            let ch = self.style.char(self.orientation);

            match self.orientation {
                SplitOrientation::Horizontal => {
                    let x = pane_area.x + pane_area.width;
                    for y in area.y..area.y + area.height {
                        let mut cell = Cell::new(ch);
                        cell.fg = Some(color);
                        ctx.buffer.set(x, y, cell);
                    }
                }
                SplitOrientation::Vertical => {
                    let y = pane_area.y + pane_area.height;
                    for x in area.x..area.x + area.width {
                        let mut cell = Cell::new(ch);
                        cell.fg = Some(color);
                        ctx.buffer.set(x, y, cell);
                    }
                }
            }
        }
    }
}

impl_styled_view!(Splitter);
impl_props_builders!(Splitter);

/// Two-pane horizontal split
pub struct HSplit {
    /// Left pane ratio
    pub ratio: f32,
    /// Minimum left width
    pub min_left: u16,
    /// Minimum right width
    pub min_right: u16,
    /// Splitter visible
    pub show_splitter: bool,
    /// Splitter color
    pub color: Color,
}

impl HSplit {
    /// Create new horizontal split
    pub fn new(ratio: f32) -> Self {
        Self {
            ratio: ratio.clamp(0.1, 0.9),
            min_left: 5,
            min_right: 5,
            show_splitter: true,
            color: Color::rgb(80, 80, 80),
        }
    }

    /// Set minimum widths
    pub fn min_widths(mut self, left: u16, right: u16) -> Self {
        self.min_left = left;
        self.min_right = right;
        self
    }

    /// Hide splitter
    pub fn hide_splitter(mut self) -> Self {
        self.show_splitter = false;
        self
    }

    /// Get left and right areas
    pub fn areas(&self, area: Rect) -> (Rect, Rect) {
        let splitter_width = if self.show_splitter { 1 } else { 0 };
        let available = area.width.saturating_sub(splitter_width);

        let left_width = (available as f32 * self.ratio).clamp(0.0, available as f32);
        let mut left_width = left_width as u16;
        left_width = left_width.max(self.min_left);
        left_width = left_width.min(available.saturating_sub(self.min_right));

        let right_width = available.saturating_sub(left_width);

        let left = Rect::new(area.x, area.y, left_width, area.height);
        let right = Rect::new(
            area.x + left_width + splitter_width,
            area.y,
            right_width,
            area.height,
        );

        (left, right)
    }
}

impl View for HSplit {
    fn render(&self, ctx: &mut RenderContext) {
        if !self.show_splitter {
            return;
        }

        let (left, _) = self.areas(ctx.area);
        let x = left.x + left.width;

        for y in ctx.area.y..ctx.area.y + ctx.area.height {
            let mut cell = Cell::new('│');
            cell.fg = Some(self.color);
            ctx.buffer.set(x, y, cell);
        }
    }
}

/// Two-pane vertical split
pub struct VSplit {
    /// Top pane ratio
    pub ratio: f32,
    /// Minimum top height
    pub min_top: u16,
    /// Minimum bottom height
    pub min_bottom: u16,
    /// Splitter visible
    pub show_splitter: bool,
    /// Splitter color
    pub color: Color,
}

impl VSplit {
    /// Create new vertical split
    pub fn new(ratio: f32) -> Self {
        Self {
            ratio: ratio.clamp(0.1, 0.9),
            min_top: 3,
            min_bottom: 3,
            show_splitter: true,
            color: Color::rgb(80, 80, 80),
        }
    }

    /// Get top and bottom areas
    pub fn areas(&self, area: Rect) -> (Rect, Rect) {
        let splitter_height = if self.show_splitter { 1 } else { 0 };
        let available = area.height.saturating_sub(splitter_height);

        let top_height = (available as f32 * self.ratio).clamp(0.0, available as f32);
        let mut top_height = top_height as u16;
        top_height = top_height.max(self.min_top);
        top_height = top_height.min(available.saturating_sub(self.min_bottom));

        let bottom_height = available.saturating_sub(top_height);

        let top = Rect::new(area.x, area.y, area.width, top_height);
        let bottom = Rect::new(
            area.x,
            area.y + top_height + splitter_height,
            area.width,
            bottom_height,
        );

        (top, bottom)
    }
}

impl View for VSplit {
    fn render(&self, ctx: &mut RenderContext) {
        if !self.show_splitter {
            return;
        }

        let (top, _) = self.areas(ctx.area);
        let y = top.y + top.height;

        for x in ctx.area.x..ctx.area.x + ctx.area.width {
            let mut cell = Cell::new('─');
            cell.fg = Some(self.color);
            ctx.buffer.set(x, y, cell);
        }
    }
}

// Helper functions

/// Create a new splitter container
pub fn splitter() -> Splitter {
    Splitter::new()
}

/// Create a new pane with an identifier
pub fn pane(id: impl Into<String>) -> Pane {
    Pane::new(id)
}

/// Create a horizontal split with the given ratio
pub fn hsplit(ratio: f32) -> HSplit {
    HSplit::new(ratio)
}

/// Create a vertical split with the given ratio
pub fn vsplit(ratio: f32) -> VSplit {
    VSplit::new(ratio)
}
