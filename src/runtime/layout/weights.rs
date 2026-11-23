//! Sharing a length among parts by weight, in whole cells

/// Split `total` cells among parts in proportion to `weights`.
///
/// Each part ends at its cumulative share of `total`, rounded: the sizes add
/// up to exactly `total`, every part gets its share rounded up or down, and
/// parts of equal weight differ by at most one cell. Rounding each share on
/// its own instead can use up the length early (four equal parts of 6 as
/// 2, 2, 2, 0) or leave the remainder to the last (1, 1, 1, 3).
///
/// A weight that is negative or not finite counts as zero. When no weight is
/// above zero, the parts share `total` equally.
pub(crate) fn split_by_weight(total: u16, weights: &[f32]) -> Vec<u16> {
    let weights: Vec<f64> = weights
        .iter()
        .map(|&w| {
            if w.is_finite() && w > 0.0 {
                f64::from(w)
            } else {
                0.0
            }
        })
        .collect();
    let sum: f64 = weights.iter().sum();
    let (weights, sum) = if sum > 0.0 && sum.is_finite() {
        (weights, sum)
    } else {
        (vec![1.0; weights.len()], weights.len() as f64)
    };

    let mut sizes = Vec::with_capacity(weights.len());
    let (mut acc, mut end) = (0.0f64, 0u16);
    for (i, w) in weights.iter().enumerate() {
        acc += w;
        let next = if i + 1 == weights.len() {
            // The last part ends at `total` exactly, whatever the float sum
            total
        } else {
            ((f64::from(total) * acc / sum).round() as u16).clamp(end, total)
        };
        sizes.push(next - end);
        end = next;
    }
    sizes
}

#[cfg(test)]
mod tests {
    use super::split_by_weight;

    #[test]
    fn equal_weights_differ_by_at_most_one_cell() {
        assert_eq!(split_by_weight(6, &[1.0; 4]), [2, 1, 2, 1]);
        assert_eq!(split_by_weight(7, &[1.0; 4]), [2, 2, 1, 2]);
        assert_eq!(split_by_weight(3, &[1.0; 4]), [1, 1, 0, 1]);
        for total in 0..60u16 {
            for n in 1..9 {
                let sizes = split_by_weight(total, &vec![1.0; n]);
                assert_eq!(
                    sizes.iter().map(|&s| u32::from(s)).sum::<u32>(),
                    u32::from(total)
                );
                let (lo, hi) = (sizes.iter().min().unwrap(), sizes.iter().max().unwrap());
                assert!(hi - lo <= 1, "{total} / {n}: {sizes:?}");
            }
        }
    }

    #[test]
    fn each_part_is_its_share_rounded_up_or_down() {
        let weights = [1.0, 2.0, 3.0, 0.5, 7.0];
        let sum: f32 = weights.iter().sum();
        for total in [0u16, 1, 5, 13, 80, 1000, u16::MAX] {
            let sizes = split_by_weight(total, &weights);
            assert_eq!(
                sizes.iter().map(|&s| u32::from(s)).sum::<u32>(),
                u32::from(total)
            );
            for (s, w) in sizes.iter().zip(weights) {
                let share = f64::from(total) * f64::from(w) / f64::from(sum);
                assert!(
                    (f64::from(*s) - share).abs() < 1.0 + 1e-9,
                    "{total}: {sizes:?}"
                );
            }
        }
    }

    #[test]
    fn odd_weights() {
        // Zero, negative and non-finite weights take nothing
        assert_eq!(
            split_by_weight(10, &[1.0, 0.0, -2.0, f32::NAN, 1.0]),
            [5, 0, 0, 0, 5]
        );
        // No weight above zero: equal shares
        assert_eq!(split_by_weight(6, &[0.0, 0.0, 0.0]), [2, 2, 2]);
        assert_eq!(split_by_weight(6, &[f32::INFINITY, 1.0]), [0, 6]);
        // No parts
        assert!(split_by_weight(6, &[]).is_empty());
        // Huge weights do not overflow the sum
        assert_eq!(split_by_weight(4, &[f32::MAX, f32::MAX]), [2, 2]);
    }
}
