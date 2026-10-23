//! Applying an edit at every cursor, and the undo steps edits make
//!
//! While an edit is applied, positions are kept as line and byte offset
//! ([`Mark`]): replacements go from the last cursor to the first, each mark
//! after a replacement moves with the text, and the marks become columns
//! again at the end. Each replacement is recorded with the text it removed,
//! so undo puts it back in reverse order.

use std::time::{Duration, Instant};

use super::pos::EditPos;
use super::text::{byte_of, col_of};
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

/// Where mark `m` goes when `from..to` is replaced by text ending at `end`:
/// before it stays, after it moves with the text, inside it goes to the end
fn shift(m: Mark, from: Mark, to: Mark, end: Mark) -> Mark {
    if m >= to {
        if m.line == to.line {
            Mark {
                line: end.line,
                byte: end.byte + (m.byte - to.byte),
            }
        } else {
            // A later line moves by the lines the replacement added or took
            Mark {
                line: m.line + end.line - to.line,
                byte: m.byte,
            }
        }
    } else if m <= from {
        m
    } else {
        end
    }
}

impl EditState {
    pub(super) fn mark(&self, pos: EditPos) -> Mark {
        Mark {
            line: pos.line,
            byte: byte_of(&self.lines[pos.line], pos.col),
        }
    }

    pub(super) fn pos(&self, mark: Mark) -> EditPos {
        EditPos {
            line: mark.line,
            col: col_of(&self.lines[mark.line], mark.byte),
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
        let mut edits: Vec<(usize, Mark, Mark, String)> = Vec::new();
        for (i, cursor) in self.cursors.iter().enumerate() {
            if let Some((from, to, text)) = plan(self, cursor) {
                let (from, to) = (self.mark(from), self.mark(to));
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
        let mut marks: Vec<(Mark, Mark)> = self
            .cursors
            .iter()
            .map(|c| (self.mark(c.anchor), self.mark(c.head)))
            .collect();
        let mut changes = Vec::new();
        for (owners, from, to, text) in merged.into_iter().rev() {
            let (removed, end) = self.splice(from, to, &text);
            for (i, (anchor, head)) in marks.iter_mut().enumerate() {
                if owners.contains(&i) {
                    *anchor = end;
                    *head = end;
                } else {
                    *anchor = shift(*anchor, from, to, end);
                    *head = shift(*head, from, to, end);
                }
            }
            changes.push(Change {
                at: from,
                removed,
                inserted: text,
            });
        }
        self.cursors = marks
            .into_iter()
            .map(|(anchor, head)| EditCursor::selecting(self.pos(anchor), self.pos(head)))
            .collect();
        self.normalize();
        // Text typed over the same text: the cursors moved, nothing to undo
        if changes.iter().all(|c| c.removed == c.inserted) {
            self.joinable = false;
            return false;
        }

        let step = Step {
            changes,
            before,
            before_primary,
            after: self.cursors.clone(),
            after_primary: self.primary,
            kind,
            time: (self.clock)(),
            joins: self.joinable,
            delay: self.group_delay,
        };
        self.history.push_merge(step);
        self.joinable = kind != Kind::Other;
        true
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
