//! Plugin registry for managing multiple plugins

use super::{Plugin, PluginContext};
use std::time::Duration;

/// Registry for managing plugins
///
/// Handles plugin lifecycle, ordering by priority, and collecting styles.
pub struct PluginRegistry {
    /// Registered plugins (sorted by priority, highest first)
    plugins: Vec<Box<dyn Plugin>>,
    /// Shared context
    context: PluginContext,
    /// Whether plugins have been initialized
    initialized: bool,
    /// Whether plugins have been mounted
    mounted: bool,
}

impl PluginRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
            context: PluginContext::new(),
            initialized: false,
            mounted: false,
        }
    }

    /// Register a plugin
    ///
    /// Plugins are sorted by priority (higher priority runs first).
    pub fn register<P: Plugin + 'static>(&mut self, plugin: P) {
        let priority = plugin.priority();
        self.plugins.push(Box::new(plugin));

        // Sort by priority (descending)
        self.plugins
            .sort_by_key(|p| std::cmp::Reverse(p.priority()));

        crate::log_debug!(
            "Registered plugin '{}' with priority {}",
            self.plugins.last().map(|p| p.name()).unwrap_or("unknown"),
            priority
        );
    }

    /// Get number of registered plugins
    pub fn len(&self) -> usize {
        self.plugins.len()
    }

    /// Check if registry is empty
    pub fn is_empty(&self) -> bool {
        self.plugins.is_empty()
    }

    /// Get plugin names
    pub fn plugin_names(&self) -> Vec<&str> {
        self.plugins.iter().map(|p| p.name()).collect()
    }

    /// Check if a plugin is registered
    pub fn has_plugin(&self, name: &str) -> bool {
        self.plugins.iter().any(|p| p.name() == name)
    }

    /// Get access to the plugin context
    pub fn context(&self) -> &PluginContext {
        &self.context
    }

    /// Get mutable access to the plugin context
    pub fn context_mut(&mut self) -> &mut PluginContext {
        &mut self.context
    }

    /// Collect all CSS styles from plugins
    pub fn collect_styles(&self) -> String {
        self.plugins
            .iter()
            .filter_map(|p| p.styles())
            .collect::<Vec<_>>()
            .join("\n\n")
    }

    // =========================================================================
    // Lifecycle methods
    // =========================================================================

    /// Initialize all plugins
    ///
    /// Called once when the app is being built.
    pub fn init(&mut self) -> crate::Result<()> {
        if self.initialized {
            return Ok(());
        }

        for plugin in &mut self.plugins {
            self.context.set_current_plugin(plugin.name());
            if let Err(e) = plugin.on_init(&mut self.context) {
                self.context.error(&format!("Init failed: {}", e));
                return Err(e);
            }
            self.context.clear_current_plugin();
        }

        self.initialized = true;
        Ok(())
    }

    /// Mount all plugins
    ///
    /// Called when the app starts running.
    pub fn mount(&mut self) -> crate::Result<()> {
        if self.mounted {
            return Ok(());
        }

        self.context.set_running(true);

        for plugin in &mut self.plugins {
            self.context.set_current_plugin(plugin.name());
            if let Err(e) = plugin.on_mount(&mut self.context) {
                self.context.error(&format!("Mount failed: {}", e));
                return Err(e);
            }
            self.context.clear_current_plugin();
        }

        self.mounted = true;
        Ok(())
    }

    /// Tick all plugins
    ///
    /// Called on each frame update.
    pub fn tick(&mut self, delta: Duration) -> crate::Result<()> {
        for plugin in &mut self.plugins {
            self.context.set_current_plugin(plugin.name());
            if let Err(e) = plugin.on_tick(&mut self.context, delta) {
                self.context.error(&format!("Tick failed: {}", e));
                // Continue with other plugins even if one fails
            }
            self.context.clear_current_plugin();
        }
        Ok(())
    }

    /// Unmount all plugins
    ///
    /// Called when the app is shutting down.
    /// Plugins are unmounted in reverse order (lowest priority first).
    pub fn unmount(&mut self) -> crate::Result<()> {
        if !self.mounted {
            return Ok(());
        }

        self.context.set_running(false);

        // Unmount in reverse order
        for plugin in self.plugins.iter_mut().rev() {
            self.context.set_current_plugin(plugin.name());
            if let Err(e) = plugin.on_unmount(&mut self.context) {
                self.context.error(&format!("Unmount failed: {}", e));
                // Continue with other plugins even if one fails
            }
            self.context.clear_current_plugin();
        }

        self.mounted = false;
        Ok(())
    }

    /// Update terminal size in context
    pub fn update_terminal_size(&mut self, width: u16, height: u16) {
        self.context.set_terminal_size(width, height);
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct CounterPlugin {
        name: &'static str,
        priority: i32,
        init_count: usize,
        tick_count: usize,
    }

    impl CounterPlugin {
        fn new(name: &'static str, priority: i32) -> Self {
            Self {
                name,
                priority,
                init_count: 0,
                tick_count: 0,
            }
        }
    }

    impl Plugin for CounterPlugin {
        fn name(&self) -> &str {
            self.name
        }

        fn priority(&self) -> i32 {
            self.priority
        }

        fn on_init(&mut self, _ctx: &mut PluginContext) -> crate::Result<()> {
            self.init_count += 1;
            Ok(())
        }

        fn on_tick(&mut self, _ctx: &mut PluginContext, _delta: Duration) -> crate::Result<()> {
            self.tick_count += 1;
            Ok(())
        }
    }

    #[test]
    fn test_registry_register() {
        let mut registry = PluginRegistry::new();
        registry.register(CounterPlugin::new("test", 0));

        assert_eq!(registry.len(), 1);
        assert!(registry.has_plugin("test"));
        assert!(!registry.has_plugin("other"));
    }

    #[test]
    fn test_registry_priority_ordering() {
        let mut registry = PluginRegistry::new();
        registry.register(CounterPlugin::new("low", -10));
        registry.register(CounterPlugin::new("high", 10));
        registry.register(CounterPlugin::new("medium", 0));

        let names = registry.plugin_names();
        assert_eq!(names, vec!["high", "medium", "low"]);
    }

    #[test]
    fn test_registry_lifecycle() {
        let mut registry = PluginRegistry::new();
        registry.register(CounterPlugin::new("test", 0));

        registry.init().unwrap();
        registry.mount().unwrap();
        registry.tick(Duration::from_millis(16)).unwrap();
        registry.tick(Duration::from_millis(16)).unwrap();
        registry.unmount().unwrap();

        // Can't easily check internal state, but this tests the flow doesn't panic
    }

    struct StyledPlugin;

    impl Plugin for StyledPlugin {
        fn name(&self) -> &str {
            "styled"
        }

        fn styles(&self) -> Option<&str> {
            Some(".plugin-widget { color: red; }")
        }
    }

    #[test]
    fn test_collect_styles() {
        let mut registry = PluginRegistry::new();
        registry.register(StyledPlugin);

        let styles = registry.collect_styles();
        assert!(styles.contains(".plugin-widget"));
    }
}
