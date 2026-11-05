//! Laying out the blocks built whole: table of contents, FIGlet headings,
//! code blocks and tables

use super::{parser, Line, Markdown, StyledText};
use crate::render::Modifier;
use crate::utils::figlet::{figlet_with_font, FigletFont};
use crate::utils::syntax::Language;
use crate::utils::unicode::{
    center_to_width, char_width, display_width, pad_to_width, right_align_to_width,
};
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
        ctx.push_wrapped(title, 0, &[]);

        for entry in &self.toc {
            let indent = "  ".repeat(entry.level.saturating_sub(1) as usize);
            let mut line = Line::new();
            line.push(StyledText::new(format!("{indent}- ")));
            line.push(
                StyledText::new(&entry.text)
                    .with_fg(self.config.toc_fg)
                    .with_modifier(Modifier::UNDERLINE),
            );
            let hang = StyledText::new(format!("{indent}  "));
            ctx.push_wrapped(line, 1, &[hang]);
        }

        let mut sep = Line::new();
        sep.push(StyledText::new("─".repeat(40)).with_fg(DARK_GRAY));
        ctx.push_row(sep);
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
            ctx.push_row(line);
        }
    }

    /// Lay out the collected code block: each source line highlighted, in a
    /// box as wide as its widest row. With a width, a line too long for the
    /// box goes on over the next rows, which keep the box's left edge and
    /// leave the line-number column blank; nothing of the code is dropped.
    pub(super) fn render_code_block(&self, ctx: &mut parser::ParserContext) {
        ctx.in_code_block = false;
        ctx.new_line();

        // pulldown-cmark may hand the block over in several text events;
        // join them and split on newlines so each source line is its own row.
        let code = std::mem::take(&mut ctx.code_block_lines).concat();
        // The box's sides take a column each
        let room = ctx
            .room()
            .map(|room| room.saturating_sub(if ctx.code_border { 2 } else { 0 }));

        // (prefix, code) per row
        let mut rows: Vec<(String, Vec<StyledText>)> = Vec::new();
        for (line_num, line) in code.lines().enumerate() {
            let edge = if ctx.code_border { "│ " } else { "" };
            let (prefix, blank) = if ctx.code_line_numbers {
                let number = format!("{:3}", line_num + 1);
                let blank = " ".repeat(display_width(&number));
                (format!("{edge}{number} │ "), format!("{edge}{blank} │ "))
            } else {
                (edge.to_string(), edge.to_string())
            };

            let tokens = if ctx.syntax_highlight && ctx.code_block_lang != Language::Unknown {
                ctx.highlighter.highlight_line(line, ctx.code_block_lang)
            } else {
                Vec::new()
            };
            let segments: Vec<StyledText> = if tokens.is_empty() {
                vec![StyledText::new(line).with_fg(ctx.code_fg)]
            } else {
                tokens
                    .iter()
                    .map(|token| {
                        let fg = ctx.highlighter.token_color(token.token_type);
                        StyledText::new(token.text.clone()).with_fg(fg)
                    })
                    .collect()
            };

            let code_room = room.map_or(0, |room| room.saturating_sub(display_width(&prefix)));
            for (i, part) in super::wrap::wrap_chars(&segments, code_room)
                .into_iter()
                .enumerate()
            {
                let prefix = if i == 0 {
                    prefix.clone()
                } else {
                    blank.clone()
                };
                rows.push((prefix, part));
            }
        }

        let row_width = |(prefix, code): &(String, Vec<StyledText>)| {
            display_width(prefix) + code.iter().map(|s| display_width(&s.text)).sum::<usize>()
        };
        // The box is as wide as its widest row (prefix included)
        let inner_width = rows.iter().map(row_width).max().unwrap_or(0).max(2);

        let border = |left: &str, right: &str| {
            let mut line = Line::new();
            line.push(
                StyledText::new(format!("{left}{}{right}", "─".repeat(inner_width)))
                    .with_fg(DISABLED_FG),
            );
            line
        };

        if ctx.code_border {
            ctx.push_row(border("┌", "┐"));
        }

        for row in &rows {
            let mut code_line = Line::new();
            let (prefix, code) = row;
            if !prefix.is_empty() {
                code_line.push(StyledText::new(prefix.clone()).with_fg(DISABLED_FG));
            }
            code_line.segments.extend(code.iter().cloned());
            if ctx.code_border {
                let pad = inner_width - row_width(row);
                code_line
                    .push(StyledText::new(format!("{} │", " ".repeat(pad))).with_fg(DISABLED_FG));
            }
            ctx.push_row(code_line);
        }

        if ctx.code_border {
            ctx.push_row(border("└", "┘"));
        }

        ctx.new_line();
    }

    /// Lay out the collected table: a box with the header row, a separator,
    /// and the body rows, each column as wide as its widest cell. A table
    /// wider than the room it has fits it (#1472): the widest columns give
    /// up columns first, and a cell's text wraps inside its column at word
    /// boundaries, so a row may take several lines. Nothing of a cell is
    /// dropped. When even a column per character cannot fit, the table is
    /// drawn as wide as its cells and cut at the edge.
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
        let cells = |col: usize| rows.iter().filter_map(move |row| row.get(col));
        let natural: Vec<usize> = (0..columns)
            .map(|col| {
                cells(col)
                    .map(|cell| display_width(cell))
                    .max()
                    .unwrap_or(0)
            })
            .collect();
        // A column is never narrower than its widest character
        let floors: Vec<usize> = (0..columns)
            .map(|col| {
                cells(col)
                    .flat_map(|cell| cell.chars().map(char_width))
                    .max()
                    .unwrap_or(0)
            })
            .collect();
        // Each column takes a space either side and a border after it, and
        // the box a border before the first
        let room = ctx
            .room()
            .and_then(|room| room.checked_sub(1 + 3 * columns));
        let widths = match room {
            Some(room) => fit_columns(&natural, &floors, room).unwrap_or(natural),
            None => natural,
        };

        let border = |left: &str, mid: &str, right: &str| {
            let segments: Vec<String> = widths.iter().map(|w| "─".repeat(w + 2)).collect();
            let mut line = Line::new();
            line.push(
                StyledText::new(format!("{left}{}{right}", segments.join(mid)))
                    .with_fg(DISABLED_FG),
            );
            line
        };

        ctx.push_row(border("┌", "┬", "┐"));
        for (row_idx, row) in rows.iter().enumerate() {
            let is_header = row_idx == 0;
            // Each cell's text as the lines it takes in its column
            let wrapped: Vec<Vec<String>> = widths
                .iter()
                .enumerate()
                .map(|(col, width)| {
                    let cell = row.get(col).map(String::as_str).unwrap_or("");
                    super::wrap::wrap_words(&[StyledText::new(cell)], *width)
                        .into_iter()
                        .map(|part| part.into_iter().map(|s| s.text).collect())
                        .collect()
                })
                .collect();
            let height = wrapped.iter().map(Vec::len).max().unwrap_or(1);
            for at in 0..height {
                let mut line = Line::new();
                line.push(StyledText::new("│").with_fg(DISABLED_FG));
                for (col, width) in widths.iter().enumerate() {
                    let part = wrapped[col].get(at).map(String::as_str).unwrap_or("");
                    let text = match alignments.get(col) {
                        Some(Alignment::Center) => center_to_width(part, *width),
                        Some(Alignment::Right) => right_align_to_width(part, *width),
                        _ => pad_to_width(part, *width),
                    };
                    let text = StyledText::new(format!(" {text} "));
                    line.push(if is_header {
                        text.with_fg(ctx.heading_fg).with_modifier(Modifier::BOLD)
                    } else {
                        text
                    });
                    line.push(StyledText::new("│").with_fg(DISABLED_FG));
                }
                ctx.push_row(line);
            }
            if is_header && rows.len() > 1 {
                ctx.push_row(border("├", "┼", "┤"));
            }
        }
        ctx.push_row(border("└", "┴", "┘"));

        ctx.new_line();
    }
}

/// Column widths summing to at most `room`: the widest columns are cut down
/// first, to the width of the next widest, and no column goes below its
/// floor or above its natural width. `None` when the floors alone are wider
/// than `room`.
fn fit_columns(natural: &[usize], floors: &[usize], room: usize) -> Option<Vec<usize>> {
    let total = natural.iter().sum::<usize>();
    if total <= room {
        return Some(natural.to_vec());
    }
    let capped = |cap: usize| -> Vec<usize> {
        natural
            .iter()
            .zip(floors)
            .map(|(&natural, &floor)| natural.min(cap).max(floor))
            .collect()
    };
    if floors.iter().sum::<usize>() > room {
        return None;
    }
    // The largest cap whose widths fit: the sum grows with the cap, and a
    // cap of 0 leaves only the floors, which fit
    let (mut low, mut high) = (0, natural.iter().copied().max().unwrap_or(0));
    while low < high {
        let mid = (low + high).div_ceil(2);
        if capped(mid).iter().sum::<usize>() <= room {
            low = mid;
        } else {
            high = mid - 1;
        }
    }
    let mut widths = capped(low);
    // The columns left out of a whole step up share what is left, first
    // columns first
    let mut spare = room - widths.iter().sum::<usize>();
    for (width, &natural) in widths.iter_mut().zip(natural) {
        if spare == 0 {
            break;
        }
        if *width == low && natural > low {
            *width += 1;
            spare -= 1;
        }
    }
    Some(widths)
}

#[cfg(test)]
mod tests {
    use super::fit_columns;

    #[test]
    fn columns_that_fit_keep_their_width() {
        assert_eq!(fit_columns(&[3, 5], &[1, 1], 8), Some(vec![3, 5]));
    }

    /// The widest column gives up columns first, down to the next widest
    #[test]
    fn the_widest_column_shrinks_first() {
        assert_eq!(fit_columns(&[3, 10], &[1, 1], 9), Some(vec![3, 6]));
        assert_eq!(fit_columns(&[3, 10], &[1, 1], 6), Some(vec![3, 3]));
        // Level, the one column over goes to the first
        assert_eq!(fit_columns(&[3, 10], &[1, 1], 5), Some(vec![3, 2]));
    }

    /// What one step of the cap leaves goes to the first capped columns
    #[test]
    fn the_spare_column_goes_to_the_first_capped_column() {
        assert_eq!(fit_columns(&[8, 8, 2], &[1, 1, 1], 11), Some(vec![5, 4, 2]));
    }

    /// A wide character keeps its column at two
    #[test]
    fn no_column_goes_below_its_floor() {
        assert_eq!(fit_columns(&[6, 6], &[2, 1], 3), Some(vec![2, 1]));
        assert_eq!(fit_columns(&[6, 6], &[2, 2], 3), None);
    }

    #[test]
    fn an_empty_column_stays_empty() {
        assert_eq!(fit_columns(&[0, 9], &[0, 1], 4), Some(vec![0, 4]));
    }
}
