//! Tag/Chip widget for labels and categories

use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// Tag style variant
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TagStyle {
    /// Filled background (default)
    #[default]
    Filled,
    /// Outlined with border
    Outlined,
    /// Subtle/light background
    Subtle,
}

/// A tag/chip widget for categories and labels
///
/// # Example
///
/// ```rust,ignore
/// use revue::prelude::*;
///
/// hstack()
///     .child(tag("Rust").color(Color::BLUE))
///     .child(tag("TUI").outlined())
///     .child(tag("Framework").closable())
/// ```
pub struct Tag {
    /// Label text
    text: String,
    /// Color
    color: Color,
    /// Text color (auto-calculated if not set)
    text_color: Option<Color>,
    /// Style
    style: TagStyle,
    /// Is closable (shows x)
    closable: bool,
    /// Icon before text
    icon: Option<char>,
    /// Is selected/active
    selected: bool,
    /// Is disabled
    disabled: bool,
    /// Widget props for CSS integration
    props: WidgetProps,
}

impl Tag {
    /// Create a new tag
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            color: Color::rgb(80, 80, 80),
            text_color: None,
            style: TagStyle::Filled,
            closable: false,
            icon: None,
            selected: false,
            disabled: false,
            props: WidgetProps::new(),
        }
    }

    /// Set color
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Set text color
    pub fn text_color(mut self, color: Color) -> Self {
        self.text_color = Some(color);
        self
    }

    /// Set style
    pub fn style(mut self, style: TagStyle) -> Self {
        self.style = style;
        self
    }

    /// Outlined style shorthand
    pub fn outlined(mut self) -> Self {
        self.style = TagStyle::Outlined;
        self
    }

    /// Subtle style shorthand
    pub fn subtle(mut self) -> Self {
        self.style = TagStyle::Subtle;
        self
    }

    /// Make closable
    pub fn closable(mut self) -> Self {
        self.closable = true;
        self
    }

    /// Set icon
    pub fn icon(mut self, icon: char) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Mark as selected
    pub fn selected(mut self) -> Self {
        self.selected = true;
        self
    }

    /// Mark as disabled
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    /// Blue color preset
    pub fn blue(mut self) -> Self {
        self.color = Color::rgb(60, 120, 200);
        self
    }

    /// Green color preset
    pub fn green(mut self) -> Self {
        self.color = Color::rgb(40, 160, 80);
        self
    }

    /// Red color preset
    pub fn red(mut self) -> Self {
        self.color = Color::rgb(200, 60, 60);
        self
    }

    /// Yellow color preset
    pub fn yellow(mut self) -> Self {
        self.color = Color::rgb(200, 180, 40);
        self
    }

    /// Purple color preset
    pub fn purple(mut self) -> Self {
        self.color = Color::rgb(140, 80, 180);
        self
    }

    /// Get effective colors
    fn effective_colors(&self) -> (Option<Color>, Color) {
        let text_color = self.text_color.unwrap_or(Color::WHITE);

        if self.disabled {
            return (Some(Color::rgb(60, 60, 60)), Color::rgb(120, 120, 120));
        }

        match self.style {
            TagStyle::Filled => (Some(self.color), text_color),
            TagStyle::Outlined => (None, self.color),
            TagStyle::Subtle => {
                // Lighten the color for background
                let light_bg = Color::rgb(
                    self.color.r.saturating_add(180),
                    self.color.g.saturating_add(180),
                    self.color.b.saturating_add(180),
                );
                (Some(light_bg), self.color)
            }
        }
    }
}

impl Default for Tag {
    fn default() -> Self {
        Self::new("")
    }
}

impl View for Tag {
    crate::impl_view_meta!("Tag");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let (bg, fg) = self.effective_colors();

        let mut content = String::new();

        // Icon
        if let Some(icon) = self.icon {
            content.push(icon);
            content.push(' ');
        }

        // Text
        content.push_str(&self.text);

        // Close button
        if self.closable {
            content.push_str(" ×");
        }

        let _text_len = content.chars().count() as u16;

        // Border characters for outlined
        let (left_char, right_char) = match self.style {
            TagStyle::Outlined => ('⟨', '⟩'),
            _ => (' ', ' '),
        };

        // Render
        let mut x = area.x;

        // Left padding/border
        let mut left = Cell::new(left_char);
        if let Some(bg_color) = bg {
            left.bg = Some(bg_color);
        }
        left.fg = Some(fg);
        ctx.buffer.set(x, area.y, left);
        x += 1;

        // Content
        for ch in content.chars() {
            if x >= area.x + area.width - 1 {
                break;
            }
            let mut cell = Cell::new(ch);
            cell.fg = Some(fg);
            if let Some(bg_color) = bg {
                cell.bg = Some(bg_color);
            }
            if self.selected {
                cell.modifier |= Modifier::BOLD;
            }
            if self.disabled {
                cell.modifier |= Modifier::DIM;
            }
            ctx.buffer.set(x, area.y, cell);
            x += 1;
        }

        // Right padding/border
        if x < area.x + area.width {
            let mut right = Cell::new(right_char);
            if let Some(bg_color) = bg {
                right.bg = Some(bg_color);
            }
            right.fg = Some(fg);
            ctx.buffer.set(x, area.y, right);
        }
    }
}

impl_styled_view!(Tag);
impl_props_builders!(Tag);

/// Create a new tag
pub fn tag(text: impl Into<String>) -> Tag {
    Tag::new(text)
}

/// Create a new chip (alias for tag)
pub fn chip(text: impl Into<String>) -> Tag {
    Tag::new(text)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    #[test]
    fn test_tag_new() {
        let t = Tag::new("Rust");
        assert_eq!(t.text, "Rust");
        assert!(!t.closable);
    }

    #[test]
    fn test_tag_styles() {
        let t = tag("Test").outlined();
        assert_eq!(t.style, TagStyle::Outlined);

        let t = tag("Test").subtle();
        assert_eq!(t.style, TagStyle::Subtle);
    }

    #[test]
    fn test_tag_colors() {
        let t = tag("Test").blue();
        assert_eq!(t.color, Color::rgb(60, 120, 200));

        let t = tag("Test").red();
        assert_eq!(t.color, Color::rgb(200, 60, 60));
    }

    #[test]
    fn test_tag_closable() {
        let t = tag("Test").closable();
        assert!(t.closable);
    }

    #[test]
    fn test_tag_icon() {
        let t = tag("Rust").icon('🦀');
        assert_eq!(t.icon, Some('🦀'));
    }

    #[test]
    fn test_tag_selected_disabled() {
        let t = tag("Test").selected().disabled();
        assert!(t.selected);
        assert!(t.disabled);
    }

    #[test]
    fn test_tag_render() {
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let t = tag("Rust").blue();
        t.render(&mut ctx);

        let text: String = (0..20)
            .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
            .collect();
        assert!(text.contains("Rust"));
    }

    #[test]
    fn test_helper_functions() {
        let t = tag("A");
        assert_eq!(t.text, "A");

        let c = chip("B");
        assert_eq!(c.text, "B");
    }

    // =========================================================================
    // TagStyle enum tests
    // =========================================================================

    #[test]
    fn test_tag_style_default() {
        let style = TagStyle::default();
        assert_eq!(style, TagStyle::Filled);
    }

    #[test]
    fn test_tag_style_clone() {
        let style = TagStyle::Outlined;
        let cloned = style.clone();
        assert_eq!(style, cloned);
    }

    #[test]
    fn test_tag_style_copy() {
        let style1 = TagStyle::Subtle;
        let style2 = style1;
        assert_eq!(style1, TagStyle::Subtle);
        assert_eq!(style2, TagStyle::Subtle);
    }

    #[test]
    fn test_tag_style_partial_eq() {
        assert_eq!(TagStyle::Filled, TagStyle::Filled);
        assert_ne!(TagStyle::Filled, TagStyle::Outlined);
    }

    #[test]
    fn test_tag_style_debug() {
        let style = TagStyle::Subtle;
        assert!(format!("{:?}", style).contains("Subtle"));
    }

    // =========================================================================
    // Tag builder tests
    // =========================================================================

    #[test]
    fn test_tag_new_default_values() {
        let t = Tag::new("Test");
        assert_eq!(t.text, "Test");
        assert_eq!(t.color, Color::rgb(80, 80, 80));
        assert_eq!(t.style, TagStyle::Filled);
        assert!(!t.closable);
        assert!(t.icon.is_none());
        assert!(!t.selected);
        assert!(!t.disabled);
        assert!(t.text_color.is_none());
    }

    #[test]
    fn test_tag_color_builder() {
        let t = Tag::new("Test").color(Color::MAGENTA);
        assert_eq!(t.color, Color::MAGENTA);
    }

    #[test]
    fn test_tag_text_color() {
        let t = Tag::new("Test").text_color(Color::CYAN);
        assert_eq!(t.text_color, Some(Color::CYAN));
    }

    #[test]
    fn test_tag_style_builder() {
        let t = Tag::new("Test").style(TagStyle::Outlined);
        assert_eq!(t.style, TagStyle::Outlined);
    }

    #[test]
    fn test_tag_green() {
        let t = tag("Test").green();
        assert_eq!(t.color, Color::rgb(40, 160, 80));
    }

    #[test]
    fn test_tag_yellow() {
        let t = tag("Test").yellow();
        assert_eq!(t.color, Color::rgb(200, 180, 40));
    }

    #[test]
    fn test_tag_purple() {
        let t = tag("Test").purple();
        assert_eq!(t.color, Color::rgb(140, 80, 180));
    }

    // =========================================================================
    // Tag Default trait tests
    // =========================================================================

    #[test]
    fn test_tag_default() {
        let t = Tag::default();
        assert_eq!(t.text, "");
        assert_eq!(t.style, TagStyle::Filled);
    }

    // =========================================================================
    // Tag builder chain tests
    // =========================================================================

    #[test]
    fn test_tag_builder_chain() {
        let t = Tag::new("Chained")
            .blue()
            .outlined()
            .closable()
            .icon('T')
            .selected();
        assert_eq!(t.text, "Chained");
        assert_eq!(t.style, TagStyle::Outlined);
        assert!(t.closable);
        assert_eq!(t.icon, Some('T'));
        assert!(t.selected);
    }

    #[test]
    fn test_tag_style_chain() {
        let t = Tag::new("Test")
            .style(TagStyle::Subtle)
            .color(Color::RED)
            .text_color(Color::WHITE);
        assert_eq!(t.style, TagStyle::Subtle);
        assert_eq!(t.color, Color::RED);
        assert_eq!(t.text_color, Some(Color::WHITE));
    }

    // =========================================================================
    // effective_colors edge cases
    // =========================================================================

    #[test]
    fn test_effective_colors_disabled() {
        let t = Tag::new("Test").disabled();
        let (bg, fg) = t.effective_colors();
        assert_eq!(bg, Some(Color::rgb(60, 60, 60)));
        assert_eq!(fg, Color::rgb(120, 120, 120));
    }

    #[test]
    fn test_effective_colors_outlined() {
        let t = Tag::new("Test").outlined().color(Color::BLUE);
        let (bg, fg) = t.effective_colors();
        assert!(bg.is_none());
        assert_eq!(fg, Color::BLUE);
    }

    #[test]
    fn test_effective_colors_filled() {
        let t = Tag::new("Test").color(Color::GREEN);
        let (bg, fg) = t.effective_colors();
        assert_eq!(bg, Some(Color::GREEN));
        assert_eq!(fg, Color::WHITE);
    }

    #[test]
    fn test_effective_colors_subtle() {
        let t = Tag::new("Test").subtle().color(Color::rgb(10, 20, 30));
        let (bg, fg) = t.effective_colors();
        // Subtle lightens the color
        assert!(bg.is_some());
        assert_eq!(fg, Color::rgb(10, 20, 30));
    }

    #[test]
    fn test_effective_colors_with_text_color() {
        let t = Tag::new("Test").text_color(Color::CYAN);
        let (_bg, fg) = t.effective_colors();
        assert_eq!(fg, Color::CYAN);
    }
}
