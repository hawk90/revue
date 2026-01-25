//! Skeleton widget for loading placeholders

use crate::render::Cell;
use crate::style::Color;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// Skeleton shape variant
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SkeletonShape {
    /// Rectangle/line (default)
    #[default]
    Rectangle,
    /// Circle/avatar placeholder
    Circle,
    /// Multiple lines (paragraph)
    Paragraph,
}

/// A skeleton widget for loading placeholders
///
/// # Example
///
/// ```rust,ignore
/// use revue::prelude::*;
///
/// if loading {
///     skeleton().width(20).height(3)
/// } else {
///     text("Content loaded!")
/// }
/// ```
pub struct Skeleton {
    /// Width (0 = fill)
    width: u16,
    /// Height
    height: u16,
    /// Shape
    shape: SkeletonShape,
    /// Number of lines (for paragraph)
    lines: u16,
    /// Animation frame (for pulse effect)
    frame: u8,
    /// Animate
    animate: bool,
    /// Color
    color: Color,
    /// Wave character
    wave_char: char,
    /// CSS styling properties (id, classes)
    props: WidgetProps,
}

impl Skeleton {
    /// Create a new skeleton
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 1,
            shape: SkeletonShape::Rectangle,
            lines: 3,
            frame: 0,
            animate: true,
            color: Color::rgb(60, 60, 60),
            wave_char: '░',
            props: WidgetProps::new(),
        }
    }

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
        self.color = color;
        self
    }

    /// Set animation frame (for external animation control)
    pub fn frame(mut self, frame: u8) -> Self {
        self.frame = frame;
        self
    }

    /// Get the skeleton character based on animation
    fn skeleton_char(&self) -> char {
        if self.animate {
            // Cycle through shading characters
            match self.frame % 4 {
                0 => '░',
                1 => '▒',
                2 => '▓',
                _ => '▒',
            }
        } else {
            self.wave_char
        }
    }
}

impl Default for Skeleton {
    fn default() -> Self {
        Self::new()
    }
}

impl View for Skeleton {
    crate::impl_view_meta!("Skeleton");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let ch = self.skeleton_char();

        match self.shape {
            SkeletonShape::Rectangle => {
                let width = if self.width > 0 {
                    self.width.min(area.width)
                } else {
                    area.width
                };
                let height = self.height.min(area.height);

                for y in 0..height {
                    for x in 0..width {
                        let mut cell = Cell::new(ch);
                        cell.fg = Some(self.color);
                        ctx.buffer.set(area.x + x, area.y + y, cell);
                    }
                }
            }
            SkeletonShape::Circle => {
                // Simple circle representation using unicode
                // For small sizes, just use filled block
                let size = self.height.max(1).min(area.height);

                if size == 1 {
                    let mut cell = Cell::new('●');
                    cell.fg = Some(self.color);
                    ctx.buffer.set(area.x, area.y, cell);
                } else if size == 2 {
                    // 2x2 circle
                    let chars = ['╭', '╮', '╰', '╯'];
                    for (i, c) in chars.iter().enumerate() {
                        let x = (i % 2) as u16;
                        let y = (i / 2) as u16;
                        let mut cell = Cell::new(*c);
                        cell.fg = Some(self.color);
                        ctx.buffer.set(area.x + x, area.y + y, cell);
                    }
                } else {
                    // Larger circle approximation
                    // Top row
                    let mut tl = Cell::new('╭');
                    tl.fg = Some(self.color);
                    ctx.buffer.set(area.x, area.y, tl);

                    for x in 1..size - 1 {
                        let mut cell = Cell::new('─');
                        cell.fg = Some(self.color);
                        ctx.buffer.set(area.x + x, area.y, cell);
                    }

                    let mut tr = Cell::new('╮');
                    tr.fg = Some(self.color);
                    ctx.buffer.set(area.x + size - 1, area.y, tr);

                    // Middle rows
                    for y in 1..size - 1 {
                        let mut left = Cell::new('│');
                        left.fg = Some(self.color);
                        ctx.buffer.set(area.x, area.y + y, left);

                        for x in 1..size - 1 {
                            let mut cell = Cell::new(ch);
                            cell.fg = Some(self.color);
                            ctx.buffer.set(area.x + x, area.y + y, cell);
                        }

                        let mut right = Cell::new('│');
                        right.fg = Some(self.color);
                        ctx.buffer.set(area.x + size - 1, area.y + y, right);
                    }

                    // Bottom row
                    let mut bl = Cell::new('╰');
                    bl.fg = Some(self.color);
                    ctx.buffer.set(area.x, area.y + size - 1, bl);

                    for x in 1..size - 1 {
                        let mut cell = Cell::new('─');
                        cell.fg = Some(self.color);
                        ctx.buffer.set(area.x + x, area.y + size - 1, cell);
                    }

                    let mut br = Cell::new('╯');
                    br.fg = Some(self.color);
                    ctx.buffer.set(area.x + size - 1, area.y + size - 1, br);
                }
            }
            SkeletonShape::Paragraph => {
                let width = if self.width > 0 {
                    self.width.min(area.width)
                } else {
                    area.width
                };
                let lines = self.lines.min(area.height);

                for line in 0..lines {
                    // Vary line lengths for realism
                    let line_width = if line == lines - 1 {
                        width * 2 / 3 // Last line shorter
                    } else if line % 2 == 1 {
                        width.saturating_sub(4) // Alternate lines slightly shorter
                    } else {
                        width
                    };

                    for x in 0..line_width {
                        let mut cell = Cell::new(ch);
                        cell.fg = Some(self.color);
                        ctx.buffer.set(area.x + x, area.y + line, cell);
                    }
                }
            }
        }
    }
}

impl_styled_view!(Skeleton);
impl_props_builders!(Skeleton);

/// Create a new skeleton
pub fn skeleton() -> Skeleton {
    Skeleton::new()
}

/// Create a text line skeleton
pub fn skeleton_text() -> Skeleton {
    Skeleton::new().height(1)
}

/// Create an avatar skeleton
pub fn skeleton_avatar() -> Skeleton {
    Skeleton::new().circle().height(3)
}

/// Create a paragraph skeleton
pub fn skeleton_paragraph() -> Skeleton {
    Skeleton::new().paragraph().lines(3)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    #[test]
    fn test_skeleton_new() {
        let s = Skeleton::new();
        assert_eq!(s.shape, SkeletonShape::Rectangle);
        assert!(s.animate);
    }

    #[test]
    fn test_skeleton_shapes() {
        let s = skeleton().circle();
        assert_eq!(s.shape, SkeletonShape::Circle);

        let s = skeleton().paragraph();
        assert_eq!(s.shape, SkeletonShape::Paragraph);
    }

    #[test]
    fn test_skeleton_dimensions() {
        let s = skeleton().width(10).height(3);
        assert_eq!(s.width, 10);
        assert_eq!(s.height, 3);
    }

    #[test]
    fn test_skeleton_animation() {
        let s = skeleton().frame(0);
        assert_eq!(s.skeleton_char(), '░');

        let s = skeleton().frame(1);
        assert_eq!(s.skeleton_char(), '▒');

        let s = skeleton().no_animate();
        assert!(!s.animate);
    }

    #[test]
    fn test_skeleton_render_rectangle() {
        let mut buffer = Buffer::new(10, 2);
        let area = Rect::new(0, 0, 10, 2);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let s = skeleton().width(5).height(2).no_animate();
        s.render(&mut ctx);

        // Should fill the area with skeleton chars
        assert_eq!(buffer.get(0, 0).map(|c| c.symbol), Some('░'));
    }

    #[test]
    fn test_skeleton_render_paragraph() {
        let mut buffer = Buffer::new(20, 5);
        let area = Rect::new(0, 0, 20, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let s = skeleton_paragraph().lines(3);
        s.render(&mut ctx);

        // Should have 3 lines
        assert!(buffer.get(0, 0).map(|c| c.symbol).is_some());
        assert!(buffer.get(0, 1).map(|c| c.symbol).is_some());
        assert!(buffer.get(0, 2).map(|c| c.symbol).is_some());
    }

    #[test]
    fn test_helper_functions() {
        let s = skeleton();
        assert_eq!(s.shape, SkeletonShape::Rectangle);

        let s = skeleton_text();
        assert_eq!(s.height, 1);

        let s = skeleton_avatar();
        assert_eq!(s.shape, SkeletonShape::Circle);

        let s = skeleton_paragraph();
        assert_eq!(s.shape, SkeletonShape::Paragraph);
    }
}
