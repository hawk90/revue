/// Popover position relative to anchor
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PopoverPosition {
    /// Above the anchor
    Top,
    /// Below the anchor
    #[default]
    Bottom,
    /// To the left of anchor
    Left,
    /// To the right of anchor
    Right,
    /// Auto-detect best position
    Auto,
}

/// Popover trigger type
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PopoverTrigger {
    /// Triggered by click
    #[default]
    Click,
    /// Triggered by hover
    Hover,
    /// Triggered by focus
    Focus,
    /// Manually controlled (no automatic trigger)
    Manual,
}

/// Popover arrow style
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PopoverArrow {
    /// No arrow
    #[default]
    None,
    /// Simple ASCII arrow
    Simple,
    /// Unicode arrow
    Unicode,
}

impl PopoverArrow {
    pub(crate) fn chars(&self, position: PopoverPosition) -> char {
        match (self, position) {
            (PopoverArrow::None, _) => ' ',
            (PopoverArrow::Simple, PopoverPosition::Top) => 'v',
            (PopoverArrow::Simple, PopoverPosition::Bottom) => '^',
            (PopoverArrow::Simple, PopoverPosition::Left) => '>',
            (PopoverArrow::Simple, PopoverPosition::Right) => '<',
            (PopoverArrow::Simple, PopoverPosition::Auto) => 'v',
            (PopoverArrow::Unicode, PopoverPosition::Top) => '▼',
            (PopoverArrow::Unicode, PopoverPosition::Bottom) => '▲',
            (PopoverArrow::Unicode, PopoverPosition::Left) => '▶',
            (PopoverArrow::Unicode, PopoverPosition::Right) => '◀',
            (PopoverArrow::Unicode, PopoverPosition::Auto) => '▼',
        }
    }
}

/// Popover visual style
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PopoverStyle {
    /// Default bordered style
    #[default]
    Default,
    /// Rounded corners
    Rounded,
    /// Minimal without border
    Minimal,
    /// Elevated with shadow effect
    Elevated,
}

impl PopoverStyle {
    pub(crate) fn colors(
        &self,
    ) -> (
        crate::style::Color,
        crate::style::Color,
        crate::style::Color,
    ) {
        use crate::style::Color;
        // (fg, bg, border)
        match self {
            PopoverStyle::Default => (Color::WHITE, Color::rgb(30, 30, 35), Color::rgb(70, 70, 80)),
            PopoverStyle::Rounded => (Color::WHITE, Color::rgb(35, 35, 40), Color::rgb(80, 80, 90)),
            PopoverStyle::Minimal => (Color::WHITE, Color::rgb(40, 40, 45), Color::rgb(40, 40, 45)),
            PopoverStyle::Elevated => {
                (Color::WHITE, Color::rgb(25, 25, 30), Color::rgb(60, 60, 70))
            }
        }
    }

    pub(crate) fn border_chars(&self) -> Option<crate::utils::border::BorderChars> {
        use crate::utils::border::BorderChars;
        match self {
            PopoverStyle::Minimal => None,
            PopoverStyle::Rounded => Some(BorderChars::ROUNDED),
            _ => Some(BorderChars::SINGLE),
        }
    }
}
