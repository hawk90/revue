//! Drawing the diff viewer: split and unified views

use super::{ChangeType, DiffLine, DiffMode, DiffViewer};
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::utils::{char_width, truncate_to_width};
use crate::widget::traits::{RenderContext, View};

/// Line rendering layout parameters
struct LineLayout {
    x: u16,
    y: u16,
    line_num_width: u16,
    content_width: usize,
}

impl DiffViewer {
    /// Render split view
    fn render_split(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 10 || area.height < 3 {
            return;
        }

        let half_width = (area.width / 2).saturating_sub(1);
        let line_num_width = if self.show_line_numbers { 5 } else { 0 };
        let content_width = half_width.saturating_sub(line_num_width) as usize;

        // Header
        self.render_header(ctx, half_width);

        // Content
        let visible_lines = (area.height - 1) as usize;
        for (i, line) in self
            .diff_lines
            .iter()
            .skip(self.scroll)
            .take(visible_lines)
            .enumerate()
        {
            let y = 1 + i as u16;

            // Left side
            let left_layout = LineLayout {
                x: 0,
                y,
                line_num_width,
                content_width,
            };
            self.render_line_half(ctx, line, true, &left_layout);

            // Separator
            let mut sep = Cell::new('│');
            sep.fg = Some(self.colors.separator);
            ctx.set(half_width, y, sep);

            // Right side
            let right_layout = LineLayout {
                x: half_width + 1,
                y,
                line_num_width,
                content_width,
            };
            self.render_line_half(ctx, line, false, &right_layout);
        }
    }

    /// Render one half of a split line
    fn render_line_half(
        &self,
        ctx: &mut RenderContext,
        line: &DiffLine,
        is_left: bool,
        layout: &LineLayout,
    ) {
        let LineLayout {
            x,
            y,
            line_num_width,
            content_width,
        } = *layout;
        let (content, line_num, bg) = if is_left {
            (
                &line.left,
                line.left_num,
                match line.change {
                    ChangeType::Removed => Some(self.colors.removed_bg),
                    ChangeType::Modified => Some(self.colors.modified_bg),
                    _ => None,
                },
            )
        } else {
            (
                &line.right,
                line.right_num,
                match line.change {
                    ChangeType::Added => Some(self.colors.added_bg),
                    ChangeType::Modified => Some(self.colors.modified_bg),
                    _ => None,
                },
            )
        };

        // Line number
        if self.show_line_numbers {
            let num_str = line_num
                .map(|n| format!("{:>4}", n))
                .unwrap_or_else(|| "    ".to_string());
            for (i, ch) in num_str.chars().enumerate() {
                if i as u16 >= line_num_width {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(self.colors.line_number);
                cell.bg = bg;
                ctx.set(x + i as u16, y, cell);
            }
        }

        // Content
        let fg = match line.change {
            ChangeType::Added => Some(self.colors.added_fg),
            ChangeType::Removed => Some(self.colors.removed_fg),
            _ => None,
        };

        let truncated = truncate_to_width(content, content_width);
        let mut dx: u16 = 0;
        for ch in truncated.chars() {
            let cw = char_width(ch) as u16;
            if dx + cw > content_width as u16 {
                break;
            }
            let mut cell = Cell::new(ch);
            cell.fg = fg;
            cell.bg = bg;
            ctx.set(x + line_num_width + dx, y, cell);
            dx += cw;
        }

        // Fill remaining with background
        for i in dx..(content_width as u16) {
            let mut cell = Cell::new(' ');
            cell.bg = bg;
            ctx.set(x + line_num_width + i, y, cell);
        }
    }

    /// Render header
    fn render_header(&self, ctx: &mut RenderContext, half_width: u16) {
        let header_bg = self.colors.header_bg;
        let name_cell = |ch: char| {
            let mut cell = Cell::new(ch);
            cell.bg = Some(header_bg);
            cell.modifier = Modifier::BOLD;
            cell
        };

        // Left header, in columns; the fill picks up where the name ends
        let used = ctx.put_str_with(0, 0, &self.left_name, half_width, name_cell);
        for x in used..half_width {
            let mut cell = Cell::new(' ');
            cell.bg = Some(header_bg);
            ctx.set(x, 0, cell);
        }

        // Separator
        let mut sep = Cell::new('│');
        sep.fg = Some(self.colors.separator);
        sep.bg = Some(self.colors.header_bg);
        ctx.set(half_width, 0, sep);

        // Right header
        let right_x = half_width + 1;
        let right_end = right_x.saturating_add(half_width);
        let used = ctx.put_str_with(right_x, 0, &self.right_name, right_end, name_cell);
        for x in right_x + used..right_end {
            let mut cell = Cell::new(' ');
            cell.bg = Some(header_bg);
            ctx.set(x, 0, cell);
        }
    }

    /// Render unified view
    fn render_unified(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let line_num_width = if self.show_line_numbers { 10u16 } else { 0 };
        let content_width = area.width.saturating_sub(line_num_width + 1) as usize;

        let visible_lines = area.height as usize;
        for (i, line) in self
            .diff_lines
            .iter()
            .skip(self.scroll)
            .take(visible_lines)
            .enumerate()
        {
            let y = i as u16;

            // Line numbers (left:right)
            if self.show_line_numbers {
                let num_str = format!(
                    "{:>4}:{:<4}",
                    line.left_num.map(|n| n.to_string()).unwrap_or_default(),
                    line.right_num.map(|n| n.to_string()).unwrap_or_default()
                );
                for (j, ch) in num_str.chars().enumerate() {
                    if j as u16 >= line_num_width {
                        break;
                    }
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(self.colors.line_number);
                    ctx.set(j as u16, y, cell);
                }
            }

            // Change indicator
            let (indicator, fg, bg) = match line.change {
                ChangeType::Added => ('+', self.colors.added_fg, self.colors.added_bg),
                ChangeType::Removed => ('-', self.colors.removed_fg, self.colors.removed_bg),
                ChangeType::Modified => ('~', self.colors.added_fg, self.colors.modified_bg),
                ChangeType::Equal => (' ', Color::WHITE, Color::default()),
            };

            let mut ind_cell = Cell::new(indicator);
            ind_cell.fg = Some(fg);
            ind_cell.bg = Some(bg);
            ctx.set(line_num_width, y, ind_cell);

            // Content
            let content = if !line.right.is_empty() {
                &line.right
            } else {
                &line.left
            };
            let truncated = truncate_to_width(content, content_width);
            let mut dx: u16 = 0;
            for ch in truncated.chars() {
                let cw = char_width(ch) as u16;
                if dx + cw > content_width as u16 {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(fg);
                cell.bg = Some(bg);
                ctx.set(line_num_width + 1 + dx, y, cell);
                dx += cw;
            }
        }
    }
}

impl View for DiffViewer {
    crate::impl_view_meta!("DiffViewer");

    fn render(&self, ctx: &mut RenderContext) {
        match self.mode {
            DiffMode::Split => self.render_split(ctx),
            DiffMode::Unified | DiffMode::Inline => self.render_unified(ctx),
        }
    }
}
