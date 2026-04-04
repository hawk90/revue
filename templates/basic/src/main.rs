use revue::prelude::*;

fn main() -> Result<()> {
    let view = vstack()
        .gap(1)
        .child(Text::heading("Hello, Revue!"))
        .child(Text::muted("A Vue-style TUI framework for Rust"))
        .child(Text::info("Press 'q' to quit"));

    App::builder().build().run(view, |event, _view, _app| {
        if let Event::Key(KeyEvent { key: Key::Char('q'), .. }) = event {
            std::process::exit(0);
        }
        false
    })
}
