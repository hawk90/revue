//! Drawing the diff viewer: split, unified and inline views

use super::pair::{inline_parts, Part, Segments};
use super::{ChangeType, DiffLine, DiffMode, DiffViewer};
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::widget::traits::{RenderContext, View};

/// Line rendering layout parameters
struct LineLayout {
    x: u16,
    y: u16,
    line_num_width: u16,
    /// The columns a line number is right-aligned in
    digits: usize,
    content_width: usize,
}

/// Which text of a diff line a row shows
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Side {
    /// The whole line: both texts of a modified pair
    Both,
    /// The old text of a modified pair (the unified `-` row)
    Old,
    /// The new text of a modified pair (the unified `+` row)
    New,
}

/// A row of the viewer: a diff line (by index), or a run of unchanged lines
/// folded away
pub(super) enum Row {
    Line(usize, Side),
    Fold(usize),
}

/// A run of text drawn in one style
type Run<'a> = (&'a str, Option<Color>, Option<Color>);

impl DiffViewer {
    /// The rows to draw: every diff line, or with a context set, the lines
    /// near a change and a fold for each longer unchanged run. The unified
    /// view draws a modified pair as its `-` row and its `+` row.
    pub(super) fn rows(&self) -> Vec<Row> {
        let lines = &self.diff_lines;
        let mut rows = Vec::new();
        let push = |rows: &mut Vec<Row>, i: usize| {
            if self.mode == DiffMode::Unified && lines[i].change == ChangeType::Modified {
                rows.push(Row::Line(i, Side::Old));
                rows.push(Row::Line(i, Side::New));
            } else {
                rows.push(Row::Line(i, Side::Both));
            }
        };
        let Some(context) = self.context_lines else {
            for i in 0..lines.len() {
                push(&mut rows, i);
            }
            return rows;
        };

        let mut near = vec![false; lines.len()];
        for (i, line) in lines.iter().enumerate() {
            if line.change != ChangeType::Equal {
                let end = i.saturating_add(context).min(lines.len() - 1);
                near[i.saturating_sub(context)..=end].fill(true);
            }
        }

        let mut i = 0;
        while i < lines.len() {
            if near[i] {
                push(&mut rows, i);
                i += 1;
                continue;
            }
            let start = i;
            while i < lines.len() && !near[i] {
                i += 1;
            }
            // A marker for one line would take as much room as the line
            if i - start == 1 {
                push(&mut rows, start);
            } else {
                rows.push(Row::Fold(i - start));
            }
        }
        rows
    }

    /// `rows` from the scrolled-to one on, as many as fit in `height`: the
    /// offset kept so the last page is full (#973)
    fn scrolled(&self, rows: Vec<Row>, height: usize) -> impl Iterator<Item = Row> {
        self.scroll.set_content(rows.len());
        self.scroll.set_viewport(height);
        let offset = self.scroll.offset();
        rows.into_iter().skip(offset).take(height)
    }

    /// The columns a line number is right-aligned in: at least four, more
    /// when the largest number has more digits, so a column is never cut
    /// and every row lines up (#1555)
    fn number_digits(&self) -> usize {
        let widest = self
            .diff_lines
            .iter()
            .flat_map(|line| [line.left_num, line.right_num])
            .flatten()
            .max()
            .unwrap_or(0);
        let digits = widest.checked_ilog10().map_or(1, |log| log + 1) as usize;
        digits.max(4)
    }

    /// Draw a fold row across the width
    fn render_fold(&self, ctx: &mut RenderContext, y: u16, hidden: usize) {
        let text = format!("⋯ {hidden} unchanged lines");
        let fg = self.colors.line_number;
        ctx.put_str_with(0, y, &text, ctx.area.width, |ch| {
            let mut cell = Cell::new(ch);
            cell.fg = Some(fg);
            cell
        });
    }

    /// The runs of one text of a modified pair: the changed words in the
    /// removed (`old`) or added colors, the rest on the modified background
    fn pair_runs<'a>(&self, line: &'a DiffLine, segments: &Segments, old: bool) -> Vec<Run<'a>> {
        let (text, runs, fg, bg) = if old {
            let c = &self.colors;
            (&line.left, &segments.old, c.removed_fg, c.removed_bg)
        } else {
            let c = &self.colors;
            (&line.right, &segments.new, c.added_fg, c.added_bg)
        };
        runs.iter()
            .map(|(changed, range)| {
                if *changed {
                    (&text[range.clone()], Some(fg), Some(bg))
                } else {
                    (&text[range.clone()], None, Some(self.colors.modified_bg))
                }
            })
            .collect()
    }

    /// The runs of an inline row for a modified pair: shared text on the
    /// modified background, removed words then added words in their colors
    fn inline_runs<'a>(&self, line: &'a DiffLine, segments: &Segments) -> Vec<Run<'a>> {
        let c = &self.colors;
        inline_parts(&line.left, &line.right, segments)
            .into_iter()
            .map(|(part, text)| match part {
                Part::Equal => (text, None, Some(c.modified_bg)),
                Part::Removed => (text, Some(c.removed_fg), Some(c.removed_bg)),
                Part::Added => (text, Some(c.added_fg), Some(c.added_bg)),
            })
            .collect()
    }

    /// The segments of diff line `index`, empty for a line that is not a
    /// modified pair
    fn segments_of(&self, index: usize) -> Segments {
        self.segments
            .get(index)
            .cloned()
            .flatten()
            .unwrap_or_default()
    }

    /// Draw `runs` one after another from `x`, stopping before column
    /// `end`; returns the columns used
    fn draw_runs(ctx: &mut RenderContext, x: u16, y: u16, end: u16, runs: &[Run]) -> u16 {
        let mut cx = x;
        for &(text, fg, bg) in runs {
            cx += ctx.put_str_with(cx, y, text, end, |ch| {
                let mut cell = Cell::new(ch);
                cell.fg = fg;
                cell.bg = bg;
                cell
            });
        }
        cx - x
    }

    /// Render split view
    fn render_split(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 10 || area.height < 3 {
            return;
        }

        let half_width = (area.width / 2).saturating_sub(1);
        // The digits and a space after them, cut at the half's edge
        let digits = self.number_digits();
        let line_num_width = if self.show_line_numbers {
            u16::try_from(digits + 1)
                .unwrap_or(u16::MAX)
                .min(half_width)
        } else {
            0
        };
        let content_width = half_width.saturating_sub(line_num_width) as usize;

        // Header
        self.render_header(ctx, half_width);

        // Content
        let visible_lines = (area.height - 1) as usize;
        for (i, row) in self.scrolled(self.rows(), visible_lines).enumerate() {
            let y = 1 + i as u16;
            let index = match row {
                Row::Line(index, _) => index,
                Row::Fold(hidden) => {
                    self.render_fold(ctx, y, hidden);
                    continue;
                }
            };

            // Left side
            let left_layout = LineLayout {
                x: 0,
                y,
                line_num_width,
                digits,
                content_width,
            };
            self.render_line_half(ctx, index, true, &left_layout);

            // Separator
            let mut sep = Cell::new('│');
            sep.fg = Some(self.colors.separator);
            ctx.set(half_width, y, sep);

            // Right side
            let right_layout = LineLayout {
                x: half_width + 1,
                y,
                line_num_width,
                digits,
                content_width,
            };
            self.render_line_half(ctx, index, false, &right_layout);
        }
    }

    /// Render one half of a split line
    fn render_line_half(
        &self,
        ctx: &mut RenderContext,
        index: usize,
        is_left: bool,
        layout: &LineLayout,
    ) {
        let LineLayout {
            x,
            y,
            line_num_width,
            digits,
            content_width,
        } = *layout;
        let line = &self.diff_lines[index];
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
                .map(|n| format!("{n:>digits$}"))
                .unwrap_or_else(|| " ".repeat(digits));
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
        let start = x + line_num_width;
        let end = start.saturating_add(content_width as u16);
        let runs = if line.change == ChangeType::Modified {
            self.pair_runs(line, &self.segments_of(index), is_left)
        } else {
            let fg = match line.change {
                ChangeType::Added => Some(self.colors.added_fg),
                ChangeType::Removed => Some(self.colors.removed_fg),
                _ => None,
            };
            vec![(content.as_str(), fg, bg)]
        };
        let used = Self::draw_runs(ctx, start, y, end, &runs);

        // Fill remaining with background
        for cx in start + used..end {
            let mut cell = Cell::new(' ');
            cell.bg = bg;
            ctx.set(cx, y, cell);
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

    /// Render the unified and inline views: one column, a `-` or `+`
    /// before each changed row. Unified draws a modified pair as its `-`
    /// row and its `+` row; inline as one `~` row, removed words before the
    /// added ones, as `git diff --word-diff=color` does.
    fn render_unified(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        // Both numbers, a colon between them and a space after
        let digits = self.number_digits();
        let line_num_width = if self.show_line_numbers {
            u16::try_from(2 * digits + 2).unwrap_or(u16::MAX)
        } else {
            0
        };
        let content_x = line_num_width.saturating_add(1);

        let visible_lines = area.height as usize;
        for (i, row) in self.scrolled(self.rows(), visible_lines).enumerate() {
            let y = i as u16;
            let (index, side) = match row {
                Row::Line(index, side) => (index, side),
                Row::Fold(hidden) => {
                    self.render_fold(ctx, y, hidden);
                    continue;
                }
            };
            let line = &self.diff_lines[index];
            let (left_num, right_num) = match side {
                Side::Both => (line.left_num, line.right_num),
                Side::Old => (line.left_num, None),
                Side::New => (None, line.right_num),
            };

            // Line numbers (left:right)
            if self.show_line_numbers {
                let num_str = format!(
                    "{:>digits$}:{:<digits$}",
                    left_num.map(|n| n.to_string()).unwrap_or_default(),
                    right_num.map(|n| n.to_string()).unwrap_or_default()
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
            let c = &self.colors;
            let (indicator, fg, bg) = match (line.change, side) {
                (ChangeType::Added, _) | (ChangeType::Modified, Side::New) => {
                    ('+', c.added_fg, c.added_bg)
                }
                (ChangeType::Removed, _) | (ChangeType::Modified, Side::Old) => {
                    ('-', c.removed_fg, c.removed_bg)
                }
                (ChangeType::Modified, Side::Both) => ('~', c.added_fg, c.modified_bg),
                (ChangeType::Equal, _) => (' ', Color::WHITE, Color::default()),
            };

            let mut ind_cell = Cell::new(indicator);
            ind_cell.fg = Some(fg);
            ind_cell.bg = Some(bg);
            ctx.set(line_num_width, y, ind_cell);

            // Content
            let runs = match (line.change, side) {
                (ChangeType::Modified, Side::Both) => {
                    self.inline_runs(line, &self.segments_of(index))
                }
                (ChangeType::Modified, side) => {
                    self.pair_runs(line, &self.segments_of(index), side == Side::Old)
                }
                (ChangeType::Removed, _) => vec![(line.left.as_str(), Some(fg), Some(bg))],
                _ => vec![(line.right.as_str(), Some(fg), Some(bg))],
            };
            Self::draw_runs(ctx, content_x, y, area.width, &runs);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    fn rows(viewer: &DiffViewer, w: u16, h: u16) -> Vec<String> {
        let mut buf = Buffer::new(w, h);
        viewer.render(&mut RenderContext::new(&mut buf, Rect::new(0, 0, w, h)));
        (0..h)
            .map(|y| {
                (0..w)
                    .map(|x| buf.get(x, y).unwrap().symbol)
                    .collect::<String>()
                    .trim_end()
                    .to_string()
            })
            .collect()
    }

    fn one_change() -> DiffViewer {
        DiffViewer::new()
            .compare("a\nb\nc\nd\ne\nf\ng\n", "a\nb\nc\nD\ne\nf\ng\n")
            .mode(DiffMode::Unified)
            .line_numbers(false)
    }

    #[test]
    fn without_context_every_line_is_shown() {
        assert_eq!(
            rows(&one_change(), 30, 8),
            [" a", " b", " c", "-d", "+D", " e", " f", " g"]
        );
    }

    #[test]
    fn context_folds_unchanged_runs_away_from_changes() {
        assert_eq!(
            rows(&one_change().context(1), 30, 8),
            [
                "⋯ 2 unchanged lines",
                " c",
                "-d",
                "+D",
                " e",
                "⋯ 2 unchanged lines",
                "",
                ""
            ]
        );
        // A run of one is shown rather than replaced by a marker as tall
        assert_eq!(
            rows(&one_change().context(2), 30, 8),
            [" a", " b", " c", "-d", "+D", " e", " f", " g"]
        );
    }

    #[test]
    fn context_folds_in_the_split_view() {
        let viewer = one_change().mode(DiffMode::Split).context(1);
        let rows = rows(&viewer, 30, 8);
        assert!(rows[1].contains("⋯ 2 unchanged lines"), "{:?}", rows[1]);
        assert!(rows[6].contains("⋯ 2 unchanged lines"), "{:?}", rows[6]);
    }
}
