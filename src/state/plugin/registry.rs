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

    // =========================================================================
    // PluginRegistry constructor tests
    // =========================================================================

    #[test]
    fn test_registry_new() {
        let registry = PluginRegistry::new();
        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
    }

    #[test]
    fn test_registry_default() {
        let registry = PluginRegistry::default();
        assert!(registry.is_empty());
    }

    // =========================================================================
    // PluginRegistry registration tests
    // =========================================================================

    #[test]
    fn test_registry_is_empty() {
        let mut registry = PluginRegistry::new();
        assert!(registry.is_empty());

        registry.register(CounterPlugin::new("test", 0));
        assert!(!registry.is_empty());
    }

    #[test]
    fn test_registry_len() {
        let mut registry = PluginRegistry::new();
        assert_eq!(registry.len(), 0);

        registry.register(CounterPlugin::new("one", 0));
        assert_eq!(registry.len(), 1);

        registry.register(CounterPlugin::new("two", 0));
        assert_eq!(registry.len(), 2);
    }

    #[test]
    fn test_registry_plugin_names() {
        let mut registry = PluginRegistry::new();
        registry.register(CounterPlugin::new("alpha", 0));
        registry.register(CounterPlugin::new("beta", 0));

        let names = registry.plugin_names();
        assert_eq!(names.len(), 2);
        assert!(names.contains(&"alpha"));
        assert!(names.contains(&"beta"));
    }

    #[test]
    fn test_registry_has_plugin() {
        let mut registry = PluginRegistry::new();
        registry.register(CounterPlugin::new("test", 0));

        assert!(registry.has_plugin("test"));
        assert!(!registry.has_plugin("nonexistent"));
    }

    #[test]
    fn test_registry_priority_descending() {
        let mut registry = PluginRegistry::new();
        registry.register(CounterPlugin::new("low", -100));
        registry.register(CounterPlugin::new("high", 100));
        registry.register(CounterPlugin::new("medium", 0));

        let names = registry.plugin_names();
        assert_eq!(names[0], "high");
        assert_eq!(names[1], "medium");
        assert_eq!(names[2], "low");
    }

    #[test]
    fn test_registry_same_priority() {
        let mut registry = PluginRegistry::new();
        registry.register(CounterPlugin::new("first", 0));
        registry.register(CounterPlugin::new("second", 0));
        registry.register(CounterPlugin::new("third", 0));

        // All same priority, order depends on stable sort
        assert_eq!(registry.len(), 3);
    }

    // =========================================================================
    // PluginRegistry context tests
    // =========================================================================

    #[test]
    fn test_registry_context() {
        let registry = PluginRegistry::new();
        let context = registry.context();

        // Context should exist and be accessible
        assert!(!context.is_running());
    }

    #[test]
    fn test_registry_context_mut() {
        let mut registry = PluginRegistry::new();
        let context = registry.context_mut();

        context.set_terminal_size(100, 50);
        // Just verify we can mutate
    }

    // =========================================================================
    // PluginRegistry lifecycle tests
    // =========================================================================

    #[test]
    fn test_registry_init_once() {
        let mut registry = PluginRegistry::new();
        registry.register(CounterPlugin::new("test", 0));

        // First init should succeed
        assert!(registry.init().is_ok());
        // Second init should be no-op (return Ok)
        assert!(registry.init().is_ok());
    }

    #[test]
    fn test_registry_mount_once() {
        let mut registry = PluginRegistry::new();
        registry.register(CounterPlugin::new("test", 0));
        registry.init().unwrap();

        assert!(registry.mount().is_ok());
        assert!(registry.mount().is_ok()); // Idempotent
    }

    #[test]
    fn test_registry_unmount_not_mounted() {
        let mut registry = PluginRegistry::new();
        registry.register(CounterPlugin::new("test", 0));

        // Unmounting without mounting should be fine
        assert!(registry.unmount().is_ok());
    }

    #[test]
    fn test_registry_full_lifecycle() {
        let mut registry = PluginRegistry::new();
        registry.register(CounterPlugin::new("first", 10));
        registry.register(CounterPlugin::new("second", 5));

        registry.init().unwrap();
        registry.mount().unwrap();
        registry.tick(Duration::from_millis(16)).unwrap();
        registry.tick(Duration::from_millis(16)).unwrap();
        registry.unmount().unwrap();

        // No panic means success
    }

    #[test]
    fn test_registry_update_terminal_size() {
        let mut registry = PluginRegistry::new();
        registry.update_terminal_size(120, 40);

        let context = registry.context();
        let (w, h) = context.terminal_size();
        assert_eq!(w, 120);
        assert_eq!(h, 40);
    }

    // =========================================================================
    // PluginRegistry styles tests
    // =========================================================================

    #[test]
    fn test_collect_styles_empty() {
        let registry = PluginRegistry::new();
        let styles = registry.collect_styles();
        assert!(styles.is_empty());
    }

    #[test]
    fn test_collect_styles_no_styles() {
        let mut registry = PluginRegistry::new();
        registry.register(CounterPlugin::new("test", 0));

        let styles = registry.collect_styles();
        assert!(styles.is_empty());
    }

    struct MultiStylePlugin {
        name: &'static str,
        styles: &'static str,
    }

    impl Plugin for MultiStylePlugin {
        fn name(&self) -> &str {
            self.name
        }

        fn styles(&self) -> Option<&str> {
            Some(self.styles)
        }
    }

    #[test]
    fn test_collect_styles_multiple() {
        let mut registry = PluginRegistry::new();
        registry.register(MultiStylePlugin {
            name: "style1",
            styles: ".widget1 { color: red; }",
        });
        registry.register(MultiStylePlugin {
            name: "style2",
            styles: ".widget2 { color: blue; }",
        });

        let styles = registry.collect_styles();
        assert!(styles.contains(".widget1"));
        assert!(styles.contains(".widget2"));
        // Styles should be joined with double newlines
        assert!(styles.contains("\n\n"));
    }

    // =========================================================================
    // Error handling tests
    // =========================================================================

    struct FailingPlugin {
        fail_on: &'static str,
    }

    impl Plugin for FailingPlugin {
        fn name(&self) -> &str {
            "failing"
        }

        fn on_init(&mut self, _ctx: &mut PluginContext) -> crate::Result<()> {
            if self.fail_on == "init" {
                return Err(crate::Error::Other(anyhow::anyhow!("init failed")));
            }
            Ok(())
        }

        fn on_mount(&mut self, _ctx: &mut PluginContext) -> crate::Result<()> {
            if self.fail_on == "mount" {
                return Err(crate::Error::Other(anyhow::anyhow!("mount failed")));
            }
            Ok(())
        }

        fn on_tick(&mut self, _ctx: &mut PluginContext, _delta: Duration) -> crate::Result<()> {
            if self.fail_on == "tick" {
                return Err(crate::Error::Other(anyhow::anyhow!("tick failed")));
            }
            Ok(())
        }

        fn on_unmount(&mut self, _ctx: &mut PluginContext) -> crate::Result<()> {
            if self.fail_on == "unmount" {
                return Err(crate::Error::Other(anyhow::anyhow!("unmount failed")));
            }
            Ok(())
        }
    }

    #[test]
    fn test_init_error_propagates() {
        let mut registry = PluginRegistry::new();
        registry.register(FailingPlugin { fail_on: "init" });

        let result = registry.init();
        assert!(result.is_err());
    }

    #[test]
    fn test_mount_error_propagates() {
        let mut registry = PluginRegistry::new();
        registry.register(FailingPlugin { fail_on: "mount" });
        registry.init().unwrap();

        let result = registry.mount();
        assert!(result.is_err());
    }

    #[test]
    fn test_tick_error_continues() {
        let mut registry = PluginRegistry::new();
        registry.register(FailingPlugin { fail_on: "tick" });
        registry.register(CounterPlugin::new("other", -10));
        registry.init().unwrap();
        registry.mount().unwrap();

        // Tick should continue even if one plugin fails
        let result = registry.tick(Duration::from_millis(16));
        assert!(result.is_ok());
    }

    #[test]
    fn test_unmount_error_continues() {
        let mut registry = PluginRegistry::new();
        registry.register(FailingPlugin { fail_on: "unmount" });
        registry.register(CounterPlugin::new("other", -10));
        registry.init().unwrap();
        registry.mount().unwrap();

        // Unmount should continue even if one plugin fails
        let result = registry.unmount();
        assert!(result.is_ok());
    }
}
