//! Image widget for displaying images using Kitty graphics protocol

use super::traits::{View, RenderContext, WidgetProps};
use crate::render::Cell;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use crate::{impl_styled_view, impl_props_builders};
use std::path::PathBuf;

/// Errors that can occur during image operations
#[derive(Debug, Clone, thiserror::Error)]
pub enum ImageError {
    /// Failed to read image file
    #[error("Failed to read image file '{path}': {message}")]
    FileRead {
        /// Path to the file that failed to read
        path: PathBuf,
        /// Error message
        message: String,
    },

    /// Failed to decode image data
    #[error("Failed to decode image: {0}")]
    DecodeError(String),

    /// Failed to guess image format
    #[error("Failed to determine image format")]
    UnknownFormat,
}

/// Result type for image operations
pub type ImageResult<T> = Result<T, ImageError>;

/// Image scaling mode
#[derive(Clone, Copy, Default)]
pub enum ScaleMode {
    /// Fit within bounds, preserve aspect ratio
    #[default]
    Fit,
    /// Fill bounds, may crop
    Fill,
    /// Stretch to fill, ignore aspect ratio
    Stretch,
    /// Keep original size
    None,
}

/// Image widget using Kitty graphics protocol
pub struct Image {
    data: Vec<u8>,
    width: u32,
    height: u32,
    format: ImageFormat,
    scale: ScaleMode,
    placeholder: char,
    id: u32,
    /// CSS styling properties (id, classes)
    props: WidgetProps,
}

/// Image format
#[derive(Clone, Copy)]
pub enum ImageFormat {
    /// PNG format
    Png,
    /// RGB raw pixels
    Rgb,
    /// RGBA raw pixels
    Rgba,
}

impl Image {
    /// Create an image from raw PNG data
    ///
    /// Returns an error if the image cannot be decoded.
    pub fn from_png(data: Vec<u8>) -> ImageResult<Self> {
        // Try to decode to get dimensions
        let reader = image::ImageReader::new(std::io::Cursor::new(&data))
            .with_guessed_format()
            .map_err(|e| ImageError::DecodeError(e.to_string()))?;
        let img = reader.decode()
            .map_err(|e| ImageError::DecodeError(e.to_string()))?;

        Ok(Self {
            data,
            width: img.width(),
            height: img.height(),
            format: ImageFormat::Png,
            scale: ScaleMode::Fit,
            placeholder: ' ',
            id: rand_id(),
            props: WidgetProps::new(),
        })
    }

    /// Create an image from raw PNG data, returning None on error
    ///
    /// This is a convenience method. Use `from_png()` if you need error details.
    pub fn try_from_png(data: Vec<u8>) -> Option<Self> {
        Self::from_png(data).ok()
    }

    /// Create an image from a file path
    ///
    /// Returns an error if the file cannot be read or the image cannot be decoded.
    pub fn from_file(path: impl AsRef<std::path::Path>) -> ImageResult<Self> {
        let path_ref = path.as_ref();
        let data = std::fs::read(path_ref).map_err(|e| ImageError::FileRead {
            path: path_ref.to_path_buf(),
            message: e.to_string(),
        })?;
        Self::from_png(data)
    }

    /// Create an image from a file path, returning None on error
    ///
    /// This is a convenience method. Use `from_file()` if you need error details.
    pub fn try_from_file(path: impl AsRef<std::path::Path>) -> Option<Self> {
        Self::from_file(path).ok()
    }

    /// Create an image from RGB pixels
    pub fn from_rgb(data: Vec<u8>, width: u32, height: u32) -> Self {
        Self {
            data,
            width,
            height,
            format: ImageFormat::Rgb,
            scale: ScaleMode::Fit,
            placeholder: ' ',
            id: rand_id(),
            props: WidgetProps::new(),
        }
    }

    /// Create an image from RGBA pixels
    pub fn from_rgba(data: Vec<u8>, width: u32, height: u32) -> Self {
        Self {
            data,
            width,
            height,
            format: ImageFormat::Rgba,
            scale: ScaleMode::Fit,
            placeholder: ' ',
            id: rand_id(),
            props: WidgetProps::new(),
        }
    }

    /// Set scaling mode
    pub fn scale(mut self, mode: ScaleMode) -> Self {
        self.scale = mode;
        self
    }

    /// Set placeholder character (shown in non-Kitty terminals)
    pub fn placeholder(mut self, ch: char) -> Self {
        self.placeholder = ch;
        self
    }

    /// Get image width in pixels
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Get image height in pixels
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Get image ID
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Calculate scaled dimensions to fit within bounds
    pub fn scaled_dimensions(&self, max_width: u16, max_height: u16) -> (u16, u16) {
        match self.scale {
            ScaleMode::None => (self.width as u16, self.height as u16),
            ScaleMode::Stretch => (max_width, max_height),
            ScaleMode::Fit => {
                let aspect = self.width as f32 / self.height as f32;
                let fit_width = max_width as f32;
                let fit_height = fit_width / aspect;

                if fit_height <= max_height as f32 {
                    (max_width, fit_height as u16)
                } else {
                    let fit_height = max_height as f32;
                    let fit_width = fit_height * aspect;
                    (fit_width as u16, max_height)
                }
            }
            ScaleMode::Fill => {
                let aspect = self.width as f32 / self.height as f32;
                let fill_width = max_width as f32;
                let fill_height = fill_width / aspect;

                if fill_height >= max_height as f32 {
                    (max_width, fill_height as u16)
                } else {
                    let fill_height = max_height as f32;
                    let fill_width = fill_height * aspect;
                    (fill_width as u16, max_height)
                }
            }
        }
    }

    /// Generate Kitty graphics protocol escape sequence
    pub fn kitty_escape(&self, cols: u16, rows: u16) -> String {
        let mut output = String::new();

        let format_code = match self.format {
            ImageFormat::Png => 100,
            ImageFormat::Rgb => 24,
            ImageFormat::Rgba => 32,
        };

        // Encode data as base64
        let encoded = BASE64.encode(&self.data);

        // Split into chunks (max 4096 bytes per chunk)
        let chunks: Vec<&str> = encoded.as_bytes()
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
                    format_code,
                    self.id,
                    cols,
                    rows,
                    more,
                    chunk
                ));
            } else {
                // Continuation chunks
                output.push_str(&format!(
                    "\x1b_Gm={};{}\x1b\\",
                    more,
                    chunk
                ));
            }
        }

        output
    }

    /// Check if the terminal likely supports Kitty graphics
    pub fn is_kitty_supported() -> bool {
        // Check for TERM_PROGRAM=kitty or KITTY_WINDOW_ID
        std::env::var("KITTY_WINDOW_ID").is_ok() ||
            std::env::var("TERM_PROGRAM").map(|v| v == "kitty").unwrap_or(false)
    }
}

impl View for Image {
    crate::impl_view_meta!("Image");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 1 || area.height < 1 {
            return;
        }

        // For now, render placeholder characters
        // In a real implementation, we would emit the Kitty escape sequence
        // through the terminal's write buffer
        let (scaled_w, scaled_h) = self.scaled_dimensions(area.width, area.height);

        // Fill area with placeholder
        for y in 0..scaled_h.min(area.height) {
            for x in 0..scaled_w.min(area.width) {
                ctx.buffer.set(area.x + x, area.y + y, Cell::new(self.placeholder));
            }
        }

        // Note: Actual Kitty protocol rendering would be done by the terminal
        // renderer, not here. This widget stores the image data and provides
        // the kitty_escape() method for the renderer to use.
    }
}

impl_styled_view!(Image);
impl_props_builders!(Image);

/// Generate a random image ID
fn rand_id() -> u32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    (now.as_nanos() % u32::MAX as u128) as u32
}

/// Helper function to create an image from a file
///
/// Returns an error if the file cannot be read or the image cannot be decoded.
pub fn image_from_file(path: impl AsRef<std::path::Path>) -> ImageResult<Image> {
    Image::from_file(path)
}

/// Helper function to create an image from a file, returning None on error
///
/// This is a convenience function. Use `image_from_file()` if you need error details.
pub fn try_image_from_file(path: impl AsRef<std::path::Path>) -> Option<Image> {
    Image::try_from_file(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::Buffer;
    use crate::layout::Rect;

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
        let img = Image::from_rgb(vec![0; 3], 1, 1)
            .scale(ScaleMode::Stretch);

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

        let img = Image::from_rgb(vec![0; 300], 10, 10)
            .placeholder('#');
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
}
