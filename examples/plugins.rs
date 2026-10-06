//! Plugin system example
//!
//! This demonstrates how to create custom plugins, use PluginContext for data storage,
//! and enable communication between plugins.
//!
//! Run with: cargo run --example plugins

use revue::plugin::{LoggerPlugin, PerformancePlugin, Plugin, PluginContext};
use revue::prelude::*;
use revue::utils::unicode::display_width;
use std::time::Duration;

/// Width of the plugin-name column in "Active Plugins" (`PerformancePlugin`).
const NAME_COLS: u16 = 17;

/// Columns `text` occupies, for sizing single-line text in an `hstack`.
fn cols(text: &str) -> u16 {
    display_width(text) as u16
}

// =============================================================================
// Custom Plugin: Counter Plugin
// =============================================================================

/// A plugin that counts ticks and stores data in PluginContext
struct CounterPlugin {
    tick_count: u64,
    update_interval: usize,
}

impl CounterPlugin {
    fn new() -> Self {
        Self {
            tick_count: 0,
            update_interval: 10, // Update shared data every 10 ticks
        }
    }

    fn update_interval(mut self, interval: usize) -> Self {
        self.update_interval = interval;
        self
    }
}

impl Plugin for CounterPlugin {
    fn name(&self) -> &str {
        "counter"
    }

    fn priority(&self) -> i32 {
        50 // Run before stats plugin
    }

    fn on_init(&mut self, ctx: &mut PluginContext) -> Result<()> {
        // Initialize shared data
        ctx.set_data("tick_count", 0u64);
        ctx.set_data("started_at", std::time::Instant::now());
        ctx.log("Counter plugin initialized");
        Ok(())
    }

    fn on_tick(&mut self, ctx: &mut PluginContext, _delta: Duration) -> Result<()> {
        self.tick_count += 1;

        // Update shared data periodically
        if self.tick_count.is_multiple_of(self.update_interval as u64) {
            ctx.set_data("tick_count", self.tick_count);
        }

        Ok(())
    }

    fn on_unmount(&mut self, ctx: &mut PluginContext) -> Result<()> {
        ctx.log(&format!("Counter plugin: {} total ticks", self.tick_count));
        Ok(())
    }
}

// =============================================================================
// Custom Plugin: Stats Plugin (reads data from Counter Plugin)
// =============================================================================

/// A plugin that reads data from other plugins and computes statistics
struct StatsPlugin {
    samples: Vec<u64>,
    max_samples: usize,
    sample_interval: usize,
    tick_count: usize,
}

impl StatsPlugin {
    fn new() -> Self {
        Self {
            samples: Vec::new(),
            max_samples: 100,
            sample_interval: 30,
            tick_count: 0,
        }
    }
}

impl Plugin for StatsPlugin {
    fn name(&self) -> &str {
        "stats"
    }

    fn priority(&self) -> i32 {
        -10 // Run after counter plugin
    }

    fn on_init(&mut self, ctx: &mut PluginContext) -> Result<()> {
        // Initialize our own data
        ctx.set_data("avg_ticks", 0.0f64);
        ctx.set_data("sample_count", 0usize);
        ctx.log("Stats plugin initialized");
        Ok(())
    }

    fn on_tick(&mut self, ctx: &mut PluginContext, _delta: Duration) -> Result<()> {
        self.tick_count += 1;

        // Sample data from counter plugin periodically
        if self.tick_count.is_multiple_of(self.sample_interval) {
            // Read data from counter plugin (cross-plugin communication!)
            if let Some(&tick_count) = ctx.get_plugin_data::<u64>("counter", "tick_count") {
                self.samples.push(tick_count);

                // Keep only recent samples
                if self.samples.len() > self.max_samples {
                    self.samples.remove(0);
                }

                // Compute and store average
                let avg = self.samples.iter().sum::<u64>() as f64 / self.samples.len() as f64;
                ctx.set_data("avg_ticks", avg);
                ctx.set_data("sample_count", self.samples.len());
            }
        }

        Ok(())
    }
}

// =============================================================================
// Custom Plugin: Theme Plugin (contributes styles)
// =============================================================================

/// A plugin that contributes custom CSS styles
struct ThemePlugin {
    theme_name: String,
}

impl ThemePlugin {
    fn new(name: &str) -> Self {
        Self {
            theme_name: name.to_string(),
        }
    }
}

impl Plugin for ThemePlugin {
    fn name(&self) -> &str {
        "theme"
    }

    fn on_init(&mut self, ctx: &mut PluginContext) -> Result<()> {
        ctx.set_data("theme_name", self.theme_name.clone());
        ctx.log(&format!("Theme plugin: {} theme loaded", self.theme_name));
        Ok(())
    }

    fn styles(&self) -> Option<&str> {
        // Contribute custom styles to the app
        Some(
            r#"
.plugin-panel {
    border: double cyan;
    padding: 1;
}

.plugin-title {
    color: #7aa2f7;
    bold: true;
}

.plugin-value {
    color: #9ece6a;
}

.plugin-label {
    color: #565f89;
}
"#,
        )
    }
}

// =============================================================================
// App View
// =============================================================================

struct PluginDemoApp {
    show_help: bool,
}

impl PluginDemoApp {
    fn new() -> Self {
        Self { show_help: false }
    }

    fn handle_key(&mut self, key: &Key) -> bool {
        match key {
            Key::Char('h') | Key::Char('?') => {
                self.show_help = !self.show_help;
                true
            }
            _ => false,
        }
    }
}

impl View for PluginDemoApp {
    fn render(&self, ctx: &mut RenderContext) {
        // Every fixed-height row/box is `child_sized` to its content; the three
        // plugin panels split the width with `child_flex` - a stack would
        // otherwise size them to their content.
        let view = vstack()
            .gap(1)
            .child_sized(
                Text::new("🔌 Plugin System Demo")
                    .bold()
                    .fg(Color::CYAN)
                    .align(Alignment::Center),
                1,
            )
            .child_sized(
                hstack()
                    .gap(2)
                    .child_flex(self.render_counter_panel(), 1.0)
                    .child_flex(self.render_stats_panel(), 1.0)
                    .child_flex(self.render_performance_panel(), 1.0),
                7,
            )
            .child_sized(self.render_plugins_info(), 7)
            .child_sized(self.render_controls(), 3);

        view.render(ctx);
    }

    fn meta(&self) -> WidgetMeta {
        WidgetMeta::new("PluginDemoApp")
    }
}

impl PluginDemoApp {
    fn render_counter_panel(&self) -> Border {
        Border::panel()
            .title("Counter Plugin")
            .class("plugin-panel")
            .child(
                vstack()
                    .gap(1)
                    .child_sized(Text::new("Tick Count").class("plugin-label"), 1)
                    .child_sized(Text::muted("(updated every 10 ticks)"), 1)
                    .child_sized(
                        Text::new("Check logs for values")
                            .class("plugin-value")
                            .align(Alignment::Center),
                        1,
                    ),
            )
    }

    fn render_stats_panel(&self) -> Border {
        Border::panel()
            .title("Stats Plugin")
            .class("plugin-panel")
            .child(
                vstack()
                    .gap(1)
                    .child_sized(Text::new("Cross-Plugin Data").class("plugin-label"), 1)
                    .child_sized(Text::muted("(reads from Counter)"), 1)
                    .child_sized(
                        Text::new("Computes averages")
                            .class("plugin-value")
                            .align(Alignment::Center),
                        1,
                    ),
            )
    }

    fn render_performance_panel(&self) -> Border {
        Border::panel()
            .title("Performance Plugin")
            .class("plugin-panel")
            .child(
                vstack()
                    .gap(1)
                    .child_sized(Text::new("Built-in Plugin").class("plugin-label"), 1)
                    .child_sized(Text::muted("(FPS & frame time)"), 1)
                    .child_sized(
                        Text::new("Tracks metrics")
                            .class("plugin-value")
                            .align(Alignment::Center),
                        1,
                    ),
            )
    }

    fn render_plugins_info(&self) -> Border {
        Border::rounded().title("Active Plugins").child(
            vstack()
                .child_sized(
                    hstack()
                        .gap(2)
                        .child_sized(Text::new("1.").fg(Color::YELLOW), 2)
                        .child_sized(Text::new("LoggerPlugin").bold(), NAME_COLS)
                        .child(Text::muted("(priority: 100)")),
                    1,
                )
                .child_sized(
                    hstack()
                        .gap(2)
                        .child_sized(Text::new("2.").fg(Color::YELLOW), 2)
                        .child_sized(Text::new("CounterPlugin").bold(), NAME_COLS)
                        .child(Text::muted("(priority: 50)")),
                    1,
                )
                .child_sized(
                    hstack()
                        .gap(2)
                        .child_sized(Text::new("3.").fg(Color::YELLOW), 2)
                        .child_sized(Text::new("ThemePlugin").bold(), NAME_COLS)
                        .child(Text::muted("(priority: 0)")),
                    1,
                )
                .child_sized(
                    hstack()
                        .gap(2)
                        .child_sized(Text::new("4.").fg(Color::YELLOW), 2)
                        .child_sized(Text::new("PerformancePlugin").bold(), NAME_COLS)
                        .child(Text::muted("(priority: 0)")),
                    1,
                )
                .child_sized(
                    hstack()
                        .gap(2)
                        .child_sized(Text::new("5.").fg(Color::YELLOW), 2)
                        .child_sized(Text::new("StatsPlugin").bold(), NAME_COLS)
                        .child(Text::muted("(priority: -10)")),
                    1,
                ),
        )
    }

    fn render_controls(&self) -> Border {
        Border::single().title("Controls").child(
            hstack()
                .gap(4)
                .child_sized(
                    hstack()
                        .gap(1)
                        .child_sized(Text::muted("[q]"), 3)
                        .child(Text::new("Quit")),
                    cols("[q] Quit"),
                )
                .child(
                    hstack()
                        .gap(1)
                        .child_sized(Text::muted("[h]"), 3)
                        .child(Text::new("Toggle Help")),
                ),
        )
    }
}

// =============================================================================
// Main
// =============================================================================

fn main() -> Result<()> {
    println!("🔌 Plugin System Example");
    println!("========================\n");
    println!("This example demonstrates:");
    println!("  • Creating custom plugins");
    println!("  • Using PluginContext for data storage");
    println!("  • Cross-plugin communication");
    println!("  • Plugin-contributed styles");
    println!("  • Built-in plugins (Logger, Performance)\n");

    // Build app with multiple plugins
    let mut app = App::builder()
        // Built-in plugins
        .plugin(LoggerPlugin::new().verbose(true).log_interval(60))
        .plugin(PerformancePlugin::new().report_interval(Duration::from_secs(5)))
        // Custom plugins
        .plugin(CounterPlugin::new().update_interval(10))
        .plugin(StatsPlugin::new())
        .plugin(ThemePlugin::new("tokyo-night"))
        .build();

    let demo = PluginDemoApp::new();

    app.run(demo, |event, app_view, _app| match event {
        Event::Key(key_event) => app_view.handle_key(&key_event.key),
        _ => false,
    })
}
