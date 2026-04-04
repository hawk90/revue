use revue::prelude::*;

/// Dashboard app with multiple panels and real-time data.
struct Dashboard {
    cpu: Signal<f32>,
    memory: Signal<f32>,
    requests: Signal<u64>,
    errors: Signal<u64>,
    tick: Signal<u64>,
}

impl Dashboard {
    fn new() -> Self {
        Self {
            cpu: signal(45.0),
            memory: signal(62.0),
            requests: signal(1423),
            errors: signal(3),
            tick: signal(0),
        }
    }

    fn update(&self) {
        self.tick.update(|v| *v += 1);
        let t = self.tick.get() as f32;
        self.cpu.set(40.0 + (t * 0.1).sin() * 20.0);
        self.memory.set(60.0 + (t * 0.05).cos() * 15.0);
        self.requests.update(|v| *v += 7);
        if self.tick.get() % 10 == 0 {
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

        vstack()
            .gap(1)
            .child(
                Text::heading("System Dashboard")
                    .class("panel-title"),
            )
            .child(
                hstack()
                    .gap(1)
                    .child(
                        Border::panel()
                            .title("CPU")
                            .child(
                                vstack()
                                    .child(Text::new(format!("{:.1}%", cpu)))
                                    .child(progress().value(cpu / 100.0)),
                            ),
                    )
                    .child(
                        Border::panel()
                            .title("Memory")
                            .child(
                                vstack()
                                    .child(Text::new(format!("{:.1}%", mem)))
                                    .child(progress().value(mem / 100.0)),
                            ),
                    ),
            )
            .child(
                hstack()
                    .gap(1)
                    .child(
                        Border::panel()
                            .title("Requests")
                            .child(Text::new(format!("{}", reqs)).class("status-ok")),
                    )
                    .child(
                        Border::panel()
                            .title("Errors")
                            .child(Text::new(format!("{}", errs)).class("status-error")),
                    ),
            )
            .child(Text::muted("Press 'q' to quit | Data updates automatically"))
            .render(ctx);
    }

    impl_view_meta!("Dashboard");
}

fn main() -> Result<()> {
    let dashboard = Dashboard::new();

    App::builder()
        .style("src/style.css")
        .build()
        .run(dashboard, |event, view, _app| {
            match event {
                Event::Key(KeyEvent { key: Key::Char('q'), .. }) => std::process::exit(0),
                Event::Tick => {
                    view.update();
                }
                _ => {}
            }
            false
        })
}
