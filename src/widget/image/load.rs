//! Creating an image: decoding PNG data or files and wrapping raw pixels, within size limits

use super::{rand_id, Image, ImageError, ImageFormat, ImageResult, ScaleMode};
use crate::widget::traits::WidgetProps;

/// Maximum image file size to prevent DoS (10MB)
const MAX_IMAGE_FILE_SIZE: usize = 10 * 1024 * 1024;

/// Maximum image dimensions to prevent memory exhaustion
const MAX_IMAGE_DIMENSION: u32 = 8192;

/// Maximum total pixels (width * height) to prevent memory exhaustion
const MAX_IMAGE_PIXELS: u64 = 67_108_864; // 8192 * 8192

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
}
