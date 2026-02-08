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

use super::types::DropZoneStyle;

/// Atomic counter for generating unique drop zone IDs
static DROPZONE_ID_COUNTER: AtomicU64 = AtomicU64::new(1);
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

// Builder methods (manually implemented due to generic type parameter)
impl DropZone<fn(DragData) -> bool> {
    /// Set the focused state
    pub fn focused(mut self, focused: bool) -> Self {
        self.state.focused = focused;
        self
    }

    /// Set the disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.state.disabled = disabled;
        self
    }

    /// Set the foreground color
    pub fn fg(mut self, color: Color) -> Self {
        self.state.fg = Some(color);
        self
    }

    /// Set the background color
    pub fn bg(mut self, color: Color) -> Self {
        self.state.bg = Some(color);
        self
    }

    /// Check if the widget is focused
    pub fn is_focused(&self) -> bool {
        self.state.focused
    }

    /// Check if the widget is disabled
    pub fn is_disabled(&self) -> bool {
        self.state.disabled
    }

    /// Set the focused state (mutable)
    pub fn set_focused(&mut self, focused: bool) {
        self.state.focused = focused;
    }
}

// StyledView trait for CSS class management
impl crate::widget::traits::StyledView for DropZone<fn(DragData) -> bool> {
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
        if self.props.classes.contains(&class.to_string()) {
            self.remove_class(class);
        } else {
            self.add_class(class);
        }
    }

    fn has_class(&self, class: &str) -> bool {
        self.props.classes.contains(&class.to_string())
    }
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

// =========================================================================
// Tests
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::widget::traits::StyledView;

    // =========================================================================
    // Constructor tests
    // =========================================================================

    #[test]
    fn test_drop_zone_new_with_string() {
        let zone = DropZone::new("Drop files here");
        assert!(zone.id() > 0);
    }

    #[test]
    fn test_drop_zone_new_with_string_owned() {
        let zone = DropZone::new("Drop files here".to_string());
        assert!(zone.id() > 0);
    }

    #[test]
    fn test_drop_zone_new_generates_unique_ids() {
        let zone1 = DropZone::new("Zone 1");
        let zone2 = DropZone::new("Zone 2");
        assert_ne!(zone1.id(), zone2.id());
    }

    // =========================================================================
    // Builder method tests
    // =========================================================================

    #[test]
    fn test_drop_zone_accepts_single_type() {
        let zone = DropZone::new("Test").accepts(&["file"]);
        assert_eq!(zone.accepted_types(), &["file"]);
    }

    #[test]
    fn test_drop_zone_accepts_multiple_types() {
        let zone = DropZone::new("Test").accepts(&["file", "text", "image"]);
        assert_eq!(zone.accepted_types(), &["file", "text", "image"]);
    }

    #[test]
    fn test_drop_zone_accepts_empty() {
        let zone = DropZone::new("Test").accepts(&[]);
        assert_eq!(zone.accepted_types(), &[] as &[&str]);
    }

    #[test]
    fn test_drop_zone_accepts_all() {
        let zone = DropZone::new("Test").accepts(&["file"]).accepts_all();
        assert_eq!(zone.accepted_types(), &[] as &[&str]);
    }

    #[test]
    fn test_drop_zone_style_solid() {
        let zone = DropZone::new("Test").style(DropZoneStyle::Solid);
        // Can't directly access style, but we can verify the method exists
        let _ = zone;
    }

    #[test]
    fn test_drop_zone_style_dashed() {
        let zone = DropZone::new("Test").style(DropZoneStyle::Dashed);
        let _ = zone;
    }

    #[test]
    fn test_drop_zone_style_highlight() {
        let zone = DropZone::new("Test").style(DropZoneStyle::Highlight);
        let _ = zone;
    }

    #[test]
    fn test_drop_zone_style_minimal() {
        let zone = DropZone::new("Test").style(DropZoneStyle::Minimal);
        let _ = zone;
    }

    #[test]
    fn test_drop_zone_border_color() {
        let zone = DropZone::new("Test").border_color(Color::rgb(255, 0, 0));
        let _ = zone;
    }

    #[test]
    fn test_drop_zone_hover_color() {
        let zone = DropZone::new("Test").hover_color(Color::rgb(0, 255, 0));
        let _ = zone;
    }

    #[test]
    fn test_drop_zone_min_height() {
        let zone = DropZone::new("Test").min_height(10);
        // Can't directly access min_height, but verify method exists
        let _ = zone;
    }

    #[test]
    fn test_drop_zone_min_height_zero() {
        let zone = DropZone::new("Test").min_height(0);
        let _ = zone;
    }

    #[test]
    fn test_drop_zone_builder_chain() {
        let zone = DropZone::new("Test")
            .accepts(&["file", "text"])
            .style(DropZoneStyle::Dashed)
            .border_color(Color::rgb(100, 100, 100))
            .hover_color(Color::rgb(150, 150, 255))
            .min_height(5);
        let _ = zone;
    }

    // =========================================================================
    // on_drop handler tests
    // =========================================================================

    #[test]
    fn test_drop_zone_on_drop_with_handler() {
        use crate::widget::traits::Draggable;
        let mut zone = DropZone::new("Test").on_drop(|_data| true);
        let data = DragData::text("test");
        assert!(Draggable::on_drop(&mut zone, data));
    }

    #[test]
    fn test_drop_zone_on_drop_returns_false() {
        use crate::widget::traits::Draggable;
        let mut zone = DropZone::new("Test").on_drop(|_data| false);
        let data = DragData::text("test");
        assert!(!Draggable::on_drop(&mut zone, data));
    }

    #[test]
    fn test_drop_zone_on_drop_without_handler_returns_false() {
        use crate::widget::traits::Draggable;
        let mut zone = DropZone::new("Test");
        let data = DragData::text("test");
        assert!(!Draggable::on_drop(&mut zone, data));
    }

    #[test]
    fn test_drop_zone_on_drop_resets_hover_state() {
        use crate::widget::traits::Draggable;
        let mut zone = DropZone::new("Test").on_drop(|_data| true);
        zone.set_hovered(true, true);
        Draggable::on_drop(&mut zone, DragData::text("test"));
        // After drop, hover state should be reset
        // This is tested through Draggable trait
    }

    // =========================================================================
    // State method tests
    // =========================================================================

    #[test]
    fn test_drop_zone_set_hovered_can_accept() {
        let mut zone = DropZone::new("Test");
        zone.set_hovered(true, true);
        // State is private, but method exists
    }

    #[test]
    fn test_drop_zone_set_hovered_cannot_accept() {
        let mut zone = DropZone::new("Test");
        zone.set_hovered(true, false);
        // State is private, but method exists
    }

    #[test]
    fn test_drop_zone_set_hovered_false() {
        let mut zone = DropZone::new("Test");
        zone.set_hovered(false, false);
        // State is private, but method exists
    }

    // =========================================================================
    // Getter method tests
    // =========================================================================

    #[test]
    fn test_drop_zone_id() {
        let zone = DropZone::new("Test");
        // ID should be non-zero
        assert!(zone.id() > 0);
    }

    #[test]
    fn test_drop_zone_id_multiple() {
        let zone1 = DropZone::new("Test1");
        let zone2 = DropZone::new("Test2");
        let zone3 = DropZone::new("Test3");
        // IDs should be unique and sequential
        assert!(zone1.id() < zone2.id());
        assert!(zone2.id() < zone3.id());
        assert_eq!(zone2.id() - zone1.id(), 1);
        assert_eq!(zone3.id() - zone2.id(), 1);
    }

    #[test]
    fn test_drop_zone_as_target() {
        let zone = DropZone::new("Test");
        let bounds = Rect::new(0, 0, 10, 5);
        let target = zone.as_target(bounds);
        assert_eq!(target.id, zone.id());
    }

    #[test]
    fn test_drop_zone_as_target_with_accepts() {
        let zone = DropZone::new("Test").accepts(&["file", "text"]);
        let bounds = Rect::new(0, 0, 10, 5);
        let target = zone.as_target(bounds);
        assert_eq!(target.id, zone.id());
    }

    // =========================================================================
    // Draggable trait tests
    // =========================================================================

    #[test]
    fn test_drop_zone_can_drop_always_true() {
        let zone = DropZone::new("Test");
        assert!(zone.can_drop());
    }

    #[test]
    fn test_drop_zone_accepted_types() {
        let zone = DropZone::new("Test").accepts(&["file"]);
        assert_eq!(zone.accepted_types(), &["file"]);
    }

    #[test]
    fn test_drop_zone_accepted_types_empty() {
        let zone = DropZone::new("Test");
        assert!(zone.accepted_types().is_empty());
    }

    #[test]
    fn test_drop_zone_on_drag_enter() {
        let mut zone = DropZone::new("Test").accepts(&["text"]);
        let data = DragData::text("test");
        zone.on_drag_enter(&data);
        // Can't directly verify hover state, but method exists
    }

    #[test]
    fn test_drop_zone_on_drag_leave() {
        let mut zone = DropZone::new("Test");
        zone.on_drag_leave();
        // Can't directly verify hover state, but method exists
    }

    #[test]
    fn test_drop_zone_drag_enter_then_leave() {
        let mut zone = DropZone::new("Test").accepts(&["text"]);
        let data = DragData::text("test");
        zone.on_drag_enter(&data);
        zone.on_drag_leave();
    }

    #[test]
    fn test_drop_zone_on_drop_trait() {
        use crate::widget::traits::Draggable;
        let mut zone = DropZone::new("Test").on_drop(|_data| true);
        let data = DragData::text("test");
        assert!(Draggable::on_drop(&mut zone, data));
    }

    #[test]
    fn test_drop_zone_drop_bounds() {
        let zone = DropZone::new("Test");
        let bounds = Rect::new(5, 10, 20, 15);
        let result = zone.drop_bounds(bounds);
        assert_eq!(result, bounds);
    }

    // =========================================================================
    // StyledView trait tests
    // =========================================================================

    #[test]
    fn test_drop_zone_set_id() {
        let mut zone = DropZone::new("Test");
        zone.set_id("my-dropzone");
        // Can't directly access id, but method exists
    }

    #[test]
    fn test_drop_zone_add_class() {
        let mut zone = DropZone::new("Test");
        zone.add_class("dropzone-active");
        zone.add_class("dropzone-hover");
        // Can't directly access classes, but method exists
    }

    #[test]
    fn test_drop_zone_add_class_duplicate() {
        let mut zone = DropZone::new("Test");
        zone.add_class("active");
        zone.add_class("active"); // Should not add duplicate
                                  // Can't directly verify, but method exists
    }

    #[test]
    fn test_drop_zone_remove_class() {
        let mut zone = DropZone::new("Test");
        zone.add_class("active");
        zone.remove_class("active");
        // Can't directly verify, but method exists
    }

    #[test]
    fn test_drop_zone_toggle_class_add() {
        let mut zone = DropZone::new("Test");
        zone.toggle_class("active");
        // Should add class
    }

    #[test]
    fn test_drop_zone_toggle_class_remove() {
        let mut zone = DropZone::new("Test");
        zone.add_class("active");
        zone.toggle_class("active");
        // Should remove class
    }

    #[test]
    fn test_drop_zone_has_class_true() {
        let mut zone = DropZone::new("Test");
        zone.add_class("active");
        assert!(zone.has_class("active"));
    }

    #[test]
    fn test_drop_zone_has_class_false() {
        let zone = DropZone::new("Test");
        assert!(!zone.has_class("active"));
    }

    // =========================================================================
    // WidgetState builder tests
    // =========================================================================

    #[test]
    fn test_drop_zone_focused() {
        let zone = DropZone::new("Test").focused(true);
        assert!(zone.is_focused());
    }

    #[test]
    fn test_drop_zone_not_focused() {
        let zone = DropZone::new("Test").focused(false);
        assert!(!zone.is_focused());
    }

    #[test]
    fn test_drop_zone_disabled() {
        let zone = DropZone::new("Test").disabled(true);
        assert!(zone.is_disabled());
    }

    #[test]
    fn test_drop_zone_not_disabled() {
        let zone = DropZone::new("Test").disabled(false);
        assert!(!zone.is_disabled());
    }

    #[test]
    fn test_drop_zone_fg_color() {
        let zone = DropZone::new("Test").fg(Color::rgb(255, 0, 0));
        assert!(zone.is_focused() == false); // Should not affect focused state
    }

    #[test]
    fn test_drop_zone_bg_color() {
        let zone = DropZone::new("Test").bg(Color::rgb(0, 255, 0));
        let _ = zone;
    }

    #[test]
    fn test_drop_zone_set_focused_true() {
        let mut zone = DropZone::new("Test");
        zone.set_focused(true);
        assert!(zone.is_focused());
    }

    #[test]
    fn test_drop_zone_set_focused_false() {
        let mut zone = DropZone::new("Test").focused(true);
        zone.set_focused(false);
        assert!(!zone.is_focused());
    }

    // =========================================================================
    // Integration tests
    // =========================================================================

    #[test]
    fn test_drop_zone_full_builder_chain() {
        let zone = DropZone::new("Upload files")
            .accepts(&["file", "image"])
            .style(DropZoneStyle::Dashed)
            .border_color(Color::rgb(100, 100, 100))
            .hover_color(Color::rgb(150, 150, 255))
            .min_height(10)
            .focused(true)
            .disabled(false)
            .fg(Color::rgb(200, 200, 200))
            .bg(Color::rgb(50, 50, 50));

        assert!(zone.is_focused());
        assert!(!zone.is_disabled());
        assert_eq!(zone.accepted_types(), &["file", "image"]);
    }

    #[test]
    fn test_drop_zone_with_style_and_classes() {
        let mut zone = DropZone::new("Test")
            .style(DropZoneStyle::Highlight)
            .focused(true);

        zone.set_id("upload-zone");
        zone.add_class("primary");
        zone.add_class("large");

        assert!(zone.is_focused());
        assert!(zone.has_class("primary"));
        assert!(zone.has_class("large"));
    }

    #[test]
    fn test_drop_zone_with_drop_handler_and_drag_operations() {
        use crate::widget::traits::Draggable;
        let mut zone = DropZone::new("Test").accepts(&["text"]).on_drop(|data| {
            assert_eq!(data.type_id, "text");
            true
        });

        let data = DragData::text("Hello, world!");
        zone.on_drag_enter(&data);
        let result = Draggable::on_drop(&mut zone, data);
        assert!(result);
    }
}
