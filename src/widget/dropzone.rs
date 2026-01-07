//! Drop zone widget for drag-and-drop targets
//!
//! A configurable drop target area that accepts dragged items.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::widget::DropZone;
//!
//! DropZone::new("Drop files here")
//!     .accepts(&["file", "text"])
//!     .on_drop(|data| {
//!         println!("Dropped: {:?}", data);
//!         true
//!     })
//! ```

use std::sync::atomic::{AtomicU64, Ordering};

use crate::event::drag::{DragData, DragId, DropTarget};
use crate::impl_view_meta;
use crate::layout::Rect;
use crate::style::Color;
use crate::widget::traits::{Draggable, RenderContext, View, WidgetProps, WidgetState};

/// Atomic counter for generating unique drop zone IDs
static DROPZONE_ID_COUNTER: AtomicU64 = AtomicU64::new(1);

/// Visual style for the drop zone
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DropZoneStyle {
    /// Solid border
    #[default]
    Solid,
    /// Dashed border
    Dashed,
    /// No border, just highlight
    Highlight,
    /// Minimal indicator
    Minimal,
}

/// Drop zone widget
pub struct DropZone<F = fn(DragData) -> bool>
where
    F: FnMut(DragData) -> bool,
{
    /// Unique identifier
    id: DragId,
    /// Placeholder text when empty
    placeholder: String,
    /// Accepted data types
    accepts: Vec<&'static str>,
    /// Visual style
    style: DropZoneStyle,
    /// Drop handler
    on_drop: Option<F>,
    /// Is currently hovered by a drag
    hovered: bool,
    /// Can accept current drag
    can_accept_current: bool,
    /// Normal border color
    border_color: Color,
    /// Hover border color
    hover_color: Color,
    /// Accept indicator color
    accept_color: Color,
    /// Reject indicator color
    reject_color: Color,
    /// Widget state
    state: WidgetState,
    /// Widget props for CSS
    props: WidgetProps,
    /// Minimum height
    min_height: u16,
}

impl DropZone<fn(DragData) -> bool> {
    /// Create a new drop zone with placeholder text
    pub fn new(placeholder: impl Into<String>) -> Self {
        let id = DROPZONE_ID_COUNTER.fetch_add(1, Ordering::Relaxed);

        Self {
            id,
            placeholder: placeholder.into(),
            accepts: Vec::new(),
            style: DropZoneStyle::default(),
            on_drop: None,
            hovered: false,
            can_accept_current: false,
            border_color: Color::rgb(100, 100, 100),
            hover_color: Color::rgb(100, 150, 255),
            accept_color: Color::rgb(100, 200, 100),
            reject_color: Color::rgb(200, 100, 100),
            state: WidgetState::new(),
            props: WidgetProps::new(),
            min_height: 3,
        }
    }
}

impl<F> DropZone<F>
where
    F: FnMut(DragData) -> bool,
{
    /// Set accepted data types
    pub fn accepts(mut self, types: &[&'static str]) -> Self {
        self.accepts = types.to_vec();
        self
    }

    /// Accept all data types
    pub fn accepts_all(mut self) -> Self {
        self.accepts.clear();
        self
    }

    /// Set visual style
    pub fn style(mut self, style: DropZoneStyle) -> Self {
        self.style = style;
        self
    }

    /// Set border color
    pub fn border_color(mut self, color: Color) -> Self {
        self.border_color = color;
        self
    }

    /// Set hover color
    pub fn hover_color(mut self, color: Color) -> Self {
        self.hover_color = color;
        self
    }

    /// Set minimum height
    pub fn min_height(mut self, height: u16) -> Self {
        self.min_height = height;
        self
    }

    /// Set drop handler
    pub fn on_drop<G>(self, handler: G) -> DropZone<G>
    where
        G: FnMut(DragData) -> bool,
    {
        DropZone {
            id: self.id,
            placeholder: self.placeholder,
            accepts: self.accepts,
            style: self.style,
            on_drop: Some(handler),
            hovered: self.hovered,
            can_accept_current: self.can_accept_current,
            border_color: self.border_color,
            hover_color: self.hover_color,
            accept_color: self.accept_color,
            reject_color: self.reject_color,
            state: self.state,
            props: self.props,
            min_height: self.min_height,
        }
    }

    /// Set hovered state (called by drag system)
    pub fn set_hovered(&mut self, hovered: bool, can_accept: bool) {
        self.hovered = hovered;
        self.can_accept_current = can_accept;
    }

    /// Get unique ID
    pub fn id(&self) -> DragId {
        self.id
    }

    /// Create a DropTarget for registration
    pub fn as_target(&self, bounds: Rect) -> DropTarget {
        DropTarget::new(self.id, bounds).accepts(&self.accepts)
    }

    /// Get current border color based on state
    fn current_border_color(&self) -> Color {
        if self.hovered {
            if self.can_accept_current {
                self.accept_color
            } else {
                self.reject_color
            }
        } else {
            self.border_color
        }
    }

    /// Get border characters based on style
    fn border_chars(&self) -> (char, char, char, char, char, char) {
        match self.style {
            DropZoneStyle::Solid => ('┌', '┐', '└', '┘', '─', '│'),
            DropZoneStyle::Dashed => ('┌', '┐', '└', '┘', '╌', '╎'),
            DropZoneStyle::Highlight | DropZoneStyle::Minimal => (' ', ' ', ' ', ' ', ' ', ' '),
        }
    }
}

impl<F> View for DropZone<F>
where
    F: FnMut(DragData) -> bool,
{
    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let height = area.height.max(self.min_height);
        let color = self.current_border_color();

        match self.style {
            DropZoneStyle::Solid | DropZoneStyle::Dashed => {
                let (tl, tr, bl, br, h, v) = self.border_chars();

                // Top border
                if let Some(cell) = ctx.buffer.get_mut(area.x, area.y) {
                    cell.symbol = tl;
                    cell.fg = Some(color);
                }
                for x in (area.x + 1)..(area.x + area.width.saturating_sub(1)) {
                    if let Some(cell) = ctx.buffer.get_mut(x, area.y) {
                        cell.symbol = h;
                        cell.fg = Some(color);
                    }
                }
                if area.width > 1 {
                    if let Some(cell) = ctx.buffer.get_mut(area.x + area.width - 1, area.y) {
                        cell.symbol = tr;
                        cell.fg = Some(color);
                    }
                }

                // Bottom border
                let bottom_y = area.y + height.saturating_sub(1);
                if let Some(cell) = ctx.buffer.get_mut(area.x, bottom_y) {
                    cell.symbol = bl;
                    cell.fg = Some(color);
                }
                for x in (area.x + 1)..(area.x + area.width.saturating_sub(1)) {
                    if let Some(cell) = ctx.buffer.get_mut(x, bottom_y) {
                        cell.symbol = h;
                        cell.fg = Some(color);
                    }
                }
                if area.width > 1 {
                    if let Some(cell) = ctx.buffer.get_mut(area.x + area.width - 1, bottom_y) {
                        cell.symbol = br;
                        cell.fg = Some(color);
                    }
                }

                // Side borders
                for y in (area.y + 1)..bottom_y {
                    if let Some(cell) = ctx.buffer.get_mut(area.x, y) {
                        cell.symbol = v;
                        cell.fg = Some(color);
                    }
                    if area.width > 1 {
                        if let Some(cell) = ctx.buffer.get_mut(area.x + area.width - 1, y) {
                            cell.symbol = v;
                            cell.fg = Some(color);
                        }
                    }
                }
            }
            DropZoneStyle::Highlight => {
                // Fill with background color when hovered
                if self.hovered {
                    let bg = if self.can_accept_current {
                        Color::rgb(30, 60, 30)
                    } else {
                        Color::rgb(60, 30, 30)
                    };
                    for y in area.y..(area.y + height) {
                        for x in area.x..(area.x + area.width) {
                            if let Some(cell) = ctx.buffer.get_mut(x, y) {
                                cell.bg = Some(bg);
                            }
                        }
                    }
                }
            }
            DropZoneStyle::Minimal => {
                // Just show indicator on left edge
                let indicator = if self.hovered {
                    if self.can_accept_current {
                        '▶'
                    } else {
                        '✗'
                    }
                } else {
                    '│'
                };
                for y in area.y..(area.y + height) {
                    if let Some(cell) = ctx.buffer.get_mut(area.x, y) {
                        cell.symbol = indicator;
                        cell.fg = Some(color);
                    }
                }
            }
        }

        // Placeholder text
        let text_y = area.y + height / 2;
        let text_x = area.x + 2;
        let max_len = area.width.saturating_sub(4) as usize;

        let display_text = if self.hovered {
            if self.can_accept_current {
                "Drop here!"
            } else {
                "Cannot drop here"
            }
        } else {
            &self.placeholder
        };

        let text_color = if self.hovered {
            color
        } else {
            Color::rgb(150, 150, 150)
        };

        for (i, ch) in display_text.chars().take(max_len).enumerate() {
            if let Some(cell) = ctx.buffer.get_mut(text_x + i as u16, text_y) {
                cell.symbol = ch;
                cell.fg = Some(text_color);
            }
        }
    }

    impl_view_meta!("DropZone");
}

impl<F> Draggable for DropZone<F>
where
    F: FnMut(DragData) -> bool,
{
    fn can_drop(&self) -> bool {
        true
    }

    fn accepted_types(&self) -> &[&'static str] {
        &self.accepts
    }

    fn on_drag_enter(&mut self, data: &DragData) {
        self.hovered = true;
        self.can_accept_current = self.can_accept(data);
    }

    fn on_drag_leave(&mut self) {
        self.hovered = false;
        self.can_accept_current = false;
    }

    fn on_drop(&mut self, data: DragData) -> bool {
        self.hovered = false;
        self.can_accept_current = false;

        if let Some(ref mut handler) = self.on_drop {
            handler(data)
        } else {
            false
        }
    }

    fn drop_bounds(&self, area: Rect) -> Rect {
        area
    }
}

/// Create a drop zone with default settings
pub fn drop_zone(placeholder: impl Into<String>) -> DropZone<fn(DragData) -> bool> {
    DropZone::new(placeholder)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dropzone_new() {
        let zone = DropZone::new("Drop here");
        assert_eq!(zone.placeholder, "Drop here");
        assert!(zone.accepts.is_empty());
    }

    #[test]
    fn test_dropzone_accepts() {
        let zone = DropZone::new("Drop files").accepts(&["file", "text"]);

        assert_eq!(zone.accepts.len(), 2);
        assert!(zone.accepts.contains(&"file"));
        assert!(zone.accepts.contains(&"text"));
    }

    #[test]
    fn test_dropzone_style() {
        let zone = DropZone::new("Test").style(DropZoneStyle::Dashed);

        assert_eq!(zone.style, DropZoneStyle::Dashed);
    }

    #[test]
    fn test_dropzone_as_target() {
        let zone = DropZone::new("Test").accepts(&["text"]);

        let bounds = Rect::new(10, 5, 20, 10);
        let target = zone.as_target(bounds);

        assert_eq!(target.bounds, bounds);
        assert!(target.accepts.contains(&"text"));
    }

    #[test]
    fn test_dropzone_hover_state() {
        let mut zone = DropZone::new("Test");

        assert!(!zone.hovered);
        zone.set_hovered(true, true);
        assert!(zone.hovered);
        assert!(zone.can_accept_current);

        zone.set_hovered(false, false);
        assert!(!zone.hovered);
    }

    #[test]
    fn test_dropzone_draggable_trait() {
        let zone = DropZone::new("Test").accepts(&["file"]);

        assert!(zone.can_drop());
        assert_eq!(zone.accepted_types(), &["file"]);
    }

    #[test]
    fn test_dropzone_accepts_all() {
        let zone = DropZone::new("Drop anything")
            .accepts(&["file"])
            .accepts_all();

        assert!(zone.accepts.is_empty());
    }

    #[test]
    fn test_dropzone_colors() {
        let zone = DropZone::new("Zone")
            .border_color(Color::RED)
            .hover_color(Color::BLUE);

        assert_eq!(zone.border_color, Color::RED);
        assert_eq!(zone.hover_color, Color::BLUE);
    }

    #[test]
    fn test_dropzone_min_height() {
        let zone = DropZone::new("Zone").min_height(5);
        assert_eq!(zone.min_height, 5);
    }

    #[test]
    fn test_dropzone_on_drop_handler() {
        use std::cell::Cell;
        use std::rc::Rc;

        let called = Rc::new(Cell::new(false));
        let called_clone = called.clone();

        let mut zone = DropZone::new("Zone").on_drop(move |_data| {
            called_clone.set(true);
            true
        });

        let data = DragData::text("test");
        let result = Draggable::on_drop(&mut zone, data);

        assert!(result);
        assert!(called.get());
    }

    #[test]
    fn test_dropzone_id() {
        let zone1 = DropZone::new("Zone 1");
        let zone2 = DropZone::new("Zone 2");

        // IDs should be unique
        assert_ne!(zone1.id(), zone2.id());
    }

    #[test]
    fn test_dropzone_current_border_color() {
        let mut zone = DropZone::new("Zone");

        // Not hovered - use border_color
        assert_eq!(zone.current_border_color(), zone.border_color);

        // Hovered and can accept - use accept_color
        zone.set_hovered(true, true);
        assert_eq!(zone.current_border_color(), zone.accept_color);

        // Hovered but cannot accept - use reject_color
        zone.set_hovered(true, false);
        assert_eq!(zone.current_border_color(), zone.reject_color);
    }

    #[test]
    fn test_dropzone_border_chars() {
        let solid = DropZone::new("Zone").style(DropZoneStyle::Solid);
        let chars = solid.border_chars();
        assert_eq!(chars.0, '┌');
        assert_eq!(chars.4, '─');

        let dashed = DropZone::new("Zone").style(DropZoneStyle::Dashed);
        let chars = dashed.border_chars();
        assert_eq!(chars.4, '╌');

        let highlight = DropZone::new("Zone").style(DropZoneStyle::Highlight);
        let chars = highlight.border_chars();
        assert_eq!(chars.0, ' ');
    }

    #[test]
    fn test_dropzone_drag_enter_leave() {
        let mut zone = DropZone::new("Zone").accepts(&["file"]);

        let data = DragData::text("test");
        zone.on_drag_enter(&data);
        assert!(zone.hovered);

        zone.on_drag_leave();
        assert!(!zone.hovered);
        assert!(!zone.can_accept_current);
    }

    #[test]
    fn test_dropzone_drop_bounds() {
        let zone = DropZone::new("Zone");
        let area = Rect::new(5, 5, 20, 10);
        let bounds = zone.drop_bounds(area);

        assert_eq!(bounds, area);
    }

    #[test]
    fn test_dropzone_render() {
        use crate::render::Buffer;

        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let zone = DropZone::new("Drop files here");
        zone.render(&mut ctx);
    }

    #[test]
    fn test_dropzone_render_hovered() {
        use crate::render::Buffer;

        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut zone = DropZone::new("Drop files here");
        zone.set_hovered(true, true);
        zone.render(&mut ctx);
    }

    #[test]
    fn test_dropzone_render_rejected() {
        use crate::render::Buffer;

        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut zone = DropZone::new("Drop files here");
        zone.set_hovered(true, false);
        zone.render(&mut ctx);
    }

    #[test]
    fn test_dropzone_render_styles() {
        use crate::render::Buffer;

        for style in [
            DropZoneStyle::Solid,
            DropZoneStyle::Dashed,
            DropZoneStyle::Highlight,
            DropZoneStyle::Minimal,
        ] {
            let mut buffer = Buffer::new(40, 10);
            let area = Rect::new(0, 0, 40, 10);
            let mut ctx = RenderContext::new(&mut buffer, area);

            let zone = DropZone::new("Zone").style(style);
            zone.render(&mut ctx);
        }
    }

    #[test]
    fn test_dropzone_render_small_area() {
        use crate::render::Buffer;

        let mut buffer = Buffer::new(5, 2);
        let area = Rect::new(0, 0, 5, 2);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let zone = DropZone::new("This is a long placeholder text");
        zone.render(&mut ctx);
    }

    #[test]
    fn test_dropzone_helper() {
        let zone = drop_zone("Drop here");
        assert_eq!(zone.placeholder, "Drop here");
    }

    #[test]
    fn test_dropzone_style_default() {
        assert_eq!(DropZoneStyle::default(), DropZoneStyle::Solid);
    }

    #[test]
    fn test_dropzone_on_drop_no_handler() {
        let mut zone = DropZone::new("Zone");
        let data = DragData::text("test");

        let result = Draggable::on_drop(&mut zone, data);
        assert!(!result); // No handler, returns false
    }

    #[test]
    fn test_dropzone_all_styles() {
        let styles = [
            DropZoneStyle::Solid,
            DropZoneStyle::Dashed,
            DropZoneStyle::Highlight,
            DropZoneStyle::Minimal,
        ];

        for style in styles {
            let zone = DropZone::new("Zone").style(style);
            assert_eq!(zone.style, style);
        }
    }
}
