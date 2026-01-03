# Getting Started with Revue

This tutorial will get you up and running with Revue in 5 minutes.

## Prerequisites

- Rust 1.70 or later
- A terminal that supports 256 colors (most modern terminals)

## Installation

Add Revue to your `Cargo.toml`:

```toml
[dependencies]
revue = "1.0"
```

## Your First App

Create a simple "Hello, World!" application:

```rust
use revue::prelude::*;

fn main() -> Result<()> {
    let mut app = App::builder().build();

    app.run_with_handler(HelloWorld, |event, _state| {
        !matches!(event.key, Key::Char('q') | Key::Escape)
    })
}

struct HelloWorld;

impl View for HelloWorld {
    fn render(&self, ctx: &mut RenderContext) {
        vstack()
            .child(Text::new("Hello, World!").bold())
            .child(Text::muted("Press 'q' to quit"))
            .render(ctx);
    }
}
```

Run your app:

```bash
cargo run
```

## Understanding the Basics

### The App Builder

Every Revue app starts with `App::builder()`:

```rust
let mut app = App::builder()
    .stylesheet("styles.css")  // Load CSS file
    .hot_reload(true)          // Enable hot reload
    .build();
```

### Views and Widgets

Views are the building blocks of your UI. Revue provides 70+ built-in widgets:

```rust
// Text display
Text::new("Hello").bold()

// Layout containers
vstack()                       // Vertical stack
    .child(Text::new("Top"))
    .child(Text::new("Bottom"))

hstack()                       // Horizontal stack
    .gap(2)
    .child(Text::new("Left"))
    .child(Text::new("Right"))

// Interactive widgets
Button::primary("Click Me")
Input::new().placeholder("Enter text...")
Checkbox::new("Enable feature").checked(true)
```

### Handling Events

Use `run_with_handler` to respond to keyboard events:

```rust
app.run_with_handler(state, |event, state| {
    match event.key {
        Key::Char('q') => false,  // Return false to quit
        Key::Up => {
            state.move_up();
            true                  // Return true to continue
        }
        _ => true,
    }
})
```

## Next Steps

- [Counter App Tutorial](./02-counter.md) - Build an interactive counter
- [Todo App Tutorial](./03-todo.md) - Create a full-featured todo list
- [Styling Guide](../guides/styling.md) - Learn about CSS-like styling

## Quick Reference

### Common Widgets

| Widget | Description | Example |
|--------|-------------|---------|
| `Text` | Display text | `Text::new("Hello").bold()` |
| `Button` | Clickable button | `Button::new("Submit").primary()` |
| `Input` | Text input | `Input::new().placeholder("...")` |
| `Progress` | Progress bar | `Progress::new(0.5)` |
| `Spinner` | Loading indicator | `Spinner::new()` |
| `List` | Scrollable list | `List::new(items)` |
| `Table` | Data table | `Table::new().columns(...)` |

### Layout Widgets

| Widget | Description | Example |
|--------|-------------|---------|
| `vstack()` | Vertical layout | `vstack().child(a).child(b)` |
| `hstack()` | Horizontal layout | `hstack().gap(2).child(a).child(b)` |
| `Border` | Add border | `Border::rounded().child(content)` |
| `ScrollView` | Scrollable area | `ScrollView::new(content)` |
| `Tabs` | Tabbed content | `Tabs::new().tab("A", view_a)` |

### Key Events

```rust
match event.key {
    Key::Char('q') => { /* q pressed */ }
    Key::Enter => { /* enter pressed */ }
    Key::Up | Key::Char('k') => { /* up navigation */ }
    Key::Down | Key::Char('j') => { /* down navigation */ }
    Key::Esc => { /* escape pressed */ }
    _ => {}
}
```
