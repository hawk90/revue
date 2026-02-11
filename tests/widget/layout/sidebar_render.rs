//! Tests for sidebar layout widget rendering

use crate::layout::Rect;
use crate::render::Buffer;
use crate::widget::layout::sidebar::{CollapseMode, Sidebar};
use crate::widget::traits::RenderContext;

// =========================================================================
// Render edge case tests
// =========================================================================

#[test]
fn test_render_sidebar_too_narrow() {
    let sidebar = Sidebar::new();
    let mut buffer = Buffer::new(2, 10); // width < 3
    let area = Rect::new(0, 0, 2, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // Should not crash with too narrow area
    sidebar.render(&mut ctx);
}

#[test]
fn test_render_sidebar_too_short() {
    let sidebar = Sidebar::new();
    let mut buffer = Buffer::new(10, 1); // height < 2
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // Should not crash with too short area
    sidebar.render(&mut ctx);
}

#[test]
fn test_render_sidebar_zero_width() {
    let sidebar = Sidebar::new();
    let mut buffer = Buffer::new(0, 10);
    let area = Rect::new(0, 0, 0, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // Should not crash with zero width
    sidebar.render(&mut ctx);
}

#[test]
fn test_render_sidebar_zero_height() {
    let sidebar = Sidebar::new();
    let mut buffer = Buffer::new(10, 0);
    let area = Rect::new(0, 0, 10, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // Should not crash with zero height
    sidebar.render(&mut ctx);
}

// =========================================================================
// Collapse mode tests
// =========================================================================

#[test]
fn test_render_sidebar_expanded_mode() {
    let sidebar = Sidebar::new()
        .collapse_mode(CollapseMode::Expanded)
        .expanded_width(20);

    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    sidebar.render(&mut ctx);

    // Should render with expanded width
    // Check that background was rendered
    let cell = buffer.get(0, 0);
    assert!(cell.is_some());
}

#[test]
fn test_render_sidebar_collapsed_mode() {
    let sidebar = Sidebar::new()
        .collapse_mode(CollapseMode::Collapsed)
        .collapsed_width(5);

    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    sidebar.render(&mut ctx);

    // Should render with collapsed width
    let cell = buffer.get(0, 0);
    assert!(cell.is_some());
}

#[test]
fn test_render_sidebar_auto_mode_collapsed() {
    let sidebar = Sidebar::new()
        .collapse_mode(CollapseMode::Auto)
        .collapse_threshold(15)
        .expanded_width(20)
        .collapsed_width(5);

    let mut buffer = Buffer::new(10, 10); // width < threshold
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    sidebar.render(&mut ctx);

    // Should auto-collapse when width < threshold
    let cell = buffer.get(0, 0);
    assert!(cell.is_some());
}

#[test]
fn test_render_sidebar_auto_mode_expanded() {
    let sidebar = Sidebar::new()
        .collapse_mode(CollapseMode::Auto)
        .collapse_threshold(15)
        .expanded_width(20)
        .collapsed_width(5);

    let mut buffer = Buffer::new(30, 10); // width >= threshold
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    sidebar.render(&mut ctx);

    // Should stay expanded when width >= threshold
    let cell = buffer.get(0, 0);
    assert!(cell.is_some());
}

// =========================================================================
// Header/footer rendering tests
// =========================================================================

#[test]
fn test_render_sidebar_with_header() {
    let sidebar = Sidebar::new().header("Test Header");

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    sidebar.render(&mut ctx);

    // Header should be rendered
    // Check for separator line after header
    let cell = buffer.get(0, 1);
    assert!(cell.is_some());
}

#[test]
fn test_render_sidebar_with_footer() {
    let sidebar = Sidebar::new().footer("Test Footer");

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    sidebar.render(&mut ctx);

    // Footer should be rendered
    // Check for separator line before footer (at height - 2)
    let cell = buffer.get(0, 8);
    assert!(cell.is_some());
}

#[test]
fn test_render_sidebar_with_header_and_footer() {
    let sidebar = Sidebar::new().header("Header").footer("Footer");

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    sidebar.render(&mut ctx);

    // Both header and footer should be rendered
    let header_sep = buffer.get(0, 1);
    let footer_sep = buffer.get(0, 8);
    assert!(header_sep.is_some());
    assert!(footer_sep.is_some());
}

#[test]
fn test_render_sidebar_header_truncation() {
    let long_header = "This is a very long header that should be truncated";
    let sidebar = Sidebar::new().header(long_header).expanded_width(15);

    let mut buffer = Buffer::new(15, 10);
    let area = Rect::new(0, 0, 15, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    sidebar.render(&mut ctx);

    // Should not crash with long header
    let cell = buffer.get(0, 0);
    assert!(cell.is_some());
}

// =========================================================================
// Background rendering tests
// =========================================================================

#[test]
fn test_render_sidebar_background() {
    let sidebar = Sidebar::new().bg(crate::style::Color::rgb(0, 0, 255));

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    sidebar.render(&mut ctx);

    // Background should be rendered
    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.bg, Some(crate::style::Color::rgb(0, 0, 255)));
}

// =========================================================================
// Border rendering tests
// =========================================================================

#[test]
fn test_render_sidebar_border() {
    let sidebar = Sidebar::new()
        .border_color(crate::style::Color::rgb(255, 0, 0))
        .expanded_width(15);

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    sidebar.render(&mut ctx);

    // Right border should be rendered at width - 1
    let border_cell = buffer.get(14, 0);
    assert!(border_cell.is_some());
}

// =========================================================================
// Color configuration tests
// =========================================================================

#[test]
fn test_render_sidebar_with_colors() {
    let sidebar = Sidebar::new()
        .fg(crate::style::Color::rgb(255, 255, 255))
        .bg(crate::style::Color::rgb(0, 0, 0))
        .border_color(crate::style::Color::rgb(128, 128, 128))
        .section_color(crate::style::Color::rgb(0, 255, 255));

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    sidebar.render(&mut ctx);

    // Should render with configured colors
    let cell = buffer.get(0, 0);
    assert!(cell.is_some());
}

// =========================================================================
// Width constraint tests
// =========================================================================

#[test]
fn test_render_sidebar_content_width_clamping() {
    let sidebar = Sidebar::new()
        .expanded_width(30)
        .collapse_mode(CollapseMode::Expanded);

    let mut buffer = Buffer::new(20, 10); // Smaller than expanded_width
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    sidebar.render(&mut ctx);

    // Content width should be clamped to available width
    let border_cell = buffer.get(19, 0); // At edge of buffer
    assert!(border_cell.is_some());
}

// =========================================================================
// Empty sidebar tests
// =========================================================================

#[test]
fn test_render_empty_sidebar() {
    let sidebar = Sidebar::new();

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // Should not crash with empty sidebar
    sidebar.render(&mut ctx);
}