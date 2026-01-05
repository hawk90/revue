//! Project templates and theme definitions

/// Generate Cargo.toml
pub fn cargo_toml(name: &str) -> String {
    format!(
        r#"[package]
name = "{name}"
version = "0.1.0"
edition = "2021"

[dependencies]
revue = {{ path = "../" }}
"#
    )
}

/// Generate .gitignore
pub fn gitignore() -> &'static str {
    r#"/target
Cargo.lock
*.swp
*.swo
.DS_Store
"#
}

/// Default CSS style
pub fn default_style() -> &'static str {
    r#"/* Revue Default Styles */

:root {
    --bg-primary: #1a1b26;
    --bg-secondary: #24283b;
    --fg-primary: #c0caf5;
    --fg-secondary: #565f89;
    --accent: #7aa2f7;
    --success: #9ece6a;
    --warning: #e0af68;
    --error: #f7768e;
}

* {
    color: var(--fg-primary);
    background: var(--bg-primary);
}

.container {
    padding: 1;
    margin: 1;
}

.header {
    background: var(--bg-secondary);
    padding: 1;
    text-align: center;
}

.title {
    color: var(--accent);
    font-weight: bold;
}

.button {
    background: var(--accent);
    color: var(--bg-primary);
    padding: 0 2;
}

.button:hover {
    background: #89b4fa;
}

.button:focus {
    border-color: var(--accent);
}

.input {
    background: var(--bg-secondary);
    border-color: var(--fg-secondary);
    padding: 0 1;
}

.input:focus {
    border-color: var(--accent);
}

.list-item {
    padding: 0 1;
}

.list-item:selected {
    background: var(--accent);
    color: var(--bg-primary);
}

.card {
    background: var(--bg-secondary);
    border-color: var(--fg-secondary);
    padding: 1;
    margin: 1;
}

.success {
    color: var(--success);
}

.warning {
    color: var(--warning);
}

.error {
    color: var(--error);
}
"#
}

// =============================================================================
// Basic Template
// =============================================================================

pub fn basic_main() -> &'static str {
    r#"//! Basic Revue Application

mod app;

use revue::prelude::*;

fn main() -> Result<()> {
    App::builder()
        .style("styles/main.css")
        .mount(app::App::new)
        .run()
}
"#
}

pub fn basic_app() -> &'static str {
    r#"//! Application component

use revue::prelude::*;

pub struct App {
    counter: Signal<i32>,
}

impl App {
    pub fn new() -> Self {
        Self {
            counter: signal(0),
        }
    }

    fn increment(&mut self) {
        self.counter.set(self.counter.get() + 1);
    }

    fn decrement(&mut self) {
        self.counter.set(self.counter.get() - 1);
    }
}

impl View for App {
    fn render(&self, ctx: &mut RenderContext) {
        vstack()
            .class("container")
            .child(
                text("🎨 Revue App")
                    .class("title")
                    .alignment(Alignment::Center)
            )
            .child(
                text(format!("Count: {}", self.counter.get()))
                    .alignment(Alignment::Center)
            )
            .child(
                hstack()
                    .child(button("- Decrement").class("button"))
                    .child(button("+ Increment").class("button"))
            )
            .render(ctx);
    }
}
"#
}

// =============================================================================
// Dashboard Template
// =============================================================================

pub fn dashboard_main() -> &'static str {
    r#"//! Dashboard Application

mod app;

use revue::prelude::*;

fn main() -> Result<()> {
    App::builder()
        .title("Dashboard")
        .style("styles/main.css")
        .mount(app::Dashboard::new)
        .run()
}
"#
}

pub fn dashboard_app() -> &'static str {
    r#"//! Dashboard component

use revue::prelude::*;

pub struct Dashboard {
    cpu_usage: Signal<f32>,
    memory_usage: Signal<f32>,
    disk_usage: Signal<f32>,
    requests: Signal<Vec<u64>>,
}

impl Dashboard {
    pub fn new() -> Self {
        Self {
            cpu_usage: signal(45.2),
            memory_usage: signal(62.8),
            disk_usage: signal(34.5),
            requests: signal(vec![120, 145, 132, 167, 189, 156, 178]),
        }
    }
}

impl View for Dashboard {
    fn render(&self, ctx: &mut RenderContext) {
        vstack()
            .class("container")
            .child(
                text("📊 System Dashboard")
                    .class("title")
                    .alignment(Alignment::Center)
            )
            .child(divider().label("Metrics"))
            .child(
                hstack()
                    .child(self.render_gauge("CPU", self.cpu_usage.get()))
                    .child(self.render_gauge("Memory", self.memory_usage.get()))
                    .child(self.render_gauge("Disk", self.disk_usage.get()))
            )
            .child(divider().label("Requests"))
            .child(
                sparkline(self.requests.get().iter().map(|&x| x as f64).collect())
                    .class("card")
            )
            .child(divider())
            .child(
                hstack()
                    .child(badge("Online").success())
                    .child(badge("3 Alerts").warning())
                    .child(text("Last updated: 2s ago").class("dimmed"))
            )
            .render(ctx);
    }
}

impl Dashboard {
    fn render_gauge(&self, label: &str, value: f32) -> impl View {
        vstack()
            .class("card")
            .child(text(label).class("title"))
            .child(gauge(value / 100.0).percentage())
            .child(text(format!("{:.1}%", value)))
    }
}
"#
}

// =============================================================================
// Todo Template
// =============================================================================

pub fn todo_main() -> &'static str {
    r#"//! Todo Application

mod app;

use revue::prelude::*;

fn main() -> Result<()> {
    App::builder()
        .title("Todo App")
        .style("styles/main.css")
        .mount(app::TodoApp::new)
        .run()
}
"#
}

pub fn todo_app() -> &'static str {
    r#"//! Todo application component

use revue::prelude::*;

#[derive(Clone)]
pub struct TodoItem {
    id: usize,
    text: String,
    completed: bool,
}

pub struct TodoApp {
    todos: Signal<Vec<TodoItem>>,
    input: Signal<String>,
    filter: Signal<Filter>,
    next_id: usize,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Filter {
    All,
    Active,
    Completed,
}

impl TodoApp {
    pub fn new() -> Self {
        Self {
            todos: signal(vec![
                TodoItem { id: 1, text: "Learn Revue".into(), completed: true },
                TodoItem { id: 2, text: "Build awesome TUI".into(), completed: false },
                TodoItem { id: 3, text: "Ship it!".into(), completed: false },
            ]),
            input: signal(String::new()),
            filter: signal(Filter::All),
            next_id: 4,
        }
    }

    fn add_todo(&mut self) {
        let text = self.input.get().trim().to_string();
        if !text.is_empty() {
            let mut todos = self.todos.get().clone();
            todos.push(TodoItem {
                id: self.next_id,
                text,
                completed: false,
            });
            self.next_id += 1;
            self.todos.set(todos);
            self.input.set(String::new());
        }
    }

    fn toggle_todo(&mut self, id: usize) {
        let mut todos = self.todos.get().clone();
        if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
            todo.completed = !todo.completed;
        }
        self.todos.set(todos);
    }

    fn delete_todo(&mut self, id: usize) {
        let todos: Vec<_> = self.todos.get()
            .iter()
            .filter(|t| t.id != id)
            .cloned()
            .collect();
        self.todos.set(todos);
    }

    fn filtered_todos(&self) -> Vec<TodoItem> {
        let filter = self.filter.get();
        self.todos.get()
            .iter()
            .filter(|t| match filter {
                Filter::All => true,
                Filter::Active => !t.completed,
                Filter::Completed => t.completed,
            })
            .cloned()
            .collect()
    }
}

impl View for TodoApp {
    fn render(&self, ctx: &mut RenderContext) {
        let todos = self.filtered_todos();
        let active_count = self.todos.get().iter().filter(|t| !t.completed).count();

        vstack()
            .class("container")
            .child(
                text("📝 Todo App")
                    .class("title")
                    .alignment(Alignment::Center)
            )
            .child(
                hstack()
                    .child(Input::new().placeholder("What needs to be done?").class("input"))
                    .child(button("Add").class("button"))
            )
            .child(divider())
            .child(
                vstack().children(
                    todos.iter().map(|todo| {
                        hstack()
                            .class("list-item")
                            .child(checkbox(&todo.text).checked(todo.completed))
                            .child(button("×").class("button error"))
                    })
                )
            )
            .child(divider())
            .child(
                hstack()
                    .child(text(format!("{} items left", active_count)))
                    .child(
                        hstack()
                            .child(button("All").class("button"))
                            .child(button("Active").class("button"))
                            .child(button("Completed").class("button"))
                    )
            )
            .render(ctx);
    }
}
"#
}

// =============================================================================
// Chat Template
// =============================================================================

pub fn chat_main() -> &'static str {
    r#"//! Chat Application

mod app;

use revue::prelude::*;

fn main() -> Result<()> {
    App::builder()
        .title("Chat")
        .style("styles/main.css")
        .mount(app::ChatApp::new)
        .run()
}
"#
}

pub fn chat_app() -> &'static str {
    r#"//! Chat application component

use revue::prelude::*;

#[derive(Clone)]
pub struct Message {
    id: usize,
    user: String,
    text: String,
    timestamp: String,
    is_self: bool,
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
    input: Signal<String>,
    users: Signal<Vec<User>>,
    current_user: String,
}

impl ChatApp {
    pub fn new() -> Self {
        Self {
            messages: signal(vec![
                Message {
                    id: 1,
                    user: "Alice".into(),
                    text: "Hey everyone! 👋".into(),
                    timestamp: "10:30".into(),
                    is_self: false,
                },
                Message {
                    id: 2,
                    user: "Bob".into(),
                    text: "Hi Alice! How's it going?".into(),
                    timestamp: "10:31".into(),
                    is_self: false,
                },
                Message {
                    id: 3,
                    user: "You".into(),
                    text: "Hello! Just built this chat with Revue!".into(),
                    timestamp: "10:32".into(),
                    is_self: true,
                },
            ]),
            input: signal(String::new()),
            users: signal(vec![
                User { name: "Alice".into(), status: UserStatus::Online },
                User { name: "Bob".into(), status: UserStatus::Online },
                User { name: "Charlie".into(), status: UserStatus::Away },
                User { name: "Diana".into(), status: UserStatus::Offline },
            ]),
            current_user: "You".into(),
        }
    }

    fn send_message(&mut self) {
        let text = self.input.get().trim().to_string();
        if !text.is_empty() {
            let mut messages = self.messages.get().clone();
            messages.push(Message {
                id: messages.len() + 1,
                user: self.current_user.clone(),
                text,
                timestamp: "Now".into(),
                is_self: true,
            });
            self.messages.set(messages);
            self.input.set(String::new());
        }
    }
}

impl View for ChatApp {
    fn render(&self, ctx: &mut RenderContext) {
        hstack()
            .class("container")
            // Sidebar
            .child(
                vstack()
                    .class("sidebar")
                    .child(text("👥 Users").class("title"))
                    .child(divider())
                    .children(
                        self.users.get().iter().map(|user| {
                            hstack()
                                .class("list-item")
                                .child(
                                    match user.status {
                                        UserStatus::Online => avatar(&user.name).online(),
                                        UserStatus::Away => avatar(&user.name).away(),
                                        UserStatus::Offline => avatar(&user.name).offline(),
                                    }
                                )
                                .child(text(&user.name))
                        })
                    )
            )
            // Main chat area
            .child(
                vstack()
                    .class("chat-area")
                    .child(
                        text("💬 General Chat")
                            .class("title")
                    )
                    .child(divider())
                    // Messages
                    .child(
                        scroll_view(
                            vstack().children(
                                self.messages.get().iter().map(|msg| {
                                    self.render_message(msg)
                                })
                            )
                        )
                    )
                    .child(divider())
                    // Input
                    .child(
                        hstack()
                            .child(
                                Input::new()
                                    .placeholder("Type a message...")
                                    .class("input")
                            )
                            .child(button("Send").class("button"))
                    )
            )
            .render(ctx);
    }
}

impl ChatApp {
    fn render_message(&self, msg: &Message) -> impl View {
        let align = if msg.is_self { "right" } else { "left" };

        hstack()
            .child(
                if !msg.is_self {
                    Some(avatar(&msg.user).small())
                } else {
                    None
                }
            )
            .child(
                vstack()
                    .class(if msg.is_self { "message self" } else { "message" })
                    .child(
                        hstack()
                            .child(text(&msg.user).class("username"))
                            .child(text(&msg.timestamp).class("timestamp"))
                    )
                    .child(text(&msg.text))
            )
    }
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
