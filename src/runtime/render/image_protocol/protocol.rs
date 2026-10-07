//! Image protocol detection and terminal graphics capabilities

use crate::utils::terminal::{is_sixel_capable, terminal_type, TerminalType};

/// Supported terminal image protocols
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ImageProtocol {
    /// Kitty graphics protocol (APC-based)
    #[default]
    Kitty,
    /// iTerm2 inline images (OSC 1337)
    Iterm2,
    /// Sixel graphics protocol
    Sixel,
    /// No graphics support, use placeholder
    None,
}

impl ImageProtocol {
    /// Detect the best available protocol for the current terminal
    pub fn detect() -> Self {
        // Check for user override first (highest priority)
        if let Ok(override_protocol) = std::env::var("REVUE_IMAGE_PROTOCOL") {
            return match override_protocol.to_lowercase().as_str() {
                "kitty" => ImageProtocol::Kitty,
                "iterm2" => ImageProtocol::Iterm2,
                "sixel" => ImageProtocol::Sixel,
                "none" => ImageProtocol::None,
                protocol => {
                    eprintln!(
                        "Unknown REVUE_IMAGE_PROTOCOL: {}, using auto-detection",
                        protocol
                    );
                    // Fall through to auto-detect
                    Self::detect_auto()
                }
            };
        }

        Self::detect_auto()
    }

    /// Auto-detect protocol without checking user override
    fn detect_auto() -> Self {
        // Use centralized terminal detection
        match terminal_type() {
            TerminalType::Kitty => ImageProtocol::Kitty,
            TerminalType::Iterm2 => ImageProtocol::Iterm2,
            TerminalType::Unknown => {
                // Check for Sixel support as fallback
                if is_sixel_capable() {
                    return ImageProtocol::Sixel;
                }
                ImageProtocol::None
            }
        }
    }

    /// Check if this protocol is supported
    pub fn is_supported(&self) -> bool {
        !matches!(self, ImageProtocol::None)
    }

    /// Get protocol name
    pub fn name(&self) -> &'static str {
        match self {
            ImageProtocol::Kitty => "Kitty",
            ImageProtocol::Iterm2 => "iTerm2",
            ImageProtocol::Sixel => "Sixel",
            ImageProtocol::None => "None",
        }
    }
}

/// Terminal capabilities for graphics
#[derive(Clone, Debug, Default)]
pub struct GraphicsCapabilities {
    /// Best available protocol
    pub protocol: ImageProtocol,
    /// Maximum image width supported
    pub max_width: Option<u32>,
    /// Maximum image height supported
    pub max_height: Option<u32>,
    /// Number of colors supported (for Sixel)
    pub color_depth: Option<u16>,
    /// Whether animation is supported
    pub animation: bool,
}

impl GraphicsCapabilities {
    /// Detect graphics capabilities
    pub fn detect() -> Self {
        let protocol = ImageProtocol::detect();

        let (max_width, max_height, animation) = match protocol {
            ImageProtocol::Kitty => (None, None, true), // No practical limits
            ImageProtocol::Iterm2 => (None, None, false), // No practical limits
            ImageProtocol::Sixel => (Some(4096), Some(4096), false), // Varies by terminal
            ImageProtocol::None => (None, None, false),
        };

        let color_depth = match protocol {
            ImageProtocol::Sixel => Some(256),
            _ => None,
        };

        Self {
            protocol,
            max_width,
            max_height,
            color_depth,
            animation,
        }
    }

    /// Check if any graphics protocol is supported
    pub fn has_graphics(&self) -> bool {
        self.protocol.is_supported()
    }
}
