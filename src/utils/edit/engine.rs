//! Applying an edit at every cursor, and the undo steps edits make
//!
//! While an edit is applied, positions are kept as line and byte offset
//! ([`Mark`]): replacements go from the last cursor to the first, each mark
//! after a replacement moves with the text, and the marks become columns
//! again at the end. Each replacement is recorded with the text it removed,
//! so undo puts it back in reverse order.

use std::collections::HashMap;
use std::time::{Duration, Instant};

use unicode_segmentation::UnicodeSegmentation;

use super::pos::EditPos;
use super::text::byte_of;
use super::EditCursor;
use super::EditState;
use crate::utils::undo::Mergeable;

/// A position as line and byte offset in that line
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct Mark {
    pub(super) line: usize,
    pub(super) byte: usize,
}

/// One replacement: at `at`, `removed` was replaced by `inserted`
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Change {
    at: Mark,
    removed: String,
    inserted: String,
}

/// What an edit was, for joining undo steps
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Kind {
    /// Typing a character
    Type,
    /// Backspace, Delete and the word deletes
    Delete,
    /// Anything else: paste, a new line, a line command
    Other,
}

/// One undo step: the replacements in the order they were made, and the
/// cursors before and after
#[derive(Clone, Debug)]
pub(super) struct Step {
    changes: Vec<Change>,
    before: Vec<EditCursor>,
    before_primary: usize,
    after: Vec<EditCursor>,
    after_primary: usize,
    kind: Kind,
    /// The text the step holds
    bytes: usize,
    /// When the last edit in the step was made
    time: Instant,
    /// Whether this step may join the one before it: typing or a delete,
    /// made within `delay` of it, with no cursor move since
    joins: bool,
    delay: Duration,
}

impl Mergeable for Step {
    fn can_merge(&self, next: &Self) -> bool {
        next.joins
            && self.kind != Kind::Other
            && next.kind != Kind::Other
            && next.time.saturating_duration_since(self.time) < next.delay
            && next.before == self.after
    }

    fn merge(mut self, next: Self) -> Self {
        self.bytes += next.bytes;
        self.changes.extend(next.changes);
        self.after = next.after;
        self.after_primary = next.after_primary;
        self.time = next.time;
        self
    }
}

/// Where `text` ends when it is put at `at`
fn end_of(at: Mark, text: &str) -> Mark {
    match text.rfind('\n') {
        Some(i) => Mark {
            line: at.line + text.matches('\n').count(),
            byte: text.len() - i - 1,
        },
        None => Mark {
            line: at.line,
            byte: at.byte + text.len(),
        },
    }
}

/// Where each cursor's marks go when the replacements `edits`, in order and not
/// overlapping, are made: before a replacement a mark stays, after it the mark
/// moves with the text, and inside it, or if the cursor made it, the mark goes
/// to the end of the new text.
///
/// One pass over the marks and the edits in order, so a thousand cursors do not
/// cost a thousand times a thousand steps.
fn moved_marks(
    marks: Vec<(Mark, Mark)>,
    edits: &[(Vec<usize>, Mark, Mark, String)],
) -> Vec<(Mark, Mark)> {
    // What the replacements so far did: lines moved, and the columns moved on
    // the line the last one ended on.
    let mut lines = 0isize;
    let mut last: Option<(usize, isize)> = None;
    let moved = |m: Mark, lines: isize, last: Option<(usize, isize)>| Mark {
        line: m.line.saturating_add_signed(lines),
        byte: match last {
            Some((line, bytes)) if line == m.line => m.byte.saturating_add_signed(bytes),
            _ => m.byte,
        },
    };

    let mut flat: Vec<(Mark, usize, bool)> = Vec::with_capacity(marks.len() * 2);
    for (i, &(anchor, head)) in marks.iter().enumerate() {
        flat.push((anchor, i, false));
        flat.push((head, i, true));
    }
    flat.sort_unstable_by_key(|m| m.0);

    let mut out = marks;
    let set = |at: (usize, bool), to: Mark, out: &mut Vec<(Mark, Mark)>| {
        if at.1 {
            out[at.0].1 = to;
        } else {
            out[at.0].0 = to;
        }
    };
    let mut owned: Vec<(usize, Mark)> = Vec::new();
    let mut flat = flat.into_iter().peekable();
    for (owners, from, to, text) in edits {
        let end = end_of(moved(*from, lines, last), text);
        while let Some(&(m, i, head)) = flat.peek().filter(|f| f.0 < *to) {
            let goes = if m <= *from {
                moved(m, lines, last)
            } else {
                end
            };
            set((i, head), goes, &mut out);
            flat.next();
        }
        lines = end.line as isize - to.line as isize;
        last = Some((to.line, end.byte as isize - to.byte as isize));
        owned.extend(owners.iter().map(|&i| (i, end)));
    }
    for (m, i, head) in flat {
        set((i, head), moved(m, lines, last), &mut out);
    }
    for (i, end) in owned {
        out[i] = (end, end);
    }
    out
}

/// Each line's cluster starts, so that many cursors on a long line do not each
/// walk it again
#[derive(Default)]
struct ClusterStarts {
    by_line: HashMap<usize, Vec<usize>>,
}

impl ClusterStarts {
    fn of(&mut self, lines: &[String], line: usize) -> &[usize] {
        self.by_line
            .entry(line)
            .or_insert_with(|| lines[line].grapheme_indices(true).map(|(i, _)| i).collect())
    }

    /// The mark of a position: where its cluster starts
    fn mark(&mut self, lines: &[String], pos: EditPos) -> Mark {
        let len = lines[pos.line].len();
        Mark {
            line: pos.line,
            byte: self
                .of(lines, pos.line)
                .get(pos.col)
                .copied()
                .unwrap_or(len),
        }
    }

    /// The column of a mark: the clusters that start before its byte
    fn pos(&mut self, lines: &[String], mark: Mark) -> EditPos {
        EditPos {
            line: mark.line,
            col: self
                .of(lines, mark.line)
                .partition_point(|&i| i < mark.byte),
        }
    }
}

impl EditState {
    pub(super) fn mark(&self, pos: EditPos) -> Mark {
        Mark {
            line: pos.line,
            byte: byte_of(&self.lines[pos.line], pos.col),
        }
    }

    /// The text between two marks
    fn text_between(&self, from: Mark, to: Mark) -> String {
        if from.line == to.line {
            return self.lines[from.line][from.byte..to.byte].to_string();
        }
        let mut out = self.lines[from.line][from.byte..].to_string();
        for line in &self.lines[from.line + 1..to.line] {
            out.push('\n');
            out.push_str(line);
        }
        out.push('\n');
        out.push_str(&self.lines[to.line][..to.byte]);
        out
    }

    /// Replace `from..to` with `text` (which may hold newlines); returns
    /// what was removed and where the new text ends
    fn splice(&mut self, from: Mark, to: Mark, text: &str) -> (String, Mark) {
        let removed = self.text_between(from, to);
        let joined = format!(
            "{}{}{}",
            &self.lines[from.line][..from.byte],
            text,
            &self.lines[to.line][to.byte..]
        );
        let end = end_of(from, text);
        let new_lines: Vec<String> = joined.split('\n').map(String::from).collect();
        self.lines.splice(from.line..=to.line, new_lines);
        (removed, end)
    }

    /// Replace, at each cursor, the range `plan` gives it with its text,
    /// from the last cursor to the first, and record one undo step. A cursor
    /// whose plan is `None` is left in place (moved with the text). Each
    /// edited cursor ends after its new text, selecting nothing. Returns
    /// whether the text changed.
    pub(super) fn apply(
        &mut self,
        kind: Kind,
        mut plan: impl FnMut(&Self, &EditCursor) -> Option<(EditPos, EditPos, String)>,
    ) -> bool {
        if self.read_only {
            return false;
        }
        let mut starts = ClusterStarts::default();
        let mut edits: Vec<(usize, Mark, Mark, String)> = Vec::new();
        for (i, cursor) in self.cursors.iter().enumerate() {
            if let Some((from, to, text)) = plan(self, cursor) {
                let (from, to) = (starts.mark(&self.lines, from), starts.mark(&self.lines, to));
                if from != to || !text.is_empty() {
                    edits.push((i, from, to, self.fit_text(&text)));
                }
            }
        }
        if edits.is_empty() {
            return false;
        }
        // Ranges that overlap (two word deletes reaching the same word) are
        // one replacement
        edits.sort_by_key(|e| e.1);
        let mut merged: Vec<(Vec<usize>, Mark, Mark, String)> = Vec::new();
        for (i, from, to, text) in edits {
            match merged.last_mut() {
                Some(last) if from < last.2 => {
                    last.0.push(i);
                    last.2 = last.2.max(to);
                    last.3.push_str(&text);
                }
                _ => merged.push((vec![i], from, to, text)),
            }
        }
        self.limit_lines(&mut merged);

        let before = self.cursors.clone();
        let before_primary = self.primary;
        let marks: Vec<(Mark, Mark)> = self
            .cursors
            .iter()
            .map(|c| {
                (
                    starts.mark(&self.lines, c.anchor),
                    starts.mark(&self.lines, c.head),
                )
            })
            .collect();
        let marks = moved_marks(marks, &merged);
        let mut changes = Vec::with_capacity(merged.len());
        for (_, from, to, text) in merged.into_iter().rev() {
            let (removed, _) = self.splice(from, to, &text);
            changes.push(Change {
                at: from,
                removed,
                inserted: text,
            });
        }
        let mut starts = ClusterStarts::default();
        self.cursors = marks
            .into_iter()
            .map(|(anchor, head)| {
                EditCursor::selecting(
                    starts.pos(&self.lines, anchor),
                    starts.pos(&self.lines, head),
                )
            })
            .collect();
        self.normalize();
        // Text typed over the same text: the cursors moved, nothing to undo
        if changes.iter().all(|c| c.removed == c.inserted) {
            self.joinable = false;
            return false;
        }

        let bytes = changes
            .iter()
            .map(|c| c.removed.len() + c.inserted.len())
            .sum();
        let step = Step {
            changes,
            before,
            before_primary,
            after: self.cursors.clone(),
            after_primary: self.primary,
            kind,
            bytes,
            time: (self.clock)(),
            joins: self.joinable,
            delay: self.group_delay,
        };
        self.history.push_merge(step);
        self.trim_history();
        self.joinable = kind != Kind::Other;
        true
    }

    /// Drop the oldest steps past [`history_bytes`](Self::history_bytes), keeping
    /// the last.
    fn trim_history(&mut self) {
        let stack = &mut self.history.undo_stack;
        let mut bytes: usize = stack.iter().map(|step| step.bytes).sum();
        while bytes > self.history_bytes && stack.len() > 1 {
            bytes -= stack.pop_front().map_or(0, |step| step.bytes);
        }
    }

    /// Make the cursors as they are now the ones the last step ends with,
    /// for an edit that places its cursors itself
    pub(super) fn set_last_step_cursors(&mut self) {
        if let Some(step) = self.history.undo_stack.back_mut() {
            step.after = self.cursors.clone();
            step.after_primary = self.primary;
        }
    }

    /// Undo the last step: put back what it removed and the cursors it
    /// started with
    pub fn undo(&mut self) -> bool {
        if self.read_only {
            return false;
        }
        let Some(step) = self.history.undo() else {
            return false;
        };
        for change in step.changes.iter().rev() {
            let to = end_of(change.at, &change.inserted);
            self.splice(change.at, to, &change.removed);
        }
        self.cursors = step.before;
        self.primary = step.before_primary;
        self.joinable = false;
        true
    }

    /// Redo the last undone step, with the cursors it ended with
    pub fn redo(&mut self) -> bool {
        if self.read_only {
            return false;
        }
        let Some(step) = self.history.redo() else {
            return false;
        };
        for change in &step.changes {
            let to = end_of(change.at, &change.removed);
            self.splice(change.at, to, &change.inserted);
        }
        self.cursors = step.after;
        self.primary = step.after_primary;
        self.joinable = false;
        true
    }
}
