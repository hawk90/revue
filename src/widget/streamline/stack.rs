//! Stacking: the layer order and each point's baseline

use super::core::Streamline;
use super::types::{StreamBaseline, StreamOrder};

impl Streamline {
    /// Compute stack positions for all layers
    pub fn compute_stacks(&self) -> Vec<Vec<(f64, f64)>> {
        if self.layers.is_empty() {
            return Vec::new();
        }

        let num_points = self
            .layers
            .iter()
            .map(|l| l.values.len())
            .max()
            .unwrap_or(0);

        if num_points == 0 {
            return Vec::new();
        }

        // Order layers if needed
        let ordered_indices = self.ordered_indices();

        let mut stacks: Vec<Vec<(f64, f64)>> = vec![Vec::new(); self.layers.len()];

        for x in 0..num_points {
            let values: Vec<f64> = ordered_indices
                .iter()
                .map(|&i| self.layers[i].values.get(x).copied().unwrap_or(0.0))
                .collect();

            let total: f64 = values.iter().sum();

            let (y0, scale) = self.baseline_at(&values, total);

            let mut y = y0;
            for (i, &orig_idx) in ordered_indices.iter().enumerate() {
                let value = values[i] * scale;
                let y_start = y;
                let y_end = y + value;
                stacks[orig_idx].push((y_start, y_end));
                y = y_end;
            }
        }

        stacks
    }

    /// Layer indices in stacking order, the first stacked lowest
    fn ordered_indices(&self) -> Vec<usize> {
        match self.order {
            StreamOrder::None => (0..self.layers.len()).collect(),
            StreamOrder::Ascending => self.sorted_by_total(false),
            StreamOrder::Descending => self.sorted_by_total(true),
            StreamOrder::InsideOut => {
                let indices = self.sorted_by_total(true);
                let mut result = Vec::with_capacity(indices.len());
                for (i, idx) in indices.into_iter().enumerate() {
                    if i % 2 == 0 {
                        result.push(idx);
                    } else {
                        result.insert(0, idx);
                    }
                }
                result
            }
        }
    }

    /// Layer indices sorted by each layer's value sum
    fn sorted_by_total(&self, descending: bool) -> Vec<usize> {
        let mut indices: Vec<_> = (0..self.layers.len()).collect();
        indices.sort_by(|&a, &b| {
            let sum_a: f64 = self.layers[a].values.iter().sum();
            let sum_b: f64 = self.layers[b].values.iter().sum();
            let (first, second) = if descending {
                (sum_b, sum_a)
            } else {
                (sum_a, sum_b)
            };
            first.total_cmp(&second)
        });
        indices
    }

    /// The lowest band's start and the value scale at one point, from the
    /// point's `values` in stacking order and their `total`
    fn baseline_at(&self, values: &[f64], total: f64) -> (f64, f64) {
        match self.baseline {
            StreamBaseline::Zero => (0.0, 1.0),
            StreamBaseline::Symmetric => (-total / 2.0, 1.0),
            StreamBaseline::Wiggle => {
                let n = self.layers.len() as f64;
                let offset: f64 = values
                    .iter()
                    .enumerate()
                    .map(|(i, &v)| (n - i as f64) * v)
                    .sum::<f64>()
                    / n;
                (-offset / 2.0, 1.0)
            }
            StreamBaseline::Expand => {
                let scale = if total > 0.0 { 1.0 / total } else { 1.0 };
                (0.0, scale)
            }
        }
    }
}
