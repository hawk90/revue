//! JsonViewer rendering: the `View` impl

use super::JsonViewer;
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::widget::data::json_viewer::helpers::line_number_width;
use crate::widget::data::json_viewer::types::{JsonNode, JsonType};
use crate::widget::traits::{RenderContext, View};

impl View for JsonViewer {
    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 5 || area.height < 2 {
            return;
        }

        let nodes = self.get_visible_nodes();
        let line_num_width = line_number_width(self.show_line_numbers, nodes.len());
        let content_width = area.width.saturating_sub(line_num_width);

        // The rows in view, following the selection (#973)
        let visible_rows = area.height as usize;
        self.scroll.set_content(nodes.len());
        self.scroll.set_viewport(visible_rows);
        if let Some(selected) = self.selection.index() {
            self.scroll.track(selected..selected + 1);
        }
        let scroll = self.scroll.offset();

        for (visible_idx, node) in nodes.iter().skip(scroll).take(visible_rows).enumerate() {
            let y = visible_idx as u16;
            if y >= area.height {
                break;
            }
            let line_idx = scroll + visible_idx;
            self.render_row(ctx, node, line_idx, y, line_num_width, content_width);
        }
    }

    crate::impl_view_meta!("JsonViewer");
}

impl JsonViewer {
    /// One node on row `y`: line number, indicator, key, value, type badge,
    /// then the row's background to the edge
    fn render_row(
        &self,
        ctx: &mut RenderContext,
        node: &JsonNode,
        line_idx: usize,
        y: u16,
        line_num_width: u16,
        content_width: u16,
    ) {
        let is_selected = self.selection.is_selected(line_idx);
        let is_match = self.search_state.search_matches.contains(&node.path);
        let plain_bg = if is_selected {
            self.selected_bg
        } else {
            self.bg
        };

        if self.show_line_numbers {
            let num_str = format!(
                "{:>width$}",
                line_idx + 1,
                width = (line_num_width - 1) as usize
            );
            put(
                ctx,
                0,
                y,
                &num_str,
                self.line_number_fg,
                plain_bg,
                Modifier::empty(),
            );
        }

        let depth = u16::try_from(node.depth).unwrap_or(u16::MAX);
        let indent = depth.saturating_mul(self.indent_size);
        let mut x = line_num_width.saturating_add(indent);

        if node.is_container() {
            let indicator = if self.collapsed.contains(&node.path) {
                "▶ "
            } else {
                "▼ "
            };
            x = put(
                ctx,
                x,
                y,
                indicator,
                self.bracket_fg,
                plain_bg,
                Modifier::empty(),
            );
        }

        let (fg, bg) = if is_selected {
            (self.selected_fg, self.selected_bg)
        } else if is_match {
            (self.match_fg, self.match_bg)
        } else {
            // The base row takes the stylesheet; selection, matches and the
            // per-token colors keep theirs.
            (
                self.fg.or_else(|| ctx.css_color_if_set()),
                self.bg.or_else(|| ctx.css_background_if_set()),
            )
        };
        // Tokens keep their own colors unless the row is selected
        let token = |own: Option<Color>| if is_selected { fg } else { own };

        if !node.key.is_empty() {
            let key_display = format!("\"{}\"", node.key);
            x = put(
                ctx,
                x,
                y,
                &key_display,
                token(self.key_fg),
                bg,
                Modifier::empty(),
            );
            x = put(ctx, x, y, ": ", fg.or(self.fg), bg, Modifier::empty());
        }

        if let Some((text, own_fg)) = self.value_text(node, content_width, indent) {
            x = put(ctx, x, y, &text, token(own_fg), bg, Modifier::empty());
        }

        if self.show_type_badges {
            let badge = format!(" {}", type_name(&node.value_type));
            x = put(
                ctx,
                x,
                y,
                &badge,
                token(self.line_number_fg),
                bg,
                Modifier::DIM,
            );
        }

        let width = ctx.area.width;
        while x < width {
            let mut cell = Cell::new(' ');
            cell.bg = bg;
            ctx.set(x, y, cell);
            x += 1;
        }
    }

    /// The text shown for a node's value, with its type's color: brackets
    /// (or a collapsed summary) for containers, the quoted string cut to the
    /// content width, the number, boolean or `null`
    fn value_text(
        &self,
        node: &JsonNode,
        content_width: u16,
        indent: u16,
    ) -> Option<(String, Option<Color>)> {
        let collapsed = self.collapsed.contains(&node.path);
        match &node.value_type {
            JsonType::Object => Some((container_text(node, collapsed, '{', '}'), self.bracket_fg)),
            JsonType::Array => Some((container_text(node, collapsed, '[', ']'), self.bracket_fg)),
            JsonType::String => node.value.as_ref().map(|value| {
                let display = format!("\"{}\"", value);
                let truncated: String = display
                    .chars()
                    .take((content_width.saturating_sub(indent.saturating_add(2))) as usize)
                    .collect();
                (truncated, self.string_fg)
            }),
            JsonType::Number => node.value.clone().map(|value| (value, self.number_fg)),
            JsonType::Boolean => node.value.clone().map(|value| (value, self.bool_fg)),
            JsonType::Null => Some(("null".to_string(), self.null_fg)),
        }
    }
}

/// A container's brackets: the opening one while it is expanded, both when
/// it is empty, and a summary with its item count when it is collapsed
fn container_text(node: &JsonNode, collapsed: bool, open: char, close: char) -> String {
    if collapsed {
        format!("{open}...{close} ({} items)", node.child_count())
    } else if node.children.is_empty() {
        format!("{open}{close}")
    } else {
        open.to_string()
    }
}

fn type_name(value_type: &JsonType) -> &'static str {
    match value_type {
        JsonType::Object => "object",
        JsonType::Array => "array",
        JsonType::String => "string",
        JsonType::Number => "number",
        JsonType::Boolean => "boolean",
        JsonType::Null => "null",
    }
}

/// `text` from column `x`, one cell per character, dropping the characters
/// past the area's right edge; returns the column after the last one drawn
fn put(
    ctx: &mut RenderContext,
    x: u16,
    y: u16,
    text: &str,
    fg: Option<Color>,
    bg: Option<Color>,
    modifier: Modifier,
) -> u16 {
    // In terminal columns: a key or value in CJK takes two per character
    let max_x = ctx.area.width;
    x + ctx.put_str_with(x, y, text, max_x, |ch| {
        let mut cell = Cell::new(ch);
        cell.fg = fg;
        cell.bg = bg;
        cell.modifier |= modifier;
        cell
    })
}
