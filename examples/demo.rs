//! Demo application showcasing Revue TUI framework
//!
//! Run with: cargo run --example demo

use revue::{
    app::App,
    event::Key,
    widget::{vstack, hstack, Text, List, Input, Border, Progress, View, RenderContext},
    style::Color,
};

/// Main application state
struct AppState {
    items: Vec<String>,
    selected: usize,
    counter: i32,
    input: Input,
    progress: f32,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            items: vec![
                "Item 1 - Welcome to Revue!".to_string(),
                "Item 2 - A Vue-style TUI".to_string(),
                "Item 3 - With CSS styling".to_string(),
                "Item 4 - And reactive state".to_string(),
                "Item 5 - Built in Rust".to_string(),
            ],
            selected: 0,
            counter: 0,
            input: Input::new().placeholder("Type here..."),
            progress: 0.3,
        }
    }
}

/// Main view component
struct MainView {
    state: AppState,
}

impl MainView {
    fn new() -> Self {
        Self {
            state: AppState::default(),
        }
    }

    fn handle_key(&mut self, key: &Key) -> bool {
        match key {
            // Vim-style navigation
            Key::Char('k') | Key::Up => {
                if self.state.selected > 0 {
                    self.state.selected -= 1;
                }
                true
            }
            Key::Char('j') | Key::Down => {
                if self.state.selected < self.state.items.len() - 1 {
                    self.state.selected += 1;
                }
                true
            }
            Key::Char('g') => {
                self.state.selected = 0;
                true
            }
            Key::Char('G') => {
                self.state.selected = self.state.items.len().saturating_sub(1);
                true
            }
            Key::Char('+') | Key::Char('=') => {
                self.state.counter += 1;
                self.state.progress = (self.state.progress + 0.1).min(1.0);
                true
            }
            Key::Char('-') => {
                self.state.counter -= 1;
                self.state.progress = (self.state.progress - 0.1).max(0.0);
                true
            }
            // Pass other keys to input
            _ => self.state.input.handle_key(key),
        }
    }
}

impl View for MainView {
    fn render(&self, ctx: &mut RenderContext) {
        // Title with border
        let title = Border::rounded()
            .title("Revue")
            .fg(Color::CYAN)
            .child(Text::new("TUI Framework Demo").fg(Color::WHITE).bold());

        // Instructions
        let instructions = Text::new("j/k: Nav | g/G: Top/Bot | +/-: Counter/Progress | q: Quit")
            .fg(Color::rgb(128, 128, 128));

        // Counter display
        let counter_text = format!("Counter: {}", self.state.counter);
        let counter = Text::new(counter_text).fg(Color::YELLOW);

        // Progress bar with border
        let progress_bar = Border::single()
            .title("Progress")
            .child(Progress::new(self.state.progress)
                .filled_color(Color::GREEN)
                .show_percentage(true));

        // Input with border
        let input_box = Border::single()
            .title("Input")
            .child(self.state.input.clone());

        // List with border
        let list = Border::double()
            .title("Items")
            .fg(Color::BLUE)
            .child(List::new(self.state.items.clone())
                .selected(self.state.selected)
                .highlight_bg(Color::BLUE)
                .highlight_fg(Color::WHITE));

        // Layout
        let layout = vstack()
            .gap(1)
            .child(title)
            .child(instructions)
            .child(hstack()
                .gap(1)
                .child(counter)
                .child(progress_bar))
            .child(input_box)
            .child(list);

        layout.render(ctx);
    }
}

fn main() -> revue::Result<()> {
    let mut app = App::builder().build();
    let view = MainView::new();

    app.run_with_handler(view, |key_event, view| {
        view.handle_key(&key_event.key)
    })
}
