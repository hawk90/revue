//! BigText widget tests extracted from src/widget/display/bigtext.rs

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::utils::figlet::FigletFont;
use revue::widget::display::bigtext::{bigtext, h1, h2, h3, BigText};
use revue::widget::traits::{RenderContext, View};

// =========================================================================
// BigText creation tests
// =========================================================================

#[test]
fn test_bigtext_creation() {
    let bt = BigText::new("Hello", 1);
    assert_eq!(bt.get_text(), "Hello");
    assert_eq!(bt.get_tier(), 1);
}

#[test]
fn test_tier_clamping() {
    let bt = BigText::new("Test", 10);
    assert_eq!(bt.get_tier(), 6);

    let bt = BigText::new("Test", 0);
    assert_eq!(bt.get_tier(), 1);
}

#[test]
fn test_helper_functions() {
    let h1 = h1("Header 1");
    assert_eq!(h1.get_tier(), 1);

    let h2 = h2("Header 2");
    assert_eq!(h2.get_tier(), 2);

    let h3 = h3("Header 3");
    assert_eq!(h3.get_tier(), 3);
}

#[test]
fn test_builder_pattern() {
    let bt = BigText::h1("Test")
        .fg(Color::CYAN)
        .bg(Color::BLACK)
        .figlet_font(FigletFont::Slant)
        .force_figlet(true);

    assert_eq!(bt.get_fg(), Some(Color::CYAN));
    assert_eq!(bt.get_bg(), Some(Color::BLACK));
    assert_eq!(bt.get_figlet_font(), FigletFont::Slant);
    assert!(bt.get_force_figlet());
}

#[test]
fn test_render_figlet() {
    let mut buffer = Buffer::new(80, 10);
    let area = Rect::new(0, 0, 80, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let bt = BigText::h1("Hi").force_figlet(true);
    bt.render(&mut ctx);

    // Should have rendered something (Figlet art)
    // Check that at least some non-space cells exist
    let mut found_content = false;
    for y in 0..10 {
        for x in 0..80 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol != ' ' {
                    found_content = true;
                    break;
                }
            }
        }
    }
    assert!(found_content, "Figlet should render some content");
}

#[test]
fn test_font_for_tier() {
    let bt = BigText::h1("Test").figlet_font(FigletFont::Block);
    assert_eq!(bt.get_font_for_tier(), FigletFont::Block);

    let bt = BigText::h2("Test");
    assert_eq!(bt.get_font_for_tier(), FigletFont::Slant);

    let bt = BigText::h3("Test");
    assert_eq!(bt.get_font_for_tier(), FigletFont::Small);

    let bt = BigText::h6("Test");
    assert_eq!(bt.get_font_for_tier(), FigletFont::Mini);
}

#[test]
fn test_empty_text() {
    let mut buffer = Buffer::new(80, 10);
    let area = Rect::new(0, 0, 80, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let bt = BigText::h1("");
    bt.render(&mut ctx);

    // Should not crash, and should not render anything
}

#[test]
fn test_text_sizing_rendering() {
    let mut buffer = Buffer::new(80, 10);
    let area = Rect::new(0, 0, 80, 10);

    // Simulate text sizing support for testing
    // render_text_sizing writes an escape sequence to the buffer
    let bt = BigText::h1("Test");

    // Call render_text_sizing directly (bypasses the is_supported check)
    bt.test_render_text_sizing(&mut RenderContext::new(&mut buffer, area));

    // Verify that a sequence was registered
    assert_eq!(buffer.sequences().len(), 1);

    // Verify the sequence contains OSC 66 marker
    let seq = &buffer.sequences()[0];
    assert!(seq.contains("\x1b]66;"), "Should contain OSC 66 marker");
    assert!(seq.contains("Test"), "Should contain the text");

    // Verify the first cell has a sequence_id
    let first_cell = buffer.get(0, 0).unwrap();
    assert!(
        first_cell.sequence_id.is_some(),
        "First cell should have sequence_id"
    );

    // Verify continuation cells
    let cont_cell = buffer.get(1, 0).unwrap();
    assert!(
        cont_cell.is_continuation(),
        "Adjacent cells should be continuations"
    );
}

#[test]
fn test_text_sizing_height() {
    let bt = BigText::h1("Test");
    // When text sizing is not supported, height is figlet height
    // When supported, height is TextSizing::height() = 2
    assert!(bt.height() > 0);
}
