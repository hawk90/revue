//! Sixel graphics encoding

/// Sixel graphics encoder
pub struct SixelEncoder<'a> {
    width: u32,
    height: u32,
    data: &'a [u8],
}

impl<'a> SixelEncoder<'a> {
    /// Create a new Sixel encoder
    pub fn new(width: u32, height: u32, rgba_data: &'a [u8]) -> Self {
        Self {
            width,
            height,
            data: rgba_data,
        }
    }

    /// Encode the image as Sixel
    pub fn encode(&self) -> String {
        let mut output = String::new();

        // Build color palette (up to 256 colors)
        let palette = self.build_palette();

        // Start Sixel sequence
        // DCS P1 ; P2 ; P3 q
        // P1 = 0 (normal), P2 = 0 (default), P3 = 0 (don't set ratio)
        output.push_str("\x1bPq");

        // Set raster attributes: width x height
        {
            use std::fmt::Write;
            let _ = write!(output, "\"1;1;{};{}", self.width, self.height);
        }

        // Define color palette
        for (idx, (r, g, b)) in palette.iter().enumerate() {
            // #idx;2;r;g;b (2 = RGB percentage)
            let r_pct = (*r as u32 * 100) / 255;
            let g_pct = (*g as u32 * 100) / 255;
            let b_pct = (*b as u32 * 100) / 255;
            {
                use std::fmt::Write;
                let _ = write!(output, "#{};2;{};{};{}", idx, r_pct, g_pct, b_pct);
            }
        }

        // Encode pixel data
        // Sixel encodes 6 vertical pixels at a time
        for band in 0..self.height.div_ceil(6) {
            let band_start = band * 6;

            // For each color in palette
            for (color_idx, _) in palette.iter().enumerate() {
                let mut _run_start = 0;
                let mut last_sixel: Option<u8> = None;
                let mut run_length = 0;

                // Select color
                {
                    use std::fmt::Write;
                    let _ = write!(output, "#{}", color_idx);
                }

                for x in 0..self.width {
                    // Build sixel byte for this column
                    let mut sixel_byte: u8 = 0;

                    for bit in 0..6 {
                        let y = band_start + bit;
                        if y >= self.height {
                            break;
                        }

                        let pixel_idx = ((y * self.width + x) * 4) as usize;
                        if pixel_idx + 3 < self.data.len() {
                            let r = self.data[pixel_idx];
                            let g = self.data[pixel_idx + 1];
                            let b = self.data[pixel_idx + 2];
                            let a = self.data[pixel_idx + 3];

                            // Check if this pixel matches current color
                            if a > 127 {
                                let (pr, pg, pb) = palette[color_idx];
                                if Self::color_match(r, g, b, pr, pg, pb) {
                                    sixel_byte |= 1 << bit;
                                }
                            }
                        }
                    }

                    // Run-length encoding
                    if Some(sixel_byte) == last_sixel {
                        run_length += 1;
                    } else {
                        // Flush previous run
                        if let Some(prev_sixel) = last_sixel {
                            Self::encode_run_into(&mut output, prev_sixel, run_length);
                        }
                        last_sixel = Some(sixel_byte);
                        _run_start = x;
                        run_length = 1;
                    }
                }

                // Flush final run
                if let Some(prev_sixel) = last_sixel {
                    Self::encode_run_into(&mut output, prev_sixel, run_length);
                }

                // Carriage return (same line, different color)
                output.push('$');
            }

            // Line feed (next band)
            output.push('-');
        }

        // End Sixel sequence
        output.push_str("\x1b\\");

        output
    }

    /// Build a color palette from the image
    fn build_palette(&self) -> Vec<(u8, u8, u8)> {
        use std::collections::HashMap;

        let mut color_counts: HashMap<(u8, u8, u8), usize> = HashMap::new();

        // Count color occurrences (quantize to reduce colors)
        for pixel in self.data.chunks(4) {
            if pixel.len() == 4 && pixel[3] > 127 {
                // Quantize to reduce color space
                let r = (pixel[0] / 32) * 32;
                let g = (pixel[1] / 32) * 32;
                let b = (pixel[2] / 32) * 32;
                *color_counts.entry((r, g, b)).or_insert(0) += 1;
            }
        }

        // Sort by frequency and take top 256
        let mut colors: Vec<_> = color_counts.into_iter().collect();
        colors.sort_by_key(|b| std::cmp::Reverse(b.1));

        colors
            .into_iter()
            .take(256)
            .map(|(color, _)| color)
            .collect()
    }

    /// Check if two colors are close enough to match
    fn color_match(r1: u8, g1: u8, b1: u8, r2: u8, g2: u8, b2: u8) -> bool {
        let dr = (r1 as i32 - r2 as i32).abs();
        let dg = (g1 as i32 - g2 as i32).abs();
        let db = (b1 as i32 - b2 as i32).abs();
        dr < 32 && dg < 32 && db < 32
    }

    /// Encode a run of sixel bytes directly into the output buffer
    fn encode_run_into(output: &mut String, sixel: u8, length: u32) {
        let sixel_char = (sixel + 63) as char;
        if length == 1 {
            output.push(sixel_char);
        } else if length <= 3 {
            for _ in 0..length {
                output.push(sixel_char);
            }
        } else {
            use std::fmt::Write;
            let _ = write!(output, "!{}{}", length, sixel_char);
        }
    }

    /// Encode a run of sixel bytes (allocating variant for tests)
    #[cfg(test)]
    fn encode_run(sixel: u8, length: u32) -> String {
        let mut s = String::new();
        Self::encode_run_into(&mut s, sixel, length);
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests that access private methods (SixelEncoder::encode_run, SixelEncoder::color_match)
    #[test]
    fn test_sixel_encode_run() {
        assert_eq!(SixelEncoder::encode_run(0, 1), "?");
        assert_eq!(SixelEncoder::encode_run(0, 3), "???");
        assert_eq!(SixelEncoder::encode_run(0, 5), "!5?");
    }

    #[test]
    fn test_sixel_color_match() {
        assert!(SixelEncoder::color_match(100, 100, 100, 100, 100, 100));
        assert!(SixelEncoder::color_match(100, 100, 100, 110, 110, 110));
        assert!(!SixelEncoder::color_match(0, 0, 0, 255, 255, 255));
    }

    // Tests that access private methods (encoder.build_palette())
    #[test]
    fn test_sixel_palette_building() {
        // Simple image with 2 colors
        let data = vec![
            255, 0, 0, 255, // Red
            0, 255, 0, 255, // Green
        ];
        let encoder = SixelEncoder::new(2, 1, &data);
        let palette = encoder.build_palette();

        assert!(!palette.is_empty());
        assert!(palette.len() <= 256);
    }
}
