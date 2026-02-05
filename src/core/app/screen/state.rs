//! Screen state helpers and SimpleScreen implementation

use super::types::{Screen, ScreenConfig, ScreenId};
use crate::widget::RenderContext;

/// Simple screen implementation for closures
pub struct SimpleScreen<F>
where
    F: Fn(&mut RenderContext) + Send + Sync,
{
    id: ScreenId,
    config: ScreenConfig,
    render_fn: F,
}

impl<F> SimpleScreen<F>
where
    F: Fn(&mut RenderContext) + Send + Sync,
{
    /// Create new simple screen
    pub fn new(id: impl Into<ScreenId>, render_fn: F) -> Self {
        Self {
            id: id.into(),
            config: ScreenConfig::default(),
            render_fn,
        }
    }

    /// Set config
    pub fn config(mut self, config: ScreenConfig) -> Self {
        self.config = config;
        self
    }
}

impl<F> Screen for SimpleScreen<F>
where
    F: Fn(&mut RenderContext) + Send + Sync,
{
    fn id(&self) -> ScreenId {
        self.id.clone()
    }

    fn config(&self) -> ScreenConfig {
        self.config.clone()
    }

    fn render(&self, ctx: &mut RenderContext) {
        (self.render_fn)(ctx);
    }
}

/// Create a screen manager
pub fn screen_manager() -> super::core::ScreenManager {
    super::core::ScreenManager::new()
}

/// Create a simple screen
pub fn simple_screen<F>(id: impl Into<ScreenId>, render_fn: F) -> SimpleScreen<F>
where
    F: Fn(&mut RenderContext) + Send + Sync,
{
    SimpleScreen::new(id, render_fn)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::app::screen::types::ScreenMode;

    #[test]
    fn test_simple_screen_new() {
        let screen = SimpleScreen::new("test", |_ctx| {});
        assert_eq!(screen.id().as_str(), "test");
    }

    #[test]
    fn test_simple_screen_with_screen_id() {
        let screen = SimpleScreen::new(ScreenId::new("my_screen"), |_ctx| {});
        assert_eq!(screen.id().as_str(), "my_screen");
    }

    #[test]
    fn test_simple_screen_config() {
        let config = ScreenConfig::modal();
        let screen = SimpleScreen::new("test", |_ctx| {}).config(config);
        // Config builder works - just verify the screen exists
        let _ = screen;
    }

    #[test]
    fn test_simple_screen_id() {
        let screen = SimpleScreen::new("abc123", |_ctx| {});
        assert_eq!(screen.id(), ScreenId::new("abc123"));
    }

    #[test]
    fn test_simple_screen_render() {
        let screen = SimpleScreen::new("test", |_ctx| {});
        // Note: Can't actually call render without a RenderContext
        // This test just verifies the screen can be created
        let _ = screen;
    }

    #[test]
    fn test_screen_manager_function() {
        let manager = screen_manager();
        let _ = manager;
    }

    #[test]
    fn test_simple_screen_function() {
        let screen = simple_screen("test", |_ctx| {});
        assert_eq!(screen.id().as_str(), "test");
    }

    #[test]
    fn test_simple_screen_function_with_screen_id() {
        let screen = simple_screen(ScreenId::new("my_screen"), |_ctx| {});
        assert_eq!(screen.id().as_str(), "my_screen");
    }
}
