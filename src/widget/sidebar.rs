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
