//! Sharing the main axis among the children: fixed, flex, content and
//! equal shares, and shrinking content that overflows

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
    /// - Auto children share remaining space equally (after flex allocation)
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
                ChildSize::Auto => auto_count += 1,
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
        let content_space = u32::from(available.saturating_sub(fixed_total));
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
            let auto_min = auto_count as u16;
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
            for (i, cs) in child_sizes.iter().enumerate() {
                if matches!(cs, ChildSize::Auto) {
                    result[i] = 1;
                }
            }
        } else if auto_count > 0 {
            // No flex: distribute remaining space equally to auto children
            let (per_auto, extra) = if remaining > 0 {
                (
                    remaining / (auto_count as u16),
                    remaining % (auto_count as u16),
                )
            } else {
                (1, 0)
            };

            let mut extra_given = 0u16;
            for (i, cs) in child_sizes.iter().enumerate() {
                if matches!(cs, ChildSize::Auto) {
                    let mut size = per_auto;
                    if extra_given < extra {
                        size += 1;
                        extra_given += 1;
                    }
                    result[i] = size;
                }
            }
        }

        result
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
