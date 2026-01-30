//! Edge case tests for complex widgets
//!
//! Tests edge cases for more complex interactive widgets:
//! - QrCode: empty data, very long data, unicode, special characters
//! - Image: invalid paths, unsupported formats, zero dimensions
//! - Link: empty URLs, very long URLs, unicode
//! - Pagination: edge cases for page navigation

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::{RenderContext, View};
use revue::widget::Link;
use revue::widget::Pagination;

#[cfg(feature = "qrcode")]
use revue::widget::{ErrorCorrection, QrCodeWidget, QrStyle};

#[cfg(feature = "image")]
use revue::widget::{Image, ImageError, ScaleMode};

/// Test QrCode widget edge cases
#[cfg(feature = "qrcode")]
mod qrcode_edge_cases {
    use super::*;

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

/// Test Image widget edge cases
#[cfg(feature = "image")]
mod image_edge_cases {
    use super::*;

    /// Create a minimal test image (1x1 red pixel)
    fn test_image() -> Image {
        Image::from_rgb(vec![255, 0, 0], 1, 1)
    }

    #[test]
    fn test_image_with_nonexistent_path() {
        let result = Image::try_from_file("/nonexistent/path/to/image.png");
        assert!(result.is_none(), "Should return None for nonexistent path");
    }

    #[test]
    fn test_image_from_file_error() {
        let result = Image::from_file("/nonexistent/path/to/image.png");
        assert!(result.is_err());
        if let Err(ImageError::FileRead { path, .. }) = result {
            assert!(path.to_str().unwrap().contains("nonexistent"));
        } else {
            panic!("Expected FileRead error");
        }
    }

    #[test]
    fn test_image_with_empty_path() {
        let result = Image::try_from_file("");
        assert!(result.is_none(), "Should return None for empty path");
    }

    #[test]
    fn test_image_with_zero_width_buffer() {
        let image = test_image();
        let mut buffer = Buffer::new(0, 10);
        let area = Rect::new(0, 0, 0, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Should handle gracefully
        image.render(&mut ctx);
    }

    #[test]
    fn test_image_with_zero_height_buffer() {
        let image = test_image();
        let mut buffer = Buffer::new(10, 0);
        let area = Rect::new(0, 0, 10, 0);
        let mut ctx = RenderContext::new(&mut buffer, area);

        image.render(&mut ctx);
    }

    #[test]
    fn test_image_with_both_zero() {
        let image = test_image();
        let mut buffer = Buffer::new(0, 0);
        let area = Rect::new(0, 0, 0, 0);
        let mut ctx = RenderContext::new(&mut buffer, area);

        image.render(&mut ctx);
    }

    #[test]
    fn test_image_width_height_getters() {
        let image = Image::from_rgb(vec![0; 12], 4, 3);
        assert_eq!(image.width(), 4);
        assert_eq!(image.height(), 3);
    }

    #[test]
    fn test_image_with_large_dimensions() {
        // Create a large image (10000x10000 would be too much memory, use smaller)
        let large_data = vec![0u8; 100 * 100 * 3]; // 100x100 RGB
        let image = Image::from_rgb(large_data, 100, 100);

        assert_eq!(image.width(), 100);
        assert_eq!(image.height(), 100);

        // Rendering with small buffer should clip
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        image.render(&mut ctx);
    }

    #[test]
    fn test_image_scale_modes() {
        // Test all scale modes (create separate images since Image doesn't implement Clone)
        let _ = test_image().scale(ScaleMode::Fit);
        let _ = test_image().scale(ScaleMode::Fill);
        let _ = test_image().scale(ScaleMode::Stretch);
        let _ = test_image().scale(ScaleMode::None);
    }

    #[test]
    fn test_image_placeholder() {
        let image = Image::from_rgb(vec![255, 0, 0], 2, 2).placeholder('#');
        let mut buffer = Buffer::new(5, 5);
        let area = Rect::new(0, 0, 5, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        image.render(&mut ctx);

        // Check placeholder was rendered
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '#');
    }

    #[test]
    fn test_image_scaled_dimensions_fit() {
        let image = Image::from_rgb(vec![0; 600], 200, 100); // 2:1 aspect
        let (w, h) = image.scaled_dimensions(80, 40);
        assert_eq!(w, 80);
        assert_eq!(h, 40);
    }

    #[test]
    fn test_image_scaled_dimensions_stretch() {
        let image = Image::from_rgb(vec![0; 300], 10, 10).scale(ScaleMode::Stretch);
        let (w, h) = image.scaled_dimensions(80, 24);
        assert_eq!(w, 80);
        assert_eq!(h, 24);
    }

    #[test]
    fn test_image_rgba() {
        let data = vec![255, 0, 0, 255, 0, 255, 0, 255]; // 2 pixels
        let image = Image::from_rgba(data, 2, 1);
        assert_eq!(image.width(), 2);
        assert_eq!(image.height(), 1);
    }
}

/// Test Link widget edge cases
mod link_edge_cases {
    use super::*;

    #[test]
    fn test_link_with_empty_url() {
        let link = Link::new("");
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        link.render(&mut ctx);
    }

    #[test]
    fn test_link_with_empty_label() {
        let link = Link::new("https://example.com").text("");
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        link.render(&mut ctx);
    }

    #[test]
    fn test_link_with_both_empty() {
        let link = Link::new("").text("");
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        link.render(&mut ctx);
    }

    #[test]
    fn test_link_with_very_long_url() {
        let long_url = "https://example.com/".repeat(100);
        let link = Link::new(&long_url);
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        link.render(&mut ctx);
    }

    #[test]
    fn test_link_with_very_long_label() {
        let long_label = "A".repeat(1000);
        let link = Link::new("https://example.com").text(&long_label);
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Should clip to buffer width
        link.render(&mut ctx);
    }

    #[test]
    fn test_link_with_unicode_label() {
        let link = Link::new("https://example.com").text("클릭하세요 🖱️");
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        link.render(&mut ctx);
    }

    #[test]
    fn test_link_with_special_chars_in_url() {
        let urls = [
            "https://example.com/path?query=value&foo=bar",
            "https://example.com/path#fragment",
            "https://user:pass@example.com/",
            "https://example.com/path/%20%space%",
        ];

        for url in urls {
            let link = Link::new(url);
            let mut buffer = Buffer::new(20, 10);
            let area = Rect::new(0, 0, 20, 10);
            let mut ctx = RenderContext::new(&mut buffer, area);

            link.render(&mut ctx);
        }
    }

    #[test]
    fn test_link_with_zero_width_buffer() {
        let link = Link::new("https://example.com");
        let mut buffer = Buffer::new(0, 10);
        let area = Rect::new(0, 0, 0, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        link.render(&mut ctx);
    }
}

/// Test Pagination widget edge cases
mod pagination_edge_cases {
    use super::*;

    #[test]
    fn test_pagination_with_zero_items() {
        let pagination = Pagination::new(0);
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        pagination.render(&mut ctx);
    }

    #[test]
    fn test_pagination_with_single_item() {
        let pagination = Pagination::new(1);
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        pagination.render(&mut ctx);
    }

    #[test]
    fn test_pagination_with_very_large_total() {
        let pagination = Pagination::new(1000);
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        pagination.render(&mut ctx);
    }

    #[test]
    fn test_pagination_first_page() {
        let pagination = Pagination::new(10).current(1);
        let mut buffer = Buffer::new(30, 10);
        let area = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        pagination.render(&mut ctx);
    }

    #[test]
    fn test_pagination_last_page() {
        let pagination = Pagination::new(10).current(10);
        let mut buffer = Buffer::new(30, 10);
        let area = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        pagination.render(&mut ctx);
    }

    #[test]
    fn test_pagination_out_of_bounds_page() {
        let pagination = Pagination::new(10).current(100);
        let mut buffer = Buffer::new(30, 10);
        let area = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        pagination.render(&mut ctx);
    }

    #[test]
    fn test_pagination_with_zero_width_buffer() {
        let pagination = Pagination::new(10);
        let mut buffer = Buffer::new(0, 10);
        let area = Rect::new(0, 0, 0, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        pagination.render(&mut ctx);
    }

    #[test]
    fn test_pagination_negative_page() {
        // Using current(0) should clamp to 1
        let pagination = Pagination::new(10).current(0);
        let mut buffer = Buffer::new(30, 10);
        let area = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        pagination.render(&mut ctx);
    }

    #[test]
    fn test_pagination_exact_multiple() {
        // Items divide evenly
        let pagination = Pagination::new(100);
        let mut buffer = Buffer::new(30, 10);
        let area = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        pagination.render(&mut ctx);
    }

    #[test]
    fn test_pagination_with_very_small_buffer() {
        let pagination = Pagination::new(10);
        let mut buffer = Buffer::new(5, 5);
        let area = Rect::new(0, 0, 5, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Should clip to buffer
        pagination.render(&mut ctx);
    }

    #[test]
    fn test_pagination_all_styles() {
        let styles = [
            revue::widget::PaginationStyle::Full,
            revue::widget::PaginationStyle::Simple,
            revue::widget::PaginationStyle::Compact,
            revue::widget::PaginationStyle::Dots,
        ];

        for style in styles {
            let pagination = Pagination::new(10).style(style);
            let mut buffer = Buffer::new(20, 10);
            let area = Rect::new(0, 0, 20, 10);
            let mut ctx = RenderContext::new(&mut buffer, area);

            pagination.render(&mut ctx);
        }
    }
}
