//! Kitty graphics protocol escape sequences

use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};

/// Kitty specific image operations
pub struct KittyImage;

impl KittyImage {
    /// Delete an image by ID
    pub fn delete(image_id: u32) -> String {
        format!("\x1b_Ga=d,i={}\x1b\\", image_id)
    }

    /// Delete all images
    pub fn delete_all() -> String {
        "\x1b_Ga=d\x1b\\".to_string()
    }

    /// Move/animate an image
    pub fn move_image(image_id: u32, x: u16, y: u16) -> String {
        format!("\x1b_Ga=p,i={},x={},y={}\x1b\\", image_id, x, y)
    }

    /// Create image with placement ID for animation
    pub fn with_placement(
        image_id: u32,
        placement_id: u32,
        data: &[u8],
        cols: u16,
        rows: u16,
    ) -> String {
        let encoded = BASE64.encode(data);
        let chunks: Vec<&str> = encoded
            .as_bytes()
            .chunks(4096)
            .map(|c| std::str::from_utf8(c).unwrap_or(""))
            .collect();

        let mut output = String::new();

        for (i, chunk) in chunks.iter().enumerate() {
            let is_first = i == 0;
            let is_last = i == chunks.len() - 1;
            let more = if is_last { 0 } else { 1 };

            {
                use std::fmt::Write;
                if is_first {
                    let _ = write!(
                        output,
                        "\x1b_Ga=T,f=100,i={},p={},c={},r={},m={};{}\x1b\\",
                        image_id, placement_id, cols, rows, more, chunk
                    );
                } else {
                    let _ = write!(output, "\x1b_Gm={};{}\x1b\\", more, chunk);
                }
            }
        }

        output
    }

    /// Query terminal for Kitty graphics support
    pub fn query_support() -> String {
        "\x1b_Gi=31,s=1,v=1,a=q,t=d,f=24;AAAA\x1b\\".to_string()
    }
}
