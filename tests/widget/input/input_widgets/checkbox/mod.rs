//! Checkbox widget tests

mod basic;
mod events;
mod state;
mod view;

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
        // Just the box; measured without the label's gap
        let row: String = (0..10).map(|x| buffer.get(x, 0).unwrap().symbol).collect();
        assert_eq!(row.trim_end(), "[ ]");
        assert_eq!(checkbox.measure(10, 10), Some((3, 1)));
    }

    #[test]
    fn test_checkbox_with_very_long_label() {
        let long_label = "A".repeat(1000);
        let checkbox = Checkbox::new(&long_label);
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        checkbox.render(&mut ctx);
        // Clipped at the area: the box, a space and six letters, on one row
        let row: String = (0..10).map(|x| buffer.get(x, 0).unwrap().symbol).collect();
        assert_eq!(row, "[ ] AAAAAA");
        assert!((0..10).all(|x| buffer.get(x, 1).unwrap().symbol == ' '));
        assert_eq!(checkbox.measure(10, 10), Some((10, 1)));
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
        // Wide glyphs: each takes two columns, the second a continuation
        assert_eq!(buffer.get(4, 0).unwrap().symbol, '✅');
        assert_eq!(buffer.get(7, 0).unwrap().symbol, '체');
        assert_eq!(checkbox.measure(20, 10), Some((15, 1)));
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

mod snapshots {
    use revue::prelude::*;
    use revue::testing::{Pilot, TestApp};

    #[test]
    fn test_checkbox_basic() {
        let view = vstack()
            .child(checkbox("Option 1"))
            .child(checkbox("Option 2"))
            .child(checkbox("Option 3"));

        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("checkbox_basic");
    }

    #[test]
    fn test_checkbox_checked() {
        let view = vstack()
            .child(checkbox("Unchecked"))
            .child(Checkbox::new("Checked").checked(true))
            .child(Checkbox::new("Disabled").disabled(true));

        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("checkbox_checked");
    }
}
