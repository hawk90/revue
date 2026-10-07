//! iTerm2 inline image escape sequences

use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};

/// iTerm2 specific image operations
pub struct Iterm2Image;

impl Iterm2Image {
    /// Create an inline image escape sequence
    pub fn inline_image(
        data: &[u8],
        width: Option<u16>,
        height: Option<u16>,
        preserve_aspect: bool,
    ) -> String {
        let encoded = BASE64.encode(data);
        let filename = BASE64.encode("image.png");

        let mut args = vec![format!("name={}", filename), "inline=1".to_string()];

        if let Some(w) = width {
            args.push(format!("width={}", w));
        }
        if let Some(h) = height {
            args.push(format!("height={}", h));
        }
        if preserve_aspect {
            args.push("preserveAspectRatio=1".to_string());
        }

        args.push(format!("size={}", data.len()));

        format!("\x1b]1337;File={}:{}\x07", args.join(";"), encoded)
    }

    /// Create a cursor-positioned image (iTerm2 specific)
    pub fn positioned_image(data: &[u8], x: u16, y: u16, width: u16, height: u16) -> String {
        let encoded = BASE64.encode(data);
        let filename = BASE64.encode("image.png");

        format!(
            "\x1b[{};{}H\x1b]1337;File=name={};width={};height={};inline=1;size={}:{}\x07",
            y + 1,
            x + 1,
            filename,
            width,
            height,
            data.len(),
            encoded
        )
    }

    /// Set custom cursor shape using an image
    pub fn custom_cursor(_data: &[u8]) -> String {
        // Note: iTerm2 doesn't support custom cursor images via escape sequences
        // This is a placeholder for future compatibility
        String::new()
    }
}
