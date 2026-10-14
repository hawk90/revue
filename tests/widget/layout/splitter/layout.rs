//! Splitter pane geometry: `pane_areas` for any panes, sizes, ratios,
//! collapsed panes, orientation and area

use proptest::prelude::*;
use revue::layout::Rect;
use revue::widget::{Pane, SplitOrientation, Splitter};

#[derive(Clone, Debug)]
struct PaneSpec {
    ratio: f32,
    min: u16,
    max: u16,
    collapsed: bool,
}

fn pane_spec() -> impl Strategy<Value = PaneSpec> {
    (
        prop_oneof![Just(0.0f32), Just(1.0f32), 0.0f32..=1.0],
        prop_oneof![Just(0u16), 0u16..12],
        prop_oneof![Just(0u16), 1u16..20],
        prop::bool::weighted(0.25),
    )
        .prop_map(|(ratio, min, max, collapsed)| PaneSpec {
            ratio,
            min,
            max,
            collapsed,
        })
}

fn build(specs: &[PaneSpec], vertical: bool) -> Splitter {
    let mut s = Splitter::new();
    if vertical {
        s = s.vertical();
    }
    for (i, p) in specs.iter().enumerate() {
        s = s.pane(
            Pane::new(format!("p{i}"))
                .ratio(p.ratio)
                .min_size(p.min)
                .max_size(p.max)
                .collapsible(),
        );
    }
    for (i, p) in specs.iter().enumerate() {
        if p.collapsed {
            s.toggle_pane(i);
        }
    }
    s
}

/// An area near the origin or against the end of the coordinate space
fn area() -> impl Strategy<Value = Rect> {
    (any::<bool>(), 0u16..60, 0u16..30, 0u16..8, 0u16..8).prop_map(|(edge, w, h, dx, dy)| {
        if edge {
            Rect::new(u16::MAX - w, u16::MAX - h, w, h)
        } else {
            Rect::new(dx, dy, w, h)
        }
    })
}

/// (start, size) along the split, and (start, size) across it
fn axes(r: Rect, vertical: bool) -> ((u16, u16), (u16, u16)) {
    if vertical {
        ((r.y, r.height), (r.x, r.width))
    } else {
        ((r.x, r.width), (r.y, r.height))
    }
}

proptest! {
    /// The visible panes, in order, tile the area along the split with a
    /// one-cell divider between neighbors: each starts one cell after the
    /// previous one ends (or at the area's end once the area is used up),
    /// the last one ends at the area's end, and every pane spans the area
    /// across the split.
    #[test]
    fn prop_splitter_panes_tile_the_area(
        specs in prop::collection::vec(pane_spec(), 0..8),
        vertical in any::<bool>(),
        area in area(),
    ) {
        let s = build(&specs, vertical);
        let areas = s.pane_areas(area);
        let visible: Vec<String> = specs
            .iter()
            .enumerate()
            .filter(|(_, p)| !p.collapsed)
            .map(|(i, _)| format!("p{i}"))
            .collect();
        let ids: Vec<String> = areas.iter().map(|(id, _)| id.clone()).collect();
        prop_assert_eq!(&ids, &visible);

        let ((start, extent), cross) = axes(area, vertical);
        let end = u32::from(start) + u32::from(extent);
        let mut next = u32::from(start);
        for (i, (_, r)) in areas.iter().enumerate() {
            let ((s0, len), c) = axes(*r, vertical);
            prop_assert_eq!(c, cross, "{} spans the area across the split", i);
            prop_assert_eq!(u32::from(s0), next.min(end), "pane {} start", i);
            prop_assert!(u32::from(s0) + u32::from(len) <= end, "pane {} leaves the area", i);
            next = u32::from(s0) + u32::from(len) + 1;
        }
        if let Some((_, last)) = areas.last() {
            let ((s0, len), _) = axes(*last, vertical);
            prop_assert_eq!(u32::from(s0) + u32::from(len), end, "the last pane ends at the area's end");
        }
    }

    /// Without size limits, every pane but the last gets its share of the
    /// space left after the dividers (rounded down), and the last one the
    /// rest.
    #[test]
    fn prop_splitter_sizes_follow_the_ratios(
        ratios in prop::collection::vec(0.05f32..=1.0, 1..7),
        vertical in any::<bool>(),
        extent in 0u16..200,
    ) {
        let specs: Vec<PaneSpec> = ratios
            .iter()
            .map(|&ratio| PaneSpec { ratio, min: 0, max: 0, collapsed: false })
            .collect();
        let s = build(&specs, vertical);
        let area = if vertical { Rect::new(0, 0, 3, extent) } else { Rect::new(0, 0, extent, 3) };
        let sizes: Vec<u16> = s
            .pane_areas(area)
            .iter()
            .map(|(_, r)| axes(*r, vertical).0 .1)
            .collect();
        let available = extent.saturating_sub(ratios.len() as u16 - 1);
        let total: f32 = ratios.iter().sum();
        let n = sizes.len();
        for (i, (&size, &ratio)) in sizes.iter().zip(&ratios).enumerate().take(n - 1) {
            let share = f64::from(available) * f64::from(ratio) / f64::from(total);
            prop_assert!((f64::from(size) - share.floor()).abs() <= 1.0, "pane {}: {} for a share of {}", i, size, share);
        }
        // When the dividers fit, panes and dividers use the extent exactly
        if usize::from(extent) >= n - 1 {
            let used: u32 = sizes.iter().map(|&s| u32::from(s)).sum::<u32>() + (n as u32 - 1);
            prop_assert_eq!(used, u32::from(extent));
        }
    }

    /// A pane but the last keeps to its limits where the area has room:
    /// no wider than `max_size` (when set), and at least `min_size` unless
    /// `max_size` is smaller or the area runs out first.
    #[test]
    fn prop_splitter_panes_keep_their_limits(
        specs in prop::collection::vec(pane_spec(), 2..6),
        vertical in any::<bool>(),
        area in area(),
    ) {
        let specs: Vec<PaneSpec> = specs.into_iter().map(|p| PaneSpec { collapsed: false, ..p }).collect();
        let s = build(&specs, vertical);
        let areas = s.pane_areas(area);
        let ((start, extent), _) = axes(area, vertical);
        let end = u32::from(start) + u32::from(extent);
        for (i, ((_, r), p)) in areas.iter().zip(&specs).enumerate().take(specs.len() - 1) {
            let ((s0, len), _) = axes(*r, vertical);
            let room = end - u32::from(s0);
            if p.max > 0 {
                prop_assert!(len <= p.max, "pane {} is {} > max {}", i, len, p.max);
            }
            if p.max == 0 || p.min <= p.max {
                prop_assert!(u32::from(len) >= u32::from(p.min).min(room), "pane {} is {} < min {}", i, len, p.min);
            }
        }
    }
}

#[test]
fn test_splitter_pane_areas_vertical_with_offset() {
    let s = Splitter::new()
        .orientation(SplitOrientation::Vertical)
        .pane(Pane::new("a").ratio(0.25).min_size(0))
        .pane(Pane::new("b").ratio(0.75).min_size(0));
    // 17 rows minus the divider: 4 and 12
    let areas = s.pane_areas(Rect::new(5, 2, 3, 17));
    assert_eq!(areas[0].1, Rect::new(5, 2, 3, 4));
    assert_eq!(areas[1].1, Rect::new(5, 7, 3, 12));
}

#[test]
fn test_splitter_zero_ratios_split_evenly() {
    let s = Splitter::new()
        .pane(Pane::new("a").ratio(0.0).min_size(0))
        .pane(Pane::new("b").ratio(0.0).min_size(0))
        .pane(Pane::new("c").ratio(0.0).min_size(0));
    let widths: Vec<u16> = s
        .pane_areas(Rect::new(0, 0, 32, 1))
        .iter()
        .map(|(_, r)| r.width)
        .collect();
    assert_eq!(widths, vec![10, 10, 10]);
}

#[test]
fn test_splitter_max_size_gives_the_rest_to_the_last_pane() {
    let s = Splitter::new()
        .pane(Pane::new("a").max_size(4))
        .pane(Pane::new("b"));
    let widths: Vec<u16> = s
        .pane_areas(Rect::new(0, 0, 41, 1))
        .iter()
        .map(|(_, r)| r.width)
        .collect();
    assert_eq!(widths, vec![4, 36]);
}

#[test]
fn test_splitter_all_collapsed_has_no_areas() {
    let mut s = Splitter::new()
        .pane(Pane::new("a").collapsible())
        .pane(Pane::new("b").collapsible());
    s.toggle_pane(0);
    s.toggle_pane(1);
    assert!(s.pane_areas(Rect::new(0, 0, 20, 5)).is_empty());
}
