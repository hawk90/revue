//! QR code display style and error correction level

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
