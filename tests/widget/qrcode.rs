//! QrCodeWidget tests

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::{qrcode, qrcode_url, ErrorCorrection, QrCodeWidget, QrStyle, View};

#[test]
fn test_qr_style_default() {
    assert_eq!(QrStyle::default(), QrStyle::HalfBlock);
}

#[test]
fn test_qr_style_partial_eq() {
    assert_eq!(QrStyle::HalfBlock, QrStyle::HalfBlock);
    assert_eq!(QrStyle::FullBlock, QrStyle::FullBlock);
    assert_eq!(QrStyle::Ascii, QrStyle::Ascii);
    assert_eq!(QrStyle::Braille, QrStyle::Braille);
}

#[test]
fn test_qr_style_ne() {
    assert_ne!(QrStyle::HalfBlock, QrStyle::FullBlock);
    assert_ne!(QrStyle::Ascii, QrStyle::Braille);
}

#[test]
fn test_qr_style_copy() {
    let style = QrStyle::Braille;
    let copied = style;
    assert_eq!(style, copied);
}

#[test]
fn test_qr_style_clone() {
    let style = QrStyle::HalfBlock;
    let cloned = style;
    assert_eq!(style, cloned);
}

#[test]
fn test_qr_style_debug() {
    let debug_str = format!("{:?}", QrStyle::Ascii);
    assert!(debug_str.contains("Ascii"));
}

#[test]
fn test_qr_style_all_variants_unique() {
    assert_ne!(QrStyle::HalfBlock, QrStyle::FullBlock);
    assert_ne!(QrStyle::HalfBlock, QrStyle::Ascii);
    assert_ne!(QrStyle::HalfBlock, QrStyle::Braille);
    assert_ne!(QrStyle::FullBlock, QrStyle::Ascii);
    assert_ne!(QrStyle::FullBlock, QrStyle::Braille);
    assert_ne!(QrStyle::Ascii, QrStyle::Braille);
}

#[test]
fn test_error_correction_default() {
    assert_eq!(ErrorCorrection::default(), ErrorCorrection::Medium);
}

#[test]
fn test_error_correction_partial_eq() {
    assert_eq!(ErrorCorrection::Low, ErrorCorrection::Low);
    assert_eq!(ErrorCorrection::Medium, ErrorCorrection::Medium);
    assert_eq!(ErrorCorrection::Quartile, ErrorCorrection::Quartile);
    assert_eq!(ErrorCorrection::High, ErrorCorrection::High);
}

#[test]
fn test_error_correction_ne() {
    assert_ne!(ErrorCorrection::Low, ErrorCorrection::Medium);
    assert_ne!(ErrorCorrection::Quartile, ErrorCorrection::High);
}

#[test]
fn test_error_correction_copy() {
    let level = ErrorCorrection::High;
    let copied = level;
    assert_eq!(level, copied);
}

#[test]
fn test_error_correction_clone() {
    let level = ErrorCorrection::Low;
    let cloned = level;
    assert_eq!(level, cloned);
}

#[test]
fn test_error_correction_debug() {
    let debug_str = format!("{:?}", ErrorCorrection::Quartile);
    assert!(debug_str.contains("Quartile"));
}

#[test]
fn test_error_correction_all_variants_unique() {
    assert_ne!(ErrorCorrection::Low, ErrorCorrection::Medium);
    assert_ne!(ErrorCorrection::Low, ErrorCorrection::Quartile);
    assert_ne!(ErrorCorrection::Low, ErrorCorrection::High);
    assert_ne!(ErrorCorrection::Medium, ErrorCorrection::Quartile);
    assert_ne!(ErrorCorrection::Medium, ErrorCorrection::High);
    assert_ne!(ErrorCorrection::Quartile, ErrorCorrection::High);
}

#[test]
fn test_qrcode_creation() {
    let qr = QrCodeWidget::new("Hello");
    assert_eq!(qr.get_data(), "Hello");
}

#[test]
fn test_qrcode_default_style() {
    let qr = QrCodeWidget::new("Test");
    assert_eq!(qr.get_style(), QrStyle::HalfBlock);
}

#[test]
fn test_qrcode_default_colors() {
    let qr = QrCodeWidget::new("Test");
    // Nothing named a color, so the stylesheet can reach it; black is applied
    // where the modules are painted.
    assert_eq!(qr.get_fg(), None);
    assert_eq!(qr.get_bg(), Color::WHITE);
}

#[test]
fn test_qrcode_default_ec_level() {
    let qr = QrCodeWidget::new("Test");
    assert_eq!(qr.get_ec_level(), ErrorCorrection::Medium);
}

#[test]
fn test_qrcode_default_quiet_zone() {
    let qr = QrCodeWidget::new("Test");
    assert_eq!(qr.get_quiet_zone(), 1);
}

#[test]
fn test_qrcode_default_inverted() {
    let qr = QrCodeWidget::new("Test");
    assert!(!qr.get_inverted());
}

#[test]
fn test_qrcode_style_builder() {
    let qr = QrCodeWidget::new("Test").style(QrStyle::Braille);
    assert_eq!(qr.get_style(), QrStyle::Braille);
}

#[test]
fn test_qrcode_style_half_block() {
    let qr = QrCodeWidget::new("Test").style(QrStyle::HalfBlock);
    assert_eq!(qr.get_style(), QrStyle::HalfBlock);
}

#[test]
fn test_qrcode_style_full_block() {
    let qr = QrCodeWidget::new("Test").style(QrStyle::FullBlock);
    assert_eq!(qr.get_style(), QrStyle::FullBlock);
}

#[test]
fn test_qrcode_style_ascii() {
    let qr = QrCodeWidget::new("Test").style(QrStyle::Ascii);
    assert_eq!(qr.get_style(), QrStyle::Ascii);
}

#[test]
fn test_qrcode_fg() {
    let qr = QrCodeWidget::new("Test").fg(Color::CYAN);
    assert_eq!(qr.get_fg(), Some(Color::CYAN));
}

#[test]
fn test_qrcode_bg() {
    let qr = QrCodeWidget::new("Test").bg(Color::YELLOW);
    assert_eq!(qr.get_bg(), Color::YELLOW);
}

#[test]
fn test_qrcode_error_correction() {
    let qr = QrCodeWidget::new("Test").error_correction(ErrorCorrection::High);
    assert_eq!(qr.get_ec_level(), ErrorCorrection::High);
}

#[test]
fn test_qrcode_error_correction_low() {
    let qr = QrCodeWidget::new("Test").error_correction(ErrorCorrection::Low);
    assert_eq!(qr.get_ec_level(), ErrorCorrection::Low);
}

#[test]
fn test_qrcode_error_correction_quartile() {
    let qr = QrCodeWidget::new("Test").error_correction(ErrorCorrection::Quartile);
    assert_eq!(qr.get_ec_level(), ErrorCorrection::Quartile);
}

#[test]
fn test_qrcode_quiet_zone() {
    let qr = QrCodeWidget::new("Test").quiet_zone(2);
    assert_eq!(qr.get_quiet_zone(), 2);
}

#[test]
fn test_qrcode_quiet_zone_zero() {
    let qr = QrCodeWidget::new("Test").quiet_zone(0);
    assert_eq!(qr.get_quiet_zone(), 0);
}

#[test]
fn test_qrcode_inverted() {
    let qr = QrCodeWidget::new("Test").inverted(true);
    assert!(qr.get_inverted());
}

#[test]
fn test_qrcode_inverted_false() {
    let qr = QrCodeWidget::new("Test").inverted(false);
    assert!(!qr.get_inverted());
}

#[test]
fn test_qrcode_set_data() {
    let mut qr = QrCodeWidget::new("Hello");
    qr.set_data("World");
    assert_eq!(qr.get_data(), "World");
}

#[test]
fn test_qrcode_set_data_string() {
    let mut qr = QrCodeWidget::new("Hello");
    qr.set_data(String::from("New Data"));
    assert_eq!(qr.get_data(), "New Data");
}

#[test]
fn test_qrcode_builder_chain() {
    let qr = QrCodeWidget::new("Test")
        .style(QrStyle::Braille)
        .fg(Color::WHITE)
        .bg(Color::BLACK)
        .error_correction(ErrorCorrection::High)
        .quiet_zone(2)
        .inverted(true);

    assert_eq!(qr.get_style(), QrStyle::Braille);
    assert_eq!(qr.get_fg(), Some(Color::WHITE));
    assert_eq!(qr.get_bg(), Color::BLACK);
    assert_eq!(qr.get_ec_level(), ErrorCorrection::High);
    assert_eq!(qr.get_quiet_zone(), 2);
    assert!(qr.get_inverted());
}

#[test]
fn test_qrcode_matrix() {
    let qr = QrCodeWidget::new("Test");
    let size = qr.required_size();
    assert!(size.is_some());
}

#[test]
fn test_qrcode_matrix_empty() {
    let qr = QrCodeWidget::new("");
    let size = qr.required_size();
    // Empty string should still generate QR code
    assert!(size.is_some());
}

#[test]
fn test_qrcode_matrix_long_data() {
    let qr = QrCodeWidget::new("https://example.com/very/long/path/that/goes/on/and/on");
    let size = qr.required_size();
    assert!(size.is_some());
}

#[test]
fn test_qrcode_required_size_half_block() {
    let qr = QrCodeWidget::new("Test").style(QrStyle::HalfBlock);
    let size = qr.required_size();
    assert!(size.is_some());
}

#[test]
fn test_qrcode_required_size_full_block() {
    let qr = QrCodeWidget::new("Test").style(QrStyle::FullBlock);
    let size = qr.required_size();
    assert!(size.is_some());
}

#[test]
fn test_qrcode_required_size_ascii() {
    let qr = QrCodeWidget::new("Test").style(QrStyle::Ascii);
    let size = qr.required_size();
    assert!(size.is_some());
}

#[test]
fn test_qrcode_required_size_braille() {
    let qr = QrCodeWidget::new("Test").style(QrStyle::Braille);
    let size = qr.required_size();
    assert!(size.is_some());
}

#[test]
fn test_qrcode_required_size_with_long_data() {
    // Longer data that should still generate a QR code
    let qr = QrCodeWidget::new("a".repeat(500));
    // Should generate a QR code with higher version
    let size = qr.required_size();
    assert!(size.is_some());
}

#[test]
fn test_qrcode_render() {
    let qr = QrCodeWidget::new("Test");

    let mut buffer = Buffer::new(50, 50);
    let area = Rect::new(0, 0, 50, 50);
    let mut ctx = RenderContext::new(&mut buffer, area);

    qr.render(&mut ctx);
}

#[test]
fn test_qrcode_render_half_block() {
    let qr = QrCodeWidget::new("Test").style(QrStyle::HalfBlock);

    let mut buffer = Buffer::new(50, 50);
    let area = Rect::new(0, 0, 50, 50);
    let mut ctx = RenderContext::new(&mut buffer, area);

    qr.render(&mut ctx);
}

#[test]
fn test_qrcode_render_full_block() {
    let qr = QrCodeWidget::new("Test").style(QrStyle::FullBlock);

    let mut buffer = Buffer::new(50, 50);
    let area = Rect::new(0, 0, 50, 50);
    let mut ctx = RenderContext::new(&mut buffer, area);

    qr.render(&mut ctx);
}

#[test]
fn test_qrcode_render_ascii() {
    let qr = QrCodeWidget::new("Test").style(QrStyle::Ascii);

    let mut buffer = Buffer::new(50, 50);
    let area = Rect::new(0, 0, 50, 50);
    let mut ctx = RenderContext::new(&mut buffer, area);

    qr.render(&mut ctx);
}

#[test]
fn test_qrcode_render_braille() {
    let qr = QrCodeWidget::new("Test").style(QrStyle::Braille);

    let mut buffer = Buffer::new(50, 50);
    let area = Rect::new(0, 0, 50, 50);
    let mut ctx = RenderContext::new(&mut buffer, area);

    qr.render(&mut ctx);
}

#[test]
fn test_qrcode_render_inverted() {
    let qr = QrCodeWidget::new("Test")
        .fg(Color::WHITE)
        .bg(Color::BLACK)
        .inverted(true);

    let mut buffer = Buffer::new(50, 50);
    let area = Rect::new(0, 0, 50, 50);
    let mut ctx = RenderContext::new(&mut buffer, area);

    qr.render(&mut ctx);
}

#[test]
fn test_qrcode_render_with_quiet_zone() {
    let qr = QrCodeWidget::new("Test").quiet_zone(4);

    let mut buffer = Buffer::new(100, 100);
    let area = Rect::new(0, 0, 100, 100);
    let mut ctx = RenderContext::new(&mut buffer, area);

    qr.render(&mut ctx);
}

#[test]
fn test_qrcode_size() {
    let qr = QrCodeWidget::new("Hello World");
    let size = qr.required_size();
    assert!(size.is_some());
}

#[test]
fn test_qrcode_error_levels() {
    for level in [
        ErrorCorrection::Low,
        ErrorCorrection::Medium,
        ErrorCorrection::Quartile,
        ErrorCorrection::High,
    ] {
        let qr = QrCodeWidget::new("Test").error_correction(level);
        assert!(qr.required_size().is_some());
    }
}

#[test]
fn test_qrcode_helper() {
    let qr = qrcode("Test Data");
    assert_eq!(qr.get_data(), "Test Data");
}

#[test]
fn test_qrcode_url_helper() {
    let qr = qrcode_url("https://example.com");
    assert_eq!(qr.get_data(), "https://example.com");
}

#[test]
fn test_qrcode_helper_chain() {
    let qr = qrcode("Test").style(QrStyle::Ascii).fg(Color::CYAN);
    assert_eq!(qr.get_data(), "Test");
    assert_eq!(qr.get_style(), QrStyle::Ascii);
}

/// Test QrCode widget edge cases
mod edge_cases {
    use revue::layout::Rect;
    use revue::render::Buffer;
    use revue::widget::traits::{RenderContext, View};

    use revue::widget::{ErrorCorrection, QrCodeWidget, QrStyle};
    #[test]
    fn test_qrcode_with_empty_data() {
        let qr = QrCodeWidget::new("");
        let mut buffer = Buffer::new(50, 50);
        let area = Rect::new(0, 0, 50, 50);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Should not panic with empty data
        qr.render(&mut ctx);
    }

    #[test]
    fn test_qrcode_with_very_long_data() {
        let long_data = "A".repeat(10000);
        let qr = QrCodeWidget::new(&long_data);
        let mut buffer = Buffer::new(100, 100);
        let area = Rect::new(0, 0, 100, 100);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Should handle long data (may truncate or fail gracefully)
        qr.render(&mut ctx);
    }

    #[test]
    fn test_qrcode_with_unicode_data() {
        let unicode_data = "안녕하세요 🎉";
        let qr = QrCodeWidget::new(unicode_data);
        let mut buffer = Buffer::new(50, 50);
        let area = Rect::new(0, 0, 50, 50);
        let mut ctx = RenderContext::new(&mut buffer, area);

        qr.render(&mut ctx);
    }

    #[test]
    fn test_qrcode_with_special_characters() {
        let special_data = "\t\r\n\x00\x01";
        let qr = QrCodeWidget::new(special_data);
        let mut buffer = Buffer::new(50, 50);
        let area = Rect::new(0, 0, 50, 50);
        let mut ctx = RenderContext::new(&mut buffer, area);

        qr.render(&mut ctx);
    }

    #[test]
    fn test_qrcode_with_zero_quiet_zone() {
        let qr = QrCodeWidget::new("Test").quiet_zone(0);
        let mut buffer = Buffer::new(50, 50);
        let area = Rect::new(0, 0, 50, 50);
        let mut ctx = RenderContext::new(&mut buffer, area);

        qr.render(&mut ctx);
    }

    #[test]
    fn test_qrcode_with_large_quiet_zone() {
        let qr = QrCodeWidget::new("Test").quiet_zone(10);
        let mut buffer = Buffer::new(100, 100);
        let area = Rect::new(0, 0, 100, 100);
        let mut ctx = RenderContext::new(&mut buffer, area);

        qr.render(&mut ctx);
    }

    #[test]
    fn test_qrcode_all_styles() {
        let styles = [
            QrStyle::HalfBlock,
            QrStyle::FullBlock,
            QrStyle::Ascii,
            QrStyle::Braille,
        ];

        for style in styles {
            let qr = QrCodeWidget::new("Test").style(style);
            let mut buffer = Buffer::new(50, 50);
            let area = Rect::new(0, 0, 50, 50);
            let mut ctx = RenderContext::new(&mut buffer, area);

            qr.render(&mut ctx);
        }
    }

    #[test]
    fn test_qrcode_with_zero_width_buffer() {
        let qr = QrCodeWidget::new("Test");
        let mut buffer = Buffer::new(0, 10);
        let area = Rect::new(0, 0, 0, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        qr.render(&mut ctx);
    }

    #[test]
    fn test_qrcode_with_zero_height_buffer() {
        let qr = QrCodeWidget::new("Test");
        let mut buffer = Buffer::new(10, 0);
        let area = Rect::new(0, 0, 10, 0);
        let mut ctx = RenderContext::new(&mut buffer, area);

        qr.render(&mut ctx);
    }

    #[test]
    fn test_qrcode_inverted_colors() {
        let qr = QrCodeWidget::new("Test").inverted(true);
        let mut buffer = Buffer::new(50, 50);
        let area = Rect::new(0, 0, 50, 50);
        let mut ctx = RenderContext::new(&mut buffer, area);

        qr.render(&mut ctx);
    }

    #[test]
    fn test_qrcode_data_update() {
        let mut qr = QrCodeWidget::new("Initial");
        qr.set_data("Updated");
        // data field is private, just verify the method doesn't panic
        let _size = qr.required_size();
    }

    #[test]
    fn test_qrcode_all_error_levels() {
        let levels = [
            ErrorCorrection::Low,
            ErrorCorrection::Medium,
            ErrorCorrection::Quartile,
            ErrorCorrection::High,
        ];

        for level in levels {
            let qr = QrCodeWidget::new("Test").error_correction(level);
            let size = qr.required_size();
            assert!(size.is_some());
        }
    }

    #[test]
    fn test_qrcode_with_url() {
        let urls = [
            "https://example.com",
            "http://very-long-domain-name.example.com/path/to/resource?query=value&another=param",
            "ftp://files.example.com/file.txt",
        ];

        for url in urls {
            let qr = QrCodeWidget::new(url);
            let size = qr.required_size();
            assert!(size.is_some());
        }
    }

    #[test]
    fn test_qrcode_required_size_all_styles() {
        let styles = [
            QrStyle::HalfBlock,
            QrStyle::FullBlock,
            QrStyle::Ascii,
            QrStyle::Braille,
        ];

        for style in styles {
            let qr = QrCodeWidget::new("Test").style(style);
            let size = qr.required_size();
            assert!(size.is_some());
        }
    }

    #[test]
    fn test_qrcode_with_newline_data() {
        let qr = QrCodeWidget::new("Line 1\nLine 2");
        let mut buffer = Buffer::new(50, 50);
        let area = Rect::new(0, 0, 50, 50);
        let mut ctx = RenderContext::new(&mut buffer, area);

        qr.render(&mut ctx);
    }
}
