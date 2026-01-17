//! Common UI symbols and characters

/// Common symbols used in TUI widgets
pub struct Symbols;

impl Symbols {
    // Arrows
    /// Left arrow
    pub const ARROW_LEFT: char = '←';
    /// Right arrow
    pub const ARROW_RIGHT: char = '→';
    /// Up arrow
    pub const ARROW_UP: char = '↑';
    /// Down arrow
    pub const ARROW_DOWN: char = '↓';

    // Triangles
    /// Triangle pointing right (filled)
    pub const TRIANGLE_RIGHT: char = '▶';
    /// Triangle pointing down (filled)
    pub const TRIANGLE_DOWN: char = '▼';
    /// Small triangle right
    pub const TRIANGLE_SMALL_RIGHT: char = '▸';
    /// Small triangle down
    pub const TRIANGLE_SMALL_DOWN: char = '▾';

    // Checkboxes
    /// Empty checkbox
    pub const CHECKBOX_EMPTY: char = '☐';
    /// Checked checkbox
    pub const CHECKBOX_CHECKED: char = '☑';
    /// Crossed checkbox
    pub const CHECKBOX_CROSSED: char = '☒';

    // Radio buttons
    /// Empty radio button
    pub const RADIO_EMPTY: char = '○';
    /// Selected radio button
    pub const RADIO_SELECTED: char = '●';

    // Stars
    /// Empty star
    pub const STAR_EMPTY: char = '☆';
    /// Filled star
    pub const STAR_FILLED: char = '★';

    // Progress
    /// Filled block
    pub const BLOCK_FULL: char = '█';
    /// 3/4 filled block
    pub const BLOCK_3_4: char = '▓';
    /// 1/2 filled block
    pub const BLOCK_HALF: char = '▒';
    /// 1/4 filled block
    pub const BLOCK_1_4: char = '░';
    /// Empty block
    pub const BLOCK_EMPTY: char = '░';
    /// Light shade block
    pub const BLOCK_LIGHT: char = '░';
    /// Medium shade block
    pub const BLOCK_MEDIUM: char = '▒';
    /// Dark shade block
    pub const BLOCK_DARK: char = '▓';

    // Separators
    /// Vertical bar separator
    pub const SEP_VERT: char = '│';
    /// Bullet point
    pub const BULLET: char = '•';
    /// Double angle right
    pub const CHEVRON_RIGHT: char = '»';
    /// Double angle left
    pub const CHEVRON_LEFT: char = '«';
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arrow_symbols() {
        assert_eq!(Symbols::ARROW_LEFT, '←');
        assert_eq!(Symbols::ARROW_RIGHT, '→');
        assert_eq!(Symbols::ARROW_UP, '↑');
        assert_eq!(Symbols::ARROW_DOWN, '↓');
    }

    #[test]
    fn test_triangle_symbols() {
        assert_eq!(Symbols::TRIANGLE_RIGHT, '▶');
        assert_eq!(Symbols::TRIANGLE_DOWN, '▼');
        assert_eq!(Symbols::TRIANGLE_SMALL_RIGHT, '▸');
        assert_eq!(Symbols::TRIANGLE_SMALL_DOWN, '▾');
    }

    #[test]
    fn test_checkbox_symbols() {
        assert_eq!(Symbols::CHECKBOX_EMPTY, '☐');
        assert_eq!(Symbols::CHECKBOX_CHECKED, '☑');
        assert_eq!(Symbols::CHECKBOX_CROSSED, '☒');
    }

    #[test]
    fn test_radio_symbols() {
        assert_eq!(Symbols::RADIO_EMPTY, '○');
        assert_eq!(Symbols::RADIO_SELECTED, '●');
    }

    #[test]
    fn test_star_symbols() {
        assert_eq!(Symbols::STAR_EMPTY, '☆');
        assert_eq!(Symbols::STAR_FILLED, '★');
    }

    #[test]
    fn test_block_symbols() {
        assert_eq!(Symbols::BLOCK_FULL, '█');
        assert_eq!(Symbols::BLOCK_3_4, '▓');
        assert_eq!(Symbols::BLOCK_HALF, '▒');
        assert_eq!(Symbols::BLOCK_1_4, '░');
        assert_eq!(Symbols::BLOCK_EMPTY, '░');
        assert_eq!(Symbols::BLOCK_LIGHT, '░');
        assert_eq!(Symbols::BLOCK_MEDIUM, '▒');
        assert_eq!(Symbols::BLOCK_DARK, '▓');
    }

    #[test]
    fn test_separator_symbols() {
        assert_eq!(Symbols::SEP_VERT, '│');
        assert_eq!(Symbols::BULLET, '•');
        assert_eq!(Symbols::CHEVRON_RIGHT, '»');
        assert_eq!(Symbols::CHEVRON_LEFT, '«');
    }
}
