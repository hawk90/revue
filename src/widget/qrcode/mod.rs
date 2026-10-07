#![allow(clippy::needless_range_loop)]
//! QR Code widget for terminal display
//!
//! Generates and displays QR codes using Unicode block characters
//! for high-resolution rendering in the terminal.

mod render;
mod types;

pub use types::{ErrorCorrection, QrStyle};

use crate::style::Color;
use crate::widget::traits::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};

#[cfg(feature = "qrcode")]
use qrcode::{EcLevel, QrCode};

impl ErrorCorrection {
    fn to_ec_level(self) -> EcLevel {
        match self {
            ErrorCorrection::Low => EcLevel::L,
            ErrorCorrection::Medium => EcLevel::M,
            ErrorCorrection::Quartile => EcLevel::Q,
            ErrorCorrection::High => EcLevel::H,
        }
    }
}

/// QR Code widget
///
/// # Example
///
/// ```rust,ignore
/// use revue::prelude::*;
///
/// let qr = QrCode::new("https://example.com")
///     .style(QrStyle::HalfBlock)
///     .fg(Color::WHITE)
///     .bg(Color::BLACK);
/// ```
#[derive(Clone)]
pub struct QrCodeWidget {
    /// Data to encode
    data: String,
    /// Display style
    style: QrStyle,
    /// Foreground color (dark modules)
    /// The color the builder named, if it named one - see #656.
    fg: Option<Color>,
    /// Background color (light modules)
    bg: Color,
    /// Error correction level
    ec_level: ErrorCorrection,
    /// Quiet zone (border) size
    quiet_zone: u8,
    /// Invert colors
    inverted: bool,
    /// CSS styling properties (id, classes)
    props: WidgetProps,
}

impl QrCodeWidget {
    /// Create a new QR code widget
    pub fn new(data: impl Into<String>) -> Self {
        Self {
            data: data.into(),
            style: QrStyle::default(),
            fg: None,
            bg: Color::WHITE,
            ec_level: ErrorCorrection::default(),
            quiet_zone: 1,
            inverted: false,
            props: WidgetProps::new(),
        }
    }

    /// Set display style
    pub fn style(mut self, style: QrStyle) -> Self {
        self.style = style;
        self
    }

    /// Set foreground color (dark modules)
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Set background color (light modules)
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = color;
        self
    }

    /// Set error correction level
    pub fn error_correction(mut self, level: ErrorCorrection) -> Self {
        self.ec_level = level;
        self
    }

    /// Set quiet zone size (border)
    pub fn quiet_zone(mut self, size: u8) -> Self {
        self.quiet_zone = size;
        self
    }

    /// Invert colors
    ///
    /// Every module, the quiet zone included, swaps between the foreground
    /// and background color - once, in every [`QrStyle`].
    pub fn inverted(mut self, inverted: bool) -> Self {
        self.inverted = inverted;
        self
    }

    /// Update the data
    pub fn set_data(&mut self, data: impl Into<String>) {
        self.data = data.into();
    }

    // Getters for testing
    #[doc(hidden)]
    pub fn get_data(&self) -> &str {
        &self.data
    }

    #[doc(hidden)]
    pub fn get_style(&self) -> QrStyle {
        self.style
    }

    #[doc(hidden)]
    pub fn get_fg(&self) -> Option<Color> {
        self.fg
    }

    #[doc(hidden)]
    pub fn get_bg(&self) -> Color {
        self.bg
    }

    #[doc(hidden)]
    pub fn get_ec_level(&self) -> ErrorCorrection {
        self.ec_level
    }

    #[doc(hidden)]
    pub fn get_quiet_zone(&self) -> u8 {
        self.quiet_zone
    }

    #[doc(hidden)]
    pub fn get_inverted(&self) -> bool {
        self.inverted
    }

    /// Get the encoded QR matrix
    fn get_matrix(&self) -> Option<Vec<Vec<bool>>> {
        let code =
            QrCode::with_error_correction_level(&self.data, self.ec_level.to_ec_level()).ok()?;
        let size = code.width();
        let quiet = self.quiet_zone as usize;
        let total_size = size + quiet * 2;

        // The quiet zone is light; inverting flips it along with the code,
        // so an inverted code keeps the contrast around its finder patterns.
        let mut matrix = vec![vec![self.inverted; total_size]; total_size];

        for y in 0..size {
            for x in 0..size {
                let dark = code[(x, y)] == qrcode::Color::Dark;
                matrix[y + quiet][x + quiet] = dark != self.inverted;
            }
        }

        Some(matrix)
    }

    /// Get the required size for this QR code
    pub fn required_size(&self) -> Option<(u16, u16)> {
        let matrix = self.get_matrix()?;
        let height = matrix.len();
        let width = if height > 0 { matrix[0].len() } else { 0 };

        match self.style {
            QrStyle::HalfBlock => Some((width as u16, height.div_ceil(2) as u16)),
            QrStyle::FullBlock | QrStyle::Ascii => Some((width as u16 * 2, height as u16)),
            QrStyle::Braille => Some((width.div_ceil(2) as u16, height.div_ceil(4) as u16)),
        }
    }
}

impl_styled_view!(QrCodeWidget);
impl_props_builders!(QrCodeWidget);

/// Create a new QR code widget
pub fn qrcode(data: impl Into<String>) -> QrCodeWidget {
    QrCodeWidget::new(data)
}

/// Create a QR code for a URL
pub fn qrcode_url(url: impl Into<String>) -> QrCodeWidget {
    QrCodeWidget::new(url)
}
