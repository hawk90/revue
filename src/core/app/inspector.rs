//! Widget Inspector for debugging UI hierarchy

use crate::layout::Rect;
use crate::render::Cell;
use crate::style::Color;
use crate::widget::theme::{DARK_GRAY, EDITOR_BG, MUTED_TEXT};
use crate::widget::{RenderContext, View};

/// Tree rendering state
struct TreeRenderState {
    x: u16,
    y: u16,
    depth: usize,
    index: usize,
    max_width: u16,
}

/// Widget info for inspection
#[derive(Clone, Debug)]
pub struct WidgetInfo {
    /// Widget type name
    pub type_name: String,
    /// Widget bounds
    pub bounds: Rect,
    /// Widget properties
    pub properties: Vec<(String, String)>,
    /// Child widgets
    pub children: Vec<WidgetInfo>,
    /// Is widget focused
    pub focused: bool,
    /// Is widget hovered
    pub hovered: bool,
}

impl WidgetInfo {
    /// Create new widget info
    pub fn new(type_name: impl Into<String>, bounds: Rect) -> Self {
        Self {
            type_name: type_name.into(),
            bounds,
            properties: Vec::new(),
            children: Vec::new(),
            focused: false,
            hovered: false,
        }
    }

    /// Add a property
    pub fn property(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.properties.push((key.into(), value.into()));
        self
    }

    /// Add a child widget
    pub fn child(mut self, child: WidgetInfo) -> Self {
        self.children.push(child);
        self
    }

    /// Set focused state
    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    /// Set hovered state
    pub fn hovered(mut self, hovered: bool) -> Self {
        self.hovered = hovered;
        self
    }

    /// Get total descendant count
    pub fn descendant_count(&self) -> usize {
        self.children.iter().map(|c| 1 + c.descendant_count()).sum()
    }
}

/// Widget Inspector overlay
#[derive(Clone)]
pub struct Inspector {
    /// Is inspector visible
    visible: bool,
    /// Selected widget index
    selected: usize,
    /// Root widget info
    root: Option<WidgetInfo>,
    /// Inspector panel width
    panel_width: u16,
    /// Show bounds overlay
    show_bounds: bool,
    /// Highlight color
    highlight_color: Color,
    /// Background color
    bg_color: Color,
}

impl Inspector {
    /// Create a new inspector
    pub fn new() -> Self {
        Self {
            visible: false,
            selected: 0,
            root: None,
            panel_width: 40,
            show_bounds: true,
            highlight_color: Color::CYAN,
            bg_color: EDITOR_BG,
        }
    }

    /// Show the inspector
    pub fn show(&mut self) {
        self.visible = true;
    }

    /// Hide the inspector
    pub fn hide(&mut self) {
        self.visible = false;
    }

    /// Toggle visibility
    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }

    /// Check if visible
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Set widget tree
    pub fn set_root(&mut self, root: WidgetInfo) {
        self.root = Some(root);
    }

    /// Clear widget tree
    pub fn clear(&mut self) {
        self.root = None;
        self.selected = 0;
    }

    /// Set panel width
    pub fn panel_width(mut self, width: u16) -> Self {
        self.panel_width = width;
        self
    }

    /// Enable/disable bounds overlay
    pub fn show_bounds(mut self, show: bool) -> Self {
        self.show_bounds = show;
        self
    }

    /// Set highlight color
    pub fn highlight_color(mut self, color: Color) -> Self {
        self.highlight_color = color;
        self
    }

    /// Select next widget
    pub fn select_next(&mut self) {
        if let Some(ref root) = self.root {
            let count = 1 + root.descendant_count();
            self.selected = (self.selected + 1) % count;
        }
    }

    /// Select previous widget
    pub fn select_prev(&mut self) {
        if let Some(ref root) = self.root {
            let count = 1 + root.descendant_count();
            self.selected = self.selected.checked_sub(1).unwrap_or(count - 1);
        }
    }

    /// Get selected widget info
    pub fn selected_widget(&self) -> Option<&WidgetInfo> {
        let root = self.root.as_ref()?;
        self.get_widget_at(root, self.selected)
    }

    /// Get widget at index (depth-first traversal)
    fn get_widget_at<'a>(
        &self,
        widget: &'a WidgetInfo,
        mut index: usize,
    ) -> Option<&'a WidgetInfo> {
        if index == 0 {
            return Some(widget);
        }
        index -= 1;

        for child in &widget.children {
            let count = 1 + child.descendant_count();
            if index < count {
                return self.get_widget_at(child, index);
            }
            index -= count;
        }

        None
    }

    /// Handle key input
    pub fn handle_key(&mut self, key: &crate::event::Key) -> bool {
        use crate::event::Key;

        if !self.visible {
            return false;
        }

        match key {
            Key::Up | Key::Char('k') => {
                self.select_prev();
                true
            }
            Key::Down | Key::Char('j') => {
                self.select_next();
                true
            }
            Key::Escape => {
                self.hide();
                true
            }
            Key::Char('b') => {
                self.show_bounds = !self.show_bounds;
                true
            }
            _ => false,
        }
    }

    /// Render the inspector panel
    pub fn render(&self, ctx: &mut RenderContext) {
        if !self.visible {
            return;
        }

        let area = ctx.area;
        let panel_x = area
            .x
            .saturating_add(area.width.saturating_sub(self.panel_width));
        let panel_width = self.panel_width.min(area.width);
        let bottom = area.y.saturating_add(area.height);

        // Draw background
        for y in area.y..bottom {
            for x in panel_x..panel_x.saturating_add(panel_width) {
                let mut cell = Cell::new(' ');
                cell.bg = Some(self.bg_color);
                put(ctx, x, y, cell);
            }
        }

        // Draw border
        for y in area.y..bottom {
            let mut cell = Cell::new('│');
            cell.fg = Some(Color::WHITE);
            cell.bg = Some(self.bg_color);
            put(ctx, panel_x, y, cell);
        }

        // Draw title
        let title = " Inspector ";
        let title_x = panel_x.saturating_add(2);
        for (i, ch) in title.chars().enumerate() {
            let Some(x) = title_x.checked_add(i as u16) else {
                break;
            };
            let mut cell = Cell::new(ch);
            cell.fg = Some(Color::CYAN);
            cell.bg = Some(self.bg_color);
            cell.modifier |= crate::render::Modifier::BOLD;
            put(ctx, x, area.y, cell);
        }

        // Draw widget tree
        if let Some(ref root) = self.root {
            let mut state = TreeRenderState {
                x: panel_x.saturating_add(2),
                y: area.y.saturating_add(2),
                depth: 0,
                index: 0,
                max_width: panel_width.saturating_sub(3),
            };
            self.render_widget_tree(ctx, root, &mut state);
        }

        // Draw selected widget properties
        if let Some(widget) = self.selected_widget() {
            let props_y = area.y.saturating_add(area.height / 2);
            self.render_properties(
                ctx,
                widget,
                panel_x.saturating_add(2),
                props_y,
                panel_width.saturating_sub(3),
            );
        }

        // Draw bounds overlay on main content
        if self.show_bounds {
            if let Some(widget) = self.selected_widget() {
                self.render_bounds_overlay(ctx, &widget.bounds, panel_x);
            }
        }
    }

    fn render_widget_tree(
        &self,
        ctx: &mut RenderContext,
        widget: &WidgetInfo,
        state: &mut TreeRenderState,
    ) -> usize {
        if state.y >= ctx.area.y.saturating_add(ctx.area.height / 2) {
            return state.index;
        }

        let is_selected = state.index == self.selected;
        let indent = "  ".repeat(state.depth);
        let prefix = if widget.children.is_empty() {
            "•"
        } else {
            "▼"
        };
        let text = format!("{}{} {}", indent, prefix, widget.type_name);

        let (fg, bg) = if is_selected {
            (Some(Color::BLACK), Some(self.highlight_color))
        } else {
            (Some(Color::WHITE), Some(self.bg_color))
        };

        for (i, ch) in text.chars().take(state.max_width as usize).enumerate() {
            let mut cell = Cell::new(ch);
            cell.fg = fg;
            cell.bg = bg;
            put(ctx, state.x.saturating_add(i as u16), state.y, cell);
        }

        // Fill rest of line with background
        for i in text.len()..(state.max_width as usize) {
            let mut cell = Cell::new(' ');
            cell.bg = bg;
            put(ctx, state.x.saturating_add(i as u16), state.y, cell);
        }

        state.y = state.y.saturating_add(1);
        state.index += 1;
        state.depth += 1;

        for child in &widget.children {
            self.render_widget_tree(ctx, child, state);
        }

        state.depth -= 1;
        state.index
    }

    fn render_properties(
        &self,
        ctx: &mut RenderContext,
        widget: &WidgetInfo,
        x: u16,
        mut y: u16,
        max_width: u16,
    ) {
        // Draw separator
        for dx in 0..max_width {
            let mut cell = Cell::new('─');
            cell.fg = Some(DARK_GRAY);
            cell.bg = Some(self.bg_color);
            put(ctx, x.saturating_add(dx), y, cell);
        }
        y = y.saturating_add(1);

        // Draw "Properties" label
        let label = "Properties";
        for (i, ch) in label.chars().enumerate() {
            let mut cell = Cell::new(ch);
            cell.fg = Some(Color::YELLOW);
            cell.bg = Some(self.bg_color);
            cell.modifier |= crate::render::Modifier::BOLD;
            put(ctx, x.saturating_add(i as u16), y, cell);
        }
        y = y.saturating_add(2);

        // Draw bounds
        let bounds_text = format!(
            "x: {}, y: {}, w: {}, h: {}",
            widget.bounds.x, widget.bounds.y, widget.bounds.width, widget.bounds.height
        );
        for (i, ch) in bounds_text.chars().take(max_width as usize).enumerate() {
            let mut cell = Cell::new(ch);
            cell.fg = Some(MUTED_TEXT);
            cell.bg = Some(self.bg_color);
            put(ctx, x.saturating_add(i as u16), y, cell);
        }
        y = y.saturating_add(1);

        // Draw custom properties
        for (key, value) in &widget.properties {
            if y >= (ctx.area.y.saturating_add(ctx.area.height)).saturating_sub(1) {
                break;
            }

            let prop_text = format!("{}: {}", key, value);
            for (i, ch) in prop_text.chars().take(max_width as usize).enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(MUTED_TEXT);
                cell.bg = Some(self.bg_color);
                put(ctx, x.saturating_add(i as u16), y, cell);
            }
            y = y.saturating_add(1);
        }
    }

    fn render_bounds_overlay(&self, ctx: &mut RenderContext, bounds: &Rect, panel_x: u16) {
        // The highlight is clipped to the inspector's own area, left of the
        // panel (and its one-column gap). Given the whole screen, as
        // intended, that is everything a widget can occupy.
        let max_x = panel_x.saturating_sub(1);
        let right = bounds.x.saturating_add(bounds.width);
        let bottom = bounds.y.saturating_add(bounds.height);
        let mut highlight = |x: u16, y: u16| {
            if x < max_x && in_area(ctx, x, y) {
                if let Some(cell) = ctx.buffer.get_mut(x, y) {
                    cell.fg = Some(self.highlight_color);
                }
            }
        };

        // Top and bottom borders
        for x in bounds.x..right {
            highlight(x, bounds.y);
            if bounds.height > 1 {
                highlight(x, bottom - 1);
            }
        }

        // Left and right borders
        for y in bounds.y..bottom {
            highlight(bounds.x, y);
            if bounds.width > 1 {
                highlight(right - 1, y);
            }
        }
    }
}

/// Whether absolute `(x, y)` is inside the context's area and not clipped.
fn in_area(ctx: &RenderContext, x: u16, y: u16) -> bool {
    ctx.area.contains(x, y) && !ctx.is_clipped(x, y)
}

/// Set the cell at absolute `(x, y)` if it is inside the context's area -
/// the panel's rows are laid out in absolute coordinates and may run past
/// the area (a narrow panel, a long type name).
fn put(ctx: &mut RenderContext, x: u16, y: u16, cell: Cell) {
    if in_area(ctx, x, y) {
        ctx.buffer.set(x, y, cell);
    }
}

impl Default for Inspector {
    fn default() -> Self {
        Self::new()
    }
}

impl View for Inspector {
    fn render(&self, ctx: &mut RenderContext) {
        Inspector::render(self, ctx);
    }
}

/// Helper function to create an inspector
pub fn inspector() -> Inspector {
    Inspector::new()
}
// KEEP HERE - Private implementation tests (accesses private fields)

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::Buffer;

    #[test]
    fn test_inspector_new() {
        let insp = Inspector::new();
        assert!(!insp.is_visible());
    }

    #[test]
    fn test_inspector_toggle() {
        let mut insp = Inspector::new();
        assert!(!insp.is_visible());

        insp.toggle();
        assert!(insp.is_visible());

        insp.toggle();
        assert!(!insp.is_visible());
    }

    #[test]
    fn test_inspector_show_hide() {
        let mut insp = Inspector::new();

        insp.show();
        assert!(insp.is_visible());

        insp.hide();
        assert!(!insp.is_visible());
    }

    #[test]
    fn test_widget_info() {
        let info = WidgetInfo::new("Text", Rect::new(0, 0, 10, 5))
            .property("content", "Hello")
            .property("color", "red");

        assert_eq!(info.type_name, "Text");
        assert_eq!(info.properties.len(), 2);
    }

    #[test]
    fn test_widget_info_children() {
        let child = WidgetInfo::new("Text", Rect::new(0, 0, 5, 1));
        let parent = WidgetInfo::new("Stack", Rect::new(0, 0, 10, 5)).child(child);

        assert_eq!(parent.children.len(), 1);
        assert_eq!(parent.descendant_count(), 1);
    }

    #[test]
    fn test_inspector_selection() {
        let mut insp = Inspector::new();

        let root = WidgetInfo::new("Root", Rect::new(0, 0, 80, 24))
            .child(WidgetInfo::new("Child1", Rect::new(0, 0, 40, 12)))
            .child(WidgetInfo::new("Child2", Rect::new(40, 0, 40, 12)));

        insp.set_root(root);

        assert_eq!(insp.selected_widget().unwrap().type_name, "Root");

        insp.select_next();
        assert_eq!(insp.selected_widget().unwrap().type_name, "Child1");

        insp.select_next();
        assert_eq!(insp.selected_widget().unwrap().type_name, "Child2");

        insp.select_prev();
        assert_eq!(insp.selected_widget().unwrap().type_name, "Child1");
    }

    #[test]
    fn test_inspector_handle_key() {
        use crate::event::Key;

        let mut insp = Inspector::new();
        insp.show();

        let root = WidgetInfo::new("Root", Rect::new(0, 0, 80, 24));
        insp.set_root(root);

        assert!(insp.handle_key(&Key::Down));
        assert!(insp.handle_key(&Key::Up));
        assert!(insp.handle_key(&Key::Escape));
        assert!(!insp.is_visible());
    }

    #[test]
    fn test_inspector_helper() {
        let insp = inspector().panel_width(50);
        assert_eq!(insp.panel_width, 50);
    }

    #[test]
    fn test_inspector_render() {
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut insp = Inspector::new();
        insp.show();
        insp.set_root(WidgetInfo::new("Root", Rect::new(0, 0, 80, 24)));
        insp.render(&mut ctx);

        // Check that inspector panel was rendered
        // The border character should be present
    }

    // =========================================================================
    // Additional inspector tests
    // =========================================================================

    #[test]
    fn test_inspector_default() {
        let insp = Inspector::default();
        assert!(!insp.is_visible());
        assert_eq!(insp.selected, 0);
        assert!(insp.root.is_none());
    }

    #[test]
    fn test_inspector_clear() {
        let mut insp = Inspector::new();
        insp.set_root(WidgetInfo::new("Root", Rect::new(0, 0, 80, 24)));
        insp.select_next();
        insp.clear();
        assert!(insp.root.is_none());
        assert_eq!(insp.selected, 0);
    }

    #[test]
    fn test_inspector_select_next_empty() {
        let mut insp = Inspector::new();
        insp.select_next();
        insp.select_prev();
        // Should not panic when no root is set
    }

    #[test]
    fn test_inspector_select_next_wraps() {
        let mut insp = Inspector::new();
        insp.set_root(WidgetInfo::new("Root", Rect::new(0, 0, 80, 24)));
        insp.select_next();
        // Should wrap back to 0
        assert_eq!(insp.selected, 0);
    }

    #[test]
    fn test_inspector_select_prev_wraps() {
        let mut insp = Inspector::new();
        let root = WidgetInfo::new("Root", Rect::new(0, 0, 80, 24))
            .child(WidgetInfo::new("Child", Rect::new(0, 0, 40, 12)));
        insp.set_root(root);
        insp.selected = 0;
        insp.select_prev();
        // Should wrap to last
        assert_eq!(insp.selected, 1);
    }

    #[test]
    fn test_inspector_handle_key_j() {
        use crate::event::Key;

        let mut insp = Inspector::new();
        insp.show();
        let root = WidgetInfo::new("Root", Rect::new(0, 0, 80, 24));
        insp.set_root(root);
        assert!(insp.handle_key(&Key::Char('j')));
    }

    #[test]
    fn test_inspector_handle_key_k() {
        use crate::event::Key;

        let mut insp = Inspector::new();
        insp.show();
        let root = WidgetInfo::new("Root", Rect::new(0, 0, 80, 24));
        insp.set_root(root);
        assert!(insp.handle_key(&Key::Char('k')));
    }

    #[test]
    fn test_inspector_handle_key_b() {
        use crate::event::Key;

        let mut insp = Inspector::new();
        insp.show();
        insp.handle_key(&Key::Char('b'));
        assert!(!insp.show_bounds);
        insp.handle_key(&Key::Char('b'));
        assert!(insp.show_bounds);
    }

    #[test]
    fn test_inspector_handle_key_when_hidden() {
        use crate::event::Key;

        let mut insp = Inspector::new();
        assert!(!insp.handle_key(&Key::Down));
        assert!(!insp.handle_key(&Key::Up));
        assert!(!insp.handle_key(&Key::Escape));
    }

    #[test]
    fn test_inspector_handle_key_unknown() {
        use crate::event::Key;

        let mut insp = Inspector::new();
        insp.show();
        assert!(!insp.handle_key(&Key::Char('x')));
        assert!(!insp.handle_key(&Key::Enter));
        assert!(!insp.handle_key(&Key::Tab));
    }

    #[test]
    fn test_inspector_panel_width_builder() {
        let insp = Inspector::new().panel_width(60);
        assert_eq!(insp.panel_width, 60);
    }

    #[test]
    fn test_inspector_show_bounds_builder() {
        let insp = Inspector::new().show_bounds(false);
        assert!(!insp.show_bounds);
    }

    #[test]
    fn test_inspector_highlight_color_builder() {
        let insp = Inspector::new().highlight_color(Color::RED);
        assert_eq!(insp.highlight_color, Color::RED);
    }

    #[test]
    fn test_widget_info_new_with_string() {
        let type_name = String::from("CustomWidget");
        let info = WidgetInfo::new(type_name.clone(), Rect::new(0, 0, 10, 10));
        assert_eq!(info.type_name, type_name);
    }

    #[test]
    fn test_widget_info_property_builder() {
        let info = WidgetInfo::new("Test", Rect::new(0, 0, 10, 10))
            .property("key1", "value1")
            .property("key2", "value2");
        assert_eq!(info.properties.len(), 2);
    }

    #[test]
    fn test_widget_info_focused_builder() {
        let info = WidgetInfo::new("Test", Rect::new(0, 0, 10, 10)).focused(true);
        assert!(info.focused);
    }

    #[test]
    fn test_widget_info_hovered_builder() {
        let info = WidgetInfo::new("Test", Rect::new(0, 0, 10, 10)).hovered(true);
        assert!(info.hovered);
    }

    #[test]
    fn test_widget_info_multiple_children() {
        let child1 = WidgetInfo::new("Child1", Rect::new(0, 0, 5, 5));
        let child2 = WidgetInfo::new("Child2", Rect::new(5, 0, 5, 5));
        let child3 = WidgetInfo::new("Child3", Rect::new(0, 5, 5, 5));
        let parent = WidgetInfo::new("Parent", Rect::new(0, 0, 10, 10))
            .child(child1)
            .child(child2)
            .child(child3);
        assert_eq!(parent.children.len(), 3);
        assert_eq!(parent.descendant_count(), 3);
    }

    #[test]
    fn test_widget_info_nested_children() {
        let grandchild = WidgetInfo::new("Grandchild", Rect::new(0, 0, 5, 5));
        let child = WidgetInfo::new("Child", Rect::new(0, 0, 10, 10)).child(grandchild);
        let parent = WidgetInfo::new("Parent", Rect::new(0, 0, 20, 20)).child(child);
        assert_eq!(parent.descendant_count(), 2);
    }

    #[test]
    fn test_widget_info_clone() {
        let info = WidgetInfo::new("Test", Rect::new(0, 0, 10, 10))
            .property("key", "value")
            .focused(true);
        let cloned = info.clone();
        assert_eq!(cloned.type_name, info.type_name);
        assert_eq!(cloned.properties.len(), info.properties.len());
        assert_eq!(cloned.focused, info.focused);
    }

    #[test]
    fn test_widget_info_empty_children() {
        let info = WidgetInfo::new("Test", Rect::new(0, 0, 10, 10));
        assert!(info.children.is_empty());
        assert_eq!(info.descendant_count(), 0);
    }

    #[test]
    fn test_inspector_render_when_hidden() {
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let insp = Inspector::new();
        // Don't show, render should do nothing
        insp.render(&mut ctx);
        // Should not panic
    }

    #[test]
    fn test_inspector_selected_widget_none() {
        let insp = Inspector::new();
        assert!(insp.selected_widget().is_none());
    }

    #[test]
    fn test_inspector_render_with_no_root() {
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut insp = Inspector::new();
        insp.show();
        insp.render(&mut ctx);
        // Should render panel but no tree
    }

    #[test]
    fn test_inspector_helper_function() {
        let insp = inspector();
        assert!(!insp.is_visible());
        assert_eq!(insp.panel_width, 40);
    }

    fn shown_with_tree() -> Inspector {
        let mut insp = Inspector::new();
        insp.set_root(
            WidgetInfo::new("Root", Rect::new(0, 0, 10, 3))
                .property("id", "main")
                .child(WidgetInfo::new("Child", Rect::new(1, 1, 4, 1))),
        );
        insp.show();
        insp
    }

    // The panel is laid out from the area's width; narrower than its frame
    // (3 columns) it used to underflow, and against the end of the
    // coordinate space its title and rows used to overflow.
    #[test]
    fn test_inspector_render_in_tiny_areas_and_at_the_coordinate_edge() {
        let insp = shown_with_tree();
        for area in [
            Rect::new(0, 0, 0, 0),
            Rect::new(2, 1, 1, 1),
            Rect::new(2, 1, 2, 1),
            Rect::new(2, 1, 1, 2),
            Rect::new(2, 1, 1, 1000),
            Rect::new(u16::MAX - 5, 1, 5, 2),
            Rect::new(1, u16::MAX - 5, 2, 5),
            Rect::new(u16::MAX - 5, u16::MAX - 5, 5, 5),
        ] {
            let mut buffer = Buffer::new(16, 16);
            let mut ctx = RenderContext::new(&mut buffer, area);
            insp.render(&mut ctx);
        }
    }

    // Every write - the panel and the highlight of the selected widget's
    // bounds - stays inside the area the inspector was given, even when the
    // widget's bounds (absolute) reach outside it.
    #[test]
    fn test_inspector_render_stays_inside_its_area() {
        let insp = shown_with_tree();
        for area in [
            Rect::new(2, 1, 40, 10),
            Rect::new(2, 1, 3, 3),
            Rect::new(2, 1, 1000, 1),
            Rect::new(2, 1, 0, 0),
        ] {
            let (bw, bh) = (area.width.min(60) + 4, area.height + 2);
            let mut buffer = Buffer::new(bw, bh);
            let mut ctx = RenderContext::new(&mut buffer, area);
            insp.render(&mut ctx);
            for y in 0..bh {
                for x in 0..bw {
                    if !area.contains(x, y) {
                        assert_eq!(
                            buffer.get(x, y),
                            Some(&Cell::default()),
                            "{area:?}: touched ({x},{y})"
                        );
                    }
                }
            }
        }
    }

    // Given the whole screen, as intended, the highlight is still drawn.
    #[test]
    fn test_inspector_highlights_the_selected_widget_on_the_whole_screen() {
        let insp = shown_with_tree();
        let mut buffer = Buffer::new(80, 24);
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 80, 24));
        insp.render(&mut ctx);
        assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::CYAN));
        assert_eq!(buffer.get(9, 2).unwrap().fg, Some(Color::CYAN));
        assert_eq!(buffer.get(10, 0).unwrap().fg, None);
    }
}
