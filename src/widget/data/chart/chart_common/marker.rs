/// Marker style for data points
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Marker {
    /// No marker
    #[default]
    None,
    /// Dot marker (•)
    Dot,
    /// Circle marker (○)
    Circle,
    /// Filled circle (●)
    FilledCircle,
    /// Square marker (□)
    Square,
    /// Filled square (■)
    FilledSquare,
    /// Diamond marker (◇)
    Diamond,
    /// Filled diamond (◆)
    FilledDiamond,
    /// Triangle marker (△)
    Triangle,
    /// Filled triangle (▲)
    FilledTriangle,
    /// Cross marker (+)
    Cross,
    /// X marker (×)
    X,
    /// Star marker (★) - filled for backward compatibility
    Star,
    /// Outline star (☆)
    StarOutline,
    /// Braille dots for high resolution
    Braille,
}

impl Marker {
    /// Get the character for this marker
    pub fn char(&self) -> char {
        match self {
            Marker::None => ' ',
            Marker::Dot => '•',
            Marker::Circle => '○',
            Marker::FilledCircle => '●',
            Marker::Square => '□',
            Marker::FilledSquare => '■',
            Marker::Diamond => '◇',
            Marker::FilledDiamond => '◆',
            Marker::Triangle => '△',
            Marker::FilledTriangle => '▲',
            Marker::Cross => '+',
            Marker::X => '×',
            Marker::Star => '★',
            Marker::StarOutline => '☆',
            Marker::Braille => '⣿',
        }
    }
}
