//! Chat application

mod app;

use app::ChatApp;
use revue::prelude::*;

fn main() -> Result<()> {
    // The path is relative to the working directory: start the app with
    // `cargo run` from the project root.
    let mut app = App::builder().style("styles/main.css").build();

    app.run(ChatApp::new(), |event, chat, app| {
        let Event::Key(KeyEvent { key, .. }) = event else {
            return false;
        };
        // Every printable key is text for the message, so quit on Esc.
        if *key == Key::Escape {
            // Stop the loop so `run` restores the terminal on the way out.
            app.quit();
            return false;
        }
        // Returning `true` asks for a redraw.
        chat.handle_key(key)
    })
}
