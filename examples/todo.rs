//! Reactive Todo List example
//!
//! Demonstrates advanced reactive patterns:
//! - Multiple signals
//! - Computed derived state
//! - Complex state transformations
//!
//! Run with: cargo run --example todo

use revue::prelude::*;
use revue::utils::unicode::display_width;

/// Width of the key column in the Controls list (`[Space]`, `[Enter]`).
const KEY_COLS: u16 = 7;

/// Columns `text` occupies, for sizing single-line text in an `hstack`.
fn cols(text: &str) -> u16 {
    display_width(text) as u16
}

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
            TodoItem {
                text: "Learn Revue TUI".to_string(),
                completed: false,
            },
            TodoItem {
                text: "Build awesome app".to_string(),
                completed: false,
            },
            TodoItem {
                text: "Try reactive patterns".to_string(),
                completed: true,
            },
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
                    items
                        .iter()
                        .filter(|item| filter.matches(item.completed))
                        .cloned()
                        .collect()
                })
            })
        });

        // Computed: counts
        let items_clone2 = items.clone();
        let active_count = computed(move || {
            items_clone2.with(|items| items.iter().filter(|item| !item.completed).count())
        });

        let items_clone3 = items.clone();
        let completed_count = computed(move || {
            items_clone3.with(|items| items.iter().filter(|item| item.completed).count())
        });

        let items_clone4 = items.clone();
        let total_count = computed(move || items_clone4.with(|items| items.len()));

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
            // Select the new item: it is appended, so it is the last row of
            // any filter that shows active items.
            if self.filter.get().matches(false) {
                self.selected
                    .set(self.filtered_items.get().len().saturating_sub(1));
            } else {
                self.clamp_selection();
            }
        }
    }

    /// Keep `selected` inside the filtered list after it shrinks.
    fn clamp_selection(&mut self) {
        let last = self.filtered_items.get().len().saturating_sub(1);
        if self.selected.get() > last {
            self.selected.set(last);
        }
    }

    fn toggle_selected(&mut self) {
        let idx = self.selected.get();
        let filter = self.filter.get();

        self.items.update(|items| {
            // Find actual index in full list based on filtered view
            let filtered_indices: Vec<_> = items
                .iter()
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

        // Under Active/Completed the toggled item leaves the view.
        self.clamp_selection();
    }

    fn delete_selected(&mut self) {
        let idx = self.selected.get();
        let filter = self.filter.get();

        self.items.update(|items| {
            let filtered: Vec<_> = items
                .iter()
                .enumerate()
                .filter(|(_, item)| filter.matches(item.completed))
                .collect();

            if let Some((actual_idx, _)) = filtered.get(idx) {
                items.remove(*actual_idx);
            }
        });

        self.clamp_selection();
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
                self.input.update(|input| {
                    input.pop();
                });
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

            items_view = items_view.child_sized(Text::new(text).fg(color), 1);
        }

        if filtered.is_empty() {
            items_view = items_view.child_sized(Text::muted("No items to show"), 1);
        }

        let filter_text = format!("Filter: {}", filter.label());
        let total_text = format!("Total: {}", total_count);
        let active_text = format!("Active: {}", active_count);

        // Every fixed-height row/box is `child_sized`; the Items box takes the
        // rest with `child_flex` - a stack would otherwise size it to its content.
        let view = vstack()
            .gap(1)
            .child_sized(
                Border::panel().title("📝 Reactive Todo List").child(
                    vstack()
                        .gap(1)
                        .child_sized(
                            hstack()
                                .gap(2)
                                .child_sized(Text::new("New:"), cols("New:"))
                                .child(Text::new(format!("[{}]", input)).fg(Color::YELLOW)),
                            1,
                        )
                        .child_sized(
                            hstack()
                                .gap(2)
                                .child_sized(
                                    Text::new(&filter_text).fg(Color::CYAN),
                                    cols(&filter_text),
                                )
                                .child_sized(Text::muted("|"), 1)
                                .child_sized(Text::new(&total_text), cols(&total_text))
                                .child_sized(
                                    Text::new(&active_text).fg(Color::GREEN),
                                    cols(&active_text),
                                )
                                .child(
                                    Text::new(format!("Done: {}", completed_count))
                                        .fg(Color::rgb(100, 100, 100)),
                                ),
                            1,
                        ),
                ),
                5,
            )
            .child_flex(Border::single().title("Items").child(items_view), 1.0)
            .child_sized(
                Border::rounded().title("Controls").child(
                    vstack()
                        .child_sized(
                            hstack()
                                .gap(2)
                                .child_sized(Text::muted("[Type]"), KEY_COLS)
                                .child(Text::new("Add text to new item")),
                            1,
                        )
                        .child_sized(
                            hstack()
                                .gap(2)
                                .child_sized(Text::muted("[Enter]"), KEY_COLS)
                                .child(Text::new("Add item")),
                            1,
                        )
                        .child_sized(
                            hstack()
                                .gap(2)
                                .child_sized(Text::muted("[↑/↓]"), KEY_COLS)
                                .child(Text::new("Navigate")),
                            1,
                        )
                        .child_sized(
                            hstack()
                                .gap(2)
                                .child_sized(Text::muted("[Space]"), KEY_COLS)
                                .child(Text::new("Toggle completed")),
                            1,
                        )
                        .child_sized(
                            hstack()
                                .gap(2)
                                .child_sized(Text::muted("[d]"), KEY_COLS)
                                .child(Text::new("Delete item")),
                            1,
                        )
                        .child_sized(
                            hstack()
                                .gap(2)
                                .child_sized(Text::muted("[f]"), KEY_COLS)
                                .child(Text::new("Cycle filter")),
                            1,
                        )
                        .child_sized(
                            hstack()
                                .gap(2)
                                .child_sized(Text::muted("[q]"), KEY_COLS)
                                .child(Text::new("Quit")),
                            1,
                        ),
                ),
                9,
            )
            .child_sized(
                Border::success_box().title("✨ Reactive Features").child(
                    vstack()
                        .child_sized(
                            Text::success(
                                "✓ filtered_items auto-updates when filter or items change",
                            ),
                            1,
                        )
                        .child_sized(
                            Text::success("✓ Counts are computed - no manual recalculation"),
                            1,
                        )
                        .child_sized(
                            Text::success("✓ All derived state is cached and efficient"),
                            1,
                        ),
                ),
                5,
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

    app.run(todo, |event, todo, _app| match event {
        Event::Key(key_event) => todo.handle_key(&key_event.key),
        _ => false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn selected_text(todo: &ReactiveTodoList) -> Option<String> {
        todo.filtered_items
            .get()
            .get(todo.selected.get())
            .map(|item| item.text.clone())
    }

    #[test]
    fn toggling_the_last_visible_item_keeps_selection_in_range() {
        let mut todo = ReactiveTodoList::new();
        todo.cycle_filter(); // Active: "Learn Revue TUI", "Build awesome app"
        todo.handle_key(&Key::Down);
        todo.toggle_selected(); // "Build awesome app" leaves the Active view

        assert_eq!(selected_text(&todo).as_deref(), Some("Learn Revue TUI"));
    }

    #[test]
    fn deleting_the_last_item_selects_the_new_last_item() {
        let mut todo = ReactiveTodoList::new();
        todo.handle_key(&Key::Down);
        todo.handle_key(&Key::Down);
        todo.delete_selected();

        assert_eq!(selected_text(&todo).as_deref(), Some("Build awesome app"));
    }

    #[test]
    fn emptying_a_filter_resets_selection() {
        let mut todo = ReactiveTodoList::new();
        todo.cycle_filter(); // Active: two items
        todo.handle_key(&Key::Down);
        todo.toggle_selected();
        todo.toggle_selected(); // nothing active is left

        assert!(todo.filtered_items.get().is_empty());
        assert_eq!(todo.selected.get(), 0);
    }

    #[test]
    fn adding_an_item_selects_it() {
        let mut todo = ReactiveTodoList::new();
        todo.input.set("Write tests".to_string());
        todo.add_item();

        assert_eq!(selected_text(&todo).as_deref(), Some("Write tests"));
    }
}
