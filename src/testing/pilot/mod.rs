//! Pilot - automated UI testing controller

mod async_pilot;
mod core;

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::event::Key;
    use crate::testing::pilot::Pilot;
    use crate::testing::{Action, KeyAction, MouseAction, TestApp, TestConfig};
    use crate::widget::{RenderContext, Text, View};

    struct SimpleView {
        text: String,
    }

    impl View for SimpleView {
        fn render(&self, ctx: &mut RenderContext) {
            Text::new(&self.text).render(ctx);
        }
    }

    #[test]
    fn test_pilot_screen_contains() {
        let view = SimpleView {
            text: "Hello, World!".to_string(),
        };
        let mut app = TestApp::new(view);
        let pilot = Pilot::new(&mut app);

        assert!(pilot.screen_contains("Hello"));
        assert!(pilot.screen_contains("World"));
        assert!(!pilot.screen_contains("Goodbye"));
    }

    #[test]
    fn test_pilot_type_text() {
        let view = SimpleView {
            text: "Input: ".to_string(),
        };
        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        pilot.type_text("test");
        assert_eq!(pilot.history().len(), 5); // 4 chars + 1 Type action
    }

    #[test]
    fn test_pilot_with_config() {
        let view = SimpleView {
            text: "Test".to_string(),
        };
        let mut app = TestApp::new(view);
        let config = TestConfig {
            width: 100,
            height: 50,
            timeout_ms: 5000,
            debug: false,
        };
        let pilot = Pilot::with_config(&mut app, config);
        assert_eq!(pilot.size(), (100, 50));
    }

    #[test]
    fn test_pilot_press_key() {
        let view = SimpleView {
            text: "Test".to_string(),
        };
        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        pilot.press_key(Key::Char('a'));
        assert_eq!(pilot.history().len(), 1);
        assert!(matches!(
            pilot.history()[0],
            Action::Key(KeyAction::Press(Key::Char('a')))
        ));
    }

    #[test]
    fn test_pilot_press_ctrl() {
        let view = SimpleView {
            text: "Test".to_string(),
        };
        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        pilot.press_ctrl(Key::Char('c'));
        assert_eq!(pilot.history().len(), 1);
        assert!(matches!(
            pilot.history()[0],
            Action::Key(KeyAction::PressCtrl(Key::Char('c')))
        ));
    }

    #[test]
    fn test_pilot_press_alt() {
        let view = SimpleView {
            text: "Test".to_string(),
        };
        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        pilot.press_alt(Key::Char('x'));
        assert_eq!(pilot.history().len(), 1);
        assert!(matches!(
            pilot.history()[0],
            Action::Key(KeyAction::PressAlt(Key::Char('x')))
        ));
    }

    #[test]
    fn test_pilot_press_enter() {
        let view = SimpleView {
            text: "Test".to_string(),
        };
        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        pilot.press_enter();
        assert!(matches!(
            pilot.history()[0],
            Action::Key(KeyAction::Press(Key::Enter))
        ));
    }

    #[test]
    fn test_pilot_press_escape() {
        let view = SimpleView {
            text: "Test".to_string(),
        };
        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        pilot.press_escape();
        assert!(matches!(
            pilot.history()[0],
            Action::Key(KeyAction::Press(Key::Escape))
        ));
    }

    #[test]
    fn test_pilot_press_tab() {
        let view = SimpleView {
            text: "Test".to_string(),
        };
        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        pilot.press_tab();
        assert!(matches!(
            pilot.history()[0],
            Action::Key(KeyAction::Press(Key::Tab))
        ));
    }

    #[test]
    fn test_pilot_press_backtab() {
        let view = SimpleView {
            text: "Test".to_string(),
        };
        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        pilot.press_backtab();
        assert!(matches!(
            pilot.history()[0],
            Action::Key(KeyAction::Press(Key::BackTab))
        ));
    }

    #[test]
    fn test_pilot_press_arrows() {
        let view = SimpleView {
            text: "Test".to_string(),
        };
        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        pilot.press_up().press_down().press_left().press_right();

        assert!(matches!(
            pilot.history()[0],
            Action::Key(KeyAction::Press(Key::Up))
        ));
        assert!(matches!(
            pilot.history()[1],
            Action::Key(KeyAction::Press(Key::Down))
        ));
        assert!(matches!(
            pilot.history()[2],
            Action::Key(KeyAction::Press(Key::Left))
        ));
        assert!(matches!(
            pilot.history()[3],
            Action::Key(KeyAction::Press(Key::Right))
        ));
    }

    #[test]
    fn test_pilot_press_ctrl_c() {
        let view = SimpleView {
            text: "Test".to_string(),
        };
        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        pilot.press_ctrl_c();
        assert!(matches!(
            pilot.history()[0],
            Action::Key(KeyAction::PressCtrl(Key::Char('c')))
        ));
    }

    #[test]
    fn test_pilot_click() {
        let view = SimpleView {
            text: "Test".to_string(),
        };
        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        pilot.click(10, 5);
        assert!(matches!(
            pilot.history()[0],
            Action::Mouse(MouseAction::Click(10, 5))
        ));
    }

    #[test]
    fn test_pilot_double_click() {
        let view = SimpleView {
            text: "Test".to_string(),
        };
        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        pilot.double_click(10, 5);
        // Double click adds 2 clicks + 1 double click action
        assert_eq!(pilot.history().len(), 3);
        assert!(matches!(
            pilot.history()[2],
            Action::Mouse(MouseAction::DoubleClick(10, 5))
        ));
    }

    #[test]
    fn test_pilot_scroll_up() {
        let view = SimpleView {
            text: "Test".to_string(),
        };
        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        pilot.scroll_up(10, 5, 3);
        assert!(matches!(
            pilot.history()[0],
            Action::Mouse(MouseAction::ScrollUp(10, 5, 3))
        ));
    }

    #[test]
    fn test_pilot_scroll_down() {
        let view = SimpleView {
            text: "Test".to_string(),
        };
        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        pilot.scroll_down(10, 5, 3);
        assert!(matches!(
            pilot.history()[0],
            Action::Mouse(MouseAction::ScrollDown(10, 5, 3))
        ));
    }

    #[test]
    fn test_pilot_wait() {
        let view = SimpleView {
            text: "Test".to_string(),
        };
        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        let start = std::time::Instant::now();
        pilot.wait(Duration::from_millis(10));
        let elapsed = start.elapsed();

        assert!(elapsed >= Duration::from_millis(10));
        assert!(matches!(pilot.history()[0], Action::Wait(_)));
    }

    #[test]
    fn test_pilot_wait_ms() {
        let view = SimpleView {
            text: "Test".to_string(),
        };
        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        pilot.wait_ms(1);
        assert!(matches!(pilot.history()[0], Action::Wait(_)));
    }

    #[test]
    fn test_pilot_assert_contains() {
        let view = SimpleView {
            text: "Hello, World!".to_string(),
        };
        let mut app = TestApp::new(view);
        let pilot = Pilot::new(&mut app);

        pilot.assert_contains("Hello");
        pilot.assert_contains("World");
    }

    #[test]
    fn test_pilot_assert_not_contains() {
        let view = SimpleView {
            text: "Hello".to_string(),
        };
        let mut app = TestApp::new(view);
        let pilot = Pilot::new(&mut app);

        pilot.assert_not_contains("Goodbye");
    }

    #[test]
    fn test_pilot_line() {
        let view = SimpleView {
            text: "Hello".to_string(),
        };
        let mut app = TestApp::new(view);
        let pilot = Pilot::new(&mut app);

        let line = pilot.line(0);
        assert!(line.contains("Hello"));
    }

    #[test]
    fn test_pilot_assert_line_contains() {
        let view = SimpleView {
            text: "Hello".to_string(),
        };
        let mut app = TestApp::new(view);
        let pilot = Pilot::new(&mut app);

        pilot.assert_line_contains(0, "Hello");
    }

    #[test]
    fn test_pilot_cell() {
        let view = SimpleView {
            text: "Hello".to_string(),
        };
        let mut app = TestApp::new(view);
        let pilot = Pilot::new(&mut app);

        let cell = pilot.cell(0, 0);
        assert_eq!(cell, Some('H'));
    }

    #[test]
    fn test_pilot_assert_cell() {
        let view = SimpleView {
            text: "Hello".to_string(),
        };
        let mut app = TestApp::new(view);
        let pilot = Pilot::new(&mut app);

        pilot.assert_cell(0, 0, 'H');
        pilot.assert_cell(1, 0, 'e');
    }

    #[test]
    fn test_pilot_screen() {
        let view = SimpleView {
            text: "Hello".to_string(),
        };
        let mut app = TestApp::new(view);
        let pilot = Pilot::new(&mut app);

        let screen = pilot.screen();
        assert!(screen.contains("Hello"));
    }

    #[test]
    fn test_pilot_find_text() {
        let view = SimpleView {
            text: "Hello".to_string(),
        };
        let mut app = TestApp::new(view);
        let pilot = Pilot::new(&mut app);

        let pos = pilot.find_text("Hello");
        assert!(pos.is_some());
    }

    #[test]
    fn test_pilot_find_text_not_found() {
        let view = SimpleView {
            text: "Hello".to_string(),
        };
        let mut app = TestApp::new(view);
        let pilot = Pilot::new(&mut app);

        let pos = pilot.find_text("Goodbye");
        assert!(pos.is_none());
    }

    #[test]
    fn test_pilot_click_text() {
        let view = SimpleView {
            text: "Hello".to_string(),
        };
        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        pilot.click_text("Hello");
        assert!(!pilot.history().is_empty());
    }

    #[test]
    fn test_pilot_history_and_clear() {
        let view = SimpleView {
            text: "Test".to_string(),
        };
        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        assert!(pilot.history().is_empty());

        pilot.press_key(Key::Char('a'));
        assert_eq!(pilot.history().len(), 1);

        pilot.clear_history();
        assert!(pilot.history().is_empty());
    }

    #[test]
    fn test_pilot_size() {
        let view = SimpleView {
            text: "Test".to_string(),
        };
        let mut app = TestApp::new(view);
        let pilot = Pilot::new(&mut app);

        assert_eq!(pilot.size(), (80, 24)); // Default TestConfig size
    }

    #[test]
    fn test_pilot_resize() {
        let view = SimpleView {
            text: "Test".to_string(),
        };
        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        pilot.resize(100, 50);
        assert_eq!(pilot.size(), (100, 50));
    }

    #[test]
    fn test_pilot_chaining() {
        let view = SimpleView {
            text: "Test".to_string(),
        };
        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        // Test method chaining
        pilot
            .press_key(Key::Char('a'))
            .press_enter()
            .press_escape()
            .click(0, 0);

        assert_eq!(pilot.history().len(), 4);
    }
}

// Re-exports
pub use async_pilot::AsyncPilot;
pub use core::Pilot;
