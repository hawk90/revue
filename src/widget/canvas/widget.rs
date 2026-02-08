//! Canvas widget implementations

use super::braille::{BrailleContext, BrailleGrid};
use super::draw::DrawContext;
use crate::widget::traits::{RenderContext, View};

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
        let mut draw_ctx = DrawContext::new(ctx.buffer, ctx.area);
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
        let mut braille_ctx = BrailleContext::new(&mut grid);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::Buffer;

    // =========================================================================
    // Canvas tests
    // =========================================================================

    #[test]
    fn test_canvas_new() {
        let canvas = Canvas::new(|_ctx| {
            // Drawing function
        });
        // Just verify it doesn't panic
    }

    #[test]
    fn test_canvas_with_draw_fn() {
        let canvas = Canvas::new(|ctx| {
            ctx.text(0, 0, "test", None);
        });
        // Just verify it creates successfully
    }

    // =========================================================================
    // canvas helper function tests
    // =========================================================================

    #[test]
    fn test_canvas_helper() {
        let canvas = canvas(|_ctx| {
            // Drawing function
        });
        // Just verify it doesn't panic
    }

    #[test]
    fn test_canvas_helper_with_draw_fn() {
        let canvas = canvas(|ctx| {
            ctx.set(5, 5, 'X');
        });
        // Just verify it creates successfully
    }

    // =========================================================================
    // BrailleCanvas tests
    // =========================================================================

    #[test]
    fn test_braille_canvas_new() {
        let canvas = BrailleCanvas::new(|_ctx| {
            // Drawing function
        });
        // Just verify it doesn't panic
    }

    #[test]
    fn test_braille_canvas_with_draw_fn() {
        let canvas = BrailleCanvas::new(|_ctx| {
            // Drawing function would go here
        });
        // Just verify it creates successfully
    }

    // =========================================================================
    // braille_canvas helper function tests
    // =========================================================================

    #[test]
    fn test_braille_canvas_helper() {
        let canvas = braille_canvas(|_ctx| {
            // Drawing function
        });
        // Just verify it doesn't panic
    }

    #[test]
    fn test_braille_canvas_helper_with_draw_fn() {
        let canvas = braille_canvas(|_ctx| {
            // Drawing function would go here
        });
        // Just verify it creates successfully
    }

    // =========================================================================
    // Closure capture tests
    // =========================================================================

    #[test]
    fn test_canvas_closure_capture() {
        let text = "captured";
        let canvas = Canvas::new(move |ctx| {
            ctx.text(0, 0, text, None);
        });
        // Just verify closure captures work
    }

    #[test]
    fn test_braille_canvas_closure_capture() {
        let value = 42;
        let canvas = BrailleCanvas::new(move |_ctx| {
            let _ = value;
        });
        // Just verify closure captures work
    }
}
