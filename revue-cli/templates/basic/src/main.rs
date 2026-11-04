//! Basic Revue application

mod app;

use app::Counter;
use revue::prelude::*;

fn main() -> Result<()> {
    // The path is relative to the working directory: start the app with
    // `cargo run` from the project root.
    let mut app = App::builder().style("styles/main.css").build();

    app.run(Counter::new(), |event, counter, app| {
        let Event::Key(KeyEvent { key, .. }) = event else {
            return false;
        };
        if let Key::Char('q') | Key::Escape = key {
            // Stop the loop so `run` restores the terminal on the way out.
            app.quit();
            return false;
        }
        // Returning `true` asks for a redraw.
        counter.handle_key(key)
    })
}
