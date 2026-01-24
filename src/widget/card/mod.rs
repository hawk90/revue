//! Card widget for grouping related content with visual boundaries
//!
//! Cards provide a structured container with optional header, body, and footer sections.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::widget::{Card, card};
//!
//! // Basic card with title
//! Card::new()
//!     .title("User Profile")
//!     .body(user_info_widget);
//!
//! // Card with header, body, and footer
//! card()
//!     .title("Settings")
//!     .subtitle("Configure your preferences")
//!     .body(settings_form)
//!     .footer(action_buttons);
//!
//! // Collapsible card
//! Card::new()
//!     .title("Details")
//!     .collapsible(true)
//!     .body(details_content);
//! ```

mod core;
mod helper;
mod types;

#[cfg(test)]
mod tests {
    use super::*;

    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::style::{BorderStyle, Color};

    #[test]
    fn test_card_new() {
        let c = Card::new();
        assert!(c.title.is_none());
        assert!(c.body.is_none());
        assert!(!c.collapsible);
        assert!(c.expanded);
    }

    #[test]
    fn test_card_title() {
        let c = Card::new().title("My Card");
        assert_eq!(c.title, Some("My Card".to_string()));
    }

    #[test]
    fn test_card_subtitle() {
        let c = Card::new().subtitle("Description");
        assert_eq!(c.subtitle, Some("Description".to_string()));
    }

    #[test]
    fn test_card_variants() {
        let c = Card::new().outlined();
        assert_eq!(c.variant, CardVariant::Outlined);

        let c = Card::new().filled();
        assert_eq!(c.variant, CardVariant::Filled);

        let c = Card::new().elevated();
        assert_eq!(c.variant, CardVariant::Elevated);

        let c = Card::new().flat();
        assert_eq!(c.variant, CardVariant::Flat);
    }

    #[test]
    fn test_card_border_styles() {
        let c = Card::new().rounded();
        assert_eq!(c.border, BorderStyle::Rounded);

        let c = Card::new().border_style(BorderStyle::Double);
        assert_eq!(c.border, BorderStyle::Double);
    }

    #[test]
    fn test_card_collapsible() {
        let mut c = Card::new().collapsible(true);
        assert!(c.is_collapsible());
        assert!(c.is_expanded());

        c.toggle();
        assert!(!c.is_expanded());

        c.expand();
        assert!(c.is_expanded());

        c.collapse();
        assert!(!c.is_expanded());
    }

    #[test]
    fn test_card_collapsible_toggle_not_collapsible() {
        let mut c = Card::new().collapsible(false);
        c.toggle();
        assert!(c.is_expanded()); // Should remain expanded
    }

    #[test]
    fn test_card_handle_key() {
        let mut c = Card::new().collapsible(true);

        assert!(c.handle_key(&Key::Enter));
        assert!(!c.is_expanded());

        assert!(c.handle_key(&Key::Char(' ')));
        assert!(c.is_expanded());

        assert!(c.handle_key(&Key::Left));
        assert!(!c.is_expanded());

        assert!(c.handle_key(&Key::Right));
        assert!(c.is_expanded());
    }

    #[test]
    fn test_card_handle_key_not_collapsible() {
        let mut c = Card::new();
        assert!(!c.handle_key(&Key::Enter));
    }

    #[test]
    fn test_card_handle_key_disabled() {
        let mut c = Card::new().collapsible(true).disabled(true);
        assert!(!c.handle_key(&Key::Enter));
    }

    #[test]
    fn test_card_colors() {
        let c = Card::new()
            .background(Color::RED)
            .border_color(Color::BLUE)
            .title_color(Color::GREEN);

        assert_eq!(c.bg_color, Some(Color::RED));
        assert_eq!(c.border_color, Some(Color::BLUE));
        assert_eq!(c.title_color, Some(Color::GREEN));
    }

    #[test]
    fn test_card_padding() {
        let c = Card::new().padding(2);
        assert_eq!(c.padding, 2);
    }

    #[test]
    fn test_card_render_basic() {
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let c = Card::new().title("Test Card");
        c.render(&mut ctx);

        // Check corners
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
        assert_eq!(buffer.get(19, 0).unwrap().symbol, '┐');
        assert_eq!(buffer.get(0, 9).unwrap().symbol, '└');
        assert_eq!(buffer.get(19, 9).unwrap().symbol, '┘');
    }

    #[test]
    fn test_card_render_rounded() {
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let c = Card::new().rounded().title("Rounded");
        c.render(&mut ctx);

        assert_eq!(buffer.get(0, 0).unwrap().symbol, '╭');
        assert_eq!(buffer.get(19, 0).unwrap().symbol, '╮');
    }

    #[test]
    fn test_card_with_body() {
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let c = Card::new().title("Title").body(Text::new("Body content"));
        c.render(&mut ctx);

        // Should have separator
        assert_eq!(buffer.get(0, 2).unwrap().symbol, '├');
    }

    #[test]
    fn test_card_default() {
        let c = Card::default();
        assert!(c.title.is_none());
    }

    #[test]
    fn test_card_helper() {
        let c = card().title("Helper");
        assert_eq!(c.title, Some("Helper".to_string()));
    }

    #[test]
    fn test_card_border_chars() {
        let chars = BorderStyle::Single.chars();
        assert_eq!(chars.top_left, '┌');
        assert_eq!(chars.top_right, '┐');
        assert_eq!(chars.bottom_left, '└');
        assert_eq!(chars.bottom_right, '┘');
        assert_eq!(chars.horizontal, '─');
        assert_eq!(chars.vertical, '│');

        let chars = BorderStyle::Rounded.chars();
        assert_eq!(chars.top_left, '╭');
    }

    #[test]
    fn test_card_collapse_icon() {
        let c = Card::new().collapsible(true).expanded(true);
        assert_eq!(c.collapse_icon(), '▼');

        let c = Card::new().collapsible(true).expanded(false);
        assert_eq!(c.collapse_icon(), '▶');
    }

    #[test]
    fn test_card_effective_colors() {
        let c = Card::new().outlined();
        let (_, border, _) = c.effective_colors();
        assert_eq!(border, Color::rgb(60, 60, 70));

        let c = Card::new().filled();
        let (bg, _, _) = c.effective_colors();
        assert_eq!(bg, Some(Color::rgb(30, 30, 35)));
    }

    #[test]
    fn test_card_clickable() {
        let c = Card::new().clickable(true);
        assert!(c.clickable);
    }
}

// Re-exports
pub use core::Card;
pub use helper::card;
pub use types::CardVariant;
