//! Image protocol support for terminal graphics
//!
//! Supports multiple terminal image protocols:
//! - Kitty graphics protocol (most capable, modern)
//! - iTerm2 inline images (OSC 1337, macOS)
//! - Sixel graphics (legacy, wide support)

mod encoder;
mod iterm2;
mod kitty;
mod protocol;
mod sixel;

pub use encoder::{ImageEncoder, ImageError, PixelFormat};
pub use iterm2::Iterm2Image;
pub use kitty::KittyImage;
pub use protocol::{GraphicsCapabilities, ImageProtocol};
pub use sixel::SixelEncoder;
