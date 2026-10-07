//! Image error, result, scale mode and pixel format types

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
