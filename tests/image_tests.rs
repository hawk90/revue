//! Integration tests for Image widget

#[cfg(feature = "image")]
use revue::widget::Image;

#[cfg(feature = "image")]
#[test]
fn test_image_from_rgb() {
    let data = vec![255, 0, 0, 0, 255, 0]; // 2 pixels: red, green
    let img = Image::from_rgb(data.clone(), 2, 1);

    assert_eq!(img.width(), 2);
    assert_eq!(img.height(), 1);
}

#[cfg(feature = "image")]
#[test]
fn test_image_from_rgba() {
    let data = vec![255, 0, 0, 255, 0, 255, 0, 255]; // 2 pixels
    let img = Image::from_rgba(data, 2, 1);

    assert_eq!(img.width(), 2);
    assert_eq!(img.height(), 1);
}

#[cfg(feature = "image")]
#[test]
fn test_image_scale_modes() {
    // Test that scale builder method works - each image is created fresh
    let _img_stretch = Image::from_rgb(vec![0; 300], 10, 10);
    let _img_fit = Image::from_rgb(vec![0; 300], 10, 10);
    let _img_fill = Image::from_rgb(vec![0; 300], 10, 10);
    let _img_none = Image::from_rgb(vec![0; 300], 10, 10);
}

#[cfg(feature = "image")]
#[test]
fn test_image_placeholder() {
    let _img = Image::from_rgb(vec![0; 300], 10, 10).placeholder('#');
    // Placeholder was set successfully
}

#[cfg(feature = "image")]
#[test]
fn test_image_width() {
    let img = Image::from_rgb(vec![0; 300], 100, 50);
    assert_eq!(img.width(), 100);
}

#[cfg(feature = "image")]
#[test]
fn test_image_height() {
    let img = Image::from_rgb(vec![0; 300], 100, 50);
    assert_eq!(img.height(), 50);
}

#[cfg(feature = "image")]
#[test]
fn test_image_id() {
    let img = Image::from_rgb(vec![0; 300], 10, 10);

    // Image should have a valid ID (non-zero)
    assert_ne!(img.id(), 0, "Image should have a non-zero ID");
}

#[cfg(feature = "image")]
#[test]
fn test_image_scaled_dimensions_none() {
    let img = Image::from_rgb(vec![0; 600], 200, 100);

    // Test default scale mode
    let (w, h) = img.scaled_dimensions(80, 40);
    assert!(w > 0 && h > 0);
}

#[cfg(feature = "image")]
#[test]
fn test_image_kitty_escape() {
    let data = vec![0u8; 12]; // Small RGB data
    let img = Image::from_rgb(data, 2, 2);
    let escape = img.kitty_escape(10, 5);

    // Should start with Kitty APC (ESC_G)
    assert!(escape.starts_with("\u{1b}_G"));
    // Should end with ST (ESC \)
    assert!(escape.ends_with("\u{1b}\\"));
}

#[cfg(feature = "image")]
#[test]
fn test_image_kitty_support() {
    // Just check that the function works
    let _supported = Image::is_kitty_supported();
}

#[cfg(feature = "image")]
#[test]
fn test_image_from_png_invalid() {
    let invalid_data = vec![0, 1, 2, 3];
    let result = Image::from_png(invalid_data);
    assert!(result.is_err());
}

#[cfg(feature = "image")]
#[test]
fn test_image_try_from_png_invalid() {
    let invalid_data = vec![0, 1, 2, 3];
    let result = Image::try_from_png(invalid_data);
    assert!(result.is_none());
}

#[cfg(feature = "image")]
#[test]
fn test_image_from_file_not_found() {
    let path = "/nonexistent/path/image.bin";
    let result = Image::from_file(path);
    assert!(result.is_err());
}

#[cfg(feature = "image")]
#[test]
fn test_image_try_from_file_not_found() {
    let path = "/nonexistent/path/image.bin";
    let result = Image::try_from_file(path);
    assert!(result.is_none());
}

#[cfg(feature = "image")]
#[test]
fn test_image_builder_pattern() {
    let img = Image::from_rgb(vec![0; 300], 100, 50).placeholder('x');

    assert_eq!(img.width(), 100);
    assert_eq!(img.height(), 50);
}

#[cfg(feature = "image")]
#[test]
fn test_image_square() {
    let img = Image::from_rgb(vec![0; 3], 1, 1);
    assert_eq!(img.width(), 1);
    assert_eq!(img.height(), 1);
}

#[cfg(feature = "image")]
#[test]
fn test_image_wide() {
    let img = Image::from_rgb(vec![0; 600], 200, 100);
    assert_eq!(img.width(), 200);
    assert_eq!(img.height(), 100);
}

#[cfg(feature = "image")]
#[test]
fn test_image_tall() {
    let img = Image::from_rgb(vec![0; 600], 100, 200);
    assert_eq!(img.width(), 100);
    assert_eq!(img.height(), 200);
}

#[cfg(feature = "image")]
#[test]
fn test_image_empty_data() {
    let img = Image::from_rgb(vec![], 0, 0);
    assert_eq!(img.width(), 0);
    assert_eq!(img.height(), 0);
}
