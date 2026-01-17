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
///     // Read data
///     if let Some(counter) = ctx.get_data::<i32>("counter") {
///         // Use counter...
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
        self.data.get(plugin_name)?.get(key)?.downcast_ref::<T>()
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
        self.data.get(plugin_name)?.get(key)?.downcast_ref::<T>()
    }

    /// Log a message (for debugging)
    pub fn log(&self, message: &str) {
        if let Some(plugin_name) = &self.current_plugin {
            crate::log_debug!("[{}] {}", plugin_name, message);
        } else {
            crate::log_debug!("{}", message);
        }
    }

    /// Log a warning
    pub fn warn(&self, message: &str) {
        if let Some(plugin_name) = &self.current_plugin {
            crate::log_warn!("[{}] {}", plugin_name, message);
        } else {
            crate::log_warn!("{}", message);
        }
    }

    /// Log an error
    pub fn error(&self, message: &str) {
        if let Some(plugin_name) = &self.current_plugin {
            crate::log_error!("[{}] {}", plugin_name, message);
        } else {
            crate::log_error!("{}", message);
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

    #[test]
    fn test_plugin_context_new() {
        let ctx = PluginContext::new();
        assert_eq!(ctx.terminal_size(), (80, 24));
        assert!(!ctx.is_running());
    }

    #[test]
    fn test_plugin_context_default() {
        let ctx = PluginContext::default();
        assert_eq!(ctx.terminal_size(), (80, 24));
        assert!(!ctx.is_running());
    }

    #[test]
    fn test_is_running() {
        let mut ctx = PluginContext::new();
        assert!(!ctx.is_running());

        ctx.set_running(true);
        assert!(ctx.is_running());

        ctx.set_running(false);
        assert!(!ctx.is_running());
    }

    #[test]
    fn test_clear_current_plugin() {
        let mut ctx = PluginContext::new();
        ctx.set_current_plugin("test");
        ctx.set_data("key", 42i32);

        // Should have data with plugin set
        assert_eq!(ctx.get_data::<i32>("key"), Some(&42));

        // Clear plugin and try to access data
        ctx.clear_current_plugin();
        // Without current plugin, get_data returns None
        assert_eq!(ctx.get_data::<i32>("key"), None);
    }

    #[test]
    fn test_remove_data() {
        let mut ctx = PluginContext::new();
        ctx.set_current_plugin("test");
        ctx.set_data("key", 42i32);

        assert_eq!(ctx.get_data::<i32>("key"), Some(&42));

        let removed = ctx.remove_data("key");
        assert!(removed);
        assert_eq!(ctx.get_data::<i32>("key"), None);
    }

    #[test]
    fn test_remove_data_nonexistent() {
        let mut ctx = PluginContext::new();
        ctx.set_current_plugin("test");

        let removed = ctx.remove_data("nonexistent");
        assert!(!removed);
    }

    #[test]
    fn test_remove_data_no_plugin() {
        let mut ctx = PluginContext::new();
        // No current plugin set
        let removed = ctx.remove_data("key");
        assert!(!removed);
    }

    #[test]
    fn test_get_plugin_data_nonexistent_plugin() {
        let ctx = PluginContext::new();
        assert_eq!(ctx.get_plugin_data::<i32>("nonexistent", "key"), None);
    }

    #[test]
    fn test_get_plugin_data_nonexistent_key() {
        let mut ctx = PluginContext::new();
        ctx.set_current_plugin("test");
        ctx.set_data("key", 42i32);

        assert_eq!(ctx.get_plugin_data::<i32>("test", "nonexistent"), None);
    }

    #[test]
    fn test_get_data_no_plugin() {
        let ctx = PluginContext::new();
        // No current plugin set
        assert_eq!(ctx.get_data::<i32>("key"), None);
    }

    #[test]
    fn test_get_data_mut_no_plugin() {
        let mut ctx = PluginContext::new();
        // No current plugin set
        assert_eq!(ctx.get_data_mut::<i32>("key"), None);
    }

    #[test]
    fn test_get_data_mut_wrong_type() {
        let mut ctx = PluginContext::new();
        ctx.set_current_plugin("test");
        ctx.set_data("key", 42i32);

        // Try to get as wrong type
        assert_eq!(ctx.get_data_mut::<String>("key"), None);
    }

    #[test]
    fn test_set_data_without_plugin() {
        let mut ctx = PluginContext::new();
        // No current plugin - should use empty string as plugin name
        ctx.set_data("key", 42i32);

        // Set a plugin and try to access
        ctx.set_current_plugin("");
        assert_eq!(ctx.get_data::<i32>("key"), Some(&42));
    }

    #[test]
    fn test_log_with_plugin() {
        let mut ctx = PluginContext::new();
        ctx.set_current_plugin("test");
        // Just verify it doesn't panic
        ctx.log("test message");
    }

    #[test]
    fn test_log_without_plugin() {
        let ctx = PluginContext::new();
        // Just verify it doesn't panic
        ctx.log("test message");
    }

    #[test]
    fn test_warn_with_plugin() {
        let mut ctx = PluginContext::new();
        ctx.set_current_plugin("test");
        // Just verify it doesn't panic
        ctx.warn("warning message");
    }

    #[test]
    fn test_warn_without_plugin() {
        let ctx = PluginContext::new();
        // Just verify it doesn't panic
        ctx.warn("warning message");
    }

    #[test]
    fn test_error_with_plugin() {
        let mut ctx = PluginContext::new();
        ctx.set_current_plugin("test");
        // Just verify it doesn't panic
        ctx.error("error message");
    }

    #[test]
    fn test_error_without_plugin() {
        let ctx = PluginContext::new();
        // Just verify it doesn't panic
        ctx.error("error message");
    }

    #[test]
    fn test_multiple_data_types() {
        let mut ctx = PluginContext::new();
        ctx.set_current_plugin("test");

        ctx.set_data("int", 42i32);
        ctx.set_data("float", 3.14f64);
        ctx.set_data("string", "hello".to_string());
        ctx.set_data("bool", true);

        assert_eq!(ctx.get_data::<i32>("int"), Some(&42));
        assert_eq!(ctx.get_data::<f64>("float"), Some(&3.14));
        assert_eq!(ctx.get_data::<String>("string"), Some(&"hello".to_string()));
        assert_eq!(ctx.get_data::<bool>("bool"), Some(&true));
    }

    #[test]
    fn test_overwrite_data() {
        let mut ctx = PluginContext::new();
        ctx.set_current_plugin("test");

        ctx.set_data("key", 42i32);
        assert_eq!(ctx.get_data::<i32>("key"), Some(&42));

        ctx.set_data("key", 100i32);
        assert_eq!(ctx.get_data::<i32>("key"), Some(&100));
    }
}
