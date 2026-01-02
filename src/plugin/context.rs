//! Plugin context for accessing app state and utilities

use std::any::Any;
use std::collections::HashMap;

/// Context provided to plugins during lifecycle hooks
///
/// Provides access to shared state, configuration, and utilities
/// that plugins can use to interact with the application.
///
/// # Example
///
/// ```rust,ignore
/// fn on_init(&mut self, ctx: &mut PluginContext) -> revue::Result<()> {
///     // Store plugin-specific data
///     ctx.set_data("counter", 0i32);
///
///     // Read configuration
///     if let Some(config) = ctx.get_config::<MyConfig>("my-plugin") {
///         // Use config...
///     }
///
///     Ok(())
/// }
/// ```
pub struct PluginContext {
    /// Shared data store (plugin name -> key -> value)
    data: HashMap<String, HashMap<String, Box<dyn Any + Send>>>,
    /// Current plugin name (set during hook calls)
    current_plugin: Option<String>,
    /// Terminal size
    terminal_size: (u16, u16),
    /// Whether the app is running
    is_running: bool,
}

impl PluginContext {
    /// Create a new plugin context
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            current_plugin: None,
            terminal_size: (80, 24),
            is_running: false,
        }
    }

    /// Set the current plugin name (called by registry)
    pub(crate) fn set_current_plugin(&mut self, name: &str) {
        self.current_plugin = Some(name.to_string());
    }

    /// Clear the current plugin name
    pub(crate) fn clear_current_plugin(&mut self) {
        self.current_plugin = None;
    }

    /// Update terminal size
    pub(crate) fn set_terminal_size(&mut self, width: u16, height: u16) {
        self.terminal_size = (width, height);
    }

    /// Set running state
    pub(crate) fn set_running(&mut self, running: bool) {
        self.is_running = running;
    }

    // =========================================================================
    // Public API
    // =========================================================================

    /// Get terminal size (width, height)
    pub fn terminal_size(&self) -> (u16, u16) {
        self.terminal_size
    }

    /// Check if app is currently running
    pub fn is_running(&self) -> bool {
        self.is_running
    }

    /// Store data for the current plugin
    ///
    /// Data is namespaced by plugin name to avoid conflicts.
    pub fn set_data<T: Any + Send>(&mut self, key: &str, value: T) {
        let plugin_name = self.current_plugin.clone().unwrap_or_default();
        let plugin_data = self.data.entry(plugin_name).or_default();
        plugin_data.insert(key.to_string(), Box::new(value));
    }

    /// Get data for the current plugin
    pub fn get_data<T: Any + Send>(&self, key: &str) -> Option<&T> {
        let plugin_name = self.current_plugin.as_ref()?;
        self.data
            .get(plugin_name)?
            .get(key)?
            .downcast_ref::<T>()
    }

    /// Get mutable data for the current plugin
    pub fn get_data_mut<T: Any + Send>(&mut self, key: &str) -> Option<&mut T> {
        let plugin_name = self.current_plugin.clone()?;
        self.data
            .get_mut(&plugin_name)?
            .get_mut(key)?
            .downcast_mut::<T>()
    }

    /// Remove data for the current plugin
    pub fn remove_data(&mut self, key: &str) -> bool {
        if let Some(plugin_name) = &self.current_plugin {
            if let Some(plugin_data) = self.data.get_mut(plugin_name) {
                return plugin_data.remove(key).is_some();
            }
        }
        false
    }

    /// Get data from another plugin (read-only)
    pub fn get_plugin_data<T: Any + Send>(&self, plugin_name: &str, key: &str) -> Option<&T> {
        self.data
            .get(plugin_name)?
            .get(key)?
            .downcast_ref::<T>()
    }

    /// Log a message (for debugging)
    pub fn log(&self, message: &str) {
        if let Some(plugin_name) = &self.current_plugin {
            tracing::debug!("[{}] {}", plugin_name, message);
        } else {
            tracing::debug!("{}", message);
        }
    }

    /// Log a warning
    pub fn warn(&self, message: &str) {
        if let Some(plugin_name) = &self.current_plugin {
            tracing::warn!("[{}] {}", plugin_name, message);
        } else {
            tracing::warn!("{}", message);
        }
    }

    /// Log an error
    pub fn error(&self, message: &str) {
        if let Some(plugin_name) = &self.current_plugin {
            tracing::error!("[{}] {}", plugin_name, message);
        } else {
            tracing::error!("{}", message);
        }
    }
}

impl Default for PluginContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_context_data() {
        let mut ctx = PluginContext::new();
        ctx.set_current_plugin("test-plugin");

        ctx.set_data("counter", 42i32);
        assert_eq!(ctx.get_data::<i32>("counter"), Some(&42));

        if let Some(counter) = ctx.get_data_mut::<i32>("counter") {
            *counter += 1;
        }
        assert_eq!(ctx.get_data::<i32>("counter"), Some(&43));
    }

    #[test]
    fn test_plugin_context_namespacing() {
        let mut ctx = PluginContext::new();

        // Plugin A sets data
        ctx.set_current_plugin("plugin-a");
        ctx.set_data("value", "from-a".to_string());

        // Plugin B sets data with same key
        ctx.set_current_plugin("plugin-b");
        ctx.set_data("value", "from-b".to_string());

        // Each plugin sees its own data
        ctx.set_current_plugin("plugin-a");
        assert_eq!(ctx.get_data::<String>("value"), Some(&"from-a".to_string()));

        ctx.set_current_plugin("plugin-b");
        assert_eq!(ctx.get_data::<String>("value"), Some(&"from-b".to_string()));

        // Cross-plugin access
        assert_eq!(
            ctx.get_plugin_data::<String>("plugin-a", "value"),
            Some(&"from-a".to_string())
        );
    }

    #[test]
    fn test_terminal_size() {
        let mut ctx = PluginContext::new();
        assert_eq!(ctx.terminal_size(), (80, 24));

        ctx.set_terminal_size(120, 40);
        assert_eq!(ctx.terminal_size(), (120, 40));
    }
}
