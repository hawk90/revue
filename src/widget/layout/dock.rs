//! Dock manager for IDE-style layouts
//!
//! Provides a flexible docking system for creating complex, resizable
//! multi-pane UIs similar to IDEs (VS Code, IntelliJ, etc.).

// Allow dead code for public API exports that aren't used yet
#![allow(dead_code)]
//!
//! # Example
//!
//! ```text
//! use revue::widget::layout::dock::{DockManager, DockArea, Panel};
//!
//! DockManager::new()
//!     .left(
//!         DockArea::new("explorer")
//!             .min_width(200)
//!             .panel(explorer_view)
//!     )
//!     .center(
//!         DockArea::new("editor")
//!             .tab("main.rs", editor1)
//!             .tab("lib.rs", editor2)
//!     )
//!     .right(
//!         DockArea::new("properties")
//!             .min_width(200)
//!             .collapsible()
//!             .panel(properties_view)
//!     )
//! ```;

use crate::widget::layout::splitter::Pane;
use crate::widget::layout::tabs::Tabs;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// Dock area position
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DockPosition {
    /// Left side
    Left,
    /// Right side
    Right,
    /// Top
    Top,
    /// Bottom
    Bottom,
    /// Center (main content area)
    Center,
}

/// A dockable area (panel with tabs)
pub struct DockArea {
    /// Area identifier
    id: String,
    /// Tabs in this area
    tabs: Vec<TabContent>,
    /// Active tab index
    active_tab: usize,
    /// Minimum size
    min_size: u16,
    /// Maximum size (0 = unlimited)
    max_size: u16,
    /// Initial size ratio (0.0 - 1.0)
    ratio: f32,
    /// Whether collapsible
    collapsible: bool,
    /// Whether collapsed
    collapsed: bool,
    /// Position
    position: DockPosition,
    /// Widget props
    props: WidgetProps,
}

/// Tab content (label + widget)
///
/// Stores a widget that can be rendered within a dock area tab.
pub struct TabContent {
    /// Tab label
    label: String,
    /// Widget to render
    widget: Option<Box<dyn View>>,
}

impl TabContent {
    /// Create a new tab content
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            widget: None,
        }
    }

    /// Set widget
    pub fn widget<W: View + 'static>(mut self, widget: W) -> Self {
        self.widget = Some(Box::new(widget));
        self
    }
}

impl DockArea {
    /// Create a new dock area
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            tabs: Vec::new(),
            active_tab: 0,
            min_size: 100,
            max_size: 0,
            ratio: 0.2,
            collapsible: false,
            collapsed: false,
            position: DockPosition::Left,
            props: WidgetProps::new(),
        }
    }

    /// Set position
    pub fn position(mut self, position: DockPosition) -> Self {
        self.position = position;
        self
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

    /// Set size ratio
    pub fn ratio(mut self, ratio: f32) -> Self {
        self.ratio = ratio.clamp(0.0, 1.0);
        self
    }

    /// Set collapsible
    pub fn collapsible(mut self) -> Self {
        self.collapsible = true;
        self
    }

    /// Set collapsed
    pub fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }

    /// Add a tab
    pub fn tab(mut self, label: impl Into<String>) -> Self {
        self.tabs.push(TabContent::new(label.into()));
        self
    }

    /// Add a tab with widget
    pub fn tab_with<W: View + 'static>(mut self, label: impl Into<String>, widget: W) -> Self {
        self.tabs.push(TabContent::new(label.into()).widget(widget));
        self
    }

    /// Add panel content (single widget, no tabs)
    pub fn panel<W: View + 'static>(mut self, widget: W) -> Self {
        let label = self.id.clone();
        self.tabs.push(TabContent::new(label).widget(widget));
        self
    }

    /// Convert to splitter pane
    fn to_pane(&self) -> Pane {
        let mut pane = Pane::new(&self.id)
            .min_size(self.min_size)
            .max_size(self.max_size)
            .ratio(self.ratio);

        if self.collapsible {
            pane = pane.collapsible();
        }
        pane.collapsed = self.collapsed;
        pane
    }
}

impl Clone for DockArea {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            tabs: Vec::new(), // Widgets can't be cloned, start with empty tabs
            active_tab: self.active_tab,
            min_size: self.min_size,
            max_size: self.max_size,
            ratio: self.ratio,
            collapsible: self.collapsible,
            collapsed: self.collapsed,
            position: self.position,
            props: self.props.clone(),
        }
    }
}

impl View for DockArea {
    fn render(&self, ctx: &mut RenderContext) {
        if self.collapsed {
            return;
        }

        let rect = ctx.area;

        // Draw tab headers if multiple tabs
        if self.tabs.len() > 1 {
            let tabs_labels: Vec<String> = self.tabs.iter().map(|t| t.label.clone()).collect();
            let tabs = Tabs::new()
                .tabs(tabs_labels)
                .fg(crate::style::Color::rgb(128, 128, 128))
                .bg(crate::style::Color::rgb(0, 0, 0));

            // Reserve 1 row for tabs
            if rect.height > 1 {
                let tab_rect = crate::layout::Rect::new(rect.x, rect.y, rect.width, 1);
                let mut tab_ctx = RenderContext::new(ctx.buffer, tab_rect);
                tabs.render(&mut tab_ctx);

                // Render active tab content below
                if let Some(active_tab) = self.tabs.get(self.active_tab) {
                    if let Some(widget) = &active_tab.widget {
                        let content_rect = crate::layout::Rect::new(
                            rect.x,
                            rect.y + 1,
                            rect.width,
                            rect.height.saturating_sub(1),
                        );
                        let mut content_ctx = RenderContext::new(ctx.buffer, content_rect);
                        widget.render(&mut content_ctx);
                    }
                }
            }
        } else if let Some(tab) = self.tabs.first() {
            // Single tab - just render content
            if let Some(widget) = &tab.widget {
                widget.render(ctx);
            }
        }
    }
}

impl_props_builders!(DockArea);
impl_styled_view!(DockArea);

/// Dock manager - orchestrates multiple dock areas
pub struct DockManager {
    /// Left dock area
    left: Option<DockArea>,
    /// Right dock area
    right: Option<DockArea>,
    /// Top dock area
    top: Option<DockArea>,
    /// Bottom dock area
    bottom: Option<DockArea>,
    /// Center dock area (main content)
    center: Option<DockArea>,
    /// Widget props
    props: WidgetProps,
}

impl DockManager {
    /// Create a new dock manager
    pub fn new() -> Self {
        Self {
            left: None,
            right: None,
            top: None,
            bottom: None,
            center: None,
            props: WidgetProps::new(),
        }
    }

    /// Set left dock area
    pub fn left(mut self, area: DockArea) -> Self {
        self.left = Some(area.position(DockPosition::Left));
        self
    }

    /// Set right dock area
    pub fn right(mut self, area: DockArea) -> Self {
        self.right = Some(area.position(DockPosition::Right));
        self
    }

    /// Set top dock area
    pub fn top(mut self, area: DockArea) -> Self {
        self.top = Some(area.position(DockPosition::Top));
        self
    }

    /// Set bottom dock area
    pub fn bottom(mut self, area: DockArea) -> Self {
        self.bottom = Some(area.position(DockPosition::Bottom));
        self
    }

    /// Set center dock area
    pub fn center(mut self, area: DockArea) -> Self {
        self.center = Some(area.position(DockPosition::Center));
        self
    }

    /// Calculate layout based on available areas
    fn calculate_layout(&self, rect: crate::layout::Rect) -> Vec<(DockArea, crate::layout::Rect)> {
        let mut layout = Vec::new();
        let mut current = rect;

        // Reserve top area
        if let Some(ref top) = self.top {
            if !top.collapsed {
                let top_height =
                    (current.height as f32 * top.ratio).max(top.min_size as f32) as u16;
                let top_rect = crate::layout::Rect::new(
                    current.x,
                    current.y,
                    current.width,
                    top_height.min(current.height),
                );
                layout.push(((*top).clone(), top_rect));
                current.y += top_height;
                current.height = current.height.saturating_sub(top_height);
            }
        }

        // Reserve bottom area
        if let Some(ref bottom) = self.bottom {
            if !bottom.collapsed {
                let bottom_height =
                    (current.height as f32 * bottom.ratio).max(bottom.min_size as f32) as u16;
                let bottom_rect = crate::layout::Rect::new(
                    current.x,
                    current.y
                        + current
                            .height
                            .saturating_sub(bottom_height.min(current.height)),
                    current.width,
                    bottom_height.min(current.height),
                );
                layout.push(((*bottom).clone(), bottom_rect));
                current.height = current
                    .height
                    .saturating_sub(bottom_height.min(current.height));
            }
        }

        // Reserve left area
        let mut middle = current;
        if let Some(ref left) = self.left {
            if !left.collapsed {
                let left_width =
                    (middle.width as f32 * left.ratio).max(left.min_size as f32) as u16;
                let left_rect = crate::layout::Rect::new(
                    middle.x,
                    middle.y,
                    left_width.min(middle.width),
                    middle.height,
                );
                layout.push(((*left).clone(), left_rect));
                middle.x += left_width;
                middle.width = middle.width.saturating_sub(left_width);
            }
        }

        // Reserve right area
        if let Some(ref right) = self.right {
            if !right.collapsed {
                let right_width =
                    (middle.width as f32 * right.ratio).max(right.min_size as f32) as u16;
                let right_rect = crate::layout::Rect::new(
                    middle.x + middle.width.saturating_sub(right_width.min(middle.width)),
                    middle.y,
                    right_width.min(middle.width),
                    middle.height,
                );
                layout.push(((*right).clone(), right_rect));
                middle.width = middle.width.saturating_sub(right_width);
            }
        }

        // Center area gets remaining space
        if let Some(ref center) = self.center {
            layout.push(((*center).clone(), middle));
        }

        layout
    }
}

impl Default for DockManager {
    fn default() -> Self {
        Self::new()
    }
}

impl View for DockManager {
    fn render(&self, ctx: &mut RenderContext) {
        let rect = ctx.area;
        let layout = self.calculate_layout(rect);

        for (area, area_rect) in layout {
            let mut area_ctx = RenderContext::new(ctx.buffer, area_rect);
            area.render(&mut area_ctx);
        }
    }
}

impl_props_builders!(DockManager);
impl_styled_view!(DockManager);

/// Create a new dock manager
pub fn dock() -> DockManager {
    DockManager::new()
}

/// Create a new dock area
pub fn dock_area(id: impl Into<String>) -> DockArea {
    DockArea::new(id)
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // DockPosition enum tests
    // =========================================================================

    #[test]
    fn test_dock_position_clone() {
        let pos = DockPosition::Right;
        assert_eq!(pos, pos.clone());
    }

    #[test]
    fn test_dock_position_copy() {
        let p1 = DockPosition::Left;
        let p2 = p1;
        assert_eq!(p1, DockPosition::Left);
        assert_eq!(p2, DockPosition::Left);
    }

    #[test]
    fn test_dock_position_debug() {
        let debug_str = format!("{:?}", DockPosition::Top);
        assert!(debug_str.contains("Top"));
    }

    #[test]
    fn test_dock_position_partial_eq() {
        assert_eq!(DockPosition::Left, DockPosition::Left);
        assert_eq!(DockPosition::Right, DockPosition::Right);
        assert_eq!(DockPosition::Top, DockPosition::Top);
        assert_eq!(DockPosition::Bottom, DockPosition::Bottom);
        assert_eq!(DockPosition::Center, DockPosition::Center);
        assert_ne!(DockPosition::Left, DockPosition::Right);
    }

    // =========================================================================
    // TabContent tests
    // =========================================================================

    #[test]
    fn test_tab_content_new() {
        let tab = TabContent::new("File 1");
        assert_eq!(tab.label, "File 1");
        assert!(tab.widget.is_none());
    }

    #[test]
    fn test_tab_content_new_string() {
        let tab = TabContent::new(String::from("Owned"));
        assert_eq!(tab.label, "Owned");
    }

    #[test]
    fn test_tab_content_widget() {
        // We can't test with actual widgets without importing more, but we can
        // verify the builder chain compiles
        let tab = TabContent::new("test");
        assert!(tab.widget.is_none());
    }

    // =========================================================================
    // DockArea builder tests
    // =========================================================================

    #[test]
    fn test_dock_area_creation() {
        let area = DockArea::new("test").min_size(100).ratio(0.3).collapsible();

        assert_eq!(area.id, "test");
        assert_eq!(area.min_size, 100);
        assert_eq!(area.ratio, 0.3);
        assert!(area.collapsible);
    }

    #[test]
    fn test_dock_area_new_defaults() {
        let area = DockArea::new("test");
        assert_eq!(area.id, "test");
        assert!(area.tabs.is_empty());
        assert_eq!(area.active_tab, 0);
        assert_eq!(area.min_size, 100);
        assert_eq!(area.max_size, 0);
        assert_eq!(area.ratio, 0.2);
        assert!(!area.collapsible);
        assert!(!area.collapsed);
        assert_eq!(area.position, DockPosition::Left);
    }

    #[test]
    fn test_dock_area_id_string() {
        let area = DockArea::new(String::from("owned"));
        assert_eq!(area.id, "owned");
    }

    #[test]
    fn test_dock_area_position() {
        let area = DockArea::new("test").position(DockPosition::Right);
        assert_eq!(area.position, DockPosition::Right);

        let area = DockArea::new("test").position(DockPosition::Top);
        assert_eq!(area.position, DockPosition::Top);

        let area = DockArea::new("test").position(DockPosition::Bottom);
        assert_eq!(area.position, DockPosition::Bottom);

        let area = DockArea::new("test").position(DockPosition::Center);
        assert_eq!(area.position, DockPosition::Center);
    }

    #[test]
    fn test_dock_area_min_size() {
        let area = DockArea::new("test").min_size(250);
        assert_eq!(area.min_size, 250);
    }

    #[test]
    fn test_dock_area_max_size() {
        let area = DockArea::new("test").max_size(500);
        assert_eq!(area.max_size, 500);
    }

    #[test]
    fn test_dock_area_ratio() {
        let area = DockArea::new("test").ratio(0.5);
        assert_eq!(area.ratio, 0.5);
    }

    #[test]
    fn test_dock_area_ratio_clamping() {
        let area = DockArea::new("test").ratio(1.5);
        assert_eq!(area.ratio, 1.0);

        let area = DockArea::new("test").ratio(-0.5);
        assert_eq!(area.ratio, 0.0);

        let area = DockArea::new("test").ratio(0.0);
        assert_eq!(area.ratio, 0.0);

        let area = DockArea::new("test").ratio(1.0);
        assert_eq!(area.ratio, 1.0);
    }

    #[test]
    fn test_dock_area_collapsible() {
        let area = DockArea::new("test").collapsible();
        assert!(area.collapsible);
    }

    #[test]
    fn test_dock_area_collapsed() {
        let area = DockArea::new("test").collapsed(true);
        assert!(area.collapsed);

        let area = DockArea::new("test").collapsed(false);
        assert!(!area.collapsed);
    }

    #[test]
    fn test_dock_area_with_tabs() {
        let area = DockArea::new("editor").tab("main.rs").tab("lib.rs");

        assert_eq!(area.tabs.len(), 2);
        assert_eq!(area.tabs[0].label, "main.rs");
        assert_eq!(area.tabs[1].label, "lib.rs");
    }

    #[test]
    fn test_dock_area_tab_string() {
        let area = DockArea::new("test").tab(String::from("file.rs"));
        assert_eq!(area.tabs.len(), 1);
        assert_eq!(area.tabs[0].label, "file.rs");
    }

    #[test]
    fn test_dock_area_builder_chain() {
        let area = DockArea::new("test")
            .position(DockPosition::Right)
            .min_size(150)
            .max_size(400)
            .ratio(0.25)
            .collapsible()
            .collapsed(false)
            .tab("tab1")
            .tab("tab2");

        assert_eq!(area.id, "test");
        assert_eq!(area.position, DockPosition::Right);
        assert_eq!(area.min_size, 150);
        assert_eq!(area.max_size, 400);
        assert_eq!(area.ratio, 0.25);
        assert!(area.collapsible);
        assert!(!area.collapsed);
        assert_eq!(area.tabs.len(), 2);
    }

    // =========================================================================
    // DockArea::to_pane tests
    // =========================================================================

    #[test]
    fn test_dock_area_to_pane() {
        let area = DockArea::new("test").min_size(100).max_size(500).ratio(0.3);

        let pane = area.to_pane();
        assert_eq!(pane.id, "test");
        assert_eq!(pane.min_size, 100);
        assert_eq!(pane.max_size, 500);
        assert_eq!(pane.ratio, 0.3);
        assert!(!pane.collapsible);
        assert!(!pane.collapsed);
    }

    #[test]
    fn test_dock_area_to_pane_collapsible() {
        let area = DockArea::new("test").min_size(100).collapsible();

        let pane = area.to_pane();
        assert!(pane.collapsible);
        assert!(!pane.collapsed);
    }

    #[test]
    fn test_dock_area_to_pane_collapsed() {
        let area = DockArea::new("test").collapsible().collapsed(true);

        let pane = area.to_pane();
        assert!(pane.collapsible);
        assert!(pane.collapsed);
    }

    // =========================================================================
    // DockArea Clone tests
    // =========================================================================

    #[test]
    fn test_dock_area_clone() {
        let area = DockArea::new("test")
            .position(DockPosition::Right)
            .min_size(150)
            .max_size(400)
            .ratio(0.25)
            .collapsible()
            .collapsed(true)
            .tab("tab1");

        let cloned = area.clone();

        assert_eq!(cloned.id, area.id);
        assert_eq!(cloned.position, area.position);
        assert_eq!(cloned.min_size, area.min_size);
        assert_eq!(cloned.max_size, area.max_size);
        assert_eq!(cloned.ratio, area.ratio);
        assert_eq!(cloned.collapsible, area.collapsible);
        assert_eq!(cloned.collapsed, area.collapsed);
        // tabs are not cloned (widgets can't be cloned)
        assert_eq!(cloned.tabs.len(), 0);
        assert_eq!(cloned.active_tab, area.active_tab);
    }

    // =========================================================================
    // DockManager builder tests
    // =========================================================================

    #[test]
    fn test_dock_manager_creation() {
        let manager = DockManager::new()
            .left(DockArea::new("explorer").min_size(200))
            .center(DockArea::new("editor"))
            .right(DockArea::new("properties").min_size(200).collapsible());

        assert!(manager.left.is_some());
        assert!(manager.center.is_some());
        assert!(manager.right.is_some());
    }

    #[test]
    fn test_dock_manager_new() {
        let manager = DockManager::new();
        assert!(manager.left.is_none());
        assert!(manager.right.is_none());
        assert!(manager.top.is_none());
        assert!(manager.bottom.is_none());
        assert!(manager.center.is_none());
    }

    #[test]
    fn test_dock_manager_left() {
        let manager = DockManager::new().left(DockArea::new("left"));
        assert!(manager.left.is_some());
        assert_eq!(manager.left.as_ref().unwrap().position, DockPosition::Left);
    }

    #[test]
    fn test_dock_manager_right() {
        let manager = DockManager::new().right(DockArea::new("right"));
        assert!(manager.right.is_some());
        assert_eq!(
            manager.right.as_ref().unwrap().position,
            DockPosition::Right
        );
    }

    #[test]
    fn test_dock_manager_top() {
        let manager = DockManager::new().top(DockArea::new("top"));
        assert!(manager.top.is_some());
        assert_eq!(manager.top.as_ref().unwrap().position, DockPosition::Top);
    }

    #[test]
    fn test_dock_manager_bottom() {
        let manager = DockManager::new().bottom(DockArea::new("bottom"));
        assert!(manager.bottom.is_some());
        assert_eq!(
            manager.bottom.as_ref().unwrap().position,
            DockPosition::Bottom
        );
    }

    #[test]
    fn test_dock_manager_center() {
        let manager = DockManager::new().center(DockArea::new("center"));
        assert!(manager.center.is_some());
        assert_eq!(
            manager.center.as_ref().unwrap().position,
            DockPosition::Center
        );
    }

    #[test]
    fn test_dock_manager_all_positions() {
        let manager = DockManager::new()
            .left(DockArea::new("left"))
            .right(DockArea::new("right"))
            .top(DockArea::new("top"))
            .bottom(DockArea::new("bottom"))
            .center(DockArea::new("center"));

        assert!(manager.left.is_some());
        assert!(manager.right.is_some());
        assert!(manager.top.is_some());
        assert!(manager.bottom.is_some());
        assert!(manager.center.is_some());
    }

    #[test]
    fn test_dock_manager_builder_chain() {
        let manager = DockManager::new()
            .left(DockArea::new("left").min_size(200))
            .center(DockArea::new("center"))
            .right(DockArea::new("right").min_size(200).collapsible())
            .top(DockArea::new("top").ratio(0.2))
            .bottom(DockArea::new("bottom").ratio(0.2));

        assert_eq!(manager.left.as_ref().unwrap().min_size, 200);
        assert!(manager.right.as_ref().unwrap().collapsible);
        assert_eq!(manager.top.as_ref().unwrap().ratio, 0.2);
        assert_eq!(manager.bottom.as_ref().unwrap().ratio, 0.2);
    }

    // =========================================================================
    // DockManager Default tests
    // =========================================================================

    #[test]
    fn test_dock_manager_default() {
        let manager = DockManager::default();
        assert!(manager.left.is_none());
        assert!(manager.right.is_none());
        assert!(manager.top.is_none());
        assert!(manager.bottom.is_none());
        assert!(manager.center.is_none());
    }

    // =========================================================================
    // Helper function tests
    // =========================================================================

    #[test]
    fn test_dock_helper() {
        let manager = dock();
        assert!(manager.left.is_none());
        assert!(manager.center.is_none());
    }

    #[test]
    fn test_dock_area_helper() {
        let area = dock_area("test");
        assert_eq!(area.id, "test");
    }

    #[test]
    fn test_dock_area_helper_string() {
        let area = dock_area(String::from("owned"));
        assert_eq!(area.id, "owned");
    }
}
