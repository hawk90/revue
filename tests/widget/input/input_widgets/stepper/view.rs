//! Stepper placement, and a model of the statuses under moves and marks

use proptest::prelude::*;
use revue::widget::{StepStatus, Stepper, StepperStyle};

use crate::traits::placement::{assert_same_when_offset, assert_zero_areas_draw_nothing};

fn four() -> Stepper {
    Stepper::new()
        .add_step("Cart")
        .add_step("배송")
        .add_step("Pay")
        .add_step("Done")
        .current(1)
}

#[test]
fn test_stepper_zero_areas_and_offset() {
    for style in [
        StepperStyle::Dots,
        StepperStyle::Numbered,
        StepperStyle::Connected,
        StepperStyle::Progress,
    ] {
        let s = four().style(style).descriptions(true);
        assert_zero_areas_draw_nothing(&s);
        assert_same_when_offset(s.clone(), 30, 3);
        assert_same_when_offset(s.vertical(), 12, 8);
    }
}

#[derive(Clone, Debug)]
enum Op {
    Next,
    Prev,
    GoTo(usize),
    Complete,
    Error(usize),
    Skip(usize),
}

fn op() -> impl Strategy<Value = Op> {
    prop_oneof![
        Just(Op::Next),
        Just(Op::Prev),
        (0usize..6).prop_map(Op::GoTo),
        Just(Op::Complete),
        (0usize..6).prop_map(Op::Error),
        (0usize..6).prop_map(Op::Skip),
    ]
}

proptest! {
    /// After any moves and marks, the current step and every status are what
    /// a plain model gives: moves set the steps before the current one
    /// Completed, the current one Active and the rest Pending, except steps
    /// marked Error or Skipped, which keep their mark; `complete_current`
    /// completes the current step and moves on.
    #[test]
    fn prop_stepper_statuses_follow_the_model(ops in prop::collection::vec(op(), 0..30)) {
        let n = 4;
        let mut s = four();
        let mut cur = 1usize;
        let mut st = vec![StepStatus::Completed, StepStatus::Active, StepStatus::Pending, StepStatus::Pending];
        let settle = |st: &mut Vec<StepStatus>, cur: usize| {
            for (i, x) in st.iter_mut().enumerate() {
                if *x != StepStatus::Error && *x != StepStatus::Skipped {
                    *x = if i < cur { StepStatus::Completed } else if i == cur { StepStatus::Active } else { StepStatus::Pending };
                }
            }
        };
        for o in &ops {
            match *o {
                Op::Next => { let moved = s.next_step(); prop_assert_eq!(moved, cur + 1 < n); if cur + 1 < n { cur += 1; settle(&mut st, cur); } }
                Op::Prev => { let moved = s.prev(); prop_assert_eq!(moved, cur > 0); if cur > 0 { cur -= 1; settle(&mut st, cur); } }
                Op::GoTo(i) => { s.go_to(i); if i < n { cur = i; settle(&mut st, cur); } }
                Op::Complete => {
                    s.complete_current();
                    st[cur] = StepStatus::Completed;
                    if cur + 1 < n { cur += 1; settle(&mut st, cur); }
                }
                Op::Error(i) => { s.mark_error(i); if i < n { st[i] = StepStatus::Error; } }
                Op::Skip(i) => { s.skip(i); if i < n { st[i] = StepStatus::Skipped; } }
            }
            prop_assert_eq!(s.current_step().unwrap().status, st[cur], "after {:?}", o);
            let done = st.iter().filter(|x| **x == StepStatus::Completed).count();
            prop_assert!((s.progress() - done as f64 / n as f64).abs() < 1e-9, "progress after {:?}", o);
        }
    }
}

#[test]
fn test_stepper_active_indicator_is_bold() {
    use revue::layout::Rect;
    use revue::render::{Buffer, Modifier};
    use revue::widget::traits::{RenderContext, View};
    for vertical in [false, true] {
        let mut s = four();
        if vertical {
            s = s.vertical();
        }
        let mut buffer = Buffer::new(30, 8);
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 30, 8));
        s.render(&mut ctx);
        // Step 1 is active: in a row of four 7-column steps, or the third row
        let (active, done) = if vertical {
            ((0, 2), (0, 0))
        } else {
            ((7, 0), (0, 0))
        };
        let bold = |(x, y): (u16, u16)| buffer.get(x, y).unwrap().modifier.contains(Modifier::BOLD);
        assert!(bold(active), "{vertical}");
        assert!(!bold(done), "{vertical}");
    }
}
