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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_variant_default() {
        let variant = CardVariant::default();
        assert_eq!(variant, CardVariant::Outlined);
    }

    #[test]
    fn test_card_variant_all_variants() {
        let _ = CardVariant::Outlined;
        let _ = CardVariant::Filled;
        let _ = CardVariant::Elevated;
        let _ = CardVariant::Flat;
    }

    #[test]
    fn test_card_variant_clone() {
        let variant = CardVariant::Elevated;
        let cloned = variant;
        assert_eq!(variant, cloned);
    }

    #[test]
    fn test_card_variant_equality() {
        assert_eq!(CardVariant::Outlined, CardVariant::Outlined);
        assert_eq!(CardVariant::Filled, CardVariant::Filled);
        assert_ne!(CardVariant::Outlined, CardVariant::Filled);
    }

    #[test]
    fn test_card_variant_copy() {
        let variant1 = CardVariant::Flat;
        let variant2 = variant1;
        assert_eq!(variant1, variant2);
    }
}
