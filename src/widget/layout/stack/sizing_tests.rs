//! Tests of the sizing rules (moved from sizing.rs)

use super::{hstack, vstack, ChildSize, Stack};
use crate::layout::Rect;
use crate::render::Buffer;
use crate::widget::traits::{RenderContext, View};
use crate::widget::Text;

#[test]
fn test_stack_render_empty_no_panic() {
    let mut buf = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buf, area);
    let s = Stack::new();
    s.render(&mut ctx);
    assert!((0..24).all(|y| (0..80).all(|x| buf.get(x, y).unwrap().symbol == ' ')));
}

#[test]
fn test_stack_render_zero_area_no_panic() {
    let mut buf = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 0, 0);
    let mut ctx = RenderContext::new(&mut buf, area);
    let s = Stack::new().child(Text::new("A"));
    s.render(&mut ctx);
    assert!((0..24).all(|y| (0..80).all(|x| buf.get(x, y).unwrap().symbol == ' ')));
}

#[test]
fn test_stack_calculate_sizes_auto() {
    let s = Stack::new()
        .child(Text::new("A"))
        .child(Text::new("B"))
        .child(Text::new("C"));
    let sizes = Stack::calculate_sizes(&s.child_sizes, 30, 3);
    assert_eq!(sizes.len(), 3);
    assert_eq!(sizes.iter().sum::<u16>(), 30);
}

#[test]
fn test_stack_calculate_sizes_fixed() {
    let s = Stack::new()
        .child_sized(Text::new("A"), 10)
        .child_sized(Text::new("B"), 20);
    let sizes = Stack::calculate_sizes(&s.child_sizes, 50, 2);
    assert_eq!(sizes, vec![10, 20]);
}

#[test]
fn test_stack_calculate_sizes_flex() {
    let s = Stack::new()
        .child_flex(Text::new("A"), 1.0)
        .child_flex(Text::new("B"), 2.0);
    let sizes = Stack::calculate_sizes(&s.child_sizes, 30, 2);
    assert_eq!(sizes[0], 10);
    assert_eq!(sizes[1], 20);
}

#[test]
fn test_stack_calculate_sizes_mixed() {
    let s = Stack::new()
        .child_sized(Text::new("Fixed"), 10)
        .child_flex(Text::new("Flex"), 1.0);
    let sizes = Stack::calculate_sizes(&s.child_sizes, 30, 2);
    assert_eq!(sizes[0], 10);
    assert_eq!(sizes[1], 20); // 30 - 10 = 20 (no auto children)
}

#[test]
fn test_stack_calculate_sizes_content_fits() {
    let sizes = [
        ChildSize::Content(3),
        ChildSize::Auto,
        ChildSize::Content(2),
    ];
    assert_eq!(Stack::calculate_sizes(&sizes, 10, 3), vec![3, 5, 2]);
}

/// Measured sizes that overflow shrink in proportion; fixed ones do not.
#[test]
fn test_stack_calculate_sizes_content_shrinks() {
    let sizes = [
        ChildSize::Fixed(2),
        ChildSize::Content(12),
        ChildSize::Content(4),
        ChildSize::Fixed(2),
    ];
    // 8 cells for 16 measured: 12 -> 6, 4 -> 2.
    assert_eq!(Stack::calculate_sizes(&sizes, 12, 4), vec![2, 6, 2, 2]);
    // 7 cells: 12*7/16 = 5, 4*7/16 = 1, the leftover cell to the first.
    assert_eq!(Stack::calculate_sizes(&sizes, 11, 4), vec![2, 6, 1, 2]);
}

/// A share does not go below a child's minimum; the others share the rest.
#[test]
fn test_stack_calculate_sizes_auto_minimum() {
    use ChildSize::{Auto, AutoMin, Fixed};
    let sizes = |c: &[ChildSize], n| Stack::calculate_sizes(c, n, c.len());
    // Below its share a minimum changes nothing.
    assert_eq!(sizes(&[AutoMin(3), Auto], 10), vec![5, 5]);
    assert_eq!(sizes(&[AutoMin(5), Auto], 10), vec![5, 5]);
    // Above its share it is kept, and rounding goes to the earliest of the rest.
    assert_eq!(sizes(&[AutoMin(6), Auto], 10), vec![6, 4]);
    assert_eq!(sizes(&[Auto, AutoMin(6), Auto], 11), vec![3, 6, 2]);
    // Pinning one can push the next one's minimum above its new share.
    assert_eq!(sizes(&[AutoMin(6), AutoMin(3), Auto], 10), vec![6, 3, 1]);
    // Minimums that do not fit do not overflow: the earlier ones are served
    // first. A child with nothing left still gets the one cell any share does.
    assert_eq!(sizes(&[AutoMin(6), AutoMin(6)], 10), vec![6, 4]);
    assert_eq!(sizes(&[AutoMin(10), Auto], 10), vec![10, 1]);
    assert_eq!(sizes(&[AutoMin(4), Auto], 0), vec![1, 1]);
    // A fixed child is never pushed off: the minimum gets what it leaves.
    assert_eq!(sizes(&[Fixed(4), AutoMin(5), Auto], 10), vec![4, 5, 1]);
    assert_eq!(sizes(&[Fixed(3), AutoMin(3), Fixed(1)], 5), vec![3, 1, 1]);
    assert_eq!(sizes(&[AutoMin(3), Fixed(5)], 5), vec![1, 5]);
    // With flex, an auto child takes its minimum and flex the rest.
    assert_eq!(sizes(&[ChildSize::Flex(1.0), AutoMin(3)], 10), vec![7, 3]);
    assert_eq!(sizes(&[ChildSize::Flex(1.0), AutoMin(30)], 10), vec![0, 10]);
}

/// Measured children shrink for a minimum, down to a cell each.
#[test]
fn test_stack_calculate_sizes_minimum_shrinks_content() {
    use ChildSize::{AutoMin, Content};
    let sizes = |c: &[ChildSize], n| Stack::calculate_sizes(c, n, c.len());
    // Room for all: nothing changes.
    assert_eq!(
        sizes(&[Content(4), AutoMin(3), Content(1)], 8),
        vec![4, 3, 1]
    );
    assert_eq!(
        sizes(&[Content(4), AutoMin(3), Content(1)], 12),
        vec![4, 7, 1]
    );
    // Short of two: the body gives them up, and the footer keeps its row.
    assert_eq!(
        sizes(&[Content(4), AutoMin(3), Content(1)], 6),
        vec![2, 3, 1]
    );
    // Every measured child keeps a cell before the minimum gets the rest.
    assert_eq!(
        sizes(&[Content(4), AutoMin(3), Content(1)], 3),
        vec![1, 1, 1]
    );
    assert_eq!(sizes(&[Content(2), AutoMin(3)], 2), vec![1, 1]);
    // No minimum: as before.
    assert_eq!(sizes(&[Content(4), Content(1)], 3), vec![2, 1]);
    // A child that measured nothing takes no cell from one that did.
    assert_eq!(
        sizes(&[Content(0), Content(1), Content(1)], 1),
        vec![0, 1, 0]
    );
    assert_eq!(
        sizes(&[Content(0), Content(1), Content(4)], 3),
        vec![0, 1, 2]
    );
    assert_eq!(
        sizes(&[Content(0), AutoMin(3), Content(1)], 2),
        vec![0, 1, 1]
    );
    // Nor does it count when a minimum reserves its room.
    assert_eq!(
        sizes(&[Content(0), AutoMin(3), Content(5)], 3),
        vec![0, 2, 1]
    );
    // One child much larger: everyone still keeps a cell.
    assert_eq!(
        sizes(
            &[Content(100), Content(1), Content(1), Content(1), Content(1)],
            5
        ),
        vec![1, 1, 1, 1, 1]
    );
    // Fewer cells than children: the earliest.
    assert_eq!(
        sizes(&[Content(5), Content(1), Content(1)], 2),
        vec![1, 1, 0]
    );
    // With flex, a minimum is served before a plain share too.
    assert_eq!(
        sizes(&[ChildSize::Auto, AutoMin(3), ChildSize::Flex(1.0)], 4),
        vec![1, 3, 0]
    );
}

/// The distribution's rules, over every mix of up to four children and a range
/// of available space: cases picked by hand missed two regressions here.
#[test]
fn test_stack_calculate_sizes_rules_hold_for_every_combination() {
    use ChildSize::{Auto, AutoAtLeast, AutoMin, Content, Fixed, Flex};
    let kinds = [
        Auto,
        AutoMin(1),
        AutoMin(3),
        AutoMin(9),
        AutoMin(u16::MAX),
        AutoAtLeast(3),
        AutoAtLeast(9),
        Fixed(0),
        Fixed(2),
        Fixed(7),
        Fixed(u16::MAX),
        Content(0),
        Content(1),
        Content(4),
        Content(12),
        Content(u16::MAX),
        Flex(1.0),
        Flex(2.5),
    ];
    let mut combos: Vec<Vec<ChildSize>> = vec![Vec::new()];
    let mut last = combos.clone();
    for _ in 0..4 {
        let mut next = Vec::new();
        for combo in &last {
            for kind in kinds {
                let mut longer = combo.clone();
                longer.push(kind);
                next.push(longer);
            }
        }
        combos.extend(next.iter().cloned());
        last = next;
    }
    for combo in &combos {
        for available in [0u16, 1, 2, 3, 5, 6, 10, 13, 40, 200, u16::MAX] {
            let sizes = Stack::calculate_sizes(combo, available, combo.len());
            let context = || format!("{combo:?} in {available} -> {sizes:?}");
            let avail = u32::from(available);
            let total = |pick: fn(&ChildSize) -> u32| combo.iter().map(pick).sum::<u32>();
            let fixed = total(|c| match c {
                Fixed(s) | AutoAtLeast(s) => u32::from(*s),
                _ => 0,
            });
            let floors = total(|c| if let AutoMin(s) = c { u32::from(*s) } else { 0 });
            let content = total(|c| if let Content(s) = c { u32::from(*s) } else { 0 });
            let shares = total(|c| u32::from(matches!(c, Auto | AutoMin(_) | AutoAtLeast(_))));
            let measured = total(|c| u32::from(matches!(c, Content(s) if *s > 0)));
            let flex = combo.iter().any(|c| matches!(c, Flex(_)));
            let sum: u32 = sizes.iter().map(|s| u32::from(*s)).sum();

            for (child, &size) in combo.iter().zip(&sizes) {
                match *child {
                    Fixed(s) => assert_eq!(size, s, "a fixed child keeps its size: {}", context()),
                    AutoAtLeast(least) => {
                        assert!(size >= least, "it does not give way: {}", context());
                    }
                    Content(s) => {
                        assert!(size <= s, "no more than it measured: {}", context());
                        if s > 0 && avail >= fixed + measured {
                            assert!(size > 0, "room for a cell each: {}", context());
                        }
                        if fixed + content + floors <= avail {
                            assert_eq!(size, s, "everything fits: {}", context());
                        }
                    }
                    AutoMin(least) if fixed + content + floors + shares <= avail => {
                        assert!(size >= least, "everything fits: {}", context());
                    }
                    _ => {}
                }
            }
            // Only fixed sizes and a share's one cell may overflow. Flex rounding
            // can too: #1344.
            if !flex {
                assert!(sum <= avail.max(fixed) + shares, "overflow: {}", context());
                if shares > 0 {
                    assert!(sum >= avail, "space left unused: {}", context());
                }
            }
        }
    }
}

#[test]
fn test_stack_calculate_sizes_empty() {
    let s = Stack::new();
    let sizes = Stack::calculate_sizes(&s.child_sizes, 100, 0);
    assert!(sizes.is_empty());
}

#[test]
fn test_stack_render_row_children() {
    let mut buf = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buf, area);
    let s = hstack().child(Text::new("AB")).child(Text::new("CD"));
    s.render(&mut ctx);
    assert_eq!(buf.get(0, 0).unwrap().symbol, 'A');
    assert_eq!(buf.get(1, 0).unwrap().symbol, 'B');
}

#[test]
fn test_stack_row_no_wrap_overflow() {
    // Without wrap, children extend beyond area (clipped by area bounds)
    let mut buf = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);
    let mut ctx = RenderContext::new(&mut buf, area);
    let s = hstack()
        .child_sized(Text::new("AAAA"), 6)
        .child_sized(Text::new("BBBB"), 6);
    s.render(&mut ctx);
    // First child renders, second starts at x=6 but is clipped at width 10
    assert_eq!(buf.get(0, 0).unwrap().symbol, 'A');
}

#[test]
fn test_stack_needs_render_skip() {
    // Verify Stack respects needs_render
    let mut buf = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let s = vstack()
        .child(Text::new("Visible"))
        .child(Text::new("Also visible"));
    let mut ctx = RenderContext::new(&mut buf, area);
    s.render(&mut ctx);
    assert_eq!(buf.get(0, 0).unwrap().symbol, 'V');
}

#[test]
fn test_stack_a_minimum_that_may_not_give_way_is_kept_and_still_shares() {
    use ChildSize::{Auto, AutoAtLeast, AutoMin, Content, Fixed, Flex};
    let sizes = |c: &[ChildSize], a| Stack::calculate_sizes(c, a, c.len());
    // Room: a share like any other, above the minimum.
    assert_eq!(sizes(&[AutoAtLeast(3), Auto], 10), vec![5, 5]);
    assert_eq!(sizes(&[AutoAtLeast(6), Auto], 10), vec![6, 4]);
    // Too little: kept, and the rest goes past the end to be cut.
    assert_eq!(sizes(&[AutoAtLeast(5), Content(1)], 4), vec![5, 0]);
    assert_eq!(sizes(&[AutoAtLeast(5), Fixed(2)], 4), vec![5, 2]);
    assert_eq!(sizes(&[AutoAtLeast(5), Auto], 4), vec![5, 1]);
    assert_eq!(sizes(&[AutoAtLeast(5), AutoAtLeast(5)], 4), vec![5, 5]);
    // Measured children shrink for it, unlike `AutoMin`, down to nothing.
    assert_eq!(sizes(&[AutoAtLeast(4), Content(4)], 5), vec![4, 1]);
    assert_eq!(sizes(&[AutoMin(4), Content(4)], 4), vec![3, 1]);
    assert_eq!(sizes(&[AutoAtLeast(4), Content(4)], 4), vec![4, 0]);
    // A minimum that may give way does, whichever comes first.
    assert_eq!(sizes(&[AutoMin(3), AutoAtLeast(3)], 3), vec![1, 3]);
    assert_eq!(sizes(&[AutoAtLeast(3), AutoMin(3)], 3), vec![3, 1]);
    assert_eq!(sizes(&[AutoMin(9), AutoAtLeast(3)], 6), vec![3, 3]);
    assert_eq!(
        sizes(&[AutoMin(3), AutoAtLeast(3), Flex(1.0)], 3),
        vec![1, 3, 0]
    );
    // Beside flex.
    assert_eq!(sizes(&[AutoAtLeast(3), Flex(1.0)], 10), vec![3, 7]);
    assert_eq!(sizes(&[AutoAtLeast(30), Flex(1.0)], 10), vec![30, 0]);
}
