//! Tests for MarkdownPresentation widget
//!
//! These tests verify the functionality of the markdown presentation widget
//! including slide navigation, mode switching, rendering, and builder methods.

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::utils::FigletFont;
use revue::widget::traits::RenderContext;
use revue::widget::{markdown_presentation, MarkdownPresentation, SlideContent, View, ViewMode};

fn render(pres: &MarkdownPresentation, w: u16, h: u16) -> Buffer {
    let mut buffer = Buffer::new(w, h);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, w, h));
    pres.render(&mut ctx);
    buffer
}

fn rows(buffer: &Buffer) -> Vec<String> {
    (0..buffer.height())
        .map(|y| {
            (0..buffer.width())
                .filter_map(|x| buffer.get(x, y).map(|c| c.symbol))
                .collect()
        })
        .collect()
}

fn find(buffer: &Buffer, text: &str) -> Option<(u16, u16)> {
    let first = text.chars().next()?;
    for y in 0..buffer.height() {
        for x in 0..buffer.width() {
            let matches = text
                .chars()
                .enumerate()
                .all(|(i, ch)| buffer.get(x + i as u16, y).map(|c| c.symbol) == Some(ch));
            if buffer.get(x, y).map(|c| c.symbol) == Some(first) && matches {
                return Some((x, y));
            }
        }
    }
    None
}

/// Slide mode with FIGlet titles, independent of the terminal running the
/// tests
fn slides(source: &str) -> MarkdownPresentation {
    MarkdownPresentation::new(source)
        .mode(ViewMode::Slides)
        .text_sizing(false)
}

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
    let pres = slides("# Test")
        .bg(Color::BLACK)
        .accent(Color::GREEN)
        .heading_fg(Color::CYAN)
        .numbers(false)
        .progress(false);
    assert_eq!(pres.current_mode(), ViewMode::Slides);

    let buffer = render(&pres, 40, 16);
    // bg fills the slide
    assert_eq!(buffer.get(39, 15).unwrap().bg, Some(Color::BLACK));
    // accent colors the rule under the title
    let (x, y) = find(&buffer, "────").expect("title separator");
    assert_eq!(buffer.get(x, y).unwrap().fg, Some(Color::GREEN));
    // heading_fg colors the FIGlet title
    let title_cell = (1..y)
        .flat_map(|y| (0..40).map(move |x| (x, y)))
        .map(|(x, y)| buffer.get(x, y).unwrap())
        .find(|c| c.symbol != ' ')
        .expect("FIGlet title");
    assert_eq!(title_cell.fg, Some(Color::CYAN));
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
    let buffer = render(&pres, 80, 24);
    let rows = rows(&buffer);

    assert!(rows[0].contains("Hello"), "{:?}", rows[0]);
    assert!(rows.iter().any(|r| r.contains("World")));
    // Mode badge in the top right corner
    assert_eq!(
        rows[1].chars().skip(70).take(9).collect::<String>(),
        " PREVIEW "
    );
    assert_eq!(buffer.get(71, 1).unwrap().bg, Some(Color::CYAN));
}

#[test]
fn test_render_slides() {
    let pres = slides("# Slide 1\n\nContent\n---\n# Slide 2");
    let buffer = render(&pres, 80, 24);
    let rows = rows(&buffer);

    assert!(rows.iter().any(|r| r.contains("Content")));
    // The second slide is not shown
    assert!(!rows.iter().any(|r| r.contains("Slide 2")));
    // Footer: progress bar, mode marker and slide number
    let footer = &rows[23];
    assert!(footer.contains('━'), "{footer:?}");
    assert!(footer.contains("[S]"), "{footer:?}");
    assert!(footer.ends_with("1/2 "), "{footer:?}");
}

#[test]
fn test_render_slide_numbers_and_progress_can_be_hidden() {
    let pres = slides("# A\n---\n# B").numbers(false).progress(false);
    let buffer = render(&pres, 40, 12);
    let footer = &rows(&buffer)[11];
    assert!(!footer.contains("1/2"), "{footer:?}");
    assert!(!footer.contains('━') && !footer.contains('─'), "{footer:?}");
    assert!(footer.contains("[S]"), "{footer:?}");
}

#[test]
fn test_render_progress_tracks_the_current_slide() {
    let mut pres = slides("# A\n---\n# B\n---\n# C\n---\n# D");
    pres.goto(1);
    let buffer = render(&pres, 60, 12);
    let footer = &rows(&buffer)[11];
    // 60 / 3 = 20 cells; half of them filled at slide 2 of 4
    assert_eq!(
        footer
            .chars()
            .skip(1)
            .take(20)
            .filter(|&c| c == '━')
            .count(),
        10
    );
    assert!(footer.ends_with("2/4 "), "{footer:?}");
}

#[test]
fn test_render_in_a_small_area() {
    // Narrower than the mode badge, the title rule and the footer
    for (w, h) in [(5, 3), (12, 6), (1, 1)] {
        let preview = MarkdownPresentation::new("# Title\n\nBody");
        render(&preview, w, h);
        render(&slides("# Title\n\nBody"), w, h);
    }
    let buffer = render(&slides("# T\n\nBody"), 12, 12);
    assert!(rows(&buffer).iter().any(|r| r.contains("[S]")));
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
fn test_progress() {
    let pres = MarkdownPresentation::new("# A\n---\n# B\n---\n# C\n---\n# D");
    assert!((pres.progress_value() - 0.25).abs() < 0.01);
}

// =========================================================================
// Slide titles: the first heading becomes the big title and is not
// repeated in the slide body
// =========================================================================

#[test]
fn test_slide_title_is_not_repeated_in_body() {
    let buffer = render(&slides("# Title\n\nContent here"), 60, 20);
    let text = rows(&buffer);
    assert!(text.iter().any(|r| r.contains("Content here")));
    assert!(!text.iter().any(|r| r.contains("Title")), "{text:#?}");

    let buffer = render(&slides("## Subtitle\n\nMore content"), 60, 20);
    let text = rows(&buffer);
    assert!(text.iter().any(|r| r.contains("More content")));
    assert!(!text.iter().any(|r| r.contains("Subtitle")), "{text:#?}");
}

#[test]
fn test_slide_without_heading_keeps_its_content() {
    let buffer = render(&slides("Just some content\n\nNo heading"), 60, 20);
    let rows = rows(&buffer);
    assert!(rows.iter().any(|r| r.contains("Just some content")));
    assert!(rows.iter().any(|r| r.contains("No heading")));
}

#[test]
fn test_slide_with_only_a_heading_has_no_body() {
    let buffer = render(&slides("# Title Only"), 60, 20);
    assert!(!rows(&buffer).iter().any(|r| r.contains("Title Only")));
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
// Title fonts
// =========================================================================

#[test]
fn test_text_sizing_disabled_draws_figlet_title() {
    let buffer = render(&slides("# Hi"), 40, 16);
    let (_, rule_y) = find(&buffer, "────").expect("title separator");
    // A FIGlet title spans several rows above the rule
    let title_rows = rows(&buffer)[..rule_y as usize]
        .iter()
        .filter(|r| !r.trim().is_empty())
        .count();
    assert!(title_rows >= 3, "{title_rows}");
}

#[test]
fn test_figlet_font() {
    let block = render(&slides("# Hi"), 60, 16);
    let small = render(&slides("# Hi").figlet_font(FigletFont::Small), 60, 16);
    assert_ne!(rows(&block), rows(&small));
}

// =========================================================================
// Link and code colors
// =========================================================================

#[test]
fn test_link_fg() {
    let pres = MarkdownPresentation::new("[site](https://example.com)").link_fg(Color::RED);
    let buffer = render(&pres, 40, 6);
    let (x, y) = find(&buffer, "site").expect("link text");
    assert_eq!(buffer.get(x, y).unwrap().fg, Some(Color::RED));
}

#[test]
#[ignore = "BUG: Markdown styles the first plain paragraph as a blockquote, overriding code_fg"]
fn test_code_fg() {
    let pres = MarkdownPresentation::new("run `make` now").code_fg(Color::GREEN);
    let buffer = render(&pres, 40, 6);
    let (x, y) = find(&buffer, "make").expect("inline code");
    assert_eq!(buffer.get(x, y).unwrap().fg, Some(Color::GREEN));
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
// Clone tests
// =========================================================================

#[test]
fn test_clone() {
    let pres1 = slides("# Test\n\nBody").bg(Color::BLACK).accent(Color::RED);
    let pres2 = pres1.clone();

    assert_eq!(pres1.source(), pres2.source());
    assert_eq!(pres1.current_mode(), pres2.current_mode());
    assert_eq!(rows(&render(&pres1, 40, 12)), rows(&render(&pres2, 40, 12)));
    assert_eq!(
        render(&pres2, 40, 12).get(0, 0).unwrap().bg,
        Some(Color::BLACK)
    );
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
        .mode(ViewMode::Slides)
        .text_sizing(false);

    assert_eq!(pres.current_mode(), ViewMode::Slides);
    let buffer = render(&pres, 40, 12);
    let footer = &rows(&buffer)[11];
    assert!(footer.ends_with("1/1 "), "{footer:?}");
    // A single slide is fully done: the whole bar is filled
    assert!(footer[footer.char_indices().nth(1).unwrap().0..].starts_with(&"━".repeat(13)));
    assert_eq!(buffer.get(1, 11).unwrap().fg, Some(Color::MAGENTA));
}
