//! Tree widget rendering

use super::rows::Row;
use super::types::TreeNode;
use super::Tree;
use crate::render::Cell;
use crate::style::Color;
use crate::widget::traits::RenderContext;

impl Tree {
    /// Internal render implementation
    pub fn render_internal(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 3 || area.height < 1 || self.root.is_empty() {
            return;
        }

        // The rows in view: the view follows the selection (#973)
        let rows = self.rows();
        self.scroll.set_content(rows.len());
        self.scroll.set_viewport(area.height as usize);
        let selected = self.selection.index;
        self.scroll.track(selected..selected + 1);
        let shown = self.scroll.visible();
        for (y, index) in shown.enumerate() {
            self.render_row(ctx, &rows, index, y as u16);
        }
    }

    /// One row: the guide lines, the expand indicator and icon, the label
    fn render_row(&self, ctx: &mut RenderContext, rows: &[Row], index: usize, y: u16) {
        let node = rows[index].node;
        let is_selected = index == self.selection.index;

        // Unselected rows take the stylesheet; the selected one keeps
        // its highlight, which a rule cannot address separately.
        let (fg, bg) = if is_selected {
            (self.selected_fg, self.selected_bg)
        } else {
            (
                self.fg.or_else(|| ctx.css_color_if_set()),
                self.bg.or_else(|| ctx.css_background_if_set()),
            )
        };

        if is_selected {
            for x in 0..ctx.area.width {
                let mut cell = Cell::new(' ');
                cell.bg = bg;
                ctx.set(x, y, cell);
            }
        }

        let x = self.draw_guides(ctx, rows, index, y, fg, bg);
        let x = draw_marker(ctx, node, x, y, fg, bg);
        self.draw_label(ctx, node, x, y, fg, bg);
    }

    /// The `│` lines of the ancestors and the `├─`/`└─` connector; returns
    /// the column after them
    fn draw_guides(
        &self,
        ctx: &mut RenderContext,
        rows: &[Row],
        index: usize,
        y: u16,
        fg: Option<Color>,
        bg: Option<Color>,
    ) -> u16 {
        let mut x = 0u16;
        let effective_indent = self.indent.max(1);
        for parent_is_last in ancestors_last(rows, index) {
            let ch = if parent_is_last { ' ' } else { '│' };
            ctx.set(x, y, cell(ch, fg, bg));
            // Past the edge there is nothing to draw, however large the indent.
            for i in 1..effective_indent.min(ctx.area.width) {
                ctx.set(x.saturating_add(i), y, cell(' ', None, bg));
            }
            x = x.saturating_add(effective_indent);
        }

        let row = &rows[index];
        if row.depth > 0 {
            let connector = if row.is_last { '└' } else { '├' };
            ctx.set(x, y, cell(connector, fg, bg));
            ctx.set(x.saturating_add(1), y, cell('─', fg, bg));
            x = x.saturating_add(2);
        }
        x
    }

    /// The label from `x`, cut at the area edge, with the fuzzy-matched
    /// characters in the highlight color
    fn draw_label(
        &self,
        ctx: &mut RenderContext,
        node: &TreeNode,
        x: u16,
        y: u16,
        fg: Option<Color>,
        bg: Option<Color>,
    ) {
        let area_width = ctx.area.width;
        let available_width = area_width.saturating_sub(x) as usize;
        let truncated = crate::utils::truncate_to_width(&node.label, available_width);
        let match_indices: Vec<usize> = self
            .get_match(&node.label)
            .map(|m| m.indices)
            .unwrap_or_default();

        let mut x = x;
        for (idx, ch) in truncated.chars().enumerate() {
            let cw = crate::utils::char_width(ch) as u16;
            if x.saturating_add(cw) > area_width {
                break;
            }
            let ch_fg = if match_indices.contains(&idx) {
                self.highlight_fg
            } else {
                fg
            };
            ctx.set(x, y, cell(ch, ch_fg, bg));
            x += cw;
        }
    }
}

/// The expand/collapse indicator and the icon; returns the column after them
fn draw_marker(
    ctx: &mut RenderContext,
    node: &TreeNode,
    x: u16,
    y: u16,
    fg: Option<Color>,
    bg: Option<Color>,
) -> u16 {
    let indicator = match (node.has_children(), node.expanded) {
        (true, true) => '▼',
        (true, false) => '▶',
        (false, _) => ' ',
    };
    ctx.set(x, y, cell(indicator, fg, bg));
    let mut x = x.saturating_add(1);
    if let Some(icon_ch) = node.icon {
        // A wide icon (📁) takes two columns
        ctx.set(x, y, cell(icon_ch, fg, bg));
        x = x.saturating_add((crate::utils::char_width(icon_ch) as u16).max(1));
    }
    x
}

/// For each ancestor of row `index`, root first: whether it is the last of
/// its siblings (then no `│` continues below it)
fn ancestors_last(rows: &[Row], index: usize) -> Vec<bool> {
    let mut flags = Vec::new();
    let mut parent = rows[index].parent;
    while let Some(p) = parent {
        flags.push(rows[p].is_last);
        parent = rows[p].parent;
    }
    flags.reverse();
    flags
}

/// A cell with optional colors
fn cell(ch: char, fg: Option<Color>, bg: Option<Color>) -> Cell {
    let mut cell = Cell::new(ch);
    cell.fg = fg;
    cell.bg = bg;
    cell
}
