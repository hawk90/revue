//! Simplest Revue example - Hello World
//!
//! Run with: cargo run --example hello_world

use revue::prelude::*;

fn main() -> Result<()> {
    // A stack shares its space equally among children added with `child`,
    // so give each one-line text its one row; the last line takes the rest.
    let view = vstack()
        .gap(1)
        .child_sized(Text::heading("Hello, Revue!"), 1)
        .child_sized(Text::muted("A Vue-style TUI framework for Rust"), 1)
        .child(Text::info("Press 'q' or Ctrl+C to quit"));

    App::builder()
        .build()
        .run(view, |_event, _view, _app| false)
}
