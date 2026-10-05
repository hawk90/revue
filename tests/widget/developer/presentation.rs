//! Presentation Mode widget tests
//!
//! Tests for public API of Presentation widget

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{presentation, slide, Presentation, Slide, SlideAlign, Transition};

// =========================================================================
// Presentation creation tests
// =========================================================================

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
fn test_slide_bg() {
    let s = Slide::new("Test").bg(Color::BLACK);
    assert_eq!(s.bg, Some(Color::BLACK));
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
    let slides = vec![slide("A"), slide("B"), slide("C")];
    let pres = Presentation::new().slides(slides);
    assert_eq!(pres.slide_count(), 3);
}

#[test]
fn test_presentation_slides_empty() {
    let pres = Presentation::new().slides(vec![]);
    assert_eq!(pres.slide_count(), 0);
}

// =========================================================================
// Navigation tests
// =========================================================================

#[test]
fn test_navigation() {
    let mut pres = Presentation::new()
        .slide(slide("Slide 1"))
        .slide(slide("Slide 2"))
        .slide(slide("Slide 3"));

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
        .slide(slide("A"))
        .slide(slide("B"))
        .slide(slide("C"));

    pres.goto(1);
    assert_eq!(pres.current_index(), 1);
}

#[test]
fn test_goto_out_of_bounds() {
    let mut pres = Presentation::new().slide(slide("A")).slide(slide("B"));

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
    let mut pres = Presentation::new().slide(slide("A")).slide(slide("B"));
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
        .slide(slide("A"))
        .slide(slide("B"))
        .slide(slide("C"));
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
    let pres = Presentation::new().slide(slide("First"));
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
    let mut pres = Presentation::new().slide(slide("A")).slide(slide("B"));
    pres.goto(1);
    let slide = pres.current_slide();
    assert!(slide.is_some());
    assert_eq!(slide.unwrap().title, "B");
}

#[test]
fn test_current_notes() {
    let pres = Presentation::new().slide(slide("Test").notes("Speaker notes"));
    let notes = pres.current_notes();
    assert!(notes.is_some());
    assert_eq!(notes.unwrap(), "Speaker notes");
}

#[test]
fn test_current_notes_no_notes() {
    let pres = Presentation::new().slide(slide("Test"));
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
    let mut pres = Presentation::new().slide(slide("Only"));
    assert!(!pres.next_slide()); // Already at end
}

#[test]
fn test_prev_empty() {
    let mut pres = Presentation::new();
    assert!(!pres.prev());
}

#[test]
fn test_prev_at_start() {
    let mut pres = Presentation::new().slide(slide("A"));
    assert!(!pres.prev()); // Already at 0
}

// =========================================================================
// Builder chain tests
// =========================================================================

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
// Enum tests
// =========================================================================

#[test]
fn test_transition_default() {
    assert_eq!(Transition::default(), Transition::None);
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

// =========================================================================
// Helper function tests
// =========================================================================

#[test]
fn test_presentation_helper() {
    let pres = presentation();
    assert!(pres.slide_count() == 0);
}

#[test]
fn test_slide_helper() {
    let s = slide("Title");
    assert_eq!(s.title, "Title");
}

// =========================================================================
// Rendering
// =========================================================================

fn render_sized(pres: &Presentation, width: u16, height: u16) -> Buffer {
    let mut buffer = Buffer::new(width, height);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, width, height));
    pres.render(&mut ctx);
    buffer
}

fn render(pres: &Presentation) -> Buffer {
    render_sized(pres, 80, 24)
}

fn row(buffer: &Buffer, y: u16) -> String {
    (0..buffer.width())
        .map(|x| buffer.get(x, y).unwrap().symbol)
        .collect()
}

fn screen(buffer: &Buffer) -> String {
    (0..buffer.height())
        .map(|y| row(buffer, y))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Foreground color of the first cell on row `y` showing `ch`.
fn fg_of(buffer: &Buffer, y: u16, ch: char) -> Option<Color> {
    (0..buffer.width())
        .map(|x| buffer.get(x, y).unwrap())
        .find(|c| c.symbol == ch)
        .and_then(|c| c.fg)
}

#[test]
fn test_slide_defaults() {
    let s = Slide::new("Test");
    assert!(s.bg.is_none());
    assert!(s.notes.is_empty());
    assert!(s.content.is_empty());
    assert_eq!(s.align, SlideAlign::Center);
}

#[test]
fn test_presentation_creation() {
    let pres = Presentation::new().title("Test").author("Author");
    assert_eq!(pres.slide_count(), 0);
    // The title slide shows title and author
    let text = screen(&render(&pres));
    assert!(text.contains("Test"));
    assert!(text.contains("Author"));
}

#[test]
fn test_render_empty_presentation() {
    let buffer = render(&Presentation::new());
    assert!(screen(&buffer).contains("Press → or Space to start"));
    // No slides: no slide number or progress bar
    assert!(!row(&buffer, 23).contains('/'));
    assert!(!row(&buffer, 23).contains('━'));
}

#[test]
fn test_render_with_content() {
    let mut pres = Presentation::new().slide(slide("Content").line("Content here"));
    // Out of range: stays on the only slide
    pres.goto(1);
    let text = screen(&render(&pres));
    assert!(text.contains("Content here"));
    assert!(!text.contains("Press → or Space to start"));
}

#[test]
fn test_render_slide_align() {
    let left = Presentation::new().slide(slide("T").line("abc").align(SlideAlign::Left));
    assert!(row(&render(&left), 6).starts_with("  abc"));

    let right = Presentation::new().slide(slide("T").line("abc").align(SlideAlign::Right));
    assert!(row(&render(&right), 6).ends_with("abc  "));

    let center = Presentation::new().slide(slide("T").line("abc"));
    let r = row(&render(&center), 6);
    assert_eq!(r.find("abc"), Some((80 - 3) / 2));
}

#[test]
fn test_render_slide_colors() {
    let pres = Presentation::new().slide(
        slide("Title")
            .line("body")
            .bg(Color::BLUE)
            .title_color(Color::YELLOW)
            .content_color(Color::GREEN),
    );
    let buffer = render(&pres);
    assert_eq!(buffer.get(0, 0).unwrap().bg, Some(Color::BLUE));
    assert_eq!(fg_of(&buffer, 2, 'T'), Some(Color::YELLOW));
    assert_eq!(fg_of(&buffer, 6, 'b'), Some(Color::GREEN));
}

#[test]
fn test_presentation_numbers() {
    let pres = || Presentation::new().slide(slide("A")).slide(slide("B"));
    assert!(row(&render(&pres()), 23).contains("1/2"));
    assert!(row(&render(&pres().numbers(true)), 23).contains("1/2"));
    assert!(!row(&render(&pres().numbers(false)), 23).contains("1/2"));

    let mut second = pres();
    second.next_slide();
    assert!(row(&render(&second), 23).contains("2/2"));
}

#[test]
fn test_presentation_progress() {
    let pres = || Presentation::new().slide(slide("A")).slide(slide("B"));
    assert!(row(&render(&pres()), 23).contains('━'));
    assert!(row(&render(&pres().progress(true)), 23).contains('━'));
    assert!(!row(&render(&pres().progress(false)), 23).contains('━'));
}

#[test]
fn test_presentation_progress_fills_with_position() {
    let mut pres = Presentation::new()
        .slide(slide("A"))
        .slide(slide("B"))
        .numbers(false);
    let filled = |p: &Presentation| row(&render(p), 23).matches('━').count();
    let first = filled(&pres);
    pres.last();
    let last = filled(&pres);
    assert!(first < last);
}

#[test]
fn test_presentation_bg() {
    let pres = Presentation::new().bg(Color::BLACK);
    assert_eq!(render(&pres).get(0, 0).unwrap().bg, Some(Color::BLACK));
}

#[test]
fn test_presentation_accent() {
    let pres = Presentation::new()
        .slide(slide("Title"))
        .accent(Color::MAGENTA);
    let buffer = render(&pres);
    // The title separator and the filled progress bar use the accent
    assert_eq!(fg_of(&buffer, 4, '─'), Some(Color::MAGENTA));
    assert_eq!(fg_of(&buffer, 23, '━'), Some(Color::MAGENTA));
}

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
        .timer(30)
        .slide(slide("One"));
    let buffer = render(&pres);
    let text = screen(&buffer);
    assert!(text.contains("Title"));
    assert!(text.contains("Author"));
    assert!(!row(&buffer, 23).contains("1/1"));
    assert!(!row(&buffer, 23).contains('━'));
    assert_eq!(buffer.get(0, 0).unwrap().bg, Some(Color::BLACK));
}

#[test]
fn test_presentation_render_small_area() {
    let title = Presentation::new()
        .title("A long presentation title")
        .slide(slide("A slide title").line("content"));
    let content = Presentation::new().slide(slide("A slide title").line("content"));
    for (w, h) in [(0, 0), (1, 1), (3, 2), (5, 3), (10, 4)] {
        render_sized(&title, w, h);
        render_sized(&content, w, h);
    }
    // Still draws the footer when it fits
    assert!(row(&render_sized(&content, 30, 4), 3).contains("1/1"));
}

#[test]
fn test_presentation_title_slide_does_not_hide_first_slide() {
    let mut pres = Presentation::new()
        .title("Deck")
        .slide(slide("Intro"))
        .slide(slide("Next"));
    let mut seen_intro = false;
    for i in 0..pres.slide_count() {
        pres.goto(i);
        seen_intro |= screen(&render(&pres)).contains("Intro");
    }
    assert!(seen_intro);
}

#[test]
fn test_presentation_title_slide_comes_before_slide_zero() {
    let mut pres = Presentation::new()
        .title("Deck")
        .slide(slide("Intro"))
        .slide(slide("Next"));
    let hint = "Press → or Space to start";
    assert!(screen(&render(&pres)).contains(hint));

    // Leaving the title slide shows slide 0
    assert!(pres.next_slide());
    assert_eq!(pres.current_index(), 0);
    let text = screen(&render(&pres));
    assert!(text.contains("Intro"));
    assert!(!text.contains(hint));

    assert!(pres.next_slide());
    assert_eq!(pres.current_index(), 1);

    // Going back from slide 0 returns to the title slide, and no further
    assert!(pres.prev());
    assert!(pres.prev());
    assert!(screen(&render(&pres)).contains(hint));
    assert!(!pres.prev());
}

#[test]
fn test_presentation_timer_is_shown() {
    let pres = || Presentation::new().slide(slide("A"));
    assert_ne!(screen(&render(&pres().timer(60))), screen(&render(&pres())));
}

#[test]
fn test_presentation_timer_counts_down_with_tick() {
    let mut pres = Presentation::new().slide(slide("A")).timer(90);
    assert!(row(&render(&pres), 23).contains("01:30"));
    pres.tick(31.0);
    assert!(row(&render(&pres), 23).contains("00:59"));
    // Stops at zero
    pres.tick(120.0);
    assert!(row(&render(&pres), 23).contains("00:00"));
}

#[test]
#[ignore = "BUG: Presentation transitions have no visible effect (transition progress is never rendered)"]
fn test_presentation_transition_is_rendered() {
    let mut pres = Presentation::new()
        .slide(slide("A").line("first"))
        .slide(slide("B").line("second"))
        .transition(Transition::Fade);
    pres.next_slide();
    let mid = screen(&render(&pres));
    pres.tick(1.0);
    let done = screen(&render(&pres));
    assert_ne!(mid, done);
}
