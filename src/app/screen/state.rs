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
