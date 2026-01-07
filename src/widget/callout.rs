//! Callout widget for highlighting important information blocks
//!
//! Provides consistent styling for notes, tips, warnings, and other important
//! information in documentation and applications.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::widget::{Callout, CalloutType, callout};
//!
//! // Basic note callout
//! Callout::note("This is important information.");
//!
//! // Warning with title
//! Callout::warning("Proceed with caution")
//!     .title("Warning");
//!
//! // Collapsible tip
//! callout("Click to expand details")
//!     .callout_type(CalloutType::Tip)
//!     .collapsible(true);
//!
//! // Custom icon
//! Callout::info("Custom icon example")
//!     .custom_icon('💡');
//! ```

use super::traits::{RenderContext, View, WidgetProps, WidgetState};
use crate::event::Key;
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::{impl_props_builders, impl_state_builders, impl_styled_view};
use unicode_width::UnicodeWidthChar;

/// Callout type determines the styling and default icon
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CalloutType {
    /// Informational note (blue) - general information
    #[default]
    Note,
    /// Helpful tip (green) - suggestions and best practices
    Tip,
    /// Important notice (purple) - highlights key information
    Important,
    /// Warning message (yellow/orange) - potential issues
    Warning,
    /// Danger/caution (red) - critical warnings
    Danger,
    /// Info message (cyan) - supplementary information
    Info,
}

impl CalloutType {
    /// Get the default icon for this callout type
    pub fn icon(&self) -> char {
        match self {
            CalloutType::Note => '📝',
            CalloutType::Tip => '💡',
            CalloutType::Important => '❗',
            CalloutType::Warning => '⚠',
            CalloutType::Danger => '🔴',
            CalloutType::Info => 'ℹ',
        }
    }

    /// Get the accent color for this callout type
    pub fn accent_color(&self) -> Color {
        match self {
            CalloutType::Note => Color::rgb(59, 130, 246), // Blue
            CalloutType::Tip => Color::rgb(34, 197, 94),   // Green
            CalloutType::Important => Color::rgb(168, 85, 247), // Purple
            CalloutType::Warning => Color::rgb(234, 179, 8), // Yellow
            CalloutType::Danger => Color::rgb(239, 68, 68), // Red
            CalloutType::Info => Color::rgb(6, 182, 212),  // Cyan
        }
    }

    /// Get the background color for this callout type
    pub fn bg_color(&self) -> Color {
        match self {
            CalloutType::Note => Color::rgb(23, 37, 53), // Dark blue
            CalloutType::Tip => Color::rgb(20, 40, 28),  // Dark green
            CalloutType::Important => Color::rgb(35, 25, 50), // Dark purple
            CalloutType::Warning => Color::rgb(45, 38, 15), // Dark yellow
            CalloutType::Danger => Color::rgb(50, 20, 20), // Dark red
            CalloutType::Info => Color::rgb(15, 40, 45), // Dark cyan
        }
    }

    /// Get the title text color for this callout type
    pub fn title_color(&self) -> Color {
        self.accent_color()
    }

    /// Get the default title for this callout type
    pub fn default_title(&self) -> &'static str {
        match self {
            CalloutType::Note => "Note",
            CalloutType::Tip => "Tip",
            CalloutType::Important => "Important",
            CalloutType::Warning => "Warning",
            CalloutType::Danger => "Danger",
            CalloutType::Info => "Info",
        }
    }
}

/// Visual variant for the callout
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CalloutVariant {
    /// Filled background with accent border
    #[default]
    Filled,
    /// Only left border accent, no background
    LeftBorder,
    /// Minimal style with just icon and text
    Minimal,
}

/// A callout widget for highlighting important information
///
/// Provides predefined styles for notes, tips, warnings, and other
/// important information blocks commonly used in documentation.
#[derive(Clone)]
pub struct Callout {
    /// Main content text
    content: String,
    /// Optional title (defaults to type name)
    title: Option<String>,
    /// Callout type
    callout_type: CalloutType,
    /// Visual variant
    variant: CalloutVariant,
    /// Show icon
    show_icon: bool,
    /// Custom icon override
    custom_icon: Option<char>,
    /// Whether the callout is collapsible
    collapsible: bool,
    /// Whether the callout is expanded (when collapsible)
    expanded: bool,
    /// Icon when collapsed
    collapsed_icon: char,
    /// Icon when expanded
    expanded_icon: char,
    /// Widget state
    state: WidgetState,
    /// Widget properties
    props: WidgetProps,
}

impl Callout {
    /// Create a new callout with content
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            title: None,
            callout_type: CalloutType::default(),
            variant: CalloutVariant::default(),
            show_icon: true,
            custom_icon: None,
            collapsible: false,
            expanded: true,
            collapsed_icon: '▶',
            expanded_icon: '▼',
            state: WidgetState::new(),
            props: WidgetProps::new(),
        }
    }

    /// Create a note callout
    pub fn note(content: impl Into<String>) -> Self {
        Self::new(content).callout_type(CalloutType::Note)
    }

    /// Create a tip callout
    pub fn tip(content: impl Into<String>) -> Self {
        Self::new(content).callout_type(CalloutType::Tip)
    }

    /// Create an important callout
    pub fn important(content: impl Into<String>) -> Self {
        Self::new(content).callout_type(CalloutType::Important)
    }

    /// Create a warning callout
    pub fn warning(content: impl Into<String>) -> Self {
        Self::new(content).callout_type(CalloutType::Warning)
    }

    /// Create a danger callout
    pub fn danger(content: impl Into<String>) -> Self {
        Self::new(content).callout_type(CalloutType::Danger)
    }

    /// Create an info callout
    pub fn info(content: impl Into<String>) -> Self {
        Self::new(content).callout_type(CalloutType::Info)
    }

    /// Set the callout type
    pub fn callout_type(mut self, callout_type: CalloutType) -> Self {
        self.callout_type = callout_type;
        self
    }

    /// Set the title (overrides default type title)
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the visual variant
    pub fn variant(mut self, variant: CalloutVariant) -> Self {
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

    /// Make the callout collapsible
    pub fn collapsible(mut self, collapsible: bool) -> Self {
        self.collapsible = collapsible;
        self
    }

    /// Set the expanded state (for collapsible callouts)
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    /// Set custom collapse/expand icons
    pub fn collapse_icons(mut self, collapsed: char, expanded: char) -> Self {
        self.collapsed_icon = collapsed;
        self.expanded_icon = expanded;
        self
    }

    /// Toggle expanded state
    pub fn toggle(&mut self) {
        if self.collapsible {
            self.expanded = !self.expanded;
        }
    }

    /// Expand the callout
    pub fn expand(&mut self) {
        self.expanded = true;
    }

    /// Collapse the callout
    pub fn collapse(&mut self) {
        self.expanded = false;
    }

    /// Check if expanded
    pub fn is_expanded(&self) -> bool {
        self.expanded
    }

    /// Check if collapsible
    pub fn is_collapsible(&self) -> bool {
        self.collapsible
    }

    /// Set expanded state mutably
    pub fn set_expanded(&mut self, expanded: bool) {
        self.expanded = expanded;
    }

    /// Get the icon to display
    fn get_icon(&self) -> char {
        self.custom_icon.unwrap_or_else(|| self.callout_type.icon())
    }

    /// Get the collapse icon based on state
    fn collapse_icon(&self) -> char {
        if self.expanded {
            self.expanded_icon
        } else {
            self.collapsed_icon
        }
    }

    /// Get the title to display
    fn get_title(&self) -> &str {
        self.title
            .as_deref()
            .unwrap_or_else(|| self.callout_type.default_title())
    }

    /// Calculate the height needed for this callout
    pub fn height(&self) -> u16 {
        if self.collapsible && !self.expanded {
            return 1; // Just header
        }

        let content_lines = self.content.lines().count().max(1) as u16;

        match self.variant {
            CalloutVariant::Filled => {
                // top border + title + content + bottom border
                2 + content_lines + 1
            }
            CalloutVariant::LeftBorder => {
                // title + content
                1 + content_lines
            }
            CalloutVariant::Minimal => {
                // title + content (no borders)
                1 + content_lines
            }
        }
    }

    /// Handle keyboard input
    ///
    /// Returns `true` if the key was handled.
    pub fn handle_key(&mut self, key: &Key) -> bool {
        if !self.collapsible || self.state.disabled {
            return false;
        }

        match key {
            Key::Enter | Key::Char(' ') => {
                self.toggle();
                true
            }
            Key::Right | Key::Char('l') => {
                self.expand();
                true
            }
            Key::Left | Key::Char('h') => {
                self.collapse();
                true
            }
            _ => false,
        }
    }
}

impl Default for Callout {
    fn default() -> Self {
        Self::new("Callout")
    }
}

impl View for Callout {
    crate::impl_view_meta!("Callout");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 5 || area.height < 1 {
            return;
        }

        let accent_color = self.callout_type.accent_color();
        let bg_color = self.callout_type.bg_color();
        let title_color = self.callout_type.title_color();

        match self.variant {
            CalloutVariant::Filled => {
                self.render_filled(ctx, accent_color, bg_color, title_color);
            }
            CalloutVariant::LeftBorder => {
                self.render_left_border(ctx, accent_color, title_color);
            }
            CalloutVariant::Minimal => {
                self.render_minimal(ctx, accent_color, title_color);
            }
        }
    }
}

impl Callout {
    fn render_filled(
        &self,
        ctx: &mut RenderContext,
        accent_color: Color,
        bg_color: Color,
        title_color: Color,
    ) {
        let area = ctx.area;

        // Fill background
        for y in area.y..area.y + area.height {
            for x in area.x..area.x + area.width {
                let mut cell = Cell::new(' ');
                cell.bg = Some(bg_color);
                ctx.buffer.set(x, y, cell);
            }
        }

        // Draw left accent border
        for y in area.y..area.y + area.height {
            let mut cell = Cell::new('┃');
            cell.fg = Some(accent_color);
            cell.bg = Some(bg_color);
            ctx.buffer.set(area.x, y, cell);
        }

        // Header line
        let mut x = area.x + 2;
        let y = area.y;

        // Collapse icon (if collapsible)
        if self.collapsible {
            let mut icon_cell = Cell::new(self.collapse_icon());
            icon_cell.fg = Some(title_color);
            icon_cell.bg = Some(bg_color);
            ctx.buffer.set(x, y, icon_cell);
            x += 2;
        }

        // Type icon
        if self.show_icon {
            let icon = self.get_icon();
            let mut icon_cell = Cell::new(icon);
            icon_cell.fg = Some(accent_color);
            icon_cell.bg = Some(bg_color);
            ctx.buffer.set(x, y, icon_cell);
            x += 2;
        }

        // Title
        let title = self.get_title();
        let max_title_x = area.x + area.width - 1;
        for ch in title.chars() {
            let char_width = ch.width().unwrap_or(0) as u16;
            if char_width == 0 {
                continue;
            }
            if x + char_width > max_title_x {
                break;
            }
            let mut cell = Cell::new(ch);
            cell.fg = Some(title_color);
            cell.bg = Some(bg_color);
            cell.modifier |= Modifier::BOLD;
            ctx.buffer.set(x, y, cell);
            // Set continuation cells for wide characters
            for i in 1..char_width {
                ctx.buffer.set(x + i, y, Cell::continuation());
            }
            x += char_width;
        }

        // Content (if expanded or not collapsible)
        if !self.collapsible || self.expanded {
            let content_x = area.x + 2;
            let content_width = area.width.saturating_sub(3);

            for (i, line) in self.content.lines().enumerate() {
                let line_y = area.y + 1 + i as u16;
                if line_y >= area.y + area.height {
                    break;
                }

                let mut offset = 0u16;
                for ch in line.chars() {
                    let char_width = ch.width().unwrap_or(0) as u16;
                    if char_width == 0 {
                        continue;
                    }
                    if offset + char_width > content_width {
                        break;
                    }
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(Color::rgb(200, 200, 200));
                    cell.bg = Some(bg_color);
                    ctx.buffer.set(content_x + offset, line_y, cell);
                    for i in 1..char_width {
                        ctx.buffer
                            .set(content_x + offset + i, line_y, Cell::continuation());
                    }
                    offset += char_width;
                }
            }
        }
    }

    fn render_left_border(&self, ctx: &mut RenderContext, accent_color: Color, title_color: Color) {
        let area = ctx.area;

        // Draw left accent border
        for y in area.y..area.y + area.height {
            let mut cell = Cell::new('┃');
            cell.fg = Some(accent_color);
            ctx.buffer.set(area.x, y, cell);
        }

        // Header line
        let mut x = area.x + 2;
        let y = area.y;

        // Collapse icon (if collapsible)
        if self.collapsible {
            let mut icon_cell = Cell::new(self.collapse_icon());
            icon_cell.fg = Some(title_color);
            ctx.buffer.set(x, y, icon_cell);
            x += 2;
        }

        // Type icon
        if self.show_icon {
            let icon = self.get_icon();
            let mut icon_cell = Cell::new(icon);
            icon_cell.fg = Some(accent_color);
            ctx.buffer.set(x, y, icon_cell);
            x += 2;
        }

        // Title
        let title = self.get_title();
        let max_title_x = area.x + area.width;
        for ch in title.chars() {
            let char_width = ch.width().unwrap_or(0) as u16;
            if char_width == 0 {
                continue;
            }
            if x + char_width > max_title_x {
                break;
            }
            let mut cell = Cell::new(ch);
            cell.fg = Some(title_color);
            cell.modifier |= Modifier::BOLD;
            ctx.buffer.set(x, y, cell);
            for i in 1..char_width {
                ctx.buffer.set(x + i, y, Cell::continuation());
            }
            x += char_width;
        }

        // Content (if expanded or not collapsible)
        if !self.collapsible || self.expanded {
            let content_x = area.x + 2;
            let content_width = area.width.saturating_sub(3);

            for (i, line) in self.content.lines().enumerate() {
                let line_y = area.y + 1 + i as u16;
                if line_y >= area.y + area.height {
                    break;
                }

                let mut offset = 0u16;
                for ch in line.chars() {
                    let char_width = ch.width().unwrap_or(0) as u16;
                    if char_width == 0 {
                        continue;
                    }
                    if offset + char_width > content_width {
                        break;
                    }
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(Color::rgb(180, 180, 180));
                    ctx.buffer.set(content_x + offset, line_y, cell);
                    for i in 1..char_width {
                        ctx.buffer
                            .set(content_x + offset + i, line_y, Cell::continuation());
                    }
                    offset += char_width;
                }
            }
        }
    }

    fn render_minimal(&self, ctx: &mut RenderContext, accent_color: Color, title_color: Color) {
        let area = ctx.area;

        // Header line
        let mut x = area.x;
        let y = area.y;

        // Collapse icon (if collapsible)
        if self.collapsible {
            let mut icon_cell = Cell::new(self.collapse_icon());
            icon_cell.fg = Some(title_color);
            ctx.buffer.set(x, y, icon_cell);
            x += 2;
        }

        // Type icon
        if self.show_icon {
            let icon = self.get_icon();
            let mut icon_cell = Cell::new(icon);
            icon_cell.fg = Some(accent_color);
            ctx.buffer.set(x, y, icon_cell);
            x += 2;
        }

        // Title
        let title = self.get_title();
        let max_title_x = area.x + area.width;
        for ch in title.chars() {
            let char_width = ch.width().unwrap_or(0) as u16;
            if char_width == 0 {
                continue;
            }
            if x + char_width > max_title_x {
                break;
            }
            let mut cell = Cell::new(ch);
            cell.fg = Some(title_color);
            cell.modifier |= Modifier::BOLD;
            ctx.buffer.set(x, y, cell);
            for i in 1..char_width {
                ctx.buffer.set(x + i, y, Cell::continuation());
            }
            x += char_width;
        }

        // Content (if expanded or not collapsible)
        if !self.collapsible || self.expanded {
            let content_x = if self.show_icon { area.x + 2 } else { area.x };
            let content_width = area
                .width
                .saturating_sub(if self.show_icon { 2 } else { 0 });

            for (i, line) in self.content.lines().enumerate() {
                let line_y = area.y + 1 + i as u16;
                if line_y >= area.y + area.height {
                    break;
                }

                let mut offset = 0u16;
                for ch in line.chars() {
                    let char_width = ch.width().unwrap_or(0) as u16;
                    if char_width == 0 {
                        continue;
                    }
                    if offset + char_width > content_width {
                        break;
                    }
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(Color::rgb(180, 180, 180));
                    ctx.buffer.set(content_x + offset, line_y, cell);
                    for i in 1..char_width {
                        ctx.buffer
                            .set(content_x + offset + i, line_y, Cell::continuation());
                    }
                    offset += char_width;
                }
            }
        }
    }
}

impl_styled_view!(Callout);
impl_state_builders!(Callout);
impl_props_builders!(Callout);

/// Helper function to create a Callout
pub fn callout(content: impl Into<String>) -> Callout {
    Callout::new(content)
}

/// Helper function to create a note Callout
pub fn note(content: impl Into<String>) -> Callout {
    Callout::note(content)
}

/// Helper function to create a tip Callout
pub fn tip(content: impl Into<String>) -> Callout {
    Callout::tip(content)
}

/// Helper function to create an important Callout
pub fn important(content: impl Into<String>) -> Callout {
    Callout::important(content)
}

/// Helper function to create a warning Callout
pub fn warning_callout(content: impl Into<String>) -> Callout {
    Callout::warning(content)
}

/// Helper function to create a danger Callout
pub fn danger(content: impl Into<String>) -> Callout {
    Callout::danger(content)
}

/// Helper function to create an info Callout
pub fn info_callout(content: impl Into<String>) -> Callout {
    Callout::info(content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    #[test]
    fn test_callout_new() {
        let c = Callout::new("Test content");
        assert_eq!(c.content, "Test content");
        assert_eq!(c.callout_type, CalloutType::Note);
        assert!(c.title.is_none());
        assert!(!c.collapsible);
        assert!(c.expanded);
    }

    #[test]
    fn test_callout_type_helpers() {
        assert_eq!(Callout::note("msg").callout_type, CalloutType::Note);
        assert_eq!(Callout::tip("msg").callout_type, CalloutType::Tip);
        assert_eq!(
            Callout::important("msg").callout_type,
            CalloutType::Important
        );
        assert_eq!(Callout::warning("msg").callout_type, CalloutType::Warning);
        assert_eq!(Callout::danger("msg").callout_type, CalloutType::Danger);
        assert_eq!(Callout::info("msg").callout_type, CalloutType::Info);
    }

    #[test]
    fn test_callout_builders() {
        let c = Callout::new("Content")
            .title("Custom Title")
            .callout_type(CalloutType::Warning)
            .variant(CalloutVariant::LeftBorder)
            .collapsible(true)
            .expanded(false)
            .icon(false);

        assert_eq!(c.title, Some("Custom Title".to_string()));
        assert_eq!(c.callout_type, CalloutType::Warning);
        assert_eq!(c.variant, CalloutVariant::LeftBorder);
        assert!(c.collapsible);
        assert!(!c.expanded);
        assert!(!c.show_icon);
    }

    #[test]
    fn test_callout_toggle() {
        let mut c = Callout::new("Test").collapsible(true);
        assert!(c.is_expanded());

        c.toggle();
        assert!(!c.is_expanded());

        c.toggle();
        assert!(c.is_expanded());
    }

    #[test]
    fn test_callout_toggle_not_collapsible() {
        let mut c = Callout::new("Test").collapsible(false);
        assert!(c.is_expanded());

        c.toggle(); // Should not change
        assert!(c.is_expanded());
    }

    #[test]
    fn test_callout_expand_collapse() {
        let mut c = Callout::new("Test").collapsible(true).expanded(false);

        c.expand();
        assert!(c.is_expanded());

        c.collapse();
        assert!(!c.is_expanded());
    }

    #[test]
    fn test_callout_height() {
        // Collapsed
        let collapsed = Callout::new("Content").collapsible(true).expanded(false);
        assert_eq!(collapsed.height(), 1);

        // Filled with single line content
        let filled = Callout::new("Single line").variant(CalloutVariant::Filled);
        assert_eq!(filled.height(), 4); // border + title + content + border

        // Filled with multi-line content
        let multi = Callout::new("Line 1\nLine 2\nLine 3").variant(CalloutVariant::Filled);
        assert_eq!(multi.height(), 6); // border + title + 3 content lines + border

        // Left border variant
        let left = Callout::new("Content").variant(CalloutVariant::LeftBorder);
        assert_eq!(left.height(), 2); // title + content

        // Minimal variant
        let minimal = Callout::new("Content").variant(CalloutVariant::Minimal);
        assert_eq!(minimal.height(), 2); // title + content
    }

    #[test]
    fn test_callout_handle_key() {
        let mut c = Callout::new("Test").collapsible(true);

        assert!(c.handle_key(&Key::Enter));
        assert!(!c.is_expanded());

        assert!(c.handle_key(&Key::Char(' ')));
        assert!(c.is_expanded());

        assert!(c.handle_key(&Key::Left));
        assert!(!c.is_expanded());

        assert!(c.handle_key(&Key::Right));
        assert!(c.is_expanded());

        assert!(!c.handle_key(&Key::Up)); // Not handled
    }

    #[test]
    fn test_callout_handle_key_not_collapsible() {
        let mut c = Callout::new("Test").collapsible(false);

        assert!(!c.handle_key(&Key::Enter));
        assert!(c.is_expanded()); // Should not change
    }

    #[test]
    fn test_callout_handle_key_disabled() {
        let mut c = Callout::new("Test").collapsible(true).disabled(true);

        assert!(!c.handle_key(&Key::Enter));
        assert!(c.is_expanded());
    }

    #[test]
    fn test_callout_type_icons() {
        assert_eq!(CalloutType::Note.icon(), '📝');
        assert_eq!(CalloutType::Tip.icon(), '💡');
        assert_eq!(CalloutType::Important.icon(), '❗');
        assert_eq!(CalloutType::Warning.icon(), '⚠');
        assert_eq!(CalloutType::Danger.icon(), '🔴');
        assert_eq!(CalloutType::Info.icon(), 'ℹ');
    }

    #[test]
    fn test_callout_type_default_titles() {
        assert_eq!(CalloutType::Note.default_title(), "Note");
        assert_eq!(CalloutType::Tip.default_title(), "Tip");
        assert_eq!(CalloutType::Important.default_title(), "Important");
        assert_eq!(CalloutType::Warning.default_title(), "Warning");
        assert_eq!(CalloutType::Danger.default_title(), "Danger");
        assert_eq!(CalloutType::Info.default_title(), "Info");
    }

    #[test]
    fn test_callout_custom_icon() {
        let c = Callout::new("Test").custom_icon('★');
        assert_eq!(c.get_icon(), '★');
        assert!(c.show_icon);
    }

    #[test]
    fn test_callout_get_title() {
        let default_title = Callout::note("Test");
        assert_eq!(default_title.get_title(), "Note");

        let custom_title = Callout::note("Test").title("Custom");
        assert_eq!(custom_title.get_title(), "Custom");
    }

    #[test]
    fn test_callout_collapse_icons() {
        let c = Callout::new("Test")
            .collapsible(true)
            .collapse_icons('+', '-');

        assert_eq!(c.collapsed_icon, '+');
        assert_eq!(c.expanded_icon, '-');
        assert_eq!(c.collapse_icon(), '-'); // expanded by default
    }

    #[test]
    fn test_callout_render_filled() {
        let mut buffer = Buffer::new(50, 5);
        let area = Rect::new(0, 0, 50, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let c = Callout::note("Test content").variant(CalloutVariant::Filled);
        c.render(&mut ctx);

        // Check left accent border
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '┃');
    }

    #[test]
    fn test_callout_render_left_border() {
        let mut buffer = Buffer::new(50, 3);
        let area = Rect::new(0, 0, 50, 3);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let c = Callout::tip("Test").variant(CalloutVariant::LeftBorder);
        c.render(&mut ctx);

        // Check left accent border
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '┃');
    }

    #[test]
    fn test_callout_render_minimal() {
        let mut buffer = Buffer::new(50, 3);
        let area = Rect::new(0, 0, 50, 3);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let c = Callout::warning("Test").variant(CalloutVariant::Minimal);
        c.render(&mut ctx);

        // Check icon
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '⚠');
    }

    #[test]
    fn test_callout_render_collapsed() {
        let mut buffer = Buffer::new(50, 5);
        let area = Rect::new(0, 0, 50, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let c = Callout::note("Hidden content")
            .collapsible(true)
            .expanded(false)
            .variant(CalloutVariant::Filled);
        c.render(&mut ctx);

        // Only header should be rendered
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '┃');
    }

    #[test]
    fn test_callout_helpers() {
        let c = callout("msg");
        assert_eq!(c.content, "msg");

        let n = note("note");
        assert_eq!(n.callout_type, CalloutType::Note);

        let t = tip("tip");
        assert_eq!(t.callout_type, CalloutType::Tip);

        let i = important("important");
        assert_eq!(i.callout_type, CalloutType::Important);

        let w = warning_callout("warning");
        assert_eq!(w.callout_type, CalloutType::Warning);

        let d = danger("danger");
        assert_eq!(d.callout_type, CalloutType::Danger);

        let info = info_callout("info");
        assert_eq!(info.callout_type, CalloutType::Info);
    }

    #[test]
    fn test_callout_default() {
        let c = Callout::default();
        assert_eq!(c.content, "Callout");
    }
}
