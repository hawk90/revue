//! Clipping region support

/// A rectangular clipping region
#[derive(Clone, Copy, Debug)]
pub struct ClipRegion {
    /// Minimum X coordinate
    pub x_min: f64,
    /// Minimum Y coordinate
    pub y_min: f64,
    /// Maximum X coordinate
    pub x_max: f64,
    /// Maximum Y coordinate
    pub y_max: f64,
}

impl ClipRegion {
    /// Create a new clipping region
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x_min: x,
            y_min: y,
            x_max: x + width,
            y_max: y + height,
        }
    }

    /// Create from min/max coordinates
    pub fn from_bounds(x_min: f64, y_min: f64, x_max: f64, y_max: f64) -> Self {
        Self {
            x_min,
            y_min,
            x_max,
            y_max,
        }
    }

    /// Check if a point is inside the clipping region
    pub fn contains(&self, x: f64, y: f64) -> bool {
        x >= self.x_min && x <= self.x_max && y >= self.y_min && y <= self.y_max
    }

    /// Intersect with another clipping region
    pub fn intersect(&self, other: &ClipRegion) -> Option<ClipRegion> {
        let x_min = self.x_min.max(other.x_min);
        let y_min = self.y_min.max(other.y_min);
        let x_max = self.x_max.min(other.x_max);
        let y_max = self.y_max.min(other.y_max);

        if x_min <= x_max && y_min <= y_max {
            Some(ClipRegion {
                x_min,
                y_min,
                x_max,
                y_max,
            })
        } else {
            None
        }
    }
}
