//! Tag/Chip widget for labels and categories

use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::widget::theme::{DARK_GRAY, SEPARATOR_COLOR, SUBTLE_GRAY};
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
#[derive(Clone)]
pub struct Tag {
    /// Label text
    text: String,
    /// Color
    /// The color the builder named, if it named one.
    ///
    /// `None` is not the same as `Some(DARK_GRAY)`. While this was a plain
    /// `Color`, naming the default explicitly was indistinguishable from
    /// saying nothing, so a stylesheet outranked a builder that had spoken.
    color: Option<Color>,
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
            color: None,
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
        self.color = Some(color);
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
        self.color = Some(Color::rgb(60, 120, 200));
        self
    }

    /// Green color preset
    pub fn green(mut self) -> Self {
        self.color = Some(Color::rgb(40, 160, 80));
        self
    }

    /// Red color preset
    pub fn red(mut self) -> Self {
        self.color = Some(Color::rgb(200, 60, 60));
        self
    }

    /// Yellow color preset
    pub fn yellow(mut self) -> Self {
        self.color = Some(Color::rgb(200, 180, 40));
        self
    }

    /// Purple color preset
    pub fn purple(mut self) -> Self {
        self.color = Some(Color::rgb(140, 80, 180));
        self
    }

    /// Get effective colors
    /// The builder's color if it moved off the initial value, else the
    /// stylesheet's, else the initial - see [`RenderContext::color_or`].
    fn effective_colors(&self, ctx: &RenderContext) -> (Option<Color>, Color) {
        let color = self.color.unwrap_or_else(|| ctx.css_color(DARK_GRAY));
        let text_color = self
            .text_color
            .unwrap_or_else(|| ctx.css_color(Color::WHITE));

        if self.disabled {
            return (Some(SEPARATOR_COLOR), SUBTLE_GRAY);
        }

        match self.style {
            TagStyle::Filled => (Some(color), text_color),
            TagStyle::Outlined => (None, color),
            TagStyle::Subtle => {
                // Lighten the color for background
                let light_bg = Color::rgb(
                    color.r.saturating_add(180),
                    color.g.saturating_add(180),
                    color.b.saturating_add(180),
                );
                (Some(light_bg), color)
            }
        }
    }
}

impl Default for Tag {
    fn default() -> Self {
        Self::new("")
    }
}

impl Tag {
    /// The icon, text and close mark, as `render` lays them out.
    fn content(&self) -> String {
        let mut content = String::new();
        if let Some(icon) = self.icon {
            content.push(icon);
            content.push(' ');
        }
        content.push_str(&self.text);
        if self.closable {
            content.push_str(" ×");
        }
        content
    }
}

impl View for Tag {
    crate::impl_view_meta!("Tag");

    /// One row: the content plus an edge column each side.
    fn measure(&self, max_width: u16, max_height: u16) -> Option<(u16, u16)> {
        let content = crate::utils::unicode::display_width(&self.content());
        let width = (content.saturating_add(2)).min(u16::MAX as usize) as u16;
        Some((width.min(max_width), 1.min(max_height)))
    }

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width == 0 || area.height == 0 {
            return;
        }
        let (bg, fg) = self.effective_colors(ctx);

        let content = self.content();

        // Border characters for outlined
        let (left_char, right_char) = match self.style {
            TagStyle::Outlined => ('⟨', '⟩'),
            _ => (' ', ' '),
        };

        // Render
        let mut x: u16 = 0;

        // Left padding/border
        let mut left = Cell::new(left_char);
        if let Some(bg_color) = bg {
            left.bg = Some(bg_color);
        }
        left.fg = Some(fg);
        ctx.set(x, 0, left);
        x += 1;

        // Content, by terminal columns, leaving the last column for the edge
        let (selected, disabled) = (self.selected, self.disabled);
        x += ctx.put_str_with(x, 0, &content, area.width - 1, |ch| {
            let mut cell = Cell::new(ch);
            cell.fg = Some(fg);
            if let Some(bg_color) = bg {
                cell.bg = Some(bg_color);
            }
            if selected {
                cell.modifier |= Modifier::BOLD;
            }
            if disabled {
                cell.modifier |= Modifier::DIM;
            }
            cell
        });

        // Right padding/border
        if x < area.width {
            let mut right = Cell::new(right_char);
            if let Some(bg_color) = bg {
                right.bg = Some(bg_color);
            }
            right.fg = Some(fg);
            ctx.set(x, 0, right);
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
