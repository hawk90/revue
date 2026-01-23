//! Image Protocol tests (from src/render/image_protocol.rs)

#![cfg(feature = "image")]
#![allow(unused_imports)]

use revue::layout::Rect;
use revue::render::*;
use revue::style::Color;

#[test]
fn test_image_protocol_default() {
    assert_eq!(ImageProtocol::default(), ImageProtocol::Kitty);
}

#[test]
fn test_image_protocol_is_supported() {
    assert!(ImageProtocol::Kitty.is_supported());
    assert!(ImageProtocol::Iterm2.is_supported());
    assert!(ImageProtocol::Sixel.is_supported());
    assert!(!ImageProtocol::None.is_supported());
}

#[test]
fn test_image_protocol_name() {
    assert_eq!(ImageProtocol::Kitty.name(), "Kitty");
    assert_eq!(ImageProtocol::Iterm2.name(), "iTerm2");
    assert_eq!(ImageProtocol::Sixel.name(), "Sixel");
    assert_eq!(ImageProtocol::None.name(), "None");
}

// test_encoder_from_rgb and test_encoder_from_rgba access private fields
// and must stay inline in src/render/image_protocol.rs

#[test]
fn test_encoder_set_protocol() {
    let data = vec![0; 12];
    let encoder = ImageEncoder::from_rgb(data, 2, 2).protocol(ImageProtocol::Sixel);
    assert_eq!(encoder.get_protocol(), ImageProtocol::Sixel);
}

#[test]
fn test_encoder_kitty_output() {
    let data = vec![0; 12];
    let encoder = ImageEncoder::from_rgb(data, 2, 2).protocol(ImageProtocol::Kitty);
    let output = encoder.encode(10, 5, 1);

    assert!(output.starts_with("\x1b_G"));
    assert!(output.ends_with("\x1b\\"));
    assert!(output.contains("a=T"));
    assert!(output.contains("f=24")); // RGB format
}

#[test]
fn test_encoder_iterm2_output() {
    let data = vec![0; 12];
    let encoder = ImageEncoder::from_rgb(data, 2, 2).protocol(ImageProtocol::Iterm2);
    let output = encoder.encode(10, 5, 1);

    assert!(output.starts_with("\x1b]1337;File="));
    assert!(output.ends_with("\x07"));
    assert!(output.contains("inline=1"));
}

#[test]
fn test_encoder_sixel_output() {
    let data = vec![255, 0, 0, 255, 0, 255, 0, 255]; // 2 pixels
    let encoder = ImageEncoder::from_rgba(data, 2, 1).protocol(ImageProtocol::Sixel);
    let output = encoder.encode(10, 5, 1);

    assert!(output.starts_with("\x1bPq"));
    assert!(output.ends_with("\x1b\\"));
}

#[test]
fn test_encoder_none_output() {
    let data = vec![0; 12];
    let encoder = ImageEncoder::from_rgb(data, 2, 2).protocol(ImageProtocol::None);
    let output = encoder.encode(10, 5, 1);
    assert!(output.is_empty());
}

// test_sixel_encode_run and test_sixel_color_match access private methods
// and must stay inline in src/render/image_protocol.rs

#[test]
fn test_sixel_encoder_basic() {
    // Single red pixel
    let data = vec![255, 0, 0, 255];
    let encoder = SixelEncoder::new(1, 1, &data);
    let output = encoder.encode();

    assert!(output.starts_with("\x1bPq"));
    assert!(output.ends_with("\x1b\\"));
}

#[test]
fn test_iterm2_inline_image() {
    let data = vec![0u8; 10];
    let output = Iterm2Image::inline_image(&data, Some(10), Some(5), true);

    assert!(output.starts_with("\x1b]1337;File="));
    assert!(output.ends_with("\x07"));
    assert!(output.contains("inline=1"));
    assert!(output.contains("width=10"));
    assert!(output.contains("height=5"));
    assert!(output.contains("preserveAspectRatio=1"));
}

#[test]
fn test_iterm2_positioned_image() {
    let data = vec![0u8; 10];
    let output = Iterm2Image::positioned_image(&data, 5, 3, 10, 5);

    assert!(output.contains("\x1b[4;6H")); // Cursor position (1-indexed)
    assert!(output.contains("inline=1"));
}

#[test]
fn test_kitty_delete() {
    let output = KittyImage::delete(42);
    assert!(output.contains("a=d"));
    assert!(output.contains("i=42"));
}

#[test]
fn test_kitty_delete_all() {
    let output = KittyImage::delete_all();
    assert_eq!(output, "\x1b_Ga=d\x1b\\");
}

#[test]
fn test_kitty_move_image() {
    let output = KittyImage::move_image(42, 10, 5);
    assert!(output.contains("a=p"));
    assert!(output.contains("i=42"));
    assert!(output.contains("x=10"));
    assert!(output.contains("y=5"));
}

#[test]
fn test_kitty_query_support() {
    let output = KittyImage::query_support();
    assert!(output.starts_with("\x1b_G"));
    assert!(output.contains("a=q"));
}

#[test]
fn test_graphics_capabilities_default() {
    let caps = GraphicsCapabilities::default();
    assert_eq!(caps.protocol, ImageProtocol::Kitty);
    assert!(!caps.animation);
}

#[test]
fn test_encoder_empty_data() {
    let encoder = ImageEncoder::from_rgb(vec![], 0, 0).protocol(ImageProtocol::Kitty);
    let output = encoder.encode(10, 5, 1);
    // Empty data produces empty output (no chunks to send)
    assert!(output.is_empty());
}

#[test]
fn test_encoder_large_image_chunking() {
    // Create data larger than 4096 bytes to test chunking
    let data = vec![0u8; 10000];
    let encoder = ImageEncoder::from_rgb(data, 100, 100).protocol(ImageProtocol::Kitty);
    let output = encoder.encode(10, 5, 1);

    // Should have continuation markers
    assert!(output.contains("m=1")); // More data coming
    assert!(output.contains("m=0")); // Final chunk
}

// test_rgb_to_rgba_conversion and test_sixel_palette_building access private methods
// and must stay inline in src/render/image_protocol.rs
