//! Moving the cursors

use unicode_segmentation::UnicodeSegmentation;

use super::pos::EditPos;
use super::text::{category, col_at_screen, screen_col, Category};
use super::EditCursor;
use super::EditState;

/// Where a move takes each cursor
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EditMotion {
    /// One character left, onto the end of the line above at a line start
    Left,
    /// One character right, onto the start of the next line at a line end
    Right,
    /// Over whitespace, then one run of word or other characters, left
    WordLeft,
    /// Over whitespace, then one run of word or other characters, right
    WordRight,
    /// One line up, at the goal column; the start of the text from the
    /// first line
    Up,
    /// One line down, at the goal column; the end of the text from the last
    /// line
    Down,
    /// Up by this many lines, at the goal column
    PageUp(usize),
    /// Down by this many lines, at the goal column
    PageDown(usize),
    /// The start of the line
    LineStart,
    /// The end of the line
    LineEnd,
    /// The start of the text
    Start,
    /// The end of the text
    End,
}

impl EditMotion {
    /// Whether this move goes up or down, keeping the goal column
    fn vertical(self) -> bool {
        matches!(
            self,
            Self::Up | Self::Down | Self::PageUp(_) | Self::PageDown(_)
        )
    }
}

impl EditState {
    /// Move every cursor; with `extend` the selections grow to the new
    /// positions, without it they collapse (a Left, Right or word move from
    /// a selection only collapses it to that side). A single-line field
    /// does not move up or down. Returns whether any cursor changed.
    pub fn move_cursors(&mut self, motion: EditMotion, extend: bool) -> bool {
        if self.single_line && motion.vertical() {
            return false;
        }
        let before = self.cursors.clone();
        let moved: Vec<EditCursor> = before
            .iter()
            .map(|&c| self.moved(c, motion, extend))
            .collect();
        let primary = self.primary;
        self.cursors = moved;
        self.primary = primary;
        self.normalize();
        self.joinable = false;
        self.cursors != before
    }

    fn moved(&self, c: EditCursor, motion: EditMotion, extend: bool) -> EditCursor {
        if !extend && c.has_selection() {
            let (from, to) = c.range();
            match motion {
                EditMotion::Left | EditMotion::WordLeft => return EditCursor::at(from),
                EditMotion::Right | EditMotion::WordRight => return EditCursor::at(to),
                _ => {}
            }
        }
        let (head, goal) = self.target(c.head, c.goal, motion);
        EditCursor {
            head,
            anchor: if extend { c.anchor } else { head },
            goal: motion.vertical().then_some(goal).flatten(),
        }
    }

    /// Where `motion` takes a cursor at `pos`, and the goal column it keeps
    fn target(
        &self,
        pos: EditPos,
        goal: Option<usize>,
        motion: EditMotion,
    ) -> (EditPos, Option<usize>) {
        let last = self.lines.len() - 1;
        let len = |line: usize| self.line_len(line);
        let at =
            |line: usize, goal: usize| EditPos::new(line, col_at_screen(&self.lines[line], goal));
        let goal = goal.unwrap_or_else(|| screen_col(&self.lines[pos.line], pos.col));
        let pos = match motion {
            EditMotion::Left if pos.col > 0 => EditPos::new(pos.line, pos.col - 1),
            EditMotion::Left if pos.line > 0 => EditPos::new(pos.line - 1, len(pos.line - 1)),
            EditMotion::Left => pos,
            EditMotion::Right if pos.col < len(pos.line) => EditPos::new(pos.line, pos.col + 1),
            EditMotion::Right if pos.line < last => EditPos::new(pos.line + 1, 0),
            EditMotion::Right => pos,
            EditMotion::WordLeft => self.word_target(pos, false),
            EditMotion::WordRight => self.word_target(pos, true),
            EditMotion::Up if pos.line == 0 => EditPos::new(0, 0),
            EditMotion::Up => at(pos.line - 1, goal),
            EditMotion::Down if pos.line == last => EditPos::new(last, len(last)),
            EditMotion::Down => at(pos.line + 1, goal),
            EditMotion::PageUp(rows) => at(pos.line.saturating_sub(rows.max(1)), goal),
            EditMotion::PageDown(rows) => at((pos.line + rows.max(1)).min(last), goal),
            EditMotion::LineStart => EditPos::new(pos.line, 0),
            EditMotion::LineEnd => EditPos::new(pos.line, len(pos.line)),
            EditMotion::Start => EditPos::new(0, 0),
            EditMotion::End => self.end(),
        };
        (pos, Some(goal))
    }

    /// Where a word move from `pos` stops, as CodeMirror's `byGroup`: at a
    /// line edge it crosses the line break; otherwise it takes the next
    /// character, skips whitespace, and goes on while the category stays
    fn word_target(&self, pos: EditPos, forward: bool) -> EditPos {
        let line = &self.lines[pos.line];
        let clusters: Vec<&str> = line.graphemes(true).collect();
        if forward && pos.col == clusters.len() {
            return if pos.line + 1 < self.lines.len() {
                EditPos::new(pos.line + 1, 0)
            } else {
                pos
            };
        }
        if !forward && pos.col == 0 {
            return match pos.line {
                0 => pos,
                line => EditPos::new(line - 1, self.line_len(line - 1)),
            };
        }
        let mut col = pos.col;
        let next = |col: usize| {
            if forward {
                clusters.get(col).copied()
            } else {
                col.checked_sub(1).and_then(|c| clusters.get(c).copied())
            }
        };
        let step = |col: usize| if forward { col + 1 } else { col - 1 };
        let mut cat = category(next(col).unwrap_or(""));
        col = step(col);
        while let Some(cluster) = next(col) {
            let next_cat = category(cluster);
            if cat == Category::Space {
                cat = next_cat;
            }
            if cat != next_cat {
                break;
            }
            col = step(col);
        }
        EditPos::new(pos.line, col)
    }
}
