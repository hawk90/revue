//! View implementation for Accordion

use crate::render::Cell;
use crate::utils::border::render_border;
use crate::widget::theme::SEPARATOR_COLOR;
use crate::widget::traits::{RenderContext, View};

use super::Accordion;

impl View for Accordion {
    crate::impl_view_meta!("Accordion");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 3 || area.height < 1 {
            return;
        }

        let (content_x_off, content_y_off, content_width, content_height) =
            if self.border_color.is_some() {
                (
                    1u16,
                    1u16,
                    area.width.saturating_sub(2),
                    area.height.saturating_sub(2),
                )
            } else {
                (0u16, 0u16, area.width, area.height)
            };

        // Draw border if set
        if let Some(border_color) = self.border_color {
            render_border(ctx, area, border_color);
        }

        let mut y = content_y_off;
        let max_y = content_y_off + content_height;

        for (section_idx, section) in self.sections.iter().enumerate() {
            if y >= max_y {
                break;
            }

            let is_selected = self.selection.is_selected(section_idx);

            // Render header
            let header_bg = if is_selected {
                self.selected_bg
            } else {
                self.header_bg
            };

            // Fill header background
            for x in content_x_off..content_x_off + content_width {
                let mut cell = Cell::new(' ');
                cell.bg = Some(header_bg);
                ctx.set(x, y, cell);
            }

            // Icon
            let mut icon_cell = Cell::new(section.icon());
            icon_cell.fg = Some(self.header_fg);
            icon_cell.bg = Some(header_bg);
            ctx.set(content_x_off + 1, y, icon_cell);

            // Title
            let title_x = content_x_off + 3;
            let max_title_width = content_width.saturating_sub(4);
            if is_selected {
                ctx.draw_text_clipped_bg_bold(
                    title_x,
                    y,
                    &section.title,
                    self.header_fg,
                    header_bg,
                    max_title_width,
                );
            } else {
                ctx.draw_text_clipped_bg(
                    title_x,
                    y,
                    &section.title,
                    self.header_fg,
                    header_bg,
                    max_title_width,
                );
            }

            y += 1;

            // Render content if expanded
            if section.expanded {
                for line in &section.content {
                    if y >= max_y {
                        break;
                    }

                    // Fill content background
                    for x in content_x_off..content_x_off + content_width {
                        let mut cell = Cell::new(' ');
                        cell.bg = Some(self.content_bg);
                        ctx.set(x, y, cell);
                    }

                    // Content with indent
                    let content_x = content_x_off + 3;
                    let max_content_width = content_width.saturating_sub(4);
                    ctx.draw_text_clipped_bg(
                        content_x,
                        y,
                        line,
                        self.content_fg,
                        self.content_bg,
                        max_content_width,
                    );

                    y += 1;
                }
            }

            // Divider
            if self.show_dividers && section_idx < self.sections.len() - 1 && y < max_y {
                for x in content_x_off..content_x_off + content_width {
                    let mut cell = Cell::new('─');
                    cell.fg = Some(SEPARATOR_COLOR);
                    ctx.set(x, y, cell);
                }
                y += 1;
            }
        }
    }
}
