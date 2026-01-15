//! Canvas widget for custom drawing
//!
//! Provides a drawing surface for rendering custom graphics like
//! Gantt charts, diagrams, graphs, etc.
//!
//! Supports two rendering modes:
//! - **Character mode**: Standard character-based drawing
//! - **Braille mode**: High-resolution drawing using braille patterns (2x4 dots per cell)

use super::traits::{RenderContext, View};
use crate::layout::Rect;
use crate::render::{Cell, Modifier};
use crate::style::Color;

// =============================================================================
// Braille Constants
// =============================================================================

/// Braille dot pattern offsets
/// Each braille character is a 2x4 grid of dots:
/// ```text
/// (0,0) (1,0)    0x01  0x08
/// (0,1) (1,1)    0x02  0x10
/// (0,2) (1,2)    0x04  0x20
/// (0,3) (1,3)    0x40  0x80
/// ```
const BRAILLE_DOTS: [[u8; 4]; 2] = [
    [0x01, 0x02, 0x04, 0x40], // Left column (x=0)
    [0x08, 0x10, 0x20, 0x80], // Right column (x=1)
];

/// Base braille character (empty pattern)
const BRAILLE_BASE: u32 = 0x2800;

// =============================================================================
// Shapes for Braille Canvas
// =============================================================================

/// A shape that can be drawn on a braille canvas
pub trait Shape {
    /// Draw the shape onto the braille grid
    fn draw(&self, grid: &mut BrailleGrid);
}

/// A line segment
#[derive(Clone, Debug)]
pub struct Line {
    /// Start X coordinate
    pub x0: f64,
    /// Start Y coordinate
    pub y0: f64,
    /// End X coordinate
    pub x1: f64,
    /// End Y coordinate
    pub y1: f64,
    /// Line color
    pub color: Color,
}

impl Line {
    /// Create a new line
    pub fn new(x0: f64, y0: f64, x1: f64, y1: f64, color: Color) -> Self {
        Self {
            x0,
            y0,
            x1,
            y1,
            color,
        }
    }
}

impl Shape for Line {
    fn draw(&self, grid: &mut BrailleGrid) {
        // Bresenham's line algorithm for floating point
        let dx = (self.x1 - self.x0).abs();
        let dy = (self.y1 - self.y0).abs();
        let sx = if self.x0 < self.x1 { 1.0 } else { -1.0 };
        let sy = if self.y0 < self.y1 { 1.0 } else { -1.0 };
        let mut err = dx - dy;

        let mut x = self.x0;
        let mut y = self.y0;

        loop {
            grid.set(x as usize, y as usize, self.color);

            if (x - self.x1).abs() < 0.5 && (y - self.y1).abs() < 0.5 {
                break;
            }

            let e2 = 2.0 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
    }
}

/// A circle
#[derive(Clone, Debug)]
pub struct Circle {
    /// Center X coordinate
    pub x: f64,
    /// Center Y coordinate
    pub y: f64,
    /// Radius
    pub radius: f64,
    /// Circle color
    pub color: Color,
}

impl Circle {
    /// Create a new circle
    pub fn new(x: f64, y: f64, radius: f64, color: Color) -> Self {
        Self {
            x,
            y,
            radius,
            color,
        }
    }
}

impl Shape for Circle {
    fn draw(&self, grid: &mut BrailleGrid) {
        // Midpoint circle algorithm
        let mut x = self.radius as i32;
        let mut y = 0i32;
        let mut err = 0i32;

        while x >= y {
            // Draw 8 octants
            let points = [
                (self.x as i32 + x, self.y as i32 + y),
                (self.x as i32 + y, self.y as i32 + x),
                (self.x as i32 - y, self.y as i32 + x),
                (self.x as i32 - x, self.y as i32 + y),
                (self.x as i32 - x, self.y as i32 - y),
                (self.x as i32 - y, self.y as i32 - x),
                (self.x as i32 + y, self.y as i32 - x),
                (self.x as i32 + x, self.y as i32 - y),
            ];

            for (px, py) in points {
                if px >= 0 && py >= 0 {
                    grid.set(px as usize, py as usize, self.color);
                }
            }

            y += 1;
            err += 1 + 2 * y;
            if 2 * (err - x) + 1 > 0 {
                x -= 1;
                err += 1 - 2 * x;
            }
        }
    }
}

/// A filled circle
#[derive(Clone, Debug)]
pub struct FilledCircle {
    /// Center X coordinate
    pub x: f64,
    /// Center Y coordinate
    pub y: f64,
    /// Radius
    pub radius: f64,
    /// Fill color
    pub color: Color,
}

impl FilledCircle {
    /// Create a new filled circle
    pub fn new(x: f64, y: f64, radius: f64, color: Color) -> Self {
        Self {
            x,
            y,
            radius,
            color,
        }
    }
}

impl Shape for FilledCircle {
    fn draw(&self, grid: &mut BrailleGrid) {
        let r2 = self.radius * self.radius;
        let min_x = (self.x - self.radius).max(0.0) as usize;
        let max_x = (self.x + self.radius) as usize;
        let min_y = (self.y - self.radius).max(0.0) as usize;
        let max_y = (self.y + self.radius) as usize;

        for py in min_y..=max_y {
            for px in min_x..=max_x {
                let dx = px as f64 - self.x;
                let dy = py as f64 - self.y;
                if dx * dx + dy * dy <= r2 {
                    grid.set(px, py, self.color);
                }
            }
        }
    }
}

/// An arc (portion of a circle)
#[derive(Clone, Debug)]
pub struct Arc {
    /// Center X coordinate
    pub x: f64,
    /// Center Y coordinate
    pub y: f64,
    /// Radius
    pub radius: f64,
    /// Start angle in radians (0 = right, counter-clockwise)
    pub start_angle: f64,
    /// End angle in radians
    pub end_angle: f64,
    /// Arc color
    pub color: Color,
}

impl Arc {
    /// Create a new arc
    pub fn new(
        x: f64,
        y: f64,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
        color: Color,
    ) -> Self {
        Self {
            x,
            y,
            radius,
            start_angle,
            end_angle,
            color,
        }
    }

    /// Create an arc from degrees
    pub fn from_degrees(
        x: f64,
        y: f64,
        radius: f64,
        start_deg: f64,
        end_deg: f64,
        color: Color,
    ) -> Self {
        Self {
            x,
            y,
            radius,
            start_angle: start_deg.to_radians(),
            end_angle: end_deg.to_radians(),
            color,
        }
    }
}

impl Shape for Arc {
    fn draw(&self, grid: &mut BrailleGrid) {
        // Normalize angles
        let start = self.start_angle;
        let mut end = self.end_angle;

        // Ensure we draw in the correct direction
        while end < start {
            end += std::f64::consts::TAU;
        }

        // Calculate number of steps based on arc length
        let arc_length = self.radius * (end - start).abs();
        let steps = (arc_length * 2.0).max(20.0) as usize;

        let step_angle = (end - start) / steps as f64;

        for i in 0..=steps {
            let angle = start + step_angle * i as f64;
            let px = self.x + self.radius * angle.cos();
            let py = self.y + self.radius * angle.sin();

            if px >= 0.0 && py >= 0.0 {
                grid.set(px as usize, py as usize, self.color);
            }
        }
    }
}

/// A polygon (closed shape with multiple vertices)
#[derive(Clone, Debug)]
pub struct Polygon {
    /// Vertices as (x, y) coordinates
    pub vertices: Vec<(f64, f64)>,
    /// Polygon color
    pub color: Color,
}

impl Polygon {
    /// Create a new polygon
    pub fn new(vertices: Vec<(f64, f64)>, color: Color) -> Self {
        Self { vertices, color }
    }

    /// Create a regular polygon
    pub fn regular(x: f64, y: f64, radius: f64, sides: usize, color: Color) -> Self {
        let mut vertices = Vec::with_capacity(sides);
        let angle_step = std::f64::consts::TAU / sides as f64;

        for i in 0..sides {
            let angle = angle_step * i as f64 - std::f64::consts::FRAC_PI_2;
            vertices.push((x + radius * angle.cos(), y + radius * angle.sin()));
        }

        Self { vertices, color }
    }
}

impl Shape for Polygon {
    fn draw(&self, grid: &mut BrailleGrid) {
        if self.vertices.len() < 2 {
            return;
        }

        // Draw edges connecting consecutive vertices
        for i in 0..self.vertices.len() {
            let p0 = self.vertices[i];
            let p1 = self.vertices[(i + 1) % self.vertices.len()];
            Line::new(p0.0, p0.1, p1.0, p1.1, self.color).draw(grid);
        }
    }
}

/// A filled polygon
#[derive(Clone, Debug)]
pub struct FilledPolygon {
    /// Vertices as (x, y) coordinates
    pub vertices: Vec<(f64, f64)>,
    /// Fill color
    pub color: Color,
}

impl FilledPolygon {
    /// Create a new filled polygon
    pub fn new(vertices: Vec<(f64, f64)>, color: Color) -> Self {
        Self { vertices, color }
    }
}

impl Shape for FilledPolygon {
    fn draw(&self, grid: &mut BrailleGrid) {
        if self.vertices.len() < 3 {
            return;
        }

        // Find bounding box
        let min_x = self
            .vertices
            .iter()
            .map(|(x, _)| *x)
            .fold(f64::INFINITY, f64::min);
        let max_x = self
            .vertices
            .iter()
            .map(|(x, _)| *x)
            .fold(f64::NEG_INFINITY, f64::max);
        let min_y = self
            .vertices
            .iter()
            .map(|(_, y)| *y)
            .fold(f64::INFINITY, f64::min);
        let max_y = self
            .vertices
            .iter()
            .map(|(_, y)| *y)
            .fold(f64::NEG_INFINITY, f64::max);

        // Scanline fill using ray casting
        for py in min_y.max(0.0) as usize..=max_y as usize {
            for px in min_x.max(0.0) as usize..=max_x as usize {
                if self.point_in_polygon(px as f64, py as f64) {
                    grid.set(px, py, self.color);
                }
            }
        }
    }
}

impl FilledPolygon {
    /// Check if point is inside polygon using ray casting
    fn point_in_polygon(&self, x: f64, y: f64) -> bool {
        let mut inside = false;
        let n = self.vertices.len();

        let mut j = n - 1;
        for i in 0..n {
            let (xi, yi) = self.vertices[i];
            let (xj, yj) = self.vertices[j];

            if ((yi > y) != (yj > y)) && (x < (xj - xi) * (y - yi) / (yj - yi) + xi) {
                inside = !inside;
            }
            j = i;
        }

        inside
    }
}

/// A rectangle outline
#[derive(Clone, Debug)]
pub struct Rectangle {
    /// Top-left X coordinate
    pub x: f64,
    /// Top-left Y coordinate
    pub y: f64,
    /// Width
    pub width: f64,
    /// Height
    pub height: f64,
    /// Rectangle color
    pub color: Color,
}

impl Rectangle {
    /// Create a new rectangle
    pub fn new(x: f64, y: f64, width: f64, height: f64, color: Color) -> Self {
        Self {
            x,
            y,
            width,
            height,
            color,
        }
    }
}

impl Shape for Rectangle {
    fn draw(&self, grid: &mut BrailleGrid) {
        let x0 = self.x;
        let y0 = self.y;
        let x1 = self.x + self.width;
        let y1 = self.y + self.height;

        // Top edge
        Line::new(x0, y0, x1, y0, self.color).draw(grid);
        // Bottom edge
        Line::new(x0, y1, x1, y1, self.color).draw(grid);
        // Left edge
        Line::new(x0, y0, x0, y1, self.color).draw(grid);
        // Right edge
        Line::new(x1, y0, x1, y1, self.color).draw(grid);
    }
}

/// A filled rectangle
#[derive(Clone, Debug)]
pub struct FilledRectangle {
    /// Top-left X coordinate
    pub x: f64,
    /// Top-left Y coordinate
    pub y: f64,
    /// Width
    pub width: f64,
    /// Height
    pub height: f64,
    /// Fill color
    pub color: Color,
}

impl FilledRectangle {
    /// Create a new filled rectangle
    pub fn new(x: f64, y: f64, width: f64, height: f64, color: Color) -> Self {
        Self {
            x,
            y,
            width,
            height,
            color,
        }
    }
}

impl Shape for FilledRectangle {
    fn draw(&self, grid: &mut BrailleGrid) {
        let x0 = self.x.max(0.0) as usize;
        let y0 = self.y.max(0.0) as usize;
        let x1 = (self.x + self.width) as usize;
        let y1 = (self.y + self.height) as usize;

        for py in y0..=y1 {
            for px in x0..=x1 {
                grid.set(px, py, self.color);
            }
        }
    }
}

/// A series of connected points (polyline)
#[derive(Clone, Debug)]
pub struct Points {
    /// Points as (x, y) coordinates
    pub coords: Vec<(f64, f64)>,
    /// Line color
    pub color: Color,
}

impl Points {
    /// Create a new points series
    pub fn new(coords: Vec<(f64, f64)>, color: Color) -> Self {
        Self { coords, color }
    }

    /// Create from x and y slices
    pub fn from_slices(xs: &[f64], ys: &[f64], color: Color) -> Self {
        let coords = xs.iter().copied().zip(ys.iter().copied()).collect();
        Self { coords, color }
    }
}

impl Shape for Points {
    fn draw(&self, grid: &mut BrailleGrid) {
        // Draw connected lines between consecutive points
        for window in self.coords.windows(2) {
            if let [p0, p1] = window {
                Line::new(p0.0, p0.1, p1.0, p1.1, self.color).draw(grid);
            }
        }
    }
}

// =============================================================================
// Transformations
// =============================================================================

/// 2D transformation matrix for translate, scale, and rotate operations
#[derive(Clone, Copy, Debug)]
pub struct Transform {
    /// Scale X
    pub sx: f64,
    /// Shear X (for rotation)
    pub shx: f64,
    /// Translate X
    pub tx: f64,
    /// Shear Y (for rotation)
    pub shy: f64,
    /// Scale Y
    pub sy: f64,
    /// Translate Y
    pub ty: f64,
}

impl Default for Transform {
    fn default() -> Self {
        Self::identity()
    }
}

impl Transform {
    /// Create an identity transform (no transformation)
    pub fn identity() -> Self {
        Self {
            sx: 1.0,
            shx: 0.0,
            tx: 0.0,
            shy: 0.0,
            sy: 1.0,
            ty: 0.0,
        }
    }

    /// Create a translation transform
    pub fn translate(x: f64, y: f64) -> Self {
        Self {
            sx: 1.0,
            shx: 0.0,
            tx: x,
            shy: 0.0,
            sy: 1.0,
            ty: y,
        }
    }

    /// Create a scale transform
    pub fn scale(sx: f64, sy: f64) -> Self {
        Self {
            sx,
            shx: 0.0,
            tx: 0.0,
            shy: 0.0,
            sy,
            ty: 0.0,
        }
    }

    /// Create a uniform scale transform
    pub fn scale_uniform(s: f64) -> Self {
        Self::scale(s, s)
    }

    /// Create a rotation transform (angle in radians)
    pub fn rotate(angle: f64) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Self {
            sx: cos,
            shx: -sin,
            tx: 0.0,
            shy: sin,
            sy: cos,
            ty: 0.0,
        }
    }

    /// Create a rotation transform from degrees
    pub fn rotate_degrees(degrees: f64) -> Self {
        Self::rotate(degrees.to_radians())
    }

    /// Apply this transform to a point
    pub fn apply(&self, x: f64, y: f64) -> (f64, f64) {
        (
            self.sx * x + self.shx * y + self.tx,
            self.shy * x + self.sy * y + self.ty,
        )
    }

    /// Combine with another transform (self * other)
    pub fn then(&self, other: &Transform) -> Self {
        Self {
            sx: self.sx * other.sx + self.shx * other.shy,
            shx: self.sx * other.shx + self.shx * other.sy,
            tx: self.sx * other.tx + self.shx * other.ty + self.tx,
            shy: self.shy * other.sx + self.sy * other.shy,
            sy: self.shy * other.shx + self.sy * other.sy,
            ty: self.shy * other.tx + self.sy * other.ty + self.ty,
        }
    }

    /// Add a translation to this transform
    pub fn with_translate(self, x: f64, y: f64) -> Self {
        self.then(&Transform::translate(x, y))
    }

    /// Add a scale to this transform
    pub fn with_scale(self, sx: f64, sy: f64) -> Self {
        self.then(&Transform::scale(sx, sy))
    }

    /// Add a rotation to this transform
    pub fn with_rotate(self, angle: f64) -> Self {
        self.then(&Transform::rotate(angle))
    }
}

// =============================================================================
// Clipping Region
// =============================================================================

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

// =============================================================================
// Layer Support
// =============================================================================

/// A drawable layer that can be composed with other layers
///
/// Layers wrap a BrailleGrid and add visibility and opacity controls.
/// Multiple layers can be composited together for complex scenes.
pub struct Layer {
    /// The underlying grid
    grid: BrailleGrid,
    /// Visibility
    visible: bool,
    /// Opacity (0.0 - 1.0)
    opacity: f32,
}

impl Layer {
    /// Create a new layer
    pub fn new(term_width: u16, term_height: u16) -> Self {
        Self {
            grid: BrailleGrid::new(term_width, term_height),
            visible: true,
            opacity: 1.0,
        }
    }

    /// Set visibility
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    /// Check if visible
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Set opacity (0.0 - 1.0)
    pub fn set_opacity(&mut self, opacity: f32) {
        self.opacity = opacity.clamp(0.0, 1.0);
    }

    /// Get opacity
    pub fn opacity(&self) -> f32 {
        self.opacity
    }

    /// Get width in braille dots
    pub fn width(&self) -> usize {
        self.grid.width()
    }

    /// Get height in braille dots
    pub fn height(&self) -> usize {
        self.grid.height()
    }

    /// Set a dot
    pub fn set(&mut self, x: usize, y: usize, color: Color) {
        self.grid.set(x, y, color);
    }

    /// Clear the layer
    pub fn clear(&mut self) {
        self.grid.clear();
    }

    /// Draw a shape onto the layer
    pub fn draw<S: Shape>(&mut self, shape: &S) {
        self.grid.draw(shape);
    }

    /// Get the underlying grid for reading
    pub fn grid(&self) -> &BrailleGrid {
        &self.grid
    }

    /// Get the underlying grid for writing
    pub fn grid_mut(&mut self) -> &mut BrailleGrid {
        &mut self.grid
    }
}

// =============================================================================
// Braille Grid
// =============================================================================

/// A high-resolution grid using braille patterns
///
/// Each terminal cell represents a 2x4 dot matrix, giving 8x the resolution
/// in the vertical direction and 2x in the horizontal direction.
pub struct BrailleGrid {
    /// Width in braille dots (2x terminal width)
    width: usize,
    /// Height in braille dots (4x terminal height)
    height: usize,
    /// Dot patterns for each cell
    cells: Vec<u8>,
    /// Colors for each cell
    colors: Vec<Option<Color>>,
    /// Terminal width
    term_width: usize,
    /// Terminal height
    term_height: usize,
}

impl BrailleGrid {
    /// Create a new braille grid for the given terminal dimensions
    pub fn new(term_width: u16, term_height: u16) -> Self {
        let tw = term_width as usize;
        let th = term_height as usize;
        let cell_count = tw * th;

        Self {
            width: tw * 2,
            height: th * 4,
            cells: vec![0; cell_count],
            colors: vec![None; cell_count],
            term_width: tw,
            term_height: th,
        }
    }

    /// Get the width in braille dots
    pub fn width(&self) -> usize {
        self.width
    }

    /// Get the height in braille dots
    pub fn height(&self) -> usize {
        self.height
    }

    /// Set a dot at the given braille coordinates
    pub fn set(&mut self, x: usize, y: usize, color: Color) {
        if x >= self.width || y >= self.height {
            return;
        }

        let cell_x = x / 2;
        let cell_y = y / 4;
        let dot_x = x % 2;
        let dot_y = y % 4;

        let cell_idx = cell_y * self.term_width + cell_x;
        if cell_idx < self.cells.len() {
            self.cells[cell_idx] |= BRAILLE_DOTS[dot_x][dot_y];
            self.colors[cell_idx] = Some(color);
        }
    }

    /// Clear the grid
    pub fn clear(&mut self) {
        self.cells.fill(0);
        self.colors.fill(None);
    }

    /// Draw a shape onto the grid
    pub fn draw<S: Shape>(&mut self, shape: &S) {
        shape.draw(self);
    }

    /// Get the braille character for a cell
    fn get_char(&self, cell_x: usize, cell_y: usize) -> char {
        let idx = cell_y * self.term_width + cell_x;
        if idx < self.cells.len() {
            char::from_u32(BRAILLE_BASE + self.cells[idx] as u32).unwrap_or('⠀')
        } else {
            '⠀'
        }
    }

    /// Get the color for a cell
    fn get_color(&self, cell_x: usize, cell_y: usize) -> Option<Color> {
        let idx = cell_y * self.term_width + cell_x;
        if idx < self.colors.len() {
            self.colors[idx]
        } else {
            None
        }
    }

    /// Render the grid to the buffer
    pub fn render(&self, buffer: &mut crate::render::Buffer, area: Rect) {
        for cy in 0..self.term_height.min(area.height as usize) {
            for cx in 0..self.term_width.min(area.width as usize) {
                let ch = self.get_char(cx, cy);
                if ch != '⠀' {
                    let mut cell = Cell::new(ch);
                    cell.fg = self.get_color(cx, cy);
                    buffer.set(area.x + cx as u16, area.y + cy as u16, cell);
                }
            }
        }
    }

    /// Composite a layer onto this grid
    ///
    /// The layer's dots are OR'd with existing dots, and colors are overwritten.
    pub fn composite_layer(&mut self, layer: &Layer) {
        if !layer.is_visible() || layer.opacity() <= 0.0 {
            return;
        }

        let layer_grid = layer.grid();
        let max_cells = self.cells.len().min(layer_grid.cells.len());

        for idx in 0..max_cells {
            let pattern = layer_grid.cells[idx];
            if pattern != 0 {
                self.cells[idx] |= pattern;
                if let Some(color) = layer_grid.colors[idx] {
                    self.colors[idx] = Some(color);
                }
            }
        }
    }
}

// =============================================================================
// Braille Canvas Context
// =============================================================================

/// A context for high-resolution braille drawing
pub struct BrailleContext<'a> {
    grid: &'a mut BrailleGrid,
}

impl<'a> BrailleContext<'a> {
    /// Get the width in braille dots
    pub fn width(&self) -> usize {
        self.grid.width()
    }

    /// Get the height in braille dots
    pub fn height(&self) -> usize {
        self.grid.height()
    }

    /// Set a single dot
    pub fn set(&mut self, x: usize, y: usize, color: Color) {
        self.grid.set(x, y, color);
    }

    /// Draw a shape
    pub fn draw<S: Shape>(&mut self, shape: &S) {
        self.grid.draw(shape);
    }

    /// Draw a line
    pub fn line(&mut self, x0: f64, y0: f64, x1: f64, y1: f64, color: Color) {
        self.draw(&Line::new(x0, y0, x1, y1, color));
    }

    /// Draw a circle
    pub fn circle(&mut self, x: f64, y: f64, radius: f64, color: Color) {
        self.draw(&Circle::new(x, y, radius, color));
    }

    /// Draw a filled circle
    pub fn filled_circle(&mut self, x: f64, y: f64, radius: f64, color: Color) {
        self.draw(&FilledCircle::new(x, y, radius, color));
    }

    /// Draw a rectangle
    pub fn rect(&mut self, x: f64, y: f64, width: f64, height: f64, color: Color) {
        self.draw(&Rectangle::new(x, y, width, height, color));
    }

    /// Draw a filled rectangle
    pub fn filled_rect(&mut self, x: f64, y: f64, width: f64, height: f64, color: Color) {
        self.draw(&FilledRectangle::new(x, y, width, height, color));
    }

    /// Draw connected points
    pub fn points(&mut self, coords: Vec<(f64, f64)>, color: Color) {
        self.draw(&Points::new(coords, color));
    }

    /// Draw an arc
    pub fn arc(
        &mut self,
        x: f64,
        y: f64,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
        color: Color,
    ) {
        self.draw(&Arc::new(x, y, radius, start_angle, end_angle, color));
    }

    /// Draw an arc using degrees
    pub fn arc_degrees(
        &mut self,
        x: f64,
        y: f64,
        radius: f64,
        start_deg: f64,
        end_deg: f64,
        color: Color,
    ) {
        self.draw(&Arc::from_degrees(x, y, radius, start_deg, end_deg, color));
    }

    /// Draw a polygon
    pub fn polygon(&mut self, vertices: Vec<(f64, f64)>, color: Color) {
        self.draw(&Polygon::new(vertices, color));
    }

    /// Draw a regular polygon
    pub fn regular_polygon(&mut self, x: f64, y: f64, radius: f64, sides: usize, color: Color) {
        self.draw(&Polygon::regular(x, y, radius, sides, color));
    }

    /// Draw a filled polygon
    pub fn filled_polygon(&mut self, vertices: Vec<(f64, f64)>, color: Color) {
        self.draw(&FilledPolygon::new(vertices, color));
    }

    /// Clear the canvas
    pub fn clear(&mut self) {
        self.grid.clear();
    }
}

// =============================================================================
// Standard Draw Context (Character-based)
// =============================================================================

/// A context for drawing on a canvas (character-based)
pub struct DrawContext<'a> {
    buffer: &'a mut crate::render::Buffer,
    area: Rect,
}

impl<'a> DrawContext<'a> {
    /// Get canvas width
    pub fn width(&self) -> u16 {
        self.area.width
    }

    /// Get canvas height
    pub fn height(&self) -> u16 {
        self.area.height
    }

    /// Get canvas area
    pub fn area(&self) -> Rect {
        self.area
    }

    /// Set a character at position
    pub fn set(&mut self, x: u16, y: u16, ch: char) {
        let abs_x = self.area.x + x;
        let abs_y = self.area.y + y;
        if x < self.area.width && y < self.area.height {
            self.buffer.set(abs_x, abs_y, Cell::new(ch));
        }
    }

    /// Set a character with style at position
    pub fn set_styled(&mut self, x: u16, y: u16, ch: char, fg: Option<Color>, bg: Option<Color>) {
        let abs_x = self.area.x + x;
        let abs_y = self.area.y + y;
        if x < self.area.width && y < self.area.height {
            let mut cell = Cell::new(ch);
            cell.fg = fg;
            cell.bg = bg;
            self.buffer.set(abs_x, abs_y, cell);
        }
    }

    /// Set a cell at position
    pub fn set_cell(&mut self, x: u16, y: u16, cell: Cell) {
        let abs_x = self.area.x + x;
        let abs_y = self.area.y + y;
        if x < self.area.width && y < self.area.height {
            self.buffer.set(abs_x, abs_y, cell);
        }
    }

    /// Draw a horizontal line
    pub fn hline(&mut self, x: u16, y: u16, length: u16, ch: char, fg: Option<Color>) {
        for i in 0..length {
            if x + i < self.area.width {
                self.set_styled(x + i, y, ch, fg, None);
            }
        }
    }

    /// Draw a vertical line
    pub fn vline(&mut self, x: u16, y: u16, length: u16, ch: char, fg: Option<Color>) {
        for i in 0..length {
            if y + i < self.area.height {
                self.set_styled(x, y + i, ch, fg, None);
            }
        }
    }

    /// Draw a rectangle outline
    pub fn rect(&mut self, x: u16, y: u16, width: u16, height: u16, fg: Option<Color>) {
        if width == 0 || height == 0 {
            return;
        }

        // Top and bottom
        self.hline(x, y, width, '─', fg);
        self.hline(x, y + height - 1, width, '─', fg);

        // Left and right
        self.vline(x, y, height, '│', fg);
        self.vline(x + width - 1, y, height, '│', fg);

        // Corners
        self.set_styled(x, y, '┌', fg, None);
        self.set_styled(x + width - 1, y, '┐', fg, None);
        self.set_styled(x, y + height - 1, '└', fg, None);
        self.set_styled(x + width - 1, y + height - 1, '┘', fg, None);
    }

    /// Fill a rectangle
    pub fn fill_rect(&mut self, rect: Rect, ch: char, fg: Option<Color>, bg: Option<Color>) {
        for dy in 0..rect.height {
            for dx in 0..rect.width {
                if rect.x + dx < self.area.width && rect.y + dy < self.area.height {
                    self.set_styled(rect.x + dx, rect.y + dy, ch, fg, bg);
                }
            }
        }
    }

    /// Draw a filled bar (for Gantt charts, progress bars, etc.)
    pub fn bar(&mut self, x: u16, y: u16, width: u16, fg: Color, bg: Option<Color>) {
        for i in 0..width {
            if x + i < self.area.width {
                let mut cell = Cell::new('█');
                cell.fg = Some(fg);
                cell.bg = bg;
                self.set_cell(x + i, y, cell);
            }
        }
    }

    /// Draw a partial bar (for fractional values)
    pub fn partial_bar(&mut self, x: u16, y: u16, width: f32, fg: Color) {
        let full_blocks = width.floor() as u16;
        let partial = width.fract();

        // Full blocks
        self.bar(x, y, full_blocks, fg, None);

        // Partial block
        if partial > 0.0 && x + full_blocks < self.area.width {
            let partial_char = match (partial * 8.0).round() as u8 {
                1 => '▏',
                2 => '▎',
                3 => '▍',
                4 => '▌',
                5 => '▋',
                6 => '▊',
                7 => '▉',
                _ => ' ',
            };
            self.set_styled(x + full_blocks, y, partial_char, Some(fg), None);
        }
    }

    /// Draw text at position
    pub fn text(&mut self, x: u16, y: u16, s: &str, fg: Option<Color>) {
        for (i, ch) in s.chars().enumerate() {
            let pos_x = x + i as u16;
            if pos_x < self.area.width {
                self.set_styled(pos_x, y, ch, fg, None);
            }
        }
    }

    /// Draw bold text at position
    pub fn text_bold(&mut self, x: u16, y: u16, s: &str, fg: Option<Color>) {
        for (i, ch) in s.chars().enumerate() {
            let pos_x = x + i as u16;
            if pos_x < self.area.width {
                let abs_x = self.area.x + pos_x;
                let abs_y = self.area.y + y;
                let mut cell = Cell::new(ch);
                cell.fg = fg;
                cell.modifier = Modifier::BOLD;
                self.buffer.set(abs_x, abs_y, cell);
            }
        }
    }

    /// Clear the canvas
    pub fn clear(&mut self) {
        self.fill_rect(
            Rect::new(0, 0, self.area.width, self.area.height),
            ' ',
            None,
            None,
        );
    }

    /// Draw a point/dot
    pub fn point(&mut self, x: u16, y: u16, fg: Color) {
        self.set_styled(x, y, '●', Some(fg), None);
    }

    /// Draw a line between two points (Bresenham's algorithm)
    pub fn line(&mut self, x0: u16, y0: u16, x1: u16, y1: u16, ch: char, fg: Option<Color>) {
        let dx = (x1 as i32 - x0 as i32).abs();
        let dy = -(y1 as i32 - y0 as i32).abs();
        let sx: i32 = if x0 < x1 { 1 } else { -1 };
        let sy: i32 = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        let mut x = x0 as i32;
        let mut y = y0 as i32;

        loop {
            if x >= 0 && y >= 0 {
                self.set_styled(x as u16, y as u16, ch, fg, None);
            }

            if x == x1 as i32 && y == y1 as i32 {
                break;
            }

            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }
}

// =============================================================================
// Canvas Widgets
// =============================================================================

/// A canvas widget for custom drawing (character-based)
pub struct Canvas<F>
where
    F: Fn(&mut DrawContext),
{
    draw_fn: F,
}

impl<F> Canvas<F>
where
    F: Fn(&mut DrawContext),
{
    /// Create a new canvas with a drawing function
    pub fn new(draw_fn: F) -> Self {
        Self { draw_fn }
    }
}

impl<F> View for Canvas<F>
where
    F: Fn(&mut DrawContext),
{
    fn render(&self, ctx: &mut RenderContext) {
        let mut draw_ctx = DrawContext {
            buffer: ctx.buffer,
            area: ctx.area,
        };
        (self.draw_fn)(&mut draw_ctx);
    }
}

/// Create a canvas with a drawing function
pub fn canvas<F>(draw_fn: F) -> Canvas<F>
where
    F: Fn(&mut DrawContext),
{
    Canvas::new(draw_fn)
}

/// A high-resolution canvas using braille patterns
///
/// Each terminal cell represents a 2x4 dot matrix, providing
/// 2x horizontal and 4x vertical resolution.
///
/// # Example
///
/// ```rust,ignore
/// use revue::prelude::*;
///
/// let chart = BrailleCanvas::new(|ctx| {
///     // Draw a sine wave
///     let points: Vec<(f64, f64)> = (0..ctx.width())
///         .map(|x| {
///             let y = (x as f64 * 0.1).sin() * 10.0 + 20.0;
///             (x as f64, y)
///         })
///         .collect();
///     ctx.points(points, Color::CYAN);
///
///     // Draw a circle
///     ctx.circle(40.0, 20.0, 15.0, Color::YELLOW);
/// });
/// ```
pub struct BrailleCanvas<F>
where
    F: Fn(&mut BrailleContext),
{
    draw_fn: F,
}

impl<F> BrailleCanvas<F>
where
    F: Fn(&mut BrailleContext),
{
    /// Create a new braille canvas with a drawing function
    pub fn new(draw_fn: F) -> Self {
        Self { draw_fn }
    }
}

impl<F> View for BrailleCanvas<F>
where
    F: Fn(&mut BrailleContext),
{
    fn render(&self, ctx: &mut RenderContext) {
        let mut grid = BrailleGrid::new(ctx.area.width, ctx.area.height);
        let mut braille_ctx = BrailleContext { grid: &mut grid };
        (self.draw_fn)(&mut braille_ctx);
        grid.render(ctx.buffer, ctx.area);
    }
}

/// Create a braille canvas with a drawing function
pub fn braille_canvas<F>(draw_fn: F) -> BrailleCanvas<F>
where
    F: Fn(&mut BrailleContext),
{
    BrailleCanvas::new(draw_fn)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::Buffer;

    #[test]
    fn test_canvas_new() {
        let c = Canvas::new(|_ctx| {});
        let _ = c;
    }

    #[test]
    fn test_draw_context_dimensions() {
        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(5, 5, 30, 10);
        let ctx = DrawContext {
            buffer: &mut buffer,
            area,
        };

        assert_eq!(ctx.width(), 30);
        assert_eq!(ctx.height(), 10);
    }

    #[test]
    fn test_draw_context_set() {
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = DrawContext {
            buffer: &mut buffer,
            area,
        };

        ctx.set(5, 5, 'X');
    }

    #[test]
    fn test_draw_context_hline() {
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = DrawContext {
            buffer: &mut buffer,
            area,
        };

        ctx.hline(2, 5, 10, '-', Some(Color::WHITE));
    }

    #[test]
    fn test_draw_context_vline() {
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = DrawContext {
            buffer: &mut buffer,
            area,
        };

        ctx.vline(5, 2, 6, '|', Some(Color::WHITE));
    }

    #[test]
    fn test_draw_context_rect() {
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = DrawContext {
            buffer: &mut buffer,
            area,
        };

        ctx.rect(2, 2, 10, 5, Some(Color::CYAN));
    }

    #[test]
    fn test_draw_context_fill_rect() {
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = DrawContext {
            buffer: &mut buffer,
            area,
        };

        ctx.fill_rect(
            Rect::new(3, 3, 5, 3),
            '#',
            Some(Color::RED),
            Some(Color::BLACK),
        );
    }

    #[test]
    fn test_draw_context_bar() {
        let mut buffer = Buffer::new(30, 5);
        let area = Rect::new(0, 0, 30, 5);
        let mut ctx = DrawContext {
            buffer: &mut buffer,
            area,
        };

        ctx.bar(5, 2, 15, Color::GREEN, None);
    }

    #[test]
    fn test_draw_context_text() {
        let mut buffer = Buffer::new(30, 5);
        let area = Rect::new(0, 0, 30, 5);
        let mut ctx = DrawContext {
            buffer: &mut buffer,
            area,
        };

        ctx.text(5, 2, "Hello World", Some(Color::WHITE));
    }

    #[test]
    fn test_draw_context_line() {
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = DrawContext {
            buffer: &mut buffer,
            area,
        };

        ctx.line(0, 0, 19, 9, '*', Some(Color::YELLOW));
    }

    #[test]
    fn test_canvas_render() {
        let c = canvas(|ctx| {
            ctx.bar(0, 0, 10, Color::BLUE, None);
            ctx.text(0, 1, "Test", Some(Color::WHITE));
        });

        let mut buffer = Buffer::new(20, 5);
        let area = Rect::new(0, 0, 20, 5);
        let mut render_ctx = RenderContext::new(&mut buffer, area);

        c.render(&mut render_ctx);
    }

    #[test]
    fn test_canvas_helper() {
        let c = canvas(|ctx| {
            ctx.point(5, 5, Color::RED);
        });
        let _ = c;
    }

    #[test]
    fn test_partial_bar() {
        let mut buffer = Buffer::new(20, 5);
        let area = Rect::new(0, 0, 20, 5);
        let mut ctx = DrawContext {
            buffer: &mut buffer,
            area,
        };

        ctx.partial_bar(0, 0, 5.5, Color::GREEN);
    }

    // Braille tests

    #[test]
    fn test_braille_grid_new() {
        let grid = BrailleGrid::new(40, 20);
        assert_eq!(grid.width(), 80); // 40 * 2
        assert_eq!(grid.height(), 80); // 20 * 4
    }

    #[test]
    fn test_braille_grid_set() {
        let mut grid = BrailleGrid::new(10, 10);
        grid.set(0, 0, Color::RED);
        grid.set(1, 0, Color::RED);

        // Cell (0,0) should have dots at (0,0) and (1,0)
        assert_eq!(grid.cells[0], 0x01 | 0x08);
    }

    #[test]
    fn test_braille_grid_get_char() {
        let mut grid = BrailleGrid::new(10, 10);

        // Set all 8 dots in the first cell
        for x in 0..2 {
            for y in 0..4 {
                grid.set(x, y, Color::WHITE);
            }
        }

        let ch = grid.get_char(0, 0);
        assert_eq!(ch, '⣿'); // Full braille character
    }

    #[test]
    fn test_braille_line() {
        let mut grid = BrailleGrid::new(20, 10);
        grid.draw(&Line::new(0.0, 0.0, 39.0, 39.0, Color::CYAN));
        // Line should be drawn
    }

    #[test]
    fn test_braille_circle() {
        let mut grid = BrailleGrid::new(20, 10);
        grid.draw(&Circle::new(20.0, 20.0, 10.0, Color::YELLOW));
        // Circle should be drawn
    }

    #[test]
    fn test_braille_filled_circle() {
        let mut grid = BrailleGrid::new(20, 10);
        grid.draw(&FilledCircle::new(20.0, 20.0, 10.0, Color::GREEN));
        // Filled circle should be drawn
    }

    #[test]
    fn test_braille_rectangle() {
        let mut grid = BrailleGrid::new(20, 10);
        grid.draw(&Rectangle::new(5.0, 5.0, 20.0, 15.0, Color::RED));
        // Rectangle should be drawn
    }

    #[test]
    fn test_braille_filled_rectangle() {
        let mut grid = BrailleGrid::new(20, 10);
        grid.draw(&FilledRectangle::new(5.0, 5.0, 20.0, 15.0, Color::BLUE));
        // Filled rectangle should be drawn
    }

    #[test]
    fn test_braille_points() {
        let mut grid = BrailleGrid::new(40, 20);
        let coords: Vec<(f64, f64)> = (0..80)
            .map(|x| {
                let y = (x as f64 * 0.1).sin() * 30.0 + 40.0;
                (x as f64, y)
            })
            .collect();
        grid.draw(&Points::new(coords, Color::MAGENTA));
        // Points should be drawn
    }

    #[test]
    fn test_braille_canvas_widget() {
        let bc = braille_canvas(|ctx| {
            ctx.line(0.0, 0.0, 20.0, 40.0, Color::WHITE);
            ctx.circle(30.0, 30.0, 10.0, Color::CYAN);
        });

        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut render_ctx = RenderContext::new(&mut buffer, area);

        bc.render(&mut render_ctx);
    }

    #[test]
    fn test_braille_context_methods() {
        let bc = braille_canvas(|ctx| {
            ctx.line(0.0, 0.0, 10.0, 10.0, Color::WHITE);
            ctx.circle(20.0, 20.0, 5.0, Color::RED);
            ctx.filled_circle(30.0, 20.0, 5.0, Color::GREEN);
            ctx.rect(40.0, 10.0, 10.0, 10.0, Color::BLUE);
            ctx.filled_rect(55.0, 10.0, 10.0, 10.0, Color::YELLOW);
            ctx.set(0, 0, Color::MAGENTA);
        });

        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut render_ctx = RenderContext::new(&mut buffer, area);

        bc.render(&mut render_ctx);
    }
}
