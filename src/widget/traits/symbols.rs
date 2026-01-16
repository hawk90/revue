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
