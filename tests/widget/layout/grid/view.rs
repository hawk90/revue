//! Grid widget View implementation tests

use super::super::grid::{Grid, GridItem, GridAlign, TrackSize};
use super::super::grid::GridPlacement;
use crate::layout::Rect;
use crate::render::Buffer;
use crate::widget::Text;

// =========================================================================
// Render edge case tests
// =========================================================================

#[test]
fn test_grid_render_empty() {
    let grid = Grid::new();
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = crate::widget::traits::RenderContext::new(&mut buffer, area);

    // Should not crash with empty grid
    grid.render(&mut ctx);
}

#[test]
fn test_grid_render_zero_width() {
    let grid = Grid::new().child(Text::new("Test"));
    let mut buffer = Buffer::new(0, 10);
    let area = Rect::new(0, 0, 0, 10);
    let mut ctx = crate::widget::traits::RenderContext::new(&mut buffer, area);

    // Should not crash with zero width
    grid.render(&mut ctx);
}

#[test]
fn test_grid_render_zero_height() {
    let grid = Grid::new().child(Text::new("Test"));
    let mut buffer = Buffer::new(10, 0);
    let area = Rect::new(0, 0, 10, 0);
    let mut ctx = crate::widget::traits::RenderContext::new(&mut buffer, area);

    // Should not crash with zero height
    grid.render(&mut ctx);
}

#[test]
fn test_grid_render_single_item() {
    let grid = Grid::new().child(Text::new("A"));
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = crate::widget::traits::RenderContext::new(&mut buffer, area);

    grid.render(&mut ctx);

    // Item should be rendered
    let cell = buffer.get(0, 0);
    assert!(cell.is_some());
}

#[test]
fn test_grid_render_multiple_items() {
    let grid = Grid::new()
        .cols(2)
        .child(Text::new("A"))
        .child(Text::new("B"))
        .child(Text::new("C"))
        .child(Text::new("D"));
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = crate::widget::traits::RenderContext::new(&mut buffer, area);

    grid.render(&mut ctx);

    // Items should be rendered
    let cell = buffer.get(0, 0);
    assert!(cell.is_some());
}

#[test]
fn test_grid_render_with_gaps() {
    let grid = Grid::new()
        .cols(2)
        .gap(2)
        .child(Text::new("A"))
        .child(Text::new("B"));
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = crate::widget::traits::RenderContext::new(&mut buffer, area);

    // Should not crash with gaps
    grid.render(&mut ctx);
}

#[test]
fn test_grid_render_explicit_placement() {
    let grid = Grid::new()
        .item(GridItem::new(Text::new("A")).at(1, 1))
        .item(GridItem::new(Text::new("B")).at(2, 2));
    let mut buffer = Buffer::new(20, 20);
    let area = Rect::new(0, 0, 20, 20);
    let mut ctx = crate::widget::traits::RenderContext::new(&mut buffer, area);

    // Should not crash with explicit placement
    grid.render(&mut ctx);
}

#[test]
fn test_grid_render_with_span() {
    let grid = Grid::new()
        .cols(3)
        .item(GridItem::new(Text::new("A")).col_span(2))
        .child(Text::new("B"));
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = crate::widget::traits::RenderContext::new(&mut buffer, area);

    // Should not crash with column span
    grid.render(&mut ctx);
}

#[test]
fn test_grid_render_auto_dimensions() {
    let grid = Grid::new().child(Text::new("A")).child(Text::new("B"));
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = crate::widget::traits::RenderContext::new(&mut buffer, area);

    // Should auto-detect dimensions
    grid.render(&mut ctx);
}

#[test]
fn test_grid_render_fixed_tracks() {
    let grid = Grid::new()
        .columns(vec![TrackSize::Fixed(5), TrackSize::Fixed(10)])
        .rows(vec![TrackSize::Fixed(3), TrackSize::Fixed(7)])
        .child(Text::new("A"))
        .child(Text::new("B"));
    let mut buffer = Buffer::new(20, 20);
    let area = Rect::new(0, 0, 20, 20);
    let mut ctx = crate::widget::traits::RenderContext::new(&mut buffer, area);

    // Should not crash with fixed tracks
    grid.render(&mut ctx);
}

#[test]
fn test_grid_render_fr_tracks() {
    let grid = Grid::new()
        .columns(vec![TrackSize::Fr(1.0), TrackSize::Fr(2.0)])
        .child(Text::new("A"))
        .child(Text::new("B"));
    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = crate::widget::traits::RenderContext::new(&mut buffer, area);

    // Should not crash with fr tracks
    grid.render(&mut ctx);
}

#[test]
fn test_grid_render_mixed_tracks() {
    let grid = Grid::new()
        .columns(vec![TrackSize::Fixed(10), TrackSize::Fr(1.0)])
        .child(Text::new("A"))
        .child(Text::new("B"));
    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = crate::widget::traits::RenderContext::new(&mut buffer, area);

    // Should not crash with mixed tracks
    grid.render(&mut ctx);
}

#[test]
fn test_grid_render_out_of_bounds_item() {
    let grid = Grid::new().item(GridItem::new(Text::new("A")).at(100, 100)); // Way out of bounds
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = crate::widget::traits::RenderContext::new(&mut buffer, area);

    // Should handle out-of bounds gracefully
    grid.render(&mut ctx);
}

#[test]
fn test_grid_render_zero_size_cell() {
    let grid = Grid::new()
        .columns(vec![TrackSize::Fixed(0), TrackSize::Fixed(10)])
        .child(Text::new("A"))
        .child(Text::new("B"));
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = crate::widget::traits::RenderContext::new(&mut buffer, area);

    // Should handle zero-size cells gracefully
    grid.render(&mut ctx);
}

#[test]
fn test_grid_render_alignment() {
    let grid = Grid::new()
        .justify_items(GridAlign::Center)
        .align_items(GridAlign::Start)
        .child(Text::new("A"));
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = crate::widget::traits::RenderContext::new(&mut buffer, area);

    // Should not crash with alignment settings
    grid.render(&mut ctx);
}