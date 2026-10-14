//! Stack proptests: how the main axis is shared, read from the screen.
//! Each child paints its whole area with its own letter.

use proptest::prelude::*;
use revue::layout::Rect;
use revue::render::{Buffer, Cell};
use revue::widget::traits::{RenderContext, View};
use revue::widget::{hstack, vstack, Stack};

/// Paints every cell of its area with one char
#[derive(Clone)]
struct Paint(char);

impl View for Paint {
    fn render(&self, ctx: &mut RenderContext) {
        for y in 0..ctx.area.height {
            for x in 0..ctx.area.width {
                ctx.set(x, y, Cell::new(self.0));
            }
        }
    }
}

fn letter(i: usize) -> char {
    (b'a' + i as u8) as char
}

/// How many cells along the main axis each child painted, read off the
/// first column (vertical) or row (horizontal) of the area at (`ox`, `oy`).
fn shares(s: &Stack, vertical: bool, n: usize, len: u16, ox: u16, oy: u16) -> (Vec<u16>, Buffer) {
    let (w, h) = if vertical { (4, len) } else { (len, 2) };
    let mut buffer = Buffer::new(w + ox + 2, h + oy + 2);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(ox, oy, w, h));
    s.render(&mut ctx);
    let mut counts = vec![0u16; n];
    for k in 0..len {
        let (x, y) = if vertical { (ox, oy + k) } else { (ox + k, oy) };
        let c = buffer.get(x, y).unwrap().symbol;
        if let Some(i) = (0..n).find(|&i| letter(i) == c) {
            counts[i] += 1;
        }
    }
    (counts, buffer)
}

fn base(vertical: bool) -> Stack {
    // Equal shares: the stack ignores what children measure
    if vertical { vstack() } else { hstack() }.content_sized(false)
}

proptest! {
    /// Children added with `child` share the axis equally: the shares differ
    /// by at most one and add up to the length, when every child gets at
    /// least one cell.
    #[test]
    fn prop_stack_equal_shares(vertical in any::<bool>(), n in 1usize..8, len in 1u16..40) {
        prop_assume!(len as usize >= n);
        let s = (0..n).fold(base(vertical), |s, i| s.child(Paint(letter(i))));
        let (counts, _) = shares(&s, vertical, n, len, 0, 0);
        prop_assert_eq!(counts.iter().sum::<u16>(), len, "{:?}", counts);
        let (lo, hi) = (counts.iter().min().unwrap(), counts.iter().max().unwrap());
        prop_assert!(hi - lo <= 1, "{:?}", counts);
    }

    /// When the fixed sizes and one cell per other child fit, each
    /// `child_sized` child gets exactly its size, and the unsized ones
    /// share the rest.
    #[test]
    fn prop_stack_fixed_sizes_kept_when_they_fit(
        vertical in any::<bool>(),
        sizes in prop::collection::vec(proptest::option::of(1u16..6), 1..7),
        extra in 0u16..12,
    ) {
        let fixed: u16 = sizes.iter().flatten().sum();
        let autos = sizes.iter().filter(|s| s.is_none()).count() as u16;
        // Just enough room, or more
        let len = fixed + autos + extra;
        let s = sizes.iter().enumerate().fold(base(vertical), |s, (i, size)| match size {
            Some(n) => s.child_sized(Paint(letter(i)), *n),
            None => s.child(Paint(letter(i))),
        });
        let (counts, _) = shares(&s, vertical, sizes.len(), len, 0, 0);
        for (i, size) in sizes.iter().enumerate() {
            if let Some(n) = size {
                prop_assert_eq!(counts[i], *n, "child {} of {:?}: {:?}", i, sizes, counts);
            }
        }
        if autos > 0 {
            prop_assert_eq!(counts.iter().sum::<u16>(), len, "{:?}", counts);
        }
    }

    /// Whatever the sizes, nothing is painted outside the stack's area.
    #[test]
    fn prop_stack_paints_inside_its_area(
        vertical in any::<bool>(),
        sizes in prop::collection::vec(proptest::option::of(0u16..30), 0..7),
        gap in 0u16..4,
        len in 0u16..20,
        ox in 0u16..3,
        oy in 0u16..3,
    ) {
        let s = sizes.iter().enumerate().fold(base(vertical).gap(gap), |s, (i, size)| match size {
            Some(n) => s.child_sized(Paint(letter(i)), *n),
            None => s.child(Paint(letter(i))),
        });
        let (_, buffer) = shares(&s, vertical, sizes.len(), len, ox, oy);
        let (w, h) = if vertical { (4, len) } else { (len, 2) };
        for y in 0..buffer.height() {
            for x in 0..buffer.width() {
                let inside = x >= ox && x < ox + w && y >= oy && y < oy + h;
                if !inside {
                    prop_assert_eq!(buffer.get(x, y).unwrap().symbol, ' ', "({}, {})", x, y);
                }
            }
        }
    }
}

#[test]
fn test_stack_zero_areas_draw_nothing() {
    // (Stack is not Clone, so the offset helper does not apply; offsets are
    // in prop_stack_paints_inside_its_area)
    use crate::traits::placement::assert_zero_areas_draw_nothing;
    use revue::widget::Text;
    for vertical in [false, true] {
        for content_sized in [true, false] {
            let make = || {
                if vertical { vstack() } else { hstack() }
                    .content_sized(content_sized)
                    .gap(1)
                    .child(Text::new("one"))
                    .child_sized(Text::new("두 글자"), 3)
                    .child_flex(Text::new("flex"), 1.0)
            };
            assert_zero_areas_draw_nothing(&make());
        }
    }
}
