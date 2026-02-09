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

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // SplitOrientation enum tests
    // =========================================================================

    #[test]
    fn test_split_orientation_default() {
        assert_eq!(SplitOrientation::default(), SplitOrientation::Horizontal);
    }

    #[test]
    fn test_split_orientation_clone() {
        let orient = SplitOrientation::Vertical;
        assert_eq!(orient, orient.clone());
    }

    #[test]
    fn test_split_orientation_copy() {
        let o1 = SplitOrientation::Horizontal;
        let o2 = o1;
        assert_eq!(o1, SplitOrientation::Horizontal);
        assert_eq!(o2, SplitOrientation::Horizontal);
    }

    #[test]
    fn test_split_orientation_partial_eq() {
        assert_eq!(SplitOrientation::Horizontal, SplitOrientation::Horizontal);
        assert_eq!(SplitOrientation::Vertical, SplitOrientation::Vertical);
        assert_ne!(SplitOrientation::Horizontal, SplitOrientation::Vertical);
    }

    #[test]
    fn test_split_orientation_debug() {
        let debug_str = format!("{:?}", SplitOrientation::Vertical);
        assert!(debug_str.contains("Vertical"));
    }

    // =========================================================================
    // SplitterStyle enum tests
    // =========================================================================

    #[test]
    fn test_splitter_style_default() {
        assert_eq!(SplitterStyle::default(), SplitterStyle::Line);
    }

    #[test]
    fn test_splitter_style_clone() {
        let style = SplitterStyle::Double;
        assert_eq!(style, style.clone());
    }

    #[test]
    fn test_splitter_style_copy() {
        let s1 = SplitterStyle::Thick;
        let s2 = s1;
        assert_eq!(s1, SplitterStyle::Thick);
        assert_eq!(s2, SplitterStyle::Thick);
    }

    #[test]
    fn test_splitter_style_partial_eq() {
        assert_eq!(SplitterStyle::Line, SplitterStyle::Line);
        assert_eq!(SplitterStyle::Double, SplitterStyle::Double);
        assert_eq!(SplitterStyle::Thick, SplitterStyle::Thick);
        assert_eq!(SplitterStyle::Hidden, SplitterStyle::Hidden);
        assert_ne!(SplitterStyle::Line, SplitterStyle::Double);
    }

    #[test]
    fn test_splitter_style_debug() {
        let debug_str = format!("{:?}", SplitterStyle::Double);
        assert!(debug_str.contains("Double"));
    }

    // =========================================================================
    // SplitterStyle::char tests
    // =========================================================================

    #[test]
    fn test_splitter_style_char_horizontal_line() {
        assert_eq!(SplitterStyle::Line.char(SplitOrientation::Horizontal), '│');
    }

    #[test]
    fn test_splitter_style_char_horizontal_double() {
        assert_eq!(
            SplitterStyle::Double.char(SplitOrientation::Horizontal),
            '║'
        );
    }

    #[test]
    fn test_splitter_style_char_horizontal_thick() {
        assert_eq!(SplitterStyle::Thick.char(SplitOrientation::Horizontal), '┃');
    }

    #[test]
    fn test_splitter_style_char_horizontal_hidden() {
        assert_eq!(
            SplitterStyle::Hidden.char(SplitOrientation::Horizontal),
            ' '
        );
    }

    #[test]
    fn test_splitter_style_char_vertical_line() {
        assert_eq!(SplitterStyle::Line.char(SplitOrientation::Vertical), '─');
    }

    #[test]
    fn test_splitter_style_char_vertical_double() {
        assert_eq!(SplitterStyle::Double.char(SplitOrientation::Vertical), '═');
    }

    #[test]
    fn test_splitter_style_char_vertical_thick() {
        assert_eq!(SplitterStyle::Thick.char(SplitOrientation::Vertical), '━');
    }

    #[test]
    fn test_splitter_style_char_vertical_hidden() {
        assert_eq!(SplitterStyle::Hidden.char(SplitOrientation::Vertical), ' ');
    }

    // =========================================================================
    // Pane tests
    // =========================================================================

    #[test]
    fn test_pane() {
        let p = Pane::new("main").min_size(10).ratio(0.6).collapsible();

        assert_eq!(p.id, "main");
        assert_eq!(p.min_size, 10);
        assert_eq!(p.ratio, 0.6);
        assert!(p.collapsible);
    }

    #[test]
    fn test_pane_new_defaults() {
        let p = Pane::new("test");
        assert_eq!(p.id, "test");
        assert_eq!(p.min_size, 5);
        assert_eq!(p.max_size, 0);
        assert_eq!(p.ratio, 0.5);
        assert!(!p.collapsible);
        assert!(!p.collapsed);
    }

    #[test]
    fn test_pane_id_string() {
        let p = Pane::new(String::from("owned"));
        assert_eq!(p.id, "owned");
    }

    #[test]
    fn test_pane_max_size() {
        let p = Pane::new("test").max_size(100);
        assert_eq!(p.max_size, 100);
    }

    #[test]
    fn test_pane_ratio_clamps_low() {
        let p = Pane::new("test").ratio(-0.5);
        assert_eq!(p.ratio, 0.0);
    }

    #[test]
    fn test_pane_ratio_clamps_high() {
        let p = Pane::new("test").ratio(1.5);
        assert_eq!(p.ratio, 1.0);
    }

    #[test]
    fn test_pane_ratio_zero() {
        let p = Pane::new("test").ratio(0.0);
        assert_eq!(p.ratio, 0.0);
    }

    #[test]
    fn test_pane_ratio_one() {
        let p = Pane::new("test").ratio(1.0);
        assert_eq!(p.ratio, 1.0);
    }

    #[test]
    fn test_pane_ratio_half() {
        let p = Pane::new("test").ratio(0.5);
        assert_eq!(p.ratio, 0.5);
    }

    #[test]
    fn test_pane_collapsible() {
        let p = Pane::new("test").collapsible();
        assert!(p.collapsible);
    }

    #[test]
    fn test_pane_toggle_collapse() {
        let mut p = Pane::new("test").collapsible();
        assert!(!p.collapsed);

        p.toggle_collapse();
        assert!(p.collapsed);

        p.toggle_collapse();
        assert!(!p.collapsed);
    }

    #[test]
    fn test_pane_toggle_collapse_not_collapsible() {
        let mut p = Pane::new("test");
        assert!(!p.collapsible);
        assert!(!p.collapsed);

        p.toggle_collapse();
        assert!(!p.collapsed); // No change
    }

    #[test]
    fn test_pane_builder_chain() {
        let p = Pane::new("test")
            .min_size(10)
            .max_size(100)
            .ratio(0.75)
            .collapsible();

        assert_eq!(p.id, "test");
        assert_eq!(p.min_size, 10);
        assert_eq!(p.max_size, 100);
        assert_eq!(p.ratio, 0.75);
        assert!(p.collapsible);
    }

    // =========================================================================
    // Splitter::new and default tests
    // =========================================================================

    #[test]
    fn test_splitter_new() {
        let s = Splitter::new();
        assert!(s.panes.is_empty());
        assert_eq!(s.orientation, SplitOrientation::Horizontal);
        assert_eq!(s.style, SplitterStyle::Line);
        assert_eq!(s.color, Color::rgb(80, 80, 80));
        assert_eq!(s.active_color, Color::CYAN);
        assert_eq!(s.focused_pane, 0);
        assert_eq!(s.splitter_width, 1);
    }

    #[test]
    fn test_splitter_default() {
        let s = Splitter::default();
        assert!(s.panes.is_empty());
    }

    // =========================================================================
    // Splitter builder tests
    // =========================================================================

    #[test]
    fn test_splitter() {
        let split = Splitter::new()
            .horizontal()
            .pane(Pane::new("left").ratio(0.3))
            .pane(Pane::new("right").ratio(0.7));

        assert_eq!(split.panes.len(), 2);
        assert_eq!(split.orientation, SplitOrientation::Horizontal);
    }

    #[test]
    fn test_splitter_vertical() {
        let split = Splitter::new().vertical();
        assert_eq!(split.orientation, SplitOrientation::Vertical);
    }

    #[test]
    fn test_splitter_horizontal() {
        let split = Splitter::new().horizontal();
        assert_eq!(split.orientation, SplitOrientation::Horizontal);
    }

    #[test]
    fn test_splitter_orientation() {
        let split = Splitter::new().orientation(SplitOrientation::Vertical);
        assert_eq!(split.orientation, SplitOrientation::Vertical);
    }

    #[test]
    fn test_splitter_pane() {
        let split = Splitter::new().pane(Pane::new("single"));
        assert_eq!(split.panes.len(), 1);
        assert_eq!(split.panes[0].id, "single");
    }

    #[test]
    fn test_splitter_panes() {
        let split = Splitter::new().panes(vec![Pane::new("a"), Pane::new("b"), Pane::new("c")]);
        assert_eq!(split.panes.len(), 3);
    }

    #[test]
    fn test_splitter_style() {
        let split = Splitter::new().style(SplitterStyle::Double);
        assert_eq!(split.style, SplitterStyle::Double);
    }

    #[test]
    fn test_splitter_color() {
        let split = Splitter::new().color(Color::RED);
        assert_eq!(split.color, Color::RED);
    }

    #[test]
    fn test_splitter_active_color() {
        let split = Splitter::new().active_color(Color::YELLOW);
        assert_eq!(split.active_color, Color::YELLOW);
    }

    // =========================================================================
    // Splitter::pane_areas tests
    // =========================================================================

    #[test]
    fn test_pane_areas() {
        let split = Splitter::new()
            .horizontal()
            .pane(Pane::new("left").ratio(0.5))
            .pane(Pane::new("right").ratio(0.5));

        let area = Rect::new(0, 0, 81, 24); // 81 = 40 + 1 + 40
        let areas = split.pane_areas(area);

        assert_eq!(areas.len(), 2);
        assert_eq!(areas[0].0, "left");
        assert_eq!(areas[1].0, "right");
    }

    #[test]
    fn test_pane_areas_vertical() {
        let split = Splitter::new()
            .vertical()
            .pane(Pane::new("top").ratio(0.5))
            .pane(Pane::new("bottom").ratio(0.5));

        let area = Rect::new(0, 0, 80, 41);
        let areas = split.pane_areas(area);

        assert_eq!(areas.len(), 2);
        assert_eq!(areas[0].0, "top");
        assert_eq!(areas[1].0, "bottom");
    }

    #[test]
    fn test_pane_areas_empty() {
        let split = Splitter::new();
        let area = Rect::new(0, 0, 80, 24);
        let areas = split.pane_areas(area);
        assert!(areas.is_empty());
    }

    #[test]
    fn test_pane_areas_collapsed() {
        let mut split = Splitter::new()
            .pane(Pane::new("a").ratio(0.3).collapsible())
            .pane(Pane::new("b").ratio(0.3).collapsible())
            .pane(Pane::new("c").ratio(0.4).collapsible());

        // Collapse the first two panes
        split.toggle_pane(0);
        split.toggle_pane(1);

        let area = Rect::new(0, 0, 81, 24);
        let areas = split.pane_areas(area);

        // Only third pane is visible
        assert_eq!(areas.len(), 1);
        assert_eq!(areas[0].0, "c");
    }

    #[test]
    fn test_pane_areas_all_collapsed() {
        let mut split = Splitter::new()
            .pane(Pane::new("a").collapsible())
            .pane(Pane::new("b").collapsible());

        split.toggle_pane(0);
        split.toggle_pane(1);

        let area = Rect::new(0, 0, 81, 24);
        let areas = split.pane_areas(area);
        assert!(areas.is_empty());
    }

    // =========================================================================
    // Splitter::focused tests
    // =========================================================================

    #[test]
    fn test_splitter_focused() {
        let split = Splitter::new()
            .pane(Pane::new("first"))
            .pane(Pane::new("second"));
        assert_eq!(split.focused(), Some("first"));
    }

    #[test]
    fn test_splitter_focused_empty() {
        let split = Splitter::new();
        assert_eq!(split.focused(), None);
    }

    #[test]
    fn test_splitter_focused_collapsed() {
        let mut split = Splitter::new()
            .pane(Pane::new("a"))
            .pane(Pane::new("b").collapsible());

        split.toggle_pane(1);
        // First pane is visible, second is collapsed
        assert_eq!(split.focused(), Some("a"));
    }

    // =========================================================================
    // Splitter::focus_next tests
    // =========================================================================

    #[test]
    fn test_focus() {
        let mut split = Splitter::new()
            .pane(Pane::new("a"))
            .pane(Pane::new("b"))
            .pane(Pane::new("c"));

        assert_eq!(split.focused(), Some("a"));

        split.focus_next();
        assert_eq!(split.focused(), Some("b"));

        split.focus_next();
        assert_eq!(split.focused(), Some("c"));

        split.focus_next();
        assert_eq!(split.focused(), Some("a"));
    }

    #[test]
    fn test_focus_next_wraps() {
        let mut split = Splitter::new().pane(Pane::new("a")).pane(Pane::new("b"));

        split.focus_next();
        assert_eq!(split.focused(), Some("b"));

        split.focus_next();
        assert_eq!(split.focused(), Some("a"));
    }

    #[test]
    fn test_focus_next_with_collapsed() {
        let mut split = Splitter::new()
            .pane(Pane::new("a"))
            .pane(Pane::new("b").collapsible())
            .pane(Pane::new("c"));

        split.toggle_pane(1);

        split.focus_next();
        assert_eq!(split.focused(), Some("c"));

        split.focus_next();
        assert_eq!(split.focused(), Some("a"));
    }

    // =========================================================================
    // Splitter::focus_prev tests
    // =========================================================================

    #[test]
    fn test_focus_prev() {
        let mut split = Splitter::new()
            .pane(Pane::new("a"))
            .pane(Pane::new("b"))
            .pane(Pane::new("c"));

        split.focused_pane = 2;
        assert_eq!(split.focused(), Some("c"));

        split.focus_prev();
        assert_eq!(split.focused(), Some("b"));

        split.focus_prev();
        assert_eq!(split.focused(), Some("a"));

        split.focus_prev();
        assert_eq!(split.focused(), Some("c"));
    }

    #[test]
    fn test_focus_prev_wraps() {
        let mut split = Splitter::new().pane(Pane::new("a")).pane(Pane::new("b"));

        split.focus_prev();
        assert_eq!(split.focused(), Some("b"));
    }

    #[test]
    fn test_focus_prev_with_collapsed() {
        let mut split = Splitter::new()
            .pane(Pane::new("a"))
            .pane(Pane::new("b").collapsible())
            .pane(Pane::new("c"));

        split.toggle_pane(1);
        split.focused_pane = 2;
        assert_eq!(split.focused(), Some("c"));

        split.focus_prev();
        assert_eq!(split.focused(), Some("a"));
    }

    // =========================================================================
    // Splitter::resize tests
    // =========================================================================

    #[test]
    fn test_start_resize() {
        let mut split = Splitter::new()
            .pane(Pane::new("a").ratio(0.5))
            .pane(Pane::new("b").ratio(0.5));

        split.start_resize(0);
        assert_eq!(split.active_divider, Some(0));
    }

    #[test]
    fn test_start_resize_invalid() {
        let mut split = Splitter::new().pane(Pane::new("a")).pane(Pane::new("b"));

        split.start_resize(1); // Last divider doesn't exist
        assert_eq!(split.active_divider, None);
    }

    #[test]
    fn test_stop_resize() {
        let mut split = Splitter::new()
            .pane(Pane::new("a").ratio(0.5))
            .pane(Pane::new("b").ratio(0.5));

        split.start_resize(0);
        assert_eq!(split.active_divider, Some(0));

        split.stop_resize();
        assert_eq!(split.active_divider, None);
    }

    #[test]
    fn test_resize() {
        let mut split = Splitter::new()
            .pane(Pane::new("a").ratio(0.5))
            .pane(Pane::new("b").ratio(0.5));

        split.start_resize(0);
        split.resize(10); // Increase by 10%

        // After resize: current ratio increases, next decreases
        assert!(split.panes[0].ratio > 0.5);
        assert!(split.panes[1].ratio < 0.5);
    }

    #[test]
    fn test_resize_clamps() {
        let mut split = Splitter::new()
            .pane(Pane::new("a").ratio(0.5))
            .pane(Pane::new("b").ratio(0.5));

        split.start_resize(0);
        split.resize(1000); // Large delta
                            // Ratios should be clamped to [0.1, 0.9]
        assert!(split.panes[0].ratio <= 0.9);
        assert!(split.panes[0].ratio >= 0.1);
        assert!(split.panes[1].ratio <= 0.9);
        assert!(split.panes[1].ratio >= 0.1);
    }

    #[test]
    fn test_resize_no_active() {
        let mut split = Splitter::new()
            .pane(Pane::new("a").ratio(0.5))
            .pane(Pane::new("b").ratio(0.5));

        let r0 = split.panes[0].ratio;
        split.resize(10);
        // No change without active divider
        assert_eq!(split.panes[0].ratio, r0);
    }

    // =========================================================================
    // Splitter::toggle_pane tests
    // =========================================================================

    #[test]
    fn test_toggle_pane() {
        let mut split = Splitter::new()
            .pane(Pane::new("a").collapsible())
            .pane(Pane::new("b"));

        assert!(!split.panes[0].collapsed);

        split.toggle_pane(0);
        assert!(split.panes[0].collapsed);

        split.toggle_pane(0);
        assert!(!split.panes[0].collapsed);
    }

    #[test]
    fn test_toggle_pane_not_collapsible() {
        let mut split = Splitter::new().pane(Pane::new("a"));

        split.toggle_pane(0);
        assert!(!split.panes[0].collapsed);
    }

    #[test]
    fn test_toggle_pane_invalid() {
        let mut split = Splitter::new().pane(Pane::new("a"));

        split.toggle_pane(5); // Invalid index
                              // Should not panic
    }

    // =========================================================================
    // Splitter::handle_key tests
    // =========================================================================

    #[test]
    fn test_handle_key_tab() {
        use crate::event::Key;

        let mut split = Splitter::new().pane(Pane::new("a")).pane(Pane::new("b"));

        assert!(split.handle_key(&Key::Tab));
        assert_eq!(split.focused(), Some("b"));
    }

    #[test]
    fn test_handle_key_left_resize() {
        use crate::event::Key;

        let mut split = Splitter::new()
            .pane(Pane::new("a").ratio(0.5))
            .pane(Pane::new("b").ratio(0.5));

        split.start_resize(0);
        assert!(split.handle_key(&Key::Left));
        // First pane ratio decreased
        assert!(split.panes[0].ratio < 0.5);
    }

    #[test]
    fn test_handle_key_left_no_resize() {
        use crate::event::Key;

        let mut split = Splitter::new()
            .pane(Pane::new("a").ratio(0.5))
            .pane(Pane::new("b").ratio(0.5));

        assert!(!split.handle_key(&Key::Left)); // No active resize
    }

    #[test]
    fn test_handle_key_right_resize() {
        use crate::event::Key;

        let mut split = Splitter::new()
            .pane(Pane::new("a").ratio(0.5))
            .pane(Pane::new("b").ratio(0.5));

        split.start_resize(0);
        assert!(split.handle_key(&Key::Right));
        // First pane ratio increased
        assert!(split.panes[0].ratio > 0.5);
    }

    #[test]
    fn test_handle_key_vim_h() {
        use crate::event::Key;

        let mut split = Splitter::new()
            .pane(Pane::new("a").ratio(0.5))
            .pane(Pane::new("b").ratio(0.5));

        split.start_resize(0);
        assert!(split.handle_key(&Key::Char('h')));
        assert!(split.panes[0].ratio < 0.5);
    }

    #[test]
    fn test_handle_key_vim_l() {
        use crate::event::Key;

        let mut split = Splitter::new()
            .pane(Pane::new("a").ratio(0.5))
            .pane(Pane::new("b").ratio(0.5));

        split.start_resize(0);
        assert!(split.handle_key(&Key::Char('l')));
        assert!(split.panes[0].ratio > 0.5);
    }

    #[test]
    fn test_handle_key_enter_stops_resize() {
        use crate::event::Key;

        let mut split = Splitter::new()
            .pane(Pane::new("a").ratio(0.5))
            .pane(Pane::new("b").ratio(0.5));

        split.start_resize(0);
        assert!(split.handle_key(&Key::Enter));
        assert_eq!(split.active_divider, None);
    }

    #[test]
    fn test_handle_key_escape_stops_resize() {
        use crate::event::Key;

        let mut split = Splitter::new()
            .pane(Pane::new("a").ratio(0.5))
            .pane(Pane::new("b").ratio(0.5));

        split.start_resize(0);
        assert!(split.handle_key(&Key::Escape));
        assert_eq!(split.active_divider, None);
    }

    #[test]
    fn test_handle_key_unhandled() {
        use crate::event::Key;

        let mut split = Splitter::new().pane(Pane::new("a")).pane(Pane::new("b"));

        assert!(!split.handle_key(&Key::Up)); // No active resize
        assert!(!split.handle_key(&Key::Char('x')));
    }

    // =========================================================================
    // HSplit tests
    // =========================================================================

    #[test]
    fn test_hsplit() {
        let split = HSplit::new(0.3);
        let area = Rect::new(0, 0, 100, 50);
        let (left, right) = split.areas(area);

        assert!(left.width > 0);
        assert!(right.width > 0);
        assert_eq!(left.width + right.width + 1, area.width);
    }

    #[test]
    fn test_hsplit_new_clamps_ratio() {
        let split = HSplit::new(-0.5);
        assert_eq!(split.ratio, 0.1); // Clamped to 0.1

        let split2 = HSplit::new(1.5);
        assert_eq!(split2.ratio, 0.9); // Clamped to 0.9
    }

    #[test]
    fn test_hsplit_new_clamps_min() {
        let split = HSplit::new(0.5);
        assert_eq!(split.min_left, 5);
        assert_eq!(split.min_right, 5);
    }

    #[test]
    fn test_hsplit_min_widths() {
        let split = HSplit::new(0.5).min_widths(10, 15);
        assert_eq!(split.min_left, 10);
        assert_eq!(split.min_right, 15);
    }

    #[test]
    fn test_hsplit_hide_splitter() {
        let split = HSplit::new(0.5).hide_splitter();
        assert!(!split.show_splitter);
    }

    #[test]
    fn test_hsplit_color_field() {
        let mut split = HSplit::new(0.5);
        split.color = Color::RED;
        assert_eq!(split.color, Color::RED);
    }

    // =========================================================================
    // VSplit tests
    // =========================================================================

    #[test]
    fn test_vsplit() {
        let split = VSplit::new(0.5);
        let area = Rect::new(0, 0, 80, 24);
        let (top, bottom) = split.areas(area);

        assert!(top.height > 0);
        assert!(bottom.height > 0);
        assert_eq!(top.height + bottom.height + 1, area.height);
    }

    #[test]
    fn test_vsplit_new_clamps_ratio() {
        let split = VSplit::new(-0.5);
        assert_eq!(split.ratio, 0.1); // Clamped to 0.1

        let split2 = VSplit::new(1.5);
        assert_eq!(split2.ratio, 0.9); // Clamped to 0.9
    }

    #[test]
    fn test_vsplit_new_clamps_min() {
        let split = VSplit::new(0.5);
        assert_eq!(split.min_top, 3);
        assert_eq!(split.min_bottom, 3);
    }

    #[test]
    fn test_vsplit_show_splitter_field() {
        let mut split = VSplit::new(0.5);
        split.show_splitter = false;
        assert!(!split.show_splitter);
    }

    #[test]
    fn test_vsplit_color_field() {
        let mut split = VSplit::new(0.5);
        split.color = Color::GREEN;
        assert_eq!(split.color, Color::GREEN);
    }

    // =========================================================================
    // Helper function tests
    // =========================================================================

    #[test]
    fn test_splitter_helper() {
        let s = splitter();
        assert!(s.panes.is_empty());
    }

    #[test]
    fn test_pane_helper() {
        let p = pane("test");
        assert_eq!(p.id, "test");
    }

    #[test]
    fn test_pane_helper_string() {
        let p = pane(String::from("owned"));
        assert_eq!(p.id, "owned");
    }

    #[test]
    fn test_hsplit_helper() {
        let s = hsplit(0.5);
        assert_eq!(s.ratio, 0.5);
    }

    #[test]
    fn test_vsplit_helper() {
        let s = vsplit(0.3);
        assert_eq!(s.ratio, 0.3);
    }

    // =========================================================================
    // Splitter builder chain tests
    // =========================================================================

    #[test]
    fn test_splitter_full_builder_chain() {
        let split = Splitter::new()
            .vertical()
            .style(SplitterStyle::Double)
            .color(Color::CYAN)
            .active_color(Color::YELLOW)
            .panes(vec![
                Pane::new("a").ratio(0.25).min_size(10),
                Pane::new("b").ratio(0.75).min_size(20),
            ]);

        assert_eq!(split.orientation, SplitOrientation::Vertical);
        assert_eq!(split.style, SplitterStyle::Double);
        assert_eq!(split.color, Color::CYAN);
        assert_eq!(split.active_color, Color::YELLOW);
        assert_eq!(split.panes.len(), 2);
        assert_eq!(split.panes[0].min_size, 10);
        assert_eq!(split.panes[1].min_size, 20);
    }
}
