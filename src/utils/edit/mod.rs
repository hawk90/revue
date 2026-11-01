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
//!   and deletes (they join each other) made within
//!   [`group_delay`](EditState::group_delay) (500 ms) of each other, with no
//!   cursor move between, are one step; any other edit (paste, a new line, a
//!   line command) is a step of its own, and typing after it starts a new
//!   one. This follows CodeMirror's history in its time limit and in
//!   joining typing with deletes; CodeMirror also checks that the changes
//!   touch, which an edit at an unmoved cursor always does.
//! - **Line commands** take the lines each cursor or selection touches, a
//!   selection ending at a line's start leaving that line out (as
//!   CodeMirror's `selectedLineBlocks`).
//! - **Single-line** fields ([`EditState::single_line`]) turn a newline or
//!   a tab in typed or pasted text into a space, and leave Enter, Up and
//!   Down to the widget.
//! - **Max lines** ([`max_lines`](EditState::max_lines)): new text that would
//!   go past the limit loses its newlines, at every cursor alike, and line
//!   copies that would go past it are not made.
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

mod clipboard;
mod engine;
mod keys;
mod motion;
mod pos;
mod text;

use std::sync::Arc;
use std::time::{Duration, Instant};

use engine::{Kind, Step};
use text::{backspace_len, byte_of, category, cluster_at, cluster_count, col_of, Category};

pub(crate) use clipboard::{clipboard_text, set_clipboard_text};
pub use keys::EditOutcome;
pub use motion::EditMotion;
pub use pos::EditPos;

use crate::utils::undo::UndoHistory;

/// Steps of undo kept by default
const DEFAULT_HISTORY: usize = 100;

/// Bytes of undo kept by default: a paste of a large file costs its size twice
/// (removed and inserted), so a step count alone does not bound the memory.
const DEFAULT_HISTORY_BYTES: usize = 64 * 1024 * 1024;

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
///
/// `Debug` prints the text: a field that holds a secret keeps it out of its
/// own `Debug`, as `MaskedInput` does.
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
    /// See [`history_bytes`](Self::history_bytes)
    history_bytes: usize,
    group_delay: Duration,
    /// Whether the next typing or delete may join the last undo step: true
    /// right after one, false after a move or any other edit
    joinable: bool,
    clock: Arc<dyn Fn() -> Instant + Send + Sync>,
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
            history_bytes: DEFAULT_HISTORY_BYTES,
            group_delay: DEFAULT_GROUP_DELAY,
            joinable: false,
            clock: Arc::new(crate::utils::clock::now),
        }
    }

    /// Empty text kept on one line: a newline or tab typed or pasted
    /// becomes a space, and Enter, Up and Down are left to the widget
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

    /// The most lines the text may have (0: no limit). Typed or pasted text
    /// whose newlines would go past it loses all of them, at every cursor.
    pub fn max_lines(mut self, max: usize) -> Self {
        self.max_lines = max;
        if max > 0 && self.lines.len() > max {
            // Text set before the limit is cut to it as well
            self.lines.truncate(max);
            let cursors = self.cursors.clone();
            self.set_cursors(cursors);
            self.history.clear();
        }
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

    /// How many bytes of text undo may hold, 64 MiB by default. The oldest steps
    /// go first, and the last edit can always be undone, however large.
    pub fn history_bytes(mut self, bytes: usize) -> Self {
        self.history_bytes = bytes;
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
    pub fn clock(mut self, clock: impl Fn() -> Instant + Send + Sync + 'static) -> Self {
        self.clock = Arc::new(clock);
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

    /// Replace the text: `\r\n` reads as `\n`, control characters other
    /// than tab and newline are dropped, a single-line field's newlines and
    /// tabs become spaces, and lines past `max_lines` are dropped. The
    /// cursor goes to the start and the undo history is cleared.
    pub fn set_text(&mut self, text: impl AsRef<str>) {
        let text = self.fit_text(&clean(text.as_ref()));
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

    /// The chars (code points) on `pos`'s line before it, for an API that
    /// counts positions in chars
    pub fn char_index(&self, pos: EditPos) -> usize {
        let pos = self.clamp(pos);
        let line = &self.lines[pos.line];
        line[..byte_of(line, pos.col)].chars().count()
    }

    /// The position `index` chars into `line`: an index inside a cluster
    /// rounds up to the cluster's end, one past the line is its end
    pub fn pos_at_char(&self, line: usize, index: usize) -> EditPos {
        let line = line.min(self.lines.len() - 1);
        let text = &self.lines[line];
        let byte = text
            .char_indices()
            .nth(index)
            .map_or(text.len(), |(i, _)| i);
        EditPos::new(line, col_of(text, byte))
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

    /// Add `cursor` (moved into the text); it becomes the primary one
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

    /// Whether any cursor selects anything
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

    /// Type `open` and `close` at every cursor, replacing what each
    /// selects, with the cursor left between them: an editor closing the
    /// bracket or quote just typed. One undo step with the typing around it
    /// (redo puts the cursors between the pair too). `close` is kept on one
    /// line: its newlines are dropped.
    pub fn insert_pair(&mut self, open: &str, close: &str) -> bool {
        if self.read_only {
            return false;
        }
        let close: String = close.chars().filter(|&c| c != '\n').collect();
        let pair = clean(&self.fit_text(&format!("{open}{close}")));
        let opened = clean(&self.fit_text(open));
        // The columns `close` adds: none when it joins `open`'s last
        // character (a combining mark), which leaves no place between them
        let back = cluster_count(&pair).saturating_sub(cluster_count(&opened));
        let changed = self.insert(&pair);
        // Typed over the same text, the edit made no change but the
        // cursors still moved past it
        for c in &mut self.cursors {
            // Each cursor ends after its `close`, on that line
            *c = EditCursor::at(EditPos::new(c.head.line, c.head.col.saturating_sub(back)));
        }
        if changed {
            self.set_last_step_cursors();
        }
        changed
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

    /// The blocks of lines the cursors touch, in order, and the block each
    /// cursor is in. A selection that ends at the start of a line does not
    /// take that line (as CodeMirror's `selectedLineBlocks`: Shift+Down from
    /// a line start selects whole lines). Blocks on the same line are one;
    /// with `adjacent`, so are blocks that meet.
    fn line_blocks(&self, adjacent: bool) -> (Vec<(usize, usize)>, Vec<usize>) {
        let mut blocks: Vec<(usize, usize)> = Vec::new();
        let mut block_of = Vec::with_capacity(self.cursors.len());
        for c in &self.cursors {
            let (from, to) = c.range();
            let last = if c.has_selection() && to.col == 0 && to.line > from.line {
                to.line - 1
            } else {
                to.line
            };
            match blocks.last_mut() {
                Some(prev) if from.line <= prev.1 + usize::from(adjacent) => {
                    prev.1 = prev.1.max(last);
                }
                _ => blocks.push((from.line, last)),
            }
            block_of.push(blocks.len() - 1);
        }
        (blocks, block_of)
    }

    /// Delete every line a cursor or its selection touches
    pub fn delete_lines(&mut self) -> bool {
        let last_line = self.lines.len() - 1;
        let (blocks, block_of) = self.line_blocks(true);
        let mut index = 0;
        self.apply(Kind::Other, |s, _| {
            let i = index;
            index += 1;
            // Only the first cursor of a block deletes it
            if i > 0 && block_of[i] == block_of[i - 1] {
                return None;
            }
            let (first, last) = blocks[block_of[i]];
            Some(if last < last_line {
                // The lines and the break after them
                (
                    EditPos::new(first, 0),
                    EditPos::new(last + 1, 0),
                    String::new(),
                )
            } else if first > 0 {
                // The last lines and the break before them
                let prev = EditPos::new(first - 1, s.line_len(first - 1));
                (prev, EditPos::new(last, s.line_len(last)), String::new())
            } else {
                (
                    EditPos::new(0, 0),
                    EditPos::new(last, s.line_len(last)),
                    String::new(),
                )
            })
        })
    }

    /// Copy every line a cursor or its selection touches below them (lines
    /// several cursors touch are copied once); the cursors move to the
    /// copy. Nothing is copied when the copies would take the text past
    /// `max_lines`.
    pub fn duplicate_lines(&mut self) -> bool {
        if self.single_line {
            return false;
        }
        let (blocks, block_of) = self.line_blocks(false);
        let added: usize = blocks.iter().map(|(f, l)| l - f + 1).sum();
        if self.max_lines > 0 && self.lines.len() + added > self.max_lines {
            return false;
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

    /// Text as this field takes it: on one line, newlines and tabs become
    /// spaces
    fn fit_text(&self, text: &str) -> String {
        if self.single_line {
            text.replace(['\n', '\t'], " ")
        } else {
            text.to_string()
        }
    }

    /// When the new text would take the text past `max_lines`, drop the
    /// newlines from every replacement, so each cursor gets the same text
    /// (on one line) rather than some keeping their breaks
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
        if self.lines.len() as isize + added > self.max_lines as isize {
            for (_, _, _, text) in edits.iter_mut() {
                text.retain(|c| c != '\n');
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
