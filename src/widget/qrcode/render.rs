//! Drawing the QR matrix in half blocks, full blocks, ASCII or Braille

use super::{QrCodeWidget, QrStyle};
use crate::render::Cell;
use crate::style::Color;
use crate::widget::traits::{RenderContext, View};

impl QrCodeWidget {
    /// Render using half block characters (▀▄█ )
    fn render_half_block(&self, ctx: &mut RenderContext, matrix: &[Vec<bool>]) {
        let area = ctx.area;
        let height = matrix.len();
        let width = if height > 0 { matrix[0].len() } else { 0 };

        // The modules take `color` and `background`. A QR code needs contrast to
        // scan, so a rule that changes these is the caller's to get right.
        // `inverted` is already applied to the matrix.
        let fg = self.fg.unwrap_or_else(|| ctx.css_color(Color::BLACK));
        let bg = ctx.css_background(self.bg);

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
                ctx.set(col as u16, row as u16, cell);
            }
        }
    }

    /// Render using full block characters
    fn render_full_block(&self, ctx: &mut RenderContext, matrix: &[Vec<bool>]) {
        let area = ctx.area;
        let height = matrix.len();
        let width = if height > 0 { matrix[0].len() } else { 0 };

        // `inverted` is already applied to the matrix.
        let fg = self.fg.unwrap_or_else(|| ctx.css_color(Color::BLACK));
        let bg = self.bg;

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
                ctx.set(col as u16 * 2, row as u16, cell);
                ctx.set(col as u16 * 2 + 1, row as u16, cell);
            }
        }
    }

    /// Render using ASCII characters
    fn render_ascii(&self, ctx: &mut RenderContext, matrix: &[Vec<bool>]) {
        let module = self.fg.unwrap_or_else(|| ctx.css_color(Color::BLACK));
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
                cell.fg = Some(module);
                cell.bg = Some(self.bg);

                ctx.set(col as u16 * 2, row as u16, cell);
                ctx.set(col as u16 * 2 + 1, row as u16, cell);
            }
        }
    }

    /// Render using Braille characters for highest resolution
    fn render_braille(&self, ctx: &mut RenderContext, matrix: &[Vec<bool>]) {
        let module = self.fg.unwrap_or_else(|| ctx.css_color(Color::BLACK));
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
                cell.fg = Some(module);
                cell.bg = Some(self.bg);
                ctx.set(col as u16, row as u16, cell);
            }
        }
    }
}

impl View for QrCodeWidget {
    fn render(&self, ctx: &mut RenderContext) {
        let Some(matrix) = self.get_matrix() else {
            // Render error message if QR generation fails
            ctx.draw_text(0, 0, "QR Error", Color::RED);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    /// The color each matrix pixel is drawn in, row by row.
    fn pixels(qr: &QrCodeWidget) -> Vec<Vec<Color>> {
        let n = qr.clone().inverted(false).get_matrix().unwrap().len();
        let mut buf = Buffer::new(120, 120);
        let mut ctx = RenderContext::new(&mut buf, Rect::new(0, 0, 120, 120));
        qr.render(&mut ctx);
        let cell = |x: usize, y: usize| buf.get(x as u16, y as u16).unwrap();
        let pick = |on: bool, x: usize, y: usize| {
            let c = cell(x, y);
            if on {
                c.fg.unwrap()
            } else {
                c.bg.unwrap()
            }
        };
        (0..n)
            .map(|r| {
                (0..n)
                    .map(|c| match qr.get_style() {
                        QrStyle::HalfBlock => {
                            let sym = cell(c, r / 2).symbol;
                            let on = if r % 2 == 0 {
                                matches!(sym, '█' | '▀')
                            } else {
                                matches!(sym, '█' | '▄')
                            };
                            pick(on, c, r / 2)
                        }
                        QrStyle::FullBlock => pick(cell(c * 2, r).symbol == '█', c * 2, r),
                        QrStyle::Ascii => pick(cell(c * 2, r).symbol == '#', c * 2, r),
                        QrStyle::Braille => {
                            let bits = cell(c / 2, r / 4).symbol as u32 - 0x2800;
                            let bit = match (r % 4, c % 2) {
                                (0, 0) => 0x01,
                                (1, 0) => 0x02,
                                (2, 0) => 0x04,
                                (0, 1) => 0x08,
                                (1, 1) => 0x10,
                                (2, 1) => 0x20,
                                (3, 0) => 0x40,
                                _ => 0x80,
                            };
                            pick(bits & bit != 0, c / 2, r / 4)
                        }
                    })
                    .collect()
            })
            .collect()
    }

    #[test]
    fn inverted_flips_every_pixel_once_in_every_style() {
        for style in [
            QrStyle::HalfBlock,
            QrStyle::FullBlock,
            QrStyle::Ascii,
            QrStyle::Braille,
        ] {
            let qr = QrCodeWidget::new("hi")
                .style(style)
                .fg(Color::BLACK)
                .bg(Color::WHITE);
            let normal = pixels(&qr);
            let inverted = pixels(&qr.clone().inverted(true));
            for (r, (a, b)) in normal.iter().zip(&inverted).enumerate() {
                for (c, (a, b)) in a.iter().zip(b).enumerate() {
                    assert_ne!(a, b, "{style:?}: pixel ({r}, {c}) is not inverted");
                }
            }
        }
    }
}
