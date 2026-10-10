//! Checkbox widget tests

mod basic;
mod events;
mod state;

/// Test Checkbox edge cases
mod edge_cases {
    use revue::layout::Rect;
    use revue::render::Buffer;
    use revue::widget::traits::{RenderContext, View};
    use revue::widget::Checkbox;
    #[test]
    fn test_checkbox_with_empty_label() {
        let checkbox = Checkbox::new("");
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        checkbox.render(&mut ctx);
    }

    #[test]
    fn test_checkbox_with_very_long_label() {
        let long_label = "A".repeat(1000);
        let checkbox = Checkbox::new(&long_label);
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        checkbox.render(&mut ctx);
    }

    #[test]
    fn test_checkbox_toggle_multiple_times() {
        let mut checkbox = Checkbox::new("Test");

        // Toggle many times using builder pattern
        for i in 0..100 {
            checkbox = checkbox.checked(i % 2 == 1);
        }

        // 99 is odd, so checked(true)
        assert!(checkbox.is_checked());
    }

    #[test]
    fn test_checkbox_with_unicode_label() {
        let checkbox = Checkbox::new("✅ 체크박스");
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        checkbox.render(&mut ctx);
    }

    #[test]
    fn test_checkbox_disabled_checked() {
        let checkbox = Checkbox::new("Test").checked(true).disabled(true);
        assert!(checkbox.is_checked());
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        checkbox.render(&mut ctx);
    }
}
