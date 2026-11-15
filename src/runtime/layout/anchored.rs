//! Placing a box next to an anchor: popovers, tooltips and dropdowns
//!
//! One set of rules for every widget that opens a box beside something:
//!
//! 1. The box goes on the first side in the preference list with room for
//!    all of it.
//! 2. When no side has room, it goes on the roomier of the first side and its
//!    opposite, shortened to the room there.
//! 3. Along the side, the box is shifted to stay inside the bounds; it is
//!    never pushed back across the anchor, so it never covers it.
//! 4. `gap` cells stay free between the anchor and the box (an arrow goes
//!    there), so an arrow can never land on the box.
//!
//! The anchor, the bounds and the result share one coordinate space: area
//! coordinates for a box kept inside its area, screen coordinates for an
//! overlay kept on the screen.

use super::Rect;

/// The side of the anchor a box goes on
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Side {
    /// Above the anchor
    Top,
    /// Below the anchor
    Bottom,
    /// Left of the anchor
    Left,
    /// Right of the anchor
    Right,
}

impl Side {
    /// The side across the anchor
    pub(crate) fn opposite(self) -> Self {
        match self {
            Side::Top => Side::Bottom,
            Side::Bottom => Side::Top,
            Side::Left => Side::Right,
            Side::Right => Side::Left,
        }
    }

    fn is_vertical(self) -> bool {
        matches!(self, Side::Top | Side::Bottom)
    }
}

/// How the box lines up with the anchor along the side it is on
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Align {
    /// The box starts where the anchor starts (a dropdown under its header)
    Start,
    /// The box is centered on the anchor (a tooltip over its target)
    Center,
}

/// Where a box landed
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Placed {
    /// The box; shorter than asked across the side when there was no room
    pub rect: Rect,
    /// The side it took
    pub side: Side,
}

/// The cells free for the box on `side` of `anchor`, past `gap`, in `bounds`
pub(crate) fn room(anchor: Rect, side: Side, gap: u16, bounds: Rect) -> u16 {
    match side {
        Side::Bottom => bounds
            .bottom()
            .saturating_sub(anchor.bottom().saturating_add(gap).max(bounds.y)),
        Side::Top => anchor
            .y
            .saturating_sub(gap)
            .min(bounds.bottom())
            .saturating_sub(bounds.y),
        Side::Right => bounds
            .right()
            .saturating_sub(anchor.right().saturating_add(gap).max(bounds.x)),
        Side::Left => anchor
            .x
            .saturating_sub(gap)
            .min(bounds.right())
            .saturating_sub(bounds.x),
    }
}

/// Place a `(width, height)` box beside `anchor` inside `bounds`, by the
/// rules in the module docs. `sides` is the preference order and must not be
/// empty. `None` when there is no room on either side of the first one.
pub(crate) fn place(
    anchor: Rect,
    (width, height): (u16, u16),
    sides: &[Side],
    align: Align,
    gap: u16,
    bounds: Rect,
) -> Option<Placed> {
    let first = *sides.first()?;
    let depth = |side: Side| if side.is_vertical() { height } else { width };
    let side = sides
        .iter()
        .copied()
        .find(|&s| room(anchor, s, gap, bounds) >= depth(s))
        .unwrap_or_else(|| {
            let other = first.opposite();
            if room(anchor, other, gap, bounds) > room(anchor, first, gap, bounds) {
                other
            } else {
                first
            }
        });
    let depth = depth(side).min(room(anchor, side, gap, bounds));
    if depth == 0 {
        return None;
    }

    // Along the side: lined up with the anchor, then shifted into the bounds
    let along = |start: u16, len: u16, lo: u16, hi: u16, size: u16| {
        let at = match align {
            Align::Start => start,
            Align::Center => start.saturating_add(len / 2).saturating_sub(size / 2),
        };
        at.min(hi.saturating_sub(size)).max(lo)
    };

    let rect = match side {
        Side::Bottom | Side::Top => {
            let x = along(anchor.x, anchor.width, bounds.x, bounds.right(), width);
            let y = if side == Side::Bottom {
                anchor.bottom().saturating_add(gap).max(bounds.y)
            } else {
                anchor.y.saturating_sub(gap).min(bounds.bottom()) - depth
            };
            Rect::new(x, y, width, depth)
        }
        Side::Right | Side::Left => {
            let y = along(anchor.y, anchor.height, bounds.y, bounds.bottom(), height);
            let x = if side == Side::Right {
                anchor.right().saturating_add(gap).max(bounds.x)
            } else {
                anchor.x.saturating_sub(gap).min(bounds.right()) - depth
            };
            Rect::new(x, y, depth, height)
        }
    };
    Some(Placed { rect, side })
}

impl Placed {
    /// The cell next to `anchor`, at its middle, on the side the box took:
    /// where an arrow goes. With a gap it lies between the anchor and the
    /// box, outside both (also outside what the box was placed with, such
    /// as a shadow). `None` without a gap.
    pub(crate) fn arrow_cell(&self, anchor: Rect, gap: u16) -> Option<(u16, u16)> {
        if gap == 0 {
            return None;
        }
        let mid_x = anchor.x.saturating_add(anchor.width / 2);
        let mid_y = anchor.y.saturating_add(anchor.height / 2);
        // A side with room past the gap leaves these on the screen
        Some(match self.side {
            Side::Bottom => (mid_x, anchor.bottom()),
            Side::Top => (mid_x, anchor.y.saturating_sub(1)),
            Side::Right => (anchor.right(), mid_y),
            Side::Left => (anchor.x.saturating_sub(1), mid_y),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SCREEN: Rect = Rect {
        x: 0,
        y: 0,
        width: 40,
        height: 20,
    };

    fn cell(x: u16, y: u16) -> Rect {
        Rect::new(x, y, 1, 1)
    }

    fn at(anchor: Rect, size: (u16, u16), side: Side) -> Option<Placed> {
        place(
            anchor,
            size,
            &[side, side.opposite()],
            Align::Center,
            1,
            SCREEN,
        )
    }

    #[test]
    fn each_side_with_room_keeps_its_side() {
        let a = cell(20, 10);
        let p = |s| at(a, (6, 3), s).unwrap();
        assert_eq!(p(Side::Bottom).rect, Rect::new(17, 12, 6, 3));
        assert_eq!(p(Side::Top).rect, Rect::new(17, 6, 6, 3));
        assert_eq!(p(Side::Right).rect, Rect::new(22, 9, 6, 3));
        assert_eq!(p(Side::Left).rect, Rect::new(13, 9, 6, 3));
        for s in [Side::Bottom, Side::Top, Side::Right, Side::Left] {
            assert_eq!(p(s).side, s);
        }
    }

    #[test]
    fn each_side_without_room_flips_to_the_opposite() {
        // Last row, first row, last column, first column
        let b = at(cell(20, 19), (6, 3), Side::Bottom).unwrap();
        assert_eq!((b.side, b.rect), (Side::Top, Rect::new(17, 15, 6, 3)));
        let t = at(cell(20, 0), (6, 3), Side::Top).unwrap();
        assert_eq!((t.side, t.rect), (Side::Bottom, Rect::new(17, 2, 6, 3)));
        let r = at(cell(39, 10), (6, 3), Side::Right).unwrap();
        assert_eq!((r.side, r.rect), (Side::Left, Rect::new(32, 9, 6, 3)));
        let l = at(cell(0, 10), (6, 3), Side::Left).unwrap();
        assert_eq!((l.side, l.rect), (Side::Right, Rect::new(2, 9, 6, 3)));
    }

    #[test]
    fn exactly_enough_room_does_not_flip() {
        // Rows 17..20 below the gap row 16: three rows for a 3-row box
        let p = at(cell(20, 15), (6, 3), Side::Bottom).unwrap();
        assert_eq!((p.side, p.rect.y, p.rect.height), (Side::Bottom, 17, 3));
        // One row less flips
        let p = at(cell(20, 16), (6, 3), Side::Bottom).unwrap();
        assert_eq!(p.side, Side::Top);
    }

    #[test]
    fn no_room_on_either_side_takes_the_roomier_one_shortened() {
        let bounds = Rect::new(0, 0, 20, 6);
        // Row 1: no row above past the gap (row 0), three below it
        let p = place(
            cell(5, 1),
            (6, 5),
            &[Side::Top, Side::Bottom],
            Align::Center,
            1,
            bounds,
        )
        .unwrap();
        assert_eq!((p.side, p.rect), (Side::Bottom, Rect::new(2, 3, 6, 3)));
        // A tie keeps the asked side
        let p = place(
            cell(5, 2),
            (6, 5),
            &[Side::Top, Side::Bottom],
            Align::Center,
            0,
            Rect::new(0, 0, 20, 5),
        )
        .unwrap();
        assert_eq!((p.side, p.rect), (Side::Top, Rect::new(2, 0, 6, 2)));
    }

    #[test]
    fn the_box_never_covers_the_anchor() {
        for y in 0..20 {
            for side in [Side::Bottom, Side::Top] {
                if let Some(p) = at(cell(20, y), (6, 8), side) {
                    assert!(!p.rect.contains(20, y), "anchor row {y}, {side:?}: {p:?}");
                }
            }
        }
        for x in 0..40 {
            for side in [Side::Left, Side::Right] {
                if let Some(p) = at(cell(x, 10), (15, 3), side) {
                    assert!(
                        !p.rect.contains(x, 10),
                        "anchor column {x}, {side:?}: {p:?}"
                    );
                }
            }
        }
    }

    #[test]
    fn shift_keeps_the_box_inside_along_the_side() {
        let p = at(cell(0, 10), (6, 3), Side::Bottom).unwrap();
        assert_eq!(p.rect.x, 0);
        let p = at(cell(39, 10), (6, 3), Side::Bottom).unwrap();
        assert_eq!(p.rect.x, 34);
        let p = at(cell(10, 0), (6, 3), Side::Right).unwrap();
        assert_eq!(p.rect.y, 0);
        let p = at(cell(10, 19), (6, 3), Side::Right).unwrap();
        assert_eq!(p.rect.y, 17);
    }

    #[test]
    fn a_box_wider_than_the_bounds_starts_at_their_edge() {
        let p = at(cell(20, 10), (50, 3), Side::Bottom).unwrap();
        assert_eq!((p.rect.x, p.rect.width), (0, 50));
    }

    #[test]
    fn start_alignment_lines_up_with_the_anchor() {
        let header = Rect::new(5, 2, 12, 1);
        let p = place(header, (12, 4), &[Side::Bottom], Align::Start, 0, SCREEN).unwrap();
        assert_eq!(p.rect, Rect::new(5, 3, 12, 4));
    }

    #[test]
    fn offset_bounds_are_respected() {
        let bounds = Rect::new(10, 4, 20, 8);
        // Bottom row of the bounds: flips up, stays inside them
        let p = place(
            cell(15, 11),
            (6, 3),
            &[Side::Bottom, Side::Top],
            Align::Center,
            1,
            bounds,
        )
        .unwrap();
        assert_eq!((p.side, p.rect), (Side::Top, Rect::new(12, 7, 6, 3)));
        // Shifted right to the bounds' left edge, not to column 0
        let p = place(
            cell(10, 5),
            (6, 3),
            &[Side::Bottom],
            Align::Center,
            1,
            bounds,
        )
        .unwrap();
        assert_eq!(p.rect.x, 10);
    }

    #[test]
    fn empty_bounds_or_box_place_nothing() {
        let none = Rect::new(0, 0, 0, 0);
        assert_eq!(
            place(cell(0, 0), (6, 3), &[Side::Bottom], Align::Center, 1, none),
            None
        );
        assert_eq!(at(cell(20, 10), (6, 0), Side::Bottom), None);
        assert_eq!(
            place(cell(0, 0), (6, 3), &[], Align::Center, 1, SCREEN),
            None
        );
    }

    #[test]
    fn no_room_beside_a_one_row_bounds() {
        let bounds = Rect::new(0, 0, 20, 1);
        assert_eq!(
            place(
                cell(5, 0),
                (6, 3),
                &[Side::Bottom, Side::Top],
                Align::Center,
                0,
                bounds
            ),
            None
        );
    }

    #[test]
    fn an_anchor_at_the_far_corner_does_not_overflow() {
        let max = Rect::new(0, 0, u16::MAX, u16::MAX);
        let p = place(
            cell(u16::MAX - 1, u16::MAX - 1),
            (6, 3),
            &[Side::Bottom, Side::Top],
            Align::Center,
            1,
            max,
        )
        .unwrap();
        assert_eq!(p.side, Side::Top);
        assert!(!p.rect.contains(u16::MAX - 1, u16::MAX - 1));
    }

    #[test]
    fn the_arrow_cell_is_between_the_anchor_and_the_box() {
        let a = cell(20, 10);
        for side in [Side::Bottom, Side::Top, Side::Right, Side::Left] {
            let p = at(a, (6, 3), side).unwrap();
            let (x, y) = p.arrow_cell(a, 1).unwrap();
            assert!(!p.rect.contains(x, y) && !a.contains(x, y), "{side:?}");
            // Next to the anchor
            assert_eq!(x.abs_diff(20) + y.abs_diff(10), 1, "{side:?}");
        }
        // Flipped too
        let a = cell(20, 19);
        let p = at(a, (6, 3), Side::Bottom).unwrap();
        assert_eq!(p.arrow_cell(a, 1), Some((20, 18)));
        assert_eq!(p.arrow_cell(a, 0), None);
        // A wider gap: the arrow stays next to the anchor, not the box
        let a = cell(20, 10);
        let p = place(a, (6, 3), &[Side::Bottom], Align::Center, 2, SCREEN).unwrap();
        assert_eq!((p.rect.y, p.arrow_cell(a, 2)), (13, Some((20, 11))));
    }
}
