//! Todo application

mod app;

use app::TodoApp;
use revue::prelude::*;

fn main() -> Result<()> {
    // The path is relative to the working directory: start the app with
    // `cargo run` from the project root.
    let mut app = App::builder().style("styles/main.css").build();

    app.run(TodoApp::new(), |event, todos, app| {
        let Event::Key(KeyEvent { key, .. }) = event else {
            return false;
        };
        // While you type a todo, `q` is text.
        if *key == Key::Char('q') && !todos.is_editing() {
            // Stop the loop so `run` restores the terminal on the way out.
            app.quit();
            return false;
        }
        // Returning `true` asks for a redraw.
        todos.handle_key(key)
    })
}
