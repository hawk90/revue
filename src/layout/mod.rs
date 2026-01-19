//! Layout engine for TUI
//!
//! Custom layout engine optimized for terminal user interfaces.
//! Supports flexbox, block, and grid layouts using integer cell coordinates.

mod block;
mod compute;
mod engine;
mod flex;
mod grid;
mod node;
mod position;
pub mod responsive;
mod tree;

pub use engine::{LayoutEngine, LayoutError, LayoutResult};
pub use responsive::{
    breakpoints, max_width, min_width, responsive as responsive_value, responsive_layout,
    Breakpoint, Breakpoints, MediaQuery, ResponsiveLayout, ResponsiveValue,
};

/// A rectangle representing a widget's position and size
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Rect {
    /// X position
    pub x: u16,
    /// Y position
    pub y: u16,
    /// Width
    pub width: u16,
    /// Height
    pub height: u16,
}

impl Rect {
    /// Create a new rectangle
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Check if a point is inside the rectangle
    pub fn contains(&self, x: u16, y: u16) -> bool {
        x >= self.x && x < self.right() && y >= self.y && y < self.bottom()
    }

    /// Get the right edge (saturates at u16::MAX to prevent overflow)
    pub fn right(&self) -> u16 {
        self.x.saturating_add(self.width)
    }

    /// Get the bottom edge (saturates at u16::MAX to prevent overflow)
    pub fn bottom(&self) -> u16 {
        self.y.saturating_add(self.height)
    }

    /// Check if this rectangle intersects with another
    pub fn intersects(&self, other: &Rect) -> bool {
        self.x < other.right()
            && self.right() > other.x
            && self.y < other.bottom()
            && self.bottom() > other.y
    }

    /// Get the intersection of two rectangles
    pub fn intersection(&self, other: &Rect) -> Option<Rect> {
        if !self.intersects(other) {
            return None;
        }

        let x = self.x.max(other.x);
        let y = self.y.max(other.y);
        let right = self.right().min(other.right());
        let bottom = self.bottom().min(other.bottom());

        Some(Rect {
            x,
            y,
            width: right - x,
            height: bottom - y,
        })
    }

    /// Get the union (bounding box) of two rectangles
    pub fn union(&self, other: &Rect) -> Rect {
        let x = self.x.min(other.x);
        let y = self.y.min(other.y);
        let right = self.right().max(other.right());
        let bottom = self.bottom().max(other.bottom());

        Rect {
            x,
            y,
            width: right.saturating_sub(x),
            height: bottom.saturating_sub(y),
        }
    }
}

/// Merge overlapping rectangles to minimize the number of update regions
pub fn merge_rects(rects: &[Rect]) -> Vec<Rect> {
    if rects.is_empty() {
        return Vec::new();
    }

    let mut merged = Vec::new();
    let mut remaining: Vec<Rect> = rects.to_vec();

    while !remaining.is_empty() {
        let mut current = remaining.remove(0);
        let mut changed = true;

        // Keep merging until no more overlaps
        while changed {
            changed = false;
            let mut i = 0;
            while i < remaining.len() {
                if current.intersects(&remaining[i]) {
                    current = current.union(&remaining[i]);
                    remaining.remove(i);
                    changed = true;
                } else {
                    i += 1;
                }
            }
        }

        merged.push(current);
    }

    merged
}

// Tests moved to tests/layout_tests.rs
