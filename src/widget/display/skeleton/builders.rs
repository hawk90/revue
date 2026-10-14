//! Skeleton builders: size, shape, animation and color

use super::{Skeleton, SkeletonShape};
use crate::style::Color;

impl Skeleton {
    /// Set width
    pub fn width(mut self, width: u16) -> Self {
        self.width = width;
        self
    }

    /// Set height
    pub fn height(mut self, height: u16) -> Self {
        self.height = height;
        self
    }

    /// Set shape
    pub fn shape(mut self, shape: SkeletonShape) -> Self {
        self.shape = shape;
        self
    }

    /// Rectangle shape shorthand
    pub fn rectangle(mut self) -> Self {
        self.shape = SkeletonShape::Rectangle;
        self
    }

    /// Circle shape shorthand (for avatar placeholders)
    pub fn circle(mut self) -> Self {
        self.shape = SkeletonShape::Circle;
        self
    }

    /// Paragraph shape shorthand
    pub fn paragraph(mut self) -> Self {
        self.shape = SkeletonShape::Paragraph;
        self
    }

    /// Set number of lines (for paragraph)
    pub fn lines(mut self, lines: u16) -> Self {
        self.lines = lines;
        self
    }

    /// Disable animation
    pub fn no_animate(mut self) -> Self {
        self.animate = false;
        self
    }

    /// Set color
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Set animation frame (for external animation control)
    pub fn frame(mut self, frame: u8) -> Self {
        self.frame = frame;
        self
    }
}
