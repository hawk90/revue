//! Presentation Mode widget tests
//!
//! Tests for public API of Presentation widget

use revue::widget::developer::{Presentation, Slide, Transition, SlideAlign};
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;

// =========================================================================
// Presentation creation tests
// =========================================================================

#[test]
fn test_presentation_creation() {
    let pres = Presentation::new().title("Test").author("Author");
    assert_eq!(pres.slide_count(), 0);
}

#[test]
fn test_presentation_default() {
    let pres = Presentation::default();
    assert!(pres.slide_count() == 0);
}

// =========================================================================
// Slide creation tests
// =========================================================================

#[test]
fn test_slide_creation() {
    let s = Slide::new("Title")
        .bullet("Point 1")
        .bullet("Point 2")
        .code("let x = 1;");
    assert_eq!(s.title, "Title");
    assert_eq!(s.content.len(), 5); // 2 bullets + empty + code + empty
}

#[test]
fn test_slide_lines() {
    let s = Slide::new("Test").lines(&["Line 1", "Line 2"]);
    assert_eq!(s.content.len(), 2);
}

#[test]
fn test_slide_lines_empty() {
    let s = Slide::new("Test").lines(&[]);
    assert_eq!(s.content.len(), 0);
}

#[test]
fn test_slide_numbered() {
    let s = Slide::new("Test").numbered(1, "First");
    assert!(s.content.iter().any(|c| c.contains("1.")));
    assert!(s.content.iter().any(|c| c.contains("First")));
}

#[test]
fn test_slide_numbered_multiple() {
    let s = Slide::new("Test")
        .numbered(1, "First")
        .numbered(2, "Second");
    assert_eq!(s.content.len(), 2);
}

#[test]
fn test_slide_notes() {
    let s = Slide::new("Test").notes("Speaker notes here");
    assert_eq!(s.notes, "Speaker notes here");
}

#[test]
fn test_slide_notes_empty() {
    let s = Slide::new("Test").notes("");
    assert_eq!(s.notes, "");
}

#[test]
fn test_slide_bg() {
    let s = Slide::new("Test").bg(Color::BLACK);
    assert_eq!(s.bg, Some(Color::BLACK));
}

#[test]
fn test_slide_bg_none() {
    let s = Slide::new("Test");
    assert!(s.bg.is_none());
}

#[test]
fn test_slide_title_color() {
    let s = Slide::new("Test").title_color(Color::MAGENTA);
    assert_eq!(s.title_color, Color::MAGENTA);
}

#[test]
fn test_slide_content_color() {
    let s = Slide::new("Test").content_color(Color::YELLOW);
    assert_eq!(s.content_color, Color::YELLOW);
}

#[test]
fn test_slide_align_left() {
    let s = Slide::new("Test").align(SlideAlign::Left);
    assert_eq!(s.align, SlideAlign::Left);
}

#[test]
fn test_slide_align_right() {
    let s = Slide::new("Test").align(SlideAlign::Right);
    assert_eq!(s.align, SlideAlign::Right);
}

#[test]
fn test_slide_code() {
    let s = Slide::new("Test").code("let x = 1;");
    assert!(s.content.len() > 2); // Has content
}

#[test]
fn test_slide_code_multiline() {
    let s = Slide::new("Test").code("line1\nline2\nline3");
    assert!(s.content.len() > 3);
}

#[test]
fn test_slide_content_bullet() {
    let s = Slide::new("Test").bullet("Point");
    assert!(s.content.iter().any(|c| c.contains("•")));
}

#[test]
fn test_slide_content_code_empty() {
    let s = Slide::new("Test").code("");
    // Code block with empty string
    assert!(s.content.iter().any(|c| c.trim().is_empty()));
}

// =========================================================================
// Presentation configuration tests
// =========================================================================

#[test]
fn test_presentation_slides() {
    let slides = vec![revue::widget::developer::slide("A"), revue::widget::developer::slide("B"), revue::widget::developer::slide("C")];
    let pres = Presentation::new().slides(slides);
    assert_eq!(pres.slide_count(), 3);
}

#[test]
fn test_presentation_slides_empty() {
    let pres = Presentation::new().slides(vec![]);
    assert_eq!(pres.slide_count(), 0);
}

#[test]
fn test_presentation_transition() {
    let pres = Presentation::new().transition(Transition::Fade);
    assert_eq!(pres.transition, Transition::Fade);
}

#[test]
fn test_presentation_transition_slide() {
    let pres = Presentation::new().transition(Transition::SlideLeft);
    assert_eq!(pres.transition, Transition::SlideLeft);
}

#[test]
fn test_presentation_transition_zoom() {
    let pres = Presentation::new().transition(Transition::ZoomIn);
    assert_eq!(pres.transition, Transition::ZoomIn);
}

#[test]
fn test_presentation_numbers_hide() {
    let pres = Presentation::new().numbers(false);
    assert!(!pres.show_numbers);
}

#[test]
fn test_presentation_numbers_show() {
    let pres = Presentation::new().numbers(true);
    assert!(pres.show_numbers);
}

#[test]
fn test_presentation_progress_hide() {
    let pres = Presentation::new().progress(false);
    assert!(!pres.show_progress);
}

#[test]
fn test_presentation_progress_show() {
    let pres = Presentation::new().progress(true);
    assert!(pres.show_progress);
}

#[test]
fn test_presentation_bg() {
    let pres = Presentation::new().bg(Color::BLACK);
    assert_eq!(pres.bg, Color::BLACK);
}

#[test]
fn test_presentation_accent() {
    let pres = Presentation::new().accent(Color::MAGENTA);
    assert_eq!(pres.accent, Color::MAGENTA);
}

#[test]
fn test_presentation_timer() {
    let pres = Presentation::new().timer(60);
    assert_eq!(pres.timer, Some(60));
}

#[test]
fn test_presentation_timer_none() {
    let pres = Presentation::new();
    assert!(pres.timer.is_none());
}

// =========================================================================
// Navigation tests
// =========================================================================

#[test]
fn test_navigation() {
    let mut pres = Presentation::new()
        .slide(revue::widget::developer::slide("Slide 1"))
        .slide(revue::widget::developer::slide("Slide 2"))
        .slide(revue::widget::developer::slide("Slide 3"));

    assert_eq!(pres.current_index(), 0);
    assert!(pres.next_slide());
    assert_eq!(pres.current_index(), 1);
    assert!(pres.prev());
    assert_eq!(pres.current_index(), 0);
    assert!(!pres.prev()); // Can't go before 0
}

#[test]
fn test_goto_valid() {
    let mut pres = Presentation::new()
        .slide(revue::widget::developer::slide("A"))
        .slide(revue::widget::developer::slide("B"))
        .slide(revue::widget::developer::slide("C"));

    pres.goto(1);
    assert_eq!(pres.current_index(), 1);
}

#[test]
fn test_goto_out_of_bounds() {
    let mut pres = Presentation::new().slide(revue::widget::developer::slide("A")).slide(revue::widget::developer::slide("B"));

    pres.goto(10); // Out of bounds
    assert_eq!(pres.current_index(), 0); // Unchanged
}

#[test]
fn test_goto_empty() {
    let mut pres = Presentation::new();
    pres.goto(0); // Should not panic
    assert_eq!(pres.current_index(), 0);
}

#[test]
fn test_first() {
    let mut pres = Presentation::new().slide(revue::widget::developer::slide("A")).slide(revue::widget::developer::slide("B"));
    pres.goto(1);
    pres.first();
    assert_eq!(pres.current_index(), 0);
}

#[test]
fn test_first_empty() {
    let mut pres = Presentation::new();
    pres.first(); // Should not panic
    assert_eq!(pres.current_index(), 0);
}

#[test]
fn test_last() {
    let mut pres = Presentation::new()
        .slide(revue::widget::developer::slide("A"))
        .slide(revue::widget::developer::slide("B"))
        .slide(revue::widget::developer::slide("C"));
    pres.last();
    assert_eq!(pres.current_index(), 2);
}

#[test]
fn test_last_empty() {
    let mut pres = Presentation::new();
    pres.last(); // Should not panic
    assert_eq!(pres.current_index(), 0);
}

// =========================================================================
// Current slide tests
// =========================================================================

#[test]
fn test_current_slide() {
    let pres = Presentation::new().slide(revue::widget::developer::slide("First"));
    let slide = pres.current_slide();
    assert!(slide.is_some());
    assert_eq!(slide.unwrap().title, "First");
}

#[test]
fn test_current_slide_empty() {
    let pres = Presentation::new();
    let slide = pres.current_slide();
    assert!(slide.is_none());
}

#[test]
fn test_current_slide_second() {
    let mut pres = Presentation::new().slide(revue::widget::developer::slide("A")).slide(revue::widget::developer::slide("B"));
    pres.goto(1);
    let slide = pres.current_slide();
    assert!(slide.is_some());
    assert_eq!(slide.unwrap().title, "B");
}

#[test]
fn test_current_notes() {
    let pres = Presentation::new().slide(revue::widget::developer::slide("Test").notes("Speaker notes"));
    let notes = pres.current_notes();
    assert!(notes.is_some());
    assert_eq!(notes.unwrap(), "Speaker notes");
}

#[test]
fn test_current_notes_no_notes() {
    let pres = Presentation::new().slide(revue::widget::developer::slide("Test"));
    let notes = pres.current_notes();
    assert!(notes.is_some());
    assert_eq!(notes.unwrap(), ""); // Empty notes
}

#[test]
fn test_current_notes_empty() {
    let pres = Presentation::new();
    let notes = pres.current_notes();
    assert!(notes.is_none());
}

// =========================================================================
// Edge case tests
// =========================================================================

#[test]
fn test_next_slide_empty() {
    let mut pres = Presentation::new();
    assert!(!pres.next_slide());
}

#[test]
fn test_next_slide_at_end() {
    let mut pres = Presentation::new().slide(revue::widget::developer::slide("Only"));
    assert!(!pres.next_slide()); // Already at end
}

#[test]
fn test_prev_empty() {
    let mut pres = Presentation::new();
    assert!(!pres.prev());
}

#[test]
fn test_prev_at_start() {
    let mut pres = Presentation::new().slide(revue::widget::developer::slide("A"));
    assert!(!pres.prev()); // Already at 0
}

// =========================================================================
// Tick tests
// =========================================================================

#[test]
fn test_tick_no_transition() {
    let mut pres = Presentation::new().transition(Transition::None);
    pres.tick(0.1);
    assert_eq!(pres.transition_progress, 1.0); // No transition
}

#[test]
fn test_tick_with_transition() {
    let mut pres = Presentation::new().transition(Transition::Fade);
    pres.tick(0.1);
    assert!(pres.transition_progress > 0.0);
}

#[test]
fn test_tick_complete() {
    let mut pres = Presentation::new().transition(Transition::Fade);
    pres.tick(1.0);
    assert_eq!(pres.transition_progress, 1.0);
}

// =========================================================================
// Builder chain tests
// =========================================================================

#[test]
fn test_presentation_builder_chain() {
    let pres = Presentation::new()
        .title("Title")
        .author("Author")
        .transition(Transition::Fade)
        .numbers(false)
        .progress(false)
        .bg(Color::BLACK)
        .accent(Color::WHITE)
        .timer(30);

    assert_eq!(pres.title, "Title");
    assert_eq!(pres.author, "Author");
    assert_eq!(pres.transition, Transition::Fade);
    assert!(!pres.show_numbers);
    assert!(!pres.show_progress);
}

#[test]
fn test_slide_builder_chain() {
    let s = Slide::new("Test")
        .notes("Notes")
        .bg(Color::BLUE)
        .title_color(Color::YELLOW)
        .content_color(Color::GREEN)
        .align(SlideAlign::Left);

    assert_eq!(s.title, "Test");
    assert_eq!(s.notes, "Notes");
    assert_eq!(s.bg, Some(Color::BLUE));
    assert_eq!(s.title_color, Color::YELLOW);
    assert_eq!(s.content_color, Color::GREEN);
    assert_eq!(s.align, SlideAlign::Left);
}

#[test]
fn test_slide_clone() {
    let s1 = Slide::new("Test").notes("Notes");
    let s2 = s1.clone();
    assert_eq!(s1.title, s2.title);
    assert_eq!(s1.notes, s2.notes);
}

// =========================================================================
// Render tests
// =========================================================================

#[test]
fn test_presentation_render() {
    let pres = Presentation::new()
        .title("Test Presentation")
        .slide(revue::widget::developer::slide("Intro").bullet("Hello"));

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = revue::widget::traits::RenderContext::new(&mut buffer, area);

    pres.render(&mut ctx);
}

#[test]
fn test_render_empty_presentation() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = revue::widget::traits::RenderContext::new(&mut buffer, area);

    let pres = Presentation::new();
    pres.render(&mut ctx); // Should show title slide
}

#[test]
fn test_render_with_content() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = revue::widget::traits::RenderContext::new(&mut buffer, area);

    let mut pres = Presentation::new().slide(revue::widget::developer::slide("Content").line("Content here"));
    pres.goto(1);
    pres.render(&mut ctx);
}

// =========================================================================
// Enum tests
// =========================================================================

#[test]
fn test_transition_default() {
    assert_eq!(Transition::default(), Transition::None);
}

#[test]
fn test_transition_clone() {
    let t = Transition::Fade;
    let cloned = t;
    assert_eq!(t, cloned);
}

#[test]
fn test_transition_copy() {
    let t1 = Transition::SlideLeft;
    let t2 = t1;
    assert_eq!(t1, Transition::SlideLeft);
    assert_eq!(t2, Transition::SlideLeft);
}

#[test]
fn test_transition_partial_eq() {
    assert_eq!(Transition::Fade, Transition::Fade);
    assert_ne!(Transition::Fade, Transition::ZoomIn);
}

#[test]
fn test_slide_align_default() {
    assert_eq!(SlideAlign::default(), SlideAlign::Center);
}

#[test]
fn test_slide_align_clone() {
    let align = SlideAlign::Left;
    let cloned = align;
    assert_eq!(align, cloned);
}

#[test]
fn test_slide_align_copy() {
    let a1 = SlideAlign::Right;
    let a2 = a1;
    assert_eq!(a1, SlideAlign::Right);
    assert_eq!(a2, SlideAlign::Right);
}

// =========================================================================
// Helper function tests
// =========================================================================

#[test]
fn test_presentation_helper() {
    let pres = revue::widget::developer::presentation();
    assert!(pres.slide_count() == 0);
}

#[test]
fn test_slide_helper() {
    let s = revue::widget::developer::slide("Title");
    assert_eq!(s.title, "Title");
}