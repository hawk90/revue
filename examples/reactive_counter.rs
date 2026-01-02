//! Reactive counter example using Signal
//!
//! This demonstrates how to use the reactive system (Signal, Computed, Effect)
//! with the current widget API.
//!
//! Run with: cargo run --example reactive_counter

use revue::prelude::*;

/// A counter widget that uses Signal for reactive state
struct ReactiveCounter {
    /// Reactive counter value
    count: Signal<i32>,
    /// Computed doubled value
    doubled: Computed<i32>,
    /// Computed status message
    status: Computed<String>,
}

impl ReactiveCounter {
    fn new() -> Self {
        // Create reactive signal
        let count = signal(0);

        // Create computed value that doubles the count
        let count_clone = count.clone();
        let doubled = computed(move || count_clone.get() * 2);

        // Create computed status message based on count
        let count_clone2 = count.clone();
        let status = computed(move || {
            let value = count_clone2.get();
            if value > 0 {
                format!("Positive: {}", value)
            } else if value < 0 {
                format!("Negative: {}", value)
            } else {
                "Zero".to_string()
            }
        });

        // Set up effect to log changes (optional)
        let count_clone3 = count.clone();
        effect(move || {
            let value = count_clone3.get();
            println!("Counter changed to: {}", value);
        });

        Self {
            count,
            doubled,
            status,
        }
    }

    fn increment(&mut self) {
        self.count.update(|v| *v += 1);
    }

    fn decrement(&mut self) {
        self.count.update(|v| *v -= 1);
    }

    fn reset(&mut self) {
        self.count.set(0);
    }

    fn handle_key(&mut self, key: &Key) -> bool {
        match key {
            Key::Up | Key::Char('k') | Key::Char('+') => {
                self.increment();
                true
            }
            Key::Down | Key::Char('j') | Key::Char('-') => {
                self.decrement();
                true
            }
            Key::Char('r') => {
                self.reset();
                true
            }
            _ => false,
        }
    }
}

impl View for ReactiveCounter {
    fn render(&self, ctx: &mut RenderContext) {
        // Get reactive values - these are cached and only recompute when dependencies change!
        let count = self.count.get();
        let doubled = self.doubled.get();
        let status = self.status.get();

        let color = if count > 0 {
            Color::GREEN
        } else if count < 0 {
            Color::RED
        } else {
            Color::WHITE
        };

        let view = vstack()
            .gap(1)
            .child(
                Border::panel().title("🔄 Reactive Counter").child(
                    vstack()
                        .gap(1)
                        .child(
                            Text::new(format!("Count: {}", count))
                                .fg(color)
                                .bold()
                                .align(Alignment::Center),
                        )
                        .child(
                            Text::new(format!("Doubled: {}", doubled))
                                .fg(Color::CYAN)
                                .align(Alignment::Center),
                        )
                        .child(
                            Text::new(format!("Status: {}", status))
                                .fg(Color::YELLOW)
                                .align(Alignment::Center),
                        ),
                ),
            )
            .child(
                Border::single().title("Controls").child(
                    vstack()
                        .child(
                            hstack()
                                .gap(2)
                                .child(Text::muted("[+/-/↑/↓]"))
                                .child(Text::new("Increment/Decrement")),
                        )
                        .child(
                            hstack()
                                .gap(2)
                                .child(Text::muted("[r]"))
                                .child(Text::new("Reset")),
                        )
                        .child(
                            hstack()
                                .gap(2)
                                .child(Text::muted("[q]"))
                                .child(Text::new("Quit")),
                        ),
                ),
            )
            .child(
                Border::rounded().title("ℹ️  How It Works").child(
                    vstack()
                        .child(Text::success("✓ count is a Signal<i32>"))
                        .child(Text::success("✓ doubled is a Computed value"))
                        .child(Text::success("✓ status is computed based on count"))
                        .child(Text::info("→ Computed values auto-update!"))
                        .child(Text::info("→ No manual recalculation needed!")),
                ),
            );

        view.render(ctx);
    }

    fn meta(&self) -> WidgetMeta {
        WidgetMeta::new("ReactiveCounter")
    }
}

fn main() -> Result<()> {
    println!("🔄 Reactive Counter Example");
    println!("This example demonstrates Signal, Computed, and Effect.\n");

    let mut app = App::builder().build();
    let counter = ReactiveCounter::new();

    app.run(counter, |event, counter, _app| match event {
        Event::Key(key_event) => counter.handle_key(&key_event.key),
        _ => false,
    })
}
