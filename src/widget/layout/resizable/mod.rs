//! Resizable widget wrapper for dynamic sizing
//!
//! Wraps any widget and provides resize handles for interactive resizing.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::widget::{Resizable, ResizeHandle, Text};
//!
//! Resizable::new(Text::new("Resizable content"))
//!     .min_size(10, 5)
//!     .max_size(80, 40)
//!     .handles(ResizeHandle::ALL)
//!     .on_resize(|w, h| println!("New size: {}x{}", w, h))
//! ```

mod types;

pub use types::{ResizeDirection, ResizeHandle, ResizeStyle};

use crate::event::Key;
use crate::layout::Rect;
use crate::style::Color;
use crate::widget::traits::{RenderContext, View, WidgetProps, WidgetState};
use crate::{impl_styled_view, impl_view_meta, impl_widget_builders};

/// Resizable widget wrapper
pub struct Resizable<F = fn(u16, u16)>
where
    F: FnMut(u16, u16),
{
    /// Current width
    width: u16,
    /// Current height
    height: u16,
    /// Minimum width
    min_width: u16,
    /// Minimum height
    min_height: u16,
    /// Maximum width (0 = unlimited)
    max_width: u16,
    /// Maximum height (0 = unlimited)
    max_height: u16,
    /// Enabled resize handles
    handles: Vec<ResizeHandle>,
    /// Handle size (corner area)
    handle_size: u16,
    /// Visual style
    style: ResizeStyle,
    /// Handle color
    handle_color: Color,
    /// Active handle color
    active_color: Color,
    /// Currently resizing
    resizing: bool,
    /// Active resize direction
    resize_direction: ResizeDirection,
    /// Hovered handle
    hovered_handle: Option<ResizeHandle>,
    /// Resize callback
    on_resize: Option<F>,
    /// Preserve aspect ratio
    preserve_aspect: bool,
    /// Initial aspect ratio (width/height)
    aspect_ratio: f32,
    /// Snap to grid
    snap_to_grid: Option<(u16, u16)>,
    /// Widget state
    state: WidgetState,
    /// Widget props
    props: WidgetProps,
}

impl Resizable<fn(u16, u16)> {
    /// Create a new resizable wrapper with initial dimensions
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width: width.max(1),
            height: height.max(1),
            min_width: 3,
            min_height: 3,
            max_width: 0,
            max_height: 0,
            handles: ResizeHandle::ALL.to_vec(),
            handle_size: 1,
            style: ResizeStyle::default(),
            handle_color: Color::rgb(100, 100, 100),
            active_color: Color::CYAN,
            resizing: false,
            resize_direction: ResizeDirection::NONE,
            hovered_handle: None,
            on_resize: None,
            preserve_aspect: false,
            aspect_ratio: width as f32 / height.max(1) as f32,
            snap_to_grid: None,
            state: WidgetState::new(),
            props: WidgetProps::new(),
        }
    }
}

impl<F> Resizable<F>
where
    F: FnMut(u16, u16),
{
    /// Set minimum size
    pub fn min_size(mut self, width: u16, height: u16) -> Self {
        self.min_width = width.max(1);
        self.min_height = height.max(1);
        self
    }

    /// Set maximum size
    pub fn max_size(mut self, width: u16, height: u16) -> Self {
        self.max_width = width;
        self.max_height = height;
        self
    }

    /// Set allowed handles
    pub fn handles(mut self, handles: &[ResizeHandle]) -> Self {
        self.handles = handles.to_vec();
        self
    }

    /// Set visual style
    pub fn style(mut self, style: ResizeStyle) -> Self {
        self.style = style;
        self
    }

    /// Set handle color
    pub fn handle_color(mut self, color: Color) -> Self {
        self.handle_color = color;
        self
    }

    /// Set active color
    pub fn active_color(mut self, color: Color) -> Self {
        self.active_color = color;
        self
    }

    /// Preserve aspect ratio during resize
    pub fn preserve_aspect_ratio(mut self) -> Self {
        self.preserve_aspect = true;
        self.aspect_ratio = self.width as f32 / self.height.max(1) as f32;
        self
    }

    /// Set custom aspect ratio
    pub fn aspect_ratio(mut self, ratio: f32) -> Self {
        self.preserve_aspect = true;
        self.aspect_ratio = ratio;
        self
    }

    /// Snap size to grid
    pub fn snap_to_grid(mut self, grid_width: u16, grid_height: u16) -> Self {
        self.snap_to_grid = Some((grid_width.max(1), grid_height.max(1)));
        self
    }

    /// Set resize callback
    pub fn on_resize<G>(self, handler: G) -> Resizable<G>
    where
        G: FnMut(u16, u16),
    {
        Resizable {
            width: self.width,
            height: self.height,
            min_width: self.min_width,
            min_height: self.min_height,
            max_width: self.max_width,
            max_height: self.max_height,
            handles: self.handles,
            handle_size: self.handle_size,
            style: self.style,
            handle_color: self.handle_color,
            active_color: self.active_color,
            resizing: self.resizing,
            resize_direction: self.resize_direction,
            hovered_handle: self.hovered_handle,
            on_resize: Some(handler),
            preserve_aspect: self.preserve_aspect,
            aspect_ratio: self.aspect_ratio,
            snap_to_grid: self.snap_to_grid,
            state: self.state,
            props: self.props,
        }
    }

    /// Get current size
    pub fn size(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    /// Set size directly
    pub fn set_size(&mut self, width: u16, height: u16) {
        let (w, h) = self.constrain_size(width, height);
        self.width = w;
        self.height = h;
    }

    /// Get content area (inside borders)
    pub fn content_area(&self, area: Rect) -> Rect {
        let border = match self.style {
            ResizeStyle::Border => 1,
            _ => 0,
        };
        Rect::new(
            area.x + border,
            area.y + border,
            self.width.saturating_sub(border * 2),
            self.height.saturating_sub(border * 2),
        )
    }

    /// Check if currently resizing
    pub fn is_resizing(&self) -> bool {
        self.resizing
    }

    /// Start resize operation
    pub fn start_resize(&mut self, handle: ResizeHandle) {
        if self.handles.contains(&handle) {
            self.resizing = true;
            self.resize_direction = ResizeDirection::from_handle(handle);
        }
    }

    /// End resize operation
    pub fn end_resize(&mut self) {
        self.resizing = false;
        self.resize_direction = ResizeDirection::NONE;
    }

    /// Apply resize delta
    pub fn apply_delta(&mut self, dx: i16, dy: i16) {
        if !self.resizing {
            return;
        }

        let new_width = if self.resize_direction.horizontal != 0 {
            let delta = dx * self.resize_direction.horizontal as i16;
            (self.width as i16 + delta).max(1) as u16
        } else {
            self.width
        };

        let new_height = if self.resize_direction.vertical != 0 {
            let delta = dy * self.resize_direction.vertical as i16;
            (self.height as i16 + delta).max(1) as u16
        } else {
            self.height
        };

        let (w, h) = self.constrain_size(new_width, new_height);

        if w != self.width || h != self.height {
            self.width = w;
            self.height = h;
            if let Some(ref mut callback) = self.on_resize {
                callback(w, h);
            }
        }
    }

    /// Constrain size within limits
    fn constrain_size(&self, mut width: u16, mut height: u16) -> (u16, u16) {
        // Apply grid snapping
        if let Some((gw, gh)) = self.snap_to_grid {
            width = ((width + gw / 2) / gw) * gw;
            height = ((height + gh / 2) / gh) * gh;
        }

        // Apply min/max constraints
        width = width.max(self.min_width);
        height = height.max(self.min_height);

        if self.max_width > 0 {
            width = width.min(self.max_width);
        }
        if self.max_height > 0 {
            height = height.min(self.max_height);
        }

        // Apply aspect ratio
        if self.preserve_aspect {
            let current_ratio = width as f32 / height.max(1) as f32;
            if (current_ratio - self.aspect_ratio).abs() > 0.01 {
                // Adjust height to match aspect ratio
                let new_height = (width as f32 / self.aspect_ratio)
                    .max(0.0)
                    .min(u16::MAX as f32) as u16;
                height = new_height.max(self.min_height);
                if self.max_height > 0 {
                    height = height.min(self.max_height);
                }
            }
        }

        (width.max(1), height.max(1))
    }

    /// Hit test for handle at position
    pub fn handle_at(&self, x: u16, y: u16, area: Rect) -> Option<ResizeHandle> {
        for handle in &self.handles {
            if handle.hit_test(x, y, area, self.handle_size) {
                return Some(*handle);
            }
        }
        None
    }

    /// Set hovered handle
    pub fn set_hovered(&mut self, handle: Option<ResizeHandle>) {
        self.hovered_handle = handle;
    }

    /// Handle keyboard resize
    pub fn handle_key(&mut self, key: &Key) -> bool {
        if !self.state.focused {
            return false;
        }

        let delta = 1i16;
        match key {
            Key::Left if self.handles.contains(&ResizeHandle::Right) => {
                self.resize_direction = ResizeDirection::from_handle(ResizeHandle::Right);
                self.resizing = true;
                self.apply_delta(-delta, 0);
                self.resizing = false;
                true
            }
            Key::Right if self.handles.contains(&ResizeHandle::Right) => {
                self.resize_direction = ResizeDirection::from_handle(ResizeHandle::Right);
                self.resizing = true;
                self.apply_delta(delta, 0);
                self.resizing = false;
                true
            }
            Key::Up if self.handles.contains(&ResizeHandle::Bottom) => {
                self.resize_direction = ResizeDirection::from_handle(ResizeHandle::Bottom);
                self.resizing = true;
                self.apply_delta(0, -delta);
                self.resizing = false;
                true
            }
            Key::Down if self.handles.contains(&ResizeHandle::Bottom) => {
                self.resize_direction = ResizeDirection::from_handle(ResizeHandle::Bottom);
                self.resizing = true;
                self.apply_delta(0, delta);
                self.resizing = false;
                true
            }
            _ => false,
        }
    }

    /// Draw resize handles
    fn draw_handles(&self, ctx: &mut RenderContext) {
        let area = ctx.area;

        match self.style {
            ResizeStyle::Border => {
                self.draw_border(ctx, area);
            }
            ResizeStyle::Subtle => {
                if self.hovered_handle.is_some() || self.resizing {
                    self.draw_border(ctx, area);
                }
            }
            ResizeStyle::Dots => {
                self.draw_corner_dots(ctx, area);
            }
            ResizeStyle::Hidden => {}
        }
    }

    fn draw_border(&self, ctx: &mut RenderContext, area: Rect) {
        let color = if self.resizing {
            self.active_color
        } else if self.hovered_handle.is_some() {
            Color::rgb(150, 150, 150)
        } else {
            self.handle_color
        };

        // Top border
        for x in area.x..area.x + self.width.min(area.width) {
            if let Some(cell) = ctx.buffer.get_mut(x, area.y) {
                let ch = if x == area.x {
                    '┌'
                } else if x == area.x + self.width - 1 {
                    '┐'
                } else {
                    '─'
                };
                cell.symbol = ch;
                cell.fg = Some(color);
            }
        }

        // Bottom border
        let bottom_y = area.y + self.height.saturating_sub(1);
        for x in area.x..area.x + self.width.min(area.width) {
            if let Some(cell) = ctx.buffer.get_mut(x, bottom_y) {
                let ch = if x == area.x {
                    '└'
                } else if x == area.x + self.width - 1 {
                    '┘'
                } else {
                    '─'
                };
                cell.symbol = ch;
                cell.fg = Some(color);
            }
        }

        // Side borders
        for y in (area.y + 1)..bottom_y {
            if let Some(cell) = ctx.buffer.get_mut(area.x, y) {
                cell.symbol = '│';
                cell.fg = Some(color);
            }
            if let Some(cell) = ctx.buffer.get_mut(area.x + self.width - 1, y) {
                cell.symbol = '│';
                cell.fg = Some(color);
            }
        }

        // Highlight active handle
        if let Some(handle) = self.hovered_handle {
            let active_color = self.active_color;
            match handle {
                ResizeHandle::TopLeft => {
                    if let Some(cell) = ctx.buffer.get_mut(area.x, area.y) {
                        cell.fg = Some(active_color);
                    }
                }
                ResizeHandle::TopRight => {
                    if let Some(cell) = ctx.buffer.get_mut(area.x + self.width - 1, area.y) {
                        cell.fg = Some(active_color);
                    }
                }
                ResizeHandle::BottomLeft => {
                    if let Some(cell) = ctx.buffer.get_mut(area.x, bottom_y) {
                        cell.fg = Some(active_color);
                    }
                }
                ResizeHandle::BottomRight => {
                    if let Some(cell) = ctx.buffer.get_mut(area.x + self.width - 1, bottom_y) {
                        cell.fg = Some(active_color);
                    }
                }
                _ => {}
            }
        }
    }

    fn draw_corner_dots(&self, ctx: &mut RenderContext, area: Rect) {
        let color = if self.resizing {
            self.active_color
        } else {
            self.handle_color
        };

        let corners = [
            (area.x, area.y),
            (area.x + self.width - 1, area.y),
            (area.x, area.y + self.height - 1),
            (area.x + self.width - 1, area.y + self.height - 1),
        ];

        for (x, y) in corners {
            if let Some(cell) = ctx.buffer.get_mut(x, y) {
                cell.symbol = '●';
                cell.fg = Some(color);
            }
        }
    }
}

impl<F> View for Resizable<F>
where
    F: FnMut(u16, u16),
{
    fn render(&self, ctx: &mut RenderContext) {
        self.draw_handles(ctx);
    }

    impl_view_meta!("Resizable");
}

impl_styled_view!(Resizable);
impl_widget_builders!(Resizable);

/// Create a new resizable wrapper
pub fn resizable(width: u16, height: u16) -> Resizable<fn(u16, u16)> {
    Resizable::new(width, height)
}

#[cfg(test)]
mod tests {
    //! Resizable widget tests

    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::style::Color;
    use crate::widget::traits::RenderContext;
    use crate::widget::traits::View;

    #[test]
    fn test_resizable_new() {
        let r = Resizable::new(20, 10);
        assert_eq!(r.size(), (20, 10));
    }

    #[test]
    fn test_resizable_constraints() {
        let mut r = Resizable::new(20, 10).min_size(5, 5).max_size(50, 30);

        r.set_size(3, 3);
        assert_eq!(r.size(), (5, 5));

        r.set_size(100, 100);
        assert_eq!(r.size(), (50, 30));
    }

    #[test]
    fn test_resizable_aspect_ratio() {
        let mut r = Resizable::new(20, 10).preserve_aspect_ratio();
        r.set_size(40, 10);
        // Height should adjust to maintain 2:1 ratio
        assert_eq!(r.width, 40);
        assert_eq!(r.height, 20);
    }

    #[test]
    fn test_resizable_grid_snap() {
        let mut r = Resizable::new(20, 10).snap_to_grid(5, 5);
        r.set_size(23, 12);
        assert_eq!(r.size(), (25, 10));
    }

    #[test]
    fn test_resizable_handles() {
        let r = Resizable::new(20, 10).handles(ResizeHandle::CORNERS);
        assert_eq!(r.handles.len(), 4);
        assert!(r.handles.contains(&ResizeHandle::TopLeft));
        assert!(!r.handles.contains(&ResizeHandle::Top));
    }

    #[test]
    fn test_resize_operation() {
        let mut r = Resizable::new(20, 10);
        r.start_resize(ResizeHandle::BottomRight);
        assert!(r.is_resizing());

        r.apply_delta(5, 3);
        assert_eq!(r.size(), (25, 13));

        r.end_resize();
        assert!(!r.is_resizing());
    }

    #[test]
    fn test_handle_hit_test() {
        let area = Rect::new(0, 0, 20, 10);

        // Bottom-right corner
        assert!(ResizeHandle::BottomRight.hit_test(19, 9, area, 1));
        assert!(!ResizeHandle::BottomRight.hit_test(10, 5, area, 1));

        // Top edge
        assert!(ResizeHandle::Top.hit_test(10, 0, area, 1));
        assert!(!ResizeHandle::Top.hit_test(0, 0, area, 1)); // Corner, not edge
    }

    #[test]
    fn test_resize_direction() {
        let dir = ResizeDirection::from_handle(ResizeHandle::BottomRight);
        assert_eq!(dir.horizontal, 1);
        assert_eq!(dir.vertical, 1);

        let dir = ResizeDirection::from_handle(ResizeHandle::Left);
        assert_eq!(dir.horizontal, -1);
        assert_eq!(dir.vertical, 0);
    }

    #[test]
    fn test_content_area() {
        let r = Resizable::new(20, 10).style(ResizeStyle::Border);
        let area = Rect::new(5, 5, 20, 10);
        let content = r.content_area(area);

        assert_eq!(content.x, 6);
        assert_eq!(content.y, 6);
        assert_eq!(content.width, 18);
        assert_eq!(content.height, 8);
    }

    // ==================== ResizeHandle Tests ====================

    #[test]
    fn test_resize_handle_all_constant() {
        assert_eq!(ResizeHandle::ALL.len(), 8);
        assert!(ResizeHandle::ALL.contains(&ResizeHandle::Top));
        assert!(ResizeHandle::ALL.contains(&ResizeHandle::Bottom));
        assert!(ResizeHandle::ALL.contains(&ResizeHandle::Left));
        assert!(ResizeHandle::ALL.contains(&ResizeHandle::Right));
        assert!(ResizeHandle::ALL.contains(&ResizeHandle::TopLeft));
        assert!(ResizeHandle::ALL.contains(&ResizeHandle::TopRight));
        assert!(ResizeHandle::ALL.contains(&ResizeHandle::BottomLeft));
        assert!(ResizeHandle::ALL.contains(&ResizeHandle::BottomRight));
    }

    #[test]
    fn test_resize_handle_edges_constant() {
        assert_eq!(ResizeHandle::EDGES.len(), 4);
        assert!(ResizeHandle::EDGES.contains(&ResizeHandle::Top));
        assert!(ResizeHandle::EDGES.contains(&ResizeHandle::Bottom));
        assert!(ResizeHandle::EDGES.contains(&ResizeHandle::Left));
        assert!(ResizeHandle::EDGES.contains(&ResizeHandle::Right));
        assert!(!ResizeHandle::EDGES.contains(&ResizeHandle::TopLeft));
    }

    #[test]
    fn test_resize_handle_corners_constant() {
        assert_eq!(ResizeHandle::CORNERS.len(), 4);
        assert!(ResizeHandle::CORNERS.contains(&ResizeHandle::TopLeft));
        assert!(ResizeHandle::CORNERS.contains(&ResizeHandle::TopRight));
        assert!(ResizeHandle::CORNERS.contains(&ResizeHandle::BottomLeft));
        assert!(ResizeHandle::CORNERS.contains(&ResizeHandle::BottomRight));
        assert!(!ResizeHandle::CORNERS.contains(&ResizeHandle::Top));
    }

    #[test]
    fn test_resize_handle_debug_clone_eq() {
        let handle = ResizeHandle::TopLeft;
        let cloned = handle;
        assert_eq!(handle, cloned);
        let _ = format!("{:?}", handle);
    }

    #[test]
    fn test_resize_handle_hit_test_top() {
        let area = Rect::new(0, 0, 20, 10);
        // Top edge (middle section, excluding corners)
        assert!(ResizeHandle::Top.hit_test(10, 0, area, 1));
        assert!(ResizeHandle::Top.hit_test(5, 0, area, 1));
        // Corner positions should not match top edge
        assert!(!ResizeHandle::Top.hit_test(0, 0, area, 1));
        assert!(!ResizeHandle::Top.hit_test(19, 0, area, 1));
        // Wrong row
        assert!(!ResizeHandle::Top.hit_test(10, 5, area, 1));
    }

    #[test]
    fn test_resize_handle_hit_test_bottom() {
        let area = Rect::new(0, 0, 20, 10);
        // Bottom edge (middle section)
        assert!(ResizeHandle::Bottom.hit_test(10, 9, area, 1));
        // Corners should not match
        assert!(!ResizeHandle::Bottom.hit_test(0, 9, area, 1));
        assert!(!ResizeHandle::Bottom.hit_test(19, 9, area, 1));
    }

    #[test]
    fn test_resize_handle_hit_test_left() {
        let area = Rect::new(0, 0, 20, 10);
        // Left edge (middle section)
        assert!(ResizeHandle::Left.hit_test(0, 5, area, 1));
        // Corners should not match
        assert!(!ResizeHandle::Left.hit_test(0, 0, area, 1));
        assert!(!ResizeHandle::Left.hit_test(0, 9, area, 1));
    }

    #[test]
    fn test_resize_handle_hit_test_right() {
        let area = Rect::new(0, 0, 20, 10);
        // Right edge (middle section)
        assert!(ResizeHandle::Right.hit_test(19, 5, area, 1));
        // Corners should not match
        assert!(!ResizeHandle::Right.hit_test(19, 0, area, 1));
        assert!(!ResizeHandle::Right.hit_test(19, 9, area, 1));
    }

    #[test]
    fn test_resize_handle_hit_test_top_left() {
        let area = Rect::new(0, 0, 20, 10);
        assert!(ResizeHandle::TopLeft.hit_test(0, 0, area, 1));
        assert!(ResizeHandle::TopLeft.hit_test(1, 1, area, 1));
        assert!(!ResizeHandle::TopLeft.hit_test(10, 5, area, 1));
    }

    #[test]
    fn test_resize_handle_hit_test_top_right() {
        let area = Rect::new(0, 0, 20, 10);
        assert!(ResizeHandle::TopRight.hit_test(19, 0, area, 1));
        assert!(ResizeHandle::TopRight.hit_test(18, 1, area, 1));
        assert!(!ResizeHandle::TopRight.hit_test(10, 5, area, 1));
    }

    #[test]
    fn test_resize_handle_hit_test_bottom_left() {
        let area = Rect::new(0, 0, 20, 10);
        assert!(ResizeHandle::BottomLeft.hit_test(0, 9, area, 1));
        assert!(ResizeHandle::BottomLeft.hit_test(1, 8, area, 1));
        assert!(!ResizeHandle::BottomLeft.hit_test(10, 5, area, 1));
    }

    // ==================== ResizeDirection Tests ====================

    #[test]
    fn test_resize_direction_none() {
        let dir = ResizeDirection::NONE;
        assert_eq!(dir.horizontal, 0);
        assert_eq!(dir.vertical, 0);
    }

    #[test]
    fn test_resize_direction_from_handle_all() {
        let top = ResizeDirection::from_handle(ResizeHandle::Top);
        assert_eq!(top.horizontal, 0);
        assert_eq!(top.vertical, -1);

        let bottom = ResizeDirection::from_handle(ResizeHandle::Bottom);
        assert_eq!(bottom.horizontal, 0);
        assert_eq!(bottom.vertical, 1);

        let left = ResizeDirection::from_handle(ResizeHandle::Left);
        assert_eq!(left.horizontal, -1);
        assert_eq!(left.vertical, 0);

        let right = ResizeDirection::from_handle(ResizeHandle::Right);
        assert_eq!(right.horizontal, 1);
        assert_eq!(right.vertical, 0);

        let top_left = ResizeDirection::from_handle(ResizeHandle::TopLeft);
        assert_eq!(top_left.horizontal, -1);
        assert_eq!(top_left.vertical, -1);

        let top_right = ResizeDirection::from_handle(ResizeHandle::TopRight);
        assert_eq!(top_right.horizontal, 1);
        assert_eq!(top_right.vertical, -1);

        let bottom_left = ResizeDirection::from_handle(ResizeHandle::BottomLeft);
        assert_eq!(bottom_left.horizontal, -1);
        assert_eq!(bottom_left.vertical, 1);

        let bottom_right = ResizeDirection::from_handle(ResizeHandle::BottomRight);
        assert_eq!(bottom_right.horizontal, 1);
        assert_eq!(bottom_right.vertical, 1);
    }

    #[test]
    fn test_resize_direction_debug_clone_eq() {
        let dir = ResizeDirection::NONE;
        let cloned = dir;
        assert_eq!(dir, cloned);
        let _ = format!("{:?}", dir);
    }

    // ==================== ResizeStyle Tests ====================

    #[test]
    fn test_resize_style_default() {
        assert_eq!(ResizeStyle::default(), ResizeStyle::Border);
    }

    #[test]
    fn test_resize_style_debug_clone_eq() {
        let style = ResizeStyle::Subtle;
        let cloned = style;
        assert_eq!(style, cloned);
        let _ = format!("{:?}", style);
    }

    #[test]
    fn test_resize_style_variants() {
        let _border = ResizeStyle::Border;
        let _subtle = ResizeStyle::Subtle;
        let _hidden = ResizeStyle::Hidden;
        let _dots = ResizeStyle::Dots;
    }

    // ==================== Builder Methods Tests ====================

    #[test]
    fn test_resizable_handle_color() {
        let r = Resizable::new(20, 10).handle_color(Color::RED);
        assert_eq!(r.handle_color, Color::RED);
    }

    #[test]
    fn test_resizable_active_color() {
        let r = Resizable::new(20, 10).active_color(Color::GREEN);
        assert_eq!(r.active_color, Color::GREEN);
    }

    #[test]
    fn test_resizable_custom_aspect_ratio() {
        let r = Resizable::new(20, 10).aspect_ratio(4.0);
        assert!(r.preserve_aspect);
        assert!((r.aspect_ratio - 4.0).abs() < 0.01);
    }

    #[test]
    fn test_resizable_style_subtle() {
        let r = Resizable::new(20, 10).style(ResizeStyle::Subtle);
        assert_eq!(r.style, ResizeStyle::Subtle);
    }

    #[test]
    fn test_resizable_style_hidden() {
        let r = Resizable::new(20, 10).style(ResizeStyle::Hidden);
        assert_eq!(r.style, ResizeStyle::Hidden);
    }

    #[test]
    fn test_resizable_style_dots() {
        let r = Resizable::new(20, 10).style(ResizeStyle::Dots);
        assert_eq!(r.style, ResizeStyle::Dots);
    }

    // ==================== Callback Tests ====================

    #[test]
    fn test_resizable_on_resize_callback() {
        use std::cell::Cell;
        use std::rc::Rc;

        let called = Rc::new(Cell::new(false));
        let width_received = Rc::new(Cell::new(0u16));
        let height_received = Rc::new(Cell::new(0u16));

        let called_clone = called.clone();
        let width_clone = width_received.clone();
        let height_clone = height_received.clone();

        let mut r = Resizable::new(20, 10).on_resize(move |w, h| {
            called_clone.set(true);
            width_clone.set(w);
            height_clone.set(h);
        });

        r.start_resize(ResizeHandle::BottomRight);
        r.apply_delta(5, 3);

        assert!(called.get());
        assert_eq!(width_received.get(), 25);
        assert_eq!(height_received.get(), 13);
    }

    // ==================== Keyboard Handling Tests ====================

    #[test]
    fn test_resizable_handle_key_not_focused() {
        let mut r = Resizable::new(20, 10);
        // Not focused by default
        let handled = r.handle_key(&Key::Right);
        assert!(!handled);
        assert_eq!(r.size(), (20, 10)); // Size unchanged
    }

    #[test]
    fn test_resizable_handle_key_right() {
        let mut r = Resizable::new(20, 10);
        r.state.focused = true;

        let handled = r.handle_key(&Key::Right);
        assert!(handled);
        assert_eq!(r.size(), (21, 10));
    }

    #[test]
    fn test_resizable_handle_key_left() {
        let mut r = Resizable::new(20, 10);
        r.state.focused = true;

        let handled = r.handle_key(&Key::Left);
        assert!(handled);
        assert_eq!(r.size(), (19, 10));
    }

    #[test]
    fn test_resizable_handle_key_down() {
        let mut r = Resizable::new(20, 10);
        r.state.focused = true;

        let handled = r.handle_key(&Key::Down);
        assert!(handled);
        assert_eq!(r.size(), (20, 11));
    }

    #[test]
    fn test_resizable_handle_key_up() {
        let mut r = Resizable::new(20, 10);
        r.state.focused = true;

        let handled = r.handle_key(&Key::Up);
        assert!(handled);
        assert_eq!(r.size(), (20, 9));
    }

    #[test]
    fn test_resizable_handle_key_unhandled() {
        let mut r = Resizable::new(20, 10);
        r.state.focused = true;

        let handled = r.handle_key(&Key::Enter);
        assert!(!handled);
    }

    #[test]
    fn test_resizable_handle_key_without_handle() {
        let mut r = Resizable::new(20, 10).handles(ResizeHandle::CORNERS);
        r.state.focused = true;

        // Right handle not in CORNERS
        let handled = r.handle_key(&Key::Right);
        assert!(!handled);
        assert_eq!(r.size(), (20, 10));
    }

    // ==================== handle_at Tests ====================

    #[test]
    fn test_resizable_handle_at() {
        let r = Resizable::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);

        // Test corner detection
        assert_eq!(r.handle_at(0, 0, area), Some(ResizeHandle::TopLeft));
        assert_eq!(r.handle_at(19, 0, area), Some(ResizeHandle::TopRight));
        assert_eq!(r.handle_at(0, 9, area), Some(ResizeHandle::BottomLeft));
        assert_eq!(r.handle_at(19, 9, area), Some(ResizeHandle::BottomRight));

        // Test center (no handle)
        assert_eq!(r.handle_at(10, 5, area), None);
    }

    #[test]
    fn test_resizable_set_hovered() {
        let mut r = Resizable::new(20, 10);
        assert_eq!(r.hovered_handle, None);

        r.set_hovered(Some(ResizeHandle::TopLeft));
        assert_eq!(r.hovered_handle, Some(ResizeHandle::TopLeft));

        r.set_hovered(None);
        assert_eq!(r.hovered_handle, None);
    }

    // ==================== Edge Cases ====================

    #[test]
    fn test_resizable_min_size_enforced() {
        let r = Resizable::new(0, 0);
        // Minimum is 1x1 even when created with 0x0
        assert_eq!(r.size(), (1, 1));
    }

    #[test]
    fn test_resizable_min_constraint_enforced() {
        let mut r = Resizable::new(20, 10).min_size(10, 5);
        r.set_size(1, 1);
        assert_eq!(r.size(), (10, 5));
    }

    #[test]
    fn test_resizable_max_only() {
        let mut r = Resizable::new(20, 10).max_size(30, 0);
        // max_height = 0 means unlimited
        r.set_size(40, 100);
        assert_eq!(r.size(), (30, 100));
    }

    #[test]
    fn test_resizable_start_resize_invalid_handle() {
        let mut r = Resizable::new(20, 10).handles(ResizeHandle::CORNERS);
        r.start_resize(ResizeHandle::Top); // Top is not in CORNERS
        assert!(!r.is_resizing());
    }

    #[test]
    fn test_resizable_apply_delta_not_resizing() {
        let mut r = Resizable::new(20, 10);
        // Not in resizing state
        r.apply_delta(10, 10);
        // Size should not change
        assert_eq!(r.size(), (20, 10));
    }

    #[test]
    fn test_resizable_apply_delta_negative() {
        let mut r = Resizable::new(20, 10);
        r.start_resize(ResizeHandle::Left);
        r.apply_delta(-5, 0);
        // Left direction means -1, so delta -5 * -1 = +5
        assert_eq!(r.size(), (25, 10));
    }

    #[test]
    fn test_content_area_non_border_style() {
        let r = Resizable::new(20, 10).style(ResizeStyle::Dots);
        let area = Rect::new(5, 5, 20, 10);
        let content = r.content_area(area);

        // No border padding for Dots style
        assert_eq!(content.x, 5);
        assert_eq!(content.y, 5);
        assert_eq!(content.width, 20);
        assert_eq!(content.height, 10);
    }

    // ==================== Rendering Tests ====================

    #[test]
    fn test_resizable_render_border() {
        let r = Resizable::new(10, 5).style(ResizeStyle::Border);
        let mut buffer = Buffer::new(20, 10);
        let rect = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, rect);

        r.render(&mut ctx);

        // Check border characters
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
        assert_eq!(buffer.get(9, 0).unwrap().symbol, '┐');
        assert_eq!(buffer.get(0, 4).unwrap().symbol, '└');
        assert_eq!(buffer.get(9, 4).unwrap().symbol, '┘');
    }

    #[test]
    fn test_resizable_render_dots() {
        let r = Resizable::new(10, 5).style(ResizeStyle::Dots);
        let mut buffer = Buffer::new(20, 10);
        let rect = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, rect);

        r.render(&mut ctx);

        // Check corner dots
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '●');
        assert_eq!(buffer.get(9, 0).unwrap().symbol, '●');
        assert_eq!(buffer.get(0, 4).unwrap().symbol, '●');
        assert_eq!(buffer.get(9, 4).unwrap().symbol, '●');
    }

    #[test]
    fn test_resizable_render_hidden() {
        let r = Resizable::new(10, 5).style(ResizeStyle::Hidden);
        let mut buffer = Buffer::new(20, 10);
        let rect = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, rect);

        r.render(&mut ctx);

        // Hidden style should not draw anything
        assert_eq!(buffer.get(0, 0).unwrap().symbol, ' ');
    }

    #[test]
    fn test_resizable_render_subtle_not_hovered() {
        let r = Resizable::new(10, 5).style(ResizeStyle::Subtle);
        let mut buffer = Buffer::new(20, 10);
        let rect = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, rect);

        r.render(&mut ctx);

        // Subtle style without hover should not draw border
        assert_eq!(buffer.get(0, 0).unwrap().symbol, ' ');
    }

    #[test]
    fn test_resizable_render_subtle_hovered() {
        let mut r = Resizable::new(10, 5).style(ResizeStyle::Subtle);
        r.set_hovered(Some(ResizeHandle::TopLeft));

        let mut buffer = Buffer::new(20, 10);
        let rect = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, rect);

        r.render(&mut ctx);

        // Subtle style with hover should draw border
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
    }

    #[test]
    fn test_resizable_render_while_resizing() {
        let mut r = Resizable::new(10, 5).style(ResizeStyle::Border);
        r.start_resize(ResizeHandle::BottomRight);

        let mut buffer = Buffer::new(20, 10);
        let rect = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, rect);

        r.render(&mut ctx);

        // Should still render border with active color
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
    }

    #[test]
    fn test_resizable_render_with_hovered_corner() {
        let mut r = Resizable::new(10, 5).style(ResizeStyle::Border);
        r.set_hovered(Some(ResizeHandle::TopRight));

        let mut buffer = Buffer::new(20, 10);
        let rect = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, rect);

        r.render(&mut ctx);

        // Border should be rendered with highlighted corner
        assert_eq!(buffer.get(9, 0).unwrap().symbol, '┐');
    }

    #[test]
    fn test_resizable_render_hovered_bottom_corners() {
        let mut r = Resizable::new(10, 5).style(ResizeStyle::Border);
        r.set_hovered(Some(ResizeHandle::BottomLeft));

        let mut buffer = Buffer::new(20, 10);
        let rect = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, rect);

        r.render(&mut ctx);
        assert_eq!(buffer.get(0, 4).unwrap().symbol, '└');

        // Test BottomRight
        r.set_hovered(Some(ResizeHandle::BottomRight));
        let mut buffer2 = Buffer::new(20, 10);
        let mut ctx2 = RenderContext::new(&mut buffer2, rect);
        r.render(&mut ctx2);
        assert_eq!(buffer2.get(9, 4).unwrap().symbol, '┘');
    }

    // ==================== Helper Function Tests ====================

    #[test]
    fn test_resizable_helper_function() {
        let r = resizable(30, 15);
        assert_eq!(r.size(), (30, 15));
    }

    // ==================== Aspect Ratio Edge Cases ====================

    #[test]
    fn test_aspect_ratio_with_max_constraint() {
        let mut r = Resizable::new(20, 10)
            .preserve_aspect_ratio()
            .max_size(50, 20);

        r.set_size(60, 10);
        // Width clamped to 50, height adjusted for 2:1 ratio
        assert_eq!(r.width, 50);
        // Height should be 25 for 2:1, but clamped to max 20
        assert!(r.height <= 20);
    }

    #[test]
    fn test_grid_snap_rounds() {
        let mut r = Resizable::new(20, 10).snap_to_grid(10, 10);

        // 23 rounds to 20 (23 + 5 = 28, 28/10 = 2, 2*10 = 20)
        r.set_size(23, 14);
        assert_eq!(r.size(), (20, 10));

        // 27 rounds to 30
        r.set_size(27, 16);
        assert_eq!(r.size(), (30, 20));
    }
}
