//! Model-based checks: any sequence of operations keeps the documented rules.

use proptest::prelude::*;
use revue::utils::ScrollState;

#[derive(Clone, Debug)]
enum Op {
    Viewport(usize),
    Content(usize),
    Insert(usize, usize),
    Remove(usize, usize),
    By(isize),
    To(usize),
    End,
    Reveal(usize, usize),
}

fn op() -> impl Strategy<Value = Op> {
    prop_oneof![
        (1usize..12).prop_map(Op::Viewport),
        (0usize..40).prop_map(Op::Content),
        (0usize..40, 0usize..6).prop_map(|(a, n)| Op::Insert(a, n)),
        (0usize..40, 0usize..6).prop_map(|(a, n)| Op::Remove(a, n)),
        (-15isize..15).prop_map(Op::By),
        (0usize..50).prop_map(Op::To),
        Just(Op::End),
        (0usize..40, 1usize..6).prop_map(|(a, n)| Op::Reveal(a, n)),
    ]
}

proptest! {
    #[test]
    fn prop_scroll_state_keeps_its_rules(ops in prop::collection::vec(op(), 1..40)) {
        let mut s = ScrollState::new();
        s.set_viewport(5);
        for op in ops {
            let (before_offset, before_following) = (s.offset(), s.is_following());
            match op {
                Op::Viewport(v) => s.set_viewport(v),
                Op::Content(c) => s.set_content(c),
                Op::Insert(at, n) => {
                    let at = at.min(s.content());
                    s.rows_inserted(at, n);
                    // Rows above the view carry it down; the end is followed instead
                    if !before_following && at < before_offset {
                        prop_assert_eq!(s.offset(), (before_offset + n).min(s.max_offset()));
                    }
                }
                Op::Remove(at, n) => {
                    let at = at.min(s.content());
                    s.rows_removed(at, n);
                }
                Op::By(d) => s.scroll_by(d),
                Op::To(r) => s.scroll_to(r),
                Op::End => {
                    s.scroll_to_end();
                    prop_assert!(s.is_following());
                }
                Op::Reveal(start, n) => {
                    let rows = start.min(s.content())..(start + n).min(s.content());
                    let was_visible = s.visible();
                    s.reveal(rows.clone());
                    if !rows.is_empty() && !s.is_following() {
                        if rows.len() <= s.viewport() {
                            prop_assert!(s.visible().start <= rows.start && rows.end <= s.visible().end,
                                "{rows:?} not in {:?}", s.visible());
                        }
                        if was_visible.start <= rows.start && rows.end <= was_visible.end {
                            prop_assert_eq!(s.offset(), was_visible.start, "a visible range scrolled");
                        }
                    }
                }
            }
            prop_assert!(s.offset() <= s.max_offset(), "offset {} past {}", s.offset(), s.max_offset());
            if s.is_following() {
                prop_assert_eq!(s.offset(), s.max_offset());
            }
            prop_assert!(s.visible().end <= s.content());
        }
    }
}
