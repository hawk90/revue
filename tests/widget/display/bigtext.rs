//! BigText widget tests extracted from src/widget/display/bigtext.rs

use revue::prelude::*;

// =========================================================================
// BigText creation tests
// =========================================================================

// BigText widget tests using public API

#[test]
fn test_bigtext_creation() {
    let bt = BigText::new("Hello", 1);
    // No public getters available for private fields
    // We can only test the height method
    assert!(bt.height() > 0);
}

#[test]
fn test_tier_clamping() {
    let bt = BigText::new("Test", 10);
    assert_eq!(bt.height(), 5); // H6 uses Mini font

    let bt = BigText::new("Test", 0);
    assert_eq!(bt.height(), 5); // H1 uses Block font
}

#[test]
fn test_helper_functions() {
    let h1 = h1("Header 1");
    let h2 = h2("Header 2");
    let h3 = h3("Header 3");

    // Can't test tier directly with public API
    // But different tiers should have different heights
    assert_eq!(h1.height(), 5); // H1 uses Block font
    assert_eq!(h2.height(), 3); // H2 uses Slant font
    assert_eq!(h3.height(), 3); // H3 uses Small font
}

#[test]
fn test_builder_pattern() {
    let bt = BigText::h1("Test")
        .fg(Color::CYAN)
        .bg(Color::BLACK)
        .figlet_font(FigletFont::Slant)
        .force_figlet(true);

    // Can't test private fields with public API
    // But we can test the height method which may change based on settings
    assert!(bt.height() > 0);
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
    // Can't test font_for_tier directly with public API
    // Test that different tiers have different heights (which depends on font)
    let h1 = BigText::h1("Test");
    let h2 = BigText::h2("Test");
    let h3 = BigText::h3("Test");
    let h6 = BigText::h6("Test");

    // H1 uses configured font (Block by default) - height = font_height(Block)
    // H2 uses Slant - height = font_height(Slant)
    // H3 uses Small - height = font_height(Small)
    // H4-H6 use Mini - height = font_height(Mini)

    // Block font is tallest, Mini is shortest
    assert!(h1.height() >= h2.height());
    assert!(h2.height() >= h3.height());
    assert_eq!(h3.height(), h6.height()); // H3, H4, H5, H6 all use Mini
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
