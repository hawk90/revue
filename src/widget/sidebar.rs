//! Sidebar Navigation widget
//!
//! A vertical navigation rail for application navigation.
//!
//! Features:
//! - Vertical navigation rail
//! - Collapsible (icon-only mode)
//! - Nested/hierarchical items
//! - Active item highlighting
//! - Icon + label format
//! - Section dividers
//! - Header/footer slots
//! - Keyboard navigation

use super::traits::{RenderContext, View, WidgetProps};
use crate::render::Cell;
use crate::style::Color;
use crate::{impl_props_builders, impl_styled_view};

/// Sidebar item representing a navigation entry
#[derive(Clone, Debug)]
pub struct SidebarItem {
    /// Unique identifier for this item
    pub id: String,
    /// Display label
    pub label: String,
    /// Icon character (emoji or unicode symbol)
    pub icon: Option<char>,
    /// Whether item is disabled
    pub disabled: bool,
    /// Badge text (e.g., notification count)
    pub badge: Option<String>,
    /// Child items for nested navigation
    pub children: Vec<SidebarItem>,
    /// Whether children are expanded
    pub expanded: bool,
}

impl SidebarItem {
    /// Create a new sidebar item
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: None,
            disabled: false,
            badge: None,
            children: Vec::new(),
            expanded: false,
        }
    }

    /// Set icon
    pub fn icon(mut self, icon: char) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set badge text
    pub fn badge(mut self, badge: impl Into<String>) -> Self {
        self.badge = Some(badge.into());
        self
    }

    /// Add child items
    pub fn children(mut self, children: Vec<SidebarItem>) -> Self {
        self.children = children;
        self
    }

    /// Set expanded state
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    /// Check if item has children
    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }
}

/// Section divider for grouping sidebar items
#[derive(Clone, Debug)]
pub struct SidebarSection {
    /// Section title (optional)
    pub title: Option<String>,
    /// Items in this section
    pub items: Vec<SidebarItem>,
}

impl SidebarSection {
    /// Create a new section without a title
    pub fn new(items: Vec<SidebarItem>) -> Self {
        Self { title: None, items }
    }

    /// Create a new section with a title
    pub fn titled(title: impl Into<String>, items: Vec<SidebarItem>) -> Self {
        Self {
            title: Some(title.into()),
            items,
        }
    }
}

/// Sidebar collapse mode
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CollapseMode {
    /// Always show full sidebar
    #[default]
    Expanded,
    /// Show icons only
    Collapsed,
    /// Auto-collapse based on width
    Auto,
}

/// Sidebar widget for vertical navigation
#[derive(Clone, Debug)]
pub struct Sidebar {
    /// Navigation sections
    sections: Vec<SidebarSection>,
    /// Currently selected item ID
    selected: Option<String>,
    /// Currently hovered item index in flattened list
    hovered: usize,
    /// Collapse mode
    collapse_mode: CollapseMode,
    /// Auto-collapse width threshold
    collapse_threshold: u16,
    /// Expanded width
    expanded_width: u16,
    /// Collapsed width (icon-only)
    collapsed_width: u16,
    /// Header content (rendered at top)
    header: Option<String>,
    /// Footer content (rendered at bottom)
    footer: Option<String>,
    /// Scroll offset
    scroll: usize,
    // Styling
    fg: Option<Color>,
    bg: Option<Color>,
    selected_fg: Option<Color>,
    selected_bg: Option<Color>,
    hover_fg: Option<Color>,
    hover_bg: Option<Color>,
    disabled_fg: Option<Color>,
    section_fg: Option<Color>,
    badge_fg: Option<Color>,
    badge_bg: Option<Color>,
    border_fg: Option<Color>,
    /// Widget props
    props: WidgetProps,
}

impl Sidebar {
    /// Create a new sidebar
    pub fn new() -> Self {
        Self {
            sections: Vec::new(),
            selected: None,
            hovered: 0,
            collapse_mode: CollapseMode::Expanded,
            collapse_threshold: 20,
            expanded_width: 24,
            collapsed_width: 4,
            header: None,
            footer: None,
            scroll: 0,
            fg: None,
            bg: Some(Color::rgb(30, 30, 40)),
            selected_fg: Some(Color::WHITE),
            selected_bg: Some(Color::BLUE),
            hover_fg: Some(Color::WHITE),
            hover_bg: Some(Color::rgb(50, 50, 70)),
            disabled_fg: Some(Color::rgb(100, 100, 100)),
            section_fg: Some(Color::rgb(128, 128, 128)),
            badge_fg: Some(Color::WHITE),
            badge_bg: Some(Color::RED),
            border_fg: Some(Color::rgb(60, 60, 80)),
            props: WidgetProps::new(),
        }
    }

    /// Add a section
    pub fn section(mut self, section: SidebarSection) -> Self {
        self.sections.push(section);
        self
    }

    /// Add multiple sections
    pub fn sections(mut self, sections: Vec<SidebarSection>) -> Self {
        self.sections = sections;
        self
    }

    /// Add items without a section title
    pub fn items(mut self, items: Vec<SidebarItem>) -> Self {
        self.sections.push(SidebarSection::new(items));
        self
    }

    /// Set selected item by ID
    pub fn selected(mut self, id: impl Into<String>) -> Self {
        self.selected = Some(id.into());
        self
    }

    /// Set collapse mode
    pub fn collapse_mode(mut self, mode: CollapseMode) -> Self {
        self.collapse_mode = mode;
        self
    }

    /// Set collapse threshold for auto mode
    pub fn collapse_threshold(mut self, width: u16) -> Self {
        self.collapse_threshold = width;
        self
    }

    /// Set expanded width
    pub fn expanded_width(mut self, width: u16) -> Self {
        self.expanded_width = width;
        self
    }

    /// Set collapsed width
    pub fn collapsed_width(mut self, width: u16) -> Self {
        self.collapsed_width = width;
        self
    }

    /// Set header text
    pub fn header(mut self, header: impl Into<String>) -> Self {
        self.header = Some(header.into());
        self
    }

    /// Set footer text
    pub fn footer(mut self, footer: impl Into<String>) -> Self {
        self.footer = Some(footer.into());
        self
    }

    /// Set foreground color
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Set background color
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    /// Set selected item style
    pub fn selected_style(mut self, fg: Color, bg: Color) -> Self {
        self.selected_fg = Some(fg);
        self.selected_bg = Some(bg);
        self
    }

    /// Set hover style
    pub fn hover_style(mut self, fg: Color, bg: Color) -> Self {
        self.hover_fg = Some(fg);
        self.hover_bg = Some(bg);
        self
    }

    /// Set disabled color
    pub fn disabled_color(mut self, color: Color) -> Self {
        self.disabled_fg = Some(color);
        self
    }

    /// Set section title color
    pub fn section_color(mut self, color: Color) -> Self {
        self.section_fg = Some(color);
        self
    }

    /// Set badge style
    pub fn badge_style(mut self, fg: Color, bg: Color) -> Self {
        self.badge_fg = Some(fg);
        self.badge_bg = Some(bg);
        self
    }

    /// Set border color
    pub fn border_color(mut self, color: Color) -> Self {
        self.border_fg = Some(color);
        self
    }

    // ─────────────────────────────────────────────────────────────────────────
    // State getters
    // ─────────────────────────────────────────────────────────────────────────

    /// Get selected item ID
    pub fn selected_id(&self) -> Option<&str> {
        self.selected.as_deref()
    }

    /// Get hovered index
    pub fn hovered_index(&self) -> usize {
        self.hovered
    }

    /// Check if sidebar is collapsed
    pub fn is_collapsed(&self) -> bool {
        matches!(self.collapse_mode, CollapseMode::Collapsed)
    }

    /// Get current width based on collapse state
    pub fn current_width(&self) -> u16 {
        match self.collapse_mode {
            CollapseMode::Expanded => self.expanded_width,
            CollapseMode::Collapsed => self.collapsed_width,
            CollapseMode::Auto => self.expanded_width, // Determined at render time
        }
    }

    /// Get flattened list of visible items
    pub fn visible_items(&self) -> Vec<FlattenedItem> {
        let mut items = Vec::new();
        for section in &self.sections {
            if section.title.is_some() {
                items.push(FlattenedItem::Section(section.title.clone()));
            }
            for item in &section.items {
                self.flatten_item(item, 0, &mut items);
            }
        }
        items
    }

    fn flatten_item(&self, item: &SidebarItem, depth: usize, items: &mut Vec<FlattenedItem>) {
        items.push(FlattenedItem::Item {
            item: item.clone(),
            depth,
        });
        if item.expanded {
            for child in &item.children {
                self.flatten_item(child, depth + 1, items);
            }
        }
    }

    /// Get total item count (excluding sections)
    pub fn item_count(&self) -> usize {
        self.visible_items()
            .iter()
            .filter(|i| matches!(i, FlattenedItem::Item { .. }))
            .count()
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Navigation
    // ─────────────────────────────────────────────────────────────────────────

    /// Move hover down
    pub fn hover_down(&mut self) {
        let items = self.visible_items();
        let item_indices: Vec<usize> = items
            .iter()
            .enumerate()
            .filter_map(|(i, item)| match item {
                FlattenedItem::Item { item: it, .. } if !it.disabled => Some(i),
                _ => None,
            })
            .collect();

        if let Some(current_pos) = item_indices.iter().position(|&i| i == self.hovered) {
            if current_pos + 1 < item_indices.len() {
                self.hovered = item_indices[current_pos + 1];
            }
        } else if !item_indices.is_empty() {
            self.hovered = item_indices[0];
        }
    }

    /// Move hover up
    pub fn hover_up(&mut self) {
        let items = self.visible_items();
        let item_indices: Vec<usize> = items
            .iter()
            .enumerate()
            .filter_map(|(i, item)| match item {
                FlattenedItem::Item { item: it, .. } if !it.disabled => Some(i),
                _ => None,
            })
            .collect();

        if let Some(current_pos) = item_indices.iter().position(|&i| i == self.hovered) {
            if current_pos > 0 {
                self.hovered = item_indices[current_pos - 1];
            }
        } else if !item_indices.is_empty() {
            self.hovered = *item_indices.last().unwrap();
        }
    }

    /// Select the currently hovered item
    pub fn select_hovered(&mut self) {
        let items = self.visible_items();
        if let Some(FlattenedItem::Item { item, .. }) = items.get(self.hovered) {
            if !item.disabled {
                self.selected = Some(item.id.clone());
            }
        }
    }

    /// Toggle expansion of hovered item
    pub fn toggle_hovered(&mut self) {
        let items = self.visible_items();
        if let Some(FlattenedItem::Item { item, .. }) = items.get(self.hovered) {
            if item.has_children() {
                self.toggle_item(&item.id.clone());
            }
        }
    }

    /// Toggle item expansion by ID
    pub fn toggle_item(&mut self, id: &str) {
        for section in &mut self.sections {
            for item in &mut section.items {
                if Self::toggle_item_recursive(item, id) {
                    return;
                }
            }
        }
    }

    fn toggle_item_recursive(item: &mut SidebarItem, id: &str) -> bool {
        if item.id == id {
            item.expanded = !item.expanded;
            return true;
        }
        for child in &mut item.children {
            if Self::toggle_item_recursive(child, id) {
                return true;
            }
        }
        false
    }

    /// Expand all items
    pub fn expand_all(&mut self) {
        for section in &mut self.sections {
            for item in &mut section.items {
                Self::set_expanded_recursive(item, true);
            }
        }
    }

    /// Collapse all items
    pub fn collapse_all(&mut self) {
        for section in &mut self.sections {
            for item in &mut section.items {
                Self::set_expanded_recursive(item, false);
            }
        }
    }

    fn set_expanded_recursive(item: &mut SidebarItem, expanded: bool) {
        item.expanded = expanded;
        for child in &mut item.children {
            Self::set_expanded_recursive(child, expanded);
        }
    }

    /// Toggle sidebar collapse mode
    pub fn toggle_collapse(&mut self) {
        self.collapse_mode = match self.collapse_mode {
            CollapseMode::Expanded => CollapseMode::Collapsed,
            CollapseMode::Collapsed => CollapseMode::Expanded,
            CollapseMode::Auto => CollapseMode::Collapsed,
        };
    }
}

impl Default for Sidebar {
    fn default() -> Self {
        Self::new()
    }
}

/// Flattened item for rendering
#[derive(Clone, Debug)]
pub enum FlattenedItem {
    /// Section header
    Section(Option<String>),
    /// Navigation item with depth
    Item {
        /// The sidebar item
        item: SidebarItem,
        /// Nesting depth (0 = root level)
        depth: usize,
    },
}

impl View for Sidebar {
    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 3 || area.height < 2 {
            return;
        }

        // Determine if collapsed based on mode and available width
        let is_collapsed = match self.collapse_mode {
            CollapseMode::Expanded => false,
            CollapseMode::Collapsed => true,
            CollapseMode::Auto => area.width < self.collapse_threshold,
        };

        let content_width = if is_collapsed {
            self.collapsed_width.min(area.width)
        } else {
            self.expanded_width.min(area.width)
        };

        // Fill background
        for y in area.y..area.y + area.height {
            for x in area.x..area.x + content_width {
                let mut cell = Cell::new(' ');
                cell.bg = self.bg;
                ctx.buffer.set(x, y, cell);
            }
        }

        let mut y = area.y;

        // Render header if present
        if let Some(header) = &self.header {
            if !is_collapsed {
                let display: String = header.chars().take(content_width as usize - 2).collect();
                let x_offset = (content_width as usize - display.chars().count()) / 2;
                for (i, ch) in display.chars().enumerate() {
                    let mut cell = Cell::new(ch).bold();
                    cell.fg = self.fg;
                    cell.bg = self.bg;
                    ctx.buffer.set(area.x + x_offset as u16 + i as u16, y, cell);
                }
            }
            y += 1;

            // Separator line after header
            for x in area.x..area.x + content_width {
                let mut cell = Cell::new('─');
                cell.fg = self.border_fg;
                cell.bg = self.bg;
                ctx.buffer.set(x, y, cell);
            }
            y += 1;
        }

        // Calculate available height for items
        let footer_height = if self.footer.is_some() { 2 } else { 0 };
        let _available_height = area.height.saturating_sub(y - area.y + footer_height);

        // Get visible items
        let items = self.visible_items();

        // Render items
        for (idx, flat_item) in items.iter().skip(self.scroll).enumerate() {
            if y >= area.y + area.height - footer_height {
                break;
            }

            match flat_item {
                FlattenedItem::Section(title) => {
                    if !is_collapsed {
                        if let Some(title_text) = title {
                            // Section title
                            let display: String = title_text
                                .chars()
                                .take(content_width as usize - 2)
                                .collect();
                            for (i, ch) in display.chars().enumerate() {
                                let mut cell = Cell::new(ch);
                                cell.fg = self.section_fg;
                                cell.bg = self.bg;
                                ctx.buffer.set(area.x + 1 + i as u16, y, cell);
                            }
                        } else {
                            // Separator line
                            for x in area.x + 1..area.x + content_width - 1 {
                                let mut cell = Cell::new('─');
                                cell.fg = self.border_fg;
                                cell.bg = self.bg;
                                ctx.buffer.set(x, y, cell);
                            }
                        }
                    }
                    y += 1;
                }
                FlattenedItem::Item { item, depth } => {
                    let actual_idx = self.scroll + idx;
                    let is_selected = self.selected.as_ref() == Some(&item.id);
                    let is_hovered = actual_idx == self.hovered;

                    // Determine colors
                    let (fg, bg) = if item.disabled {
                        (self.disabled_fg, self.bg)
                    } else if is_selected {
                        (self.selected_fg, self.selected_bg)
                    } else if is_hovered {
                        (self.hover_fg, self.hover_bg)
                    } else {
                        (self.fg, self.bg)
                    };

                    // Fill row background
                    for x in area.x..area.x + content_width {
                        let mut cell = Cell::new(' ');
                        cell.bg = bg;
                        ctx.buffer.set(x, y, cell);
                    }

                    let indent = if is_collapsed { 0 } else { (*depth as u16) * 2 };
                    let mut x = area.x + 1 + indent;

                    // Expand/collapse indicator for items with children
                    if item.has_children() && !is_collapsed {
                        let indicator = if item.expanded { '▼' } else { '▶' };
                        let mut cell = Cell::new(indicator);
                        cell.fg = fg;
                        cell.bg = bg;
                        ctx.buffer.set(x, y, cell);
                        x += 2;
                    } else if !is_collapsed {
                        x += 2; // Align with items that have children
                    }

                    // Icon
                    if let Some(icon) = item.icon {
                        let mut cell = Cell::new(icon);
                        cell.fg = fg;
                        cell.bg = bg;
                        ctx.buffer.set(x, y, cell);
                        x += 2;
                    }

                    // Label (only if not collapsed)
                    if !is_collapsed {
                        let max_label_width = content_width.saturating_sub(x - area.x + 1);
                        let badge_space = item.badge.as_ref().map(|b| b.len() + 2).unwrap_or(0);
                        let label_width = (max_label_width as usize).saturating_sub(badge_space);
                        let display: String = item.label.chars().take(label_width).collect();

                        for ch in display.chars() {
                            if x < area.x + content_width - badge_space as u16 {
                                let mut cell = Cell::new(ch);
                                cell.fg = fg;
                                cell.bg = bg;
                                ctx.buffer.set(x, y, cell);
                                x += 1;
                            }
                        }

                        // Badge
                        if let Some(badge) = &item.badge {
                            let badge_x = area.x + content_width - badge.len() as u16 - 2;
                            for (i, ch) in badge.chars().enumerate() {
                                let mut cell = Cell::new(ch);
                                cell.fg = self.badge_fg;
                                cell.bg = self.badge_bg;
                                ctx.buffer.set(badge_x + i as u16, y, cell);
                            }
                        }
                    }

                    y += 1;
                }
            }
        }

        // Render footer if present
        if let Some(footer) = &self.footer {
            // Separator line before footer
            let footer_y = area.y + area.height - 2;
            for x in area.x..area.x + content_width {
                let mut cell = Cell::new('─');
                cell.fg = self.border_fg;
                cell.bg = self.bg;
                ctx.buffer.set(x, footer_y, cell);
            }

            if !is_collapsed {
                let display: String = footer.chars().take(content_width as usize - 2).collect();
                let x_offset = (content_width as usize - display.chars().count()) / 2;
                for (i, ch) in display.chars().enumerate() {
                    let mut cell = Cell::new(ch);
                    cell.fg = self.section_fg;
                    cell.bg = self.bg;
                    ctx.buffer
                        .set(area.x + x_offset as u16 + i as u16, footer_y + 1, cell);
                }
            }
        }

        // Right border
        for y in area.y..area.y + area.height {
            let border_x = area.x + content_width - 1;
            if border_x < area.x + area.width {
                let mut cell = Cell::new('│');
                cell.fg = self.border_fg;
                cell.bg = self.bg;
                ctx.buffer.set(border_x, y, cell);
            }
        }
    }

    crate::impl_view_meta!("Sidebar");
}

impl_styled_view!(Sidebar);
impl_props_builders!(Sidebar);

/// Helper function to create a sidebar
pub fn sidebar() -> Sidebar {
    Sidebar::new()
}

/// Helper function to create a sidebar item
pub fn sidebar_item(id: impl Into<String>, label: impl Into<String>) -> SidebarItem {
    SidebarItem::new(id, label)
}

/// Helper function to create a sidebar section
pub fn sidebar_section(items: Vec<SidebarItem>) -> SidebarSection {
    SidebarSection::new(items)
}

/// Helper function to create a titled sidebar section
pub fn sidebar_section_titled(title: impl Into<String>, items: Vec<SidebarItem>) -> SidebarSection {
    SidebarSection::titled(title, items)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    // =========================================================================
    // SidebarItem Tests
    // =========================================================================

    #[test]
    fn test_sidebar_item_new() {
        let item = SidebarItem::new("home", "Home");
        assert_eq!(item.id, "home");
        assert_eq!(item.label, "Home");
        assert!(item.icon.is_none());
        assert!(!item.disabled);
        assert!(item.badge.is_none());
        assert!(item.children.is_empty());
        assert!(!item.expanded);
    }

    #[test]
    fn test_sidebar_item_icon() {
        let item = SidebarItem::new("home", "Home").icon('🏠');
        assert_eq!(item.icon, Some('🏠'));
    }

    #[test]
    fn test_sidebar_item_disabled() {
        let item = SidebarItem::new("home", "Home").disabled(true);
        assert!(item.disabled);
    }

    #[test]
    fn test_sidebar_item_badge() {
        let item = SidebarItem::new("inbox", "Inbox").badge("5");
        assert_eq!(item.badge, Some("5".to_string()));
    }

    #[test]
    fn test_sidebar_item_children() {
        let children = vec![
            SidebarItem::new("child1", "Child 1"),
            SidebarItem::new("child2", "Child 2"),
        ];
        let item = SidebarItem::new("parent", "Parent").children(children);
        assert_eq!(item.children.len(), 2);
    }

    #[test]
    fn test_sidebar_item_expanded() {
        let item = SidebarItem::new("folder", "Folder").expanded(true);
        assert!(item.expanded);
    }

    #[test]
    fn test_sidebar_item_has_children() {
        let item =
            SidebarItem::new("folder", "Folder").children(vec![SidebarItem::new("file", "File")]);
        assert!(item.has_children());

        let empty_item = SidebarItem::new("file", "File");
        assert!(!empty_item.has_children());
    }

    #[test]
    fn test_sidebar_item_builder_chain() {
        let item = SidebarItem::new("nav", "Navigation")
            .icon('📁')
            .disabled(false)
            .badge("3")
            .expanded(true);

        assert_eq!(item.icon, Some('📁'));
        assert!(!item.disabled);
        assert_eq!(item.badge, Some("3".to_string()));
        assert!(item.expanded);
    }

    // =========================================================================
    // SidebarSection Tests
    // =========================================================================

    #[test]
    fn test_sidebar_section_new() {
        let items = vec![SidebarItem::new("home", "Home")];
        let section = SidebarSection::new(items);
        assert!(section.title.is_none());
        assert_eq!(section.items.len(), 1);
    }

    #[test]
    fn test_sidebar_section_titled() {
        let items = vec![SidebarItem::new("home", "Home")];
        let section = SidebarSection::titled("Main", items);
        assert_eq!(section.title, Some("Main".to_string()));
        assert_eq!(section.items.len(), 1);
    }

    // =========================================================================
    // CollapseMode Tests
    // =========================================================================

    #[test]
    fn test_collapse_mode_default() {
        assert_eq!(CollapseMode::default(), CollapseMode::Expanded);
    }

    #[test]
    fn test_collapse_mode_equality() {
        assert_eq!(CollapseMode::Expanded, CollapseMode::Expanded);
        assert_ne!(CollapseMode::Expanded, CollapseMode::Collapsed);
    }

    // =========================================================================
    // Sidebar Creation Tests
    // =========================================================================

    #[test]
    fn test_sidebar_new() {
        let sb = Sidebar::new();
        assert!(sb.selected_id().is_none());
        assert_eq!(sb.hovered_index(), 0);
        assert!(!sb.is_collapsed());
    }

    #[test]
    fn test_sidebar_default() {
        let sb = Sidebar::default();
        assert!(!sb.is_collapsed());
    }

    #[test]
    fn test_sidebar_helper() {
        let sb = sidebar();
        assert!(sb.selected_id().is_none());
    }

    #[test]
    fn test_sidebar_item_helper() {
        let item = sidebar_item("test", "Test");
        assert_eq!(item.id, "test");
        assert_eq!(item.label, "Test");
    }

    #[test]
    fn test_sidebar_section_helper() {
        let section = sidebar_section(vec![sidebar_item("a", "A")]);
        assert!(section.title.is_none());
        assert_eq!(section.items.len(), 1);
    }

    #[test]
    fn test_sidebar_section_titled_helper() {
        let section = sidebar_section_titled("Title", vec![sidebar_item("a", "A")]);
        assert_eq!(section.title, Some("Title".to_string()));
    }

    // =========================================================================
    // Sidebar Builder Tests
    // =========================================================================

    #[test]
    fn test_sidebar_section_builder() {
        let sb =
            Sidebar::new().section(SidebarSection::new(vec![SidebarItem::new("home", "Home")]));
        assert_eq!(sb.item_count(), 1);
    }

    #[test]
    fn test_sidebar_sections_builder() {
        let sb = Sidebar::new().sections(vec![
            SidebarSection::new(vec![SidebarItem::new("a", "A")]),
            SidebarSection::new(vec![SidebarItem::new("b", "B")]),
        ]);
        assert_eq!(sb.item_count(), 2);
    }

    #[test]
    fn test_sidebar_items_builder() {
        let sb = Sidebar::new().items(vec![
            SidebarItem::new("home", "Home"),
            SidebarItem::new("settings", "Settings"),
        ]);
        assert_eq!(sb.item_count(), 2);
    }

    #[test]
    fn test_sidebar_selected() {
        let sb = Sidebar::new()
            .items(vec![SidebarItem::new("home", "Home")])
            .selected("home");
        assert_eq!(sb.selected_id(), Some("home"));
    }

    #[test]
    fn test_sidebar_collapse_mode() {
        let sb = Sidebar::new().collapse_mode(CollapseMode::Collapsed);
        assert!(sb.is_collapsed());
    }

    #[test]
    fn test_sidebar_collapse_threshold() {
        let sb = Sidebar::new().collapse_threshold(30);
        assert_eq!(sb.collapse_threshold, 30);
    }

    #[test]
    fn test_sidebar_expanded_width() {
        let sb = Sidebar::new().expanded_width(32);
        assert_eq!(sb.expanded_width, 32);
    }

    #[test]
    fn test_sidebar_collapsed_width() {
        let sb = Sidebar::new().collapsed_width(6);
        assert_eq!(sb.collapsed_width, 6);
    }

    #[test]
    fn test_sidebar_header() {
        let sb = Sidebar::new().header("App Name");
        assert_eq!(sb.header, Some("App Name".to_string()));
    }

    #[test]
    fn test_sidebar_footer() {
        let sb = Sidebar::new().footer("v1.0.0");
        assert_eq!(sb.footer, Some("v1.0.0".to_string()));
    }

    #[test]
    fn test_sidebar_fg() {
        let sb = Sidebar::new().fg(Color::WHITE);
        assert_eq!(sb.fg, Some(Color::WHITE));
    }

    #[test]
    fn test_sidebar_bg() {
        let sb = Sidebar::new().bg(Color::BLACK);
        assert_eq!(sb.bg, Some(Color::BLACK));
    }

    #[test]
    fn test_sidebar_selected_style() {
        let sb = Sidebar::new().selected_style(Color::WHITE, Color::BLUE);
        assert_eq!(sb.selected_fg, Some(Color::WHITE));
        assert_eq!(sb.selected_bg, Some(Color::BLUE));
    }

    #[test]
    fn test_sidebar_hover_style() {
        let sb = Sidebar::new().hover_style(Color::YELLOW, Color::CYAN);
        assert_eq!(sb.hover_fg, Some(Color::YELLOW));
        assert_eq!(sb.hover_bg, Some(Color::CYAN));
    }

    #[test]
    fn test_sidebar_disabled_color() {
        let sb = Sidebar::new().disabled_color(Color::rgb(128, 128, 128));
        assert_eq!(sb.disabled_fg, Some(Color::rgb(128, 128, 128)));
    }

    #[test]
    fn test_sidebar_section_color() {
        let sb = Sidebar::new().section_color(Color::CYAN);
        assert_eq!(sb.section_fg, Some(Color::CYAN));
    }

    #[test]
    fn test_sidebar_badge_style() {
        let sb = Sidebar::new().badge_style(Color::WHITE, Color::RED);
        assert_eq!(sb.badge_fg, Some(Color::WHITE));
        assert_eq!(sb.badge_bg, Some(Color::RED));
    }

    #[test]
    fn test_sidebar_border_color() {
        let sb = Sidebar::new().border_color(Color::MAGENTA);
        assert_eq!(sb.border_fg, Some(Color::MAGENTA));
    }

    // =========================================================================
    // State Getter Tests
    // =========================================================================

    #[test]
    fn test_sidebar_current_width_expanded() {
        let sb = Sidebar::new()
            .expanded_width(30)
            .collapse_mode(CollapseMode::Expanded);
        assert_eq!(sb.current_width(), 30);
    }

    #[test]
    fn test_sidebar_current_width_collapsed() {
        let sb = Sidebar::new()
            .collapsed_width(5)
            .collapse_mode(CollapseMode::Collapsed);
        assert_eq!(sb.current_width(), 5);
    }

    #[test]
    fn test_sidebar_current_width_auto() {
        let sb = Sidebar::new()
            .expanded_width(25)
            .collapse_mode(CollapseMode::Auto);
        // Auto mode returns expanded_width (actual collapse determined at render)
        assert_eq!(sb.current_width(), 25);
    }

    #[test]
    fn test_sidebar_visible_items_empty() {
        let sb = Sidebar::new();
        assert!(sb.visible_items().is_empty());
    }

    #[test]
    fn test_sidebar_visible_items_flat() {
        let sb = Sidebar::new().items(vec![SidebarItem::new("a", "A"), SidebarItem::new("b", "B")]);
        let items = sb.visible_items();
        assert_eq!(items.len(), 2);
    }

    #[test]
    fn test_sidebar_visible_items_with_section_title() {
        let sb = Sidebar::new().section(SidebarSection::titled(
            "Section",
            vec![SidebarItem::new("a", "A")],
        ));
        let items = sb.visible_items();
        assert_eq!(items.len(), 2); // Section header + item
    }

    #[test]
    fn test_sidebar_visible_items_nested_collapsed() {
        let sb = Sidebar::new()
            .items(vec![SidebarItem::new("parent", "Parent")
                .children(vec![SidebarItem::new("child", "Child")])]);
        let items = sb.visible_items();
        // Parent not expanded, so child is hidden
        assert_eq!(items.len(), 1);
    }

    #[test]
    fn test_sidebar_visible_items_nested_expanded() {
        let sb = Sidebar::new().items(vec![SidebarItem::new("parent", "Parent")
            .expanded(true)
            .children(vec![SidebarItem::new("child", "Child")])]);
        let items = sb.visible_items();
        // Parent expanded, so child is visible
        assert_eq!(items.len(), 2);
    }

    #[test]
    fn test_sidebar_item_count() {
        let sb = Sidebar::new()
            .section(SidebarSection::titled(
                "Main",
                vec![SidebarItem::new("a", "A")],
            ))
            .items(vec![SidebarItem::new("b", "B")]);
        // item_count excludes sections
        assert_eq!(sb.item_count(), 2);
    }

    // =========================================================================
    // Navigation Tests
    // =========================================================================

    #[test]
    fn test_sidebar_hover_down() {
        let mut sb = Sidebar::new().items(vec![
            SidebarItem::new("a", "A"),
            SidebarItem::new("b", "B"),
            SidebarItem::new("c", "C"),
        ]);
        assert_eq!(sb.hovered_index(), 0);
        sb.hover_down();
        assert_eq!(sb.hovered_index(), 1);
        sb.hover_down();
        assert_eq!(sb.hovered_index(), 2);
    }

    #[test]
    fn test_sidebar_hover_down_at_end() {
        let mut sb =
            Sidebar::new().items(vec![SidebarItem::new("a", "A"), SidebarItem::new("b", "B")]);
        sb.hover_down();
        sb.hover_down();
        sb.hover_down(); // Should stay at last
        assert_eq!(sb.hovered_index(), 1);
    }

    #[test]
    fn test_sidebar_hover_down_skips_disabled() {
        let mut sb = Sidebar::new().items(vec![
            SidebarItem::new("a", "A"),
            SidebarItem::new("b", "B").disabled(true),
            SidebarItem::new("c", "C"),
        ]);
        sb.hover_down();
        // Should skip disabled item B and go to C
        assert_eq!(sb.hovered_index(), 2);
    }

    #[test]
    fn test_sidebar_hover_up() {
        let mut sb = Sidebar::new().items(vec![
            SidebarItem::new("a", "A"),
            SidebarItem::new("b", "B"),
            SidebarItem::new("c", "C"),
        ]);
        sb.hovered = 2;
        sb.hover_up();
        assert_eq!(sb.hovered_index(), 1);
        sb.hover_up();
        assert_eq!(sb.hovered_index(), 0);
    }

    #[test]
    fn test_sidebar_hover_up_at_start() {
        let mut sb =
            Sidebar::new().items(vec![SidebarItem::new("a", "A"), SidebarItem::new("b", "B")]);
        sb.hover_up(); // Already at 0
        assert_eq!(sb.hovered_index(), 0);
    }

    #[test]
    fn test_sidebar_select_hovered() {
        let mut sb =
            Sidebar::new().items(vec![SidebarItem::new("a", "A"), SidebarItem::new("b", "B")]);
        sb.hovered = 1;
        sb.select_hovered();
        assert_eq!(sb.selected_id(), Some("b"));
    }

    #[test]
    fn test_sidebar_select_hovered_disabled() {
        let mut sb = Sidebar::new().items(vec![SidebarItem::new("a", "A").disabled(true)]);
        sb.select_hovered();
        // Should not select disabled item
        assert!(sb.selected_id().is_none());
    }

    #[test]
    fn test_sidebar_toggle_hovered() {
        let mut sb = Sidebar::new()
            .items(vec![SidebarItem::new("parent", "Parent")
                .children(vec![SidebarItem::new("child", "Child")])]);
        assert_eq!(sb.visible_items().len(), 1);
        sb.toggle_hovered();
        assert_eq!(sb.visible_items().len(), 2);
        sb.toggle_hovered();
        assert_eq!(sb.visible_items().len(), 1);
    }

    #[test]
    fn test_sidebar_toggle_item() {
        let mut sb = Sidebar::new()
            .items(vec![SidebarItem::new("folder", "Folder")
                .children(vec![SidebarItem::new("file", "File")])]);
        sb.toggle_item("folder");
        let items = sb.visible_items();
        assert_eq!(items.len(), 2);
    }

    #[test]
    fn test_sidebar_expand_all() {
        let mut sb = Sidebar::new().items(vec![
            SidebarItem::new("a", "A").children(vec![SidebarItem::new("a1", "A1")]),
            SidebarItem::new("b", "B").children(vec![SidebarItem::new("b1", "B1")]),
        ]);
        sb.expand_all();
        assert_eq!(sb.visible_items().len(), 4);
    }

    #[test]
    fn test_sidebar_collapse_all() {
        let mut sb = Sidebar::new().items(vec![
            SidebarItem::new("a", "A")
                .expanded(true)
                .children(vec![SidebarItem::new("a1", "A1")]),
            SidebarItem::new("b", "B")
                .expanded(true)
                .children(vec![SidebarItem::new("b1", "B1")]),
        ]);
        assert_eq!(sb.visible_items().len(), 4);
        sb.collapse_all();
        assert_eq!(sb.visible_items().len(), 2);
    }

    #[test]
    fn test_sidebar_toggle_collapse() {
        let mut sb = Sidebar::new().collapse_mode(CollapseMode::Expanded);
        assert!(!sb.is_collapsed());
        sb.toggle_collapse();
        assert!(sb.is_collapsed());
        sb.toggle_collapse();
        assert!(!sb.is_collapsed());
    }

    #[test]
    fn test_sidebar_toggle_collapse_from_auto() {
        let mut sb = Sidebar::new().collapse_mode(CollapseMode::Auto);
        sb.toggle_collapse();
        assert!(sb.is_collapsed());
    }

    // =========================================================================
    // FlattenedItem Tests
    // =========================================================================

    #[test]
    fn test_flattened_item_section() {
        let flat = FlattenedItem::Section(Some("Title".to_string()));
        if let FlattenedItem::Section(title) = flat {
            assert_eq!(title, Some("Title".to_string()));
        } else {
            panic!("Expected Section");
        }
    }

    #[test]
    fn test_flattened_item_item() {
        let flat = FlattenedItem::Item {
            item: SidebarItem::new("test", "Test"),
            depth: 2,
        };
        if let FlattenedItem::Item { item, depth } = flat {
            assert_eq!(item.id, "test");
            assert_eq!(depth, 2);
        } else {
            panic!("Expected Item");
        }
    }

    // =========================================================================
    // Render Tests
    // =========================================================================

    #[test]
    fn test_sidebar_render_basic() {
        let mut buffer = Buffer::new(30, 10);
        let area = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let sb = Sidebar::new().items(vec![
            SidebarItem::new("home", "Home").icon('🏠'),
            SidebarItem::new("settings", "Settings").icon('⚙'),
        ]);
        sb.render(&mut ctx);
        // Should not panic
    }

    #[test]
    fn test_sidebar_render_with_header() {
        let mut buffer = Buffer::new(30, 15);
        let area = Rect::new(0, 0, 30, 15);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let sb = Sidebar::new()
            .header("My App")
            .items(vec![SidebarItem::new("home", "Home")]);
        sb.render(&mut ctx);
    }

    #[test]
    fn test_sidebar_render_with_footer() {
        let mut buffer = Buffer::new(30, 15);
        let area = Rect::new(0, 0, 30, 15);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let sb = Sidebar::new()
            .footer("v1.0.0")
            .items(vec![SidebarItem::new("home", "Home")]);
        sb.render(&mut ctx);
    }

    #[test]
    fn test_sidebar_render_collapsed() {
        let mut buffer = Buffer::new(30, 10);
        let area = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let sb = Sidebar::new()
            .collapse_mode(CollapseMode::Collapsed)
            .items(vec![SidebarItem::new("home", "Home").icon('🏠')]);
        sb.render(&mut ctx);
    }

    #[test]
    fn test_sidebar_render_with_sections() {
        let mut buffer = Buffer::new(30, 15);
        let area = Rect::new(0, 0, 30, 15);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let sb = Sidebar::new()
            .section(SidebarSection::titled(
                "Main",
                vec![SidebarItem::new("home", "Home")],
            ))
            .section(SidebarSection::titled(
                "Settings",
                vec![SidebarItem::new("prefs", "Preferences")],
            ));
        sb.render(&mut ctx);
    }

    #[test]
    fn test_sidebar_render_nested() {
        let mut buffer = Buffer::new(30, 15);
        let area = Rect::new(0, 0, 30, 15);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let sb = Sidebar::new().items(vec![SidebarItem::new("folder", "Folder")
            .expanded(true)
            .children(vec![SidebarItem::new("file", "File")])]);
        sb.render(&mut ctx);
    }

    #[test]
    fn test_sidebar_render_with_badges() {
        let mut buffer = Buffer::new(30, 10);
        let area = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let sb = Sidebar::new().items(vec![SidebarItem::new("inbox", "Inbox").badge("5")]);
        sb.render(&mut ctx);
    }

    #[test]
    fn test_sidebar_render_with_disabled() {
        let mut buffer = Buffer::new(30, 10);
        let area = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let sb = Sidebar::new().items(vec![
            SidebarItem::new("active", "Active"),
            SidebarItem::new("disabled", "Disabled").disabled(true),
        ]);
        sb.render(&mut ctx);
    }

    #[test]
    fn test_sidebar_render_with_selected() {
        let mut buffer = Buffer::new(30, 10);
        let area = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let sb = Sidebar::new()
            .items(vec![
                SidebarItem::new("a", "Item A"),
                SidebarItem::new("b", "Item B"),
            ])
            .selected("b");
        sb.render(&mut ctx);
    }

    #[test]
    fn test_sidebar_render_small_area() {
        let mut buffer = Buffer::new(2, 1);
        let area = Rect::new(0, 0, 2, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let sb = Sidebar::new().items(vec![SidebarItem::new("home", "Home")]);
        sb.render(&mut ctx); // Should handle gracefully
    }

    #[test]
    fn test_sidebar_render_auto_collapse() {
        let mut buffer = Buffer::new(15, 10);
        let area = Rect::new(0, 0, 15, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let sb = Sidebar::new()
            .collapse_mode(CollapseMode::Auto)
            .collapse_threshold(20)
            .items(vec![SidebarItem::new("home", "Home").icon('🏠')]);
        // Width (15) < threshold (20), so should render collapsed
        sb.render(&mut ctx);
    }

    // =========================================================================
    // Edge Cases
    // =========================================================================

    #[test]
    fn test_sidebar_empty() {
        let sb = Sidebar::new();
        assert_eq!(sb.item_count(), 0);
        assert!(sb.visible_items().is_empty());
    }

    #[test]
    fn test_sidebar_navigation_on_empty() {
        let mut sb = Sidebar::new();
        sb.hover_up();
        sb.hover_down();
        sb.select_hovered();
        // Should not panic
        assert_eq!(sb.hovered_index(), 0);
    }

    #[test]
    fn test_sidebar_toggle_nonexistent_item() {
        let mut sb = Sidebar::new().items(vec![SidebarItem::new("a", "A")]);
        sb.toggle_item("nonexistent");
        // Should not panic
    }

    #[test]
    fn test_sidebar_deeply_nested() {
        let sb = Sidebar::new().items(vec![SidebarItem::new("l1", "Level 1")
            .expanded(true)
            .children(vec![SidebarItem::new("l2", "Level 2")
                .expanded(true)
                .children(vec![SidebarItem::new("l3", "Level 3")
                    .expanded(true)
                    .children(vec![SidebarItem::new("l4", "Level 4")])])])]);

        let items = sb.visible_items();
        assert_eq!(items.len(), 4);

        // Check depths
        if let FlattenedItem::Item { depth, .. } = &items[3] {
            assert_eq!(*depth, 3);
        }
    }
}
