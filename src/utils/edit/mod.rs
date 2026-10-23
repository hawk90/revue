//! The text a field edits: lines, cursors with selections, and undo
//!
//! [`EditState`] is one edit model for every text field, single-line or
//! multi-line (#1261), kept apart from the widget that draws it, as
//! [`ScrollState`](super::ScrollState) is for scrolling.
//!
//! - **Positions** are a line and a column in grapheme clusters
//!   ([`EditPos`]), so a cursor never lands inside what the user sees as one
//!   character. Left, Right and Delete go over a whole cluster; Backspace
//!   takes the last code point of the cluster before the cursor (a dependent
//!   vowel, a Hangul jamo, a combining mark), except an emoji sequence, which
//!   goes whole.
//! - **Cursors**: one or more, each with its own selection. Every edit
//!   applies at every cursor, and cursors that meet are merged.
//! - **Words** are runs of one category: letters and numbers of any script
//!   and `_`, whitespace, or anything else (as CodeMirror's groups). A word
//!   move skips whitespace and goes over one run; `foo.bar(x)` stops at
//!   `foo`, `.`, `bar`, `(`, `x`.
//! - **Up and Down** aim for the screen column the move started from, kept
//!   across short lines.
//! - **Undo** steps restore the text and the cursors and selections. Typing
//!   and deletes made within [`group_delay`](EditState::group_delay) (500 ms)
//!   of each other, with no cursor move between, are one step (as
//!   CodeMirror's history); any other edit is a step of its own.
//! - **Single-line** fields ([`EditState::single_line`]) turn a newline in
//!   typed or pasted text into a space, and leave Enter, Up and Down to the
//!   widget.
//! - **Read-only** text moves and selects but does not change.
//!
//! [`EditState::edit_key`] applies the standard keys; a widget handles its
//! own keys first and passes the rest on.
//!
//! # Example
//!
//! ```
//! use revue::event::{Key, KeyEvent};
//! use revue::utils::{EditOutcome, EditPos, EditState};
//!
//! let mut field = EditState::single_line().with_text("hello");
//! field.set_cursor(EditPos::new(0, 5));
//! assert_eq!(field.edit_key(&KeyEvent::new(Key::Char('!')), 1), EditOutcome::Edited);
//! assert_eq!(field.text(), "hello!");
//! assert!(field.undo());
//! assert_eq!(field.text(), "hello");
//! ```

mod engine;
mod keys;
mod motion;
mod pos;
mod text;

use std::rc::Rc;
use std::time::{Duration, Instant};

use engine::{Kind, Step};
use text::{backspace_len, byte_of, category, cluster_at, cluster_count, Category};

pub use keys::EditOutcome;
pub use motion::EditMotion;
pub use pos::EditPos;

use crate::utils::undo::UndoHistory;

/// Steps of undo kept by default
const DEFAULT_HISTORY: usize = 100;

/// Typing within this of the last edit joins its undo step, by default
const DEFAULT_GROUP_DELAY: Duration = Duration::from_millis(500);

/// A cursor and the selection it makes: the text between its anchor and
/// its head. With the anchor on the head nothing is selected.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct EditCursor {
    head: EditPos,
    anchor: EditPos,
    /// The screen column Up and Down aim for, kept across short lines
    goal: Option<usize>,
}

impl EditCursor {
    /// A cursor at `pos`, selecting nothing
    pub const fn at(pos: EditPos) -> Self {
        Self {
            head: pos,
            anchor: pos,
            goal: None,
        }
    }

    /// A cursor at `head` selecting back to `anchor`
    pub const fn selecting(anchor: EditPos, head: EditPos) -> Self {
        Self {
            head,
            anchor,
            goal: None,
        }
    }

    /// Where the cursor is: the end of the selection it moves
    pub fn head(&self) -> EditPos {
        self.head
    }

    /// Where the selection started; the head when nothing is selected
    pub fn anchor(&self) -> EditPos {
        self.anchor
    }

    /// Whether anything is selected
    pub fn has_selection(&self) -> bool {
        self.head != self.anchor
    }

    /// The selection's start and end, in text order (equal when nothing is
    /// selected)
    pub fn range(&self) -> (EditPos, EditPos) {
        if self.anchor <= self.head {
            (self.anchor, self.head)
        } else {
            (self.head, self.anchor)
        }
    }
}

/// The text a field edits. See the [module docs](self).
#[derive(Clone)]
pub struct EditState {
    lines: Vec<String>,
    /// Sorted, not overlapping, never empty
    cursors: Vec<EditCursor>,
    /// Index of the cursor that is "the" cursor (the one shown as the
    /// field's cursor, kept when the others are cleared)
    primary: usize,
    single_line: bool,
    max_lines: usize,
    read_only: bool,
    history: UndoHistory<Step>,
    group_delay: Duration,
    /// Whether the next typing or delete may join the last undo step: true
    /// right after one, false after a move or any other edit
    joinable: bool,
    clock: Rc<dyn Fn() -> Instant>,
}

impl std::fmt::Debug for EditState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EditState")
            .field("lines", &self.lines)
            .field("cursors", &self.cursors)
            .field("primary", &self.primary)
            .field("single_line", &self.single_line)
            .field("max_lines", &self.max_lines)
            .field("read_only", &self.read_only)
            .finish_non_exhaustive()
    }
}

impl Default for EditState {
    fn default() -> Self {
        Self::multi_line()
    }
}

impl EditState {
    /// Empty text that may have any number of lines
    pub fn multi_line() -> Self {
        Self {
            lines: vec![String::new()],
            cursors: vec![EditCursor::default()],
            primary: 0,
            single_line: false,
            max_lines: 0,
            read_only: false,
            history: UndoHistory::with_max_size(DEFAULT_HISTORY),
            group_delay: DEFAULT_GROUP_DELAY,
            joinable: false,
            clock: Rc::new(Instant::now),
        }
    }

    /// Empty text kept on one line: a newline typed or pasted becomes a
    /// space, and Enter, Up and Down are left to the widget
    pub fn single_line() -> Self {
        Self {
            single_line: true,
            ..Self::multi_line()
        }
    }

    /// Start with `text`, the cursor at its start
    pub fn with_text(mut self, text: impl AsRef<str>) -> Self {
        self.set_text(text);
        self
    }

    /// The most lines the text may have (0: no limit). Newlines that would
    /// go past it are dropped from typed or pasted text.
    pub fn max_lines(mut self, max: usize) -> Self {
        self.max_lines = max;
        self
    }

    /// Text that moves and selects but does not change
    pub fn read_only(mut self, read_only: bool) -> Self {
        self.read_only = read_only;
        self
    }

    /// Set whether the text can be changed
    pub fn set_read_only(&mut self, read_only: bool) {
        self.read_only = read_only;
    }

    /// Whether the text cannot be changed
    pub fn is_read_only(&self) -> bool {
        self.read_only
    }

    /// The undo steps kept (100 by default); the oldest go first
    pub fn history_limit(mut self, steps: usize) -> Self {
        self.history.set_max_size(steps);
        self
    }

    /// How close typing must follow the last edit to join its undo step
    /// (500 ms by default)
    pub fn group_delay(mut self, delay: Duration) -> Self {
        self.group_delay = delay;
        self
    }

    /// Where edit times come from (the system clock by default), so a test
    /// can control how typing is grouped for undo
    pub fn clock(mut self, clock: impl Fn() -> Instant + 'static) -> Self {
        self.clock = Rc::new(clock);
        self
    }

    // -----------------------------------------------------------------
    // Text
    // -----------------------------------------------------------------

    /// The text, lines joined by `\n`
    pub fn text(&self) -> String {
        self.lines.join("\n")
    }

    /// The lines
    pub fn lines(&self) -> &[String] {
        &self.lines
    }

    /// Line `line`, if there is one
    pub fn line(&self, line: usize) -> Option<&str> {
        self.lines.get(line).map(String::as_str)
    }

    /// How many lines there are (an empty text has one)
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    /// Whether the text is empty
    pub fn is_empty(&self) -> bool {
        self.lines.len() == 1 && self.lines[0].is_empty()
    }

    /// Replace the text: `\r\n` reads as `\n`, a single-line field's
    /// newlines become spaces, and lines past `max_lines` are dropped. The
    /// cursor goes to the start and the undo history is cleared.
    pub fn set_text(&mut self, text: impl AsRef<str>) {
        let text = text.as_ref().replace("\r\n", "\n");
        let text = if self.single_line {
            text.replace('\n', " ")
        } else {
            text
        };
        self.lines = text.split('\n').map(String::from).collect();
        if self.max_lines > 0 {
            self.lines.truncate(self.max_lines);
        }
        self.cursors = vec![EditCursor::default()];
        self.primary = 0;
        self.history.clear();
        self.joinable = false;
    }

    /// The columns (grapheme clusters) in `line`
    pub fn line_len(&self, line: usize) -> usize {
        self.lines.get(line).map_or(0, |l| cluster_count(l))
    }

    /// The position after the last character
    pub fn end(&self) -> EditPos {
        let line = self.lines.len() - 1;
        EditPos::new(line, self.line_len(line))
    }

    /// `pos` moved into the text: the line to the last, the column to the
    /// line's end
    pub fn clamp(&self, pos: EditPos) -> EditPos {
        let line = pos.line.min(self.lines.len() - 1);
        EditPos::new(line, pos.col.min(self.line_len(line)))
    }

    // -----------------------------------------------------------------
    // Cursors
    // -----------------------------------------------------------------

    /// The primary cursor's position
    pub fn cursor(&self) -> EditPos {
        self.cursors[self.primary].head
    }

    /// The primary cursor
    pub fn primary(&self) -> &EditCursor {
        &self.cursors[self.primary]
    }

    /// Every cursor, in text order
    pub fn cursors(&self) -> &[EditCursor] {
        &self.cursors
    }

    /// One cursor at `pos`, selecting nothing
    pub fn set_cursor(&mut self, pos: EditPos) {
        self.set_cursors(vec![EditCursor::at(self.clamp(pos))]);
    }

    /// One cursor at `head`, selecting back to `anchor`
    pub fn select(&mut self, anchor: EditPos, head: EditPos) {
        self.set_cursors(vec![EditCursor::selecting(
            self.clamp(anchor),
            self.clamp(head),
        )]);
    }

    /// Select the whole text
    pub fn select_all(&mut self) {
        self.select(EditPos::default(), self.end());
    }

    /// Add a cursor at `pos`; it becomes the primary one
    pub fn add_cursor(&mut self, cursor: EditCursor) {
        let cursor = EditCursor::selecting(self.clamp(cursor.anchor), self.clamp(cursor.head));
        self.cursors.push(cursor);
        self.primary = self.cursors.len() - 1;
        self.normalize();
        self.joinable = false;
    }

    /// Keep only the primary cursor
    pub fn clear_secondary(&mut self) {
        let primary = self.cursors[self.primary];
        self.set_cursors(vec![primary]);
    }

    /// Replace every cursor (the last one given is the primary); positions
    /// are moved into the text
    pub fn set_cursors(&mut self, cursors: Vec<EditCursor>) {
        let cursors: Vec<EditCursor> = cursors
            .into_iter()
            .map(|c| EditCursor {
                head: self.clamp(c.head),
                anchor: self.clamp(c.anchor),
                goal: c.goal,
            })
            .collect();
        if cursors.is_empty() {
            return;
        }
        self.primary = cursors.len() - 1;
        self.cursors = cursors;
        self.normalize();
        self.joinable = false;
    }

    /// Whether the primary cursor selects anything
    pub fn has_selection(&self) -> bool {
        self.cursors.iter().any(EditCursor::has_selection)
    }

    /// The selected text of every cursor that selects something, in text
    /// order, joined by `\n`
    pub fn selected_text(&self) -> String {
        self.cursors
            .iter()
            .filter(|c| c.has_selection())
            .map(|c| {
                let (from, to) = c.range();
                self.text_in(from, to)
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// The text between two positions
    pub fn text_in(&self, from: EditPos, to: EditPos) -> String {
        let (from, to) = (self.clamp(from.min(to)), self.clamp(from.max(to)));
        let (a, b) = (self.mark(from), self.mark(to));
        if a.line == b.line {
            return self.lines[a.line][a.byte..b.byte].to_string();
        }
        let mut out = self.lines[a.line][a.byte..].to_string();
        for line in &self.lines[a.line + 1..b.line] {
            out.push('\n');
            out.push_str(line);
        }
        out.push('\n');
        out.push_str(&self.lines[b.line][..b.byte]);
        out
    }

    /// Sort the cursors and merge those that meet or overlap, keeping track
    /// of the primary one
    fn normalize(&mut self) {
        let primary = self.cursors[self.primary];
        let mut cursors = std::mem::take(&mut self.cursors);
        cursors.sort_by_key(|c| c.range());
        let mut merged: Vec<EditCursor> = Vec::with_capacity(cursors.len());
        let mut primary_at = 0;
        for cursor in cursors {
            let is_primary = cursor == primary;
            match merged.last_mut() {
                Some(last) if meets(last, &cursor) => {
                    let (a, b) = last.range();
                    let (c, d) = cursor.range();
                    let (from, to) = (a.min(c), b.max(d));
                    // The merged selection keeps the later cursor's direction
                    *last = if cursor.head >= cursor.anchor {
                        EditCursor::selecting(from, to)
                    } else {
                        EditCursor::selecting(to, from)
                    };
                    if is_primary {
                        primary_at = merged.len() - 1;
                    }
                }
                _ => {
                    if is_primary {
                        primary_at = merged.len();
                    }
                    merged.push(cursor);
                }
            }
        }
        self.cursors = merged;
        self.primary = primary_at;
    }

    // -----------------------------------------------------------------
    // Edits
    // -----------------------------------------------------------------

    /// Type `text` at every cursor, replacing what each selects. Typing
    /// joins the last undo step when it follows it closely (see the module
    /// docs). Control characters other than tab and newline are dropped.
    pub fn insert(&mut self, text: &str) -> bool {
        let kind = if text.contains('\n') && !self.single_line {
            Kind::Other
        } else {
            Kind::Type
        };
        self.replace_selections(text, kind)
    }

    /// Paste `text` at every cursor, replacing what each selects, as an
    /// undo step of its own
    pub fn paste(&mut self, text: &str) -> bool {
        self.replace_selections(text, Kind::Other)
    }

    fn replace_selections(&mut self, text: &str, kind: Kind) -> bool {
        let text = clean(text);
        self.apply(kind, |_, c| {
            let (from, to) = c.range();
            Some((from, to, text.clone()))
        })
    }

    /// Delete what each cursor selects; a cursor selecting nothing stays
    pub fn delete_selection(&mut self) -> bool {
        self.apply(Kind::Other, |_, c| {
            c.has_selection().then(|| {
                let (from, to) = c.range();
                (from, to, String::new())
            })
        })
    }

    /// Backspace at every cursor: the selection, or the last code point of
    /// the cluster before the cursor (the whole cluster for an emoji
    /// sequence), or the line break before the line
    pub fn backspace(&mut self) -> bool {
        self.apply(Kind::Delete, |s, c| {
            if c.has_selection() {
                let (from, to) = c.range();
                return Some((from, to, String::new()));
            }
            let pos = c.head;
            if pos.col == 0 {
                return (pos.line > 0).then(|| {
                    let prev = EditPos::new(pos.line - 1, s.line_len(pos.line - 1));
                    (prev, pos, String::new())
                });
            }
            // Remove the bytes Backspace takes from the cluster's end; the
            // rest of the cluster stays (the edit keeps it as text)
            let line = &s.lines[pos.line];
            let cluster = cluster_at(line, pos.col - 1)?;
            let end = byte_of(line, pos.col);
            let start = end - backspace_len(cluster);
            let kept = &line[byte_of(line, pos.col - 1)..start];
            Some((EditPos::new(pos.line, pos.col - 1), pos, kept.to_string()))
        })
    }

    /// Delete at every cursor: the selection, or the cluster after the
    /// cursor, or the line break after the line
    pub fn delete(&mut self) -> bool {
        self.apply(Kind::Delete, |s, c| {
            if c.has_selection() {
                let (from, to) = c.range();
                return Some((from, to, String::new()));
            }
            let pos = c.head;
            let next = if pos.col < s.line_len(pos.line) {
                EditPos::new(pos.line, pos.col + 1)
            } else if pos.line + 1 < s.lines.len() {
                EditPos::new(pos.line + 1, 0)
            } else {
                return None;
            };
            Some((pos, next, String::new()))
        })
    }

    /// Delete the selection, or back to the start of the word before each
    /// cursor (a single space next to the cursor is skipped; a longer run of
    /// whitespace is a word of its own)
    pub fn delete_word_backward(&mut self) -> bool {
        self.apply(Kind::Delete, |s, c| {
            let (from, to) = c.range();
            if c.has_selection() {
                return Some((from, to, String::new()));
            }
            Some((s.word_delete_end(to, false), to, String::new()))
        })
    }

    /// Delete the selection, or on to the end of the word after each cursor
    pub fn delete_word_forward(&mut self) -> bool {
        self.apply(Kind::Delete, |s, c| {
            let (from, to) = c.range();
            if c.has_selection() {
                return Some((from, to, String::new()));
            }
            Some((from, s.word_delete_end(from, true), String::new()))
        })
    }

    /// Where a word delete from `pos` stops, as CodeMirror's `deleteByGroup`:
    /// at a line edge it takes the line break; otherwise a single space next
    /// to the cursor is skipped, then one run of the same category
    fn word_delete_end(&self, pos: EditPos, forward: bool) -> EditPos {
        let line = &self.lines[pos.line];
        let len = cluster_count(line);
        if forward && pos.col == len {
            return if pos.line + 1 < self.lines.len() {
                EditPos::new(pos.line + 1, 0)
            } else {
                pos
            };
        }
        if !forward && pos.col == 0 {
            return if pos.line == 0 {
                pos
            } else {
                EditPos::new(pos.line - 1, self.line_len(pos.line - 1))
            };
        }
        let clusters: Vec<&str> =
            unicode_segmentation::UnicodeSegmentation::graphemes(line.as_str(), true).collect();
        let mut col = pos.col;
        let mut cat: Option<Category> = None;
        loop {
            let next = if forward {
                clusters.get(col)
            } else {
                col.checked_sub(1).and_then(|c| clusters.get(c))
            };
            let Some(&cluster) = next else { break };
            let next_cat = category(cluster);
            if cat.is_some_and(|c| c != next_cat) {
                break;
            }
            if cluster != " " || col != pos.col {
                cat = Some(next_cat);
            }
            col = if forward { col + 1 } else { col - 1 };
        }
        EditPos::new(pos.line, col)
    }

    /// Delete every line a cursor or its selection touches
    pub fn delete_lines(&mut self) -> bool {
        let last = self.lines.len() - 1;
        self.apply(Kind::Other, |s, c| {
            let (from, to) = c.range();
            Some(if to.line < last {
                (
                    EditPos::new(from.line, 0),
                    EditPos::new(to.line + 1, 0),
                    String::new(),
                )
            } else if from.line > 0 {
                let prev = EditPos::new(from.line - 1, s.line_len(from.line - 1));
                (
                    prev,
                    EditPos::new(to.line, s.line_len(to.line)),
                    String::new(),
                )
            } else {
                (
                    EditPos::new(0, 0),
                    EditPos::new(to.line, s.line_len(to.line)),
                    String::new(),
                )
            })
        })
    }

    /// Copy every line a cursor or its selection touches below them (lines
    /// several cursors touch are copied once); the cursors move to the copy
    pub fn duplicate_lines(&mut self) -> bool {
        if self.single_line {
            return false;
        }
        // The blocks of lines to copy, and the block each cursor is in
        let mut blocks: Vec<(usize, usize)> = Vec::new();
        let mut block_of = Vec::with_capacity(self.cursors.len());
        for c in &self.cursors {
            let (from, to) = c.range();
            match blocks.last_mut() {
                Some(last) if from.line <= last.1 => last.1 = last.1.max(to.line),
                _ => blocks.push((from.line, to.line)),
            }
            block_of.push(blocks.len() - 1);
        }
        let cursors = self.cursors.clone();
        let primary = self.primary;
        let mut index = 0;
        let changed = self.apply(Kind::Other, |s, _| {
            let i = index;
            index += 1;
            // Only the first cursor of a block copies it
            if i > 0 && block_of[i] == block_of[i - 1] {
                return None;
            }
            let (first, last) = blocks[block_of[i]];
            let end = EditPos::new(last, s.line_len(last));
            let copy = s.text_in(EditPos::new(first, 0), end);
            Some((end, end, format!("\n{copy}")))
        });
        if changed {
            // Each cursor moves down past its own block and the copies above
            let moved: Vec<EditCursor> = cursors
                .iter()
                .zip(&block_of)
                .map(|(c, &b)| {
                    let shift: usize = blocks[..=b].iter().map(|(f, l)| l - f + 1).sum();
                    EditCursor::selecting(
                        EditPos::new(c.anchor.line + shift, c.anchor.col),
                        EditPos::new(c.head.line + shift, c.head.col),
                    )
                })
                .collect();
            self.cursors = moved;
            self.primary = primary;
            self.normalize();
            // Redo puts them there too
            self.set_last_step_cursors();
        }
        changed
    }

    /// Whether there is a step to undo
    pub fn can_undo(&self) -> bool {
        self.history.can_undo()
    }

    /// Whether there is a step to redo
    pub fn can_redo(&self) -> bool {
        self.history.can_redo()
    }

    /// Forget every undo and redo step
    pub fn clear_history(&mut self) {
        self.history.clear();
        self.joinable = false;
    }

    // -----------------------------------------------------------------
    // Rules for new text
    // -----------------------------------------------------------------

    /// Text as this field takes it: on one line, newlines become spaces
    fn fit_text(&self, text: &str) -> String {
        if self.single_line {
            text.replace('\n', " ")
        } else {
            text.to_string()
        }
    }

    /// Drop the newlines that would take the text past `max_lines`, from
    /// the last replacement back
    fn limit_lines(&self, edits: &mut [(Vec<usize>, engine::Mark, engine::Mark, String)]) {
        if self.max_lines == 0 {
            return;
        }
        let added: isize = edits
            .iter()
            .map(|(_, from, to, text)| {
                text.matches('\n').count() as isize - (to.line - from.line) as isize
            })
            .sum();
        let mut excess = (self.lines.len() as isize + added - self.max_lines as isize).max(0);
        for (_, _, _, text) in edits.iter_mut().rev() {
            while excess > 0 {
                let Some(i) = text.rfind('\n') else { break };
                text.remove(i);
                excess -= 1;
            }
        }
    }
}

/// Whether `b` (after `a` in text order) merges into `a`, as CodeMirror's
/// `EditorSelection.normalized`: a cursor selecting nothing when it is at
/// or inside `a`, a selection when it starts inside `a` (selections that
/// only touch stay apart)
fn meets(a: &EditCursor, b: &EditCursor) -> bool {
    let (_, a_to) = a.range();
    let (b_from, _) = b.range();
    if b.has_selection() {
        b_from < a_to
    } else {
        b_from <= a_to
    }
}

/// `text` without `\r\n` and without control characters other than tab
/// and newline
fn clean(text: &str) -> String {
    text.replace("\r\n", "\n")
        .chars()
        .filter(|&c| !c.is_control() || c == '\n' || c == '\t')
        .collect()
}
