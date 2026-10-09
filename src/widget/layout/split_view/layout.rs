//! Pane sizes along the split axis

/// What a pane asks for: a share of the room, within its bounds
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct PaneSpec {
    /// Share of the room, relative to the other panes
    pub weight: f32,
    /// Fewest cells
    pub min: u16,
    /// Most cells (0 = no limit)
    pub max: u16,
}

/// Sizes for `specs` that add up to exactly `available` cells.
///
/// Each pane gets its weight's share, held to its bounds: a pane whose share
/// falls outside them gets the bound, and the others share what is left.
/// Shares are rounded so the boundaries between panes land where the exact
/// shares put them, so the sizes always add up. When the minimums alone
/// overrun `available`, the panes at the end are cut; when every pane is held
/// at its bound and room is left over, the last pane takes it.
pub(super) fn sizes(specs: &[PaneSpec], available: u16) -> Vec<u16> {
    let mut fixed: Vec<Option<u16>> = vec![None; specs.len()];

    loop {
        let free: Vec<usize> = (0..specs.len()).filter(|&i| fixed[i].is_none()).collect();
        if free.is_empty() {
            break;
        }
        let room = f64::from(available.saturating_sub(fixed_total(&fixed)));
        let (weights, total) = free_weights(specs, &free);
        let mut changed = false;
        for (&i, w) in free.iter().zip(weights) {
            let share = room * w / total;
            let spec = specs[i];
            if share < f64::from(spec.min) {
                fixed[i] = Some(spec.min);
                changed = true;
            } else if spec.max > 0 && share > f64::from(spec.max) {
                fixed[i] = Some(spec.max);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }

    // Round the free panes' shares at their cumulative boundaries
    let free: Vec<usize> = (0..specs.len()).filter(|&i| fixed[i].is_none()).collect();
    let room = available.saturating_sub(fixed_total(&fixed));
    let (weights, total) = free_weights(specs, &free);
    let mut sizes: Vec<u16> = fixed.iter().map(|s| s.unwrap_or(0)).collect();
    let (mut sum, mut end) = (0.0, 0u16);
    for (&i, w) in free.iter().zip(weights) {
        sum += w;
        let next = (f64::from(room) * sum / total).round() as u16;
        sizes[i] = next.saturating_sub(end);
        end = next;
    }

    // Fit the total to `available`
    let used: u32 = sizes.iter().map(|&s| u32::from(s)).sum();
    if used > u32::from(available) {
        let mut left = available;
        for size in &mut sizes {
            *size = (*size).min(left);
            left -= *size;
        }
    } else if let Some(last) = sizes.last_mut() {
        *last += available - used as u16;
    }
    sizes
}

fn fixed_total(fixed: &[Option<u16>]) -> u16 {
    fixed
        .iter()
        .flatten()
        .fold(0u16, |sum, &s| sum.saturating_add(s))
}

/// The free panes' weights and their sum. Negative weights count as 0;
/// when none is positive, each counts as 1, so they share equally.
fn free_weights(specs: &[PaneSpec], free: &[usize]) -> (Vec<f64>, f64) {
    let weights: Vec<f64> = free
        .iter()
        .map(|&i| f64::from(specs[i].weight.max(0.0)))
        .collect();
    let sum: f64 = weights.iter().sum();
    if sum > 0.0 {
        (weights, sum)
    } else {
        (vec![1.0; free.len()], free.len() as f64)
    }
}

/// The weight to give pane `a` so that it gets `target` cells, keeping the
/// sum of its weight and the next pane's.
///
/// Searches the weights between 0 and that sum for those that give `target`
/// and returns the middle of them, so the split keeps its proportion when
/// `available` changes. A target outside what the bounds allow is moved to
/// the nearest size they do.
pub(super) fn weight_for(specs: &[PaneSpec], available: u16, a: usize, target: u16) -> f32 {
    let pair = specs[a].weight.max(0.0) + specs[a + 1].weight.max(0.0);
    let size_at = |w: f32| {
        let mut specs = specs.to_vec();
        specs[a].weight = w;
        specs[a + 1].weight = pair - w;
        sizes(&specs, available)[a]
    };
    let target = target.clamp(size_at(0.0), size_at(pair));

    // The least weight that gives at least `size` cells
    let least = |size: u16| {
        let (mut lo, mut hi) = (0.0f32, pair);
        for _ in 0..40 {
            let mid = (lo + hi) / 2.0;
            if size_at(mid) >= size {
                hi = mid;
            } else {
                lo = mid;
            }
        }
        hi
    };
    let lo = least(target);
    let hi = if size_at(pair) > target {
        least(target + 1)
    } else {
        pair
    };
    (lo + hi) / 2.0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spec(weight: f32, min: u16, max: u16) -> PaneSpec {
        PaneSpec { weight, min, max }
    }

    #[test]
    fn shares_by_weight_and_adds_up() {
        assert_eq!(sizes(&[spec(1.0, 0, 0); 3], 20), vec![7, 6, 7]);
        assert_eq!(sizes(&[spec(1.0, 0, 0), spec(3.0, 0, 0)], 20), vec![5, 15]);
    }

    #[test]
    fn holds_panes_to_their_bounds() {
        // a wants 2 of 20 but needs 5; b takes the rest
        assert_eq!(sizes(&[spec(0.1, 5, 0), spec(0.9, 0, 0)], 20), vec![5, 15]);
        // a wants 18 but may have 6
        assert_eq!(sizes(&[spec(0.9, 0, 6), spec(0.1, 0, 0)], 20), vec![6, 14]);
    }

    #[test]
    fn cuts_the_last_panes_when_the_minimums_overrun() {
        assert_eq!(sizes(&[spec(1.0, 8, 0); 3], 20), vec![8, 8, 4]);
        assert_eq!(sizes(&[spec(1.0, 8, 0); 3], 0), vec![0, 0, 0]);
    }

    #[test]
    fn the_last_pane_takes_what_the_bounds_leave() {
        assert_eq!(sizes(&[spec(1.0, 0, 3), spec(1.0, 0, 4)], 20), vec![3, 17]);
    }

    #[test]
    fn zero_weights_share_equally() {
        assert_eq!(sizes(&[spec(0.0, 0, 0); 2], 10), vec![5, 5]);
    }

    #[test]
    fn no_panes() {
        assert!(sizes(&[], 10).is_empty());
    }

    #[test]
    fn weight_for_gives_the_target_and_keeps_the_proportion() {
        let specs = [spec(0.5, 0, 0), spec(0.5, 0, 0)];
        let w = weight_for(&specs, 20, 0, 5);
        let resized = [spec(w, 0, 0), spec(1.0 - w, 0, 0)];
        assert_eq!(sizes(&resized, 20), vec![5, 15]);
        assert_eq!(sizes(&resized, 40), vec![10, 30]);
    }

    #[test]
    fn weight_for_stops_at_the_bounds() {
        let specs = [spec(0.5, 0, 0), spec(0.5, 8, 0)];
        let w = weight_for(&specs, 20, 0, 19);
        assert_eq!(
            sizes(&[spec(w, 0, 0), spec(1.0 - w, 8, 0)], 20),
            vec![12, 8]
        );
    }
}
