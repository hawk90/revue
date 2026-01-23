//! Type definitions for figlet rendering

/// Figlet font style
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FigletFont {
    /// Block style using Unicode box-drawing characters
    #[default]
    Block,
    /// Slant style
    Slant,
    /// Simple banner style
    Banner,
    /// Small compact style
    Small,
    /// Mini style (3 rows)
    Mini,
}
