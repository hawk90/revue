//! Plugin trait definition

use super::PluginContext;
use std::time::Duration;

/// Plugin trait for extending Revue applications
///
/// Plugins can hook into the application lifecycle to add functionality
/// such as logging, analytics, state persistence, or custom behaviors.
///
/// # Lifecycle
///
/// 1. `on_init` - Called once when the plugin is registered
/// 2. `on_mount` - Called when the app starts running
/// 3. `on_tick` - Called on each tick (frame update)
/// 4. `on_unmount` - Called when the app is shutting down
///
/// # Example
///
/// ```rust,ignore
/// use revue::plugin::{Plugin, PluginContext};
///
/// struct MyPlugin;
///
/// impl Plugin for MyPlugin {
///     fn name(&self) -> &str { "my-plugin" }
///
///     fn on_init(&mut self, ctx: &mut PluginContext) -> revue::Result<()> {
///         ctx.set_data("initialized", true);
///         Ok(())
///     }
/// }
/// ```
pub trait Plugin: Send {
    /// Plugin name (should be unique)
    fn name(&self) -> &str;

    /// Called when the plugin is first registered
    ///
    /// Use this for one-time initialization like loading configuration
    /// or setting up resources.
    fn on_init(&mut self, _ctx: &mut PluginContext) -> crate::Result<()> {
        Ok(())
    }

    /// Called when the app starts running
    ///
    /// Use this to set up state that depends on the app being ready.
    fn on_mount(&mut self, _ctx: &mut PluginContext) -> crate::Result<()> {
        Ok(())
    }

    /// Called on each tick (frame update)
    ///
    /// Use this for periodic tasks like polling, animations, or metrics.
    /// Keep this lightweight as it runs every frame.
    fn on_tick(&mut self, _ctx: &mut PluginContext, _delta: Duration) -> crate::Result<()> {
        Ok(())
    }

    /// Called when the app is shutting down
    ///
    /// Use this for cleanup, saving state, or flushing buffers.
    fn on_unmount(&mut self, _ctx: &mut PluginContext) -> crate::Result<()> {
        Ok(())
    }

    /// Optional CSS styles contributed by this plugin
    ///
    /// Return CSS string that will be merged with the app's stylesheet.
    fn styles(&self) -> Option<&str> {
        None
    }

    /// Plugin priority (higher = runs first)
    ///
    /// Default is 0. Use positive values for early initialization,
    /// negative for late cleanup.
    fn priority(&self) -> i32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    struct TestPlugin {
        init_called: bool,
        mount_called: bool,
        tick_count: usize,
    }

    impl Plugin for TestPlugin {
        fn name(&self) -> &str {
            "test"
        }

        fn on_init(&mut self, _ctx: &mut PluginContext) -> crate::Result<()> {
            self.init_called = true;
            Ok(())
        }

        fn on_mount(&mut self, _ctx: &mut PluginContext) -> crate::Result<()> {
            self.mount_called = true;
            Ok(())
        }

        fn on_tick(&mut self, _ctx: &mut PluginContext, _delta: Duration) -> crate::Result<()> {
            self.tick_count += 1;
            Ok(())
        }
    }

    #[test]
    fn test_plugin_default_implementations() {
        struct MinimalPlugin;
        impl Plugin for MinimalPlugin {
            fn name(&self) -> &str {
                "minimal"
            }
        }

        let plugin = MinimalPlugin;
        assert_eq!(plugin.name(), "minimal");
        assert_eq!(plugin.priority(), 0);
        assert!(plugin.styles().is_none());
    }

    #[test]
    fn test_plugin_lifecycle_calls() {
        let mut plugin = TestPlugin {
            init_called: false,
            mount_called: false,
            tick_count: 0,
        };
        let mut ctx = PluginContext::new();
        ctx.set_current_plugin("test");

        plugin.on_init(&mut ctx).unwrap();
        assert!(plugin.init_called);

        plugin.on_mount(&mut ctx).unwrap();
        assert!(plugin.mount_called);

        plugin.on_tick(&mut ctx, Duration::from_millis(16)).unwrap();
        assert_eq!(plugin.tick_count, 1);

        plugin.on_tick(&mut ctx, Duration::from_millis(16)).unwrap();
        assert_eq!(plugin.tick_count, 2);
    }

    #[test]
    fn test_plugin_with_styles() {
        struct StyledPlugin;
        impl Plugin for StyledPlugin {
            fn name(&self) -> &str {
                "styled"
            }
            fn styles(&self) -> Option<&str> {
                Some(".button { color: red; }")
            }
        }

        let plugin = StyledPlugin;
        assert_eq!(plugin.styles(), Some(".button { color: red; }"));
    }

    #[test]
    fn test_plugin_with_priority() {
        struct HighPriorityPlugin;
        impl Plugin for HighPriorityPlugin {
            fn name(&self) -> &str {
                "high"
            }
            fn priority(&self) -> i32 {
                100
            }
        }

        struct LowPriorityPlugin;
        impl Plugin for LowPriorityPlugin {
            fn name(&self) -> &str {
                "low"
            }
            fn priority(&self) -> i32 {
                -100
            }
        }

        assert_eq!(HighPriorityPlugin.priority(), 100);
        assert_eq!(LowPriorityPlugin.priority(), -100);
    }

    #[test]
    fn test_plugin_on_unmount() {
        struct UnmountPlugin {
            unmounted: bool,
        }
        impl Plugin for UnmountPlugin {
            fn name(&self) -> &str {
                "unmount"
            }
            fn on_unmount(&mut self, _ctx: &mut PluginContext) -> crate::Result<()> {
                self.unmounted = true;
                Ok(())
            }
        }

        let mut plugin = UnmountPlugin { unmounted: false };
        let mut ctx = PluginContext::new();
        ctx.set_current_plugin("unmount");

        plugin.on_unmount(&mut ctx).unwrap();
        assert!(plugin.unmounted);
    }
}
