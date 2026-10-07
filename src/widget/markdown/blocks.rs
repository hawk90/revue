//! Laying out the blocks built whole: table of contents, FIGlet headings,
//! code blocks and tables

use super::{parser, Line, Markdown, StyledText};
use crate::render::Modifier;
use crate::utils::figlet::{figlet_with_font, FigletFont};
use crate::utils::syntax::Language;
use crate::utils::unicode::{center_to_width, display_width, pad_to_width, right_align_to_width};
use crate::widget::theme::{DARK_GRAY, DISABLED_FG};

impl Markdown {
    /// Lay out the table of contents ahead of the document: the title, one
    /// entry per heading indented by its level, then a separator. Nothing is
    /// drawn when the document has no headings.
    pub(super) fn render_toc(&self, ctx: &mut parser::ParserContext) {
        if self.toc.is_empty() {
            return;
        }

        let mut title = Line::new();
        title.push(
            StyledText::new(&self.config.toc_title)
                .with_fg(self.config.heading_fg)
                .with_modifier(Modifier::BOLD),
        );
        ctx.lines.push(title);

        for entry in &self.toc {
            let indent = "  ".repeat(entry.level.saturating_sub(1) as usize);
            let mut line = Line::new();
            line.push(StyledText::new(format!("{indent}- ")));
            line.push(
                StyledText::new(&entry.text)
                    .with_fg(self.config.toc_fg)
                    .with_modifier(Modifier::UNDERLINE),
            );
            ctx.lines.push(line);
        }

        let mut sep = Line::new();
        sep.push(StyledText::new("─".repeat(40)).with_fg(DARK_GRAY));
        ctx.lines.push(sep);
    }

    /// Draw the finished heading as FIGlet big text, one row per art line,
    /// in place of the usual `#`-marked line.
    pub(super) fn render_figlet_heading(&self, ctx: &mut parser::ParserContext, font: FigletFont) {
        ctx.in_heading = false;
        ctx.current_modifier &= !Modifier::BOLD;
        ctx.current_fg = None;
        ctx.flush_line();
        let art = figlet_with_font(&ctx.heading_text, font);
        for art_line in art.lines() {
            let mut line = Line::new();
            line.push(
                StyledText::new(art_line)
                    .with_fg(ctx.heading_fg)
                    .with_modifier(Modifier::BOLD),
            );
            ctx.lines.push(line);
        }
    }

    pub(super) fn render_code_block(&self, ctx: &mut parser::ParserContext) {
        ctx.in_code_block = false;
        ctx.new_line();

        // pulldown-cmark may hand the block over in several text events;
        // join them and split on newlines so each source line is its own row.
        let code = std::mem::take(&mut ctx.code_block_lines).concat();
        let rows: Vec<(String, &str)> = code
            .lines()
            .enumerate()
            .map(|(line_num, line)| {
                let mut prefix = if ctx.code_border {
                    "│ ".to_string()
                } else {
                    String::new()
                };
                if ctx.code_line_numbers {
                    prefix.push_str(&format!("{:3} │ ", line_num + 1));
                }
                (prefix, line)
            })
            .collect();

        // The box is as wide as its widest line (prefix included)
        let inner_width = rows
            .iter()
            .map(|(prefix, line)| display_width(prefix) + display_width(line))
            .max()
            .unwrap_or(0)
            .max(2);

        let border = |left: &str, right: &str| {
            let mut line = Line::new();
            line.push(
                StyledText::new(format!("{left}{}{right}", "─".repeat(inner_width)))
                    .with_fg(DISABLED_FG),
            );
            line
        };

        if ctx.code_border {
            ctx.lines.push(border("┌", "┐"));
        }

        for (prefix, line) in &rows {
            let mut code_line = Line::new();

            if !prefix.is_empty() {
                code_line.push(StyledText::new(prefix.clone()).with_fg(DISABLED_FG));
            }

            // Apply syntax highlighting if enabled
            let tokens = if ctx.syntax_highlight && ctx.code_block_lang != Language::Unknown {
                ctx.highlighter.highlight_line(line, ctx.code_block_lang)
            } else {
                Vec::new()
            };
            if tokens.is_empty() {
                code_line.push(StyledText::new(*line).with_fg(ctx.code_fg));
            } else {
                // Render highlighted code - tokens contain the text directly
                for token in &tokens {
                    let fg = ctx.highlighter.token_color(token.token_type);
                    code_line.push(StyledText::new(token.text.clone()).with_fg(fg));
                }
            }

            if ctx.code_border {
                let pad = inner_width - display_width(prefix) - display_width(line);
                code_line
                    .push(StyledText::new(format!("{} │", " ".repeat(pad))).with_fg(DISABLED_FG));
            }

            ctx.lines.push(code_line);
        }

        if ctx.code_border {
            ctx.lines.push(border("└", "┘"));
        }

        ctx.new_line();
    }

    /// Lay out the collected table: a box with the header row, a separator,
    /// and the body rows, each column as wide as its widest cell. Rows wider
    /// than the widget are clipped at render time like any other line.
    pub(super) fn render_table(&self, ctx: &mut parser::ParserContext) {
        use pulldown_cmark::Alignment;

        ctx.in_table = false;
        ctx.in_table_head = false;
        let rows = std::mem::take(&mut ctx.table_rows);
        let alignments = std::mem::take(&mut ctx.table_alignments);

        let columns = rows.iter().map(Vec::len).max().unwrap_or(0);
        if columns == 0 {
            return;
        }
        let widths: Vec<usize> = (0..columns)
            .map(|col| {
                rows.iter()
                    .filter_map(|row| row.get(col))
                    .map(|cell| display_width(cell))
                    .max()
                    .unwrap_or(0)
            })
            .collect();

        let border = |left: &str, mid: &str, right: &str| {
            let segments: Vec<String> = widths.iter().map(|w| "─".repeat(w + 2)).collect();
            let mut line = Line::new();
            line.push(
                StyledText::new(format!("{left}{}{right}", segments.join(mid)))
                    .with_fg(DISABLED_FG),
            );
            line
        };

        ctx.lines.push(border("┌", "┬", "┐"));
        for (row_idx, row) in rows.iter().enumerate() {
            let is_header = row_idx == 0;
            let mut line = Line::new();
            line.push(StyledText::new("│").with_fg(DISABLED_FG));
            for (col, width) in widths.iter().enumerate() {
                let cell = row.get(col).map(String::as_str).unwrap_or("");
                let text = match alignments.get(col) {
                    Some(Alignment::Center) => center_to_width(cell, *width),
                    Some(Alignment::Right) => right_align_to_width(cell, *width),
                    _ => pad_to_width(cell, *width),
                };
                let text = StyledText::new(format!(" {text} "));
                line.push(if is_header {
                    text.with_fg(ctx.heading_fg).with_modifier(Modifier::BOLD)
                } else {
                    text
                });
                line.push(StyledText::new("│").with_fg(DISABLED_FG));
            }
            ctx.lines.push(line);
            if is_header && rows.len() > 1 {
                ctx.lines.push(border("├", "┼", "┤"));
            }
        }
        ctx.lines.push(border("└", "┴", "┘"));

        ctx.new_line();
    }
}
