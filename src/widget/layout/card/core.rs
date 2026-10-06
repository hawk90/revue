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

use super::super::border::BorderType;
use crate::event::Key;
use crate::layout::Rect;
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::utils::unicode::char_width;
use crate::widget::theme::{DARK_BG, LIGHT_GRAY};
use crate::widget::traits::{RenderContext, View, WidgetProps, WidgetState};
use crate::{impl_styled_view, impl_widget_builders};

use super::types::CardVariant;

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
    /// Minimum width constraint (0 = no constraint)
    min_width: u16,
    /// Minimum height constraint (0 = no constraint)
    min_height: u16,
    /// Maximum width constraint (0 = no constraint)
    max_width: u16,
    /// Maximum height constraint (0 = no constraint)
    max_height: u16,
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
            min_width: 0,
            min_height: 0,
            max_width: 0,
            max_height: 0,
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
    ///
    /// [`CardVariant::Flat`] has no border, so choosing it clears the border
    /// just as [`Card::flat`] does; a `border_style` set afterwards still wins.
    pub fn variant(mut self, variant: CardVariant) -> Self {
        self.variant = variant;
        if variant == CardVariant::Flat {
            self.border = BorderType::None;
        }
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

    /// Set minimum width constraint
    pub fn min_width(mut self, width: u16) -> Self {
        self.min_width = width;
        self
    }

    /// Set minimum height constraint
    pub fn min_height(mut self, height: u16) -> Self {
        self.min_height = height;
        self
    }

    /// Set maximum width constraint (0 = no limit)
    pub fn max_width(mut self, width: u16) -> Self {
        self.max_width = width;
        self
    }

    /// Set maximum height constraint (0 = no limit)
    pub fn max_height(mut self, height: u16) -> Self {
        self.max_height = height;
        self
    }

    /// Set both min width and height
    pub fn min_size(self, width: u16, height: u16) -> Self {
        self.min_width(width).min_height(height)
    }

    /// Set both max width and height (0 = no limit)
    pub fn max_size(self, width: u16, height: u16) -> Self {
        self.max_width(width).max_height(height)
    }

    /// Set all size constraints at once
    pub fn constrain(self, min_w: u16, min_h: u16, max_w: u16, max_h: u16) -> Self {
        self.min_width(min_w)
            .min_height(min_h)
            .max_width(max_w)
            .max_height(max_h)
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
    /// [`effective_colors`](Self::effective_colors), with the stylesheet
    /// filling in whatever the builder left unset.
    ///
    /// The builder is the inline style and wins; a variant's defaults are the
    /// bottom row, below the stylesheet. Same precedence as every other paint
    /// property in the crate.
    fn effective_colors_with_css(&self, ctx: &RenderContext) -> (Option<Color>, Color, Color) {
        let (bg, border, title) = self.effective_colors();

        let css_bg = ctx
            .style
            .map(|s| s.visual.background)
            .filter(|c| *c != Color::default());
        let bg = self.bg_color.or(css_bg).or(bg);

        let border = match (self.border_color, ctx.css_border_or_text_color()) {
            (Some(explicit), _) => explicit,
            (None, Some(css)) => css,
            (None, None) => border,
        };

        (bg, border, title)
    }

    /// The border to draw: the stylesheet's if it named one, else the builder's.
    ///
    /// See `Border::border_type_with_css` - a stylesheet that said nothing
    /// leaves the builder's choice alone, and `border-style: none` removes the
    /// border.
    fn border_type_with_css(&self, ctx: &RenderContext) -> BorderType {
        match ctx.css_border_style() {
            None => self.border,
            Some(crate::style::BorderStyle::None) => BorderType::None,
            Some(crate::style::BorderStyle::Solid) | Some(crate::style::BorderStyle::Dashed) => {
                BorderType::Single
            }
            Some(crate::style::BorderStyle::Double) => BorderType::Double,
            Some(crate::style::BorderStyle::Rounded) => BorderType::Rounded,
        }
    }

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
            CardVariant::Flat => DARK_BG,
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

    /// As wide as offered (the frame and background run the full width) and
    /// as tall as its rows: the frame, title, subtitle, header, separators,
    /// the body's measured height and the footer - plus the shadow row of
    /// `Elevated`. A body that fills (`None`) makes the card fill. Never
    /// shorter than the 3 rows `render` needs to draw anything.
    ///
    /// The frame counted is the builder's: a stylesheet `border-style`
    /// is applied at paint time, which `measure` cannot see.
    fn measure(&self, max_width: u16, max_height: u16) -> Option<(u16, u16)> {
        let shown = self.expanded || !self.collapsible;
        let frame: u16 = if self.border != BorderType::None {
            2
        } else {
            0
        };
        let shadow = u16::from(self.variant == CardVariant::Elevated);

        let mut rows = frame + shadow;
        rows += u16::from(self.title.is_some()) + u16::from(self.subtitle.is_some());
        let has_header = self.title.is_some() || self.subtitle.is_some() || self.header.is_some();
        if shown {
            rows += u16::from(self.header.is_some());
            if let Some(body) = &self.body {
                rows += u16::from(has_header); // separator under the header
                let side = frame + self.padding.saturating_mul(2) + shadow;
                let (_, h) = body.measure(
                    max_width.saturating_sub(side),
                    max_height.saturating_sub(rows),
                )?;
                rows = rows.saturating_add(h);
            }
            rows = rows.saturating_add(self.footer_height());
        }

        let mut h = rows.max(3).max(self.min_height);
        if self.max_height > 0 {
            h = h.min(self.max_height);
        }
        Some((max_width, h.min(max_height)))
    }

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 4 || area.height < 3 {
            return;
        }

        let (bg_color, border_color, title_color) = self.effective_colors_with_css(ctx);
        let border_type = self.border_type_with_css(ctx);
        let chars = border_type.chars();
        let has_border = border_type != BorderType::None;

        // Fill background for filled/elevated variants
        if let Some(bg) = bg_color {
            ctx.fill_box_background(bg);
        }

        // Draw shadow for elevated variant (inside area bounds). The shadow
        // takes the last column and row and the card is drawn in what is
        // left, so its border does not paint over the shadow.
        let area = if self.variant == CardVariant::Elevated && area.width > 4 && area.height > 3 {
            let shadow_color = Color::rgb(20, 20, 20);
            // Right shadow (last column, below the card's top row)
            for y in 1..area.height {
                let mut cell = Cell::new('▌');
                cell.fg = Some(shadow_color);
                ctx.set(area.width - 1, y, cell);
            }
            // Bottom shadow (last row, right of the card's left edge)
            for x in 1..area.width {
                let mut cell = Cell::new('▀');
                cell.fg = Some(shadow_color);
                ctx.set(x, area.height - 1, cell);
            }
            Rect::new(area.x, area.y, area.width - 1, area.height - 1)
        } else {
            area
        };

        // Draw border
        if has_border {
            // Corners
            ctx.set(0, 0, Cell::new(chars.top_left).fg(border_color));
            ctx.set(
                area.width - 1,
                0,
                Cell::new(chars.top_right).fg(border_color),
            );
            ctx.set(
                0,
                area.height - 1,
                Cell::new(chars.bottom_left).fg(border_color),
            );
            ctx.set(
                area.width - 1,
                area.height - 1,
                Cell::new(chars.bottom_right).fg(border_color),
            );

            // Top and bottom borders
            for x in 1..area.width - 1 {
                ctx.set(x, 0, Cell::new(chars.horizontal).fg(border_color));
                ctx.set(
                    x,
                    area.height - 1,
                    Cell::new(chars.horizontal).fg(border_color),
                );
            }

            // Side borders
            for y in 1..area.height - 1 {
                ctx.set(0, y, Cell::new(chars.vertical).fg(border_color));
                ctx.set(
                    area.width - 1,
                    y,
                    Cell::new(chars.vertical).fg(border_color),
                );
            }
        }

        // Content area (relative offsets within this widget)
        let content_x = if has_border {
            1 + self.padding
        } else {
            self.padding
        };
        let content_width = if has_border {
            area.width.saturating_sub(2 + self.padding * 2)
        } else {
            area.width.saturating_sub(self.padding * 2)
        };
        let mut current_y: u16 = if has_border { 1 } else { 0 };

        // Draw title
        if let Some(ref title) = self.title {
            let title_x = content_x;

            // Collapse icon
            if self.collapsible {
                let mut icon_cell = Cell::new(self.collapse_icon());
                icon_cell.fg = Some(title_color);
                ctx.set(title_x, current_y, icon_cell);

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
                color: LIGHT_GRAY,
                max_width: content_width,
                bold: false,
            }
            .draw(ctx);
            current_y += 1;
        }

        // Draw custom header
        if let Some(ref header) = self.header {
            if self.expanded || !self.collapsible {
                let header_area = ctx.sub_area(content_x, current_y, content_width, 1);
                ctx.render_child(header.as_ref(), header_area);
                current_y += 1;
            }
        }

        // Draw header separator if we have header content and body
        let has_header = self.title.is_some() || self.subtitle.is_some() || self.header.is_some();
        if has_header && self.body.is_some() && (self.expanded || !self.collapsible) {
            // Separator line
            let sep_y = current_y;
            if has_border {
                ctx.set(0, sep_y, Cell::new('├').fg(border_color));
                ctx.set(area.width - 1, sep_y, Cell::new('┤').fg(border_color));
                for x in 1..area.width - 1 {
                    ctx.set(x, sep_y, Cell::new('─').fg(border_color));
                }
            } else {
                for x in 0..area.width {
                    ctx.set(x, sep_y, Cell::new('─').fg(Color::rgb(50, 50, 50)));
                }
            }
            current_y += 1;
        }

        // Draw body (only if expanded or not collapsible)
        if let Some(ref body) = self.body {
            if self.expanded || !self.collapsible {
                let footer_height = self.footer_height();
                let body_end = if has_border {
                    area.height - 1 - footer_height
                } else {
                    area.height - footer_height
                };
                let body_height = body_end.saturating_sub(current_y);

                if body_height > 0 {
                    let body_area = ctx.sub_area(content_x, current_y, content_width, body_height);
                    ctx.render_child(body.as_ref(), body_area);
                    current_y += body_height;
                }
            }
        }

        // Draw footer separator and content
        if let Some(ref footer) = self.footer {
            if self.expanded || !self.collapsible {
                let footer_y = if has_border {
                    area.height - 2
                } else {
                    area.height - 1
                };

                // Footer separator.
                //
                // `>=`, not `>`: the body stops at `body_end`, which is this
                // same row, so it occupies up to `sep_y - 1` and leaves this one
                // free. Requiring a gap meant a card with both a body and a
                // footer never drew the footer at all.
                let sep_y = footer_y - 1;
                if sep_y >= current_y {
                    if has_border {
                        ctx.set(0, sep_y, Cell::new('├').fg(border_color));
                        ctx.set(area.width - 1, sep_y, Cell::new('┤').fg(border_color));
                        for x in 1..area.width - 1 {
                            ctx.set(x, sep_y, Cell::new('─').fg(border_color));
                        }
                    }

                    // Footer content
                    let footer_area = ctx.sub_area(content_x, footer_y, content_width, 1);
                    ctx.render_child(footer.as_ref(), footer_area);
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
            ctx.set(self.x + offset, self.y, cell);
            for i in 1..ch_width {
                ctx.set(self.x + offset + i, self.y, Cell::continuation());
            }
            offset += ch_width;
        }
    }
}

impl_styled_view!(Card);
impl_widget_builders!(Card);
