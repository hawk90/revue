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

    // =========================================================================
    // Additional ScreenId and SimpleScreen tests
    // =========================================================================

    #[test]
    fn test_screen_id_new_with_string() {
        let id = ScreenId::new(String::from("test_id"));
        assert_eq!(id.as_str(), "test_id");
    }

    #[test]
    fn test_screen_id_from_str() {
        let id = ScreenId::from("from_str");
        assert_eq!(id.as_str(), "from_str");
    }

    #[test]
    fn test_screen_id_from_string() {
        let id = ScreenId::from(String::from("from_string"));
        assert_eq!(id.as_str(), "from_string");
    }

    #[test]
    fn test_screen_id_clone() {
        let id1 = ScreenId::new("clone_test");
        let id2 = id1.clone();
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_screen_id_equality() {
        let id1 = ScreenId::new("same");
        let id2 = ScreenId::new("same");
        let id3 = ScreenId::new("different");
        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_screen_id_display() {
        let id = ScreenId::new("display_test");
        assert_eq!(format!("{}", id), "display_test");
    }

    #[test]
    fn test_screen_id_debug() {
        let id = ScreenId::new("debug_test");
        let debug_str = format!("{:?}", id);
        assert!(debug_str.contains("debug_test"));
    }

    #[test]
    fn test_simple_screen_with_string_id() {
        let screen = SimpleScreen::new(String::from("string_id"), |_ctx| {});
        assert_eq!(screen.id().as_str(), "string_id");
    }

    #[test]
    fn test_simple_screen_config_builder() {
        let config = ScreenConfig::default();
        let screen = SimpleScreen::new("test", |_ctx| {}).config(config);
        let _ = screen;
    }

    #[test]
    fn test_simple_screen_clone_id() {
        let screen = SimpleScreen::new("test", |_ctx| {});
        let id = screen.id();
        let id_clone = id.clone();
        assert_eq!(id, id_clone);
    }

    #[test]
    fn test_simple_screen_multiple_configs() {
        let config1 = ScreenConfig::modal();
        let config2 = ScreenConfig::popup();
        let screen1 = SimpleScreen::new("test1", |_ctx| {}).config(config1);
        let screen2 = SimpleScreen::new("test2", |_ctx| {}).config(config2);
        assert_eq!(screen1.id().as_str(), "test1");
        assert_eq!(screen2.id().as_str(), "test2");
    }

    #[test]
    fn test_simple_screen_function_chain() {
        let screen = simple_screen("chain_test", |_ctx| {}).config(ScreenConfig::default());
        assert_eq!(screen.id().as_str(), "chain_test");
    }

    #[test]
    fn test_simple_screen_empty_id() {
        let screen = SimpleScreen::new("", |_ctx| {});
        assert_eq!(screen.id().as_str(), "");
    }

    #[test]
    fn test_screen_id_with_special_chars() {
        let id = ScreenId::new("test-screen_123");
        assert_eq!(id.as_str(), "test-screen_123");
    }

    #[test]
    fn test_screen_id_with_unicode() {
        let id = ScreenId::new("화면-id");
        assert_eq!(id.as_str(), "화면-id");
    }
}
