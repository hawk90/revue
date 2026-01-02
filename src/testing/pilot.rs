//! Pilot - automated UI testing controller

use super::{Action, KeyAction, MouseAction, TestApp, TestConfig};
use crate::event::{Key, KeyEvent};
use crate::render::Buffer;
use std::time::Duration;

/// Pilot controller for automated testing
pub struct Pilot<'a, V: crate::widget::View> {
    /// The test app being controlled
    app: &'a mut TestApp<V>,
    /// Test configuration
    config: TestConfig,
    /// Action history
    history: Vec<Action>,
}

impl<'a, V: crate::widget::View> Pilot<'a, V> {
    /// Create a new pilot for a test app
    pub fn new(app: &'a mut TestApp<V>) -> Self {
        Self {
            app,
            config: TestConfig::default(),
            history: Vec::new(),
        }
    }

    /// Create with custom config
    pub fn with_config(app: &'a mut TestApp<V>, config: TestConfig) -> Self {
        Self {
            app,
            config,
            history: Vec::new(),
        }
    }

    // =========================================================================
    // Key Actions
    // =========================================================================

    /// Press a key
    pub fn press_key(&mut self, key: Key) -> &mut Self {
        let event = KeyEvent::new(key);
        self.app.send_key(event);
        self.history.push(Action::Key(KeyAction::Press(key)));
        self
    }

    /// Press a key with ctrl modifier
    pub fn press_ctrl(&mut self, key: Key) -> &mut Self {
        let event = KeyEvent::ctrl(key);
        self.app.send_key(event);
        self.history.push(Action::Key(KeyAction::PressCtrl(key)));
        self
    }

    /// Press a key with alt modifier
    pub fn press_alt(&mut self, key: Key) -> &mut Self {
        let event = KeyEvent::alt(key);
        self.app.send_key(event);
        self.history.push(Action::Key(KeyAction::PressAlt(key)));
        self
    }

    /// Press Enter
    pub fn press_enter(&mut self) -> &mut Self {
        self.press_key(Key::Enter)
    }

    /// Press Escape
    pub fn press_escape(&mut self) -> &mut Self {
        self.press_key(Key::Escape)
    }

    /// Press Tab
    pub fn press_tab(&mut self) -> &mut Self {
        self.press_key(Key::Tab)
    }

    /// Press Shift+Tab (back tab)
    pub fn press_backtab(&mut self) -> &mut Self {
        self.press_key(Key::BackTab)
    }

    /// Press arrow up
    pub fn press_up(&mut self) -> &mut Self {
        self.press_key(Key::Up)
    }

    /// Press arrow down
    pub fn press_down(&mut self) -> &mut Self {
        self.press_key(Key::Down)
    }

    /// Press arrow left
    pub fn press_left(&mut self) -> &mut Self {
        self.press_key(Key::Left)
    }

    /// Press arrow right
    pub fn press_right(&mut self) -> &mut Self {
        self.press_key(Key::Right)
    }

    /// Type a string (sends each character as a key press)
    pub fn type_text(&mut self, text: &str) -> &mut Self {
        for ch in text.chars() {
            self.press_key(Key::Char(ch));
        }
        self.history
            .push(Action::Key(KeyAction::Type(text.to_string())));
        self
    }

    /// Press Ctrl+C
    pub fn press_ctrl_c(&mut self) -> &mut Self {
        self.press_ctrl(Key::Char('c'))
    }

    // =========================================================================
    // Mouse Actions
    // =========================================================================

    /// Click at position
    pub fn click(&mut self, x: u16, y: u16) -> &mut Self {
        self.app.send_click(x, y);
        self.history.push(Action::Mouse(MouseAction::Click(x, y)));
        self
    }

    /// Double click at position
    pub fn double_click(&mut self, x: u16, y: u16) -> &mut Self {
        self.click(x, y);
        self.click(x, y);
        self.history
            .push(Action::Mouse(MouseAction::DoubleClick(x, y)));
        self
    }

    /// Scroll up at position
    pub fn scroll_up(&mut self, x: u16, y: u16, amount: u16) -> &mut Self {
        self.app.send_scroll(x, y, -(amount as i16));
        self.history
            .push(Action::Mouse(MouseAction::ScrollUp(x, y, amount)));
        self
    }

    /// Scroll down at position
    pub fn scroll_down(&mut self, x: u16, y: u16, amount: u16) -> &mut Self {
        self.app.send_scroll(x, y, amount as i16);
        self.history
            .push(Action::Mouse(MouseAction::ScrollDown(x, y, amount)));
        self
    }

    // =========================================================================
    // Timing
    // =========================================================================

    /// Wait for a duration
    pub fn wait(&mut self, duration: Duration) -> &mut Self {
        std::thread::sleep(duration);
        self.history.push(Action::Wait(duration));
        self
    }

    /// Wait for milliseconds
    pub fn wait_ms(&mut self, ms: u64) -> &mut Self {
        self.wait(Duration::from_millis(ms))
    }

    /// Wait for a condition to be true
    pub fn wait_until<F>(&mut self, condition: F) -> &mut Self
    where
        F: Fn(&Buffer) -> bool,
    {
        let start = std::time::Instant::now();
        let timeout = Duration::from_millis(self.config.timeout_ms);

        while !condition(self.app.buffer()) {
            if start.elapsed() >= timeout {
                panic!("wait_until timed out after {:?}", timeout);
            }
            std::thread::sleep(Duration::from_millis(10));
            self.app.render();
        }

        self
    }

    // =========================================================================
    // Assertions
    // =========================================================================

    /// Check if screen contains text
    pub fn screen_contains(&self, text: &str) -> bool {
        self.app.screen_text().contains(text)
    }

    /// Assert screen contains text
    pub fn assert_contains(&self, text: &str) {
        assert!(
            self.screen_contains(text),
            "Expected screen to contain '{}', but got:\n{}",
            text,
            self.app.screen_text()
        );
    }

    /// Assert screen does not contain text
    pub fn assert_not_contains(&self, text: &str) {
        assert!(
            !self.screen_contains(text),
            "Expected screen NOT to contain '{}', but it did:\n{}",
            text,
            self.app.screen_text()
        );
    }

    /// Get text at specific line
    pub fn line(&self, row: u16) -> String {
        self.app.get_line(row)
    }

    /// Assert line contains text
    pub fn assert_line_contains(&self, row: u16, text: &str) {
        let line = self.line(row);
        assert!(
            line.contains(text),
            "Expected line {} to contain '{}', but got: '{}'",
            row,
            text,
            line
        );
    }

    /// Get cell at position
    pub fn cell(&self, x: u16, y: u16) -> Option<char> {
        self.app.get_cell(x, y)
    }

    /// Assert cell equals
    pub fn assert_cell(&self, x: u16, y: u16, expected: char) {
        let actual = self.cell(x, y);
        assert_eq!(
            actual,
            Some(expected),
            "Expected cell ({}, {}) to be '{}', but got {:?}",
            x,
            y,
            expected,
            actual
        );
    }

    // =========================================================================
    // Snapshots
    // =========================================================================

    /// Get current screen as string
    pub fn screen(&self) -> String {
        self.app.screen_text()
    }

    /// Assert screen matches expected string
    pub fn assert_screen(&self, expected: &str) {
        let actual = self.screen();
        let expected_trimmed = expected.trim();
        let actual_trimmed = actual.trim();

        assert_eq!(
            actual_trimmed, expected_trimmed,
            "Screen does not match expected.\nExpected:\n{}\n\nActual:\n{}",
            expected_trimmed, actual_trimmed
        );
    }

    /// Capture snapshot or compare against existing snapshot
    ///
    /// Snapshots are stored in `tests/snapshots/` directory.
    /// Set `REVUE_UPDATE_SNAPSHOTS=1` environment variable to update snapshots.
    ///
    /// # Example
    ///
    /// ```ignore
    /// pilot.press_up()
    ///     .snapshot("counter_at_1")
    ///     .press_up()
    ///     .snapshot("counter_at_2");
    /// ```
    pub fn snapshot(&mut self, name: &str) -> &mut Self {
        let manager = super::snapshot::SnapshotManager::new();
        manager.assert_buffer_snapshot(name, self.app.buffer());
        self
    }

    // =========================================================================
    // Query
    // =========================================================================

    /// Find position of text on screen
    pub fn find_text(&self, text: &str) -> Option<(u16, u16)> {
        self.app.find_text(text)
    }

    /// Click on text (finds and clicks)
    pub fn click_text(&mut self, text: &str) -> &mut Self {
        if let Some((x, y)) = self.find_text(text) {
            self.click(x, y);
        } else {
            panic!("Text '{}' not found on screen", text);
        }
        self
    }

    // =========================================================================
    // State
    // =========================================================================

    /// Get action history
    pub fn history(&self) -> &[Action] {
        &self.history
    }

    /// Clear action history
    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    /// Get screen size
    pub fn size(&self) -> (u16, u16) {
        (self.config.width, self.config.height)
    }

    /// Resize screen
    pub fn resize(&mut self, width: u16, height: u16) {
        self.config.width = width;
        self.config.height = height;
        self.app.resize(width, height);
    }

    // =========================================================================
    // Async Support
    // =========================================================================

    /// Wait for a duration (async version)
    pub async fn wait_async(&mut self, duration: Duration) -> &mut Self {
        #[cfg(feature = "async")]
        {
            tokio::time::sleep(duration).await;
        }
        #[cfg(not(feature = "async"))]
        {
            // Fallback to sync sleep if tokio not available
            std::thread::sleep(duration);
        }
        self.history.push(Action::Wait(duration));
        self
    }

    /// Wait for milliseconds (async version)
    pub async fn wait_ms_async(&mut self, ms: u64) -> &mut Self {
        self.wait_async(Duration::from_millis(ms)).await
    }

    /// Wait for a condition to be true (async version)
    pub async fn wait_until_async<F>(&mut self, condition: F) -> &mut Self
    where
        F: Fn(&Buffer) -> bool,
    {
        let start = std::time::Instant::now();
        let timeout = Duration::from_millis(self.config.timeout_ms);

        while !condition(self.app.buffer()) {
            if start.elapsed() >= timeout {
                panic!("wait_until_async timed out after {:?}", timeout);
            }
            #[cfg(feature = "async")]
            {
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
            #[cfg(not(feature = "async"))]
            {
                std::thread::sleep(Duration::from_millis(10));
            }
            self.app.render();
        }

        self
    }

    /// Run an async test with the pilot
    pub fn run_async<F, Fut>(&mut self, f: F)
    where
        F: FnOnce(&mut Self) -> Fut,
        Fut: std::future::Future<Output = ()>,
    {
        #[cfg(feature = "async")]
        {
            let rt = tokio::runtime::Runtime::new().expect("Failed to create async runtime");
            rt.block_on(f(self));
        }
        #[cfg(not(feature = "async"))]
        {
            // Without async feature, just call the future in a blocking way
            // This is a simplified approach - the user should enable async feature
            let _ = f;
            panic!("Async support requires the 'async' feature to be enabled");
        }
    }
}

/// Async test runner for Pilot
///
/// Provides a more ergonomic way to run async tests.
///
/// # Example
///
/// ```rust,ignore
/// use revue::testing::*;
///
/// #[tokio::test]
/// async fn test_async_pilot() {
///     let view = MyView::new();
///     let mut app = TestApp::new(view);
///     let mut pilot = Pilot::new(&mut app);
///
///     pilot.type_text("hello");
///     pilot.wait_ms_async(100).await;
///     pilot.assert_contains("hello");
/// }
/// ```
pub struct AsyncPilot;

impl AsyncPilot {
    /// Create a test runner that will run the test asynchronously
    #[cfg(feature = "async")]
    pub async fn run<V, F, Fut>(view: V, f: F)
    where
        V: crate::widget::View,
        F: FnOnce(Pilot<'_, V>) -> Fut,
        Fut: std::future::Future<Output = ()>,
    {
        let mut app = TestApp::new(view);
        let pilot = Pilot::new(&mut app);
        f(pilot).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
}
