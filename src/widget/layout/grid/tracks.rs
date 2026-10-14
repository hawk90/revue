//! Track sizing: how wide each column and how tall each row is, and
//! where each track starts

use super::core::Grid;
use super::types::TrackSize;

impl Grid {
    /// Calculate track sizes
    pub(crate) fn calculate_tracks(
        &self,
        available: u16,
        tracks: &[TrackSize],
        _auto_size: TrackSize,
        gap: u16,
    ) -> Vec<u16> {
        if tracks.is_empty() {
            return vec![];
        }

        let total_gaps = (tracks.len().saturating_sub(1)) as u16 * gap;
        let available = available.saturating_sub(total_gaps);

        // First pass: calculate fixed sizes and collect fr units
        let mut sizes: Vec<u16> = vec![0; tracks.len()];
        let mut total_fr = 0.0f32;
        let mut remaining = available;

        for (i, track) in tracks.iter().enumerate() {
            match track {
                TrackSize::Fixed(size) => {
                    sizes[i] = *size;
                    remaining = remaining.saturating_sub(*size);
                }
                TrackSize::Percent(pct) => {
                    let size = ((available as f32) * pct / 100.0) as u16;
                    sizes[i] = size;
                    remaining = remaining.saturating_sub(size);
                }
                TrackSize::Fr(fr) => {
                    total_fr += fr;
                }
                TrackSize::Auto | TrackSize::MinContent | TrackSize::MaxContent => {
                    // For now, treat auto as 1fr
                    total_fr += 1.0;
                }
            }
        }

        // Second pass: distribute remaining space to fr units
        if total_fr > 0.0 {
            let per_fr = (remaining as f32) / total_fr;
            for (i, track) in tracks.iter().enumerate() {
                match track {
                    TrackSize::Fr(fr) => {
                        sizes[i] = (per_fr * fr) as u16;
                    }
                    TrackSize::Auto | TrackSize::MinContent | TrackSize::MaxContent => {
                        sizes[i] = per_fr as u16;
                    }
                    _ => {}
                }
            }
        }

        sizes
    }

    /// Get track positions (cumulative)
    pub(crate) fn track_positions(&self, sizes: &[u16], gap: u16) -> Vec<u16> {
        let mut positions = Vec::with_capacity(sizes.len() + 1);
        let mut pos = 0u16;
        positions.push(pos);

        for (i, &size) in sizes.iter().enumerate() {
            pos += size;
            if i < sizes.len() - 1 {
                pos += gap;
            }
            positions.push(pos);
        }

        positions
    }

    // Test accessor methods
    #[doc(hidden)]
    pub fn test_calculate_tracks(
        &self,
        available: u16,
        tracks: &[TrackSize],
        auto_size: TrackSize,
        gap: u16,
    ) -> Vec<u16> {
        self.calculate_tracks(available, tracks, auto_size, gap)
    }

    #[doc(hidden)]
    pub fn test_track_positions(&self, sizes: &[u16], gap: u16) -> Vec<u16> {
        self.track_positions(sizes, gap)
    }
}
