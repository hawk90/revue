//! Tests for Image widget
//!
//! Extracted from src/widget/image.rs

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::RenderContext;
use revue::widget::{image_from_file, try_image_from_file, Image, ImageError, ImageFormat, ScaleMode};

#[test]
fn test_image_from_rgb() {
    let data = vec![255, 0, 0, 0, 255, 0]; // 2 pixels: red, green
    let img = Image::from_rgb(data, 2, 1);
    assert_eq!(img.width(), 2);
    assert_eq!(img.height(), 1);
}

#[test]
fn test_image_from_rgba() {
    let data = vec![255, 0, 0, 255, 0, 255, 0, 255]; // 2 pixels
    let img = Image::from_rgba(data, 2, 1);
    assert_eq!(img.width(), 2);
    assert_eq!(img.height(), 1);
}

#[test]
fn test_image_scale() {
    let img = Image::from_rgb(vec![0; 3], 1, 1).scale(ScaleMode::Stretch);

    let (w, h) = img.scaled_dimensions(80, 24);
    assert_eq!(w, 80);
    assert_eq!(h, 24);
}

#[test]
fn test_image_fit_wide() {
    let img = Image::from_rgb(vec![0; 600], 200, 100); // 2:1 aspect
    let (w, h) = img.scaled_dimensions(80, 40);
    assert_eq!(w, 80);
    assert_eq!(h, 40);
}

#[test]
fn test_image_fit_tall() {
    let img = Image::from_rgb(vec![0; 600], 100, 200); // 1:2 aspect
    let (w, h) = img.scaled_dimensions(80, 40);
    assert_eq!(w, 20);
    assert_eq!(h, 40);
}

#[test]
fn test_image_render_placeholder() {
    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let img = Image::from_rgb(vec![0; 300], 10, 10).placeholder('#');
    img.render(&mut ctx);

    // Check placeholder was rendered
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '#');
}

#[test]
fn test_kitty_escape() {
    let data = vec![0u8; 12]; // Small RGB data
    let img = Image::from_rgb(data, 2, 2);
    let escape = img.kitty_escape(10, 5);

    // Should start with Kitty APC
    assert!(escape.starts_with("\x1b_G"));
    // Should end with ST
    assert!(escape.ends_with("\x1b\\"));
}

#[test]
fn test_scale_mode_none() {
    let img = Image::from_rgb(vec![0; 300], 100, 50);
    // None keeps original dimensions
    let img = img.scale(ScaleMode::None);
    let (w, h) = img.scaled_dimensions(80, 40);
    assert_eq!(w, 100);
    assert_eq!(h, 50);
}

#[test]
fn test_from_png_invalid_data() {
    let invalid_data = vec![0, 1, 2, 3]; // Not valid PNG
    let result = Image::from_png(invalid_data);
    assert!(result.is_err());
    assert!(matches!(result, Err(ImageError::DecodeError(_))));
}

#[test]
fn test_try_from_png_invalid_data() {
    let invalid_data = vec![0, 1, 2, 3];
    let result = Image::try_from_png(invalid_data);
    assert!(result.is_none());
}

#[test]
fn test_from_file_not_found() {
    let result = Image::from_file("/nonexistent/path/image.png");
    assert!(result.is_err());
    if let Err(ImageError::FileRead { path, .. }) = result {
        assert_eq!(path.to_str().unwrap(), "/nonexistent/path/image.png");
    } else {
        panic!("Expected FileRead error");
    }
}

#[test]
fn test_try_from_file_not_found() {
    let result = Image::try_from_file("/nonexistent/path/image.png");
    assert!(result.is_none());
}

#[test]
fn test_image_error_display() {
    use std::path::PathBuf;
    let err = ImageError::FileRead {
        path: PathBuf::from("/test/path.png"),
        message: "file not found".to_string(),
    };
    let display = format!("{}", err);
    assert!(display.contains("/test/path.png"));
    assert!(display.contains("file not found"));

    let err = ImageError::DecodeError("invalid format".to_string());
    let display = format!("{}", err);
    assert!(display.contains("invalid format"));
}

#[test]
fn test_scale_mode_default() {
    assert_eq!(ScaleMode::default(), ScaleMode::Fit);
}

#[test]
fn test_scale_mode_clone() {
    let mode = ScaleMode::Fill;
    assert_eq!(mode, mode.clone());
}

#[test]
fn test_scale_mode_copy() {
    let m1 = ScaleMode::Stretch;
    let m2 = m1;
    assert_eq!(m1, ScaleMode::Stretch);
    assert_eq!(m2, ScaleMode::Stretch);
}

#[test]
fn test_image_format_clone() {
    let fmt = ImageFormat::Rgb;
    assert_eq!(fmt, fmt.clone());
}

#[test]
fn test_image_format_copy() {
    let f1 = ImageFormat::Rgba;
    let f2 = f1;
    assert_eq!(f1, ImageFormat::Rgba);
    assert_eq!(f2, ImageFormat::Rgba);
}

#[test]
fn test_image_placeholder() {
    let img = Image::from_rgb(vec![0; 3], 1, 1).placeholder('X');
    // Can't access placeholder directly, but verify it compiles
    let _ = img;
}

#[test]
fn test_image_scale_fit() {
    let img = Image::from_rgb(vec![0; 3], 1, 1).scale(ScaleMode::Fit);
    let (w, h) = img.scaled_dimensions(80, 40);
    // 1x1 image fits in 80x40, preserving aspect ratio gives 40x40
    assert_eq!(w, 40);
    assert_eq!(h, 40);
}

#[test]
fn test_image_scale_fill() {
    let img = Image::from_rgb(vec![0; 300], 10, 10).scale(ScaleMode::Fill);
    let (w, h) = img.scaled_dimensions(80, 40);
    assert!(w > 0);
    assert!(h > 0);
}

#[test]
fn test_image_width() {
    let img = Image::from_rgb(vec![0; 300], 10, 10);
    assert_eq!(img.width(), 10);
}

#[test]
fn test_image_height() {
    let img = Image::from_rgb(vec![0; 300], 10, 10);
    assert_eq!(img.height(), 10);
}

#[test]
fn test_image_id() {
    let img = Image::from_rgb(vec![0; 3], 1, 1);
    assert!(img.id() > 0);
}

#[test]
fn test_image_unique_id() {
    let img1 = Image::from_rgb(vec![0; 3], 1, 1);
    let img2 = Image::from_rgb(vec![0; 3], 1, 1);
    // IDs should be different unless created at exactly the same time
    // This test verifies the ID generation works
    let _ = (img1.id(), img2.id());
}

#[test]
fn test_scaled_dimensions_none() {
    let img = Image::from_rgb(vec![0; 300], 100, 50).scale(ScaleMode::None);
    let (w, h) = img.scaled_dimensions(80, 40);
    assert_eq!(w, 100);
    assert_eq!(h, 50);
}

#[test]
fn test_scaled_dimensions_stretch() {
    let img = Image::from_rgb(vec![0; 300], 10, 10).scale(ScaleMode::Stretch);
    let (w, h) = img.scaled_dimensions(80, 40);
    assert_eq!(w, 80);
    assert_eq!(h, 40);
}

#[test]
fn test_scaled_dimensions_fit_square() {
    let img = Image::from_rgb(vec![0; 300], 100, 100).scale(ScaleMode::Fit);
    let (w, h) = img.scaled_dimensions(80, 40);
    assert_eq!(w, 40);
    assert_eq!(h, 40);
}

#[test]
fn test_scaled_dimensions_fill_square() {
    let img = Image::from_rgb(vec![0; 300], 100, 100).scale(ScaleMode::Fill);
    let (w, h) = img.scaled_dimensions(80, 40);
    // 100x100 image fills 80x40, preserving aspect ratio gives 80x80 (then cropped)
    assert_eq!(w, 80);
    assert_eq!(h, 80);
}

#[test]
fn test_kitty_escape_format_code() {
    let img = Image::from_rgb(vec![0u8; 12], 2, 2);
    let escape = img.kitty_escape(10, 5);
    // RGB format code is 24
    assert!(escape.contains("f=24,"));
}

#[test]
fn test_kitty_escape_contains_id() {
    let img = Image::from_rgb(vec![0u8; 12], 2, 2);
    let id = img.id();
    let escape = img.kitty_escape(10, 5);
    assert!(escape.contains(&format!("i={}", id)));
}

#[test]
fn test_kitty_escape_dimensions() {
    let img = Image::from_rgb(vec![0u8; 12], 2, 2);
    let escape = img.kitty_escape(20, 10);
    assert!(escape.contains("c=20,"));
    assert!(escape.contains("r=10,"));
}

#[test]
fn test_is_kitty_supported_returns_bool() {
    // Just verify the method works and returns a bool
    let supported = Image::is_kitty_supported();
    let _ = supported; // Result depends on environment
}

#[test]
fn test_image_from_file_helper() {
    let result = image_from_file("/nonexistent/test.png");
    assert!(result.is_err());
}

#[test]
fn test_try_image_from_file_helper() {
    let result = try_image_from_file("/nonexistent/test.png");
    assert!(result.is_none());
}

#[test]
fn test_image_error_clone() {
    let err = ImageError::DecodeError("test".to_string());
    let cloned = err.clone();
    assert_eq!(format!("{}", err), format!("{}", cloned));
}

#[test]
fn test_image_error_unknown_format_display() {
    let err = ImageError::UnknownFormat;
    let display = format!("{}", err);
    assert!(display.contains("determine") || display.contains("format"));
}

#[test]
fn test_image_result_type() {
    // Verify ImageResult type alias works
    let result: Result<Image, ImageError> = Ok(Image::from_rgb(vec![0; 3], 1, 1));
    assert!(result.is_ok());
}

#[test]
fn test_image_from_rgb_single_pixel() {
    let img = Image::from_rgb(vec![255, 128, 64], 1, 1);
    assert_eq!(img.width(), 1);
    assert_eq!(img.height(), 1);
}

#[test]
fn test_image_from_rgba_single_pixel() {
    let img = Image::from_rgba(vec![255, 128, 64, 255], 1, 1);
    assert_eq!(img.width(), 1);
    assert_eq!(img.height(), 1);
}

#[test]
fn test_image_large_dimensions() {
    let width = 1920u32;
    let height = 1080u32;
    let img = Image::from_rgb(vec![0; (width * height * 3) as usize], width, height);
    assert_eq!(img.width(), width);
    assert_eq!(img.height(), height);
}

#[test]
fn test_image_render_zero_width() {
    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 0, 5); // Zero width
    let mut ctx = RenderContext::new(&mut buffer, area);

    let img = Image::from_rgb(vec![0; 3], 1, 1);
    img.render(&mut ctx); // Should not panic
}

#[test]
fn test_image_render_zero_height() {
    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 0); // Zero height
    let mut ctx = RenderContext::new(&mut buffer, area);

    let img = Image::from_rgb(vec![0; 3], 1, 1);
    img.render(&mut ctx); // Should not panic
}
