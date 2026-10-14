//! Pane geometry: where each visible pane sits along the split

use super::{SplitOrientation, Splitter};
use crate::layout::Rect;

impl Splitter {
    /// Get pane areas
    pub fn pane_areas(&self, area: Rect) -> Vec<(String, Rect)> {
        let mut areas = Vec::new();
        let visible_panes: Vec<_> = self.panes.iter().filter(|p| !p.collapsed).collect();

        if visible_panes.is_empty() {
            return areas;
        }

        let dividers = u16::try_from(visible_panes.len().saturating_sub(1)).unwrap_or(u16::MAX);
        let total_splitter_width = dividers.saturating_mul(self.splitter_width);
        let extent = match self.orientation {
            SplitOrientation::Horizontal => area.width,
            SplitOrientation::Vertical => area.height,
        };
        let available = extent.saturating_sub(total_splitter_width);

        // Normalize ratios
        let total_ratio: f32 = visible_panes.iter().map(|p| p.ratio).sum();
        let mut offset = 0u16;

        for (i, pane) in visible_panes.iter().enumerate() {
            let ratio = if total_ratio > 0.0 {
                pane.ratio / total_ratio
            } else {
                1.0 / visible_panes.len() as f32
            };
            let size = (available as f32 * ratio).clamp(0.0, available as f32);
            let mut size = size as u16;

            // Apply constraints
            size = size.max(pane.min_size);
            if pane.max_size > 0 {
                size = size.min(pane.max_size);
            }

            // Last pane takes the remaining space. `offset` already counts
            // the dividers placed so far, so measure it against the whole
            // extent, not against `available` (which has them taken out).
            // Panes whose minimum sizes overrun the extent are cut at its end
            // (and later ones get no room), so no pane leaves the area and
            // no coordinate runs past `u16::MAX`.
            let remaining = extent.saturating_sub(offset);
            if i == visible_panes.len() - 1 {
                size = remaining;
            }
            size = size.min(remaining);
            let start = offset.min(extent);

            let pane_area = match self.orientation {
                SplitOrientation::Horizontal => {
                    Rect::new(area.x.saturating_add(start), area.y, size, area.height)
                }
                SplitOrientation::Vertical => {
                    Rect::new(area.x, area.y.saturating_add(start), area.width, size)
                }
            };

            areas.push((pane.id.clone(), pane_area));
            offset = offset
                .saturating_add(size)
                .saturating_add(self.splitter_width);
        }

        areas
    }
}
