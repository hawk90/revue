#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    #[test]
    fn test_modal_new() {
        let m = Modal::new();
        assert!(!m.is_visible());
        assert!(m.title.is_empty());
        assert!(m.content.is_empty());
        assert!(m.buttons.is_empty());
    }

    #[test]
    fn test_modal_builder() {
        let m = Modal::new()
            .title("Test")
            .content("Hello\nWorld")
            .ok_cancel();

        assert_eq!(m.title, "Test");
        assert_eq!(m.content.len(), 2);
        assert_eq!(m.buttons.len(), 2);
    }

    #[test]
    fn test_modal_visibility() {
        let mut m = Modal::new();
        assert!(!m.is_visible());

        m.show();
        assert!(m.is_visible());

        m.hide();
        assert!(!m.is_visible());

        m.toggle();
        assert!(m.is_visible());
    }

    #[test]
    fn test_modal_button_navigation() {
        let mut m = Modal::new().ok_cancel();

        assert_eq!(m.selected_button(), 0);

        m.next_button();
        assert_eq!(m.selected_button(), 1);

        m.next_button(); // Wraps around
        assert_eq!(m.selected_button(), 0);

        m.prev_button(); // Wraps around
        assert_eq!(m.selected_button(), 1);
    }

    #[test]
    fn test_modal_handle_key() {
        use crate::event::Key;

        let mut m = Modal::new().yes_no();
        m.show();

        // Navigate buttons
        m.handle_key(&Key::Right);
        assert_eq!(m.selected_button(), 1);

        m.handle_key(&Key::Left);
        assert_eq!(m.selected_button(), 0);

        // Confirm selection
        let result = m.handle_key(&Key::Enter);
        assert_eq!(result, Some(0));

        // Escape closes
        m.handle_key(&Key::Escape);
        assert!(!m.is_visible());
    }

    #[test]
    fn test_modal_presets() {
        let alert = Modal::alert("Title", "Message");
        assert_eq!(alert.title, "Title");
        assert_eq!(alert.buttons.len(), 1);

        let confirm = Modal::confirm("Title", "Question?");
        assert_eq!(confirm.buttons.len(), 2);

        let error = Modal::error("Something went wrong");
        assert_eq!(error.title, "Error");
    }

    #[test]
    fn test_modal_render_hidden() {
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let m = Modal::new().title("Test");
        m.render(&mut ctx);

        // Hidden modal shouldn't render anything special
        assert_eq!(buffer.get(0, 0).unwrap().symbol, ' ');
    }

    #[test]
    fn test_modal_render_visible() {
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut m = Modal::new().title("Test Dialog").content("Hello").ok();
        m.show();
        m.render(&mut ctx);

        // Modal should render centered - check for border characters
        // The exact position depends on centering calculation
        let center_x = (80 - 40) / 2;
        let center_y = (24 - m.required_height()) / 2;

        assert_eq!(buffer.get(center_x, center_y).unwrap().symbol, '┌');
    }

    #[test]
    fn test_modal_button_styles() {
        let btn = ModalButton::new("Test");
        assert!(matches!(btn.style, ModalButtonStyle::Default));

        let btn = ModalButton::primary("OK");
        assert!(matches!(btn.style, ModalButtonStyle::Primary));

        let btn = ModalButton::danger("Delete");
        assert!(matches!(btn.style, ModalButtonStyle::Danger));
    }

    #[test]
    fn test_modal_helper() {
        let m = modal().title("Quick").ok();

        assert_eq!(m.title, "Quick");
    }

    #[test]
    fn test_modal_with_body() {
        use crate::widget::Text;

        let m = Modal::new()
            .title("Form")
            .body(Text::new("Custom content"))
            .height(10)
            .ok();

        assert!(m.body.is_some());
        assert_eq!(m.height, Some(10));
    }

    #[test]
    fn test_modal_body_render() {
        use crate::widget::Text;

        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut m = Modal::new()
            .title("Body Test")
            .body(Text::new("Widget content"))
            .width(50)
            .height(12)
            .ok();
        m.show();
        m.render(&mut ctx);

        // Modal with body should render
        let center_x = (80 - 50) / 2;
        let center_y = (24 - 12) / 2;
        assert_eq!(buffer.get(center_x, center_y).unwrap().symbol, '┌');
    }

    #[test]
    fn test_modal_render_small_area_no_panic() {
        // Test that rendering in very small areas doesn't panic
        // This is the fix for issue #154

        // Width = 0
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 0, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);
        let mut m = Modal::new().title("Test").ok();
        m.show();
        m.render(&mut ctx); // Should not panic

        // Width = 1
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 1, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);
        let mut m = Modal::new().title("Test").ok();
        m.show();
        m.render(&mut ctx); // Should not panic

        // Width = 2
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 2, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);
        let mut m = Modal::new().title("Test").ok();
        m.show();
        m.render(&mut ctx); // Should not panic

        // Height = 0
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 0);
        let mut ctx = RenderContext::new(&mut buffer, area);
        let mut m = Modal::new().title("Test").ok();
        m.show();
        m.render(&mut ctx); // Should not panic

        // Height = 1
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        let mut m = Modal::new().title("Test").ok();
        m.show();
        m.render(&mut ctx); // Should not panic

        // Height = 2
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 2);
        let mut ctx = RenderContext::new(&mut buffer, area);
        let mut m = Modal::new().title("Test").ok();
        m.show();
        m.render(&mut ctx); // Should not panic

        // Both width and height = 0
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 0, 0);
        let mut ctx = RenderContext::new(&mut buffer, area);
        let mut m = Modal::new().title("Test").ok();
        m.show();
        m.render(&mut ctx); // Should not panic
    }

    #[test]
    fn test_modal_render_width_2_border() {
        // Specific test for width=2 which was mentioned in the issue
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 4, 10); // Small width after subtracting 4 for margins
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut m = Modal::new().title("X").width(2).height(4);
        m.show();
        m.render(&mut ctx); // Should not panic
    }
}
