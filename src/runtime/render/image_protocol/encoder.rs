//! Image encoder that turns pixel data into terminal escape sequences

use super::protocol::ImageProtocol;
use super::sixel::SixelEncoder;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};

/// Image encoding/decoding errors
#[derive(Debug, thiserror::Error)]
pub enum ImageError {
    /// PNG encoding failed
    #[error("PNG encoding failed: {0}")]
    EncodeFailed(String),

    /// PNG decoding failed
    #[error("PNG decoding failed: {0}")]
    DecodeFailed(String),

    /// Invalid image format
    #[error("Invalid image format: {0}")]
    InvalidFormat(String),

    /// Invalid image dimensions
    #[error("Invalid image dimensions: {0}x{1}")]
    InvalidDimensions(u32, u32),
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

    /// Encode the image as PNG bytes
    ///
    /// Returns the PNG-encoded data or an error if encoding fails.
    pub fn encode_to_png(&self) -> Result<Vec<u8>, ImageError> {
        self.encode_to_png_internal()
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
        let chunks: Vec<String> = encoded
            .as_bytes()
            .chunks(4096)
            .filter_map(|c| {
                std::str::from_utf8(c)
                    .ok()
                    .map(|s| s.to_string())
                    .or_else(|| {
                        log_warn!(
                            "Invalid UTF-8 in base64 chunk, skipping (this should not happen)"
                        );
                        None
                    })
            })
            .collect();

        for (i, chunk) in chunks.iter().enumerate() {
            let is_first = i == 0;
            let is_last = i == chunks.len() - 1;
            let more = if is_last { 0 } else { 1 };

            if is_first {
                // First chunk includes all parameters
                use std::fmt::Write;
                let _ = write!(
                    output,
                    "\x1b_Ga=T,f={},i={},c={},r={},m={};{}\x1b\\",
                    format_code, image_id, cols, rows, more, chunk
                );
            } else {
                // Continuation chunks
                use std::fmt::Write;
                let _ = write!(output, "\x1b_Gm={};{}\x1b\\", more, chunk);
            }
        }

        output
    }

    /// Encode using iTerm2 inline image protocol (OSC 1337)
    fn encode_iterm2(&self, cols: u16, rows: u16) -> String {
        // Convert to PNG if not already
        let png_data = match self.format {
            PixelFormat::Png => Ok(self.data.clone()),
            PixelFormat::Rgb | PixelFormat::Rgba => self.encode_to_png_internal(),
        };

        let png_data = match png_data {
            Ok(data) => data,
            Err(e) => {
                log_warn!("Failed to encode PNG for iTerm2: {}", e);
                return String::new();
            }
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

    /// Convert image data to PNG format (internal)
    fn encode_to_png_internal(&self) -> Result<Vec<u8>, ImageError> {
        use image::ImageEncoder;

        let mut png_data = Vec::new();

        // Validate dimensions
        if self.width == 0 || self.height == 0 {
            return Err(ImageError::InvalidDimensions(self.width, self.height));
        }

        // Use image crate to encode
        let color_type = match self.format {
            PixelFormat::Rgb => image::ExtendedColorType::Rgb8,
            PixelFormat::Rgba => image::ExtendedColorType::Rgba8,
            PixelFormat::Png => return Ok(self.data.clone()),
        };

        image::codecs::png::PngEncoder::new(&mut png_data)
            .write_image(&self.data, self.width, self.height, color_type)
            .map_err(|e| ImageError::EncodeFailed(e.to_string()))?;

        Ok(png_data)
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
                match image::load_from_memory(&self.data) {
                    Ok(img) => img.to_rgba8().into_raw(),
                    Err(err) => {
                        log_warn!(
                            "PNG decoding failed: {} ({}x{}, {} bytes)",
                            err,
                            self.width,
                            self.height,
                            self.data.len()
                        );
                        vec![0; (self.width * self.height * 4) as usize]
                    }
                }
            }
        }
    }
}

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

    // Tests that access private methods (encoder.to_rgba())
    #[test]
    fn test_rgb_to_rgba_conversion() {
        let rgb_data = vec![255, 0, 0, 0, 255, 0]; // 2 RGB pixels
        let encoder = ImageEncoder::from_rgb(rgb_data, 2, 1);
        let rgba = encoder.to_rgba();

        assert_eq!(rgba.len(), 8); // 2 RGBA pixels
        assert_eq!(rgba[3], 255); // Alpha added
        assert_eq!(rgba[7], 255); // Alpha added
    }

    // Tests for error handling
    #[test]
    fn test_encode_to_png_valid_dimensions() {
        let data = vec![255, 0, 0, 255, 0, 255, 0, 255];
        let encoder = ImageEncoder::from_rgba(data, 2, 1);
        let result = encoder.encode_to_png();
        assert!(result.is_ok());
        let png_data = result.unwrap();
        assert!(!png_data.is_empty());
    }

    #[test]
    fn test_encode_to_png_invalid_dimensions() {
        let data = vec![255, 0, 0, 255];
        let encoder = ImageEncoder::from_rgba(data, 0, 1);
        let result = encoder.encode_to_png();
        assert!(result.is_err());
        match result {
            Err(ImageError::InvalidDimensions(0, 1)) => {}
            _ => panic!("Expected InvalidDimensions error"),
        }
    }

    #[test]
    fn test_encode_to_png_zero_height() {
        let data = vec![255, 0, 0, 255];
        let encoder = ImageEncoder::from_rgba(data, 1, 0);
        let result = encoder.encode_to_png();
        assert!(result.is_err());
        match result {
            Err(ImageError::InvalidDimensions(1, 0)) => {}
            _ => panic!("Expected InvalidDimensions error"),
        }
    }

    #[test]
    fn test_image_error_display() {
        let err = ImageError::EncodeFailed("test error".to_string());
        assert_eq!(format!("{}", err), "PNG encoding failed: test error");

        let err = ImageError::InvalidDimensions(0, 100);
        assert_eq!(format!("{}", err), "Invalid image dimensions: 0x100");
    }
}
