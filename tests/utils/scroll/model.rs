//! `ScrollState` against a reference model (#1162)
//!
//! The model keeps the content as a list of row ids, so "the rows on screen
//! stay put" is checked by which row is on top, and it finds the
//! [`ScrollAlign::Nearest`] offset by trying every offset. After every
//! operation the real state must equal the model's, and the rules in the
//! module docs must hold.

use proptest::prelude::*;
use revue::utils::{ScrollAlign, ScrollState};

#[derive(Clone, Debug)]
enum Op {
    Viewport(usize),
    Content(usize),
    /// Insert rows at a fraction (in sixteenths, 0..=16) of the content
    Insert(usize, usize),
    /// Remove rows from a fraction (in sixteenths, 0..16) of the content
    Remove(usize, usize),
    By(isize),
    To(usize),
    SetOffset(usize),
    Top,
    End,
    PageUp,
    PageDown,
    Follow(bool),
    /// Reveal rows: a start fraction (in sixteenths), a length, an align
    Reveal(usize, usize, ScrollAlign),
    Track(usize, usize, ScrollAlign),
}

fn align() -> impl Strategy<Value = ScrollAlign> {
    prop_oneof![
        3 => Just(ScrollAlign::Nearest),
        1 => Just(ScrollAlign::Start),
        1 => Just(ScrollAlign::Center),
        1 => Just(ScrollAlign::End),
    ]
}

fn op() -> impl Strategy<Value = Op> {
    prop_oneof![
        (0usize..12).prop_map(Op::Viewport),
        (0usize..40).prop_map(Op::Content),
        (0usize..=16, 1usize..6).prop_map(|(a, n)| Op::Insert(a, n)),
        (0usize..16, 1usize..6).prop_map(|(a, n)| Op::Remove(a, n)),
        (-15isize..15).prop_map(Op::By),
        (0usize..50).prop_map(Op::To),
        (0usize..50).prop_map(Op::SetOffset),
        Just(Op::Top),
        Just(Op::End),
        Just(Op::PageUp),
        Just(Op::PageDown),
        any::<bool>().prop_map(Op::Follow),
        (0usize..16, 1usize..6, align()).prop_map(|(s, n, a)| Op::Reveal(s, n, a)),
        (0usize..16, 1usize..6, align()).prop_map(|(s, n, a)| Op::Track(s, n, a)),
    ]
}

/// What a view scrolled in rows should do
#[derive(Clone, Debug)]
struct Model {
    rows: Vec<u32>,
    next_id: u32,
    top: usize,
    viewport: usize,
    following: bool,
    /// The view has a tail mode
    tail: bool,
    /// A reveal waiting for the first draw
    pending: Option<(usize, usize, ScrollAlign)>,
    tracked: Option<(usize, usize)>,
}

impl Model {
    fn new(tail: Option<bool>) -> Self {
        Self {
            rows: Vec::new(),
            next_id: 0,
            top: 0,
            viewport: 0,
            following: tail.unwrap_or(false),
            tail: tail.is_some(),
            pending: None,
            tracked: None,
        }
    }

    fn fresh(&mut self, n: usize) -> Vec<u32> {
        let ids = (self.next_id..self.next_id + n as u32).collect();
        self.next_id += n as u32;
        ids
    }

    /// The last page full; before the first draw, the last row on top
    fn max(&self) -> usize {
        self.rows.len().saturating_sub(self.viewport.max(1))
    }

    /// The last row is on screen. Unknown before the first draw.
    fn at_end(&self) -> bool {
        self.viewport > 0 && self.top >= self.max()
    }

    fn visible(&self) -> std::ops::Range<usize> {
        let end = (self.top + self.viewport).min(self.rows.len());
        self.top.min(end)..end
    }

    /// In range, and on the last page while following
    fn settle(&mut self) {
        self.top = if self.following {
            self.max()
        } else {
            self.top.min(self.max())
        };
    }

    /// Move the view; a move follows the end when it ends there
    fn place(&mut self, target: usize) -> bool {
        let target = target.min(self.max());
        let moved = target != self.top;
        if moved {
            self.top = target;
            self.following = self.tail && self.at_end();
        }
        moved
    }

    fn scroll_to(&mut self, row: usize) {
        self.pending = None;
        // Asking for the end while there follows it again
        if !self.place(row) && self.at_end() {
            self.following = self.tail;
        }
    }

    /// The offset that shows `start..end` at `align`
    fn reveal_target(&self, start: usize, end: usize, align: ScrollAlign) -> usize {
        let vp = self.viewport;
        match align {
            ScrollAlign::Nearest if end - start > vp => start,
            // Of the offsets that show the range, the one nearest the view
            ScrollAlign::Nearest => (0..=self.max())
                .filter(|&o| o <= start && end <= o + vp)
                .min_by_key(|&o| o.abs_diff(self.top))
                .unwrap_or(start),
            ScrollAlign::Start => start,
            ScrollAlign::End => end.saturating_sub(vp),
            ScrollAlign::Center => (start + (end - start) / 2).saturating_sub(vp / 2),
        }
    }

    fn reveal(&mut self, start: usize, end: usize, align: ScrollAlign) {
        self.place(self.reveal_target(start, end, align));
        self.settle();
    }

    fn apply(&mut self, op: &Op) {
        match *op {
            Op::Viewport(v) => {
                self.viewport = v;
                if v > 0 {
                    if let Some((s, e, a)) = self.pending.take() {
                        self.place(self.reveal_target(s, e, a));
                    }
                }
                self.settle();
            }
            Op::Content(c) => {
                if c < self.rows.len() {
                    self.rows.truncate(c);
                } else {
                    let ids = self.fresh(c - self.rows.len());
                    self.rows.extend(ids);
                }
                self.settle();
            }
            Op::Insert(at, n) => {
                let at = at * self.rows.len() / 16;
                let top_id = self.rows.get(self.top).copied();
                let ids = self.fresh(n);
                self.rows.splice(at..at, ids);
                // Rows above the view carry its top row down with them
                if at < self.top {
                    self.top = self.rows.iter().position(|&id| Some(id) == top_id).unwrap();
                }
                self.settle();
            }
            Op::Remove(at, n) => {
                let at = at * self.rows.len() / 16;
                let end = (at + n).min(self.rows.len());
                let old: Vec<u32> = self.rows.clone();
                self.rows.drain(at..end);
                // The first row left from the old top down goes on top
                self.top = old[self.top.min(old.len())..]
                    .iter()
                    .find_map(|id| self.rows.iter().position(|r| r == id))
                    .unwrap_or(self.rows.len());
                self.settle();
            }
            Op::By(d) => self.scroll_to(self.top.saturating_add_signed(d)),
            Op::To(r) => self.scroll_to(r),
            Op::SetOffset(r) => {
                self.pending = None;
                self.place(r);
            }
            Op::Top => self.scroll_to(0),
            Op::End => {
                self.pending = None;
                if self.tail {
                    self.following = true;
                    self.settle();
                } else {
                    self.top = self.max();
                }
            }
            Op::PageUp => self.scroll_to(self.top.saturating_sub(self.viewport.max(1))),
            Op::PageDown => self.scroll_to(self.top + self.viewport.max(1)),
            Op::Follow(f) => {
                self.tail |= f;
                self.following = f;
                self.settle();
            }
            Op::Reveal(..) | Op::Track(..) => unreachable!("applied with the range"),
        }
    }
}

/// The rows a reveal stands for in `content` rows, if there are any
fn range(content: usize, start: usize, len: usize) -> Option<(usize, usize)> {
    (content > 0).then(|| {
        let start = start * content / 16;
        (start, (start + len).min(content))
    })
}

fn check(s: &ScrollState, m: &Model, op: &Op) -> Result<(), TestCaseError> {
    prop_assert_eq!(
        (s.offset(), s.viewport(), s.content(), s.is_following()),
        (m.top, m.viewport, m.rows.len(), m.following),
        "after {:?}",
        op
    );
    prop_assert_eq!(s.visible(), m.visible());
    prop_assert_eq!(s.max_offset(), m.max());
    prop_assert!(s.offset() <= s.content().saturating_sub(s.viewport().max(1)));
    prop_assert!(s.visible().end <= s.content());
    prop_assert!(
        m.tail || !s.is_following(),
        "a view without a tail mode follows"
    );
    if s.is_following() && s.viewport() > 0 {
        prop_assert_eq!(
            s.visible().end,
            s.content(),
            "following without the end on screen"
        );
    }
    Ok(())
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(512))]

    /// Any operations: the state equals the model's after each one, the
    /// offset stays in range, a following view shows the end, a revealed
    /// range that fits is on screen, and rows inserted or removed above the
    /// view keep its top row on top.
    #[test]
    fn scroll_state_follows_the_model(
        tail in prop::option::of(any::<bool>()),
        ops in prop::collection::vec(op(), 1..60),
    ) {
        let mut s = match tail {
            Some(follow) => ScrollState::new().follow_end(follow),
            None => ScrollState::new(),
        };
        let mut m = Model::new(tail);
        for op in &ops {
            let (following, old_top, old_len) = (m.following, m.top, m.rows.len());
            let top_id = m.rows.get(m.top).copied();
            match *op {
                Op::Viewport(v) => s.set_viewport(v),
                Op::Content(c) => s.set_content(c),
                Op::Insert(at, n) => s.rows_inserted(at * old_len / 16, n),
                Op::Remove(at, n) => s.rows_removed(at * old_len / 16, n),
                Op::By(d) => s.scroll_by(d),
                Op::To(r) => s.scroll_to(r),
                Op::SetOffset(r) => s.set_offset(r),
                Op::Top => s.scroll_to_top(),
                Op::End => s.scroll_to_end(),
                Op::PageUp => s.page_up(),
                Op::PageDown => s.page_down(),
                Op::Follow(f) => s.set_follow_end(f),
                Op::Reveal(start, len, align) | Op::Track(start, len, align) => {
                    let Some((start, end)) = range(old_len, start, len) else {
                        continue;
                    };
                    let tracked = matches!(op, Op::Track(..));
                    if tracked {
                        s.track_aligned(start..end, align);
                    } else {
                        s.reveal_aligned(start..end, align);
                    }
                    let unchanged =
                        tracked && m.tracked.replace((start, end)) == Some((start, end));
                    if unchanged {
                        // The same rows as last time: the view stays
                    } else if m.viewport == 0 && (tracked || align != ScrollAlign::Start) {
                        m.pending = Some((start, end, align));
                    } else {
                        m.reveal(start, end, align);
                    }
                    if !unchanged && m.viewport > 0 && end - start <= m.viewport {
                        let shown = s.visible();
                        prop_assert!(shown.start <= start && end <= shown.end,
                            "{:?} not in {:?} after {:?}", start..end, shown, op);
                    }
                    check(&s, &m, op)?;
                    continue;
                }
            }
            m.apply(op);
            check(&s, &m, op)?;

            // The rows on screen stay put when rows come or go above them.
            // Rows inserted at the top row go on screen.
            let above = match *op {
                Op::Insert(at, _) => at * old_len / 16 < old_top,
                Op::Remove(..) => true,
                _ => false,
            };
            if let (true, Some(id), false) = (above, top_id, following) {
                if let Some(at) = m.rows.iter().position(|&r| r == id) {
                    if at <= m.max() {
                        prop_assert_eq!(s.offset(), at, "the top row moved after {:?}", op);
                    }
                }
            }
        }
    }
}
