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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect_new() {
        let rect = Rect::new(10, 20, 30, 40);
        assert_eq!(rect.x, 10);
        assert_eq!(rect.y, 20);
        assert_eq!(rect.width, 30);
        assert_eq!(rect.height, 40);
    }

    #[test]
    fn test_rect_contains() {
        let rect = Rect::new(10, 10, 20, 20);

        assert!(rect.contains(10, 10)); // Top-left
        assert!(rect.contains(15, 15)); // Center
        assert!(rect.contains(29, 29)); // Just inside
        assert!(!rect.contains(30, 30)); // Just outside
        assert!(!rect.contains(5, 15)); // Left of rect
    }

    #[test]
    fn test_rect_edges() {
        let rect = Rect::new(10, 20, 30, 40);
        assert_eq!(rect.right(), 40);
        assert_eq!(rect.bottom(), 60);
    }

    #[test]
    fn test_rect_intersects() {
        let r1 = Rect::new(0, 0, 20, 20);
        let r2 = Rect::new(10, 10, 20, 20);
        let r3 = Rect::new(100, 100, 10, 10);

        assert!(r1.intersects(&r2));
        assert!(r2.intersects(&r1));
        assert!(!r1.intersects(&r3));
    }

    #[test]
    fn test_rect_intersection() {
        let r1 = Rect::new(0, 0, 20, 20);
        let r2 = Rect::new(10, 10, 20, 20);

        let intersection = r1.intersection(&r2).unwrap();
        assert_eq!(intersection, Rect::new(10, 10, 10, 10));

        let r3 = Rect::new(100, 100, 10, 10);
        assert!(r1.intersection(&r3).is_none());
    }

    #[test]
    fn test_rect_union() {
        let r1 = Rect::new(0, 0, 20, 20);
        let r2 = Rect::new(10, 10, 30, 30);

        let union = r1.union(&r2);
        assert_eq!(union, Rect::new(0, 0, 40, 40));
    }

    #[test]
    fn test_merge_rects_empty() {
        let rects: Vec<Rect> = vec![];
        let merged = merge_rects(&rects);
        assert!(merged.is_empty());
    }

    #[test]
    fn test_merge_rects_single() {
        let rects = vec![Rect::new(0, 0, 10, 10)];
        let merged = merge_rects(&rects);
        assert_eq!(merged.len(), 1);
        assert_eq!(merged[0], Rect::new(0, 0, 10, 10));
    }

    #[test]
    fn test_merge_rects_overlapping() {
        let rects = vec![Rect::new(0, 0, 20, 20), Rect::new(10, 10, 20, 20)];
        let merged = merge_rects(&rects);
        assert_eq!(merged.len(), 1);
        assert_eq!(merged[0], Rect::new(0, 0, 30, 30));
    }

    #[test]
    fn test_merge_rects_non_overlapping() {
        let rects = vec![Rect::new(0, 0, 10, 10), Rect::new(50, 50, 10, 10)];
        let merged = merge_rects(&rects);
        assert_eq!(merged.len(), 2);
        assert!(merged.contains(&Rect::new(0, 0, 10, 10)));
        assert!(merged.contains(&Rect::new(50, 50, 10, 10)));
    }

    #[test]
    fn test_merge_rects_multiple_overlapping() {
        let rects = vec![
            Rect::new(0, 0, 10, 10),
            Rect::new(5, 5, 10, 10),
            Rect::new(10, 10, 10, 10),
        ];
        let merged = merge_rects(&rects);
        assert_eq!(merged.len(), 1);
        assert_eq!(merged[0], Rect::new(0, 0, 20, 20));
    }
}
