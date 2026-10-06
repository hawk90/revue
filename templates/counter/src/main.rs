use revue::prelude::*;

/// Counter app demonstrating reactive state with Signal and Computed.
struct CounterApp {
    count: Signal<i32>,
    doubled: Computed<i32>,
}

impl CounterApp {
    fn new() -> Self {
        let count = signal(0);
        let count_clone = count.clone();
        let doubled = computed(move || count_clone.get() * 2);
        Self { count, doubled }
    }

    fn increment(&self) {
        self.count.update(|v| *v += 1);
    }

    fn decrement(&self) {
        self.count.update(|v| *v -= 1);
    }

    fn reset(&self) {
        self.count.set(0);
    }
}

impl View for CounterApp {
    fn render(&self, ctx: &mut RenderContext) {
        let count = self.count.get();
        let doubled = self.doubled.get();

        Border::new()
            .title("Counter")
            .child(
                vstack()
                    .gap(1)
                    .child(Text::new(format!("Count: {}", count)))
                    .child(Text::muted(format!("Doubled: {}", doubled)))
                    .child(Divider::new())
                    .child(Text::info(
                        "[+] increment  [-] decrement  [r] reset  [q] quit",
                    )),
            )
            .render(ctx);
    }

    fn meta(&self) -> WidgetMeta {
        WidgetMeta::new("CounterApp")
    }
}

fn main() -> Result<()> {
    let app_state = CounterApp::new();

    // The Border is this view's own body, so it is styled through the view's
    // type. A widget added with `.child(...)` gets its own node and class.
    App::builder()
        .css("CounterApp { border: rounded cyan; }")
        .build()
        .run(app_state, |event, view, app| {
            let Event::Key(KeyEvent { key, .. }) = event else {
                return false;
            };
            match key {
                Key::Char('+') | Key::Char('=') | Key::Up => view.increment(),
                Key::Char('-') | Key::Down => view.decrement(),
                Key::Char('r') => view.reset(),
                // Stop the loop so `run` restores the terminal on the way out.
                Key::Char('q') => app.quit(),
                _ => return false,
            }
            // Returning `true` asks for a redraw.
            true
        })
}
