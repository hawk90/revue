//! Sizing and drawing the modal: centered box, title, body or text content, and buttons

use super::{Modal, ModalButton, ModalButtonStyle};
use crate::render::Cell;
use crate::style::Color;
use crate::widget::traits::{RenderContext, View};

impl View for Modal {
    fn render(&self, ctx: &mut RenderContext) {
        if !self.visible {
            return;
        }

        let area = ctx.area;
        let modal_width = self.width.min(area.width.saturating_sub(4));
        let modal_height = self.required_height().min(area.height.saturating_sub(2));

        // Center the modal (relative coordinates)
        let x = (area.width.saturating_sub(modal_width)) / 2;
        let y = (area.height.saturating_sub(modal_height)) / 2;

        // Draw border
        self.render_border(ctx, x, y, modal_width, modal_height);
        self.draw_title(ctx, x, y, modal_width);
        self.draw_content(ctx, x, y, modal_width, modal_height);
        self.draw_buttons(ctx, x, y, modal_width, modal_height);
    }

    crate::impl_view_meta!(@common);

    fn meta(&self) -> crate::dom::WidgetMeta {
        self.props
            .build_meta("Modal", false, false)
            .focus_scope(self.visible)
    }
}

impl Modal {
    /// Calculate required height
    #[doc(hidden)]
    pub fn required_height(&self) -> u16 {
        // If height is explicitly set, use it
        if let Some(h) = self.height {
            return h;
        }

        // For body widget, use a default content height of 5 lines
        let content_lines = if self.body.is_some() {
            5u16
        } else {
            self.content.len() as u16
        };

        let button_line = if self.buttons.is_empty() { 0 } else { 1 };
        // top border + title + title separator + content + padding + buttons + bottom border
        3 + content_lines + 1 + button_line + 1
    }

    /// Whether the title row and its separator are drawn
    fn has_title(&self, modal_width: u16) -> bool {
        !self.title.is_empty() && modal_width > 4
    }

    /// The title on the first row inside the border, and the separator below it
    fn draw_title(&self, ctx: &mut RenderContext, x: u16, y: u16, modal_width: u16) {
        if self.has_title(modal_width) {
            let title_x = x + 2;
            let title_width = modal_width.saturating_sub(4);
            let title_fg = self.title_fg.unwrap_or_else(|| ctx.css_color(Color::WHITE));
            ctx.draw_text_clipped_bold(title_x, y + 1, &self.title, title_fg, title_width);

            // Title separator
            for dx in 1..modal_width.saturating_sub(1) {
                ctx.set(x + dx, y + 2, Cell::new('─'));
            }
            ctx.set(x, y + 2, Cell::new('├'));
            ctx.set(x + modal_width.saturating_sub(1), y + 2, Cell::new('┤'));
        }
    }

    /// The body widget, or else the text lines, below the title
    fn draw_content(
        &self,
        ctx: &mut RenderContext,
        x: u16,
        y: u16,
        modal_width: u16,
        modal_height: u16,
    ) {
        // Draw content — adjust for title presence
        let has_title = self.has_title(modal_width);
        let content_y = if has_title { y + 3 } else { y + 1 };
        let content_width = modal_width.saturating_sub(4);
        // borders(2) + buttons(1) + padding(1) + title+separator(2 if present)
        let content_height = if has_title {
            modal_height.saturating_sub(6)
        } else {
            modal_height.saturating_sub(4)
        };

        if let Some(ref body_widget) = self.body {
            // Render child widget
            let content_area = ctx.sub_area(x + 2, content_y, content_width, content_height);
            ctx.render_child(body_widget.as_ref(), content_area);
        } else {
            // Render text content
            for (i, line) in self.content.iter().enumerate() {
                let cy = content_y + i as u16;
                if cy >= y + modal_height.saturating_sub(2) {
                    break;
                }
                ctx.draw_text_clipped(x + 2, cy, line, Color::rgb(220, 220, 220), content_width);
            }
        }
    }

    /// The buttons centered on the row above the bottom border, or none
    /// when they do not all fit
    fn draw_buttons(
        &self,
        ctx: &mut RenderContext,
        x: u16,
        y: u16,
        modal_width: u16,
        modal_height: u16,
    ) {
        if !self.buttons.is_empty() && modal_height > 2 {
            let button_y = y + modal_height.saturating_sub(2);
            let total_button_width: usize = self
                .buttons
                .iter()
                .map(|b| crate::utils::display_width(&b.label) + 4) // [ label ]
                .sum::<usize>()
                + (self.buttons.len() - 1) * 2; // spacing

            // Skip drawing buttons if they don't fit
            if total_button_width as u16 > modal_width {
                return;
            }
            let start_x = x + (modal_width.saturating_sub(total_button_width as u16)) / 2;
            let mut bx = start_x;

            for (i, button) in self.buttons.iter().enumerate() {
                let is_selected = i == self.selected_button;
                let btn_x = Self::draw_button(ctx, button, is_selected, bx, button_y);
                bx = btn_x + 2;
            }
        }
    }

    /// One `[ label ]` button from `bx`; returns the column after it
    fn draw_button(
        ctx: &mut RenderContext,
        button: &ModalButton,
        is_selected: bool,
        bx: u16,
        button_y: u16,
    ) -> u16 {
        let button_text = format!("[ {} ]", button.label);

        let (fg, bg) = if is_selected {
            match button.style {
                ModalButtonStyle::Primary => (Some(Color::WHITE), Some(Color::BLUE)),
                ModalButtonStyle::Danger => (Some(Color::WHITE), Some(Color::RED)),
                ModalButtonStyle::Default => (Some(Color::BLACK), Some(Color::WHITE)),
            }
        } else {
            (None, None)
        };

        let mut btn_x = bx;
        for ch in button_text.chars() {
            let cw = crate::utils::char_width(ch) as u16;
            let mut cell = Cell::new(ch);
            cell.fg = fg;
            cell.bg = bg;
            if is_selected {
                cell.modifier |= crate::render::Modifier::BOLD;
            }
            ctx.set(btn_x, button_y, cell);
            btn_x += cw;
        }
        btn_x
    }

    /// A border cell in the border color
    fn border_cell(&self, ch: char) -> Cell {
        let mut cell = Cell::new(ch);
        cell.fg = self.border_fg;
        cell
    }

    fn render_border(&self, ctx: &mut RenderContext, x: u16, y: u16, width: u16, height: u16) {
        if width < 2 || height < 2 {
            return;
        }

        // Clear interior with spaces
        for dy in 1..height.saturating_sub(1) {
            for dx in 1..width.saturating_sub(1) {
                ctx.set(x + dx, y + dy, Cell::new(' '));
            }
        }

        // Top border
        ctx.set(x, y, self.border_cell('┌'));

        for dx in 1..width.saturating_sub(1) {
            ctx.set(x + dx, y, self.border_cell('─'));
        }

        ctx.set(x + width.saturating_sub(1), y, self.border_cell('┐'));

        // Sides
        for dy in 1..height.saturating_sub(1) {
            let cell = self.border_cell('│');
            ctx.set(x, y + dy, cell);
            ctx.set(x + width.saturating_sub(1), y + dy, cell);
        }

        // Bottom border
        ctx.set(x, y + height.saturating_sub(1), self.border_cell('└'));

        for dx in 1..width.saturating_sub(1) {
            ctx.set(x + dx, y + height.saturating_sub(1), self.border_cell('─'));
        }

        ctx.set(
            x + width.saturating_sub(1),
            y + height.saturating_sub(1),
            self.border_cell('┘'),
        );
    }
}
