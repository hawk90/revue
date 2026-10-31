//! Sharing the main axis among the children: fixed, flex, content and
//! equal shares, and shrinking content that overflows

use std::cmp::Reverse;
use std::collections::BinaryHeap;

use super::{ChildSize, Stack};
impl Stack {
    /// Calculate sizes for children based on available space
    ///
    /// Strategy:
    /// - Fixed children get their exact size
    /// - Content (measured) children get their measured size; if together
    ///   with the fixed children they overflow, they shrink to fit what the
    ///   fixed children leave, each in proportion to its size - so a body
    ///   one row too tall loses that row instead of pushing the footer off
    ///   the screen
    /// - Flex children share remaining space proportionally by grow factor
    /// - Auto children share remaining space equally (after flex allocation);
    ///   one with a minimum keeps it, and the others share what is left
    /// - Minimums make room by shrinking the measured children, down to a cell
    ///   each, and never push a fixed child off: past that they get what is
    ///   left, like any other share
    pub(super) fn calculate_sizes(child_sizes: &[ChildSize], available: u16, n: usize) -> Vec<u16> {
        if n == 0 {
            return Vec::new();
        }

        // First pass: calculate fixed space and collect flex/auto info
        let mut auto_count: usize = 0;
        let mut total_grow: f32 = 0.0;
        let mut fixed_total = 0u16;
        let mut content_total = 0u32;

        for cs in child_sizes {
            match cs {
                ChildSize::Fixed(size) => fixed_total = fixed_total.saturating_add(*size),
                ChildSize::Content(size) => content_total += u32::from(*size),
                ChildSize::Flex(grow) => total_grow += grow,
                ChildSize::Auto | ChildSize::AutoMin(_) => auto_count += 1,
            }
        }

        let mut result = vec![0u16; n];

        // Assign fixed sizes
        for (i, cs) in child_sizes.iter().enumerate() {
            if let ChildSize::Fixed(size) = cs {
                result[i] = *size;
            }
        }

        // Measured sizes, shrunk to fit if they overflow.
        // A child that measured nothing has no cell to keep.
        let content_count = child_sizes
            .iter()
            .filter(|cs| matches!(cs, ChildSize::Content(size) if *size > 0))
            .count() as u32;
        let floors = child_sizes.iter().fold(0u32, |sum, cs| match cs {
            ChildSize::AutoMin(least) => sum + u32::from(*least),
            _ => sum,
        });
        let after_fixed = u32::from(available.saturating_sub(fixed_total));
        let reserved = floors.min(after_fixed.saturating_sub(content_count));
        let content_space = after_fixed - reserved;
        let content_used =
            Self::fit_content(child_sizes, content_total, content_space, &mut result);
        let remaining = available
            .saturating_sub(fixed_total)
            .saturating_sub(content_used);

        if total_grow > 0.0 {
            // Distribute remaining space to flex children proportionally
            let flex_indices: Vec<usize> = child_sizes
                .iter()
                .enumerate()
                .filter(|(_, cs)| matches!(cs, ChildSize::Flex(_)))
                .map(|(i, _)| i)
                .collect();

            // Space for flex children: remaining minus space for auto children
            // Each auto child's size while flex takes the rest: a cell, or its
            // minimum as far as there is room.
            // Minimums first, as without flex.
            let mut room = remaining;
            let mut auto_sizes = vec![0u16; n];
            for (i, cs) in child_sizes.iter().enumerate() {
                if let ChildSize::AutoMin(least) = cs {
                    auto_sizes[i] = (*least).min(room).max(1);
                    room = room.saturating_sub(auto_sizes[i]);
                }
            }
            for (i, cs) in child_sizes.iter().enumerate() {
                if matches!(cs, ChildSize::Auto) {
                    auto_sizes[i] = 1;
                    room = room.saturating_sub(1);
                }
            }
            let auto_min: u16 = remaining.saturating_sub(room);
            let flex_space = remaining.saturating_sub(auto_min);
            let mut distributed: u16 = 0;

            for (fi, &i) in flex_indices.iter().enumerate() {
                let grow = match child_sizes[i] {
                    ChildSize::Flex(g) => g,
                    _ => 0.0,
                };
                let size = if fi == flex_indices.len() - 1 {
                    flex_space.saturating_sub(distributed)
                } else {
                    ((flex_space as f32) * grow / total_grow).round() as u16
                };
                result[i] = size;
                distributed = distributed.saturating_add(size);
            }

            // Auto children get 1 pixel each when flex is active
            for (i, size) in auto_sizes.into_iter().enumerate() {
                if size > 0 {
                    result[i] = size;
                }
            }
        } else if auto_count > 0 {
            Self::share_remaining(child_sizes, remaining, auto_count, &mut result);
        }

        result
    }

    /// No flex: the auto children share `remaining` equally, rounding leftovers
    /// to the earliest. A child whose minimum is above its share keeps as much
    /// of it as is left, and leaves the others less; the total never overflows
    /// because of a minimum.
    fn share_remaining(
        child_sizes: &[ChildSize],
        remaining: u16,
        auto_count: usize,
        result: &mut [u16],
    ) {
        let mut pinned = vec![false; child_sizes.len()];
        let mut left = remaining;
        let mut sharing = auto_count as u16;
        loop {
            let share = left.checked_div(sharing).unwrap_or(0);
            let mut changed = false;
            for (i, cs) in child_sizes.iter().enumerate() {
                if let ChildSize::AutoMin(least) = cs {
                    if !pinned[i] && *least > share && left > 0 {
                        let size = (*least).min(left);
                        pinned[i] = true;
                        result[i] = size;
                        left -= size;
                        sharing -= 1;
                        changed = true;
                    }
                }
            }
            if !changed {
                break;
            }
        }
        if sharing == 0 {
            return;
        }
        // Nothing left still gives each child a cell.
        let (per_auto, extra) = if left > 0 {
            (left / sharing, left % sharing)
        } else {
            (1, 0)
        };
        let mut extra_given = 0u16;
        for (i, cs) in child_sizes.iter().enumerate() {
            if matches!(cs, ChildSize::Auto | ChildSize::AutoMin(_)) && !pinned[i] {
                let mut size = per_auto;
                if extra_given < extra {
                    size += 1;
                    extra_given += 1;
                }
                result[i] = size;
            }
        }
    }

    /// Give each `Content` child its measured size, or - when they total more
    /// than `space` - a share of `space` in proportion to that size (CSS
    /// `flex-shrink: 1`). Rounding leftovers go to the earliest children.
    /// Returns the space used.
    pub(super) fn fit_content(
        child_sizes: &[ChildSize],
        total: u32,
        space: u32,
        result: &mut [u16],
    ) -> u16 {
        let content = || {
            child_sizes
                .iter()
                .enumerate()
                .filter_map(|(i, cs)| match cs {
                    ChildSize::Content(size) => Some((i, u32::from(*size))),
                    _ => None,
                })
        };
        if total <= space {
            for (i, size) in content() {
                result[i] = size as u16;
            }
            return total as u16;
        }
        let mut used = 0u32;
        for (i, size) in content() {
            let share = size * space / total;
            result[i] = share as u16;
            used += share;
        }
        // Integer division leaves at most one cell per child undistributed.
        // It goes first to a child left with none, so a footer is not rounded
        // away beside a tall body, then to the earliest.
        for (i, size) in content() {
            if used >= space {
                break;
            }
            if size > 0 && result[i] == 0 {
                result[i] = 1;
                used += 1;
            }
        }
        // Still some with none, and room for one each: the largest shares give
        // up a cell, so proportions change only as far as that takes.
        // A heap: with thousands of children this runs on every frame.
        let mut donors: BinaryHeap<(u16, Reverse<usize>)> = content()
            .filter(|&(i, _)| result[i] > 1)
            .map(|(i, _)| (result[i], Reverse(i)))
            .collect();
        for (i, size) in content() {
            if size == 0 || result[i] > 0 {
                continue;
            }
            let Some((_, Reverse(donor))) = donors.pop() else {
                break;
            };
            result[donor] -= 1;
            result[i] = 1;
            if result[donor] > 1 {
                donors.push((result[donor], Reverse(donor)));
            }
        }
        for (i, size) in content() {
            if used >= space {
                break;
            }
            if u32::from(result[i]) < size {
                result[i] += 1;
                used += 1;
            }
        }
        used as u16
    }
}
