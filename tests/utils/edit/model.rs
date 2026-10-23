//! EditState against a plain model of one cursor (text and a byte offset,
//! clusters counted with unicode-segmentation), and the invariants any mix
//! of edits, moves, cursors and undo keeps

use proptest::prelude::*;
use revue::utils::{EditCursor, EditMotion, EditPos, EditState};
use unicode_segmentation::UnicodeSegmentation;

/// Pieces of text: plain, wide, a combining mark and an emoji modifier on
/// their own (they join the cluster before them), emoji parts, newlines
const PIECES: [&str; 13] = [
    "a",
    "b",
    " ",
    "xy",
    "한",
    "e",
    "\u{301}",
    "👍",
    "\u{1F3FD}",
    "\u{200D}",
    "🇰",
    "\n",
    "a\nb",
];

#[derive(Clone, Debug)]
enum Op {
    Insert(&'static str),
    Backspace,
    Delete,
    Left(bool),
    Right(bool),
    Home(bool),
    End(bool),
}

fn op() -> impl Strategy<Value = Op> {
    prop_oneof![
        4 => prop::sample::select(PIECES.to_vec()).prop_map(Op::Insert),
        2 => Just(Op::Backspace),
        1 => Just(Op::Delete),
        2 => any::<bool>().prop_map(Op::Left),
        2 => any::<bool>().prop_map(Op::Right),
        1 => any::<bool>().prop_map(Op::Home),
        1 => any::<bool>().prop_map(Op::End),
    ]
}

/// One cursor over lines of text, positions as (line, byte)
struct Model {
    lines: Vec<String>,
    head: (usize, usize),
    anchor: (usize, usize),
}

/// Byte offsets where the clusters of `line` start, and its length
fn starts(line: &str) -> Vec<usize> {
    let mut s: Vec<usize> = line.grapheme_indices(true).map(|(i, _)| i).collect();
    s.push(line.len());
    s
}

/// The column of `byte`: clusters starting before it
fn col(line: &str, byte: usize) -> usize {
    line.grapheme_indices(true)
        .take_while(|&(i, _)| i < byte)
        .count()
}

impl Model {
    fn text(&self) -> String {
        self.lines.join("\n")
    }

    fn pos(&self, (line, byte): (usize, usize)) -> EditPos {
        EditPos::new(line, col(&self.lines[line], byte))
    }

    /// A position moved to the end of the cluster it is inside
    fn ceil(&self, (line, byte): (usize, usize)) -> (usize, usize) {
        let s = starts(&self.lines[line]);
        (line, *s.iter().find(|&&b| b >= byte).unwrap())
    }

    /// Flat offset, for replacing across lines
    fn flat(&self, (line, byte): (usize, usize)) -> usize {
        self.lines[..line]
            .iter()
            .map(|l| l.len() + 1)
            .sum::<usize>()
            + byte
    }

    fn unflat(&self, mut at: usize) -> (usize, usize) {
        for (i, l) in self.lines.iter().enumerate() {
            if at <= l.len() {
                return (i, at);
            }
            at -= l.len() + 1;
        }
        unreachable!()
    }

    fn replace(&mut self, from: (usize, usize), to: (usize, usize), text: &str) {
        let (a, b) = (self.flat(from), self.flat(to));
        let mut all = self.text();
        all.replace_range(a..b, text);
        self.lines = all.split('\n').map(String::from).collect();
        self.head = self.ceil(self.unflat(a + text.len()));
        self.anchor = self.head;
    }

    fn range(&self) -> ((usize, usize), (usize, usize)) {
        (self.head.min(self.anchor), self.head.max(self.anchor))
    }

    fn apply(&mut self, op: &Op) {
        let (from, to) = self.range();
        let selecting = from != to;
        match *op {
            Op::Insert(t) => self.replace(from, to, t),
            Op::Backspace | Op::Delete if selecting => self.replace(from, to, ""),
            Op::Backspace => {
                let (line, byte) = self.head;
                if byte == 0 {
                    if line > 0 {
                        let prev = (line - 1, self.lines[line - 1].len());
                        self.replace(prev, self.head, "");
                    }
                    return;
                }
                let l = &self.lines[line];
                let start = *starts(l).iter().rev().find(|&&b| b < byte).unwrap();
                let cluster = &l[start..byte];
                let joiners = ['\u{200D}', '\u{FE0F}', '\u{20E3}'];
                let emoji = cluster.chars().any(|c| {
                    joiners.contains(&c)
                        || ('\u{1F1E6}'..='\u{1F1FF}').contains(&c)
                        || ('\u{1F3FB}'..='\u{1F3FF}').contains(&c)
                });
                let cut = if cluster.chars().count() == 1 || emoji {
                    start
                } else {
                    byte - cluster.chars().last().unwrap().len_utf8()
                };
                self.replace((line, cut), self.head, "");
            }
            Op::Delete => {
                let (line, byte) = self.head;
                let l = &self.lines[line];
                if byte < l.len() {
                    let end = *starts(l).iter().find(|&&b| b > byte).unwrap();
                    self.replace(self.head, (line, end), "");
                } else if line + 1 < self.lines.len() {
                    self.replace(self.head, (line + 1, 0), "");
                }
            }
            Op::Left(extend) | Op::Right(extend) => {
                let left = matches!(op, Op::Left(_));
                if !extend && selecting {
                    self.head = if left { from } else { to };
                } else {
                    let (line, byte) = self.head;
                    let s = starts(&self.lines[line]);
                    self.head = if left {
                        match s.iter().rev().find(|&&b| b < byte) {
                            Some(&b) => (line, b),
                            None if line > 0 => (line - 1, self.lines[line - 1].len()),
                            None => self.head,
                        }
                    } else {
                        match s.iter().find(|&&b| b > byte) {
                            Some(&b) => (line, b),
                            None if line + 1 < self.lines.len() => (line + 1, 0),
                            None => self.head,
                        }
                    };
                }
                if !extend {
                    self.anchor = self.head;
                }
            }
            Op::Home(extend) | Op::End(extend) => {
                let line = self.head.0;
                self.head = if matches!(op, Op::Home(_)) {
                    (line, 0)
                } else {
                    (line, self.lines[line].len())
                };
                if !extend {
                    self.anchor = self.head;
                }
            }
        }
    }
}

fn run(state: &mut EditState, op: &Op) {
    match *op {
        Op::Insert(t) => {
            state.insert(t);
        }
        Op::Backspace => {
            state.backspace();
        }
        Op::Delete => {
            state.delete();
        }
        Op::Left(e) => {
            state.move_cursors(EditMotion::Left, e);
        }
        Op::Right(e) => {
            state.move_cursors(EditMotion::Right, e);
        }
        Op::Home(e) => {
            state.move_cursors(EditMotion::LineStart, e);
        }
        Op::End(e) => {
            state.move_cursors(EditMotion::LineEnd, e);
        }
    }
}

/// Every cursor inside the text, in order, not overlapping; the primary one
/// of them
fn check_cursors(s: &EditState) -> Result<(), TestCaseError> {
    let cursors = s.cursors();
    prop_assert!(!cursors.is_empty());
    for c in cursors {
        for pos in [c.head(), c.anchor()] {
            prop_assert!(pos.line < s.line_count());
            prop_assert!(
                pos.col <= s.line_len(pos.line),
                "{:?} past {:?}",
                pos,
                s.line(pos.line)
            );
        }
    }
    for pair in cursors.windows(2) {
        let (_, a_to) = pair[0].range();
        let (b_from, _) = pair[1].range();
        prop_assert!(a_to <= b_from, "overlap {:?}", pair);
        prop_assert!(pair[0].range() != pair[1].range());
    }
    prop_assert!(cursors.iter().any(|c| c.head() == s.cursor()));
    Ok(())
}

proptest! {
    /// One cursor: the text, the cursor and the selection follow a plain
    /// model of each edit and move, clusters and all
    #[test]
    fn one_cursor_follows_the_model(ops in prop::collection::vec(op(), 0..40)) {
        let mut state = EditState::multi_line();
        let mut model = Model { lines: vec![String::new()], head: (0, 0), anchor: (0, 0) };
        for op in &ops {
            run(&mut state, op);
            model.apply(op);
            prop_assert_eq!(state.text(), model.text(), "after {:?}", op);
            prop_assert_eq!(state.cursor(), model.pos(model.head), "after {:?}", op);
            prop_assert_eq!(state.primary().anchor(), model.pos(model.anchor), "after {:?}", op);
        }
    }

    /// Any edits at any cursors keep the cursors in order and inside the
    /// text; undoing every step gives back the first text, and redoing them
    /// all the last
    #[test]
    fn many_cursors_keep_their_order_and_undo_goes_back_to_the_start(
        start in prop::collection::vec(prop::sample::select(PIECES.to_vec()), 0..8),
        steps in prop::collection::vec(
            (op(), prop::option::of((0usize..4, 0usize..6, any::<bool>())), 0u8..14),
            0..30,
        ),
    ) {
        let first: String = start.concat();
        let mut s = EditState::multi_line().with_text(&first).history_limit(1000);
        for (op, add, special) in &steps {
            if let Some((line, col, select)) = *add {
                let pos = EditPos::new(line, col);
                s.add_cursor(if select {
                    EditCursor::selecting(EditPos::new(line, 0), pos)
                } else {
                    EditCursor::at(pos)
                });
            }
            let extend = special % 2 == 0;
            match special {
                0 => {
                    s.duplicate_lines();
                }
                1 => {
                    s.delete_word_backward();
                }
                2 => {
                    s.delete_word_forward();
                }
                3 => {
                    s.delete_lines();
                }
                4 | 5 => {
                    s.move_cursors(EditMotion::Up, extend);
                }
                6 | 7 => {
                    s.move_cursors(EditMotion::Down, extend);
                }
                8 | 9 => {
                    s.move_cursors(EditMotion::WordRight, extend);
                }
                10 => {
                    s.move_cursors(EditMotion::WordLeft, extend);
                }
                11 => {
                    s.paste("p\nq");
                }
                _ => run(&mut s, op),
            }
            check_cursors(&s)?;
        }
        let last = s.text();
        while s.undo() {
            check_cursors(&s)?;
        }
        prop_assert_eq!(s.text(), first.replace("\r\n", "\n"));
        while s.redo() {
            check_cursors(&s)?;
        }
        prop_assert_eq!(s.text(), last);
    }
}
