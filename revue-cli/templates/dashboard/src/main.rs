//! Dashboard application

mod app;

use app::Dashboard;
use revue::prelude::*;

fn main() -> Result<()> {
    // The path is relative to the working directory: start the app with
    // `cargo run` from the project root.
    let mut app = App::builder().style("styles/main.css").build();

    app.run(Dashboard::new(), |event, dashboard, app| match event {
        Event::Key(KeyEvent {
            key: Key::Char('q') | Key::Escape,
            ..
        }) => {
            // Stop the loop so `run` restores the terminal on the way out.
            app.quit();
            false
        }
        // Returning `true` asks for a redraw.
        Event::Tick => dashboard.on_tick(),
        _ => false,
    })
}
