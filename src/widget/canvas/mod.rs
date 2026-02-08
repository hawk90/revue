//! Canvas widget for custom drawing
//!
//! Provides a drawing surface for rendering custom graphics like
//! Gantt charts, diagrams, graphs, etc.
//!
//! Supports two rendering modes:
//! - **Character mode**: Standard character-based drawing
//! - **Braille mode**: High-resolution drawing using braille patterns (2x4 dots per cell)

mod braille;
mod clip;
mod draw;
mod grid;
mod layer;
#[cfg(test)]
mod tests {
    use super::*;

    use crate::layout::Rect;
    use crate::prelude::RenderContext;
    use crate::render::Buffer;
    use crate::style::Color;
    use crate::widget::canvas::{BrailleGrid, DrawContext};
    use crate::widget::traits::View;

    // Import braille shapes using the re-exported path
    use crate::widget::canvas::{Circle, FilledCircle, FilledRectangle, Line, Rectangle};

    // Standard canvas tests

    #[test]
    fn test_canvas_new() {
        let c = Canvas::new(|_ctx| {});
        let _ = c;
    }

    #[test]
    fn test_draw_context_dimensions() {
        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(5, 5, 30, 10);
        let ctx = DrawContext::new(&mut buffer, area);

        assert_eq!(ctx.width(), 30);
        assert_eq!(ctx.height(), 10);
    }

    #[test]
    fn test_draw_context_set() {
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = DrawContext::new(&mut buffer, area);

        ctx.set(5, 5, 'X');
    }

    #[test]
    fn test_draw_context_hline() {
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = DrawContext::new(&mut buffer, area);

        ctx.hline(2, 5, 10, '-', Some(Color::WHITE));
    }

    #[test]
    fn test_draw_context_vline() {
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = DrawContext::new(&mut buffer, area);

        ctx.vline(5, 2, 6, '|', Some(Color::WHITE));
    }

    #[test]
    fn test_draw_context_rect() {
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = DrawContext::new(&mut buffer, area);

        ctx.rect(2, 2, 10, 5, Some(Color::CYAN));
    }

    #[test]
    fn test_draw_context_fill_rect() {
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = DrawContext::new(&mut buffer, area);

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
        let mut ctx = DrawContext::new(&mut buffer, area);

        ctx.bar(5, 2, 15, Color::GREEN, None);
    }

    #[test]
    fn test_draw_context_text() {
        let mut buffer = Buffer::new(30, 5);
        let area = Rect::new(0, 0, 30, 5);
        let mut ctx = DrawContext::new(&mut buffer, area);

        ctx.text(5, 2, "Hello World", Some(Color::WHITE));
    }

    #[test]
    fn test_draw_context_line() {
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = DrawContext::new(&mut buffer, area);

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
        let mut ctx = DrawContext::new(&mut buffer, area);

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
        assert_eq!(grid.cells()[0], 0x01 | 0x08);
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

    // Integration and edge case tests

    #[test]
    fn test_canvas_multiple_draw_operations() {
        use crate::style::Color;

        let c = canvas(|ctx| {
            ctx.hline(0, 0, 10, '-', Some(Color::WHITE));
            ctx.vline(0, 0, 5, '|', Some(Color::WHITE));
            ctx.rect(5, 2, 8, 4, Some(Color::CYAN));
            ctx.fill_rect(Rect::new(6, 3, 6, 2), '#', Some(Color::RED), None);
            ctx.text(0, 8, "Test", Some(Color::YELLOW));
        });

        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut render_ctx = RenderContext::new(&mut buffer, area);

        c.render(&mut render_ctx);
    }

    #[test]
    fn test_canvas_clear_and_redraw() {
        use crate::style::Color;

        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);

        // First draw
        let c1 = canvas(|ctx| {
            ctx.text(0, 0, "First", Some(Color::WHITE));
        });
        let mut render_ctx = RenderContext::new(&mut buffer, area);
        c1.render(&mut render_ctx);

        // Clear and redraw
        let c2 = canvas(|ctx| {
            ctx.clear();
            ctx.text(0, 0, "Second", Some(Color::WHITE));
        });
        let mut render_ctx2 = RenderContext::new(&mut buffer, area);
        c2.render(&mut render_ctx2);
    }

    #[test]
    fn test_canvas_edge_clipping() {
        use crate::style::Color;

        let c = canvas(|ctx| {
            // Draw at the edge of the canvas
            ctx.hline(0, 0, 20, '-', Some(Color::WHITE));
            ctx.vline(19, 0, 10, '|', Some(Color::WHITE));
            ctx.rect(0, 0, 20, 10, Some(Color::CYAN));
        });

        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut render_ctx = RenderContext::new(&mut buffer, area);

        c.render(&mut render_ctx);
    }

    #[test]
    fn test_braille_canvas_complex_scene() {
        use crate::style::Color;

        let bc = braille_canvas(|ctx| {
            // Draw a complex scene with multiple shapes
            ctx.rect(5.0, 5.0, 30.0, 20.0, Color::WHITE);
            ctx.circle(20.0, 15.0, 8.0, Color::CYAN);
            ctx.filled_circle(40.0, 15.0, 5.0, Color::BLUE);
            ctx.line(0.0, 30.0, 50.0, 30.0, Color::YELLOW);
            ctx.filled_rect(10.0, 35.0, 20.0, 5.0, Color::GREEN);
        });

        let mut buffer = Buffer::new(30, 15);
        let area = Rect::new(0, 0, 30, 15);
        let mut render_ctx = RenderContext::new(&mut buffer, area);

        bc.render(&mut render_ctx);
    }

    #[test]
    fn test_braille_canvas_precision() {
        use crate::style::Color;

        let bc = braille_canvas(|ctx| {
            // Test high-resolution drawing
            for i in 0..10 {
                ctx.set(i * 2, i, Color::WHITE);
            }
        });

        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut render_ctx = RenderContext::new(&mut buffer, area);

        bc.render(&mut render_ctx);
    }

    #[test]
    fn test_canvas_with_all_colors() {
        use crate::style::Color;

        let colors = vec![
            Color::WHITE,
            Color::BLACK,
            Color::RED,
            Color::GREEN,
            Color::BLUE,
            Color::YELLOW,
            Color::CYAN,
            Color::MAGENTA,
        ];

        let c = canvas(|ctx| {
            for (i, color) in colors.iter().enumerate() {
                ctx.set(i as u16, 0, '█');
            }
        });

        let mut buffer = Buffer::new(20, 5);
        let area = Rect::new(0, 0, 20, 5);
        let mut render_ctx = RenderContext::new(&mut buffer, area);

        c.render(&mut render_ctx);
    }

    #[test]
    fn test_canvas_line_diagonal_all_quadrants() {
        use crate::style::Color;

        let c = canvas(|ctx| {
            // Draw lines in all directions
            ctx.line(0, 0, 10, 10, '*', Some(Color::WHITE)); // bottom-right
            ctx.line(19, 0, 9, 10, '*', Some(Color::WHITE)); // bottom-left
            ctx.line(0, 9, 10, 0, '*', Some(Color::WHITE)); // top-right
            ctx.line(19, 9, 9, 0, '*', Some(Color::WHITE)); // top-left
        });

        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut render_ctx = RenderContext::new(&mut buffer, area);

        c.render(&mut render_ctx);
    }

    #[test]
    fn test_braille_canvas_overlap_shapes() {
        use crate::style::Color;

        let bc = braille_canvas(|ctx| {
            // Test overlapping shapes
            ctx.circle(20.0, 20.0, 15.0, Color::RED);
            ctx.circle(25.0, 20.0, 15.0, Color::BLUE);
            ctx.circle(22.5, 15.0, 15.0, Color::GREEN);
        });

        let mut buffer = Buffer::new(30, 15);
        let area = Rect::new(0, 0, 30, 15);
        let mut render_ctx = RenderContext::new(&mut buffer, area);

        bc.render(&mut render_ctx);
    }
}
mod transform;
mod widget;

pub use braille::{
    Arc, BrailleGrid, Circle, FilledCircle, FilledPolygon, FilledRectangle, Line, Points, Polygon,
    Rectangle, Shape,
};
pub use clip::ClipRegion;
pub use draw::DrawContext;
pub use layer::Layer;
pub use transform::Transform;
pub use widget::{braille_canvas, canvas, BrailleCanvas, Canvas};

// Re-export braille context at the top level
pub use braille::BrailleContext;
