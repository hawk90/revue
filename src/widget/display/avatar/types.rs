//! Avatar size and shape options

/// Avatar size
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AvatarSize {
    /// Small (1 char)
    Small,
    /// Medium (3 chars)
    #[default]
    Medium,
    /// Large (5 chars with border)
    Large,
}

/// Avatar shape
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AvatarShape {
    /// Circle (using Unicode characters)
    #[default]
    Circle,
    /// Square/box
    Square,
    /// Rounded square
    Rounded,
}
