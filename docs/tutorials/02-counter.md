# Building a Counter App

In this tutorial, you'll build an interactive counter application that demonstrates state management and event handling in Revue.

## What You'll Learn

- Managing application state
- Handling keyboard events
- Using reactive signals
- Styling with themes

## The Basic Counter

Let's start with a simple counter using basic state:

```rust
use revue::prelude::*;

struct Counter {
    count: i32,
}

impl Counter {
    fn new() -> Self {
        Self { count: 0 }
    }

    fn increment(&mut self) {
        self.count += 1;
    }

    fn decrement(&mut self) {
        self.count = self.count.saturating_sub(1);
    }
}

impl View for Counter {
    fn render(&self, ctx: &mut RenderContext) {
        vstack()
            .child(Text::new("Counter App").bold())
            .child(Text::new(format!("Count: {}", self.count)))
            .child(Text::muted("[+] Increment  [-] Decrement  [q] Quit"))
            .render(ctx);
    }
}

fn main() -> Result<()> {
    let mut app = App::builder().build();
    let counter = Counter::new();

    // run_with_handler provides a simplified keyboard-only handler (key_event, view).
    // For full event access (mouse, resize, etc.), use app.run(view, |event, view, app| { ... }).
    app.run_with_handler(counter, |event, counter| {
        match event.key {
            Key::Char('q') | Key::Escape => false,
            Key::Char('+') | Key::Char('=') => {
                counter.increment();
                true
            }
            Key::Char('-') => {
                counter.decrement();
                true
            }
            _ => true,
        }
    })
}
```

## Adding Style

Let's make the counter look better with borders and colors:

```rust
impl View for Counter {
    fn render(&self, ctx: &mut RenderContext) {
        let count_display = Text::new(format!("{}", self.count))
            .style("font-size: 3; color: cyan;");

        let content = vstack()
            .gap(1)
            .child(Text::new("Counter").bold())
            .child(count_display)
            .child(
                hstack().gap(2)
                    .child(Button::new("-").on_click(|| {}))
                    .child(Button::new("+").on_click(|| {}))
            );

        Border::rounded()
            .child(content)
            .title(" Counter App ")
            .render(ctx);
    }
}
```

## Using Reactive Signals

For more complex apps, use reactive signals for automatic UI updates:

```rust
use revue::prelude::*;

fn main() -> Result<()> {
    let app = App::builder()
        .title("Reactive Counter")
        .build();

    // Create a reactive signal
    let count = signal(0i32);

    // Create a computed value that automatically updates
    let display = computed({
        let count = count.clone();
        move || format!("Count: {}", count.get())
    });

    // Create an effect for side effects
    effect({
        let count = count.clone();
        move || {
            let value = count.get();
            if value % 10 == 0 && value > 0 {
                announce(&format!("Milestone: {}", value));
            }
        }
    });

    app.run_with_handler(count, |event, count| {
        match event.key {
            Key::Char('q') => false,
            Key::Char('+') | Key::Char('=') | Key::Up => {
                count.set(count.get() + 1);
                true
            }
            Key::Char('-') | Key::Down => {
                count.set((count.get() - 1).max(0));
                true
            }
            Key::Char('r') => {
                count.set(0);  // Reset
                true
            }
            _ => true,
        }
    })
}
```

## Adding Multiple Counters

Extend the app to manage multiple counters:

```rust
struct MultiCounter {
    counters: Vec<i32>,
    selected: usize,
}

impl MultiCounter {
    fn new(count: usize) -> Self {
        Self {
            counters: vec![0; count],
            selected: 0,
        }
    }

    fn increment(&mut self) {
        self.counters[self.selected] += 1;
    }

    fn decrement(&mut self) {
        let val = &mut self.counters[self.selected];
        *val = val.saturating_sub(1);
    }

    fn next(&mut self) {
        self.selected = (self.selected + 1) % self.counters.len();
    }

    fn prev(&mut self) {
        self.selected = self.selected.checked_sub(1)
            .unwrap_or(self.counters.len() - 1);
    }
}

impl View for MultiCounter {
    fn render(&self, ctx: &mut RenderContext) {
        let mut row = hstack().gap(3);

        for (i, count) in self.counters.iter().enumerate() {
            let is_selected = i == self.selected;
            let counter_view = vstack()
                .child(Text::new(format!("Counter {}", i + 1)))
                .child(Text::new(format!("{}", count)).bold());

            let bordered = if is_selected {
                Border::new(counter_view)
                    .style("border-color: cyan;")
                    .rounded()
            } else {
                Border::new(counter_view).rounded()
            };

            row = row.child(bordered);
        }

        vstack()
            .child(Text::new("Multi-Counter").bold())
            .child(row)
            .child(Text::new("[←/→] Select  [+/-] Change  [q] Quit").muted())
            .render(ctx);
    }
}
```

## Adding Animation

Make the counter more dynamic with animations:

```rust
use revue::prelude::*;

struct AnimatedCounter {
    count: i32,
    animation: Tween<f64>,
}

impl AnimatedCounter {
    fn new() -> Self {
        Self {
            count: 0,
            animation: Tween::new(0.0, 300), // 300ms animation
        }
    }

    fn increment(&mut self) {
        self.count += 1;
        self.animation.animate_to(self.count as f64);
    }
}

impl View for AnimatedCounter {
    fn render(&self, ctx: &mut RenderContext) {
        let animated_value = self.animation.value();
        let scale = 1.0 + (animated_value.fract() * 0.2); // Bounce effect

        vstack()
            .child(Text::new(format!("{}", self.count))
                .style(&format!("transform: scale({:.2});", scale)))
            .render(ctx);
    }
}
```

## Complete Example

See the full working example at `examples/counter.rs`:

```bash
cargo run --example counter
```

## Exercises

1. **Step Counter**: Add a configurable step amount (increment by 1, 5, or 10)
2. **Counter History**: Track and display the last 5 values
3. **Target Counter**: Set a target value and show progress toward it
4. **Timer Counter**: Auto-increment the counter every second

## Next Steps

- [Todo App Tutorial](./03-todo.md) - Build a more complex application
- [State Management Guide](../guides/state.md) - Deep dive into reactive state
