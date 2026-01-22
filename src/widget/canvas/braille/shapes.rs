//! Shape types for braille canvas drawing

use super::super::grid::Grid;
use crate::style::Color;

/// A shape that can be drawn on a braille canvas
pub trait Shape {
    /// Draw the shape onto the braille grid
    fn draw(&self, grid: &mut dyn Grid);
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
    fn draw(&self, grid: &mut dyn Grid) {
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
    fn draw(&self, grid: &mut dyn Grid) {
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
    fn draw(&self, grid: &mut dyn Grid) {
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
    fn draw(&self, grid: &mut dyn Grid) {
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
    fn draw(&self, grid: &mut dyn Grid) {
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
    fn draw(&self, grid: &mut dyn Grid) {
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
    fn draw(&self, grid: &mut dyn Grid) {
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
    fn draw(&self, grid: &mut dyn Grid) {
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
    fn draw(&self, grid: &mut dyn Grid) {
        // Draw connected lines between consecutive points
        for window in self.coords.windows(2) {
            if let [p0, p1] = window {
                Line::new(p0.0, p0.1, p1.0, p1.1, self.color).draw(grid);
            }
        }
    }
}
