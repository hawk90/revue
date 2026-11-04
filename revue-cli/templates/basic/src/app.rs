//! The application's root view

use revue::prelude::*;

/// A counter: state in a `Signal`, a view that renders it, and key handling.
pub struct Counter {
    count: Signal<i32>,
}

impl Counter {
    pub fn new() -> Self {
        Self { count: signal(0) }
    }

    /// Handles a key press and returns whether the view changed.
    pub fn handle_key(&mut self, key: &Key) -> bool {
        match key {
            Key::Char('+') | Key::Char('=') | Key::Up => self.count.update(|n| *n += 1),
            Key::Char('-') | Key::Down => self.count.update(|n| *n -= 1),
            Key::Char('r') => self.count.set(0),
            _ => return false,
        }
        true
    }
}

impl Default for Counter {
    fn default() -> Self {
        Self::new()
    }
}

impl View for Counter {
    fn render(&self, ctx: &mut RenderContext) {
        Border::rounded()
            .title(" Revue App ")
            .child(
                vstack()
                    .gap(1)
                    .child(Text::new("Hello from Revue!").class("title"))
                    .child(Text::new(format!("Count: {}", self.count.get())).class("count"))
                    .child(Text::muted(
                        "[+] increment  [-] decrement  [r] reset  [q] quit",
                    )),
            )
            .render(ctx);
    }

    fn meta(&self) -> WidgetMeta {
        WidgetMeta::new("Counter")
    }
}
