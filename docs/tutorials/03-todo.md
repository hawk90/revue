# Building a Todo App

In this tutorial, you'll build a full-featured todo list application. This demonstrates real-world patterns for state management, list handling, and user input.

## What You'll Build

- Add, complete, and delete todos
- Filter by status (all, active, completed)
- Persist todos (optional)
- Keyboard navigation

## Project Structure

```rust
use revue::prelude::*;

#[derive(Clone)]
struct Todo {
    id: usize,
    text: String,
    completed: bool,
}

#[derive(Clone, Copy, PartialEq)]
enum Filter {
    All,
    Active,
    Completed,
}

struct TodoApp {
    todos: Vec<Todo>,
    filter: Filter,
    input: String,
    selected: usize,
    next_id: usize,
    editing: bool,
}
```

## Implementing the App

### Initialization

```rust
impl TodoApp {
    fn new() -> Self {
        Self {
            todos: vec![
                Todo { id: 1, text: "Learn Revue".into(), completed: true },
                Todo { id: 2, text: "Build an app".into(), completed: false },
                Todo { id: 3, text: "Deploy to production".into(), completed: false },
            ],
            filter: Filter::All,
            input: String::new(),
            selected: 0,
            next_id: 4,
            editing: false,
        }
    }

    fn filtered_todos(&self) -> Vec<&Todo> {
        self.todos.iter()
            .filter(|t| match self.filter {
                Filter::All => true,
                Filter::Active => !t.completed,
                Filter::Completed => t.completed,
            })
            .collect()
    }

    fn add_todo(&mut self) {
        if !self.input.trim().is_empty() {
            self.todos.push(Todo {
                id: self.next_id,
                text: self.input.trim().to_string(),
                completed: false,
            });
            self.next_id += 1;
            self.input.clear();
            announce_success("Todo added");
        }
    }

    fn toggle_selected(&mut self) {
        let filtered = self.filtered_todos();
        if let Some(todo) = filtered.get(self.selected) {
            let id = todo.id;
            if let Some(t) = self.todos.iter_mut().find(|t| t.id == id) {
                t.completed = !t.completed;
                let status = if t.completed { "completed" } else { "active" };
                announce(&format!("Todo marked as {}", status));
            }
        }
    }

    fn delete_selected(&mut self) {
        let filtered = self.filtered_todos();
        if let Some(todo) = filtered.get(self.selected) {
            let id = todo.id;
            self.todos.retain(|t| t.id != id);
            self.selected = self.selected.saturating_sub(1);
            announce("Todo deleted");
        }
    }

    fn clear_completed(&mut self) {
        let count = self.todos.iter().filter(|t| t.completed).count();
        self.todos.retain(|t| !t.completed);
        announce(&format!("{} todos cleared", count));
    }
}
```

### Rendering the UI

```rust
impl View for TodoApp {
    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;

        // Main container
        let content = vstack()
            .child(self.render_header())
            .child(self.render_input())
            .child(self.render_list())
            .child(self.render_footer());

        Border::new(content)
            .title(" Todo App ")
            .rounded()
            .render(ctx);
    }
}

impl TodoApp {
    fn render_header(&self) -> impl View {
        let total = self.todos.len();
        let completed = self.todos.iter().filter(|t| t.completed).count();
        let active = total - completed;

        hstack()
            .child(Text::new("Todo List").bold())
            .child(Text::new(format!("{} active, {} completed", active, completed)).muted())
    }

    fn render_input(&self) -> impl View {
        hstack().gap(1)
            .child(Text::new(">"))
            .child(
                if self.editing {
                    Input::new()
                        .value(&self.input)
                        .placeholder("What needs to be done?")
                        .focused()
                } else {
                    Input::new()
                        .value(&self.input)
                        .placeholder("Press 'a' to add new todo")
                }
            )
    }

    fn render_list(&self) -> impl View {
        let filtered = self.filtered_todos();
        let mut list = vstack();

        if filtered.is_empty() {
            return vstack().child(
                Text::new(match self.filter {
                    Filter::All => "No todos yet. Press 'a' to add one!",
                    Filter::Active => "No active todos!",
                    Filter::Completed => "No completed todos!",
                }).muted()
            );
        }

        for (i, todo) in filtered.iter().enumerate() {
            let is_selected = i == self.selected;

            let checkbox = if todo.completed { "[x]" } else { "[ ]" };
            let prefix = if is_selected { ">" } else { " " };

            let line = format!("{} {} {}", prefix, checkbox, todo.text);

            let text = if is_selected {
                Text::new(line).bold()
            } else if todo.completed {
                Text::new(line).style("text-decoration: line-through; color: gray;")
            } else {
                Text::new(line)
            };

            list = list.child(text);
        }

        list
    }

    fn render_footer(&self) -> impl View {
        // Filter tabs
        let filters = hstack().gap(2)
            .child(self.filter_button("All", Filter::All))
            .child(self.filter_button("Active", Filter::Active))
            .child(self.filter_button("Completed", Filter::Completed));

        vstack()
            .child(filters)
            .child(Text::new("[a]dd [Enter]toggle [d]elete [c]lear completed [q]uit").muted())
    }

    fn filter_button(&self, label: &str, filter: Filter) -> impl View {
        if self.filter == filter {
            Text::new(format!("[{}]", label)).bold()
        } else {
            Text::new(format!(" {} ", label))
        }
    }
}
```

### Event Handling

```rust
fn main() -> Result<()> {
    let app = App::builder()
        .title("Todo App")
        .build();

    let todo_app = TodoApp::new();

    app.run_with_handler(todo_app, |event, state| {
        if state.editing {
            // Input mode
            match event.key {
                Key::Esc => {
                    state.editing = false;
                    true
                }
                Key::Enter => {
                    state.add_todo();
                    state.editing = false;
                    true
                }
                Key::Backspace => {
                    state.input.pop();
                    true
                }
                Key::Char(c) => {
                    state.input.push(c);
                    true
                }
                _ => true,
            }
        } else {
            // Navigation mode
            match event.key {
                Key::Char('q') | Key::Esc => false,
                Key::Char('a') => {
                    state.editing = true;
                    true
                }
                Key::Up | Key::Char('k') => {
                    state.selected = state.selected.saturating_sub(1);
                    true
                }
                Key::Down | Key::Char('j') => {
                    let max = state.filtered_todos().len().saturating_sub(1);
                    state.selected = (state.selected + 1).min(max);
                    true
                }
                Key::Enter | Key::Char(' ') => {
                    state.toggle_selected();
                    true
                }
                Key::Char('d') | Key::Delete => {
                    state.delete_selected();
                    true
                }
                Key::Char('c') => {
                    state.clear_completed();
                    true
                }
                Key::Char('1') => {
                    state.filter = Filter::All;
                    true
                }
                Key::Char('2') => {
                    state.filter = Filter::Active;
                    true
                }
                Key::Char('3') => {
                    state.filter = Filter::Completed;
                    true
                }
                _ => true,
            }
        }
    })
}
```

## Adding Persistence

Save todos to a file:

```rust
use std::fs;
use std::path::PathBuf;

impl TodoApp {
    fn save_path() -> PathBuf {
        dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("revue-todo.json")
    }

    fn load() -> Self {
        let path = Self::save_path();
        if path.exists() {
            if let Ok(data) = fs::read_to_string(&path) {
                if let Ok(todos) = serde_json::from_str(&data) {
                    return Self {
                        todos,
                        ..Self::new()
                    };
                }
            }
        }
        Self::new()
    }

    fn save(&self) {
        if let Ok(data) = serde_json::to_string_pretty(&self.todos) {
            let _ = fs::write(Self::save_path(), data);
        }
    }
}
```

## Using DevTools

Enable DevTools to debug your app:

```rust
fn main() -> Result<()> {
    // Enable devtools globally
    enable_devtools();

    let app = App::builder()
        .title("Todo App")
        .with_devtools(true)  // F12 to toggle
        .build();

    // ...
}
```

## Complete Example

See the full working example at `examples/reactive_todo.rs`:

```bash
cargo run --example reactive_todo
```

## Exercises

1. **Due Dates**: Add due dates to todos with date picker
2. **Categories**: Add categories/tags to todos
3. **Search**: Add fuzzy search for todos
4. **Drag & Drop**: Reorder todos with drag and drop
5. **Undo/Redo**: Add undo/redo functionality

## Next Steps

- [Testing Guide](../guides/testing.md) - Learn to test your app
- [Accessibility Guide](../guides/accessibility.md) - Make your app accessible
