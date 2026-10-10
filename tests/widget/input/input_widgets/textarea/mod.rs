//! TextArea widget tests

mod bounds;
mod content;
mod cursor;
mod find_replace;
mod selection;
mod undo;
mod unicode;
mod vertical_scroll;
mod wrap;

mod snapshots {

    use revue::testing::{Pilot, TestApp};

    #[test]
    fn test_textarea_basic() {
        use revue::widget::TextArea;

        let view =
            TextArea::new().content("Hello, World!\nThis is a multi-line text area.\nLine 3 here.");

        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("textarea_basic");
    }

    #[test]
    fn test_textarea_with_line_numbers() {
        use revue::widget::TextArea;

        let view = TextArea::new()
            .content("fn main() {\n    println!(\"Hello\");\n}")
            .line_numbers(true);

        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("textarea_line_numbers");
    }

    #[test]
    fn test_textarea_with_placeholder() {
        use revue::widget::TextArea;

        let view = TextArea::new().placeholder("Enter your code here...");

        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("textarea_placeholder");
    }
}
