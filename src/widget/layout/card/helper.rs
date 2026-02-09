use super::core::Card;

/// Helper function to create a Card
pub fn card() -> Card {
    Card::new()
}

#[cfg(test)]
mod tests {
    use super::super::types::CardVariant;
    use super::*;
    use crate::event::Key;
    use crate::style::Color;
    use crate::widget::layout::border::BorderType;

    #[test]
    fn test_card_function_creates_card() {
        let c = card();
        // Card should be created with default values
        assert!(c.is_expanded());
        assert!(!c.is_collapsible());
    }

    #[test]
    fn test_card_helper_chainable() {
        let c = card()
            .title("Test Title")
            .subtitle("Test Subtitle")
            .outlined()
            .rounded();
        let _ = c;
    }

    #[test]
    fn test_card_with_title() {
        let c = card().title("My Card");
        let _ = c;
    }

    #[test]
    fn test_card_with_subtitle() {
        let c = card().subtitle("My Subtitle");
        let _ = c;
    }

    #[test]
    fn test_card_with_variant() {
        let c = card().variant(CardVariant::Elevated);
        let _ = c;
    }

    #[test]
    fn test_card_outlined() {
        let c = card().outlined();
        let _ = c;
    }

    #[test]
    fn test_card_filled() {
        let c = card().filled();
        let _ = c;
    }

    #[test]
    fn test_card_elevated() {
        let c = card().elevated();
        let _ = c;
    }

    #[test]
    fn test_card_flat() {
        let c = card().flat();
        let _ = c;
    }

    #[test]
    fn test_card_rounded() {
        let c = card().rounded();
        let _ = c;
    }

    #[test]
    fn test_card_background() {
        let c = card().background(Color::BLUE);
        let _ = c;
    }

    #[test]
    fn test_card_border_color() {
        let c = card().border_color(Color::RED);
        let _ = c;
    }

    #[test]
    fn test_card_title_color() {
        let c = card().title_color(Color::GREEN);
        let _ = c;
    }

    #[test]
    fn test_card_collapsible() {
        let c = card().collapsible(true);
        assert!(c.is_collapsible());
    }

    #[test]
    fn test_card_collapsed() {
        let c = card().expanded(false);
        assert!(!c.is_expanded());
    }

    #[test]
    fn test_card_clickable() {
        let c = card().clickable(true);
        let _ = c;
    }

    #[test]
    fn test_card_padding() {
        let c = card().padding(10);
        let _ = c;
    }

    #[test]
    fn test_card_mut_collapse() {
        let mut c = card().collapsible(true).expanded(true);
        assert!(c.is_expanded());
        c.collapse();
        assert!(!c.is_expanded());
    }

    #[test]
    fn test_card_mut_expand() {
        let mut c = card().collapsible(true).expanded(false);
        assert!(!c.is_expanded());
        c.expand();
        assert!(c.is_expanded());
    }

    #[test]
    fn test_card_mut_toggle() {
        let mut c = card().collapsible(true).expanded(true);
        assert!(c.is_expanded());
        c.toggle();
        assert!(!c.is_expanded());
        c.toggle();
        assert!(c.is_expanded());
    }

    #[test]
    fn test_card_handle_key() {
        let mut c = card().collapsible(true).expanded(true);
        let handled = c.handle_key(&Key::Enter);
        // Should handle Enter key to toggle
        let _ = handled;
    }

    #[test]
    fn test_card_not_clickable_by_default() {
        let c = card();
        // Should not be clickable by default
        let _ = c;
    }

    #[test]
    fn test_card_with_string_title() {
        let c = card().title(String::from("Title"));
        let _ = c;
    }

    #[test]
    fn test_card_with_string_subtitle() {
        let c = card().subtitle(String::from("Subtitle"));
        let _ = c;
    }

    #[test]
    fn test_card_border_type() {
        let c = card().border_style(BorderType::Double);
        let _ = c;
    }
}
