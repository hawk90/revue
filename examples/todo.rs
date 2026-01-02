//! Todo App example - Task management demo
//!
//! Run with: cargo run --example todo

use revue::prelude::*;

#[derive(Clone)]
struct TodoItem {
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
    items: Vec<TodoItem>,
    input: Input,
    selected: usize,
    filter: Filter,
    next_id: usize,
    editing: bool,
}

impl TodoApp {
    fn new() -> Self {
        Self {
            items: vec![
                TodoItem { id: 1, text: "Learn Revue framework".into(), completed: true },
                TodoItem { id: 2, text: "Build awesome TUI apps".into(), completed: false },
                TodoItem { id: 3, text: "Share with the community".into(), completed: false },
            ],
            input: Input::new().placeholder("What needs to be done?"),
            selected: 0,
            filter: Filter::All,
            next_id: 4,
            editing: false,
        }
    }

    fn filtered_items(&self) -> Vec<&TodoItem> {
        self.items.iter().filter(|item| match self.filter {
            Filter::All => true,
            Filter::Active => !item.completed,
            Filter::Completed => item.completed,
        }).collect()
    }

    fn active_count(&self) -> usize {
        self.items.iter().filter(|i| !i.completed).count()
    }

    fn completed_count(&self) -> usize {
        self.items.iter().filter(|i| i.completed).count()
    }

    fn handle_key(&mut self, key: &Key) -> bool {
        if self.editing {
            match key {
                Key::Enter => {
                    let text = self.input.text().trim().to_string();
                    if !text.is_empty() {
                        self.items.push(TodoItem {
                            id: self.next_id,
                            text,
                            completed: false,
                        });
                        self.next_id += 1;
                        self.input.clear();
                    }
                    self.editing = false;
                    true
                }
                Key::Escape => {
                    self.editing = false;
                    self.input.clear();
                    true
                }
                _ => self.input.handle_key(key),
            }
        } else {
            match key {
                Key::Char('a') | Key::Char('i') => {
                    self.editing = true;
                    true
                }
                Key::Up | Key::Char('k') => {
                    if self.selected > 0 {
                        self.selected -= 1;
                    }
                    true
                }
                Key::Down | Key::Char('j') => {
                    let max_idx = self.filtered_items().len().saturating_sub(1);
                    if self.selected < max_idx {
                        self.selected += 1;
                    }
                    true
                }
                Key::Enter | Key::Char(' ') => {
                    let filtered: Vec<_> = self.filtered_items().iter().map(|i| i.id).collect();
                    if let Some(&id) = filtered.get(self.selected) {
                        if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
                            item.completed = !item.completed;
                        }
                    }
                    true
                }
                Key::Char('d') | Key::Delete => {
                    let filtered: Vec<_> = self.filtered_items().iter().map(|i| i.id).collect();
                    if let Some(&id) = filtered.get(self.selected) {
                        self.items.retain(|i| i.id != id);
                        if self.selected > 0 && self.selected >= self.filtered_items().len() {
                            self.selected -= 1;
                        }
                    }
                    true
                }
                Key::Char('1') => {
                    self.filter = Filter::All;
                    self.selected = 0;
                    true
                }
                Key::Char('2') => {
                    self.filter = Filter::Active;
                    self.selected = 0;
                    true
                }
                Key::Char('3') => {
                    self.filter = Filter::Completed;
                    self.selected = 0;
                    true
                }
                Key::Char('c') => {
                    self.items.retain(|i| !i.completed);
                    self.selected = 0;
                    true
                }
                _ => false,
            }
        }
    }
}

impl View for TodoApp {
    fn render(&self, ctx: &mut RenderContext) {
        // Header
        let header = Border::rounded()
            .child(Text::new("Todo App").fg(Color::MAGENTA).bold());

        // Input area
        let input_border = if self.editing {
            Border::rounded().fg(Color::CYAN)
        } else {
            Border::rounded().fg(Color::rgb(80, 80, 80))
        };
        let input_area = input_border.child(self.input.clone());

        // Filter tabs
        let filters = [
            ("1:All", Filter::All),
            ("2:Active", Filter::Active),
            ("3:Done", Filter::Completed),
        ];
        let mut filter_bar = hstack().gap(2);
        for (label, f) in filters.iter() {
            let text = if *f == self.filter {
                Text::new(*label).fg(Color::CYAN).bold()
            } else {
                Text::new(*label).fg(Color::rgb(100, 100, 100))
            };
            filter_bar = filter_bar.child(text);
        }

        // Todo list
        let filtered = self.filtered_items();
        let list_content = if filtered.is_empty() {
            vstack().child(Text::new("No items").fg(Color::rgb(100, 100, 100)).italic())
        } else {
            let mut list = vstack();
            for (i, item) in filtered.iter().enumerate() {
                let checkbox = if item.completed { "[x]" } else { "[ ]" };
                let checkbox_color = if item.completed { Color::GREEN } else { Color::rgb(80, 80, 80) };
                let text_color = if item.completed { Color::rgb(100, 100, 100) } else { Color::WHITE };

                // Create item text (with visual strikethrough using dashes for completed)
                let item_text = if item.completed {
                    format!("{} ~~{}~~", checkbox, item.text)
                } else {
                    format!("{} {}", checkbox, item.text)
                };

                let row = if i == self.selected {
                    Text::new(item_text).fg(Color::CYAN).bold()
                } else {
                    Text::new(format!("{} {}", checkbox, item.text))
                        .fg(if item.completed { checkbox_color } else { text_color })
                };
                list = list.child(row);
            }
            list
        };

        let list_panel = Border::rounded()
            .title(format!("Tasks ({})", filtered.len()))
            .child(list_content);

        // Status bar
        let status = hstack()
            .child(Text::new(format!("{} active  ", self.active_count())).fg(Color::YELLOW))
            .child(Text::new(format!("{} completed", self.completed_count())).fg(Color::GREEN));

        // Help
        let help = if self.editing {
            Text::new("Enter: Add | Esc: Cancel").fg(Color::rgb(80, 80, 80))
        } else {
            Text::new("a: Add | Space: Toggle | d: Delete | c: Clear done | q: Quit").fg(Color::rgb(80, 80, 80))
        };

        // Layout
        let layout = vstack()
            .gap(1)
            .child(header)
            .child(input_area)
            .child(filter_bar)
            .child(list_panel)
            .child(status)
            .child(help);

        layout.render(ctx);
    }
}

fn main() -> Result<()> {
    let mut app = App::builder().build();
    let todo = TodoApp::new();

    app.run_with_handler(todo, |key_event, todo| {
        todo.handle_key(&key_event.key)
    })
}
