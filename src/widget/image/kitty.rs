//! Encoding an image as a Kitty graphics protocol escape sequence

use super::{Image, ImageFormat};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};

impl Image {
    /// Generate Kitty graphics protocol escape sequence
    pub fn kitty_escape(&self, cols: u16, rows: u16) -> String {
        let mut output = String::new();

        let format_code = match self.format {
            ImageFormat::Png => 100,
            ImageFormat::Rgb => 24,
            ImageFormat::Rgba => 32,
        };

        // Raw pixels need their size (`s`, `v`); PNG carries its own.
        let size = match self.format {
            ImageFormat::Png => String::new(),
            ImageFormat::Rgb | ImageFormat::Rgba => {
                format!("s={},v={},", self.width, self.height)
            }
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
                    "\x1b_Ga=T,f={},{}i={},c={},r={},m={};{}\x1b\\",
                    format_code, size, self.id, cols, rows, more, chunk
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_pixels_carry_their_size() {
        // Kitty cannot read raw RGB/RGBA data without its pixel size.
        let rgba = Image::from_rgba(vec![0; 3 * 2 * 4], 3, 2).kitty_escape(1, 1);
        assert!(rgba.starts_with("\x1b_Ga=T,f=32,s=3,v=2,"), "{rgba:?}");
        let rgb = Image::from_rgb(vec![0; 3 * 2 * 3], 3, 2).kitty_escape(1, 1);
        assert!(rgb.starts_with("\x1b_Ga=T,f=24,s=3,v=2,"), "{rgb:?}");
    }
}
