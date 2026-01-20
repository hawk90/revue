//! Presentation widget integration tests

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::{RenderContext, StyledView, View};
use revue::widget::{presentation, slide, Presentation, Slide, SlideAlign, Transition};

// =============================================================================
// Transition Enum Tests
// =============================================================================

#[test]
fn test_transition_none() {
    let t = Transition::None;
    assert_eq!(t, Transition::default());
}

#[test]
fn test_transition_fade() {
    let t = Transition::Fade;
    assert_eq!(t, Transition::Fade);
}

#[test]
fn test_transition_slide_left() {
    let t = Transition::SlideLeft;
    assert_eq!(t, Transition::SlideLeft);
}

#[test]
fn test_transition_slide_right() {
    let t = Transition::SlideRight;
    assert_eq!(t, Transition::SlideRight);
}

#[test]
fn test_transition_slide_up() {
    let t = Transition::SlideUp;
    assert_eq!(t, Transition::SlideUp);
}

#[test]
fn test_transition_zoom_in() {
    let t = Transition::ZoomIn;
    assert_eq!(t, Transition::ZoomIn);
}

#[test]
fn test_transition_partial_eq() {
    assert_eq!(Transition::Fade, Transition::Fade);
    assert_ne!(Transition::Fade, Transition::None);
    assert_ne!(Transition::SlideLeft, Transition::SlideRight);
}

// =============================================================================
// SlideAlign Enum Tests
// =============================================================================

#[test]
fn test_slide_align_left() {
    let align = SlideAlign::Left;
    assert_eq!(align, SlideAlign::Left);
}

#[test]
fn test_slide_align_center() {
    let align = SlideAlign::Center;
    assert_eq!(align, SlideAlign::Center);
    assert_eq!(align, SlideAlign::default());
}

#[test]
fn test_slide_align_right() {
    let align = SlideAlign::Right;
    assert_eq!(align, SlideAlign::Right);
}

#[test]
fn test_slide_align_partial_eq() {
    assert_eq!(SlideAlign::Left, SlideAlign::Left);
    assert_ne!(SlideAlign::Left, SlideAlign::Center);
    assert_ne!(SlideAlign::Center, SlideAlign::Right);
}

// =============================================================================
// Slide Constructor Tests
// =============================================================================

#[test]
fn test_slide_new() {
    let s = Slide::new("Test Title");
    assert_eq!(s.title, "Test Title");
    assert!(s.content.is_empty());
    assert!(s.notes.is_empty());
    assert!(s.bg.is_none());
    assert_eq!(s.title_color, Color::CYAN);
    assert_eq!(s.content_color, Color::WHITE);
    assert_eq!(s.align, SlideAlign::Center);
}

#[test]
fn test_slide_helper_function() {
    let s = slide("Helper Title");
    assert_eq!(s.title, "Helper Title");
}

// =============================================================================
// Slide Builder Methods Tests
// =============================================================================

#[test]
fn test_slide_line() {
    let s = Slide::new("Title").line("First line").line("Second line");
    assert_eq!(s.content.len(), 2);
    assert_eq!(s.content[0], "First line");
    assert_eq!(s.content[1], "Second line");
}

#[test]
fn test_slide_lines() {
    let s = Slide::new("Title").lines(&["Line 1", "Line 2", "Line 3"]);
    assert_eq!(s.content.len(), 3);
    assert_eq!(s.content[0], "Line 1");
    assert_eq!(s.content[1], "Line 2");
    assert_eq!(s.content[2], "Line 3");
}

#[test]
fn test_slide_lines_empty() {
    let s = Slide::new("Title").lines(&[]);
    assert_eq!(s.content.len(), 0);
}

#[test]
fn test_slide_bullet() {
    let s = Slide::new("Title")
        .bullet("First point")
        .bullet("Second point")
        .bullet("Third point");
    assert_eq!(s.content.len(), 3);
    assert_eq!(s.content[0], "  • First point");
    assert_eq!(s.content[1], "  • Second point");
    assert_eq!(s.content[2], "  • Third point");
}

#[test]
fn test_slide_numbered() {
    let s = Slide::new("Title")
        .numbered(1, "First item")
        .numbered(2, "Second item")
        .numbered(3, "Third item");
    assert_eq!(s.content.len(), 3);
    assert_eq!(s.content[0], "  1. First item");
    assert_eq!(s.content[1], "  2. Second item");
    assert_eq!(s.content[2], "  3. Third item");
}

#[test]
fn test_slide_code_single_line() {
    let s = Slide::new("Title").code("let x = 42;");
    assert_eq!(s.content.len(), 3); // empty, code, empty
    assert_eq!(s.content[0], "");
    assert_eq!(s.content[1], "    let x = 42;");
    assert_eq!(s.content[2], "");
}

#[test]
fn test_slide_code_multi_line() {
    let code = "fn main() {\n    println!(\"Hello\");\n}";
    let s = Slide::new("Title").code(code);
    assert_eq!(s.content.len(), 5); // empty, 3 lines code, empty
    assert!(s.content[1].contains("fn main()"));
    assert!(s.content[2].contains("println!"));
    assert!(s.content[3].contains("}"));
}

#[test]
fn test_slide_notes() {
    let s = Slide::new("Title").notes("Speaker notes here");
    assert_eq!(s.notes, "Speaker notes here");
}

#[test]
fn test_slide_bg() {
    let s = Slide::new("Title").bg(Color::BLUE);
    assert_eq!(s.bg, Some(Color::BLUE));
}

#[test]
fn test_slide_bg_none() {
    let s = Slide::new("Title");
    assert_eq!(s.bg, None);
}

#[test]
fn test_slide_title_color() {
    let s = Slide::new("Title").title_color(Color::RED);
    assert_eq!(s.title_color, Color::RED);
}

#[test]
fn test_slide_content_color() {
    let s = Slide::new("Title").content_color(Color::GREEN);
    assert_eq!(s.content_color, Color::GREEN);
}

#[test]
fn test_slide_builder_align_left() {
    let s = Slide::new("Title").align(SlideAlign::Left);
    assert_eq!(s.align, SlideAlign::Left);
}

#[test]
fn test_slide_builder_align_center() {
    let s = Slide::new("Title").align(SlideAlign::Center);
    assert_eq!(s.align, SlideAlign::Center);
}

#[test]
fn test_slide_builder_align_right() {
    let s = Slide::new("Title").align(SlideAlign::Right);
    assert_eq!(s.align, SlideAlign::Right);
}

#[test]
fn test_slide_chained_builders() {
    let s = Slide::new("Complex Slide")
        .line("Introduction")
        .bullet("Point 1")
        .bullet("Point 2")
        .code("example()")
        .notes("Remember to explain the code")
        .bg(Color::BLACK)
        .title_color(Color::YELLOW)
        .content_color(Color::WHITE)
        .align(SlideAlign::Left);

    assert_eq!(s.title, "Complex Slide");
    assert_eq!(s.content.len(), 6); // line + 2 bullets + empty + code + empty
    assert_eq!(s.notes, "Remember to explain the code");
    assert_eq!(s.bg, Some(Color::BLACK));
    assert_eq!(s.title_color, Color::YELLOW);
    assert_eq!(s.content_color, Color::WHITE);
    assert_eq!(s.align, SlideAlign::Left);
}

// =============================================================================
// Presentation Constructor Tests
// =============================================================================

#[test]
fn test_presentation_new() {
    let pres = Presentation::new();
    assert_eq!(pres.slide_count(), 0);
    assert_eq!(pres.current_index(), 0);
    assert!(pres.current_slide().is_none());
    assert!(pres.current_notes().is_none());
}

#[test]
fn test_presentation_helper_function() {
    let pres = presentation();
    assert_eq!(pres.slide_count(), 0);
}

#[test]
fn test_presentation_default() {
    let pres = Presentation::default();
    assert_eq!(pres.slide_count(), 0);
    assert_eq!(pres.current_index(), 0);
}

// =============================================================================
// Presentation Builder Methods Tests
// =============================================================================

#[test]
fn test_presentation_title() {
    let pres = Presentation::new().title("My Presentation");
    // Title is private, but we can verify it through rendering
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    pres.render(&mut ctx);
    // Check that title appears in buffer
    let mut found_title = false;
    for y in 0..24 {
        for x in 0..80 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == 'M' {
                    found_title = true;
                    break;
                }
            }
        }
    }
    assert!(found_title);
}

#[test]
fn test_presentation_author() {
    let pres = Presentation::new().title("Presentation").author("John Doe");
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    pres.render(&mut ctx);
    // Author appears in title slide
}

#[test]
fn test_presentation_slide_single() {
    let pres = Presentation::new().slide(slide("Slide 1"));
    assert_eq!(pres.slide_count(), 1);
}

#[test]
fn test_presentation_slide_multiple() {
    let pres = Presentation::new()
        .slide(slide("Slide 1"))
        .slide(slide("Slide 2"))
        .slide(slide("Slide 3"));
    assert_eq!(pres.slide_count(), 3);
}

#[test]
fn test_presentation_slides_vec() {
    let slides = vec![
        slide("Slide 1").bullet("Point 1"),
        slide("Slide 2").bullet("Point 2"),
        slide("Slide 3").bullet("Point 3"),
    ];
    let pres = Presentation::new().slides(slides);
    assert_eq!(pres.slide_count(), 3);
}

#[test]
fn test_presentation_slides_empty_vec() {
    let pres = Presentation::new().slides(vec![]);
    assert_eq!(pres.slide_count(), 0);
}

#[test]
fn test_presentation_transition_none() {
    let _pres = Presentation::new().transition(Transition::None);
    // Transition is private, verified through behavior
}

#[test]
fn test_presentation_transition_fade() {
    let _pres = Presentation::new().transition(Transition::Fade);
}

#[test]
fn test_presentation_transition_slide_left() {
    let _pres = Presentation::new().transition(Transition::SlideLeft);
}

#[test]
fn test_presentation_transition_slide_right() {
    let _pres = Presentation::new().transition(Transition::SlideRight);
}

#[test]
fn test_presentation_transition_slide_up() {
    let _pres = Presentation::new().transition(Transition::SlideUp);
}

#[test]
fn test_presentation_transition_zoom_in() {
    let _pres = Presentation::new().transition(Transition::ZoomIn);
}

#[test]
fn test_presentation_numbers_true() {
    let _pres = Presentation::new().numbers(true);
    // Show numbers is private, verified through rendering
}

#[test]
fn test_presentation_numbers_false() {
    let _pres = Presentation::new().numbers(false);
}

#[test]
fn test_presentation_progress_true() {
    let _pres = Presentation::new().progress(true);
}

#[test]
fn test_presentation_progress_false() {
    let _pres = Presentation::new().progress(false);
}

#[test]
fn test_presentation_bg() {
    let _pres = Presentation::new().bg(Color::BLACK);
    // bg is private, verified through rendering
}

#[test]
fn test_presentation_accent() {
    let _pres = Presentation::new().accent(Color::MAGENTA);
}

#[test]
fn test_presentation_timer() {
    let _pres = Presentation::new().timer(600); // 10 minutes
                                                // Timer is private, verified through rendering
}

#[test]
fn test_presentation_timer_zero() {
    let _pres = Presentation::new().timer(0);
}

#[test]
fn test_presentation_chained_builders() {
    let pres = Presentation::new()
        .title("Full Presentation")
        .author("Presenter Name")
        .slide(slide("Intro").bullet("Welcome"))
        .slides(vec![
            slide("Topic 1").bullet("Point 1"),
            slide("Topic 2").bullet("Point 2"),
        ])
        .transition(Transition::Fade)
        .numbers(true)
        .progress(true)
        .bg(Color::BLACK)
        .accent(Color::CYAN)
        .timer(1800); // 30 minutes

    assert_eq!(pres.slide_count(), 3);
}

// =============================================================================
// Navigation Tests
// =============================================================================

#[test]
fn test_navigation_next_slide() {
    let mut pres = Presentation::new()
        .slide(slide("Slide 1"))
        .slide(slide("Slide 2"))
        .slide(slide("Slide 3"));

    assert_eq!(pres.current_index(), 0);
    assert!(pres.next_slide());
    assert_eq!(pres.current_index(), 1);
    assert!(pres.next_slide());
    assert_eq!(pres.current_index(), 2);
    assert!(!pres.next_slide()); // Can't go past end
    assert_eq!(pres.current_index(), 2);
}

#[test]
fn test_navigation_prev() {
    let mut pres = Presentation::new()
        .slide(slide("Slide 1"))
        .slide(slide("Slide 2"))
        .slide(slide("Slide 3"));

    pres.next_slide();
    pres.next_slide();
    assert_eq!(pres.current_index(), 2);

    assert!(pres.prev());
    assert_eq!(pres.current_index(), 1);
    assert!(pres.prev());
    assert_eq!(pres.current_index(), 0);
    assert!(!pres.prev()); // Can't go before 0
    assert_eq!(pres.current_index(), 0);
}

#[test]
fn test_navigation_goto_valid() {
    let mut pres = Presentation::new()
        .slide(slide("Slide 1"))
        .slide(slide("Slide 2"))
        .slide(slide("Slide 3"))
        .slide(slide("Slide 4"));

    pres.goto(2);
    assert_eq!(pres.current_index(), 2);

    pres.goto(0);
    assert_eq!(pres.current_index(), 0);

    pres.goto(3);
    assert_eq!(pres.current_index(), 3);
}

#[test]
fn test_navigation_goto_invalid() {
    let mut pres = Presentation::new()
        .slide(slide("Slide 1"))
        .slide(slide("Slide 2"));

    pres.goto(5); // Out of bounds
    assert_eq!(pres.current_index(), 0); // Should not change

    pres.goto(1);
    assert_eq!(pres.current_index(), 1);

    pres.goto(100); // Way out of bounds
    assert_eq!(pres.current_index(), 1); // Should not change
}

#[test]
fn test_navigation_first() {
    let mut pres = Presentation::new()
        .slide(slide("Slide 1"))
        .slide(slide("Slide 2"))
        .slide(slide("Slide 3"));

    pres.goto(2);
    assert_eq!(pres.current_index(), 2);

    pres.first();
    assert_eq!(pres.current_index(), 0);
}

#[test]
fn test_navigation_last() {
    let mut pres = Presentation::new()
        .slide(slide("Slide 1"))
        .slide(slide("Slide 2"))
        .slide(slide("Slide 3"));

    pres.last();
    assert_eq!(pres.current_index(), 2);
}

#[test]
fn test_navigation_last_empty() {
    let mut pres = Presentation::new();
    pres.last();
    assert_eq!(pres.current_index(), 0);
}

#[test]
fn test_navigation_first_empty() {
    let mut pres = Presentation::new();
    pres.first();
    assert_eq!(pres.current_index(), 0);
}

#[test]
fn test_navigation_next_prev_cycle() {
    let mut pres = Presentation::new()
        .slide(slide("Slide 1"))
        .slide(slide("Slide 2"));

    // Navigate forward
    assert!(pres.next_slide());
    assert_eq!(pres.current_index(), 1);
    assert!(!pres.next_slide());

    // Navigate backward
    assert!(pres.prev());
    assert_eq!(pres.current_index(), 0);
    assert!(!pres.prev());

    // Navigate forward again
    assert!(pres.next_slide());
    assert_eq!(pres.current_index(), 1);
}

// =============================================================================
// Query Methods Tests
// =============================================================================

#[test]
fn test_query_current_index_initial() {
    let pres = Presentation::new()
        .slide(slide("Slide 1"))
        .slide(slide("Slide 2"));
    assert_eq!(pres.current_index(), 0);
}

#[test]
fn test_query_current_index_after_navigation() {
    let mut pres = Presentation::new()
        .slide(slide("Slide 1"))
        .slide(slide("Slide 2"))
        .slide(slide("Slide 3"));

    pres.next_slide();
    assert_eq!(pres.current_index(), 1);

    pres.goto(2);
    assert_eq!(pres.current_index(), 2);
}

#[test]
fn test_query_slide_count_empty() {
    let pres = Presentation::new();
    assert_eq!(pres.slide_count(), 0);
}

#[test]
fn test_query_slide_count_single() {
    let pres = Presentation::new().slide(slide("Slide 1"));
    assert_eq!(pres.slide_count(), 1);
}

#[test]
fn test_query_slide_count_multiple() {
    let pres = Presentation::new()
        .slide(slide("Slide 1"))
        .slide(slide("Slide 2"))
        .slide(slide("Slide 3"))
        .slide(slide("Slide 4"));
    assert_eq!(pres.slide_count(), 4);
}

#[test]
fn test_query_current_slide_empty() {
    let pres = Presentation::new();
    assert!(pres.current_slide().is_none());
}

#[test]
fn test_query_current_slide_valid() {
    let pres = Presentation::new().slide(slide("First Slide"));
    let slide = pres.current_slide();
    assert!(slide.is_some());
    assert_eq!(slide.unwrap().title, "First Slide");
}

#[test]
fn test_query_current_slide_after_navigation() {
    let mut pres = Presentation::new()
        .slide(slide("Slide 1"))
        .slide(slide("Slide 2"))
        .slide(slide("Slide 3"));

    pres.goto(1);
    let slide = pres.current_slide();
    assert!(slide.is_some());
    assert_eq!(slide.unwrap().title, "Slide 2");

    pres.next_slide();
    let slide = pres.current_slide();
    assert_eq!(slide.unwrap().title, "Slide 3");
}

#[test]
fn test_query_current_notes_empty() {
    let pres = Presentation::new().slide(slide("Slide 1"));
    assert!(pres.current_notes().is_some());
    assert_eq!(pres.current_notes().unwrap(), "");
}

#[test]
fn test_query_current_notes_with_notes() {
    let pres = Presentation::new()
        .slide(slide("Slide 1").notes("Remember to mention this important point"));
    assert_eq!(
        pres.current_notes().unwrap(),
        "Remember to mention this important point"
    );
}

#[test]
fn test_query_current_notes_after_navigation() {
    let mut pres = Presentation::new()
        .slide(slide("Slide 1").notes("Notes for slide 1"))
        .slide(slide("Slide 2").notes("Notes for slide 2"));

    assert_eq!(pres.current_notes().unwrap(), "Notes for slide 1");

    pres.next_slide();
    assert_eq!(pres.current_notes().unwrap(), "Notes for slide 2");

    pres.prev();
    assert_eq!(pres.current_notes().unwrap(), "Notes for slide 1");
}

#[test]
fn test_query_current_notes_no_slides() {
    let pres = Presentation::new();
    assert!(pres.current_notes().is_none());
}

// =============================================================================
// Timer Tests
// =============================================================================

#[test]
fn test_tick_basic() {
    let mut pres = Presentation::new().transition(Transition::Fade);
    // tick() is used to advance transition animation
    // We can't directly access transition_progress, but we can call it
    pres.tick(0.1);
    pres.tick(0.2);
    pres.tick(0.5);
    // Should not panic
}

#[test]
fn test_tick_large_dt() {
    let mut pres = Presentation::new().transition(Transition::Fade);
    pres.tick(100.0); // Very large delta time
                      // Should clamp to 1.0
}

#[test]
fn test_tick_zero_dt() {
    let mut pres = Presentation::new().transition(Transition::Fade);
    pres.tick(0.0);
}

#[test]
fn test_tick_negative_dt() {
    let mut pres = Presentation::new().transition(Transition::Fade);
    pres.tick(-0.1); // Negative delta time (shouldn't happen in practice)
}

#[test]
fn test_tick_multiple_calls() {
    let mut pres = Presentation::new().transition(Transition::Fade);
    for _ in 0..10 {
        pres.tick(0.05);
    }
    // Should complete transition
}

#[test]
fn test_tick_after_navigation() {
    let mut pres = Presentation::new()
        .slide(slide("Slide 1"))
        .slide(slide("Slide 2"))
        .transition(Transition::Fade);

    pres.next_slide();
    pres.tick(0.1);
    pres.tick(0.1);
    pres.tick(0.1);

    pres.prev();
    pres.tick(0.1);
    pres.tick(0.1);
}

// =============================================================================
// Rendering Tests
// =============================================================================

#[test]
fn test_render_title_slide_only() {
    let pres = Presentation::new().title("Title Only Presentation");

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    pres.render(&mut ctx);

    // Should render title slide
    let mut found_title = false;
    for y in 0..24 {
        for x in 0..80 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == 'T' {
                    found_title = true;
                    break;
                }
            }
        }
    }
    assert!(found_title);
}

#[test]
fn test_render_title_slide_with_author() {
    let pres = Presentation::new()
        .title("My Presentation")
        .author("Jane Doe");

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    pres.render(&mut ctx);

    // Should render both title and author
}

#[test]
fn test_render_content_slide_basic() {
    let pres = Presentation::new()
        .title("Presentation")
        .slide(slide("First Slide").line("Content here"));

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    pres.render(&mut ctx);
}

#[test]
fn test_render_content_slide_with_bullets() {
    let pres = Presentation::new().slide(
        slide("Bullet Points")
            .bullet("First point")
            .bullet("Second point")
            .bullet("Third point"),
    );

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    pres.render(&mut ctx);
}

#[test]
fn test_render_content_slide_with_code() {
    let pres = Presentation::new()
        .slide(slide("Code Example").code("fn main() {\n    println!(\"Hello\");\n}"));

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    pres.render(&mut ctx);
}

#[test]
fn test_render_with_custom_background() {
    let pres = Presentation::new()
        .bg(Color::rgb(30, 30, 40))
        .slide(slide("Custom BG"));

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    pres.render(&mut ctx);

    // Check background color
    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.bg, Some(Color::rgb(30, 30, 40)));
}

#[test]
fn test_render_with_custom_accent() {
    let pres = Presentation::new()
        .accent(Color::MAGENTA)
        .slide(slide("Accent Color"));

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    pres.render(&mut ctx);
}

#[test]
fn test_render_with_slide_background() {
    let pres = Presentation::new().slide(slide("Custom Slide BG").bg(Color::BLUE).line("Content"));

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    pres.render(&mut ctx);

    // Check that slide background is rendered
    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.bg, Some(Color::BLUE));
}

#[test]
fn test_render_with_title_color() {
    let pres = Presentation::new().slide(
        slide("Colored Title")
            .title_color(Color::YELLOW)
            .line("Content"),
    );

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    pres.render(&mut ctx);
}

#[test]
fn test_render_with_content_color() {
    let pres = Presentation::new().slide(
        slide("Content Color")
            .content_color(Color::GREEN)
            .line("Green text"),
    );

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    pres.render(&mut ctx);
}

#[test]
fn test_render_with_left_alignment() {
    let pres = Presentation::new().slide(
        slide("Left Aligned")
            .align(SlideAlign::Left)
            .line("Left aligned text"),
    );

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    pres.render(&mut ctx);
}

#[test]
fn test_render_with_center_alignment() {
    let pres = Presentation::new().slide(
        slide("Center Aligned")
            .align(SlideAlign::Center)
            .line("Centered text"),
    );

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    pres.render(&mut ctx);
}

#[test]
fn test_render_with_right_alignment() {
    let pres = Presentation::new().slide(
        slide("Right Aligned")
            .align(SlideAlign::Right)
            .line("Right aligned text"),
    );

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    pres.render(&mut ctx);
}

#[test]
fn test_render_with_numbers_enabled() {
    let pres = Presentation::new()
        .numbers(true)
        .slide(slide("Slide 1"))
        .slide(slide("Slide 2"));

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    pres.render(&mut ctx);

    // Check that slide numbers appear in footer
    let mut found_digit = false;
    for x in 0..80 {
        if let Some(cell) = buffer.get(x, 23) {
            if cell.symbol.is_ascii_digit() {
                found_digit = true;
                break;
            }
        }
    }
    assert!(found_digit);
}

#[test]
fn test_render_with_numbers_disabled() {
    let pres = Presentation::new()
        .numbers(false)
        .slide(slide("Slide 1"))
        .slide(slide("Slide 2"));

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    pres.render(&mut ctx);
}

#[test]
fn test_render_with_progress_enabled() {
    let pres = Presentation::new()
        .progress(true)
        .slide(slide("Slide 1"))
        .slide(slide("Slide 2"))
        .slide(slide("Slide 3"));

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    pres.render(&mut ctx);

    // Check that progress bar appears
    let mut found_bar = false;
    for x in 0..80 {
        if let Some(cell) = buffer.get(x, 23) {
            if cell.symbol == '━' || cell.symbol == '─' {
                found_bar = true;
                break;
            }
        }
    }
    assert!(found_bar);
}

#[test]
fn test_render_with_progress_disabled() {
    let pres = Presentation::new().progress(false).slide(slide("Slide 1"));

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    pres.render(&mut ctx);
}

#[test]
fn test_render_small_buffer() {
    let pres = Presentation::new().slide(slide("Small").line("Content"));

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    pres.render(&mut ctx);
}

#[test]
fn test_render_large_buffer() {
    let pres = Presentation::new().slide(slide("Large").line("Content"));

    let mut buffer = Buffer::new(120, 40);
    let area = Rect::new(0, 0, 120, 40);
    let mut ctx = RenderContext::new(&mut buffer, area);

    pres.render(&mut ctx);
}

#[test]
fn test_render_with_offset() {
    let pres = Presentation::new().slide(slide("Offset").line("Content"));

    let mut buffer = Buffer::new(100, 40);
    let area = Rect::new(10, 5, 80, 30);
    let mut ctx = RenderContext::new(&mut buffer, area);

    pres.render(&mut ctx);
}

// =============================================================================
// Edge Cases Tests
// =============================================================================

#[test]
fn test_edge_case_empty_presentation() {
    let pres = Presentation::new();
    assert_eq!(pres.slide_count(), 0);
    assert_eq!(pres.current_index(), 0);
    assert!(pres.current_slide().is_none());
    assert!(pres.current_notes().is_none());

    // Should not panic when navigating
    let mut pres = Presentation::new();
    assert!(!pres.next_slide());
    assert!(!pres.prev());
    pres.goto(0);
    pres.first();
    pres.last();
}

#[test]
fn test_edge_case_single_slide() {
    let mut pres = Presentation::new().slide(slide("Only Slide"));

    assert_eq!(pres.slide_count(), 1);
    assert_eq!(pres.current_index(), 0);

    // Can't navigate
    assert!(!pres.next_slide());
    assert_eq!(pres.current_index(), 0);
    assert!(!pres.prev());
    assert_eq!(pres.current_index(), 0);
}

#[test]
fn test_edge_case_very_long_title() {
    let long_title = "This is a very long title that might exceed the normal rendering area and could cause issues";
    let pres = Presentation::new().title(long_title);

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    pres.render(&mut ctx);
}

#[test]
fn test_edge_case_very_long_content() {
    let long_content = "This is a very long line of content that exceeds the width of the slide and should be truncated or handled gracefully";
    let pres = Presentation::new().slide(slide("Long Content").line(long_content));

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    pres.render(&mut ctx);
}

#[test]
fn test_edge_case_many_content_lines() {
    let mut slide_builder = slide("Many Lines");
    for i in 0..50 {
        slide_builder = slide_builder.line(format!("Line {}", i));
    }

    let pres = Presentation::new().slide(slide_builder);

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    pres.render(&mut ctx);
}

#[test]
fn test_edge_case_empty_slide_title() {
    let pres = Presentation::new().slide(Slide::new(""));

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    pres.render(&mut ctx);
}

#[test]
fn test_edge_case_empty_slide_content() {
    let pres = Presentation::new().slide(Slide::new("Empty Slide"));

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    pres.render(&mut ctx);
}

#[test]
fn test_edge_case_goto_zero() {
    let mut pres = Presentation::new()
        .slide(slide("Slide 1"))
        .slide(slide("Slide 2"));

    pres.goto(1);
    assert_eq!(pres.current_index(), 1);

    pres.goto(0);
    assert_eq!(pres.current_index(), 0);
}

#[test]
fn test_edge_case_multiple_rapid_navigation() {
    let mut pres = Presentation::new()
        .slide(slide("S1"))
        .slide(slide("S2"))
        .slide(slide("S3"))
        .slide(slide("S4"))
        .slide(slide("S5"));

    pres.next_slide();
    pres.next_slide();
    pres.prev();
    pres.goto(3);
    pres.prev();
    pres.next_slide();
    pres.last();
    pres.first();

    assert_eq!(pres.current_index(), 0);
}

#[test]
fn test_edge_case_unicode_content() {
    let pres = Presentation::new().slide(
        slide("Unicode")
            .line("Hello 世界")
            .line("Привет мир")
            .line("🎉🎊🎈"),
    );

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    pres.render(&mut ctx);
}

#[test]
fn test_edge_case_special_characters_in_code() {
    let pres =
        Presentation::new().slide(slide("Special Chars").code("let x = \"\\n\\t\\r\"; // comment"));

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    pres.render(&mut ctx);
}

#[test]
fn test_edge_case_very_small_area() {
    let pres = Presentation::new().slide(slide("Test"));

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 10, 5); // Small but usable area
    let mut ctx = RenderContext::new(&mut buffer, area);

    pres.render(&mut ctx);
}

#[test]
fn test_edge_case_very_tall_area() {
    let pres = Presentation::new().slide(slide("Tall"));

    let mut buffer = Buffer::new(80, 100);
    let area = Rect::new(0, 0, 80, 100);
    let mut ctx = RenderContext::new(&mut buffer, area);

    pres.render(&mut ctx);
}

#[test]
fn test_edge_case_very_wide_area() {
    let pres = Presentation::new().slide(slide("Wide"));

    let mut buffer = Buffer::new(200, 24);
    let area = Rect::new(0, 0, 200, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    pres.render(&mut ctx);
}

// =============================================================================
// StyledView Trait Tests
// =============================================================================

#[test]
fn test_presentation_styled_view_set_id() {
    let mut pres = Presentation::new();
    StyledView::set_id(&mut pres, "test-presentation");
    assert_eq!(View::id(&pres), Some("test-presentation"));
}

#[test]
fn test_presentation_styled_view_add_class() {
    let mut pres = Presentation::new();
    StyledView::add_class(&mut pres, "fullscreen");
    StyledView::add_class(&mut pres, "dark-mode");
    assert!(StyledView::has_class(&pres, "fullscreen"));
    assert!(StyledView::has_class(&pres, "dark-mode"));
    assert_eq!(View::classes(&pres).len(), 2);
}

#[test]
fn test_presentation_styled_view_add_class_no_duplicates() {
    let mut pres = Presentation::new();
    StyledView::add_class(&mut pres, "test");
    StyledView::add_class(&mut pres, "test");
    let classes = View::classes(&pres);
    assert_eq!(classes.len(), 1);
    assert!(classes.contains(&"test".to_string()));
}

#[test]
fn test_presentation_styled_view_remove_class() {
    let mut pres = Presentation::new()
        .element_id("pres")
        .class("a")
        .class("b")
        .class("c");
    StyledView::remove_class(&mut pres, "b");
    assert!(StyledView::has_class(&pres, "a"));
    assert!(!StyledView::has_class(&pres, "b"));
    assert!(StyledView::has_class(&pres, "c"));
}

#[test]
fn test_presentation_styled_view_remove_nonexistent_class() {
    let mut pres = Presentation::new().class("test");
    StyledView::remove_class(&mut pres, "nonexistent");
    assert!(StyledView::has_class(&pres, "test"));
}

#[test]
fn test_presentation_styled_view_toggle_class_add() {
    let mut pres = Presentation::new();
    StyledView::toggle_class(&mut pres, "active");
    assert!(StyledView::has_class(&pres, "active"));
}

#[test]
fn test_presentation_styled_view_toggle_class_remove() {
    let mut pres = Presentation::new().class("active");
    StyledView::toggle_class(&mut pres, "active");
    assert!(!StyledView::has_class(&pres, "active"));
}

#[test]
fn test_presentation_styled_view_has_class() {
    let pres = Presentation::new().class("present");
    assert!(StyledView::has_class(&pres, "present"));
    assert!(!StyledView::has_class(&pres, "absent"));
}

// =============================================================================
// View Trait Tests
// =============================================================================

#[test]
fn test_presentation_view_widget_type() {
    let pres = Presentation::new();
    assert_eq!(pres.widget_type(), "Presentation");
}

#[test]
fn test_presentation_view_id_none() {
    let pres = Presentation::new();
    assert!(View::id(&pres).is_none());
}

#[test]
fn test_presentation_view_id_some() {
    let pres = Presentation::new().element_id("my-presentation");
    assert_eq!(View::id(&pres), Some("my-presentation"));
}

#[test]
fn test_presentation_view_classes_empty() {
    let pres = Presentation::new();
    assert!(View::classes(&pres).is_empty());
}

#[test]
fn test_presentation_view_classes_with_values() {
    let pres = Presentation::new().class("fullscreen").class("presenting");
    let classes = View::classes(&pres);
    assert_eq!(classes.len(), 2);
    assert!(classes.contains(&"fullscreen".to_string()));
    assert!(classes.contains(&"presenting".to_string()));
}

#[test]
fn test_presentation_view_meta() {
    let pres = Presentation::new()
        .element_id("test-id")
        .class("test-class")
        .title("Test");
    let meta = pres.meta();
    assert_eq!(meta.widget_type, "Presentation");
    assert_eq!(meta.id, Some("test-id".to_string()));
    assert!(meta.classes.contains("test-class"));
}

#[test]
fn test_presentation_view_children_default() {
    let pres = Presentation::new();
    assert!(View::children(&pres).is_empty());
}

// =============================================================================
// Clone Tests
// =============================================================================

#[test]
fn test_slide_clone() {
    let slide1 = Slide::new("Original")
        .bullet("Point 1")
        .notes("Notes")
        .bg(Color::BLUE)
        .title_color(Color::RED)
        .content_color(Color::GREEN)
        .align(SlideAlign::Left);

    let slide2 = slide1.clone();

    assert_eq!(slide1.title, slide2.title);
    assert_eq!(slide1.content.len(), slide2.content.len());
    assert_eq!(slide1.notes, slide2.notes);
    assert_eq!(slide1.bg, slide2.bg);
    assert_eq!(slide1.title_color, slide2.title_color);
    assert_eq!(slide1.content_color, slide2.content_color);
    assert_eq!(slide1.align, slide2.align);
}

// =============================================================================
// Integration Tests
// =============================================================================

#[test]
fn test_integration_full_presentation_lifecycle() {
    let mut pres = Presentation::new()
        .title("Complete Presentation")
        .author("Dr. Presenter")
        .transition(Transition::Fade)
        .numbers(true)
        .progress(true)
        .bg(Color::BLACK)
        .accent(Color::CYAN)
        .slides(vec![
            slide("Introduction")
                .bullet("Welcome")
                .bullet("Agenda")
                .notes("Start with enthusiasm"),
            slide("Main Topic")
                .line("Important concepts")
                .code("example = true")
                .notes("Explain carefully"),
            slide("Conclusion")
                .bullet("Summary")
                .bullet("Q&A")
                .notes("Leave time for questions"),
        ]);

    // Verify initial state
    assert_eq!(pres.slide_count(), 3);
    assert_eq!(pres.current_index(), 0);
    assert!(pres.current_notes().is_some()); // Title slide

    // Navigate through slides
    assert!(pres.next_slide());
    assert_eq!(pres.current_index(), 1);
    assert_eq!(pres.current_slide().unwrap().title, "Main Topic");

    // Simulate transition
    for _ in 0..10 {
        pres.tick(0.05);
    }

    // Navigate more
    assert!(pres.next_slide());
    assert_eq!(pres.current_index(), 2);
    assert_eq!(pres.current_slide().unwrap().title, "Conclusion");

    // Navigate back
    assert!(pres.prev());
    assert_eq!(pres.current_index(), 1);

    // Jump to specific slide
    pres.goto(0);
    assert_eq!(pres.current_index(), 0);

    // Jump to last
    pres.last();
    assert_eq!(pres.current_index(), 2);

    // Back to first
    pres.first();
    assert_eq!(pres.current_index(), 0);

    // Render at various states
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    pres.render(&mut ctx);

    pres.next_slide();
    pres.render(&mut ctx);

    pres.last();
    pres.render(&mut ctx);
}

#[test]
fn test_integration_navigation_boundaries() {
    let mut pres = Presentation::new()
        .slide(slide("1"))
        .slide(slide("2"))
        .slide(slide("3"));

    // Try to go before start
    assert_eq!(pres.current_index(), 0);
    assert!(!pres.prev());
    assert_eq!(pres.current_index(), 0);

    // Go to end
    while pres.next_slide() {}
    assert_eq!(pres.current_index(), 2);

    // Try to go past end
    assert!(!pres.next_slide());
    assert_eq!(pres.current_index(), 2);

    // Go back to start
    while pres.prev() {}
    assert_eq!(pres.current_index(), 0);
}

#[test]
fn test_integration_complex_slide_content() {
    let pres = Presentation::new().slide(
        slide("Complex Content")
            .line("Introduction text")
            .bullet("First major point")
            .bullet("Second major point")
            .numbered(1, "Numbered item")
            .numbered(2, "Another numbered item")
            .line("More explanation")
            .code("fn complex() {\n    // Code here\n    return true;\n}")
            .bullet("Final point")
            .notes("This slide has a bit of everything"),
    );

    let slide = pres.current_slide().unwrap();
    assert_eq!(slide.content.len(), 13); // All the content lines: 1 line + 2 bullets + 2 numbered + 1 line + 6 code (4 + 2 empty) + 1 bullet
    assert!(!slide.notes.is_empty());

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    pres.render(&mut ctx);
}

#[test]
fn test_integration_all_alignments() {
    let mut pres = Presentation::new()
        .slide(slide("Left").align(SlideAlign::Left).line("Left aligned"))
        .slide(slide("Center").align(SlideAlign::Center).line("Centered"))
        .slide(
            slide("Right")
                .align(SlideAlign::Right)
                .line("Right aligned"),
        );

    assert_eq!(pres.slide_count(), 3);

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // Render each slide
    pres.render(&mut ctx);
    pres.next_slide();
    pres.render(&mut ctx);
    pres.next_slide();
    pres.render(&mut ctx);
}

#[test]
fn test_integration_all_transitions() {
    let transitions = [
        Transition::None,
        Transition::Fade,
        Transition::SlideLeft,
        Transition::SlideRight,
        Transition::SlideUp,
        Transition::ZoomIn,
    ];

    for transition in transitions {
        let mut pres = Presentation::new()
            .slide(slide("Test"))
            .slide(slide("Next"))
            .transition(transition);

        pres.next_slide();
        pres.tick(0.1);
        pres.tick(0.1);
        pres.tick(0.1);

        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        pres.render(&mut ctx);
    }
}

// =============================================================================
