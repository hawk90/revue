//! Tooltip position, arrow and style options

use crate::style::Color;
use crate::utils::border::BorderChars;
use crate::widget::theme::{DARK_BG, EDITOR_BG};

/// Tooltip position relative to anchor
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TooltipPosition {
    /// Above the anchor
    #[default]
    Top,
    /// Below the anchor
    Bottom,
    /// To the left of anchor
    Left,
    /// To the right of anchor
    Right,
    /// Auto-detect best position
    Auto,
}

/// Tooltip arrow style
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TooltipArrow {
    /// No arrow
    #[default]
    None,
    /// Simple arrow
    Simple,
    /// Unicode arrow
    Unicode,
}

/// Tooltip style
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TooltipStyle {
    /// Simple text
    #[default]
    Plain,
    /// With border
    Bordered,
    /// Rounded corners
    Rounded,
    /// Info style (cyan)
    Info,
    /// Warning style (yellow)
    Warning,
    /// Error style (red)
    Error,
    /// Success style (green)
    Success,
}

impl TooltipStyle {
    pub(super) fn colors(&self) -> (Color, Color) {
        match self {
            TooltipStyle::Plain => (Color::WHITE, DARK_BG),
            TooltipStyle::Bordered => (Color::WHITE, EDITOR_BG),
            TooltipStyle::Rounded => (Color::WHITE, EDITOR_BG),
            TooltipStyle::Info => (Color::WHITE, Color::rgb(30, 80, 100)),
            TooltipStyle::Warning => (Color::BLACK, Color::rgb(180, 150, 0)),
            TooltipStyle::Error => (Color::WHITE, Color::rgb(150, 30, 30)),
            TooltipStyle::Success => (Color::WHITE, Color::rgb(30, 100, 50)),
        }
    }

    pub(super) fn border_chars(&self) -> Option<BorderChars> {
        match self {
            TooltipStyle::Plain => None,
            TooltipStyle::Bordered
            | TooltipStyle::Info
            | TooltipStyle::Warning
            | TooltipStyle::Error
            | TooltipStyle::Success => Some(BorderChars::SINGLE),
            TooltipStyle::Rounded => Some(BorderChars::ROUNDED),
        }
    }
}
