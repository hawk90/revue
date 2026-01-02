//! Interactive counter example
//!
//! Run with: cargo run --example counter

use revue::prelude::*;

struct Counter {
    value: i32,
}

impl Counter {
    fn new() -> Self {
        Self { value: 0 }
    }

    fn handle_key(&mut self, key: &Key) -> bool {
        match key {
            Key::Up | Key::Char('k') | Key::Char('+') => {
                self.value += 1;
                true
            }
            Key::Down | Key::Char('j') | Key::Char('-') => {
                self.value -= 1;
                true
            }
            Key::Char('r') => {
                self.value = 0;
                true
            }
            _ => false,
        }
    }
}

impl View for Counter {
    fn render(&self, ctx: &mut RenderContext) {
        let color = if self.value > 0 {
            Color::GREEN
        } else if self.value < 0 {
            Color::RED
        } else {
            Color::WHITE
        };

        let view = vstack()
            .gap(1)
            .child(
                Border::panel()
                    .title("Counter")
                    .child(Text::new(format!("{:^20}", self.value)).fg(color).bold()),
            )
            .child(
                hstack()
                    .gap(2)
                    .child(Text::muted("[+/-]"))
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
            );

        view.render(ctx);
    }
}

fn main() -> Result<()> {
    let mut app = App::builder().build();
    let counter = Counter::new();

    app.run_with_handler(counter, |key_event, counter| {
        counter.handle_key(&key_event.key)
    })
}
