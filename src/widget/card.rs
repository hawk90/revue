//! Card widget for grouping related content with visual boundaries
//!
//! Cards provide a structured container with optional header, body, and footer sections.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::widget::{Card, card};
//!
//! // Basic card with title
//! Card::new()
//!     .title("User Profile")
//!     .body(user_info_widget);
//!
//! // Card with header, body, and footer
//! card()
//!     .title("Settings")
//!     .subtitle("Configure your preferences")
//!     .body(settings_form)
//!     .footer(action_buttons);
//!
//! // Collapsible card
//! Card::new()
//!     .title("Details")
//!     .collapsible(true)
//!     .body(details_content);
//! ```

use super::border::BorderType;
use super::traits::{RenderContext, View, WidgetProps, WidgetState};
use crate::event::Key;
use crate::layout::Rect;
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::utils::unicode::char_width;
use crate::{impl_styled_view, impl_widget_builders};

/// Card visual variant
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CardVariant {
    /// Default with border
    #[default]
    Outlined,
    /// Filled background
    Filled,
    /// Elevated with shadow effect
    Elevated,
    /// Minimal without border
    Flat,
}

/// A card widget for grouping content with structure
pub struct Card {
    /// Card title
    title: Option<String>,
    /// Card subtitle
    subtitle: Option<String>,
    /// Header content (rendered below title)
    header: Option<Box<dyn View>>,
    /// Main body content
    body: Option<Box<dyn View>>,
    /// Footer content
    footer: Option<Box<dyn View>>,
    /// Visual variant
    variant: CardVariant,
    /// Border style
    border: BorderType,
    /// Background color
    bg_color: Option<Color>,
    /// Border/accent color
    border_color: Option<Color>,
    /// Title color
    title_color: Option<Color>,
    /// Whether the card is collapsible
    collapsible: bool,
    /// Whether the card is expanded (when collapsible)
    expanded: bool,
    /// Whether the card is clickable
    clickable: bool,
    /// Padding inside the card
    padding: u16,
    /// Widget state
    state: WidgetState,
    /// Widget properties
    props: WidgetProps,
}

impl Card {
    /// Create a new card
    pub fn new() -> Self {
        Self {
            title: None,
            subtitle: None,
            header: None,
            body: None,
            footer: None,
            variant: CardVariant::default(),
            border: BorderType::default(),
            bg_color: None,
            border_color: None,
            title_color: None,
            collapsible: false,
            expanded: true,
            clickable: false,
            padding: 1,
            state: WidgetState::new(),
            props: WidgetProps::new(),
        }
    }

    /// Set the card title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the card subtitle
    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    /// Set custom header content
    pub fn header(mut self, header: impl View + 'static) -> Self {
        self.header = Some(Box::new(header));
        self
    }

    /// Set the body content
    pub fn body(mut self, body: impl View + 'static) -> Self {
        self.body = Some(Box::new(body));
        self
    }

    /// Set the footer content
    pub fn footer(mut self, footer: impl View + 'static) -> Self {
        self.footer = Some(Box::new(footer));
        self
    }

    /// Set the visual variant
    pub fn variant(mut self, variant: CardVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the border style
    pub fn border_style(mut self, border: BorderType) -> Self {
        self.border = border;
        self
    }

    /// Use outlined variant
    pub fn outlined(mut self) -> Self {
        self.variant = CardVariant::Outlined;
        self
    }

    /// Use filled variant
    pub fn filled(mut self) -> Self {
        self.variant = CardVariant::Filled;
        self
    }

    /// Use elevated variant
    pub fn elevated(mut self) -> Self {
        self.variant = CardVariant::Elevated;
        self
    }

    /// Use flat variant (no border)
    pub fn flat(mut self) -> Self {
        self.variant = CardVariant::Flat;
        self.border = BorderType::None;
        self
    }

    /// Use rounded border
    pub fn rounded(mut self) -> Self {
        self.border = BorderType::Rounded;
        self
    }

    /// Set background color
    pub fn background(mut self, color: Color) -> Self {
        self.bg_color = Some(color);
        self
    }

    /// Set border/accent color
    pub fn border_color(mut self, color: Color) -> Self {
        self.border_color = Some(color);
        self
    }

    /// Set title color
    pub fn title_color(mut self, color: Color) -> Self {
        self.title_color = Some(color);
        self
    }

    /// Make the card collapsible
    pub fn collapsible(mut self, collapsible: bool) -> Self {
        self.collapsible = collapsible;
        self
    }

    /// Set the expanded state (for collapsible cards)
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    /// Make the card clickable
    pub fn clickable(mut self, clickable: bool) -> Self {
        self.clickable = clickable;
        self
    }

    /// Set padding inside the card
    pub fn padding(mut self, padding: u16) -> Self {
        self.padding = padding;
        self
    }

    /// Toggle expanded state
    pub fn toggle(&mut self) {
        if self.collapsible {
            self.expanded = !self.expanded;
        }
    }

    /// Expand the card
    pub fn expand(&mut self) {
        self.expanded = true;
    }

    /// Collapse the card
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

    /// Handle keyboard input
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

    /// Get the collapse indicator character
    fn collapse_icon(&self) -> char {
        if self.expanded {
            '▼'
        } else {
            '▶'
        }
    }

    /// Calculate footer height
    fn footer_height(&self) -> u16 {
        if self.footer.is_some() {
            2 // separator + footer content
        } else {
            0
        }
    }

    /// Get effective colors based on variant and state
    fn effective_colors(&self) -> (Option<Color>, Color, Color) {
        let default_bg: Option<Color> = match self.variant {
            CardVariant::Outlined => None,
            CardVariant::Filled => Some(Color::rgb(30, 30, 35)),
            CardVariant::Elevated => Some(Color::rgb(35, 35, 40)),
            CardVariant::Flat => None,
        };

        let default_border = match self.variant {
            CardVariant::Outlined => Color::rgb(60, 60, 70),
            CardVariant::Filled => Color::rgb(50, 50, 60),
            CardVariant::Elevated => Color::rgb(70, 70, 80),
            CardVariant::Flat => Color::rgb(40, 40, 40),
        };

        let default_title = Color::WHITE;

        let bg = self.bg_color.or(default_bg);
        let border = self.border_color.unwrap_or(default_border);
        let title = self.title_color.unwrap_or(default_title);

        // Adjust for focus state
        if self.state.focused && self.clickable {
            (bg, Color::CYAN, title)
        } else {
            (bg, border, title)
        }
    }
}

impl Default for Card {
    fn default() -> Self {
        Self::new()
    }
}

impl View for Card {
    crate::impl_view_meta!("Card");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 4 || area.height < 3 {
            return;
        }

        let (bg_color, border_color, title_color) = self.effective_colors();
        let chars = self.border.chars();
        let has_border = self.border != BorderType::None;

        // Fill background for filled/elevated variants
        if let Some(bg) = bg_color {
            for y in area.y..area.y + area.height {
                for x in area.x..area.x + area.width {
                    let mut cell = Cell::new(' ');
                    cell.bg = Some(bg);
                    ctx.buffer.set(x, y, cell);
                }
            }
        }

        // Draw shadow for elevated variant
        if self.variant == CardVariant::Elevated && area.width > 2 && area.height > 1 {
            let shadow_color = Color::rgb(20, 20, 20);
            // Right shadow
            for y in area.y + 1..area.y + area.height {
                let mut cell = Cell::new('▌');
                cell.fg = Some(shadow_color);
                ctx.buffer.set(area.x + area.width, y, cell);
            }
            // Bottom shadow
            for x in area.x + 1..=area.x + area.width {
                let mut cell = Cell::new('▀');
                cell.fg = Some(shadow_color);
                ctx.buffer.set(x, area.y + area.height, cell);
            }
        }

        // Draw border
        if has_border {
            // Corners
            ctx.buffer
                .set(area.x, area.y, Cell::new(chars.top_left).fg(border_color));
            ctx.buffer.set(
                area.x + area.width - 1,
                area.y,
                Cell::new(chars.top_right).fg(border_color),
            );
            ctx.buffer.set(
                area.x,
                area.y + area.height - 1,
                Cell::new(chars.bottom_left).fg(border_color),
            );
            ctx.buffer.set(
                area.x + area.width - 1,
                area.y + area.height - 1,
                Cell::new(chars.bottom_right).fg(border_color),
            );

            // Top and bottom borders
            for x in area.x + 1..area.x + area.width - 1 {
                ctx.buffer
                    .set(x, area.y, Cell::new(chars.horizontal).fg(border_color));
                ctx.buffer.set(
                    x,
                    area.y + area.height - 1,
                    Cell::new(chars.horizontal).fg(border_color),
                );
            }

            // Side borders
            for y in area.y + 1..area.y + area.height - 1 {
                ctx.buffer
                    .set(area.x, y, Cell::new(chars.vertical).fg(border_color));
                ctx.buffer.set(
                    area.x + area.width - 1,
                    y,
                    Cell::new(chars.vertical).fg(border_color),
                );
            }
        }

        // Content area
        let content_x = if has_border {
            area.x + 1 + self.padding
        } else {
            area.x + self.padding
        };
        let content_width = if has_border {
            area.width.saturating_sub(2 + self.padding * 2)
        } else {
            area.width.saturating_sub(self.padding * 2)
        };
        let mut current_y = if has_border { area.y + 1 } else { area.y };

        // Draw title
        if let Some(ref title) = self.title {
            let title_x = content_x;

            // Collapse icon
            if self.collapsible {
                let mut icon_cell = Cell::new(self.collapse_icon());
                icon_cell.fg = Some(title_color);
                ctx.buffer.set(title_x, current_y, icon_cell);

                TextDraw {
                    text: title,
                    x: title_x + 2,
                    y: current_y,
                    color: title_color,
                    max_width: content_width.saturating_sub(2),
                    bold: true,
                }
                .draw(ctx);
            } else {
                TextDraw {
                    text: title,
                    x: title_x,
                    y: current_y,
                    color: title_color,
                    max_width: content_width,
                    bold: true,
                }
                .draw(ctx);
            }
            current_y += 1;
        }

        // Draw subtitle
        if let Some(ref subtitle) = self.subtitle {
            TextDraw {
                text: subtitle,
                x: content_x,
                y: current_y,
                color: Color::rgb(150, 150, 150),
                max_width: content_width,
                bold: false,
            }
            .draw(ctx);
            current_y += 1;
        }

        // Draw custom header
        if let Some(ref header) = self.header {
            if self.expanded || !self.collapsible {
                let header_area = Rect::new(content_x, current_y, content_width, 1);
                let mut header_ctx = RenderContext::new(ctx.buffer, header_area);
                header.render(&mut header_ctx);
                current_y += 1;
            }
        }

        // Draw header separator if we have header content and body
        let has_header = self.title.is_some() || self.subtitle.is_some() || self.header.is_some();
        if has_header && self.body.is_some() && (self.expanded || !self.collapsible) {
            // Separator line
            let sep_y = current_y;
            if has_border {
                ctx.buffer
                    .set(area.x, sep_y, Cell::new('├').fg(border_color));
                ctx.buffer.set(
                    area.x + area.width - 1,
                    sep_y,
                    Cell::new('┤').fg(border_color),
                );
                for x in area.x + 1..area.x + area.width - 1 {
                    ctx.buffer.set(x, sep_y, Cell::new('─').fg(border_color));
                }
            } else {
                for x in area.x..area.x + area.width {
                    ctx.buffer
                        .set(x, sep_y, Cell::new('─').fg(Color::rgb(50, 50, 50)));
                }
            }
            current_y += 1;
        }

        // Draw body (only if expanded or not collapsible)
        if let Some(ref body) = self.body {
            if self.expanded || !self.collapsible {
                let footer_height = self.footer_height();
                let body_end = if has_border {
                    area.y + area.height - 1 - footer_height
                } else {
                    area.y + area.height - footer_height
                };
                let body_height = body_end.saturating_sub(current_y);

                if body_height > 0 {
                    let body_area = Rect::new(content_x, current_y, content_width, body_height);
                    let mut body_ctx = RenderContext::new(ctx.buffer, body_area);
                    body.render(&mut body_ctx);
                    current_y += body_height;
                }
            }
        }

        // Draw footer separator and content
        if let Some(ref footer) = self.footer {
            if self.expanded || !self.collapsible {
                let footer_y = if has_border {
                    area.y + area.height - 2
                } else {
                    area.y + area.height - 1
                };

                // Footer separator
                let sep_y = footer_y - 1;
                if sep_y > current_y {
                    if has_border {
                        ctx.buffer
                            .set(area.x, sep_y, Cell::new('├').fg(border_color));
                        ctx.buffer.set(
                            area.x + area.width - 1,
                            sep_y,
                            Cell::new('┤').fg(border_color),
                        );
                        for x in area.x + 1..area.x + area.width - 1 {
                            ctx.buffer.set(x, sep_y, Cell::new('─').fg(border_color));
                        }
                    }

                    // Footer content
                    let footer_area = Rect::new(content_x, footer_y, content_width, 1);
                    let mut footer_ctx = RenderContext::new(ctx.buffer, footer_area);
                    footer.render(&mut footer_ctx);
                }
            }
        }
    }
}

/// Text drawing parameters for Card rendering
struct TextDraw<'a> {
    text: &'a str,
    x: u16,
    y: u16,
    color: Color,
    max_width: u16,
    bold: bool,
}

impl TextDraw<'_> {
    /// Draw text with clipping and wide character support
    fn draw(self, ctx: &mut RenderContext) {
        let mut offset = 0u16;
        for ch in self.text.chars() {
            let ch_width = char_width(ch) as u16;
            if ch_width == 0 {
                continue;
            }
            if offset + ch_width > self.max_width {
                break;
            }
            let mut cell = Cell::new(ch);
            cell.fg = Some(self.color);
            if self.bold {
                cell.modifier |= Modifier::BOLD;
            }
            ctx.buffer.set(self.x + offset, self.y, cell);
            for i in 1..ch_width {
                ctx.buffer
                    .set(self.x + offset + i, self.y, Cell::continuation());
            }
            offset += ch_width;
        }
    }
}

impl_styled_view!(Card);
impl_widget_builders!(Card);

/// Helper function to create a Card
pub fn card() -> Card {
    Card::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::Buffer;
    use crate::widget::Text;

    #[test]
    fn test_card_new() {
        let c = Card::new();
        assert!(c.title.is_none());
        assert!(c.body.is_none());
        assert!(!c.collapsible);
        assert!(c.expanded);
    }

    #[test]
    fn test_card_title() {
        let c = Card::new().title("My Card");
        assert_eq!(c.title, Some("My Card".to_string()));
    }

    #[test]
    fn test_card_subtitle() {
        let c = Card::new().subtitle("Description");
        assert_eq!(c.subtitle, Some("Description".to_string()));
    }

    #[test]
    fn test_card_variants() {
        let c = Card::new().outlined();
        assert_eq!(c.variant, CardVariant::Outlined);

        let c = Card::new().filled();
        assert_eq!(c.variant, CardVariant::Filled);

        let c = Card::new().elevated();
        assert_eq!(c.variant, CardVariant::Elevated);

        let c = Card::new().flat();
        assert_eq!(c.variant, CardVariant::Flat);
    }

    #[test]
    fn test_card_border_styles() {
        let c = Card::new().rounded();
        assert_eq!(c.border, BorderType::Rounded);

        let c = Card::new().border_style(BorderType::Double);
        assert_eq!(c.border, BorderType::Double);
    }

    #[test]
    fn test_card_collapsible() {
        let mut c = Card::new().collapsible(true);
        assert!(c.is_collapsible());
        assert!(c.is_expanded());

        c.toggle();
        assert!(!c.is_expanded());

        c.expand();
        assert!(c.is_expanded());

        c.collapse();
        assert!(!c.is_expanded());
    }

    #[test]
    fn test_card_collapsible_toggle_not_collapsible() {
        let mut c = Card::new().collapsible(false);
        c.toggle();
        assert!(c.is_expanded()); // Should remain expanded
    }

    #[test]
    fn test_card_handle_key() {
        let mut c = Card::new().collapsible(true);

        assert!(c.handle_key(&Key::Enter));
        assert!(!c.is_expanded());

        assert!(c.handle_key(&Key::Char(' ')));
        assert!(c.is_expanded());

        assert!(c.handle_key(&Key::Left));
        assert!(!c.is_expanded());

        assert!(c.handle_key(&Key::Right));
        assert!(c.is_expanded());
    }

    #[test]
    fn test_card_handle_key_not_collapsible() {
        let mut c = Card::new();
        assert!(!c.handle_key(&Key::Enter));
    }

    #[test]
    fn test_card_handle_key_disabled() {
        let mut c = Card::new().collapsible(true).disabled(true);
        assert!(!c.handle_key(&Key::Enter));
    }

    #[test]
    fn test_card_colors() {
        let c = Card::new()
            .background(Color::RED)
            .border_color(Color::BLUE)
            .title_color(Color::GREEN);

        assert_eq!(c.bg_color, Some(Color::RED));
        assert_eq!(c.border_color, Some(Color::BLUE));
        assert_eq!(c.title_color, Some(Color::GREEN));
    }

    #[test]
    fn test_card_padding() {
        let c = Card::new().padding(2);
        assert_eq!(c.padding, 2);
    }

    #[test]
    fn test_card_render_basic() {
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let c = Card::new().title("Test Card");
        c.render(&mut ctx);

        // Check corners
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
        assert_eq!(buffer.get(19, 0).unwrap().symbol, '┐');
        assert_eq!(buffer.get(0, 9).unwrap().symbol, '└');
        assert_eq!(buffer.get(19, 9).unwrap().symbol, '┘');
    }

    #[test]
    fn test_card_render_rounded() {
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let c = Card::new().rounded().title("Rounded");
        c.render(&mut ctx);

        assert_eq!(buffer.get(0, 0).unwrap().symbol, '╭');
        assert_eq!(buffer.get(19, 0).unwrap().symbol, '╮');
    }

    #[test]
    fn test_card_with_body() {
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let c = Card::new().title("Title").body(Text::new("Body content"));
        c.render(&mut ctx);

        // Should have separator
        assert_eq!(buffer.get(0, 2).unwrap().symbol, '├');
    }

    #[test]
    fn test_card_default() {
        let c = Card::default();
        assert!(c.title.is_none());
    }

    #[test]
    fn test_card_helper() {
        let c = card().title("Helper");
        assert_eq!(c.title, Some("Helper".to_string()));
    }

    #[test]
    fn test_card_border_chars() {
        let chars = BorderType::Single.chars();
        assert_eq!(chars.top_left, '┌');
        assert_eq!(chars.top_right, '┐');
        assert_eq!(chars.bottom_left, '└');
        assert_eq!(chars.bottom_right, '┘');
        assert_eq!(chars.horizontal, '─');
        assert_eq!(chars.vertical, '│');

        let chars = BorderType::Rounded.chars();
        assert_eq!(chars.top_left, '╭');
    }

    #[test]
    fn test_card_collapse_icon() {
        let c = Card::new().collapsible(true).expanded(true);
        assert_eq!(c.collapse_icon(), '▼');

        let c = Card::new().collapsible(true).expanded(false);
        assert_eq!(c.collapse_icon(), '▶');
    }

    #[test]
    fn test_card_effective_colors() {
        let c = Card::new().outlined();
        let (_, border, _) = c.effective_colors();
        assert_eq!(border, Color::rgb(60, 60, 70));

        let c = Card::new().filled();
        let (bg, _, _) = c.effective_colors();
        assert_eq!(bg, Some(Color::rgb(30, 30, 35)));
    }

    #[test]
    fn test_card_clickable() {
        let c = Card::new().clickable(true);
        assert!(c.clickable);
    }
}
