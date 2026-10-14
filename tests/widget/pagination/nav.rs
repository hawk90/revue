//! Moving between pages against a plain model.

use proptest::prelude::*;
use revue::widget::Pagination;

#[derive(Debug, Clone, Copy)]
enum Op {
    Next,
    Prev,
    First,
    Last,
    Goto(u16),
    SetTotal(u16),
}

fn op() -> impl Strategy<Value = Op> {
    prop_oneof![
        3 => Just(Op::Next),
        3 => Just(Op::Prev),
        1 => Just(Op::First),
        1 => Just(Op::Last),
        // past both ends, so the clamps are reached
        2 => (0u16..16).prop_map(Op::Goto),
        // down to no pages, and below the current page
        2 => (0u16..13).prop_map(Op::SetTotal),
    ]
}

/// Pages count from 1 to `total`; with no pages the pagination stays on
/// page 1.
#[derive(Debug)]
struct Model {
    total: u16,
    current: u16,
}

impl Model {
    fn last_page(&self) -> u16 {
        self.total.max(1)
    }

    /// Apply `op`; returns what `next_page`/`prev_page` return (whether the
    /// page moved), `None` for the others.
    fn apply(&mut self, op: Op) -> Option<bool> {
        match op {
            Op::Next => {
                let moved = self.current < self.total;
                if moved {
                    self.current += 1;
                }
                Some(moved)
            }
            Op::Prev => {
                let moved = self.current > 1;
                if moved {
                    self.current -= 1;
                }
                Some(moved)
            }
            Op::First => {
                self.current = 1;
                None
            }
            Op::Last => {
                self.current = self.last_page();
                None
            }
            Op::Goto(page) => {
                self.current = page.clamp(1, self.last_page());
                None
            }
            Op::SetTotal(total) => {
                self.total = total;
                self.current = self.current.min(self.last_page());
                None
            }
        }
    }
}

fn apply(p: &mut Pagination, op: Op) -> Option<bool> {
    match op {
        Op::Next => Some(p.next_page()),
        Op::Prev => Some(p.prev_page()),
        Op::First => {
            p.first();
            None
        }
        Op::Last => {
            p.last();
            None
        }
        Op::Goto(page) => {
            p.goto(page);
            None
        }
        Op::SetTotal(total) => {
            p.set_total(total);
            None
        }
    }
}

proptest! {
    #[test]
    fn moves_follow_the_model(
        total in 0u16..13,
        start in 0u16..16,
        ops in prop::collection::vec(op(), 0..40),
    ) {
        let mut p = Pagination::new(total).current(start);
        let mut model = Model { total, current: start.clamp(1, total.max(1)) };
        prop_assert_eq!(p.get_current(), model.current);
        for op in ops {
            prop_assert_eq!(apply(&mut p, op), model.apply(op), "{:?}", op);
            prop_assert_eq!(p.get_current(), model.current, "after {:?}", op);
            prop_assert_eq!(p.get_total(), model.total);
            prop_assert_eq!(p.is_first(), model.current == 1);
            // With no pages `is_last()` is false although nothing follows
            // page 1 (draft-pd-1); checked only with pages
            if model.total > 0 {
                prop_assert_eq!(p.is_last(), model.current == model.total);
            }
        }
    }
}
