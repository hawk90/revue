//! Implementations for type-related methods

use super::types::PropertySource;
use super::types::StyleCategory;

impl PropertySource {
    /// Get display label
    pub fn label(&self) -> &'static str {
        match self {
            Self::Inline => "inline",
            Self::Class => "class",
            Self::Id => "id",
            Self::Inherited => "inherited",
            Self::Computed => "computed",
            Self::Theme => "theme",
        }
    }

    /// Get icon
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Inline => "•",
            Self::Class => ".",
            Self::Id => "#",
            Self::Inherited => "↑",
            Self::Computed => "○",
            Self::Theme => "◆",
        }
    }
}

impl StyleCategory {
    /// Get category label
    pub fn label(&self) -> &'static str {
        match self {
            Self::Layout => "Layout",
            Self::Typography => "Typography",
            Self::Colors => "Colors",
            Self::Border => "Border",
            Self::Effects => "Effects",
            Self::Other => "Other",
        }
    }

    /// All categories
    pub fn all() -> &'static [StyleCategory] {
        &[
            Self::Layout,
            Self::Typography,
            Self::Colors,
            Self::Border,
            Self::Effects,
            Self::Other,
        ]
    }

    /// Categorize a property name
    pub fn from_property(name: &str) -> Self {
        match name {
            // Border must come before layout (border-width vs width)
            n if n.starts_with("border") => Self::Border,
            n if n.starts_with("margin")
                || n.starts_with("padding")
                || n.contains("width")
                || n.contains("height")
                || n.starts_with("flex")
                || n.starts_with("grid")
                || n == "display"
                || n == "position" =>
            {
                Self::Layout
            }
            n if n.starts_with("font")
                || n.starts_with("text")
                || n == "line-height"
                || n == "letter-spacing" =>
            {
                Self::Typography
            }
            n if n.contains("color") || n.contains("background") => Self::Colors,
            n if n.contains("shadow") || n == "opacity" || n.starts_with("transform") => {
                Self::Effects
            }
            _ => Self::Other,
        }
    }
}
