//! Tests for MarkdownPresentation widget
//!
//! These tests verify the functionality of the markdown presentation widget
//! including slide navigation, mode switching, rendering, and builder methods.

use super::*;

#[test]
fn test_creation() {
    let pres = MarkdownPresentation::new("# Hello\n\n---\n\n# World");
    assert_eq!(pres.slide_count(), 2);
    assert_eq!(pres.current_index(), 0);
}

#[test]
fn test_navigation() {
    let mut pres = MarkdownPresentation::new("# A\n---\n# B\n---\n# C");

    assert_eq!(pres.current_index(), 0);
    assert!(pres.is_first());
    assert!(!pres.is_last());

    assert!(pres.next_slide());
    assert_eq!(pres.current_index(), 1);

    assert!(pres.next_slide());
    assert_eq!(pres.current_index(), 2);
    assert!(pres.is_last());

    assert!(!pres.next_slide()); // Can't go past last
    assert_eq!(pres.current_index(), 2);

    pres.first();
    assert_eq!(pres.current_index(), 0);

    pres.last();
    assert_eq!(pres.current_index(), 2);

    pres.goto(1);
    assert_eq!(pres.current_index(), 1);
}

#[test]
fn test_mode_toggle() {
    let mut pres = MarkdownPresentation::new("# Test");

    assert_eq!(pres.current_mode(), ViewMode::Preview);

    pres.toggle_mode();
    assert_eq!(pres.current_mode(), ViewMode::Slides);

    pres.toggle_mode();
    assert_eq!(pres.current_mode(), ViewMode::Preview);
}

#[test]
fn test_builder_pattern() {
    let pres = MarkdownPresentation::new("# Test")
        .bg(Color::BLACK)
        .accent(Color::GREEN)
        .heading_fg(Color::CYAN)
        .numbers(false)
        .progress(false)
        .mode(ViewMode::Slides);

    assert_eq!(pres.current_mode(), ViewMode::Slides);
    assert_eq!(pres.bg, Color::BLACK);
    assert_eq!(pres.accent, Color::GREEN);
}

#[test]
fn test_indicator() {
    let pres = MarkdownPresentation::new("# A\n---\n# B\n---\n# C");
    assert_eq!(pres.indicator(), "1/3");
    assert_eq!(pres.indicator_bracketed(), "[1/3]");
}

#[test]
fn test_render_preview() {
    let pres = MarkdownPresentation::new("# Hello\n\nWorld").mode(ViewMode::Preview);

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    pres.render(&mut ctx);
    // Should not crash
}

#[test]
fn test_render_slides() {
    let pres = MarkdownPresentation::new("# Slide 1\n\nContent\n---\n# Slide 2")
        .mode(ViewMode::Slides);

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    pres.render(&mut ctx);
    // Should not crash
}

#[test]
fn test_current_notes() {
    let md = "# Title\n\nContent\n\n<!-- notes: Speaker note here -->";
    let pres = MarkdownPresentation::new(md);

    assert_eq!(pres.current_notes(), Some("Speaker note here"));
}

#[test]
fn test_reload() {
    let mut pres = MarkdownPresentation::new("# A\n---\n# B");
    assert_eq!(pres.slide_count(), 2);

    pres.next_slide();
    assert_eq!(pres.current_index(), 1);

    pres.reload("# X\n---\n# Y\n---\n# Z");
    assert_eq!(pres.slide_count(), 3);
    assert_eq!(pres.current_index(), 0); // Reset after reload
}

#[test]
fn test_scroll() {
    let mut pres = MarkdownPresentation::new("# Test");

    assert_eq!(pres.scroll_offset, 0);

    pres.scroll_down(5);
    assert_eq!(pres.scroll_offset, 5);

    pres.scroll_up(2);
    assert_eq!(pres.scroll_offset, 3);

    pres.scroll_to_top();
    assert_eq!(pres.scroll_offset, 0);
}

#[test]
fn test_progress() {
    let pres = MarkdownPresentation::new("# A\n---\n# B\n---\n# C\n---\n# D");
    assert!((pres.progress_value() - 0.25).abs() < 0.01);
}

#[test]
fn test_strip_title() {
    let pres = MarkdownPresentation::new("");

    let content = "# Title\n\nContent here";
    let stripped = pres.strip_title(content);
    assert_eq!(stripped.trim(), "Content here");

    let content2 = "## Subtitle\n\nMore content";
    let stripped2 = pres.strip_title(content2);
    assert_eq!(stripped2.trim(), "More content");
}

// =========================================================================
// ViewMode enum tests
// =========================================================================

#[test]
fn test_view_mode_default() {
    let mode = ViewMode::default();
    assert_eq!(mode, ViewMode::Preview);
}

#[test]
fn test_view_mode_clone() {
    let mode = ViewMode::Slides;
    let cloned = mode;
    assert_eq!(mode, cloned);
}

#[test]
fn test_view_mode_copy() {
    let mode1 = ViewMode::Slides;
    let mode2 = mode1;
    assert_eq!(mode1, ViewMode::Slides);
    assert_eq!(mode2, ViewMode::Slides);
}

#[test]
fn test_view_mode_partial_eq() {
    assert_eq!(ViewMode::Preview, ViewMode::Preview);
    assert_ne!(ViewMode::Preview, ViewMode::Slides);
}

// =========================================================================
// MarkdownPresentation::from_slides tests
// =========================================================================

#[test]
fn test_from_slides() {
    use crate::widget::slides::SlideContent;

    let slides = vec![
        SlideContent::new("# Slide 1"),
        SlideContent::new("# Slide 2"),
    ];

    let pres = MarkdownPresentation::from_slides(slides);
    assert_eq!(pres.slide_count(), 2);
    assert_eq!(pres.source(), "# Slide 1\n---\n# Slide 2");
}

#[test]
fn test_from_slides_empty() {
    let pres = MarkdownPresentation::from_slides(vec![]);
    assert_eq!(pres.slide_count(), 0);
}

// =========================================================================
// MarkdownPresentation::text_sizing tests
// =========================================================================

#[test]
fn test_text_sizing() {
    let _pres = MarkdownPresentation::new("# Test").text_sizing(true);
    // The actual value depends on terminal support
    // Just verify the method doesn't panic
}

#[test]
fn test_text_sizing_disabled() {
    let pres = MarkdownPresentation::new("# Test").text_sizing(false);
    assert!(!pres.use_text_sizing);
}

// =========================================================================
// MarkdownPresentation::figlet_font tests
// =========================================================================

#[test]
fn test_figlet_font() {
    let pres = MarkdownPresentation::new("# Test").figlet_font(FigletFont::Small);
    assert_eq!(pres.figlet_font, FigletFont::Small);
}

// =========================================================================
// MarkdownPresentation::link_fg tests
// =========================================================================

#[test]
fn test_link_fg() {
    let pres = MarkdownPresentation::new("# Test").link_fg(Color::RED);
    assert_eq!(pres.link_fg, Color::RED);
}

// =========================================================================
// MarkdownPresentation::code_fg tests
// =========================================================================

#[test]
fn test_code_fg() {
    let pres = MarkdownPresentation::new("# Test").code_fg(Color::GREEN);
    assert_eq!(pres.code_fg, Color::GREEN);
}

// =========================================================================
// MarkdownPresentation::current_slide tests
// =========================================================================

#[test]
fn test_current_slide() {
    let pres = MarkdownPresentation::new("# A\n---\n# B");
    assert!(pres.current_slide().is_some());
    assert_eq!(pres.current_slide().unwrap().markdown(), "# A\n");
}

#[test]
fn test_current_slide_empty() {
    let pres = MarkdownPresentation::new("");
    assert!(pres.current_slide().is_none());
}

// =========================================================================
// MarkdownPresentation::slides tests
// =========================================================================

#[test]
fn test_slides() {
    let pres = MarkdownPresentation::new("# A\n---\n# B\n---\n# C");
    assert_eq!(pres.slides().len(), 3);
}

#[test]
fn test_slides_empty() {
    let pres = MarkdownPresentation::new("");
    assert_eq!(pres.slides().len(), 0); // No slides for empty markdown
}

// =========================================================================
// MarkdownPresentation::source tests
// =========================================================================

#[test]
fn test_source() {
    let markdown = "# Hello\n\nWorld";
    let pres = MarkdownPresentation::new(markdown);
    assert_eq!(pres.source(), markdown);
}

// =========================================================================
// MarkdownPresentation::prev_slide tests
// =========================================================================

#[test]
fn test_prev_slide() {
    let mut pres = MarkdownPresentation::new("# A\n---\n# B\n---\n# C");
    pres.last();
    assert_eq!(pres.current_index(), 2);

    assert!(pres.prev_slide());
    assert_eq!(pres.current_index(), 1);

    assert!(pres.prev_slide());
    assert_eq!(pres.current_index(), 0);

    assert!(!pres.prev_slide()); // Can't go before first
    assert_eq!(pres.current_index(), 0);
}

// =========================================================================
// Helper function tests
// =========================================================================

#[test]
fn test_markdown_presentation_helper() {
    let pres = markdown_presentation("# Test");
    assert_eq!(pres.source(), "# Test");
}

// =========================================================================
// MarkdownPresentation Default tests
// =========================================================================

#[test]
fn test_default() {
    let pres = MarkdownPresentation::default();
    assert_eq!(pres.source(), "");
    assert_eq!(pres.current_mode(), ViewMode::Preview);
}

// =========================================================================
// Edge case tests
// =========================================================================

#[test]
fn test_scroll_edge_cases() {
    let mut pres = MarkdownPresentation::new("# Test");

    pres.scroll_up(10); // Should not go negative
    assert_eq!(pres.scroll_offset, 0);

    pres.scroll_down(usize::MAX); // Should saturate
    assert!(pres.scroll_offset > 0);
}

#[test]
fn test_goto_bounds() {
    let mut pres = MarkdownPresentation::new("# A\n---\n# B\n---\n# C");

    pres.goto(10); // Out of bounds - stays at current (0)
    assert_eq!(pres.current_index(), 0);

    pres.goto(1);
    assert_eq!(pres.current_index(), 1);
}

#[test]
fn test_mode_setter() {
    let pres = MarkdownPresentation::new("# Test").mode(ViewMode::Slides);
    assert_eq!(pres.current_mode(), ViewMode::Slides);
}

// =========================================================================
// ViewMode::Slides tests
// =========================================================================

#[test]
fn test_view_mode_slides_rendering() {
    let pres = MarkdownPresentation::new("# Slide 1\n\n---\n# Slide 2").mode(ViewMode::Slides);
    assert_eq!(pres.current_mode(), ViewMode::Slides);
}

// =========================================================================
// Clone tests
// =========================================================================

#[test]
fn test_clone() {
    let pres1 = MarkdownPresentation::new("# Test")
        .bg(Color::BLACK)
        .accent(Color::RED)
        .mode(ViewMode::Slides);
    let pres2 = pres1.clone();

    assert_eq!(pres1.source(), pres2.source());
    assert_eq!(pres1.current_mode(), pres2.current_mode());
}

// =========================================================================
// Debug trait tests
// =========================================================================

#[test]
fn test_view_mode_debug() {
    let mode = ViewMode::Slides;
    let debug_str = format!("{:?}", mode);
    assert!(debug_str.contains("Slides"));
}

#[test]
fn test_markdown_presentation_debug() {
    let pres = MarkdownPresentation::new("# Test");
    let debug_str = format!("{:?}", pres);
    assert!(debug_str.contains("MarkdownPresentation"));
}

// =========================================================================
// Combined builder tests
// =========================================================================

#[test]
fn test_combined_builder() {
    let pres = MarkdownPresentation::new("# Test")
        .bg(Color::rgb(10, 10, 20))
        .accent(Color::MAGENTA)
        .heading_fg(Color::WHITE)
        .link_fg(Color::CYAN)
        .code_fg(Color::YELLOW)
        .numbers(true)
        .progress(true)
        .mode(ViewMode::Slides);

    assert_eq!(pres.current_mode(), ViewMode::Slides);
    assert!(pres.show_numbers);
    assert!(pres.show_progress);
}

// =========================================================================
// Strip title edge cases
// =========================================================================

#[test]
fn test_strip_title_no_heading() {
    let pres = MarkdownPresentation::new("");
    let content = "Just some content\n\nNo heading";
    let stripped = pres.strip_title(content);
    assert_eq!(stripped.trim(), "Just some content\n\nNo heading");
}

#[test]
fn test_strip_title_only_heading() {
    let pres = MarkdownPresentation::new("");
    let content = "# Title Only";
    let stripped = pres.strip_title(content);
    assert!(stripped.trim().is_empty());
}