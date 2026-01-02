//! Test app wrapper for testing views without a terminal

use super::TestConfig;
use crate::event::{KeyEvent, MouseButton, MouseEvent, MouseEventKind};
use crate::layout::Rect;
use crate::render::Buffer;
use crate::widget::{RenderContext, View};

/// Type alias for key event handler
type KeyHandler<V> = Box<dyn FnMut(&KeyEvent, &mut V) -> bool>;

/// Type alias for mouse event handler
type MouseHandler<V> = Box<dyn FnMut(&MouseEvent, &mut V) -> bool>;

/// Type alias for scroll event handler
type ScrollHandler<V> = Box<dyn FnMut(u16, u16, i16, &mut V) -> bool>;

/// A test application that can run views without a real terminal
pub struct TestApp<V: View> {
    /// The view being tested
    view: V,
    /// Render buffer
    buffer: Buffer,
    /// Screen dimensions
    width: u16,
    height: u16,
    /// Key event handler
    key_handler: Option<KeyHandler<V>>,
    /// Mouse event handler
    mouse_handler: Option<MouseHandler<V>>,
    /// Scroll event handler
    scroll_handler: Option<ScrollHandler<V>>,
    /// Whether app is running
    running: bool,
}

impl<V: View> TestApp<V> {
    /// Create a new test app with default size (80x24)
    pub fn new(view: V) -> Self {
        Self::with_size(view, 80, 24)
    }

    /// Create with custom size
    pub fn with_size(view: V, width: u16, height: u16) -> Self {
        let mut app = Self {
            view,
            buffer: Buffer::new(width, height),
            width,
            height,
            key_handler: None,
            mouse_handler: None,
            scroll_handler: None,
            running: true,
        };
        app.render();
        app
    }

    /// Create with config
    pub fn with_config(view: V, config: TestConfig) -> Self {
        Self::with_size(view, config.width, config.height)
    }

    /// Set key event handler
    pub fn on_key<F>(mut self, handler: F) -> Self
    where
        F: FnMut(&KeyEvent, &mut V) -> bool + 'static,
    {
        self.key_handler = Some(Box::new(handler));
        self
    }

    /// Set mouse event handler
    ///
    /// The handler receives the mouse event and a mutable reference to the view.
    /// Return true to trigger a re-render after the event is processed.
    pub fn on_mouse<F>(mut self, handler: F) -> Self
    where
        F: FnMut(&MouseEvent, &mut V) -> bool + 'static,
    {
        self.mouse_handler = Some(Box::new(handler));
        self
    }

    /// Set scroll event handler
    ///
    /// The handler receives (x, y, delta, view) where delta is positive for scroll up
    /// and negative for scroll down. Return true to trigger a re-render.
    pub fn on_scroll<F>(mut self, handler: F) -> Self
    where
        F: FnMut(u16, u16, i16, &mut V) -> bool + 'static,
    {
        self.scroll_handler = Some(Box::new(handler));
        self
    }

    /// Get reference to the view
    pub fn view(&self) -> &V {
        &self.view
    }

    /// Get mutable reference to the view
    pub fn view_mut(&mut self) -> &mut V {
        &mut self.view
    }

    /// Get the buffer
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    /// Render the view to buffer
    pub fn render(&mut self) {
        self.buffer.clear();
        let area = Rect::new(0, 0, self.width, self.height);
        let mut ctx = RenderContext::new(&mut self.buffer, area);
        self.view.render(&mut ctx);
    }

    /// Send a key event
    pub fn send_key(&mut self, event: KeyEvent) {
        if let Some(ref mut handler) = self.key_handler {
            let should_render = handler(&event, &mut self.view);
            if should_render {
                self.render();
            }
        } else {
            // Default: always re-render
            self.render();
        }
    }

    /// Send a click event at the specified position
    pub fn send_click(&mut self, x: u16, y: u16) {
        let event = MouseEvent::new(x, y, MouseEventKind::Down(MouseButton::Left));
        self.send_mouse(event);
    }

    /// Send a mouse event
    pub fn send_mouse(&mut self, event: MouseEvent) {
        if let Some(ref mut handler) = self.mouse_handler {
            let should_render = handler(&event, &mut self.view);
            if should_render {
                self.render();
            }
        } else {
            // Default: always re-render
            self.render();
        }
    }

    /// Send a scroll event
    ///
    /// `delta` is positive for scroll up, negative for scroll down.
    pub fn send_scroll(&mut self, x: u16, y: u16, delta: i16) {
        if let Some(ref mut handler) = self.scroll_handler {
            let should_render = handler(x, y, delta, &mut self.view);
            if should_render {
                self.render();
            }
        } else {
            // Default: always re-render
            self.render();
        }
    }

    /// Send scroll up event
    pub fn scroll_up(&mut self, x: u16, y: u16) {
        let event = MouseEvent::new(x, y, MouseEventKind::ScrollUp);
        self.send_mouse(event);
    }

    /// Send scroll down event
    pub fn scroll_down(&mut self, x: u16, y: u16) {
        let event = MouseEvent::new(x, y, MouseEventKind::ScrollDown);
        self.send_mouse(event);
    }

    /// Resize the screen
    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        self.buffer.resize(width, height);
        self.render();
    }

    /// Get screen as text
    pub fn screen_text(&self) -> String {
        let mut lines = Vec::new();
        for y in 0..self.height {
            let mut line = String::new();
            for x in 0..self.width {
                if let Some(cell) = self.buffer.get(x, y) {
                    line.push(cell.symbol);
                } else {
                    line.push(' ');
                }
            }
            // Trim trailing spaces
            let trimmed = line.trim_end();
            lines.push(trimmed.to_string());
        }

        // Remove trailing empty lines
        while lines.last().map(|l| l.is_empty()).unwrap_or(false) {
            lines.pop();
        }

        lines.join("\n")
    }

    /// Get a specific line
    pub fn get_line(&self, row: u16) -> String {
        if row >= self.height {
            return String::new();
        }

        let mut line = String::new();
        for x in 0..self.width {
            if let Some(cell) = self.buffer.get(x, row) {
                line.push(cell.symbol);
            } else {
                line.push(' ');
            }
        }
        line.trim_end().to_string()
    }

    /// Get cell at position
    pub fn get_cell(&self, x: u16, y: u16) -> Option<char> {
        self.buffer.get(x, y).map(|c| c.symbol)
    }

    /// Find text on screen, returns position of first occurrence
    pub fn find_text(&self, text: &str) -> Option<(u16, u16)> {
        for y in 0..self.height {
            let line = self.get_line(y);
            if let Some(x) = line.find(text) {
                return Some((x as u16, y));
            }
        }
        None
    }

    /// Check if screen contains text
    pub fn contains(&self, text: &str) -> bool {
        self.screen_text().contains(text)
    }

    /// Check if running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Stop the app
    pub fn quit(&mut self) {
        self.running = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::widget::Text;

    struct HelloView;

    impl View for HelloView {
        fn render(&self, ctx: &mut RenderContext) {
            Text::new("Hello, Test!").render(ctx);
        }
    }

    #[test]
    fn test_app_new() {
        let app = TestApp::new(HelloView);
        assert!(app.contains("Hello"));
        assert!(app.contains("Test"));
    }

    #[test]
    fn test_app_screen_text() {
        let app = TestApp::new(HelloView);
        let text = app.screen_text();
        assert!(text.starts_with("Hello, Test!"));
    }

    #[test]
    fn test_app_find_text() {
        let app = TestApp::new(HelloView);
        let pos = app.find_text("Test");
        assert!(pos.is_some());
        let (x, y) = pos.unwrap();
        assert_eq!(y, 0);
        assert!(x > 0);
    }

    #[test]
    fn test_app_get_line() {
        let app = TestApp::new(HelloView);
        let line = app.get_line(0);
        assert!(line.contains("Hello"));
    }

    #[test]
    fn test_app_resize() {
        let mut app = TestApp::with_size(HelloView, 40, 10);
        assert_eq!(app.width, 40);
        assert_eq!(app.height, 10);

        app.resize(100, 50);
        assert_eq!(app.width, 100);
        assert_eq!(app.height, 50);
    }
}
