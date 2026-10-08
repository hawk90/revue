//! Reactive counter example using Signal
//!
//! This demonstrates how to use the reactive system (Signal, Computed)
//! with the current widget API.
//!
//! Run with: cargo run --example counter

use revue::prelude::*;

/// Width of the key column in the Controls list.
const KEY_COLS: u16 = 9;

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

        // Each row and box here is `child_sized` to its content. (A stack sizes
        // children to their content on its own; the explicit sizes pin the
        // layout.)
        let view = vstack()
            .gap(1)
            .child_sized(
                Border::panel().title("🔄 Reactive Counter").child(
                    vstack()
                        .gap(1)
                        .child_sized(
                            Text::new(format!("Count: {}", count))
                                .fg(color)
                                .bold()
                                .align(Alignment::Center),
                            1,
                        )
                        .child_sized(
                            Text::new(format!("Doubled: {}", doubled))
                                .fg(Color::CYAN)
                                .align(Alignment::Center),
                            1,
                        )
                        .child_sized(
                            Text::new(format!("Status: {}", status))
                                .fg(Color::YELLOW)
                                .align(Alignment::Center),
                            1,
                        ),
                ),
                7,
            )
            .child_sized(
                Border::single().title("Controls").child(
                    vstack()
                        .child_sized(
                            hstack()
                                .gap(2)
                                .child_sized(Text::muted("[+/-/↑/↓]"), KEY_COLS)
                                .child(Text::new("Increment/Decrement")),
                            1,
                        )
                        .child_sized(
                            hstack()
                                .gap(2)
                                .child_sized(Text::muted("[r]"), KEY_COLS)
                                .child(Text::new("Reset")),
                            1,
                        )
                        .child_sized(
                            hstack()
                                .gap(2)
                                .child_sized(Text::muted("[q]"), KEY_COLS)
                                .child(Text::new("Quit")),
                            1,
                        ),
                ),
                5,
            )
            .child_sized(
                Border::rounded().title("ℹ️  How It Works").child(
                    vstack()
                        .child_sized(Text::success("✓ count is a Signal<i32>"), 1)
                        .child_sized(Text::success("✓ doubled is a Computed value"), 1)
                        .child_sized(Text::success("✓ status is computed based on count"), 1)
                        .child_sized(Text::info("→ Computed values auto-update!"), 1)
                        .child_sized(Text::info("→ No manual recalculation needed!"), 1),
                ),
                7,
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
