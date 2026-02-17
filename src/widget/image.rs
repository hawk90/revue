//! Image widget for displaying images using Kitty graphics protocol

use crate::render::Cell;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use std::path::PathBuf;

/// Maximum image file size to prevent DoS (10MB)
const MAX_IMAGE_FILE_SIZE: usize = 10 * 1024 * 1024;

/// Maximum image dimensions to prevent memory exhaustion
const MAX_IMAGE_DIMENSION: u32 = 8192;

/// Maximum total pixels (width * height) to prevent memory exhaustion
const MAX_IMAGE_PIXELS: u64 = 67_108_864; // 8192 * 8192

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

    /// Image file too large
    #[error("Image file size ({size} bytes) exceeds maximum ({max} bytes)")]
    FileTooLarge {
        /// Actual file size
        size: usize,
        /// Maximum allowed size
        max: usize,
    },

    /// Image dimensions too large
    #[error("Image dimensions ({width}x{height}) exceed maximum ({max}x{max})")]
    DimensionsTooLarge {
        /// Image width
        width: u32,
        /// Image height
        height: u32,
        /// Maximum allowed dimension
        max: u32,
    },
}

/// Result type for image operations
pub type ImageResult<T> = Result<T, ImageError>;

/// Image scaling mode
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
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
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
    /// # Errors
    ///
    /// Returns `Err(ImageError::FileTooLarge)` if data size exceeds MAX_IMAGE_FILE_SIZE.
    /// Returns `Err(ImageError::DimensionsTooLarge)` if image dimensions exceed limits.
    /// Returns `Err(ImageError::DecodeError)` if:
    /// - The data is not a valid image format
    /// - The image is corrupted
    /// - The image format is not supported
    pub fn from_png(data: Vec<u8>) -> ImageResult<Self> {
        // Check file size
        if data.len() > MAX_IMAGE_FILE_SIZE {
            return Err(ImageError::FileTooLarge {
                size: data.len(),
                max: MAX_IMAGE_FILE_SIZE,
            });
        }

        // Try to decode to get dimensions
        let reader = image::ImageReader::new(std::io::Cursor::new(&data))
            .with_guessed_format()
            .map_err(|e| ImageError::DecodeError(e.to_string()))?;
        let img = reader
            .decode()
            .map_err(|e| ImageError::DecodeError(e.to_string()))?;

        // Check dimensions
        let width = img.width();
        let height = img.height();

        if width > MAX_IMAGE_DIMENSION || height > MAX_IMAGE_DIMENSION {
            return Err(ImageError::DimensionsTooLarge {
                width,
                height,
                max: MAX_IMAGE_DIMENSION,
            });
        }

        // Check total pixels to prevent integer overflow
        let pixels = width as u64 * height as u64;
        if pixels > MAX_IMAGE_PIXELS {
            return Err(ImageError::DimensionsTooLarge {
                width,
                height,
                max: MAX_IMAGE_DIMENSION,
            });
        }

        Ok(Self {
            data,
            width,
            height,
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
    /// # Errors
    ///
    /// Returns `Err(ImageError::FileRead)` if:
    /// - The file does not exist
    /// - The file cannot be read (permission denied, etc.)
    ///
    /// Returns `Err(ImageError::FileTooLarge)` if file size exceeds MAX_IMAGE_FILE_SIZE.
    /// Returns `Err(ImageError::DecodeError)` if the image cannot be decoded.
    pub fn from_file(path: impl AsRef<std::path::Path>) -> ImageResult<Self> {
        let path_ref = path.as_ref();

        // Check file size before reading to prevent DoS
        let metadata = std::fs::metadata(path_ref).map_err(|e| ImageError::FileRead {
            path: path_ref.to_path_buf(),
            message: e.to_string(),
        })?;

        let file_len = metadata.len() as usize;
        if file_len > MAX_IMAGE_FILE_SIZE {
            return Err(ImageError::FileTooLarge {
                size: file_len,
                max: MAX_IMAGE_FILE_SIZE,
            });
        }

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
    ///
    /// # Panics
    ///
    /// Panics if width or height exceeds MAX_IMAGE_DIMENSION, or if total pixels
    /// exceed MAX_IMAGE_PIXELS. This is intentional to catch programming errors early.
    pub fn from_rgb(data: Vec<u8>, width: u32, height: u32) -> Self {
        assert!(
            width <= MAX_IMAGE_DIMENSION && height <= MAX_IMAGE_DIMENSION,
            "Image dimensions {}x{} exceed maximum {}x{}",
            width,
            height,
            MAX_IMAGE_DIMENSION,
            MAX_IMAGE_DIMENSION
        );

        let pixels = width as u64 * height as u64;
        assert!(
            pixels <= MAX_IMAGE_PIXELS,
            "Image total pixels ({}) exceed maximum ({})",
            pixels,
            MAX_IMAGE_PIXELS
        );

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
    ///
    /// # Panics
    ///
    /// Panics if width or height exceeds MAX_IMAGE_DIMENSION, or if total pixels
    /// exceed MAX_IMAGE_PIXELS. This is intentional to catch programming errors early.
    pub fn from_rgba(data: Vec<u8>, width: u32, height: u32) -> Self {
        assert!(
            width <= MAX_IMAGE_DIMENSION && height <= MAX_IMAGE_DIMENSION,
            "Image dimensions {}x{} exceed maximum {}x{}",
            width,
            height,
            MAX_IMAGE_DIMENSION,
            MAX_IMAGE_DIMENSION
        );

        let pixels = width as u64 * height as u64;
        assert!(
            pixels <= MAX_IMAGE_PIXELS,
            "Image total pixels ({}) exceed maximum ({})",
            pixels,
            MAX_IMAGE_PIXELS
        );

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

    // Getters for testing
    #[doc(hidden)]
    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    #[doc(hidden)]
    pub fn get_format(&self) -> ImageFormat {
        self.format
    }

    #[doc(hidden)]
    pub fn get_scale(&self) -> ScaleMode {
        self.scale
    }

    #[doc(hidden)]
    pub fn get_placeholder(&self) -> char {
        self.placeholder
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
                output.push_str(&format!(
                    "\x1b_Ga=T,f={},i={},c={},r={},m={};{}\x1b\\",
                    format_code, self.id, cols, rows, more, chunk
                ));
            } else {
                // Continuation chunks
                output.push_str(&format!("\x1b_Gm={};{}\x1b\\", more, chunk));
            }
        }

        output
    }

    /// Check if the terminal likely supports Kitty graphics
    pub fn is_kitty_supported() -> bool {
        // Check for TERM_PROGRAM=kitty or KITTY_WINDOW_ID
        std::env::var("KITTY_WINDOW_ID").is_ok()
            || std::env::var("TERM_PROGRAM")
                .map(|v| v == "kitty")
                .unwrap_or(false)
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
                ctx.buffer
                    .set(area.x + x, area.y + y, Cell::new(self.placeholder));
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
