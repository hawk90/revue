//! Todo view

use revue::prelude::*;

#[derive(Clone)]
pub struct TodoItem {
    text: String,
    completed: bool,
}

impl TodoItem {
    fn new(text: impl Into<String>, completed: bool) -> Self {
        Self {
            text: text.into(),
            completed,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Filter {
    All,
    Active,
    Completed,
}

impl Filter {
    fn matches(self, item: &TodoItem) -> bool {
        match self {
            Filter::All => true,
            Filter::Active => !item.completed,
            Filter::Completed => item.completed,
        }
    }

    fn next(self) -> Self {
        match self {
            Filter::All => Filter::Active,
            Filter::Active => Filter::Completed,
            Filter::Completed => Filter::All,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Filter::All => "All",
            Filter::Active => "Active",
            Filter::Completed => "Completed",
        }
    }
}

pub struct TodoApp {
    todos: Signal<Vec<TodoItem>>,
    filter: Signal<Filter>,
    /// Position of the selected row among the visible (filtered) todos.
    selected: Signal<usize>,
    input: Input,
    /// Whether keys go to the input box rather than the list.
    editing: bool,
}

impl TodoApp {
    pub fn new() -> Self {
        Self {
            todos: signal(vec![
                TodoItem::new("Learn Revue", true),
                TodoItem::new("Build awesome TUI", false),
                TodoItem::new("Ship it!", false),
            ]),
            filter: signal(Filter::All),
            selected: signal(0),
            input: Input::new().placeholder("What needs to be done?"),
            editing: false,
        }
    }

    /// Whether keys are going to the input box (so `q` is text, not quit).
    pub fn is_editing(&self) -> bool {
        self.editing
    }

    /// Handles a key press and returns whether the view changed.
    pub fn handle_key(&mut self, key: &Key) -> bool {
        if self.editing {
            match key {
                Key::Enter => self.add_todo(),
                Key::Escape | Key::Tab => self.editing = false,
                _ => return self.input.handle_key(key),
            }
            return true;
        }

        match key {
            Key::Char('a') | Key::Char('i') | Key::Tab => self.editing = true,
            Key::Up | Key::Char('k') => self.selected.update(|s| *s = s.saturating_sub(1)),
            Key::Down | Key::Char('j') => {
                let last = self.visible_indices().len().saturating_sub(1);
                self.selected.update(|s| *s = (*s + 1).min(last));
            }
            Key::Char(' ') | Key::Enter => self.toggle_selected(),
            Key::Char('d') | Key::Delete => self.delete_selected(),
            Key::Char('f') => {
                self.filter.update(|f| *f = f.next());
                self.selected.set(0);
            }
            _ => return false,
        }
        true
    }

    fn add_todo(&mut self) {
        let text = self.input.text().trim().to_string();
        if text.is_empty() {
            return;
        }
        self.todos
            .update(|todos| todos.push(TodoItem::new(text, false)));
        self.input.clear();
        // The new todo is active, so it shows unless the filter is Completed.
        if self.filter.get() != Filter::Completed {
            let last = self.visible_indices().len().saturating_sub(1);
            self.selected.set(last);
        }
    }

    fn toggle_selected(&mut self) {
        if let Some(index) = self.selected_index() {
            self.todos
                .update(|todos| todos[index].completed = !todos[index].completed);
            // Under Active or Completed the todo just left the view.
            self.clamp_selection();
        }
    }

    fn delete_selected(&mut self) {
        if let Some(index) = self.selected_index() {
            self.todos.update(|todos| {
                todos.remove(index);
            });
            self.clamp_selection();
        }
    }

    /// Indices into `todos` of the todos the filter shows.
    fn visible_indices(&self) -> Vec<usize> {
        let filter = self.filter.get();
        self.todos.with(|todos| {
            (0..todos.len())
                .filter(|&i| filter.matches(&todos[i]))
                .collect()
        })
    }

    fn selected_index(&self) -> Option<usize> {
        self.visible_indices().get(self.selected.get()).copied()
    }

    fn clamp_selection(&mut self) {
        let last = self.visible_indices().len().saturating_sub(1);
        self.selected.update(|s| *s = (*s).min(last));
    }
}

impl Default for TodoApp {
    fn default() -> Self {
        Self::new()
    }
}

impl View for TodoApp {
    fn render(&self, ctx: &mut RenderContext) {
        let todos = self.todos.get();
        let visible = self.visible_indices();
        let selected = self.selected.get();
        let filter = self.filter.get();
        let left = todos.iter().filter(|t| !t.completed).count();

        // Every row is one line high: `child_sized(.., 1)`. Each box is its
        // content plus 2 lines of border.
        let mut list = vstack();
        for (row, &index) in visible.iter().enumerate() {
            let todo = &todos[index];
            let marker = if row == selected && !self.editing {
                "> "
            } else {
                "  "
            };
            list = list.child_sized(
                hstack()
                    .child_sized(Text::new(marker).class("marker"), 2)
                    .child(Checkbox::new(&todo.text).checked(todo.completed)),
                1,
            );
        }
        if visible.is_empty() {
            list = list.child_sized(Text::muted("  Nothing here"), 1);
        }
        let list_rows = visible.len().max(1) as u16;

        let input_box = Border::rounded()
            .title(" New todo ")
            .class(if self.editing { "editing" } else { "idle" })
            .child(self.input.clone().focused(self.editing));

        let list_box = Border::rounded()
            .title(format!(" {} ", filter.label()))
            .child(list);

        let help = if self.editing {
            "[Enter] add  [Esc] done"
        } else {
            "[a] add  [j/k] move  [Space] toggle  [d] delete  [f] filter  [q] quit"
        };

        vstack()
            .gap(1)
            .child_sized(Text::new("Todo App").class("title"), 1)
            .child_sized(input_box, 3)
            .child_sized(list_box, list_rows + 2)
            .child_sized(
                Text::new(format!(
                    "{left} item{} left",
                    if left == 1 { "" } else { "s" }
                )),
                1,
            )
            .child_sized(Text::muted(help), 1)
            .render(ctx);
    }

    fn meta(&self) -> WidgetMeta {
        WidgetMeta::new("TodoApp")
    }
}
