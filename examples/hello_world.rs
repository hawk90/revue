//! Simplest Revue example - Hello World
//!
//! Run with: cargo run --example hello_world

use revue::prelude::*;

fn main() -> Result<()> {
    let view = vstack()
        .gap(1)
        .child(Text::heading("Hello, Revue!"))
        .child(Text::muted("A Vue-style TUI framework for Rust"))
        .child(Text::info("Press 'q' or Ctrl+C to quit"));

    App::builder().build().run(view, |_event, _view, _app| false)
}
