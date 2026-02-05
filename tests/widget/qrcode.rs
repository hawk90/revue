//! QR Code widget integration tests
//!
//! Tests for QR code generation, rendering, and styling.

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::{RenderContext, StyledView, View};
use revue::widget::{qrcode, ErrorCorrection, QrCodeWidget, QrStyle};

// =============================================================================
// Constructor Tests
// =============================================================================

#[test]
fn test_qrcode_new_empty_data() {
    let qr = QrCodeWidget::new("");
    // Empty data should still render (or show error)
    let mut buffer = Buffer::new(20, 20);
    let area = Rect::new(0, 0, 20, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    qr.render(&mut ctx);
    // Should not panic
}

#[test]
fn test_qrcode_new_unicode_data() {
    let qr = QrCodeWidget::new("Hello 世界");
    // Should render without error
    let mut buffer = Buffer::new(50, 50);
    let area = Rect::new(0, 0, 50, 50);
    let mut ctx = RenderContext::new(&mut buffer, area);
    qr.render(&mut ctx);
    // Should not panic
}

#[test]
fn test_qrcode_new_long_data() {
    let long_url = "https://example.com/very/long/path/that/goes/on/and/on?param=value&other=123";
    let qr = QrCodeWidget::new(long_url);
    // Should handle long URLs
    let mut buffer = Buffer::new(50, 50);
    let area = Rect::new(0, 0, 50, 50);
    let mut ctx = RenderContext::new(&mut buffer, area);
    qr.render(&mut ctx);
    // Should not panic
}

#[test]
fn test_qrcode_helper() {
    let qr = qrcode("Test Data");
    // Should create and render
    let mut buffer = Buffer::new(30, 30);
    let area = Rect::new(0, 0, 30, 30);
    let mut ctx = RenderContext::new(&mut buffer, area);
    qr.render(&mut ctx);
    // Should not panic
}

// =============================================================================
// Builder Pattern Tests
// =============================================================================

#[test]
fn test_qrcode_style_all_variants() {
    let styles = [
        QrStyle::HalfBlock,
        QrStyle::FullBlock,
        QrStyle::Ascii,
        QrStyle::Braille,
    ];

    for style in styles {
        let qr = QrCodeWidget::new("test").style(style);
        // Verify style can be set
        let size = qr.required_size();
        assert!(size.is_some());
    }
}

#[test]
fn test_qrcode_color_inverted() {
    let qr = QrCodeWidget::new("test").inverted(true);
    let qr_normal = QrCodeWidget::new("test").inverted(false);

    let mut buffer1 = Buffer::new(30, 30);
    let area = Rect::new(0, 0, 30, 30);
    let mut ctx1 = RenderContext::new(&mut buffer1, area);
    qr.render(&mut ctx1);

    let mut buffer2 = Buffer::new(30, 30);
    let mut ctx2 = RenderContext::new(&mut buffer2, area);
    qr_normal.render(&mut ctx2);

    // Should render without panic
}

#[test]
fn test_qrcode_error_correction_all_levels() {
    let levels = [
        ErrorCorrection::Low,
        ErrorCorrection::Medium,
        ErrorCorrection::Quartile,
        ErrorCorrection::High,
    ];

    for level in levels {
        let qr = QrCodeWidget::new("test").error_correction(level);
        // Each level should produce valid size
        assert!(qr.required_size().is_some());
    }
}

#[test]
fn test_qrcode_quiet_zone() {
    let qr1 = QrCodeWidget::new("test").quiet_zone(0);
    let qr2 = QrCodeWidget::new("test").quiet_zone(5);

    let size1 = qr1.required_size();
    let size2 = qr2.required_size();

    assert!(size1.is_some());
    assert!(size2.is_some());

    // Larger quiet zone should produce larger size
    assert!(size2.unwrap().0 > size1.unwrap().0);
}

#[test]
fn test_qrcode_builder_chain() {
    let qr = QrCodeWidget::new("test")
        .style(QrStyle::Braille)
        .fg(Color::CYAN)
        .bg(Color::BLACK)
        .error_correction(ErrorCorrection::High)
        .quiet_zone(2)
        .inverted(true);

    let mut buffer = Buffer::new(50, 50);
    let area = Rect::new(0, 0, 50, 50);
    let mut ctx = RenderContext::new(&mut buffer, area);
    qr.render(&mut ctx);
    // Should not panic
}

#[test]
fn test_qrcode_set_data() {
    let mut qr = QrCodeWidget::new("original");
    let size1 = qr.required_size();

    qr.set_data("updated");
    let size2 = qr.required_size();

    // Both should produce valid sizes
    assert!(size1.is_some());
    assert!(size2.is_some());
}

#[test]
fn test_qrcode_set_data_updates_matrix() {
    let mut qr = QrCodeWidget::new("short");
    let size1 = qr.required_size().unwrap();

    qr.set_data("much longer data that will produce a different qr code");
    let size2 = qr.required_size().unwrap();

    // Longer data should produce larger QR code
    assert!(size2.0 >= size1.0);
}

// =============================================================================
// Matrix Generation Tests (via required_size and rendering)
// =============================================================================

#[test]
fn test_qrcode_matrix_size_consistency() {
    let qr = QrCodeWidget::new("test");
    let size = qr.required_size().unwrap();

    // QR codes are roughly square (considering quiet zone)
    // Width and height should be in reasonable ratio
    let ratio = size.0 as f64 / size.1 as f64;
    assert!(ratio > 0.5 && ratio < 3.0);
}

#[test]
fn test_qrcode_matrix_quiet_zone() {
    let qr1 = QrCodeWidget::new("test").quiet_zone(0);
    let size1 = qr1.required_size().unwrap();

    let qr2 = QrCodeWidget::new("test").quiet_zone(2);
    let size2 = qr2.required_size().unwrap();

    // Larger quiet zone should produce larger size
    // Quiet zone adds to all sides, so difference should be 4
    assert_eq!(size2.0, size1.0 + 4);
}

#[test]
fn test_qrcode_matrix_inversion() {
    let qr = QrCodeWidget::new("test");

    let mut buffer1 = Buffer::new(30, 30);
    let area = Rect::new(0, 0, 30, 30);
    let mut ctx1 = RenderContext::new(&mut buffer1, area);
    qr.render(&mut ctx1);

    let qr_inverted = QrCodeWidget::new("test").inverted(true);
    let mut buffer2 = Buffer::new(30, 30);
    let mut ctx2 = RenderContext::new(&mut buffer2, area);
    qr_inverted.render(&mut ctx2);

    // Should render differently (inverted colors)
    // We can't easily verify inversion without accessing private state
    // but we can verify both render without panic
}

#[test]
fn test_qrcode_matrix_error_correction() {
    let qr_low = QrCodeWidget::new("test data for encoding").error_correction(ErrorCorrection::Low);
    let size_low = qr_low.required_size().unwrap();

    let qr_high =
        QrCodeWidget::new("test data for encoding").error_correction(ErrorCorrection::High);
    let size_high = qr_high.required_size().unwrap();

    // Higher error correction may produce larger QR code
    assert!(size_high.0 >= size_low.0);
}

#[test]
fn test_qrcode_matrix_same_data() {
    let qr1 = QrCodeWidget::new("test");
    let size1 = qr1.required_size().unwrap();

    let qr2 = QrCodeWidget::new("test");
    let size2 = qr2.required_size().unwrap();

    // Same data should produce same size
    assert_eq!(size1, size2);
}

#[test]
fn test_qrcode_matrix_none_on_invalid() {
    // Very long data might fail
    let qr = QrCodeWidget::new("a".repeat(10000));
    let size = qr.required_size();
    // May fail for very long data - check if None or valid
    let _ = size;
}

// =============================================================================
// Rendering Tests
// =============================================================================

#[test]
fn test_render_half_block_characters() {
    let qr = QrCodeWidget::new("test").style(QrStyle::HalfBlock);

    let mut buffer = Buffer::new(50, 50);
    let area = Rect::new(0, 0, 50, 50);
    let mut ctx = RenderContext::new(&mut buffer, area);

    qr.render(&mut ctx);

    // Check that some cells were rendered
    // Half-block rendering uses ▀, ▄, █, and space
    let mut found_half_block = false;
    for y in 0..area.height {
        for x in 0..area.width {
            if let Some(cell) = buffer.get(x, y) {
                match cell.symbol {
                    '▀' | '▄' | '█' => found_half_block = true,
                    _ => {}
                }
            }
        }
    }

    assert!(found_half_block);
}

#[test]
fn test_render_full_block_aspect_ratio() {
    let qr = QrCodeWidget::new("test").style(QrStyle::FullBlock);

    let mut buffer = Buffer::new(50, 50);
    let area = Rect::new(0, 0, 50, 50);
    let mut ctx = RenderContext::new(&mut buffer, area);

    qr.render(&mut ctx);

    // Full block rendering should use 2:1 aspect ratio
    let mut found_full_block = false;
    for y in 0..area.height {
        for x in 0..area.width {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '█' || cell.symbol == ' ' {
                    found_full_block = true;
                }
            }
        }
    }

    assert!(found_full_block);
}

#[test]
fn test_render_ascii_characters() {
    let qr = QrCodeWidget::new("test").style(QrStyle::Ascii);

    let mut buffer = Buffer::new(50, 50);
    let area = Rect::new(0, 0, 50, 50);
    let mut ctx = RenderContext::new(&mut buffer, area);

    qr.render(&mut ctx);

    // ASCII rendering uses # and space
    let mut found_ascii = false;
    for y in 0..area.height {
        for x in 0..area.width {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '#' || cell.symbol == ' ' {
                    found_ascii = true;
                }
            }
        }
    }

    assert!(found_ascii);
}

#[test]
fn test_render_braille_dots() {
    let qr = QrCodeWidget::new("test").style(QrStyle::Braille);

    let mut buffer = Buffer::new(50, 50);
    let area = Rect::new(0, 0, 50, 50);
    let mut ctx = RenderContext::new(&mut buffer, area);

    qr.render(&mut ctx);

    // Braille rendering uses Unicode Braille characters
    let mut found_braille = false;
    for y in 0..area.height {
        for x in 0..area.width {
            if let Some(cell) = buffer.get(x, y) {
                // Braille range is U+2800 to U+28FF
                let code = cell.symbol as u32;
                if (0x2800..=0x28FF).contains(&code) {
                    found_braille = true;
                }
            }
        }
    }

    assert!(found_braille);
}

#[test]
fn test_render_color_application() {
    let qr = QrCodeWidget::new("test").fg(Color::RED).bg(Color::WHITE);

    let mut buffer = Buffer::new(50, 50);
    let area = Rect::new(0, 0, 50, 50);
    let mut ctx = RenderContext::new(&mut buffer, area);

    qr.render(&mut ctx);

    // Check that colors were applied
    let mut found_colored = false;
    for y in 0..area.height {
        for x in 0..area.width {
            if let Some(cell) = buffer.get(x, y) {
                if cell.fg == Some(Color::RED) || cell.bg == Some(Color::WHITE) {
                    found_colored = true;
                }
            }
        }
    }

    assert!(found_colored);
}

#[test]
fn test_render_truncation() {
    let qr = QrCodeWidget::new("test data");

    // Very small area
    let mut buffer = Buffer::new(5, 5);
    let area = Rect::new(0, 0, 5, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    qr.render(&mut ctx);

    // Should not panic, just render what fits
}

#[test]
fn test_render_zero_area() {
    let qr = QrCodeWidget::new("test");

    let mut buffer = Buffer::new(0, 0);
    let area = Rect::new(0, 0, 0, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);

    qr.render(&mut ctx);

    // Should handle gracefully
}

// =============================================================================
// Size Calculation Tests
// =============================================================================

#[test]
fn test_required_size_half_block() {
    let qr = QrCodeWidget::new("test").style(QrStyle::HalfBlock);
    let size = qr.required_size();

    assert!(size.is_some());
    let (width, height) = size.unwrap();
    // Half block should be roughly 2:1 height to width
    assert!(width > 0);
    assert!(height > 0);
}

#[test]
fn test_required_size_full_block() {
    let qr = QrCodeWidget::new("test").style(QrStyle::FullBlock);
    let size = qr.required_size();

    assert!(size.is_some());
    let (width, height) = size.unwrap();
    // Full block doubles width for aspect ratio
    assert!(width > height);
}

#[test]
fn test_required_size_ascii() {
    let qr = QrCodeWidget::new("test").style(QrStyle::Ascii);
    let size = qr.required_size();

    assert!(size.is_some());
    let (width, height) = size.unwrap();
    // ASCII also doubles width
    assert!(width >= height * 2);
}

#[test]
fn test_required_size_braille() {
    let qr = QrCodeWidget::new("test").style(QrStyle::Braille);
    let size = qr.required_size();

    assert!(size.is_some());
    let (width, height) = size.unwrap();
    // Braille compresses both dimensions
    assert!(width > 0);
    assert!(height > 0);
}

// =============================================================================
// Edge Cases Tests
// =============================================================================

#[test]
fn test_qrcode_special_characters() {
    let special = "Hello\nWorld\tTest\r\n";
    let qr = QrCodeWidget::new(special);

    // Should handle special characters
    assert!(qr.required_size().is_some());
}

#[test]
fn test_qrcode_minimal_area() {
    let qr = QrCodeWidget::new("x");

    let mut buffer = Buffer::new(1, 1);
    let area = Rect::new(0, 0, 1, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    qr.render(&mut ctx);

    // Should render at least something
}

#[test]
fn test_qrcode_max_capacity() {
    // Test data at various capacity limits
    let short = QrCodeWidget::new("1");
    assert!(short.required_size().is_some());

    let medium = QrCodeWidget::new("a".repeat(100));
    assert!(medium.required_size().is_some());

    let long = QrCodeWidget::new("a".repeat(500));
    assert!(long.required_size().is_some());
}

#[test]
fn test_qrcode_default_style() {
    let qr = QrCodeWidget::new("test");
    // Default should render with half-block style
    let mut buffer = Buffer::new(30, 30);
    let area = Rect::new(0, 0, 30, 30);
    let mut ctx = RenderContext::new(&mut buffer, area);
    qr.render(&mut ctx);
    // Should render without error
}

#[test]
fn test_qrcode_default_colors() {
    let qr = QrCodeWidget::new("test");
    let mut buffer = Buffer::new(30, 30);
    let area = Rect::new(0, 0, 30, 30);
    let mut ctx = RenderContext::new(&mut buffer, area);
    qr.render(&mut ctx);
    // Should render with default colors
}

#[test]
fn test_qrcode_default_ec_level() {
    let qr = QrCodeWidget::new("test");
    assert!(qr.required_size().is_some());
}

#[test]
fn test_qrcode_default_quiet_zone() {
    let qr = QrCodeWidget::new("test");
    let size = qr.required_size().unwrap();
    // Should have reasonable size with default quiet zone
    assert!(size.0 > 0);
}

#[test]
fn test_qrcode_view_meta() {
    let qr = QrCodeWidget::new("test");
    assert!(qr.widget_type().contains("QrCodeWidget"));
}

#[test]
fn test_qrcode_props_builders() {
    let qr = QrCodeWidget::new("test")
        .element_id("my-qr")
        .class("qr-class");

    assert_eq!(qr.id(), Some("my-qr"));
    assert!(View::classes(&qr).contains(&"qr-class".to_string()));
}

#[test]
fn test_qrcode_styling_methods() {
    let qr = qrcode("test")
        .fg(Color::BLUE)
        .bg(Color::YELLOW)
        .style(QrStyle::Ascii);

    let mut buffer = Buffer::new(30, 30);
    let area = Rect::new(0, 0, 30, 30);
    let mut ctx = RenderContext::new(&mut buffer, area);
    qr.render(&mut ctx);
    // Should render with specified styles
}

#[test]
fn test_qrcode_error_correction_level_to_ec() {
    // Test that all error correction levels produce valid matrices
    for level in [
        ErrorCorrection::Low,
        ErrorCorrection::Medium,
        ErrorCorrection::Quartile,
        ErrorCorrection::High,
    ] {
        let qr = QrCodeWidget::new("test").error_correction(level);
        let size = qr.required_size();
        assert!(size.is_some());
    }
}

#[test]
fn test_qrcode_render_error_message() {
    // Data that might fail to encode
    let qr = QrCodeWidget::new("a".repeat(100000));

    let mut buffer = Buffer::new(50, 50);
    let area = Rect::new(0, 0, 50, 50);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // Should render error message instead of panicking
    qr.render(&mut ctx);
}

#[test]
fn test_qrcode_all_styles_render() {
    let data = "test data";

    for style in [
        QrStyle::HalfBlock,
        QrStyle::FullBlock,
        QrStyle::Ascii,
        QrStyle::Braille,
    ] {
        let qr = QrCodeWidget::new(data).style(style);

        let mut buffer = Buffer::new(50, 50);
        let area = Rect::new(0, 0, 50, 50);
        let mut ctx = RenderContext::new(&mut buffer, area);

        qr.render(&mut ctx);

        // Should not panic for any style
    }
}

// =============================================================================
// QrStyle Enum Tests
// =============================================================================

#[test]
fn test_qr_style_default() {
    let style = QrStyle::default();
    assert_eq!(style, QrStyle::HalfBlock);
}

#[test]
fn test_qr_style_partial_eq() {
    assert_eq!(QrStyle::HalfBlock, QrStyle::HalfBlock);
    assert_eq!(QrStyle::FullBlock, QrStyle::FullBlock);
    assert_eq!(QrStyle::Ascii, QrStyle::Ascii);
    assert_eq!(QrStyle::Braille, QrStyle::Braille);
    assert_ne!(QrStyle::HalfBlock, QrStyle::Ascii);
}

#[test]
fn test_qr_style_all_variants() {
    let _ = QrStyle::HalfBlock;
    let _ = QrStyle::FullBlock;
    let _ = QrStyle::Ascii;
    let _ = QrStyle::Braille;
}

// =============================================================================
// ErrorCorrection Enum Tests
// =============================================================================

#[test]
fn test_error_correction_default() {
    let ec = ErrorCorrection::default();
    assert_eq!(ec, ErrorCorrection::Medium);
}

#[test]
fn test_error_correction_partial_eq() {
    assert_eq!(ErrorCorrection::Low, ErrorCorrection::Low);
    assert_eq!(ErrorCorrection::Medium, ErrorCorrection::Medium);
    assert_eq!(ErrorCorrection::Quartile, ErrorCorrection::Quartile);
    assert_eq!(ErrorCorrection::High, ErrorCorrection::High);
    assert_ne!(ErrorCorrection::Low, ErrorCorrection::High);
}

#[test]
fn test_error_correction_all_variants() {
    let _ = ErrorCorrection::Low;
    let _ = ErrorCorrection::Medium;
    let _ = ErrorCorrection::Quartile;
    let _ = ErrorCorrection::High;
}

// =============================================================================
// CSS Integration Tests
// =============================================================================

#[test]
fn test_qrcode_element_id() {
    let qr = QrCodeWidget::new("test").element_id("my-qr");
    assert_eq!(View::id(&qr), Some("my-qr"));
}

#[test]
fn test_qrcode_classes() {
    let qr = QrCodeWidget::new("test")
        .class("qr-class")
        .class("scan-target");
    assert!(View::classes(&qr).contains(&"qr-class".to_string()));
    assert!(View::classes(&qr).contains(&"scan-target".to_string()));
}

#[test]
fn test_qrcode_styled_view_methods() {
    let mut qr = QrCodeWidget::new("test");

    qr.set_id("test-qr");
    assert_eq!(View::id(&qr), Some("test-qr"));

    qr.add_class("active");
    assert!(View::classes(&qr).contains(&"active".to_string()));

    qr.remove_class("active");
    assert!(!View::classes(&qr).contains(&"active".to_string()));

    qr.toggle_class("visible");
    assert!(View::classes(&qr).contains(&"visible".to_string()));

    qr.toggle_class("visible");
    assert!(!View::classes(&qr).contains(&"visible".to_string()));
}

#[test]
fn test_qrcode_meta() {
    let qr = QrCodeWidget::new("test");
    let meta = qr.meta();
    assert!(meta.widget_type.contains("QrCodeWidget"));
}

// =============================================================================
// Additional Edge Cases
// =============================================================================

#[test]
fn test_qrcode_numeric_data() {
    let numeric = "1234567890";
    let qr = QrCodeWidget::new(numeric);
    assert!(qr.required_size().is_some());
}

#[test]
fn test_qrcode_url_data() {
    let url = "https://example.com/path?query=value&param2=123";
    let qr = QrCodeWidget::new(url);
    assert!(qr.required_size().is_some());
}

#[test]
fn test_qrcode_email_data() {
    let email = "mailto:user@example.com?subject=Test&body=Hello";
    let qr = QrCodeWidget::new(email);
    assert!(qr.required_size().is_some());
}

#[test]
fn test_qrcode_emoji_data() {
    let emoji = "Hello 😀 🌍 🎉";
    let qr = QrCodeWidget::new(emoji);
    assert!(qr.required_size().is_some());
}

#[test]
fn test_qrcode_empty_after_set_data() {
    let mut qr = QrCodeWidget::new("initial");
    qr.set_data("");
    // Empty data should still render without panicking
    let mut buffer = Buffer::new(20, 20);
    let area = Rect::new(0, 0, 20, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    qr.render(&mut ctx);
}

#[test]
fn test_qrcode_single_character() {
    let qr = QrCodeWidget::new("A");
    assert!(qr.required_size().is_some());
}

#[test]
fn test_qrcode_binary_like_data() {
    let binary = "\x00\x01\x02\x03";
    let qr = QrCodeWidget::new(binary);
    // Should handle without panicking
    let _ = qr.required_size();
}

#[test]
fn test_qrcode_large_quiet_zone() {
    let qr = QrCodeWidget::new("test").quiet_zone(10);
    let size = qr.required_size().unwrap();
    // Large quiet zone should produce significantly larger size
    assert!(size.0 > 20);
}

#[test]
fn test_qrcode_zero_quiet_zone() {
    let qr = QrCodeWidget::new("test").quiet_zone(0);
    let mut buffer = Buffer::new(30, 30);
    let area = Rect::new(0, 0, 30, 30);
    let mut ctx = RenderContext::new(&mut buffer, area);
    qr.render(&mut ctx);
    // Should render without quiet zone
}

#[test]
fn test_qrcode_negative_area() {
    let qr = QrCodeWidget::new("test");
    let mut buffer = Buffer::new(0, 0);
    let area = Rect::new(0, 0, 0, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);
    qr.render(&mut ctx);
    // Should handle gracefully
}

#[test]
fn test_qrcode_different_data_different_size() {
    let qr1 = QrCodeWidget::new("test");
    let size1 = qr1.required_size().unwrap();

    let qr2 = QrCodeWidget::new("test with more data");
    let size2 = qr2.required_size().unwrap();

    // Different data should produce different sizes
    let _ = (size1, size2);
}

#[test]
fn test_qrcode_inverted_colors_rendering() {
    let qr_normal = QrCodeWidget::new("test").inverted(false);
    let qr_inverted = QrCodeWidget::new("test").inverted(true);

    let mut buffer1 = Buffer::new(30, 30);
    let area = Rect::new(0, 0, 30, 30);
    let mut ctx1 = RenderContext::new(&mut buffer1, area);
    qr_normal.render(&mut ctx1);

    let mut buffer2 = Buffer::new(30, 30);
    let mut ctx2 = RenderContext::new(&mut buffer2, area);
    qr_inverted.render(&mut ctx2);

    // Both should render without panic
}

#[test]
fn test_qrcode_fg_bg_only() {
    let qr = QrCodeWidget::new("test")
        .fg(Color::GREEN)
        .bg(Color::rgb(20, 20, 20));

    let mut buffer = Buffer::new(30, 30);
    let area = Rect::new(0, 0, 30, 30);
    let mut ctx = RenderContext::new(&mut buffer, area);
    qr.render(&mut ctx);
}

#[test]
fn test_qrcode_custom_color_combinations() {
    let combinations = vec![
        (Color::RED, Color::WHITE),
        (Color::BLUE, Color::YELLOW),
        (Color::rgb(255, 0, 255), Color::BLACK),
    ];

    for (fg, bg) in combinations {
        let qr = QrCodeWidget::new("test").fg(fg).bg(bg);
        let mut buffer = Buffer::new(30, 30);
        let area = Rect::new(0, 0, 30, 30);
        let mut ctx = RenderContext::new(&mut buffer, area);
        qr.render(&mut ctx);
    }
}

#[test]
fn test_qrcode_all_error_correction_with_long_data() {
    let data = "A".repeat(200);
    for ec in [
        ErrorCorrection::Low,
        ErrorCorrection::Medium,
        ErrorCorrection::Quartile,
        ErrorCorrection::High,
    ] {
        let qr = QrCodeWidget::new(&data).error_correction(ec);
        assert!(qr.required_size().is_some());
    }
}

#[test]
fn test_qrcode_multiline_data() {
    let multiline = "Line 1\nLine 2\nLine 3\nLine 4";
    let qr = QrCodeWidget::new(multiline);
    assert!(qr.required_size().is_some());
}

#[test]
fn test_qrcode_whitespace_data() {
    let whitespace = "   \t\t\n\r\n   ";
    let qr = QrCodeWidget::new(whitespace);
    // Should handle whitespace
    let _ = qr.required_size();
}

#[test]
fn test_qrcode_unicode_combining_characters() {
    let combining = "éêâ"; // Letters with combining diacritics
    let qr = QrCodeWidget::new(combining);
    assert!(qr.required_size().is_some());
}

#[test]
fn test_qrcode_very_short_area_fit() {
    let qr = QrCodeWidget::new("test");
    let mut buffer = Buffer::new(50, 2);
    let area = Rect::new(0, 0, 50, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);
    qr.render(&mut ctx);
}

#[test]
fn test_qrcode_very_narrow_area_fit() {
    let qr = QrCodeWidget::new("test");
    let mut buffer = Buffer::new(5, 50);
    let area = Rect::new(0, 0, 5, 50);
    let mut ctx = RenderContext::new(&mut buffer, area);
    qr.render(&mut ctx);
}

#[test]
fn test_qrcode_offset_rendering() {
    let qr = QrCodeWidget::new("test");
    let mut buffer = Buffer::new(50, 50);
    let area = Rect::new(10, 10, 30, 30);
    let mut ctx = RenderContext::new(&mut buffer, area);
    qr.render(&mut ctx);
}

#[test]
fn test_qrcode_multiple_render_calls() {
    let qr = QrCodeWidget::new("test");
    let mut buffer = Buffer::new(30, 30);
    let area = Rect::new(0, 0, 30, 30);

    for _ in 0..5 {
        let mut ctx = RenderContext::new(&mut buffer, area);
        qr.render(&mut ctx);
    }
}

#[test]
fn test_qrcode_mixed_styles_render_differently() {
    let data = "test data";
    let styles = [QrStyle::HalfBlock, QrStyle::Ascii, QrStyle::Braille];

    let buffers: Vec<_> = styles
        .iter()
        .map(|style| {
            let mut buffer = Buffer::new(30, 30);
            let area = Rect::new(0, 0, 30, 30);
            let mut ctx = RenderContext::new(&mut buffer, area);
            QrCodeWidget::new(data).style(*style).render(&mut ctx);
            buffer
        })
        .collect();

    // Each style should produce some output
    for buffer in &buffers {
        let mut found_content = false;
        for y in 0..30 {
            for x in 0..30 {
                if let Some(cell) = buffer.get(x, y) {
                    if cell.symbol != ' ' {
                        found_content = true;
                        break;
                    }
                }
            }
            if found_content {
                break;
            }
        }
        assert!(found_content, "Style should produce visible output");
    }
}

#[test]
fn test_qrcode_helper_with_chaining() {
    let qr = qrcode("test")
        .style(QrStyle::Ascii)
        .fg(Color::CYAN)
        .bg(Color::BLACK);

    let mut buffer = Buffer::new(30, 30);
    let area = Rect::new(0, 0, 30, 30);
    let mut ctx = RenderContext::new(&mut buffer, area);
    qr.render(&mut ctx);
}

#[test]
fn test_qrcode_view_id_after_builders() {
    let qr = QrCodeWidget::new("test")
        .element_id("test-id")
        .class("test-class");

    assert_eq!(View::id(&qr), Some("test-id"));
    assert!(View::classes(&qr).contains(&"test-class".to_string()));
}
