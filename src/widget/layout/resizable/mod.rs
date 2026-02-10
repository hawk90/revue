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
