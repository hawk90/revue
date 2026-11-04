//! Dashboard view

use revue::prelude::*;
use std::time::{Duration, Instant};

/// How often the simulated metrics change.
const UPDATE_INTERVAL: Duration = Duration::from_secs(1);

/// How many request samples the sparkline keeps: more than a terminal is
/// wide, so the line is always full.
const HISTORY: usize = 300;

/// A gauge at or above this percentage counts as an alert.
const ALERT_AT: f64 = 70.0;

pub struct Dashboard {
    cpu_usage: Signal<f64>,
    memory_usage: Signal<f64>,
    disk_usage: Signal<f64>,
    requests: Signal<Vec<u64>>,
    updates: Signal<u64>,
    last_update: Instant,
}

impl Dashboard {
    pub fn new() -> Self {
        Self {
            cpu_usage: signal(45.2),
            memory_usage: signal(62.8),
            disk_usage: signal(34.5),
            requests: signal((0..HISTORY).map(|i| requests_at(i as f64)).collect()),
            updates: signal(0),
            last_update: Instant::now(),
        }
    }

    /// Called on every `Event::Tick`. Advances the simulated metrics once per
    /// `UPDATE_INTERVAL` and returns whether the view must redraw.
    pub fn on_tick(&mut self) -> bool {
        if self.last_update.elapsed() < UPDATE_INTERVAL {
            return false;
        }
        self.last_update = Instant::now();

        self.updates.update(|n| *n += 1);
        let t = self.updates.get() as f64;
        self.cpu_usage.set(50.0 + (t * 0.4).sin() * 30.0);
        self.memory_usage.set(62.0 + (t * 0.15).cos() * 10.0);
        self.disk_usage.set((34.5 + t * 0.05).min(100.0));
        self.requests.update(|samples| {
            samples.remove(0);
            samples.push(requests_at(HISTORY as f64 + t));
        });
        true
    }

    fn gauge_panel(title: &str, percent: f64) -> Border {
        Border::rounded().title(format!(" {title} ")).child(
            Gauge::new()
                .percent(percent)
                // A gauge draws at most `width` cells; this fills its panel.
                .width(u16::MAX)
                .thresholds(ALERT_AT / 100.0, 0.9),
        )
    }
}

/// Simulated requests per second at sample `t`.
fn requests_at(t: f64) -> u64 {
    (150.0 + (t * 0.3).sin() * 40.0 + (t * 1.7).cos() * 15.0) as u64
}

impl Default for Dashboard {
    fn default() -> Self {
        Self::new()
    }
}

impl View for Dashboard {
    fn render(&self, ctx: &mut RenderContext) {
        let usage = [
            ("CPU", self.cpu_usage.get()),
            ("Memory", self.memory_usage.get()),
            ("Disk", self.disk_usage.get()),
        ];
        let alerts = usage.iter().filter(|(_, p)| *p >= ALERT_AT).count();
        let requests = self.requests.get();
        let latest = requests.last().copied().unwrap_or_default();

        // A row of `child_flex` children has no content height of its own,
        // so each row gets a fixed height with `child_sized`: its content
        // lines plus 2 for the border.
        let mut gauges = hstack().gap(1);
        for (title, percent) in usage {
            gauges = gauges.child_flex(Self::gauge_panel(title, percent), 1.0);
        }

        let requests_panel = Border::rounded()
            .title(format!(" Requests/s: {latest} "))
            .child(Sparkline::new(requests.iter().map(|&r| r as f64)));

        let alert_badge = match alerts {
            0 => Badge::new("No alerts").info(),
            1 => Badge::new("1 alert").warning(),
            n => Badge::new(format!("{n} alerts")).warning(),
        };
        let status = hstack()
            .gap(1)
            .child_sized(Badge::new("Online").success(), 8)
            .child_sized(alert_badge, 11)
            .child(Text::muted(format!(
                "Updates: {}  [q] quit",
                self.updates.get()
            )));

        vstack()
            .gap(1)
            .child_sized(Text::new("System Dashboard").class("title"), 1)
            .child_sized(gauges, 3)
            .child_sized(requests_panel, 3)
            .child_sized(status, 1)
            .render(ctx);
    }

    fn meta(&self) -> WidgetMeta {
        WidgetMeta::new("Dashboard")
    }
}
