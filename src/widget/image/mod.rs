//! Image widget for displaying images using Kitty graphics protocol

mod kitty;
mod load;
mod types;

pub use types::{ImageError, ImageFormat, ImageResult, ScaleMode};

use crate::render::Cell;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// Image widget using Kitty graphics protocol
#[derive(Clone)]
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

impl Image {
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
                ctx.set(x, y, Cell::new(self.placeholder));
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
