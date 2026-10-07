//! Creating an image: decoding PNG data or files and wrapping raw pixels, within size limits

use super::{rand_id, Image, ImageError, ImageFormat, ImageResult, ScaleMode};
use crate::widget::traits::WidgetProps;

/// Maximum image file size to prevent DoS (10MB)
const MAX_IMAGE_FILE_SIZE: usize = 10 * 1024 * 1024;

/// Maximum image dimensions to prevent memory exhaustion
const MAX_IMAGE_DIMENSION: u32 = 8192;

/// Maximum total pixels (width * height) to prevent memory exhaustion
const MAX_IMAGE_PIXELS: u64 = 67_108_864; // 8192 * 8192

/// Read up to one byte past [`MAX_IMAGE_FILE_SIZE`]: enough for
/// [`Image::from_png`] to see the data is too large, never more.
fn read_image(path: &std::path::Path) -> std::io::Result<Vec<u8>> {
    use std::io::Read;
    let mut data = Vec::new();
    std::fs::File::open(path)?
        .take(MAX_IMAGE_FILE_SIZE as u64 + 1)
        .read_to_end(&mut data)?;
    Ok(data)
}

impl Image {
    /// Create an image from raw PNG data
    ///
    /// PNG data is kept as it is (`ImageFormat::Png`). Other formats the
    /// decoder recognizes (JPEG, BMP, GIF, ...) are decoded to RGBA pixels and
    /// kept as `ImageFormat::Rgba`, so the terminal is never told raw bytes
    /// of another format are PNG.
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
        let is_png = reader.format() == Some(image::ImageFormat::Png);
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

        // PNG bytes go to the terminal as they are (Kitty `f=100`); any other
        // format is sent as the decoded RGBA pixels (`f=32`).
        let (data, format) = if is_png {
            (data, ImageFormat::Png)
        } else {
            (img.into_rgba8().into_raw(), ImageFormat::Rgba)
        };

        Ok(Self {
            data,
            width,
            height,
            format,
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
    ///
    /// The file may be any format the decoder recognizes; see
    /// [`from_png`](Self::from_png) for how it is kept.
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

        // Capped: the metadata length is not the read length for a FIFO or a
        // file still being written. Past the limit, `from_png` refuses it.
        let data = read_image(path_ref).map_err(|e| ImageError::FileRead {
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

#[cfg(test)]
mod tests {
    use super::*;

    /// A 3x2 image with a different color in every pixel
    fn sample() -> image::RgbImage {
        image::RgbImage::from_fn(3, 2, |x, y| image::Rgb([x as u8 * 80, y as u8 * 120, 200]))
    }

    fn encode(format: image::ImageFormat) -> Vec<u8> {
        let mut bytes = std::io::Cursor::new(Vec::new());
        sample().write_to(&mut bytes, format).unwrap();
        bytes.into_inner()
    }

    #[test]
    fn png_is_kept_as_png() {
        let data = encode(image::ImageFormat::Png);
        let png = Image::from_png(data.clone()).unwrap();
        assert_eq!(png.get_format(), ImageFormat::Png);
        assert_eq!(png.get_data(), &data[..]);
        assert!(png.kitty_escape(1, 1).contains("f=100"));
    }

    #[test]
    fn other_formats_load_as_rgba_pixels() {
        let expected: Vec<u8> = image::DynamicImage::ImageRgb8(sample())
            .to_rgba8()
            .into_raw();

        let bmp = Image::from_png(encode(image::ImageFormat::Bmp)).unwrap();
        assert_eq!(bmp.get_format(), ImageFormat::Rgba);
        assert_eq!((bmp.width(), bmp.height()), (3, 2));
        assert_eq!(bmp.get_data(), &expected[..]);
        // Raw RGBA for Kitty, not `f=100` (PNG).
        let escape = bmp.kitty_escape(1, 1);
        assert!(escape.contains("f=32") && !escape.contains("f=100"));

        // JPEG is lossy: check the shape, not the exact pixels.
        let jpeg = Image::from_png(encode(image::ImageFormat::Jpeg)).unwrap();
        assert_eq!(jpeg.get_format(), ImageFormat::Rgba);
        assert_eq!((jpeg.width(), jpeg.height()), (3, 2));
        assert_eq!(jpeg.get_data().len(), 3 * 2 * 4);
    }

    #[test]
    fn data_that_does_not_decode_is_an_error() {
        let garbage = Image::from_png(b"not an image at all".to_vec());
        assert!(matches!(garbage, Err(ImageError::DecodeError(_))));
    }
}
