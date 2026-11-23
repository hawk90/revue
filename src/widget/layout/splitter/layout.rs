//! Pane geometry: where each visible pane sits along the split

use super::super::split_view::layout::{sizes as pane_sizes, PaneSpec};
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

        // The same sizing as SplitView: each pane its ratio's share, held to
        // its min and max, rounded so the panes add up to `available` and
        // equal ratios differ by at most a cell (#1514). The last pane keeps
        // its minimum too; it used to get only what the others left.
        let specs: Vec<PaneSpec> = visible_panes
            .iter()
            .map(|p| PaneSpec {
                weight: p.ratio,
                min: p.min_size,
                max: p.max_size,
            })
            .collect();
        let sizes = pane_sizes(&specs, available);

        // Panes go one after another with a divider between them; whatever
        // reaches past the extent is cut at its end, so no pane leaves the
        // area and no coordinate runs past `u16::MAX`
        let mut offset = 0u16;
        for (pane, size) in visible_panes.iter().zip(sizes) {
            let start = offset.min(extent);
            let size = size.min(extent - start);
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
