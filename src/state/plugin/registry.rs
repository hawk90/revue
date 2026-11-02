//! Plugin registry for managing multiple plugins

use super::{Plugin, PluginContext};
use std::time::Duration;

/// A registered plugin and how far through its lifecycle it is.
///
/// Kept per plugin because the list re-sorts on every `register`: a count of
/// "the first n are done" would point at other plugins afterwards (#877).
struct Entry {
    plugin: Box<dyn Plugin>,
    /// A panic has disabled it
    disabled: bool,
    /// `on_init` succeeded
    initialized: bool,
    /// `on_mount` succeeded and `on_unmount` has not run since
    mounted: bool,
}

/// Registry for managing plugins
///
/// Handles plugin lifecycle, ordering by priority, and collecting styles.
///
/// A plugin is mounted only after its `on_init` succeeded, and only mounted
/// plugins get `on_tick` and `on_unmount`.
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
    /// [`init`](Self::init) has gone through every plugin
    initialized: bool,
    /// [`mount`](Self::mount) has gone through every plugin
    mounted: bool,
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
            mounted: false,
        }
    }

    /// Register a plugin
    ///
    /// Plugins are sorted by priority (higher priority runs first).
    ///
    /// A plugin registered after [`init`](Self::init) or [`mount`](Self::mount)
    /// has finished catches up here: it gets the hooks it missed, so it ticks
    /// like the others. A hook that fails is logged to the context as usual and
    /// the next `init` or `mount` tries it again; until then the plugin does
    /// not tick. Its [`styles`](Plugin::styles) are not applied, since
    /// the stylesheet was built with the app.
    pub fn register<P: Plugin + 'static>(&mut self, plugin: P) {
        let mut entry = Entry {
            plugin: Box::new(plugin),
            disabled: false,
            initialized: false,
            mounted: false,
        };
        crate::log_debug!(
            "Registered plugin '{}' with priority {}",
            entry.plugin.name(),
            entry.plugin.priority()
        );

        // Errors and panics are already in the context's log, and there is no
        // caller to return them to: `App::plugins_mut` hands out a plain
        // registry to code that is not expecting a `Result` from this.
        if self.mounted {
            let _ = Self::mount_entry(&mut entry, &mut self.context);
        } else if self.initialized {
            let _ = Self::init_entry(&mut entry, &mut self.context);
        }
        self.plugins.push(entry);

        // Sort by priority (descending)
        self.plugins
            .sort_by_key(|e| std::cmp::Reverse(e.plugin.priority()));
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

    /// Run `on_init` unless the plugin is done with it or disabled.
    fn init_entry(entry: &mut Entry, context: &mut PluginContext) -> Hook {
        if entry.initialized || entry.disabled {
            return Hook::Ok;
        }
        let hook = Self::run_hook(entry, context, "Init", |p, ctx| p.on_init(ctx));
        entry.initialized = matches!(hook, Hook::Ok);
        hook
    }

    /// Take the plugin as far as mounted, `on_init` first if it is still owed.
    fn mount_entry(entry: &mut Entry, context: &mut PluginContext) -> Hook {
        let init = Self::init_entry(entry, context);
        if !matches!(init, Hook::Ok) {
            return init;
        }
        if entry.mounted || entry.disabled {
            return Hook::Ok;
        }
        let hook = Self::run_hook(entry, context, "Mount", |p, ctx| p.on_mount(ctx));
        entry.mounted = matches!(hook, Hook::Ok);
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
        let mut panicked = None;
        for entry in &mut self.plugins {
            match Self::init_entry(entry, &mut self.context) {
                Hook::Ok => {}
                Hook::Failed(e) => return Err(e),
                Hook::Panicked(e) => {
                    panicked.get_or_insert(e);
                }
            }
        }

        self.initialized = true;
        panicked.map_or(Ok(()), Err)
    }

    /// Mount all plugins
    ///
    /// Called when the app starts running.
    ///
    /// A plugin whose `on_init` has not succeeded gets it first, and is not
    /// mounted unless it does.
    ///
    /// Stops at the first plugin that fails and returns its error. The plugins
    /// mounted before it stay mounted: [`unmount`](Self::unmount) unmounts them,
    /// and calling `mount` again goes on from the one that failed. Until then it
    /// and the plugins after it do not tick. A plugin whose hook panics is
    /// disabled instead, and the others are still mounted; the panic is returned
    /// as the error.
    pub fn mount(&mut self) -> crate::Result<()> {
        self.context.set_running(true);

        let mut panicked = None;
        for entry in &mut self.plugins {
            match Self::mount_entry(entry, &mut self.context) {
                Hook::Ok => {}
                Hook::Failed(e) => return Err(e),
                Hook::Panicked(e) => {
                    panicked.get_or_insert(e);
                }
            }
        }

        self.initialized = true;
        self.mounted = true;
        panicked.map_or(Ok(()), Err)
    }

    /// Tick all plugins
    ///
    /// Called on each frame update, for the mounted plugins only. A failing
    /// tick does not stop the plugins after it. A panicking one disables its
    /// plugin and is returned as the error once every plugin has ticked.
    pub fn tick(&mut self, delta: Duration) -> crate::Result<()> {
        let mut panicked = None;
        for entry in self.plugins.iter_mut().filter(|e| e.mounted && !e.disabled) {
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
        if !self.mounted && !self.plugins.iter().any(|e| e.mounted) {
            return Ok(());
        }

        self.context.set_running(false);

        // Unmount in reverse order - only the plugins that were mounted
        let mut panicked = None;
        for entry in self.plugins.iter_mut().rev() {
            if !std::mem::take(&mut entry.mounted) || entry.disabled {
                continue;
            }
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

    /// Records every hook, tick included; fails `fail_in` if it names one.
    struct Ticker {
        name: &'static str,
        priority: i32,
        fail_in: &'static str,
        log: Log,
    }

    type Log = std::sync::Arc<std::sync::Mutex<Vec<String>>>;

    impl Ticker {
        fn hook(&self, hook: &str) -> crate::Result<()> {
            self.log
                .lock()
                .unwrap()
                .push(format!("{}:{hook}", self.name));
            match hook == self.fail_in {
                true => Err(crate::Error::Render(format!("{hook} failed"))),
                false => Ok(()),
            }
        }
    }

    impl Plugin for Ticker {
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
        fn on_tick(&mut self, _: &mut PluginContext, _: Duration) -> crate::Result<()> {
            self.hook("tick")
        }
        fn on_unmount(&mut self, _: &mut PluginContext) -> crate::Result<()> {
            self.hook("unmount")
        }
    }

    struct Rig {
        registry: PluginRegistry,
        log: Log,
    }

    impl Rig {
        fn new() -> Self {
            Self {
                registry: PluginRegistry::new(),
                log: Log::default(),
            }
        }

        fn add(&mut self, name: &'static str, priority: i32, fail_in: &'static str) {
            self.registry.register(Ticker {
                name,
                priority,
                fail_in,
                log: self.log.clone(),
            });
        }

        fn tick(&mut self) {
            self.registry.tick(Duration::from_millis(16)).unwrap();
        }

        /// Everything logged since the last call.
        fn drain(&self) -> Vec<String> {
            std::mem::take(&mut *self.log.lock().unwrap())
        }
    }

    #[test]
    fn nothing_ticks_before_mount_even_after_init() {
        let mut rig = Rig::new();
        rig.add("a", 0, "");
        rig.tick();
        assert_eq!(rig.drain(), Vec::<String>::new());

        rig.registry.init().unwrap();
        rig.drain();
        rig.tick();
        assert_eq!(rig.drain(), Vec::<String>::new());
    }

    #[test]
    fn a_failed_mount_ticks_only_the_plugins_before_it() {
        let mut rig = Rig::new();
        rig.add("first", 10, "");
        rig.add("bad", 5, "mount");
        rig.add("after", 0, "");
        rig.registry.init().unwrap();
        assert!(rig.registry.mount().is_err());
        rig.drain();

        rig.tick();
        assert_eq!(rig.drain(), vec!["first:tick"]);
    }

    #[test]
    fn a_retried_mount_goes_on_from_the_one_that_failed() {
        let mut rig = Rig::new();
        rig.add("first", 10, "");
        rig.add("bad", 5, "mount");
        rig.registry.init().unwrap();
        assert!(rig.registry.mount().is_err());
        rig.drain();

        assert!(rig.registry.mount().is_err());
        assert_eq!(rig.drain(), vec!["bad:mount"]);
    }

    #[test]
    fn a_failed_init_is_not_mounted_and_does_not_tick() {
        let mut rig = Rig::new();
        rig.add("first", 10, "");
        rig.add("bad", 0, "init");
        assert!(rig.registry.init().is_err());
        rig.drain();

        assert!(rig.registry.mount().is_err());
        assert_eq!(rig.drain(), vec!["first:mount", "bad:init"]);
        rig.tick();
        assert_eq!(rig.drain(), vec!["first:tick"]);
    }

    #[test]
    fn mount_without_init_runs_on_init_first() {
        let mut rig = Rig::new();
        rig.add("a", 0, "");
        rig.registry.mount().unwrap();
        assert_eq!(rig.drain(), vec!["a:init", "a:mount"]);
    }

    #[test]
    fn a_higher_priority_plugin_registered_after_mount_catches_up_and_keeps_the_others_state() {
        let mut rig = Rig::new();
        rig.add("old", 0, "");
        rig.registry.init().unwrap();
        rig.registry.mount().unwrap();
        rig.drain();

        rig.add("new", 10, "");
        assert_eq!(rig.drain(), vec!["new:init", "new:mount"]);

        rig.tick();
        assert_eq!(rig.drain(), vec!["new:tick", "old:tick"]);

        rig.registry.mount().unwrap();
        assert_eq!(rig.drain(), Vec::<String>::new(), "nothing mounts twice");

        rig.registry.unmount().unwrap();
        assert_eq!(rig.drain(), vec!["old:unmount", "new:unmount"]);
    }

    #[test]
    fn a_lower_priority_plugin_registered_after_mount_catches_up_too() {
        let mut rig = Rig::new();
        rig.add("old", 10, "");
        rig.registry.mount().unwrap();
        rig.drain();

        rig.add("new", 0, "");
        assert_eq!(rig.drain(), vec!["new:init", "new:mount"]);
        rig.tick();
        assert_eq!(rig.drain(), vec!["old:tick", "new:tick"]);
    }

    #[test]
    fn a_plugin_registered_between_init_and_mount_is_initialized_and_waits() {
        let mut rig = Rig::new();
        rig.add("old", 0, "");
        rig.registry.init().unwrap();
        rig.drain();

        rig.add("new", 10, "");
        assert_eq!(rig.drain(), vec!["new:init"]);
        rig.tick();
        assert_eq!(rig.drain(), Vec::<String>::new());

        rig.registry.mount().unwrap();
        assert_eq!(rig.drain(), vec!["new:mount", "old:mount"]);
    }

    #[test]
    fn a_plugin_registered_before_init_gets_nothing_yet() {
        let mut rig = Rig::new();
        rig.add("a", 0, "");
        assert_eq!(rig.drain(), Vec::<String>::new());
    }

    #[test]
    fn a_late_plugin_that_fails_does_not_tick_and_is_retried() {
        let mut rig = Rig::new();
        rig.add("old", 10, "");
        rig.registry.mount().unwrap();
        rig.drain();

        rig.add("bad", 0, "mount");
        assert_eq!(rig.drain(), vec!["bad:init", "bad:mount"]);

        rig.tick();
        assert_eq!(rig.drain(), vec!["old:tick"]);

        assert!(rig.registry.mount().is_err());
        assert_eq!(rig.drain(), vec!["bad:mount"]);

        rig.registry.unmount().unwrap();
        assert_eq!(rig.drain(), vec!["old:unmount"]);
    }

    #[test]
    fn unmounted_plugins_stop_ticking_and_mount_again_without_a_second_init() {
        let mut rig = Rig::new();
        rig.add("a", 0, "");
        rig.registry.init().unwrap();
        rig.registry.mount().unwrap();
        rig.registry.unmount().unwrap();
        rig.drain();

        rig.tick();
        assert_eq!(rig.drain(), Vec::<String>::new());

        rig.registry.unmount().unwrap();
        assert_eq!(rig.drain(), Vec::<String>::new(), "nothing unmounts twice");

        rig.registry.mount().unwrap();
        assert_eq!(rig.drain(), vec!["a:mount"]);
    }

    #[test]
    fn a_plugin_registered_after_unmount_waits_for_the_next_mount() {
        let mut rig = Rig::new();
        rig.add("old", 0, "");
        rig.registry.mount().unwrap();
        rig.registry.unmount().unwrap();
        rig.drain();

        rig.add("new", 10, "");
        assert_eq!(rig.drain(), vec!["new:init"]);

        rig.registry.mount().unwrap();
        assert_eq!(rig.drain(), vec!["new:mount", "old:mount"]);
    }
}
