//! Empty State widget for displaying no-data scenarios gracefully
//!
//! A dedicated widget for consistent, helpful empty state displays.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::widget::{EmptyState, EmptyStateType, empty_state};
//!
//! // Basic empty state
//! EmptyState::new("No items yet")
//!     .description("Create your first item to get started");
//!
//! // Search with no results
//! empty_state("No results found")
//!     .state_type(EmptyStateType::NoResults)
//!     .description("Try adjusting your search terms")
//!     .action("Clear search");
//!
//! // Error state
//! EmptyState::error("Failed to load data")
//!     .description("Check your connection and try again")
//!     .action("Retry");
//! ```

use super::traits::{RenderContext, View, WidgetProps, WidgetState};
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::{impl_props_builders, impl_state_builders, impl_styled_view};

/// Empty state type/scenario
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum EmptyStateType {
    /// No data available (default)
    #[default]
    Empty,
    /// Search returned no results
    NoResults,
    /// Error occurred
    Error,
    /// No permission to view
    NoPermission,
    /// Offline/disconnected
    Offline,
    /// First-time user experience
    FirstUse,
}

impl EmptyStateType {
    /// Get the default icon for this state type
    pub fn icon(&self) -> char {
        match self {
            EmptyStateType::Empty => '📭',
            EmptyStateType::NoResults => '🔍',
            EmptyStateType::Error => '⚠',
            EmptyStateType::NoPermission => '🔒',
            EmptyStateType::Offline => '📡',
            EmptyStateType::FirstUse => '🚀',
        }
    }

    /// Get the accent color for this state type
    pub fn color(&self) -> Color {
        match self {
            EmptyStateType::Empty => Color::rgb(128, 128, 128),
            EmptyStateType::NoResults => Color::rgb(100, 149, 237),
            EmptyStateType::Error => Color::rgb(220, 80, 80),
            EmptyStateType::NoPermission => Color::rgb(255, 165, 0),
            EmptyStateType::Offline => Color::rgb(128, 128, 128),
            EmptyStateType::FirstUse => Color::rgb(100, 200, 100),
        }
    }
}

/// Empty state visual variant
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum EmptyStateVariant {
    /// Full display with border (default)
    #[default]
    Full,
    /// Compact inline display
    Compact,
    /// Minimal text-only
    Minimal,
}

/// An empty state widget for no-data scenarios
///
/// Displays a consistent, helpful message when there's no content to show.
pub struct EmptyState {
    /// Primary message/title
    title: String,
    /// Optional description text
    description: Option<String>,
    /// State type
    state_type: EmptyStateType,
    /// Visual variant
    variant: EmptyStateVariant,
    /// Show icon
    show_icon: bool,
    /// Custom icon override
    custom_icon: Option<char>,
    /// Optional action button text
    action: Option<String>,
    /// Widget state
    state: WidgetState,
    /// Widget properties
    props: WidgetProps,
}

impl EmptyState {
    /// Create a new empty state with a title
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: None,
            state_type: EmptyStateType::default(),
            variant: EmptyStateVariant::default(),
            show_icon: true,
            custom_icon: None,
            action: None,
            state: WidgetState::new(),
            props: WidgetProps::new(),
        }
    }

    /// Create an empty state for no results
    pub fn no_results(title: impl Into<String>) -> Self {
        Self::new(title).state_type(EmptyStateType::NoResults)
    }

    /// Create an error empty state
    pub fn error(title: impl Into<String>) -> Self {
        Self::new(title).state_type(EmptyStateType::Error)
    }

    /// Create a no permission empty state
    pub fn no_permission(title: impl Into<String>) -> Self {
        Self::new(title).state_type(EmptyStateType::NoPermission)
    }

    /// Create an offline empty state
    pub fn offline(title: impl Into<String>) -> Self {
        Self::new(title).state_type(EmptyStateType::Offline)
    }

    /// Create a first-use empty state
    pub fn first_use(title: impl Into<String>) -> Self {
        Self::new(title).state_type(EmptyStateType::FirstUse)
    }

    /// Set the description text
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the state type
    pub fn state_type(mut self, state_type: EmptyStateType) -> Self {
        self.state_type = state_type;
        self
    }

    /// Set the visual variant
    pub fn variant(mut self, variant: EmptyStateVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Show/hide the icon
    pub fn icon(mut self, show: bool) -> Self {
        self.show_icon = show;
        self
    }

    /// Set a custom icon
    pub fn custom_icon(mut self, icon: char) -> Self {
        self.custom_icon = Some(icon);
        self.show_icon = true;
        self
    }

    /// Set an action button text
    pub fn action(mut self, action: impl Into<String>) -> Self {
        self.action = Some(action.into());
        self
    }

    /// Get the icon to display
    fn get_icon(&self) -> char {
        self.custom_icon.unwrap_or_else(|| self.state_type.icon())
    }

    /// Calculate the height needed for this empty state
    pub fn height(&self) -> u16 {
        match self.variant {
            EmptyStateVariant::Full => {
                let mut h = 5; // icon + title + padding
                if self.description.is_some() {
                    h += 1;
                }
                if self.action.is_some() {
                    h += 2;
                }
                h
            }
            EmptyStateVariant::Compact => {
                let mut h = 3;
                if self.description.is_some() {
                    h += 1;
                }
                h
            }
            EmptyStateVariant::Minimal => 1,
        }
    }
}

impl Default for EmptyState {
    fn default() -> Self {
        Self::new("No items")
    }
}

impl View for EmptyState {
    crate::impl_view_meta!("EmptyState");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 5 || area.height < 1 {
            return;
        }

        match self.variant {
            EmptyStateVariant::Full => self.render_full(ctx),
            EmptyStateVariant::Compact => self.render_compact(ctx),
            EmptyStateVariant::Minimal => self.render_minimal(ctx),
        }
    }
}

impl EmptyState {
    fn render_full(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let accent = self.state_type.color();

        // Calculate vertical centering
        let content_height = self.height();
        let start_y = if area.height > content_height {
            area.y + (area.height - content_height) / 2
        } else {
            area.y
        };

        let mut y = start_y;

        // Icon (centered, large)
        if self.show_icon && y < area.y + area.height {
            let icon = self.get_icon();
            let icon_x = area.x + area.width / 2;
            let mut cell = Cell::new(icon);
            cell.fg = Some(accent);
            ctx.buffer.set(icon_x, y, cell);
            y += 2;
        }

        // Title (centered, bold)
        if y < area.y + area.height {
            let title_len = self.title.chars().count() as u16;
            let title_x = area.x + area.width.saturating_sub(title_len) / 2;
            for (i, ch) in self.title.chars().enumerate() {
                if title_x + i as u16 >= area.x + area.width {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(Color::WHITE);
                cell.modifier |= Modifier::BOLD;
                ctx.buffer.set(title_x + i as u16, y, cell);
            }
            y += 1;
        }

        // Description (centered, dimmed)
        if let Some(ref desc) = self.description {
            if y < area.y + area.height {
                let desc_len = desc.chars().count() as u16;
                let desc_x = area.x + area.width.saturating_sub(desc_len) / 2;
                for (i, ch) in desc.chars().enumerate() {
                    if desc_x + i as u16 >= area.x + area.width {
                        break;
                    }
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(Color::rgb(150, 150, 150));
                    ctx.buffer.set(desc_x + i as u16, y, cell);
                }
                y += 2;
            }
        }

        // Action button (centered)
        if let Some(ref action_text) = self.action {
            if y < area.y + area.height {
                let btn_text = format!("[ {} ]", action_text);
                let btn_len = btn_text.chars().count() as u16;
                let btn_x = area.x + area.width.saturating_sub(btn_len) / 2;
                for (i, ch) in btn_text.chars().enumerate() {
                    if btn_x + i as u16 >= area.x + area.width {
                        break;
                    }
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(accent);
                    ctx.buffer.set(btn_x + i as u16, y, cell);
                }
            }
        }
    }

    fn render_compact(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let accent = self.state_type.color();
        let mut y = area.y;

        // Icon + Title on same line
        let mut x = area.x;
        if self.show_icon {
            let icon = self.get_icon();
            let mut cell = Cell::new(icon);
            cell.fg = Some(accent);
            ctx.buffer.set(x, y, cell);
            x += 2;
        }

        for (i, ch) in self.title.chars().enumerate() {
            if x + i as u16 >= area.x + area.width {
                break;
            }
            let mut cell = Cell::new(ch);
            cell.fg = Some(Color::WHITE);
            cell.modifier |= Modifier::BOLD;
            ctx.buffer.set(x + i as u16, y, cell);
        }
        y += 1;

        // Description
        if let Some(ref desc) = self.description {
            if y < area.y + area.height {
                let desc_x = if self.show_icon { area.x + 2 } else { area.x };
                for (i, ch) in desc.chars().enumerate() {
                    if desc_x + i as u16 >= area.x + area.width {
                        break;
                    }
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(Color::rgb(150, 150, 150));
                    ctx.buffer.set(desc_x + i as u16, y, cell);
                }
                y += 1;
            }
        }

        // Action
        if let Some(ref action_text) = self.action {
            if y < area.y + area.height {
                let action_x = if self.show_icon { area.x + 2 } else { area.x };
                let btn_text = format!("[{}]", action_text);
                for (i, ch) in btn_text.chars().enumerate() {
                    if action_x + i as u16 >= area.x + area.width {
                        break;
                    }
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(accent);
                    ctx.buffer.set(action_x + i as u16, y, cell);
                }
            }
        }
    }

    fn render_minimal(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let accent = self.state_type.color();
        let mut x = area.x;

        // Icon
        if self.show_icon {
            let icon = self.get_icon();
            let mut cell = Cell::new(icon);
            cell.fg = Some(accent);
            ctx.buffer.set(x, area.y, cell);
            x += 2;
        }

        // Title
        for (i, ch) in self.title.chars().enumerate() {
            if x + i as u16 >= area.x + area.width {
                break;
            }
            let mut cell = Cell::new(ch);
            cell.fg = Some(Color::rgb(150, 150, 150));
            ctx.buffer.set(x + i as u16, area.y, cell);
        }
    }
}

impl_styled_view!(EmptyState);
impl_state_builders!(EmptyState);
impl_props_builders!(EmptyState);

/// Helper function to create an EmptyState
pub fn empty_state(title: impl Into<String>) -> EmptyState {
    EmptyState::new(title)
}

/// Helper function to create a no-results EmptyState
pub fn no_results(title: impl Into<String>) -> EmptyState {
    EmptyState::no_results(title)
}

/// Helper function to create an error EmptyState
pub fn empty_error(title: impl Into<String>) -> EmptyState {
    EmptyState::error(title)
}

/// Helper function to create a first-use EmptyState
pub fn first_use(title: impl Into<String>) -> EmptyState {
    EmptyState::first_use(title)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    #[test]
    fn test_empty_state_new() {
        let es = EmptyState::new("No items");
        assert_eq!(es.title, "No items");
        assert_eq!(es.state_type, EmptyStateType::Empty);
        assert!(es.description.is_none());
        assert!(es.action.is_none());
        assert!(es.show_icon);
    }

    #[test]
    fn test_empty_state_builders() {
        let es = EmptyState::new("No results")
            .description("Try a different search")
            .state_type(EmptyStateType::NoResults)
            .variant(EmptyStateVariant::Compact)
            .action("Clear");

        assert_eq!(es.title, "No results");
        assert_eq!(es.description, Some("Try a different search".to_string()));
        assert_eq!(es.state_type, EmptyStateType::NoResults);
        assert_eq!(es.variant, EmptyStateVariant::Compact);
        assert_eq!(es.action, Some("Clear".to_string()));
    }

    #[test]
    fn test_empty_state_type_helpers() {
        assert_eq!(
            EmptyState::no_results("msg").state_type,
            EmptyStateType::NoResults
        );
        assert_eq!(EmptyState::error("msg").state_type, EmptyStateType::Error);
        assert_eq!(
            EmptyState::no_permission("msg").state_type,
            EmptyStateType::NoPermission
        );
        assert_eq!(
            EmptyState::offline("msg").state_type,
            EmptyStateType::Offline
        );
        assert_eq!(
            EmptyState::first_use("msg").state_type,
            EmptyStateType::FirstUse
        );
    }

    #[test]
    fn test_empty_state_type_icons() {
        assert_eq!(EmptyStateType::Empty.icon(), '📭');
        assert_eq!(EmptyStateType::NoResults.icon(), '🔍');
        assert_eq!(EmptyStateType::Error.icon(), '⚠');
        assert_eq!(EmptyStateType::NoPermission.icon(), '🔒');
        assert_eq!(EmptyStateType::Offline.icon(), '📡');
        assert_eq!(EmptyStateType::FirstUse.icon(), '🚀');
    }

    #[test]
    fn test_empty_state_custom_icon() {
        let es = EmptyState::new("Test").custom_icon('★');
        assert_eq!(es.get_icon(), '★');
        assert!(es.show_icon);
    }

    #[test]
    fn test_empty_state_height() {
        let minimal = EmptyState::new("msg").variant(EmptyStateVariant::Minimal);
        assert_eq!(minimal.height(), 1);

        let compact = EmptyState::new("msg").variant(EmptyStateVariant::Compact);
        assert_eq!(compact.height(), 3);

        let compact_desc = EmptyState::new("msg")
            .variant(EmptyStateVariant::Compact)
            .description("desc");
        assert_eq!(compact_desc.height(), 4);

        let full = EmptyState::new("msg").variant(EmptyStateVariant::Full);
        assert_eq!(full.height(), 5);

        let full_with_action = EmptyState::new("msg")
            .variant(EmptyStateVariant::Full)
            .action("Click");
        assert_eq!(full_with_action.height(), 7);
    }

    #[test]
    fn test_empty_state_render_full() {
        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let es = EmptyState::new("No items yet")
            .description("Create your first item")
            .action("Create");
        es.render(&mut ctx);

        // Smoke test - ensure it doesn't panic
    }

    #[test]
    fn test_empty_state_render_compact() {
        let mut buffer = Buffer::new(40, 5);
        let area = Rect::new(0, 0, 40, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let es = EmptyState::new("No results")
            .variant(EmptyStateVariant::Compact)
            .description("Try again");
        es.render(&mut ctx);

        // Check icon is rendered
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '📭');
    }

    #[test]
    fn test_empty_state_render_minimal() {
        let mut buffer = Buffer::new(40, 1);
        let area = Rect::new(0, 0, 40, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let es = EmptyState::new("Empty")
            .variant(EmptyStateVariant::Minimal)
            .state_type(EmptyStateType::NoResults);
        es.render(&mut ctx);

        // Check icon
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '🔍');
    }

    #[test]
    fn test_empty_state_render_no_icon() {
        let mut buffer = Buffer::new(40, 5);
        let area = Rect::new(0, 0, 40, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let es = EmptyState::new("No icon")
            .variant(EmptyStateVariant::Compact)
            .icon(false);
        es.render(&mut ctx);

        // First char should be 'N' from title, not icon
        assert_eq!(buffer.get(0, 0).unwrap().symbol, 'N');
    }

    #[test]
    fn test_empty_state_helpers() {
        let es = empty_state("msg");
        assert_eq!(es.title, "msg");

        let nr = no_results("search");
        assert_eq!(nr.state_type, EmptyStateType::NoResults);

        let err = empty_error("error");
        assert_eq!(err.state_type, EmptyStateType::Error);

        let fu = first_use("welcome");
        assert_eq!(fu.state_type, EmptyStateType::FirstUse);
    }

    #[test]
    fn test_empty_state_default() {
        let es = EmptyState::default();
        assert_eq!(es.title, "No items");
    }
}
