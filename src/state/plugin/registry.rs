//! Plugin registry for managing multiple plugins

use super::{Plugin, PluginContext};
use std::time::Duration;

/// A registered plugin, and whether a panic has disabled it.
struct Entry {
    plugin: Box<dyn Plugin>,
    disabled: bool,
}

/// Registry for managing plugins
///
/// Handles plugin lifecycle, ordering by priority, and collecting styles.
///
/// # Panicking plugins
///
/// A hook that panics disables its plugin instead of ending the app. The
/// panic is caught, reported through the context's error log, and returned
/// as an error from that lifecycle call; the plugin gets no more hooks - not
/// even `on_unmount`, since its state is whatever the panic left. The other
/// plugins carry on. [`disabled_plugins`](Self::disabled_plugins) lists the
/// plugins disabled so far.
///
/// This needs unwinding: built with `panic = "abort"`, a panic in a plugin
/// ends the process like any other.
pub struct PluginRegistry {
    /// Registered plugins (sorted by priority, highest first)
    plugins: Vec<Entry>,
    /// Shared context
    context: PluginContext,
    /// Whether plugins have been initialized
    initialized: bool,
    /// How many plugins (in order) `on_init` has succeeded for: a retry after
    /// a failure goes on from the one that failed
    init_done: usize,
    /// Whether plugins have been mounted
    mounted: bool,
    /// How many plugins (in order) are mounted: after a failed `mount` these
    /// are still mounted, and `unmount` must unmount them
    mount_done: usize,
}

/// How one plugin hook went.
enum Hook {
    Ok,
    Failed(crate::Error),
    Panicked(crate::Error),
}

impl PluginRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
            context: PluginContext::new(),
            initialized: false,
            init_done: 0,
            mounted: false,
            mount_done: 0,
        }
    }

    /// Register a plugin
    ///
    /// Plugins are sorted by priority (higher priority runs first).
    pub fn register<P: Plugin + 'static>(&mut self, plugin: P) {
        let priority = plugin.priority();
        self.plugins.push(Entry {
            plugin: Box::new(plugin),
            disabled: false,
        });

        // Sort by priority (descending)
        self.plugins
            .sort_by_key(|e| std::cmp::Reverse(e.plugin.priority()));

        crate::log_debug!(
            "Registered plugin '{}' with priority {}",
            self.plugins
                .last()
                .map(|e| e.plugin.name())
                .unwrap_or("unknown"),
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
        self.plugins.iter().map(|e| e.plugin.name()).collect()
    }

    /// Check if a plugin is registered
    pub fn has_plugin(&self, name: &str) -> bool {
        self.plugins.iter().any(|e| e.plugin.name() == name)
    }

    /// Names of the plugins a panic has disabled, in priority order
    ///
    /// See [Panicking plugins](Self#panicking-plugins).
    pub fn disabled_plugins(&self) -> Vec<&str> {
        self.plugins
            .iter()
            .filter(|e| e.disabled)
            .map(|e| e.plugin.name())
            .collect()
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
            .filter_map(|e| e.plugin.styles())
            .collect::<Vec<_>>()
            .join("\n\n")
    }

    // =========================================================================
    // Lifecycle methods
    // =========================================================================

    /// Run hook `name` of one plugin with the context pointed at it.
    ///
    /// A panic disables the plugin; an error or a panic is logged to the
    /// context as `"<Stage> failed: ..."`, as before.
    fn run_hook(
        entry: &mut Entry,
        context: &mut PluginContext,
        stage: &str,
        hook: impl FnOnce(&mut dyn Plugin, &mut PluginContext) -> crate::Result<()>,
    ) -> Hook {
        context.set_current_plugin(entry.plugin.name());
        let outcome = crate::render::catch_panic(std::panic::AssertUnwindSafe(|| {
            hook(entry.plugin.as_mut(), context)
        }));
        let hook = match outcome {
            Ok(Ok(())) => Hook::Ok,
            Ok(Err(e)) => {
                context.error(&format!("{stage} failed: {e}"));
                Hook::Failed(e)
            }
            Err(payload) => {
                entry.disabled = true;
                let message = format!(
                    "plugin '{}' panicked in {stage}: {}",
                    entry.plugin.name(),
                    crate::tasks::panic_message(&*payload)
                );
                context.error(&format!("{stage} failed: {message}"));
                Hook::Panicked(crate::Error::Other(anyhow::anyhow!(message)))
            }
        };
        context.clear_current_plugin();
        hook
    }

    /// Initialize all plugins
    ///
    /// Called once when the app is being built.
    ///
    /// Stops at the first plugin whose `on_init` fails and returns its error;
    /// calling `init` again goes on from that plugin, so none is initialized
    /// twice. A plugin whose `on_init` panics is disabled instead, and the
    /// others are still initialized; the panic is returned as the error.
    pub fn init(&mut self) -> crate::Result<()> {
        if self.initialized {
            return Ok(());
        }

        let mut panicked = None;
        for entry in self.plugins.iter_mut().skip(self.init_done) {
            if !entry.disabled {
                match Self::run_hook(entry, &mut self.context, "Init", |p, ctx| p.on_init(ctx)) {
                    Hook::Ok => {}
                    Hook::Failed(e) => return Err(e),
                    Hook::Panicked(e) => {
                        panicked.get_or_insert(e);
                    }
                }
            }
            self.init_done += 1;
        }

        self.initialized = true;
        panicked.map_or(Ok(()), Err)
    }

    /// Mount all plugins
    ///
    /// Called when the app starts running.
    ///
    /// Stops at the first plugin whose `on_mount` fails and returns its
    /// error. The plugins mounted before it stay mounted: [`unmount`](Self::unmount)
    /// unmounts them, and calling `mount` again goes on from the one that failed.
    /// A plugin whose `on_mount` panics is disabled instead, and the others are
    /// still mounted; the panic is returned as the error.
    pub fn mount(&mut self) -> crate::Result<()> {
        if self.mounted {
            return Ok(());
        }

        self.context.set_running(true);

        let mut panicked = None;
        for entry in self.plugins.iter_mut().skip(self.mount_done) {
            if !entry.disabled {
                match Self::run_hook(entry, &mut self.context, "Mount", |p, ctx| p.on_mount(ctx)) {
                    Hook::Ok => {}
                    Hook::Failed(e) => return Err(e),
                    Hook::Panicked(e) => {
                        panicked.get_or_insert(e);
                    }
                }
            }
            self.mount_done += 1;
        }

        self.mounted = true;
        panicked.map_or(Ok(()), Err)
    }

    /// Tick all plugins
    ///
    /// Called on each frame update. A failing tick does not stop the plugins
    /// after it. A panicking one disables its plugin and is returned as the
    /// error once every plugin has ticked.
    pub fn tick(&mut self, delta: Duration) -> crate::Result<()> {
        let mut panicked = None;
        for entry in self.plugins.iter_mut().filter(|e| !e.disabled) {
            // Continue with other plugins even if one fails
            if let Hook::Panicked(e) = Self::run_hook(entry, &mut self.context, "Tick", |p, ctx| {
                p.on_tick(ctx, delta)
            }) {
                panicked.get_or_insert(e);
            }
        }
        panicked.map_or(Ok(()), Err)
    }

    /// Unmount all plugins
    ///
    /// Called when the app is shutting down.
    /// Plugins are unmounted in reverse order (lowest priority first). Only
    /// mounted plugins are unmounted - after a failed [`mount`](Self::mount),
    /// those before the one that failed - and not the ones a panic disabled.
    /// A panicking `on_unmount` is returned as the error once every plugin has
    /// been unmounted.
    pub fn unmount(&mut self) -> crate::Result<()> {
        // Not mounted - and no plugins left mounted by a mount that failed
        if !self.mounted && self.mount_done == 0 {
            return Ok(());
        }

        self.context.set_running(false);

        // Unmount in reverse order - only the plugins that were mounted
        let mounted = self.mount_done.min(self.plugins.len());
        let mut panicked = None;
        for entry in self.plugins[..mounted]
            .iter_mut()
            .rev()
            .filter(|e| !e.disabled)
        {
            // Continue with other plugins even if one fails
            if let Hook::Panicked(e) =
                Self::run_hook(entry, &mut self.context, "Unmount", |p, ctx| {
                    p.on_unmount(ctx)
                })
            {
                panicked.get_or_insert(e);
            }
        }

        self.mounted = false;
        self.mount_done = 0;
        panicked.map_or(Ok(()), Err)
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

    /// Records its hooks; fails `fail_in` if it names one.
    struct Recorder {
        name: &'static str,
        priority: i32,
        fail_in: &'static str,
        log: std::sync::Arc<std::sync::Mutex<Vec<String>>>,
    }

    impl Recorder {
        fn hook(&self, hook: &str) -> crate::Result<()> {
            self.log
                .lock()
                .unwrap()
                .push(format!("{}:{hook}", self.name));
            if hook == self.fail_in {
                return Err(crate::Error::Render(format!("{hook} failed")));
            }
            Ok(())
        }
    }

    impl Plugin for Recorder {
        fn name(&self) -> &str {
            self.name
        }
        fn priority(&self) -> i32 {
            self.priority
        }
        fn on_init(&mut self, _: &mut PluginContext) -> crate::Result<()> {
            self.hook("init")
        }
        fn on_mount(&mut self, _: &mut PluginContext) -> crate::Result<()> {
            self.hook("mount")
        }
        fn on_unmount(&mut self, _: &mut PluginContext) -> crate::Result<()> {
            self.hook("unmount")
        }
    }

    fn recorded(
        fail_in: &'static str,
    ) -> (
        PluginRegistry,
        std::sync::Arc<std::sync::Mutex<Vec<String>>>,
    ) {
        let log = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
        let mut registry = PluginRegistry::new();
        for (name, priority, fails) in [("first", 10, ""), ("second", 0, fail_in)] {
            registry.register(Recorder {
                name,
                priority,
                fail_in: fails,
                log: log.clone(),
            });
        }
        (registry, log)
    }

    #[test]
    fn a_failed_mount_leaves_the_mounted_plugins_to_unmount() {
        let (mut registry, log) = recorded("mount");
        registry.init().unwrap();
        assert!(registry.mount().is_err());
        registry.unmount().unwrap();
        let log = log.lock().unwrap().clone();
        assert!(log.contains(&"first:unmount".to_string()), "{log:?}");
        assert!(!log.contains(&"second:unmount".to_string()), "{log:?}");
    }

    #[test]
    fn a_retried_init_does_not_initialize_a_plugin_twice() {
        let (mut registry, log) = recorded("init");
        assert!(registry.init().is_err());
        assert!(registry.init().is_err());
        let log = log.lock().unwrap();
        assert_eq!(log.iter().filter(|e| *e == "first:init").count(), 1);
        assert_eq!(log.iter().filter(|e| *e == "second:init").count(), 2);
    }
}
