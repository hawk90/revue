/// Card visual variant
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CardVariant {
    /// Default with border
    #[default]
    Outlined,
    /// Filled background
    Filled,
    /// Elevated with shadow effect
    Elevated,
    /// Minimal without border
    Flat,
}
