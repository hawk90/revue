//! Reactive Todo List example
//!
//! Demonstrates advanced reactive patterns:
//! - Multiple signals
//! - Computed derived state
//! - Complex state transformations
//!
//! Run with: cargo run --example reactive_todo

use revue::prelude::*;

#[derive(Clone, Debug, PartialEq)]
enum Filter {
    All,
    Active,
    Completed,
}

impl Filter {
    fn matches(&self, completed: bool) -> bool {
        match self {
            Filter::All => true,
            Filter::Active => !completed,
            Filter::Completed => completed,
        }
    }

    fn label(&self) -> &str {
        match self {
            Filter::All => "All",
            Filter::Active => "Active",
            Filter::Completed => "Completed",
        }
    }
}

#[derive(Clone, Debug)]
struct TodoItem {
    text: String,
    completed: bool,
}

struct ReactiveTodoList {
    /// All todo items (reactive)
    items: Signal<Vec<TodoItem>>,
    /// Current filter (reactive)
    filter: Signal<Filter>,
    /// Input buffer for new items
    input: Signal<String>,
    /// Selected index
    selected: Signal<usize>,

    // Computed values (automatically update when dependencies change)
    /// Filtered items based on current filter
    filtered_items: Computed<Vec<TodoItem>>,
    /// Count of active items
    active_count: Computed<usize>,
    /// Count of completed items
    completed_count: Computed<usize>,
    /// Total count
    total_count: Computed<usize>,
}

impl ReactiveTodoList {
    fn new() -> Self {
        // Initialize reactive state
        let items = signal(vec![
            TodoItem { text: "Learn Revue TUI".to_string(), completed: false },
            TodoItem { text: "Build awesome app".to_string(), completed: false },
            TodoItem { text: "Try reactive patterns".to_string(), completed: true },
        ]);
        let filter = signal(Filter::All);
        let input = signal(String::new());
        let selected = signal(0);

        // Computed: filtered items
        let items_clone = items.clone();
        let filter_clone = filter.clone();
        let filtered_items = computed(move || {
            items_clone.with(|items| {
                filter_clone.with(|filter| {
                    items.iter()
                        .filter(|item| filter.matches(item.completed))
                        .cloned()
                        .collect()
                })
            })
        });

        // Computed: counts
        let items_clone2 = items.clone();
        let active_count = computed(move || {
            items_clone2.with(|items| {
                items.iter().filter(|item| !item.completed).count()
            })
        });

        let items_clone3 = items.clone();
        let completed_count = computed(move || {
            items_clone3.with(|items| {
                items.iter().filter(|item| item.completed).count()
            })
        });

        let items_clone4 = items.clone();
        let total_count = computed(move || items_clone4.with(|items| items.len()));

        // Effect: log when items change (optional)
        let items_clone5 = items.clone();
        effect(move || {
            let count = items_clone5.with(|items| items.len());
            println!("Total items: {}", count);
        });

        Self {
            items,
            filter,
            input,
            selected,
            filtered_items,
            active_count,
            completed_count,
            total_count,
        }
    }

    fn add_item(&mut self) {
        let text = self.input.get();
        if !text.is_empty() {
            self.items.update(|items| {
                items.push(TodoItem {
                    text: text.clone(),
                    completed: false,
                });
            });
            self.input.set(String::new());
            self.selected.set(0);
        }
    }

    fn toggle_selected(&mut self) {
        let idx = self.selected.get();
        let filter = self.filter.get();

        self.items.update(|items| {
            // Find actual index in full list based on filtered view
            let filtered_indices: Vec<_> = items.iter()
                .enumerate()
                .filter(|(_, item)| filter.matches(item.completed))
                .map(|(i, _)| i)
                .collect();

            if let Some(&actual_idx) = filtered_indices.get(idx) {
                if let Some(item) = items.get_mut(actual_idx) {
                    item.completed = !item.completed;
                }
            }
        });
    }

    fn delete_selected(&mut self) {
        let idx = self.selected.get();
        let filter = self.filter.get();

        self.items.update(|items| {
            let filtered: Vec<_> = items.iter()
                .enumerate()
                .filter(|(_, item)| filter.matches(item.completed))
                .collect();

            if let Some((actual_idx, _)) = filtered.get(idx) {
                items.remove(*actual_idx);
            }
        });

        // Adjust selection
        let filtered_len = self.filtered_items.get().len();
        if filtered_len > 0 {
            let current = self.selected.get();
            if current >= filtered_len {
                self.selected.set(filtered_len - 1);
            }
        } else {
            self.selected.set(0);
        }
    }

    fn cycle_filter(&mut self) {
        self.filter.update(|f| {
            *f = match f {
                Filter::All => Filter::Active,
                Filter::Active => Filter::Completed,
                Filter::Completed => Filter::All,
            };
        });
        self.selected.set(0);
    }

    fn handle_key(&mut self, key: &Key) -> bool {
        match key {
            Key::Enter => {
                self.add_item();
                true
            }
            Key::Char(' ') => {
                self.toggle_selected();
                true
            }
            Key::Char('d') | Key::Delete => {
                self.delete_selected();
                true
            }
            Key::Char('f') => {
                self.cycle_filter();
                true
            }
            Key::Up | Key::Char('k') => {
                self.selected.update(|s| *s = s.saturating_sub(1));
                true
            }
            Key::Down | Key::Char('j') => {
                let max = self.filtered_items.get().len().saturating_sub(1);
                self.selected.update(|s| *s = (*s + 1).min(max));
                true
            }
            Key::Char(c) => {
                self.input.update(|input| input.push(*c));
                true
            }
            Key::Backspace => {
                self.input.update(|input| { input.pop(); });
                true
            }
            _ => false,
        }
    }
}

impl View for ReactiveTodoList {
    fn render(&self, ctx: &mut RenderContext) {
        // Get reactive values - all cached and auto-updated!
        let filtered = self.filtered_items.get();
        let active_count = self.active_count.get();
        let completed_count = self.completed_count.get();
        let total_count = self.total_count.get();
        let filter = self.filter.get();
        let input = self.input.get();
        let selected = self.selected.get();

        // Build UI
        let mut items_view = vstack();
        for (i, item) in filtered.iter().enumerate() {
            let is_selected = i == selected;
            let prefix = if is_selected { "→ " } else { "  " };
            let checkbox = if item.completed { "[✓]" } else { "[ ]" };
            let text = format!("{}{} {}", prefix, checkbox, item.text);

            let color = if is_selected {
                Color::CYAN
            } else if item.completed {
                Color::rgb(100, 100, 100)
            } else {
                Color::WHITE
            };

            items_view = items_view.child(Text::new(text).fg(color));
        }

        if filtered.is_empty() {
            items_view = items_view.child(Text::muted("No items to show"));
        }

        let view = vstack()
            .gap(1)
            .child(
                Border::panel()
                    .title("📝 Reactive Todo List")
                    .child(
                        vstack()
                            .gap(1)
                            .child(
                                hstack()
                                    .gap(2)
                                    .child(Text::new("New:"))
                                    .child(Text::new(format!("[{}]", input)).fg(Color::YELLOW))
                            )
                            .child(
                                hstack()
                                    .gap(2)
                                    .child(Text::new(format!("Filter: {}", filter.label())).fg(Color::CYAN))
                                    .child(Text::muted("|"))
                                    .child(Text::new(format!("Total: {}", total_count)))
                                    .child(Text::new(format!("Active: {}", active_count)).fg(Color::GREEN))
                                    .child(Text::new(format!("Done: {}", completed_count)).fg(Color::rgb(100, 100, 100)))
                            )
                    )
            )
            .child(
                Border::single()
                    .title("Items")
                    .child(items_view)
            )
            .child(
                Border::rounded()
                    .title("Controls")
                    .child(
                        vstack()
                            .child(hstack().gap(2).child(Text::muted("[Type]")).child(Text::new("Add text to new item")))
                            .child(hstack().gap(2).child(Text::muted("[Enter]")).child(Text::new("Add item")))
                            .child(hstack().gap(2).child(Text::muted("[↑/↓]")).child(Text::new("Navigate")))
                            .child(hstack().gap(2).child(Text::muted("[Space]")).child(Text::new("Toggle completed")))
                            .child(hstack().gap(2).child(Text::muted("[d]")).child(Text::new("Delete item")))
                            .child(hstack().gap(2).child(Text::muted("[f]")).child(Text::new("Cycle filter")))
                            .child(hstack().gap(2).child(Text::muted("[q]")).child(Text::new("Quit")))
                    )
            )
            .child(
                Border::success_box()
                    .title("✨ Reactive Features")
                    .child(
                        vstack()
                            .child(Text::success("✓ filtered_items auto-updates when filter or items change"))
                            .child(Text::success("✓ Counts are computed - no manual recalculation"))
                            .child(Text::success("✓ All derived state is cached and efficient"))
                    )
            );

        view.render(ctx);
    }

    fn meta(&self) -> WidgetMeta {
        WidgetMeta::new("ReactiveTodoList")
    }
}

fn main() -> Result<()> {
    println!("📝 Reactive Todo List Example");
    println!("Demonstrates Signal, Computed, and derived state.\n");

    let mut app = App::builder().build();
    let todo = ReactiveTodoList::new();

    app.run(todo, |event, todo, _app| {
        match event {
            Event::Key(key_event) => todo.handle_key(&key_event.key),
            _ => false,
        }
    })
}
