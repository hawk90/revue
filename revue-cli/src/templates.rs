//! Project templates and theme definitions

/// The `revue` version requirement written into generated projects.
///
/// The templates are written against this release line. A test checks it
/// against the version of the `revue` crate in this repository, so a major
/// release that leaves the templates behind fails CI.
pub const REVUE_VERSION: &str = "3.0";

/// Generate Cargo.toml
pub fn cargo_toml(name: &str) -> String {
    format!(
        r#"[package]
name = "{name}"
version = "0.1.0"
edition = "2021"

[dependencies]
revue = "{REVUE_VERSION}"
"#
    )
}

/// Generate .gitignore
///
/// `Cargo.lock` is not ignored: a generated project is an application, and
/// an application commits its lock file.
pub fn gitignore() -> &'static str {
    r#"/target
*.swp
*.swo
.DS_Store
"#
}

/// The templates `revue new --template` accepts.
pub const PROJECT_TEMPLATES: &[&str] = &["basic", "dashboard", "todo", "chat"];

/// The source files `revue new` writes for one template.
pub struct ProjectFiles {
    /// `src/main.rs`
    pub main_rs: &'static str,
    /// `src/app.rs`
    pub app_rs: &'static str,
    /// `styles/main.css`, which `main.rs` loads
    pub style_css: &'static str,
}

/// The files of the template called `template`, if there is one.
pub fn project_files(template: &str) -> Option<ProjectFiles> {
    let (main_rs, app_rs, style_css) = match template {
        "basic" => (basic_main(), basic_app(), basic_style()),
        "dashboard" => (dashboard_main(), dashboard_app(), dashboard_style()),
        "todo" => (todo_main(), todo_app(), todo_style()),
        "chat" => (chat_main(), chat_app(), chat_style()),
        _ => return None,
    };
    Some(ProjectFiles {
        main_rs,
        app_rs,
        style_css,
    })
}

// =============================================================================
// Basic Template: a counter
// =============================================================================

/// `src/main.rs` of the `basic` template
pub fn basic_main() -> &'static str {
    r#"//! Basic Revue application

mod app;

use app::Counter;
use revue::prelude::*;

fn main() -> Result<()> {
    // The path is relative to the working directory: start the app with
    // `cargo run` from the project root.
    let mut app = App::builder().style("styles/main.css").build();

    app.run(Counter::new(), |event, counter, app| {
        let Event::Key(KeyEvent { key, .. }) = event else {
            return false;
        };
        if let Key::Char('q') | Key::Escape = key {
            // Stop the loop so `run` restores the terminal on the way out.
            app.quit();
            return false;
        }
        // Returning `true` asks for a redraw.
        counter.handle_key(key)
    })
}
"#
}

/// `src/app.rs` of the `basic` template
pub fn basic_app() -> &'static str {
    r#"//! The application's root view

use revue::prelude::*;

/// A counter: state in a `Signal`, a view that renders it, and key handling.
pub struct Counter {
    count: Signal<i32>,
}

impl Counter {
    pub fn new() -> Self {
        Self { count: signal(0) }
    }

    /// Handles a key press and returns whether the view changed.
    pub fn handle_key(&mut self, key: &Key) -> bool {
        match key {
            Key::Char('+') | Key::Char('=') | Key::Up => self.count.update(|n| *n += 1),
            Key::Char('-') | Key::Down => self.count.update(|n| *n -= 1),
            Key::Char('r') => self.count.set(0),
            _ => return false,
        }
        true
    }
}

impl Default for Counter {
    fn default() -> Self {
        Self::new()
    }
}

impl View for Counter {
    fn render(&self, ctx: &mut RenderContext) {
        Border::rounded()
            .title(" Revue App ")
            .child(
                vstack()
                    .gap(1)
                    .child(Text::new("Hello from Revue!").class("title"))
                    .child(Text::new(format!("Count: {}", self.count.get())).class("count"))
                    .child(Text::muted(
                        "[+] increment  [-] decrement  [r] reset  [q] quit",
                    )),
            )
            .render(ctx);
    }

    fn meta(&self) -> WidgetMeta {
        WidgetMeta::new("Counter")
    }
}
"#
}

/// `styles/main.css` of the `basic` template
pub fn basic_style() -> &'static str {
    r#"/* Starter styles: loaded by `.style("styles/main.css")` in main.rs */

:root {
    --fg: #c0caf5;
    --accent: #7aa2f7;
}

/* The Border is the view's own body, so it is styled through the view's type.
   A widget added with `.child(...)` gets its own node and can use a class. */
Counter {
    border-color: var(--accent);
}

.title {
    color: var(--accent);
    font-weight: bold;
}

.count {
    color: var(--fg);
    font-weight: bold;
}
"#
}

// =============================================================================
// Dashboard Template: simulated system metrics in gauges and a sparkline
// =============================================================================

/// `src/main.rs` of the `dashboard` template
pub fn dashboard_main() -> &'static str {
    r#"//! Dashboard application

mod app;

use app::Dashboard;
use revue::prelude::*;

fn main() -> Result<()> {
    // The path is relative to the working directory: start the app with
    // `cargo run` from the project root.
    let mut app = App::builder().style("styles/main.css").build();

    app.run(Dashboard::new(), |event, dashboard, app| match event {
        Event::Key(KeyEvent {
            key: Key::Char('q') | Key::Escape,
            ..
        }) => {
            // Stop the loop so `run` restores the terminal on the way out.
            app.quit();
            false
        }
        // Returning `true` asks for a redraw.
        Event::Tick => dashboard.on_tick(),
        _ => false,
    })
}
"#
}

/// `src/app.rs` of the `dashboard` template
pub fn dashboard_app() -> &'static str {
    r#"//! Dashboard view

use revue::prelude::*;
use std::time::{Duration, Instant};

/// How often the simulated metrics change.
const UPDATE_INTERVAL: Duration = Duration::from_secs(1);

/// How many request samples the sparkline keeps: more than a terminal is
/// wide, so the line is always full.
const HISTORY: usize = 300;

/// A gauge at or above this percentage counts as an alert.
const ALERT_AT: f64 = 70.0;

pub struct Dashboard {
    cpu_usage: Signal<f64>,
    memory_usage: Signal<f64>,
    disk_usage: Signal<f64>,
    requests: Signal<Vec<u64>>,
    updates: Signal<u64>,
    last_update: Instant,
}

impl Dashboard {
    pub fn new() -> Self {
        Self {
            cpu_usage: signal(45.2),
            memory_usage: signal(62.8),
            disk_usage: signal(34.5),
            requests: signal((0..HISTORY).map(|i| requests_at(i as f64)).collect()),
            updates: signal(0),
            last_update: Instant::now(),
        }
    }

    /// Called on every `Event::Tick`. Advances the simulated metrics once per
    /// `UPDATE_INTERVAL` and returns whether the view must redraw.
    pub fn on_tick(&mut self) -> bool {
        if self.last_update.elapsed() < UPDATE_INTERVAL {
            return false;
        }
        self.last_update = Instant::now();

        self.updates.update(|n| *n += 1);
        let t = self.updates.get() as f64;
        self.cpu_usage.set(50.0 + (t * 0.4).sin() * 30.0);
        self.memory_usage.set(62.0 + (t * 0.15).cos() * 10.0);
        self.disk_usage.set((34.5 + t * 0.05).min(100.0));
        self.requests.update(|samples| {
            samples.remove(0);
            samples.push(requests_at(HISTORY as f64 + t));
        });
        true
    }

    fn gauge_panel(title: &str, percent: f64) -> Border {
        Border::rounded().title(format!(" {title} ")).child(
            Gauge::new()
                .percent(percent)
                // A gauge draws at most `width` cells; this fills its panel.
                .width(u16::MAX)
                .thresholds(ALERT_AT / 100.0, 0.9),
        )
    }
}

/// Simulated requests per second at sample `t`.
fn requests_at(t: f64) -> u64 {
    (150.0 + (t * 0.3).sin() * 40.0 + (t * 1.7).cos() * 15.0) as u64
}

impl Default for Dashboard {
    fn default() -> Self {
        Self::new()
    }
}

impl View for Dashboard {
    fn render(&self, ctx: &mut RenderContext) {
        let usage = [
            ("CPU", self.cpu_usage.get()),
            ("Memory", self.memory_usage.get()),
            ("Disk", self.disk_usage.get()),
        ];
        let alerts = usage.iter().filter(|(_, p)| *p >= ALERT_AT).count();
        let requests = self.requests.get();
        let latest = requests.last().copied().unwrap_or_default();

        // A row of `child_flex` children has no content height of its own,
        // so each row gets a fixed height with `child_sized`: its content
        // lines plus 2 for the border.
        let mut gauges = hstack().gap(1);
        for (title, percent) in usage {
            gauges = gauges.child_flex(Self::gauge_panel(title, percent), 1.0);
        }

        let requests_panel = Border::rounded()
            .title(format!(" Requests/s: {latest} "))
            .child(Sparkline::new(requests.iter().map(|&r| r as f64)));

        let alert_badge = match alerts {
            0 => Badge::new("No alerts").info(),
            1 => Badge::new("1 alert").warning(),
            n => Badge::new(format!("{n} alerts")).warning(),
        };
        let status = hstack()
            .gap(1)
            .child_sized(Badge::new("Online").success(), 8)
            .child_sized(alert_badge, 11)
            .child(Text::muted(format!(
                "Updates: {}  [q] quit",
                self.updates.get()
            )));

        vstack()
            .gap(1)
            .child_sized(Text::new("System Dashboard").class("title"), 1)
            .child_sized(gauges, 3)
            .child_sized(requests_panel, 3)
            .child_sized(status, 1)
            .render(ctx);
    }

    fn meta(&self) -> WidgetMeta {
        WidgetMeta::new("Dashboard")
    }
}
"#
}

/// `styles/main.css` of the `dashboard` template
pub fn dashboard_style() -> &'static str {
    r#"/* Dashboard styles: loaded by `.style("styles/main.css")` in main.rs */

:root {
    --muted: #565f89;
    --accent: #7aa2f7;
    --success: #9ece6a;
}

/* The panels. */
Border {
    border-color: var(--muted);
}

Sparkline {
    color: var(--success);
}

.title {
    color: var(--accent);
    font-weight: bold;
}
"#
}

// =============================================================================
// Todo Template: a todo list with an input box and filters
// =============================================================================

/// `src/main.rs` of the `todo` template
pub fn todo_main() -> &'static str {
    r#"//! Todo application

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
"#
}

/// `src/app.rs` of the `todo` template
pub fn todo_app() -> &'static str {
    r#"//! Todo view

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
"#
}

/// `styles/main.css` of the `todo` template
pub fn todo_style() -> &'static str {
    r#"/* Todo styles: loaded by `.style("styles/main.css")` in main.rs */

:root {
    --muted: #565f89;
    --accent: #7aa2f7;
}

Border {
    border-color: var(--muted);
}

/* The input box while you type in it. */
.editing {
    border-color: var(--accent);
}

.title {
    color: var(--accent);
    font-weight: bold;
}

.marker {
    color: var(--accent);
    font-weight: bold;
}
"#
}

// =============================================================================
// Chat Template: a chat room with a user list and a message input
// =============================================================================

/// `src/main.rs` of the `chat` template
pub fn chat_main() -> &'static str {
    r#"//! Chat application

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
"#
}

/// `src/app.rs` of the `chat` template
pub fn chat_app() -> &'static str {
    r#"//! Chat view

use revue::prelude::*;

/// Width of the users sidebar, borders included.
const SIDEBAR_WIDTH: u16 = 20;

#[derive(Clone)]
pub struct Message {
    user: String,
    text: String,
    timestamp: String,
    is_self: bool,
}

impl Message {
    fn new(user: &str, text: &str, timestamp: &str, is_self: bool) -> Self {
        Self {
            user: user.into(),
            text: text.into(),
            timestamp: timestamp.into(),
            is_self,
        }
    }
}

#[derive(Clone)]
pub struct User {
    name: String,
    status: UserStatus,
}

#[derive(Clone, Copy)]
pub enum UserStatus {
    Online,
    Away,
    Offline,
}

pub struct ChatApp {
    messages: Signal<Vec<Message>>,
    users: Vec<User>,
    input: Input,
    current_user: String,
}

impl ChatApp {
    pub fn new() -> Self {
        let user = |name: &str, status| User {
            name: name.into(),
            status,
        };
        Self {
            messages: signal(vec![
                Message::new("Alice", "Hey everyone!", "10:30", false),
                Message::new("Bob", "Hi Alice! How's it going?", "10:31", false),
                Message::new(
                    "You",
                    "Hello! Just built this chat with Revue!",
                    "10:32",
                    true,
                ),
            ]),
            users: vec![
                user("Alice", UserStatus::Online),
                user("Bob", UserStatus::Online),
                user("Charlie", UserStatus::Away),
                user("Diana", UserStatus::Offline),
            ],
            input: Input::new().placeholder("Type a message...").focused(true),
            current_user: "You".into(),
        }
    }

    /// Handles a key press and returns whether the view changed.
    pub fn handle_key(&mut self, key: &Key) -> bool {
        match key {
            Key::Enter => {
                self.send_message();
                true
            }
            _ => self.input.handle_key(key),
        }
    }

    fn send_message(&mut self) {
        let text = self.input.text().trim().to_string();
        if text.is_empty() {
            return;
        }
        let message = Message::new(&self.current_user, &text, "now", true);
        self.messages.update(|messages| messages.push(message));
        self.input.clear();
    }

    fn render_user(user: &User) -> Stack {
        let avatar = match user.status {
            UserStatus::Online => Avatar::new(&user.name).online(),
            UserStatus::Away => Avatar::new(&user.name).away(),
            UserStatus::Offline => Avatar::new(&user.name).offline(),
        };
        // A small avatar is the initial plus a status dot: 2 columns.
        hstack()
            .gap(1)
            .child_sized(avatar.small(), 2)
            .child(Text::new(&user.name))
    }

    fn render_message(message: &Message) -> Stack {
        let name = format!("{}:", message.user);
        let name_width = name.chars().count() as u16;
        hstack()
            .gap(1)
            .child_sized(Text::new(&message.timestamp).class("timestamp"), 5)
            .child_sized(
                Text::new(name).class(if message.is_self { "self" } else { "username" }),
                name_width,
            )
            .child(Text::new(&message.text))
    }
}

impl Default for ChatApp {
    fn default() -> Self {
        Self::new()
    }
}

impl View for ChatApp {
    fn render(&self, ctx: &mut RenderContext) {
        let messages = self.messages.get();

        // Show the newest messages that fit: the screen minus the help line,
        // the input box (3 lines) and the message panel's border (2 lines).
        let fit = ctx.area.height.saturating_sub(1 + 3 + 2) as usize;
        let mut message_list = vstack();
        for message in messages.iter().skip(messages.len().saturating_sub(fit)) {
            message_list = message_list.child_sized(Self::render_message(message), 1);
        }

        let mut user_list = vstack();
        for user in &self.users {
            user_list = user_list.child_sized(Self::render_user(user), 1);
        }

        // Fixed-size pieces use `child_sized`; the sidebar and the message
        // panel take the rest of the screen with `child_flex`.
        let sidebar = Border::rounded().title(" Users ").child(user_list);
        let chat = vstack()
            .child_flex(
                Border::rounded().title(" # general ").child(message_list),
                1.0,
            )
            .child_sized(
                Border::rounded()
                    .class("composer")
                    .child(self.input.clone()),
                3,
            );

        vstack()
            .child_flex(
                hstack()
                    .gap(1)
                    .child_sized(sidebar, SIDEBAR_WIDTH)
                    .child_flex(chat, 1.0),
                1.0,
            )
            .child_sized(Text::muted("[Enter] send  [Esc] quit"), 1)
            .render(ctx);
    }

    fn meta(&self) -> WidgetMeta {
        WidgetMeta::new("ChatApp")
    }
}
"#
}

/// `styles/main.css` of the `chat` template
pub fn chat_style() -> &'static str {
    r#"/* Chat styles: loaded by `.style("styles/main.css")` in main.rs */

:root {
    --muted: #565f89;
    --accent: #7aa2f7;
    --other: #e0af68;
}

Border {
    border-color: var(--muted);
}

/* The box around the message input. */
.composer {
    border-color: var(--accent);
}

.timestamp {
    color: var(--muted);
}

.username {
    color: var(--other);
    font-weight: bold;
}

/* Your own name on the messages you sent. */
.self {
    color: var(--accent);
    font-weight: bold;
}
"#
}

// =============================================================================
// Theme Definitions
// =============================================================================

pub fn theme_dracula() -> &'static str {
    r#"/* Dracula Theme for Revue */
/* https://draculatheme.com */

:root {
    --bg-primary: #282a36;
    --bg-secondary: #44475a;
    --fg-primary: #f8f8f2;
    --fg-secondary: #6272a4;
    --accent: #bd93f9;
    --success: #50fa7b;
    --warning: #ffb86c;
    --error: #ff5555;
    --cyan: #8be9fd;
    --pink: #ff79c6;
    --yellow: #f1fa8c;
}

* {
    color: var(--fg-primary);
    background: var(--bg-primary);
}

.title {
    color: var(--accent);
}

.button {
    background: var(--accent);
    color: var(--bg-primary);
}

.button:hover {
    background: var(--pink);
}

.input {
    background: var(--bg-secondary);
    border-color: var(--fg-secondary);
}

.input:focus {
    border-color: var(--accent);
}

.list-item:selected {
    background: var(--accent);
    color: var(--bg-primary);
}

.success { color: var(--success); }
.warning { color: var(--warning); }
.error { color: var(--error); }
"#
}

pub fn theme_nord() -> &'static str {
    r#"/* Nord Theme for Revue */
/* https://www.nordtheme.com */

:root {
    --polar-night-0: #2e3440;
    --polar-night-1: #3b4252;
    --polar-night-2: #434c5e;
    --polar-night-3: #4c566a;
    --snow-storm-0: #d8dee9;
    --snow-storm-1: #e5e9f0;
    --snow-storm-2: #eceff4;
    --frost-0: #8fbcbb;
    --frost-1: #88c0d0;
    --frost-2: #81a1c1;
    --frost-3: #5e81ac;
    --aurora-red: #bf616a;
    --aurora-orange: #d08770;
    --aurora-yellow: #ebcb8b;
    --aurora-green: #a3be8c;
    --aurora-purple: #b48ead;

    --bg-primary: var(--polar-night-0);
    --bg-secondary: var(--polar-night-1);
    --fg-primary: var(--snow-storm-0);
    --fg-secondary: var(--polar-night-3);
    --accent: var(--frost-1);
    --success: var(--aurora-green);
    --warning: var(--aurora-yellow);
    --error: var(--aurora-red);
}

* {
    color: var(--fg-primary);
    background: var(--bg-primary);
}

.title {
    color: var(--frost-1);
}

.button {
    background: var(--frost-2);
    color: var(--polar-night-0);
}

.button:hover {
    background: var(--frost-1);
}

.input {
    background: var(--polar-night-1);
    border-color: var(--polar-night-3);
}

.input:focus {
    border-color: var(--frost-1);
}

.list-item:selected {
    background: var(--frost-2);
    color: var(--polar-night-0);
}
"#
}

pub fn theme_monokai() -> &'static str {
    r#"/* Monokai Theme for Revue */

:root {
    --bg-primary: #272822;
    --bg-secondary: #3e3d32;
    --fg-primary: #f8f8f2;
    --fg-secondary: #75715e;
    --accent: #a6e22e;
    --pink: #f92672;
    --orange: #fd971f;
    --yellow: #e6db74;
    --purple: #ae81ff;
    --cyan: #66d9ef;

    --success: var(--accent);
    --warning: var(--orange);
    --error: var(--pink);
}

* {
    color: var(--fg-primary);
    background: var(--bg-primary);
}

.title {
    color: var(--pink);
}

.button {
    background: var(--pink);
    color: var(--fg-primary);
}

.button:hover {
    background: var(--accent);
    color: var(--bg-primary);
}

.input {
    background: var(--bg-secondary);
    border-color: var(--fg-secondary);
}

.input:focus {
    border-color: var(--cyan);
}

.list-item:selected {
    background: var(--pink);
    color: var(--fg-primary);
}
"#
}

pub fn theme_gruvbox() -> &'static str {
    r#"/* Gruvbox Theme for Revue */
/* https://github.com/morhetz/gruvbox */

:root {
    --bg-hard: #1d2021;
    --bg: #282828;
    --bg-soft: #32302f;
    --bg1: #3c3836;
    --bg2: #504945;
    --bg3: #665c54;
    --bg4: #7c6f64;

    --fg: #ebdbb2;
    --fg0: #fbf1c7;
    --fg1: #ebdbb2;
    --fg2: #d5c4a1;
    --fg3: #bdae93;
    --fg4: #a89984;

    --red: #fb4934;
    --green: #b8bb26;
    --yellow: #fabd2f;
    --blue: #83a598;
    --purple: #d3869b;
    --aqua: #8ec07c;
    --orange: #fe8019;

    --bg-primary: var(--bg);
    --bg-secondary: var(--bg1);
    --fg-primary: var(--fg);
    --fg-secondary: var(--fg4);
    --accent: var(--yellow);
    --success: var(--green);
    --warning: var(--orange);
    --error: var(--red);
}

* {
    color: var(--fg-primary);
    background: var(--bg-primary);
}

.title {
    color: var(--yellow);
}

.button {
    background: var(--yellow);
    color: var(--bg);
}

.button:hover {
    background: var(--orange);
}

.input {
    background: var(--bg1);
    border-color: var(--bg3);
}

.input:focus {
    border-color: var(--yellow);
}

.list-item:selected {
    background: var(--yellow);
    color: var(--bg);
}
"#
}

pub fn theme_catppuccin() -> &'static str {
    r#"/* Catppuccin Mocha Theme for Revue */
/* https://github.com/catppuccin/catppuccin */

:root {
    --rosewater: #f5e0dc;
    --flamingo: #f2cdcd;
    --pink: #f5c2e7;
    --mauve: #cba6f7;
    --red: #f38ba8;
    --maroon: #eba0ac;
    --peach: #fab387;
    --yellow: #f9e2af;
    --green: #a6e3a1;
    --teal: #94e2d5;
    --sky: #89dceb;
    --sapphire: #74c7ec;
    --blue: #89b4fa;
    --lavender: #b4befe;

    --text: #cdd6f4;
    --subtext1: #bac2de;
    --subtext0: #a6adc8;
    --overlay2: #9399b2;
    --overlay1: #7f849c;
    --overlay0: #6c7086;
    --surface2: #585b70;
    --surface1: #45475a;
    --surface0: #313244;
    --base: #1e1e2e;
    --mantle: #181825;
    --crust: #11111b;

    --bg-primary: var(--base);
    --bg-secondary: var(--surface0);
    --fg-primary: var(--text);
    --fg-secondary: var(--overlay0);
    --accent: var(--mauve);
    --success: var(--green);
    --warning: var(--yellow);
    --error: var(--red);
}

* {
    color: var(--fg-primary);
    background: var(--bg-primary);
}

.title {
    color: var(--mauve);
}

.button {
    background: var(--mauve);
    color: var(--base);
}

.button:hover {
    background: var(--pink);
}

.input {
    background: var(--surface0);
    border-color: var(--surface2);
}

.input:focus {
    border-color: var(--mauve);
}

.list-item:selected {
    background: var(--mauve);
    color: var(--base);
}

.success { color: var(--green); }
.warning { color: var(--yellow); }
.error { color: var(--red); }
"#
}

// =============================================================================
// Component Templates
// =============================================================================

/// Search component template
pub fn component_search() -> &'static str {
    r#"//! Search component with filter state
//!
//! Generated by: revue add search

use revue::prelude::*;

pub struct SearchComponent {
    search: SearchState,
    items: Vec<String>,
}

impl SearchComponent {
    pub fn new() -> Self {
        Self {
            search: SearchState::new().mode(SearchMode::Fuzzy),
            items: vec![
                "Apple".to_string(),
                "Banana".to_string(),
                "Cherry".to_string(),
            ],
        }
    }

    pub fn handle_key(&mut self, key: &Key) -> bool {
        match key {
            Key::Char('/') if !self.search.is_active() => {
                self.search.activate();
                true
            }
            Key::Escape if self.search.is_active() => {
                self.search.deactivate();
                self.search.clear();
                true
            }
            Key::Backspace if self.search.is_active() => {
                self.search.pop();
                true
            }
            Key::Char(c) if self.search.is_active() => {
                self.search.push(*c);
                true
            }
            _ => false,
        }
    }

    pub fn filtered_items(&self) -> Vec<&String> {
        self.search.filter(&self.items, |s| s.clone())
    }
}

impl View for SearchComponent {
    fn render(&self, ctx: &mut RenderContext) {
        let filtered = self.filtered_items();

        vstack()
            .gap(1)
            .child(
                if self.search.is_active() {
                    Text::new(format!("Search: {}_", self.search.query()))
                } else {
                    Text::muted("Press / to search")
                }
            )
            .child(
                list()
                    .items(filtered.iter().map(|s| s.as_str()).collect())
            )
            .render(ctx);
    }
}
"#
}

/// Form component template
pub fn component_form() -> &'static str {
    r#"//! Form component with validation
//!
//! Generated by: revue add form

use revue::prelude::*;

pub struct FormComponent {
    form: FormState,
}

impl FormComponent {
    pub fn new() -> Self {
        let form = FormState::new()
            .field("username", |f| f
                .label("Username")
                .placeholder("Enter username")
                .required()
                .min_length(3))
            .field("email", |f| f
                .email()
                .label("Email")
                .placeholder("user@example.com"))
            .field("password", |f| f
                .password()
                .label("Password")
                .required()
                .min_length(8))
            .build();

        Self { form }
    }

    pub fn handle_key(&self, key: &Key) -> bool {
        match key {
            Key::Tab => {
                self.form.focus_next();
                true
            }
            Key::BackTab => {
                self.form.focus_prev();
                true
            }
            Key::Enter => {
                if self.form.submit() {
                    // Form is valid, handle submission
                    println!("Form submitted: {:?}", self.form.values());
                }
                true
            }
            _ => false,
        }
    }
}

impl View for FormComponent {
    fn render(&self, ctx: &mut RenderContext) {
        vstack()
            .gap(1)
            .child(Text::new("Registration Form").bold())
            .child(divider())
            .child(
                vstack()
                    .gap(1)
                    .children(
                        self.form.iter().map(|(name, field)| {
                            let _is_focused = self.form.focused().as_deref() == Some(name);
                            vstack()
                                .child(Text::new(&field.label))
                                .child(
                                    input()
                                        .value(&field.value())
                                        .placeholder(&field.placeholder)
                                )
                                .child(
                                    if let Some(err) = field.first_error() {
                                        Text::new(&err).fg(Color::RED)
                                    } else {
                                        Text::empty()
                                    }
                                )
                        }).collect()
                    )
            )
            .child(
                button("Submit")
                    .variant(ButtonVariant::Primary)
            )
            .render(ctx);
    }
}
"#
}

/// Navigation component template
pub fn component_navigation() -> &'static str {
    r#"//! Navigation component with history
//!
//! Generated by: revue add navigation

use revue::prelude::*;

pub struct NavigationComponent {
    nav: NavigationState,
}

impl NavigationComponent {
    pub fn new() -> Self {
        Self {
            nav: NavigationState::new("home"),
        }
    }

    pub fn handle_key(&mut self, key: &Key) -> bool {
        match key {
            Key::Left | Key::Char('h') if self.nav.can_go_back() => {
                self.nav.back();
                true
            }
            Key::Right | Key::Char('l') if self.nav.can_go_forward() => {
                self.nav.forward();
                true
            }
            Key::Char('1') => {
                self.nav.push("home");
                true
            }
            Key::Char('2') => {
                self.nav.push("settings");
                true
            }
            Key::Char('3') => {
                self.nav.push("profile");
                true
            }
            _ => false,
        }
    }

    pub fn current_page(&self) -> &str {
        self.nav.path()
    }
}

impl View for NavigationComponent {
    fn render(&self, ctx: &mut RenderContext) {
        vstack()
            .gap(1)
            .child(
                hstack()
                    .gap(2)
                    .child(
                        if self.nav.can_go_back() {
                            Text::new("← Back")
                        } else {
                            Text::muted("← Back")
                        }
                    )
                    .child(Text::new(self.nav.path()).bold())
                    .child(
                        if self.nav.can_go_forward() {
                            Text::new("Forward →")
                        } else {
                            Text::muted("Forward →")
                        }
                    )
            )
            .child(divider())
            .child(
                match self.nav.path() {
                    "home" => Text::new("Welcome to Home"),
                    "settings" => Text::new("Settings Page"),
                    "profile" => Text::new("Profile Page"),
                    _ => Text::new("Unknown Page"),
                }
            )
            .child(divider())
            .child(Text::muted("[1] Home  [2] Settings  [3] Profile  [←/→] Navigate"))
            .render(ctx);
    }
}
"#
}

/// Modal component template
pub fn component_modal() -> &'static str {
    r#"//! Modal dialog component
//!
//! Generated by: revue add modal

use revue::prelude::*;

pub struct ModalComponent {
    show_modal: bool,
    confirm: ConfirmState,
}

impl ModalComponent {
    pub fn new() -> Self {
        Self {
            show_modal: false,
            confirm: ConfirmState::new(),
        }
    }

    pub fn open(&mut self) {
        self.show_modal = true;
    }

    pub fn close(&mut self) {
        self.show_modal = false;
    }

    pub fn handle_key(&mut self, key: &Key) -> bool {
        if self.show_modal {
            match key {
                Key::Escape | Key::Char('n') => {
                    self.close();
                    true
                }
                Key::Enter | Key::Char('y') => {
                    // Handle confirm action
                    self.close();
                    true
                }
                _ => false,
            }
        } else {
            match key {
                Key::Char('m') => {
                    self.open();
                    true
                }
                _ => false,
            }
        }
    }
}

impl View for ModalComponent {
    fn render(&self, ctx: &mut RenderContext) {
        vstack()
            .child(Text::new("Press [m] to open modal"))
            .child(
                if self.show_modal {
                    modal()
                        .title("Confirm Action")
                        .body("Are you sure you want to proceed?")
                        .button("Yes", ModalButtonStyle::Primary)
                        .button("No", ModalButtonStyle::Secondary)
                } else {
                    modal().visible(false)
                }
            )
            .render(ctx);
    }
}
"#
}

/// Toast component template
pub fn component_toast() -> &'static str {
    r#"//! Toast notification component
//!
//! Generated by: revue add toast

use revue::prelude::*;

pub struct ToastComponent {
    message: MessageState,
}

impl ToastComponent {
    pub fn new() -> Self {
        Self {
            message: MessageState::new(),
        }
    }

    pub fn show_success(&mut self, msg: &str) {
        self.message.set_success(msg);
    }

    pub fn show_error(&mut self, msg: &str) {
        self.message.set_error(msg);
    }

    pub fn show_info(&mut self, msg: &str) {
        self.message.set_info(msg);
    }

    pub fn handle_key(&mut self, key: &Key) -> bool {
        match key {
            Key::Char('s') => {
                self.show_success("Operation successful!");
                true
            }
            Key::Char('e') => {
                self.show_error("Something went wrong!");
                true
            }
            Key::Char('i') => {
                self.show_info("Here's some information");
                true
            }
            _ => false,
        }
    }

    pub fn tick(&mut self) -> bool {
        self.message.check_timeout()
    }
}

impl View for ToastComponent {
    fn render(&self, ctx: &mut RenderContext) {
        vstack()
            .gap(1)
            .child(Text::new("[s] Success  [e] Error  [i] Info"))
            .child(
                if let Some(msg) = self.message.get() {
                    toast(msg)
                        .level(ToastLevel::Info)
                        .position(ToastPosition::TopRight)
                } else {
                    toast("").visible(false)
                }
            )
            .render(ctx);
    }
}
"#
}

/// Command palette component template
pub fn component_command_palette() -> &'static str {
    r#"//! Command palette component
//!
//! Generated by: revue add command-palette

use revue::prelude::*;

pub struct CommandPaletteComponent {
    palette: CommandPalette,
    show_palette: bool,
}

impl CommandPaletteComponent {
    pub fn new() -> Self {
        let commands = vec![
            Command::new("file.open", "Open File"),
            Command::new("file.save", "Save File"),
            Command::new("file.close", "Close File"),
            Command::new("edit.undo", "Undo"),
            Command::new("edit.redo", "Redo"),
            Command::new("view.theme", "Change Theme"),
        ];

        Self {
            palette: command_palette().commands(commands),
            show_palette: false,
        }
    }

    pub fn handle_key(&mut self, key: &Key) -> Option<&str> {
        if self.show_palette {
            match key {
                Key::Escape => {
                    self.show_palette = false;
                    None
                }
                Key::Enter => {
                    let cmd = self.palette.selected_command();
                    self.show_palette = false;
                    cmd.map(|c| c.id.as_str())
                }
                _ => {
                    // Handle palette navigation
                    None
                }
            }
        } else {
            match key {
                Key::Ctrl('p') | Key::Ctrl('k') => {
                    self.show_palette = true;
                    None
                }
                _ => None,
            }
        }
    }
}

impl View for CommandPaletteComponent {
    fn render(&self, ctx: &mut RenderContext) {
        vstack()
            .child(Text::new("Press Ctrl+P to open command palette"))
            .child(
                if self.show_palette {
                    self.palette.clone()
                } else {
                    command_palette().visible(false)
                }
            )
            .render(ctx);
    }
}
"#
}

/// Table component template
pub fn component_table() -> &'static str {
    r#"//! Data table component
//!
//! Generated by: revue add table

use revue::prelude::*;

pub struct TableComponent {
    data: Vec<Vec<String>>,
    selected_row: usize,
}

impl TableComponent {
    pub fn new() -> Self {
        Self {
            data: vec![
                vec!["1".into(), "Alice".into(), "alice@example.com".into()],
                vec!["2".into(), "Bob".into(), "bob@example.com".into()],
                vec!["3".into(), "Charlie".into(), "charlie@example.com".into()],
            ],
            selected_row: 0,
        }
    }

    pub fn handle_key(&mut self, key: &Key) -> bool {
        match key {
            Key::Up | Key::Char('k') => {
                if self.selected_row > 0 {
                    self.selected_row -= 1;
                }
                true
            }
            Key::Down | Key::Char('j') => {
                if self.selected_row < self.data.len() - 1 {
                    self.selected_row += 1;
                }
                true
            }
            _ => false,
        }
    }

    pub fn selected(&self) -> Option<&Vec<String>> {
        self.data.get(self.selected_row)
    }
}

impl View for TableComponent {
    fn render(&self, ctx: &mut RenderContext) {
        table()
            .column(column("ID").width(5))
            .column(column("Name").width(15))
            .column(column("Email").width(25))
            .rows(self.data.clone())
            .selected(self.selected_row)
            .render(ctx);
    }
}
"#
}

/// Tabs component template
pub fn component_tabs() -> &'static str {
    r#"//! Tab navigation component
//!
//! Generated by: revue add tabs

use revue::prelude::*;

pub struct TabsComponent {
    active_tab: usize,
    tabs: Vec<(&'static str, &'static str)>,
}

impl TabsComponent {
    pub fn new() -> Self {
        Self {
            active_tab: 0,
            tabs: vec![
                ("overview", "Overview"),
                ("details", "Details"),
                ("settings", "Settings"),
            ],
        }
    }

    pub fn handle_key(&mut self, key: &Key) -> bool {
        match key {
            Key::Tab | Key::Right | Key::Char('l') => {
                self.active_tab = (self.active_tab + 1) % self.tabs.len();
                true
            }
            Key::BackTab | Key::Left | Key::Char('h') => {
                self.active_tab = if self.active_tab == 0 {
                    self.tabs.len() - 1
                } else {
                    self.active_tab - 1
                };
                true
            }
            Key::Char(c) if c.is_ascii_digit() => {
                let idx = c.to_digit(10).unwrap() as usize;
                if idx > 0 && idx <= self.tabs.len() {
                    self.active_tab = idx - 1;
                }
                true
            }
            _ => false,
        }
    }

    pub fn active_id(&self) -> &str {
        self.tabs.get(self.active_tab).map(|(id, _)| *id).unwrap_or("")
    }
}

impl View for TabsComponent {
    fn render(&self, ctx: &mut RenderContext) {
        vstack()
            .gap(1)
            .child(
                tabs()
                    .tabs(self.tabs.iter().map(|(id, label)| {
                        Tab::new(*id, *label)
                    }).collect())
                    .active(self.active_tab)
            )
            .child(divider())
            .child(
                match self.active_id() {
                    "overview" => Text::new("This is the Overview tab content"),
                    "details" => Text::new("This is the Details tab content"),
                    "settings" => Text::new("This is the Settings tab content"),
                    _ => Text::new("Unknown tab"),
                }
            )
            .render(ctx);
    }
}
"#
}
