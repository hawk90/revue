use revue::prelude::*;
use std::time::{Duration, Instant};

/// How often the simulated metrics change.
const UPDATE_INTERVAL: Duration = Duration::from_millis(500);

/// Dashboard app with multiple panels and real-time data.
struct Dashboard {
    cpu: Signal<f32>,
    memory: Signal<f32>,
    requests: Signal<u64>,
    errors: Signal<u64>,
    tick: Signal<u64>,
    last_update: Instant,
}

impl Dashboard {
    fn new() -> Self {
        Self {
            cpu: signal(45.0),
            memory: signal(62.0),
            requests: signal(1423),
            errors: signal(3),
            tick: signal(0),
            last_update: Instant::now(),
        }
    }

    /// Called on every `Event::Tick` (about 60 times a second). Advances the
    /// data every `UPDATE_INTERVAL` and returns whether the view must redraw.
    fn on_tick(&mut self) -> bool {
        if self.last_update.elapsed() < UPDATE_INTERVAL {
            return false;
        }
        self.last_update = Instant::now();
        self.update();
        true
    }

    fn update(&self) {
        self.tick.update(|v| *v += 1);
        let t = self.tick.get() as f32;
        self.cpu.set(40.0 + (t * 0.1).sin() * 20.0);
        self.memory.set(60.0 + (t * 0.05).cos() * 15.0);
        self.requests.update(|v| *v += 7);
        if self.tick.get().is_multiple_of(10) {
            self.errors.update(|v| *v += 1);
        }
    }
}

impl View for Dashboard {
    fn render(&self, ctx: &mut RenderContext) {
        let cpu = self.cpu.get();
        let mem = self.memory.get();
        let reqs = self.requests.get();
        let errs = self.errors.get();

        // Each panel takes `child_flex(.., 1.0)` so a row splits the width
        // evenly. A row of flex children has no content height of its own, so
        // each row gets a fixed height (content lines + 2 border lines), and
        // the body takes the rest of the screen so the footer sits on the last
        // line.
        let cpu_panel = Border::new().title("CPU").child(
            vstack()
                .child(Text::new(format!("{:.1}%", cpu)))
                .child(progress(cpu / 100.0)),
        );
        let memory_panel = Border::new().title("Memory").child(
            vstack()
                .child(Text::new(format!("{:.1}%", mem)))
                .child(progress(mem / 100.0)),
        );
        let requests_panel = Border::new()
            .title("Requests")
            .child(Text::new(format!("{}", reqs)).class("status-ok"));
        let errors_panel = Border::new()
            .title("Errors")
            .child(Text::new(format!("{}", errs)).class("status-error"));

        let body = vstack()
            .gap(1)
            .child_sized(
                hstack()
                    .gap(1)
                    .child_flex(cpu_panel, 1.0)
                    .child_flex(memory_panel, 1.0),
                4,
            )
            .child_sized(
                hstack()
                    .gap(1)
                    .child_flex(requests_panel, 1.0)
                    .child_flex(errors_panel, 1.0),
                3,
            );

        vstack()
            .gap(1)
            // `Text::heading` would set its own color, which outranks the
            // stylesheet; plain text lets `.panel-title` style it.
            .child(Text::new("System Dashboard").class("panel-title"))
            .child_flex(body, 1.0)
            .child(Text::muted(
                "Press 'q' to quit | Data updates automatically",
            ))
            .render(ctx);
    }

    fn meta(&self) -> WidgetMeta {
        WidgetMeta::new("Dashboard")
    }
}

fn main() -> Result<()> {
    let dashboard = Dashboard::new();

    App::builder()
        .style("src/style.css")
        .build()
        .run(dashboard, |event, view, app| match event {
            Event::Key(KeyEvent {
                key: Key::Char('q'),
                ..
            }) => {
                // Stop the loop so `run` restores the terminal on the way out.
                app.quit();
                false
            }
            // Returning `true` asks for a redraw.
            Event::Tick => view.on_tick(),
            _ => false,
        })
}
