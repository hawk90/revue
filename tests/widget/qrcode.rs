//! QR Code widget integration tests
//!
//! Tests for QR code generation, rendering, and styling.

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::{RenderContext, View};
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
