#![allow(clippy::needless_range_loop)]
//! QR Code widget for terminal display
//!
//! Generates and displays QR codes using Unicode block characters
//! for high-resolution rendering in the terminal.

use crate::render::Cell;
use crate::style::Color;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

#[cfg(feature = "qrcode")]
use qrcode::{EcLevel, QrCode};

/// QR Code display style
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum QrStyle {
    /// Use Unicode half blocks (▀▄█ ) - 2 rows per line
    #[default]
    HalfBlock,
    /// Use full blocks (██  ) - 1 row per line
    FullBlock,
    /// Use ASCII (## and spaces)
    Ascii,
    /// Use Braille characters for highest resolution
    Braille,
}

/// Error correction level
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ErrorCorrection {
    /// ~7% error correction
    Low,
    /// ~15% error correction
    #[default]
    Medium,
    /// ~25% error correction
    Quartile,
    /// ~30% error correction
    High,
}

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
pub struct QrCodeWidget {
    /// Data to encode
    data: String,
    /// Display style
    style: QrStyle,
    /// Foreground color (dark modules)
    fg: Color,
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
            fg: Color::BLACK,
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
        self.fg = color;
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
    pub fn inverted(mut self, inverted: bool) -> Self {
        self.inverted = inverted;
        self
    }

    /// Update the data
    pub fn set_data(&mut self, data: impl Into<String>) {
        self.data = data.into();
    }

    /// Get the encoded QR matrix
    fn get_matrix(&self) -> Option<Vec<Vec<bool>>> {
        let code =
            QrCode::with_error_correction_level(&self.data, self.ec_level.to_ec_level()).ok()?;
        let size = code.width();
        let quiet = self.quiet_zone as usize;
        let total_size = size + quiet * 2;

        let mut matrix = vec![vec![false; total_size]; total_size];

        for y in 0..size {
            for x in 0..size {
                let dark = code[(x, y)] == qrcode::Color::Dark;
                matrix[y + quiet][x + quiet] = if self.inverted { !dark } else { dark };
            }
        }

        Some(matrix)
    }

    /// Render using half block characters (▀▄█ )
    fn render_half_block(&self, ctx: &mut RenderContext, matrix: &[Vec<bool>]) {
        let area = ctx.area;
        let height = matrix.len();
        let width = if height > 0 { matrix[0].len() } else { 0 };

        let (fg, bg) = if self.inverted {
            (self.bg, self.fg)
        } else {
            (self.fg, self.bg)
        };

        // Two rows of QR = one terminal row
        for row in 0..height.div_ceil(2) {
            if row as u16 >= area.height {
                break;
            }

            for col in 0..width {
                if col as u16 >= area.width {
                    break;
                }

                let top = matrix
                    .get(row * 2)
                    .and_then(|r| r.get(col))
                    .copied()
                    .unwrap_or(false);
                let bottom = matrix
                    .get(row * 2 + 1)
                    .and_then(|r| r.get(col))
                    .copied()
                    .unwrap_or(false);

                let (ch, cell_fg, cell_bg) = match (top, bottom) {
                    (true, true) => ('█', Some(fg), Some(bg)),
                    (true, false) => ('▀', Some(fg), Some(bg)),
                    (false, true) => ('▄', Some(fg), Some(bg)),
                    (false, false) => (' ', Some(bg), Some(bg)),
                };

                let mut cell = Cell::new(ch);
                cell.fg = cell_fg;
                cell.bg = cell_bg;
                ctx.buffer
                    .set(area.x + col as u16, area.y + row as u16, cell);
            }
        }
    }

    /// Render using full block characters
    fn render_full_block(&self, ctx: &mut RenderContext, matrix: &[Vec<bool>]) {
        let area = ctx.area;
        let height = matrix.len();
        let width = if height > 0 { matrix[0].len() } else { 0 };

        let (fg, bg) = if self.inverted {
            (self.bg, self.fg)
        } else {
            (self.fg, self.bg)
        };

        for row in 0..height {
            if row as u16 >= area.height {
                break;
            }

            for col in 0..width {
                if col as u16 * 2 + 1 >= area.width {
                    break;
                }

                let dark = matrix[row][col];
                let ch = if dark { '█' } else { ' ' };

                let mut cell = Cell::new(ch);
                cell.fg = Some(if dark { fg } else { bg });
                cell.bg = Some(bg);

                // Two columns per module for aspect ratio
                ctx.buffer
                    .set(area.x + col as u16 * 2, area.y + row as u16, cell);
                ctx.buffer
                    .set(area.x + col as u16 * 2 + 1, area.y + row as u16, cell);
            }
        }
    }

    /// Render using ASCII characters
    fn render_ascii(&self, ctx: &mut RenderContext, matrix: &[Vec<bool>]) {
        let area = ctx.area;
        let height = matrix.len();
        let width = if height > 0 { matrix[0].len() } else { 0 };

        for row in 0..height {
            if row as u16 >= area.height {
                break;
            }

            for col in 0..width {
                if col as u16 * 2 + 1 >= area.width {
                    break;
                }

                let dark = matrix[row][col];
                let ch = if dark { '#' } else { ' ' };

                let mut cell = Cell::new(ch);
                cell.fg = Some(self.fg);
                cell.bg = Some(self.bg);

                ctx.buffer
                    .set(area.x + col as u16 * 2, area.y + row as u16, cell);
                ctx.buffer
                    .set(area.x + col as u16 * 2 + 1, area.y + row as u16, cell);
            }
        }
    }

    /// Render using Braille characters for highest resolution
    fn render_braille(&self, ctx: &mut RenderContext, matrix: &[Vec<bool>]) {
        let area = ctx.area;
        let height = matrix.len();
        let width = if height > 0 { matrix[0].len() } else { 0 };

        // Braille: 2 wide x 4 tall dots per character
        // ⠁⠂⠄⡀ ⠈⠐⠠⢀
        let braille_base: u32 = 0x2800;

        for row in 0..height.div_ceil(4) {
            if row as u16 >= area.height {
                break;
            }

            for col in 0..width.div_ceil(2) {
                if col as u16 >= area.width {
                    break;
                }

                let mut dots: u8 = 0;

                // Map matrix pixels to braille dots
                // Braille dot positions:
                // 1 4
                // 2 5
                // 3 6
                // 7 8
                let get = |r: usize, c: usize| -> bool {
                    matrix
                        .get(r)
                        .and_then(|row| row.get(c))
                        .copied()
                        .unwrap_or(false)
                };

                let base_row = row * 4;
                let base_col = col * 2;

                if get(base_row, base_col) {
                    dots |= 0x01;
                } // dot 1
                if get(base_row + 1, base_col) {
                    dots |= 0x02;
                } // dot 2
                if get(base_row + 2, base_col) {
                    dots |= 0x04;
                } // dot 3
                if get(base_row, base_col + 1) {
                    dots |= 0x08;
                } // dot 4
                if get(base_row + 1, base_col + 1) {
                    dots |= 0x10;
                } // dot 5
                if get(base_row + 2, base_col + 1) {
                    dots |= 0x20;
                } // dot 6
                if get(base_row + 3, base_col) {
                    dots |= 0x40;
                } // dot 7
                if get(base_row + 3, base_col + 1) {
                    dots |= 0x80;
                } // dot 8

                let ch = char::from_u32(braille_base + dots as u32).unwrap_or('⠀');

                let mut cell = Cell::new(ch);
                cell.fg = Some(self.fg);
                cell.bg = Some(self.bg);
                ctx.buffer
                    .set(area.x + col as u16, area.y + row as u16, cell);
            }
        }
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

impl View for QrCodeWidget {
    fn render(&self, ctx: &mut RenderContext) {
        let Some(matrix) = self.get_matrix() else {
            // Render error message if QR generation fails
            let msg = "QR Error";
            for (i, ch) in msg.chars().enumerate() {
                if i as u16 >= ctx.area.width {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(Color::RED);
                ctx.buffer.set(ctx.area.x + i as u16, ctx.area.y, cell);
            }
            return;
        };

        match self.style {
            QrStyle::HalfBlock => self.render_half_block(ctx, &matrix),
            QrStyle::FullBlock => self.render_full_block(ctx, &matrix),
            QrStyle::Ascii => self.render_ascii(ctx, &matrix),
            QrStyle::Braille => self.render_braille(ctx, &matrix),
        }
    }

    crate::impl_view_meta!("QrCodeWidget");
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    // =========================================================================
    // QrStyle enum tests
    // =========================================================================

    #[test]
    fn test_qr_style_default() {
        assert_eq!(QrStyle::default(), QrStyle::HalfBlock);
    }

    #[test]
    fn test_qr_style_partial_eq() {
        assert_eq!(QrStyle::HalfBlock, QrStyle::HalfBlock);
        assert_eq!(QrStyle::FullBlock, QrStyle::FullBlock);
        assert_eq!(QrStyle::Ascii, QrStyle::Ascii);
        assert_eq!(QrStyle::Braille, QrStyle::Braille);
    }

    #[test]
    fn test_qr_style_ne() {
        assert_ne!(QrStyle::HalfBlock, QrStyle::FullBlock);
        assert_ne!(QrStyle::Ascii, QrStyle::Braille);
    }

    #[test]
    fn test_qr_style_copy() {
        let style = QrStyle::Braille;
        let copied = style;
        assert_eq!(style, copied);
    }

    #[test]
    fn test_qr_style_clone() {
        let style = QrStyle::HalfBlock;
        let cloned = style.clone();
        assert_eq!(style, cloned);
    }

    #[test]
    fn test_qr_style_debug() {
        let debug_str = format!("{:?}", QrStyle::Ascii);
        assert!(debug_str.contains("Ascii"));
    }

    #[test]
    fn test_qr_style_all_variants_unique() {
        assert_ne!(QrStyle::HalfBlock, QrStyle::FullBlock);
        assert_ne!(QrStyle::HalfBlock, QrStyle::Ascii);
        assert_ne!(QrStyle::HalfBlock, QrStyle::Braille);
        assert_ne!(QrStyle::FullBlock, QrStyle::Ascii);
        assert_ne!(QrStyle::FullBlock, QrStyle::Braille);
        assert_ne!(QrStyle::Ascii, QrStyle::Braille);
    }

    // =========================================================================
    // ErrorCorrection enum tests
    // =========================================================================

    #[test]
    fn test_error_correction_default() {
        assert_eq!(ErrorCorrection::default(), ErrorCorrection::Medium);
    }

    #[test]
    fn test_error_correction_partial_eq() {
        assert_eq!(ErrorCorrection::Low, ErrorCorrection::Low);
        assert_eq!(ErrorCorrection::Medium, ErrorCorrection::Medium);
        assert_eq!(ErrorCorrection::Quartile, ErrorCorrection::Quartile);
        assert_eq!(ErrorCorrection::High, ErrorCorrection::High);
    }

    #[test]
    fn test_error_correction_ne() {
        assert_ne!(ErrorCorrection::Low, ErrorCorrection::Medium);
        assert_ne!(ErrorCorrection::Quartile, ErrorCorrection::High);
    }

    #[test]
    fn test_error_correction_copy() {
        let level = ErrorCorrection::High;
        let copied = level;
        assert_eq!(level, copied);
    }

    #[test]
    fn test_error_correction_clone() {
        let level = ErrorCorrection::Low;
        let cloned = level.clone();
        assert_eq!(level, cloned);
    }

    #[test]
    fn test_error_correction_debug() {
        let debug_str = format!("{:?}", ErrorCorrection::Quartile);
        assert!(debug_str.contains("Quartile"));
    }

    #[test]
    fn test_error_correction_all_variants_unique() {
        assert_ne!(ErrorCorrection::Low, ErrorCorrection::Medium);
        assert_ne!(ErrorCorrection::Low, ErrorCorrection::Quartile);
        assert_ne!(ErrorCorrection::Low, ErrorCorrection::High);
        assert_ne!(ErrorCorrection::Medium, ErrorCorrection::Quartile);
        assert_ne!(ErrorCorrection::Medium, ErrorCorrection::High);
        assert_ne!(ErrorCorrection::Quartile, ErrorCorrection::High);
    }

    // =========================================================================
    // QrCodeWidget tests
    // =========================================================================

    #[test]
    fn test_qrcode_creation() {
        let qr = QrCodeWidget::new("Hello");
        assert_eq!(qr.data, "Hello");
    }

    #[test]
    fn test_qrcode_default_style() {
        let qr = QrCodeWidget::new("Test");
        assert_eq!(qr.style, QrStyle::HalfBlock);
    }

    #[test]
    fn test_qrcode_default_colors() {
        let qr = QrCodeWidget::new("Test");
        assert_eq!(qr.fg, Color::BLACK);
        assert_eq!(qr.bg, Color::WHITE);
    }

    #[test]
    fn test_qrcode_default_ec_level() {
        let qr = QrCodeWidget::new("Test");
        assert_eq!(qr.ec_level, ErrorCorrection::Medium);
    }

    #[test]
    fn test_qrcode_default_quiet_zone() {
        let qr = QrCodeWidget::new("Test");
        assert_eq!(qr.quiet_zone, 1);
    }

    #[test]
    fn test_qrcode_default_inverted() {
        let qr = QrCodeWidget::new("Test");
        assert!(!qr.inverted);
    }

    #[test]
    fn test_qrcode_style_builder() {
        let qr = QrCodeWidget::new("Test").style(QrStyle::Braille);
        assert_eq!(qr.style, QrStyle::Braille);
    }

    #[test]
    fn test_qrcode_style_half_block() {
        let qr = QrCodeWidget::new("Test").style(QrStyle::HalfBlock);
        assert_eq!(qr.style, QrStyle::HalfBlock);
    }

    #[test]
    fn test_qrcode_style_full_block() {
        let qr = QrCodeWidget::new("Test").style(QrStyle::FullBlock);
        assert_eq!(qr.style, QrStyle::FullBlock);
    }

    #[test]
    fn test_qrcode_style_ascii() {
        let qr = QrCodeWidget::new("Test").style(QrStyle::Ascii);
        assert_eq!(qr.style, QrStyle::Ascii);
    }

    #[test]
    fn test_qrcode_fg() {
        let qr = QrCodeWidget::new("Test").fg(Color::CYAN);
        assert_eq!(qr.fg, Color::CYAN);
    }

    #[test]
    fn test_qrcode_bg() {
        let qr = QrCodeWidget::new("Test").bg(Color::YELLOW);
        assert_eq!(qr.bg, Color::YELLOW);
    }

    #[test]
    fn test_qrcode_error_correction() {
        let qr = QrCodeWidget::new("Test").error_correction(ErrorCorrection::High);
        assert_eq!(qr.ec_level, ErrorCorrection::High);
    }

    #[test]
    fn test_qrcode_error_correction_low() {
        let qr = QrCodeWidget::new("Test").error_correction(ErrorCorrection::Low);
        assert_eq!(qr.ec_level, ErrorCorrection::Low);
    }

    #[test]
    fn test_qrcode_error_correction_quartile() {
        let qr = QrCodeWidget::new("Test").error_correction(ErrorCorrection::Quartile);
        assert_eq!(qr.ec_level, ErrorCorrection::Quartile);
    }

    #[test]
    fn test_qrcode_quiet_zone() {
        let qr = QrCodeWidget::new("Test").quiet_zone(2);
        assert_eq!(qr.quiet_zone, 2);
    }

    #[test]
    fn test_qrcode_quiet_zone_zero() {
        let qr = QrCodeWidget::new("Test").quiet_zone(0);
        assert_eq!(qr.quiet_zone, 0);
    }

    #[test]
    fn test_qrcode_inverted() {
        let qr = QrCodeWidget::new("Test").inverted(true);
        assert!(qr.inverted);
    }

    #[test]
    fn test_qrcode_inverted_false() {
        let qr = QrCodeWidget::new("Test").inverted(false);
        assert!(!qr.inverted);
    }

    #[test]
    fn test_qrcode_set_data() {
        let mut qr = QrCodeWidget::new("Hello");
        qr.set_data("World");
        assert_eq!(qr.data, "World");
    }

    #[test]
    fn test_qrcode_set_data_string() {
        let mut qr = QrCodeWidget::new("Hello");
        qr.set_data(String::from("New Data"));
        assert_eq!(qr.data, "New Data");
    }

    #[test]
    fn test_qrcode_builder_chain() {
        let qr = QrCodeWidget::new("Test")
            .style(QrStyle::Braille)
            .fg(Color::WHITE)
            .bg(Color::BLACK)
            .error_correction(ErrorCorrection::High)
            .quiet_zone(2)
            .inverted(true);

        assert_eq!(qr.style, QrStyle::Braille);
        assert_eq!(qr.fg, Color::WHITE);
        assert_eq!(qr.bg, Color::BLACK);
        assert_eq!(qr.ec_level, ErrorCorrection::High);
        assert_eq!(qr.quiet_zone, 2);
        assert!(qr.inverted);
    }

    #[test]
    fn test_qrcode_matrix() {
        let qr = QrCodeWidget::new("Test");
        let matrix = qr.get_matrix();
        assert!(matrix.is_some());
    }

    #[test]
    fn test_qrcode_matrix_empty() {
        let qr = QrCodeWidget::new("");
        let matrix = qr.get_matrix();
        // Empty string should still generate QR code
        assert!(matrix.is_some());
    }

    #[test]
    fn test_qrcode_matrix_long_data() {
        let qr = QrCodeWidget::new("https://example.com/very/long/path/that/goes/on/and/on");
        let matrix = qr.get_matrix();
        assert!(matrix.is_some());
    }

    #[test]
    fn test_qrcode_required_size_half_block() {
        let qr = QrCodeWidget::new("Test").style(QrStyle::HalfBlock);
        let size = qr.required_size();
        assert!(size.is_some());
    }

    #[test]
    fn test_qrcode_required_size_full_block() {
        let qr = QrCodeWidget::new("Test").style(QrStyle::FullBlock);
        let size = qr.required_size();
        assert!(size.is_some());
    }

    #[test]
    fn test_qrcode_required_size_ascii() {
        let qr = QrCodeWidget::new("Test").style(QrStyle::Ascii);
        let size = qr.required_size();
        assert!(size.is_some());
    }

    #[test]
    fn test_qrcode_required_size_braille() {
        let qr = QrCodeWidget::new("Test").style(QrStyle::Braille);
        let size = qr.required_size();
        assert!(size.is_some());
    }

    #[test]
    fn test_qrcode_required_size_with_long_data() {
        // Longer data that should still generate a QR code
        let qr = QrCodeWidget::new("a".repeat(500));
        // Should generate a QR code with higher version
        let size = qr.required_size();
        assert!(size.is_some());
    }

    #[test]
    fn test_qrcode_render() {
        let qr = QrCodeWidget::new("Test");

        let mut buffer = Buffer::new(50, 50);
        let area = Rect::new(0, 0, 50, 50);
        let mut ctx = RenderContext::new(&mut buffer, area);

        qr.render(&mut ctx);
    }

    #[test]
    fn test_qrcode_render_half_block() {
        let qr = QrCodeWidget::new("Test").style(QrStyle::HalfBlock);

        let mut buffer = Buffer::new(50, 50);
        let area = Rect::new(0, 0, 50, 50);
        let mut ctx = RenderContext::new(&mut buffer, area);

        qr.render(&mut ctx);
    }

    #[test]
    fn test_qrcode_render_full_block() {
        let qr = QrCodeWidget::new("Test").style(QrStyle::FullBlock);

        let mut buffer = Buffer::new(50, 50);
        let area = Rect::new(0, 0, 50, 50);
        let mut ctx = RenderContext::new(&mut buffer, area);

        qr.render(&mut ctx);
    }

    #[test]
    fn test_qrcode_render_ascii() {
        let qr = QrCodeWidget::new("Test").style(QrStyle::Ascii);

        let mut buffer = Buffer::new(50, 50);
        let area = Rect::new(0, 0, 50, 50);
        let mut ctx = RenderContext::new(&mut buffer, area);

        qr.render(&mut ctx);
    }

    #[test]
    fn test_qrcode_render_braille() {
        let qr = QrCodeWidget::new("Test").style(QrStyle::Braille);

        let mut buffer = Buffer::new(50, 50);
        let area = Rect::new(0, 0, 50, 50);
        let mut ctx = RenderContext::new(&mut buffer, area);

        qr.render(&mut ctx);
    }

    #[test]
    fn test_qrcode_render_inverted() {
        let qr = QrCodeWidget::new("Test")
            .fg(Color::WHITE)
            .bg(Color::BLACK)
            .inverted(true);

        let mut buffer = Buffer::new(50, 50);
        let area = Rect::new(0, 0, 50, 50);
        let mut ctx = RenderContext::new(&mut buffer, area);

        qr.render(&mut ctx);
    }

    #[test]
    fn test_qrcode_render_with_quiet_zone() {
        let qr = QrCodeWidget::new("Test").quiet_zone(4);

        let mut buffer = Buffer::new(100, 100);
        let area = Rect::new(0, 0, 100, 100);
        let mut ctx = RenderContext::new(&mut buffer, area);

        qr.render(&mut ctx);
    }

    #[test]
    fn test_qrcode_size() {
        let qr = QrCodeWidget::new("Hello World");
        let size = qr.required_size();
        assert!(size.is_some());
    }

    #[test]
    fn test_qrcode_error_levels() {
        for level in [
            ErrorCorrection::Low,
            ErrorCorrection::Medium,
            ErrorCorrection::Quartile,
            ErrorCorrection::High,
        ] {
            let qr = QrCodeWidget::new("Test").error_correction(level);
            assert!(qr.get_matrix().is_some());
        }
    }

    // =========================================================================
    // Helper function tests
    // =========================================================================

    #[test]
    fn test_qrcode_helper() {
        let qr = qrcode("Test Data");
        assert_eq!(qr.data, "Test Data");
    }

    #[test]
    fn test_qrcode_url_helper() {
        let qr = qrcode_url("https://example.com");
        assert_eq!(qr.data, "https://example.com");
    }

    #[test]
    fn test_qrcode_helper_chain() {
        let qr = qrcode("Test").style(QrStyle::Ascii).fg(Color::CYAN);
        assert_eq!(qr.data, "Test");
        assert_eq!(qr.style, QrStyle::Ascii);
    }
}
