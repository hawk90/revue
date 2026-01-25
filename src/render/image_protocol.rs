//! Image protocol support for terminal graphics
//!
//! Supports multiple terminal image protocols:
//! - Kitty graphics protocol (most capable, modern)
//! - iTerm2 inline images (OSC 1337, macOS)
//! - Sixel graphics (legacy, wide support)

use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};

/// Supported terminal image protocols
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ImageProtocol {
    /// Kitty graphics protocol (APC-based)
    #[default]
    Kitty,
    /// iTerm2 inline images (OSC 1337)
    Iterm2,
    /// Sixel graphics protocol
    Sixel,
    /// No graphics support, use placeholder
    None,
}

impl ImageProtocol {
    /// Detect the best available protocol for the current terminal
    pub fn detect() -> Self {
        // Check for Kitty first (most capable)
        if is_kitty_terminal() {
            return ImageProtocol::Kitty;
        }

        // Check for iTerm2
        if is_iterm2_terminal() {
            return ImageProtocol::Iterm2;
        }

        // Check for Sixel support
        if is_sixel_capable() {
            return ImageProtocol::Sixel;
        }

        ImageProtocol::None
    }

    /// Check if this protocol is supported
    pub fn is_supported(&self) -> bool {
        !matches!(self, ImageProtocol::None)
    }

    /// Get protocol name
    pub fn name(&self) -> &'static str {
        match self {
            ImageProtocol::Kitty => "Kitty",
            ImageProtocol::Iterm2 => "iTerm2",
            ImageProtocol::Sixel => "Sixel",
            ImageProtocol::None => "None",
        }
    }
}

/// Check if running in Kitty terminal
pub fn is_kitty_terminal() -> bool {
    std::env::var("KITTY_WINDOW_ID").is_ok()
        || std::env::var("TERM_PROGRAM")
            .map(|v| v == "kitty")
            .unwrap_or(false)
}

/// Check if running in iTerm2 terminal
pub fn is_iterm2_terminal() -> bool {
    std::env::var("TERM_PROGRAM")
        .map(|v| v == "iTerm.app")
        .unwrap_or(false)
        || std::env::var("LC_TERMINAL")
            .map(|v| v == "iTerm2")
            .unwrap_or(false)
}

/// Check if terminal supports Sixel
pub fn is_sixel_capable() -> bool {
    // Check for known Sixel-capable terminals
    if let Ok(term) = std::env::var("TERM") {
        // mlterm, xterm with Sixel, foot, etc.
        if term.contains("mlterm")
            || term.contains("yaft")
            || term.contains("foot")
            || term.contains("contour")
        {
            return true;
        }
    }

    // Check for VTE-based terminals with Sixel support
    if std::env::var("VTE_VERSION").is_ok() {
        if let Ok(colorterm) = std::env::var("COLORTERM") {
            if colorterm == "truecolor" {
                // Modern VTE may support Sixel
                return true;
            }
        }
    }

    // xterm with Sixel support
    if std::env::var("XTERM_VERSION").is_ok() {
        return true;
    }

    false
}

/// Image encoder for terminal protocols
#[derive(Clone, Debug)]
pub struct ImageEncoder {
    /// Target protocol
    protocol: ImageProtocol,
    /// Image data (raw pixels or encoded)
    data: Vec<u8>,
    /// Image width in pixels
    width: u32,
    /// Image height in pixels
    height: u32,
    /// Pixel format
    format: PixelFormat,
}

/// Pixel format for image data
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PixelFormat {
    /// Raw RGB pixels (3 bytes per pixel)
    Rgb,
    /// Raw RGBA pixels (4 bytes per pixel)
    #[default]
    Rgba,
    /// PNG encoded data
    Png,
}

impl ImageEncoder {
    /// Create a new encoder with raw RGB data
    pub fn from_rgb(data: Vec<u8>, width: u32, height: u32) -> Self {
        Self {
            protocol: ImageProtocol::detect(),
            data,
            width,
            height,
            format: PixelFormat::Rgb,
        }
    }

    /// Create a new encoder with raw RGBA data
    pub fn from_rgba(data: Vec<u8>, width: u32, height: u32) -> Self {
        Self {
            protocol: ImageProtocol::detect(),
            data,
            width,
            height,
            format: PixelFormat::Rgba,
        }
    }

    /// Create a new encoder with PNG data
    pub fn from_png(data: Vec<u8>, width: u32, height: u32) -> Self {
        Self {
            protocol: ImageProtocol::detect(),
            data,
            width,
            height,
            format: PixelFormat::Png,
        }
    }

    /// Set the target protocol explicitly
    pub fn protocol(mut self, protocol: ImageProtocol) -> Self {
        self.protocol = protocol;
        self
    }

    /// Get the target protocol
    pub fn get_protocol(&self) -> ImageProtocol {
        self.protocol
    }

    /// Encode the image for terminal display
    pub fn encode(&self, cols: u16, rows: u16, image_id: u32) -> String {
        match self.protocol {
            ImageProtocol::Kitty => self.encode_kitty(cols, rows, image_id),
            ImageProtocol::Iterm2 => self.encode_iterm2(cols, rows),
            ImageProtocol::Sixel => self.encode_sixel(cols, rows),
            ImageProtocol::None => String::new(),
        }
    }

    /// Encode using Kitty graphics protocol
    fn encode_kitty(&self, cols: u16, rows: u16, image_id: u32) -> String {
        let mut output = String::new();

        let format_code = match self.format {
            PixelFormat::Png => 100,
            PixelFormat::Rgb => 24,
            PixelFormat::Rgba => 32,
        };

        // Encode data as base64
        let encoded = BASE64.encode(&self.data);

        // Split into chunks (max 4096 bytes per chunk)
        let chunks: Vec<&str> = encoded
            .as_bytes()
            .chunks(4096)
            .map(|c| std::str::from_utf8(c).unwrap_or(""))
            .collect();

        for (i, chunk) in chunks.iter().enumerate() {
            let is_first = i == 0;
            let is_last = i == chunks.len() - 1;
            let more = if is_last { 0 } else { 1 };

            if is_first {
                // First chunk includes all parameters
                output.push_str(&format!(
                    "\x1b_Ga=T,f={},i={},c={},r={},m={};{}\x1b\\",
                    format_code, image_id, cols, rows, more, chunk
                ));
            } else {
                // Continuation chunks
                output.push_str(&format!("\x1b_Gm={};{}\x1b\\", more, chunk));
            }
        }

        output
    }

    /// Encode using iTerm2 inline image protocol (OSC 1337)
    fn encode_iterm2(&self, cols: u16, rows: u16) -> String {
        // Convert to PNG if not already
        let png_data = match self.format {
            PixelFormat::Png => self.data.clone(),
            PixelFormat::Rgb | PixelFormat::Rgba => self.encode_to_png(),
        };

        // Base64 encode the image
        let encoded = BASE64.encode(&png_data);

        // Build the iTerm2 escape sequence
        // Format: OSC 1337 ; File=[args]:base64data BEL
        let args = format!(
            "name={};size={};width={};height={};inline=1",
            BASE64.encode("image"),
            png_data.len(),
            cols,
            rows
        );

        format!("\x1b]1337;File={}:{}\x07", args, encoded)
    }

    /// Encode using Sixel graphics protocol
    fn encode_sixel(&self, _cols: u16, _rows: u16) -> String {
        // Convert to RGBA if needed
        let rgba_data = self.to_rgba();

        // Create a Sixel encoder
        let sixel = SixelEncoder::new(self.width, self.height, &rgba_data);
        sixel.encode()
    }

    /// Convert image data to PNG format
    fn encode_to_png(&self) -> Vec<u8> {
        use image::ImageEncoder;

        let mut png_data = Vec::new();

        // Use image crate to encode
        let color_type = match self.format {
            PixelFormat::Rgb => image::ExtendedColorType::Rgb8,
            PixelFormat::Rgba => image::ExtendedColorType::Rgba8,
            PixelFormat::Png => return self.data.clone(),
        };

        if let Err(err) = image::codecs::png::PngEncoder::new(&mut png_data).write_image(
            &self.data,
            self.width,
            self.height,
            color_type,
        ) {
            // Avoid hard failure here; return empty data and log for diagnosis
            log_warn!(
                "PNG encoding failed: {} ({}x{}, format {:?})",
                err,
                self.width,
                self.height,
                color_type
            );
        }

        png_data
    }

    /// Convert to RGBA data
    fn to_rgba(&self) -> Vec<u8> {
        match self.format {
            PixelFormat::Rgba => self.data.clone(),
            PixelFormat::Rgb => {
                // Convert RGB to RGBA
                let mut rgba = Vec::with_capacity(self.data.len() / 3 * 4);
                for chunk in self.data.chunks(3) {
                    if chunk.len() == 3 {
                        rgba.extend_from_slice(chunk);
                        rgba.push(255); // Alpha
                    }
                }
                rgba
            }
            PixelFormat::Png => {
                // Decode PNG to RGBA
                if let Ok(img) = image::load_from_memory(&self.data) {
                    img.to_rgba8().into_raw()
                } else {
                    vec![0; (self.width * self.height * 4) as usize]
                }
            }
        }
    }
}

/// Sixel graphics encoder
pub struct SixelEncoder<'a> {
    width: u32,
    height: u32,
    data: &'a [u8],
}

impl<'a> SixelEncoder<'a> {
    /// Create a new Sixel encoder
    pub fn new(width: u32, height: u32, rgba_data: &'a [u8]) -> Self {
        Self {
            width,
            height,
            data: rgba_data,
        }
    }

    /// Encode the image as Sixel
    pub fn encode(&self) -> String {
        let mut output = String::new();

        // Build color palette (up to 256 colors)
        let palette = self.build_palette();

        // Start Sixel sequence
        // DCS P1 ; P2 ; P3 q
        // P1 = 0 (normal), P2 = 0 (default), P3 = 0 (don't set ratio)
        output.push_str("\x1bPq");

        // Set raster attributes: width x height
        output.push_str(&format!("\"1;1;{};{}", self.width, self.height));

        // Define color palette
        for (idx, (r, g, b)) in palette.iter().enumerate() {
            // #idx;2;r;g;b (2 = RGB percentage)
            let r_pct = (*r as u32 * 100) / 255;
            let g_pct = (*g as u32 * 100) / 255;
            let b_pct = (*b as u32 * 100) / 255;
            output.push_str(&format!("#{};2;{};{};{}", idx, r_pct, g_pct, b_pct));
        }

        // Encode pixel data
        // Sixel encodes 6 vertical pixels at a time
        for band in 0..self.height.div_ceil(6) {
            let band_start = band * 6;

            // For each color in palette
            for (color_idx, _) in palette.iter().enumerate() {
                let mut _run_start = 0;
                let mut last_sixel: Option<u8> = None;
                let mut run_length = 0;

                // Select color
                output.push('#');
                output.push_str(&color_idx.to_string());

                for x in 0..self.width {
                    // Build sixel byte for this column
                    let mut sixel_byte: u8 = 0;

                    for bit in 0..6 {
                        let y = band_start + bit;
                        if y >= self.height {
                            break;
                        }

                        let pixel_idx = ((y * self.width + x) * 4) as usize;
                        if pixel_idx + 3 < self.data.len() {
                            let r = self.data[pixel_idx];
                            let g = self.data[pixel_idx + 1];
                            let b = self.data[pixel_idx + 2];
                            let a = self.data[pixel_idx + 3];

                            // Check if this pixel matches current color
                            if a > 127 {
                                let (pr, pg, pb) = palette[color_idx];
                                if Self::color_match(r, g, b, pr, pg, pb) {
                                    sixel_byte |= 1 << bit;
                                }
                            }
                        }
                    }

                    // Run-length encoding
                    if Some(sixel_byte) == last_sixel {
                        run_length += 1;
                    } else {
                        // Flush previous run
                        if let Some(prev_sixel) = last_sixel {
                            output.push_str(&Self::encode_run(prev_sixel, run_length));
                        }
                        last_sixel = Some(sixel_byte);
                        _run_start = x;
                        run_length = 1;
                    }
                }

                // Flush final run
                if let Some(prev_sixel) = last_sixel {
                    output.push_str(&Self::encode_run(prev_sixel, run_length));
                }

                // Carriage return (same line, different color)
                output.push('$');
            }

            // Line feed (next band)
            output.push('-');
        }

        // End Sixel sequence
        output.push_str("\x1b\\");

        output
    }

    /// Build a color palette from the image
    fn build_palette(&self) -> Vec<(u8, u8, u8)> {
        use std::collections::HashMap;

        let mut color_counts: HashMap<(u8, u8, u8), usize> = HashMap::new();

        // Count color occurrences (quantize to reduce colors)
        for pixel in self.data.chunks(4) {
            if pixel.len() == 4 && pixel[3] > 127 {
                // Quantize to reduce color space
                let r = (pixel[0] / 32) * 32;
                let g = (pixel[1] / 32) * 32;
                let b = (pixel[2] / 32) * 32;
                *color_counts.entry((r, g, b)).or_insert(0) += 1;
            }
        }

        // Sort by frequency and take top 256
        let mut colors: Vec<_> = color_counts.into_iter().collect();
        colors.sort_by(|a, b| b.1.cmp(&a.1));

        colors
            .into_iter()
            .take(256)
            .map(|(color, _)| color)
            .collect()
    }

    /// Check if two colors are close enough to match
    fn color_match(r1: u8, g1: u8, b1: u8, r2: u8, g2: u8, b2: u8) -> bool {
        let dr = (r1 as i32 - r2 as i32).abs();
        let dg = (g1 as i32 - g2 as i32).abs();
        let db = (b1 as i32 - b2 as i32).abs();
        dr < 32 && dg < 32 && db < 32
    }

    /// Encode a run of sixel bytes
    fn encode_run(sixel: u8, length: u32) -> String {
        let sixel_char = (sixel + 63) as char;
        if length == 1 {
            sixel_char.to_string()
        } else if length <= 3 {
            std::iter::repeat_n(sixel_char, length as usize).collect()
        } else {
            format!("!{}{}", length, sixel_char)
        }
    }
}

/// iTerm2 specific image operations
pub struct Iterm2Image;

impl Iterm2Image {
    /// Create an inline image escape sequence
    pub fn inline_image(
        data: &[u8],
        width: Option<u16>,
        height: Option<u16>,
        preserve_aspect: bool,
    ) -> String {
        let encoded = BASE64.encode(data);
        let filename = BASE64.encode("image.png");

        let mut args = vec![format!("name={}", filename), format!("inline=1")];

        if let Some(w) = width {
            args.push(format!("width={}", w));
        }
        if let Some(h) = height {
            args.push(format!("height={}", h));
        }
        if preserve_aspect {
            args.push("preserveAspectRatio=1".to_string());
        }

        args.push(format!("size={}", data.len()));

        format!("\x1b]1337;File={}:{}\x07", args.join(";"), encoded)
    }

    /// Create a cursor-positioned image (iTerm2 specific)
    pub fn positioned_image(data: &[u8], x: u16, y: u16, width: u16, height: u16) -> String {
        let encoded = BASE64.encode(data);
        let filename = BASE64.encode("image.png");

        format!(
            "\x1b[{};{}H\x1b]1337;File=name={};width={};height={};inline=1;size={}:{}\x07",
            y + 1,
            x + 1,
            filename,
            width,
            height,
            data.len(),
            encoded
        )
    }

    /// Set custom cursor shape using an image
    pub fn custom_cursor(_data: &[u8]) -> String {
        // Note: iTerm2 doesn't support custom cursor images via escape sequences
        // This is a placeholder for future compatibility
        String::new()
    }
}

/// Kitty specific image operations
pub struct KittyImage;

impl KittyImage {
    /// Delete an image by ID
    pub fn delete(image_id: u32) -> String {
        format!("\x1b_Ga=d,i={}\x1b\\", image_id)
    }

    /// Delete all images
    pub fn delete_all() -> String {
        "\x1b_Ga=d\x1b\\".to_string()
    }

    /// Move/animate an image
    pub fn move_image(image_id: u32, x: u16, y: u16) -> String {
        format!("\x1b_Ga=p,i={},x={},y={}\x1b\\", image_id, x, y)
    }

    /// Create image with placement ID for animation
    pub fn with_placement(
        image_id: u32,
        placement_id: u32,
        data: &[u8],
        cols: u16,
        rows: u16,
    ) -> String {
        let encoded = BASE64.encode(data);
        let chunks: Vec<&str> = encoded
            .as_bytes()
            .chunks(4096)
            .map(|c| std::str::from_utf8(c).unwrap_or(""))
            .collect();

        let mut output = String::new();

        for (i, chunk) in chunks.iter().enumerate() {
            let is_first = i == 0;
            let is_last = i == chunks.len() - 1;
            let more = if is_last { 0 } else { 1 };

            if is_first {
                output.push_str(&format!(
                    "\x1b_Ga=T,f=100,i={},p={},c={},r={},m={};{}\x1b\\",
                    image_id, placement_id, cols, rows, more, chunk
                ));
            } else {
                output.push_str(&format!("\x1b_Gm={};{}\x1b\\", more, chunk));
            }
        }

        output
    }

    /// Query terminal for Kitty graphics support
    pub fn query_support() -> String {
        "\x1b_Gi=31,s=1,v=1,a=q,t=d,f=24;AAAA\x1b\\".to_string()
    }
}

/// Terminal capabilities for graphics
#[derive(Clone, Debug, Default)]
pub struct GraphicsCapabilities {
    /// Best available protocol
    pub protocol: ImageProtocol,
    /// Maximum image width supported
    pub max_width: Option<u32>,
    /// Maximum image height supported
    pub max_height: Option<u32>,
    /// Number of colors supported (for Sixel)
    pub color_depth: Option<u16>,
    /// Whether animation is supported
    pub animation: bool,
}

impl GraphicsCapabilities {
    /// Detect graphics capabilities
    pub fn detect() -> Self {
        let protocol = ImageProtocol::detect();

        let (max_width, max_height, animation) = match protocol {
            ImageProtocol::Kitty => (None, None, true), // No practical limits
            ImageProtocol::Iterm2 => (None, None, false), // No practical limits
            ImageProtocol::Sixel => (Some(4096), Some(4096), false), // Varies by terminal
            ImageProtocol::None => (None, None, false),
        };

        let color_depth = match protocol {
            ImageProtocol::Sixel => Some(256),
            _ => None,
        };

        Self {
            protocol,
            max_width,
            max_height,
            color_depth,
            animation,
        }
    }

    /// Check if any graphics protocol is supported
    pub fn has_graphics(&self) -> bool {
        self.protocol.is_supported()
    }
}

// Most tests moved to tests/render_tests.rs
// These tests access private fields/methods and must stay inline
#[cfg(test)]
mod tests {
    use super::*;

    // Tests that access private fields (encoder.width, encoder.height, encoder.format)
    #[test]
    fn test_encoder_from_rgb() {
        let data = vec![255, 0, 0, 0, 255, 0]; // 2 red/green pixels
        let encoder = ImageEncoder::from_rgb(data, 2, 1);
        assert_eq!(encoder.width, 2);
        assert_eq!(encoder.height, 1);
        assert_eq!(encoder.format, PixelFormat::Rgb);
    }

    #[test]
    fn test_encoder_from_rgba() {
        let data = vec![255, 0, 0, 255, 0, 255, 0, 255];
        let encoder = ImageEncoder::from_rgba(data, 2, 1);
        assert_eq!(encoder.width, 2);
        assert_eq!(encoder.height, 1);
        assert_eq!(encoder.format, PixelFormat::Rgba);
    }

    // Tests that access private methods (SixelEncoder::encode_run, SixelEncoder::color_match)
    #[test]
    fn test_sixel_encode_run() {
        assert_eq!(SixelEncoder::encode_run(0, 1), "?");
        assert_eq!(SixelEncoder::encode_run(0, 3), "???");
        assert_eq!(SixelEncoder::encode_run(0, 5), "!5?");
    }

    #[test]
    fn test_sixel_color_match() {
        assert!(SixelEncoder::color_match(100, 100, 100, 100, 100, 100));
        assert!(SixelEncoder::color_match(100, 100, 100, 110, 110, 110));
        assert!(!SixelEncoder::color_match(0, 0, 0, 255, 255, 255));
    }

    // Tests that access private methods (encoder.to_rgba(), encoder.build_palette())
    #[test]
    fn test_rgb_to_rgba_conversion() {
        let rgb_data = vec![255, 0, 0, 0, 255, 0]; // 2 RGB pixels
        let encoder = ImageEncoder::from_rgb(rgb_data, 2, 1);
        let rgba = encoder.to_rgba();

        assert_eq!(rgba.len(), 8); // 2 RGBA pixels
        assert_eq!(rgba[3], 255); // Alpha added
        assert_eq!(rgba[7], 255); // Alpha added
    }

    #[test]
    fn test_sixel_palette_building() {
        // Simple image with 2 colors
        let data = vec![
            255, 0, 0, 255, // Red
            0, 255, 0, 255, // Green
        ];
        let encoder = SixelEncoder::new(2, 1, &data);
        let palette = encoder.build_palette();

        assert!(!palette.is_empty());
        assert!(palette.len() <= 256);
    }
}
