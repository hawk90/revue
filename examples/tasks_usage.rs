//! Tasks module usage example
//!
//! Demonstrates TaskRunner, Timer, and EventBus usage in a TUI app.
//!
//! Run with: cargo run --example tasks_usage

use revue::prelude::*;
use revue::tasks::{EventBus, TaskRunner, Timer};
use revue::utils::unicode::display_width;
use std::time::Duration;

/// Width of the key column in the Controls list.
const KEY_COLS: u16 = 3;

/// Columns `text` occupies, for sizing single-line text in an `hstack`.
fn cols(text: &str) -> u16 {
    display_width(text) as u16
}

#[derive(Clone)]
struct FetchResult {
    data: String,
    count: usize,
}

struct TasksDemo {
    // Task management
    tasks: TaskRunner<FetchResult>,
    timer: Timer,
    bus: EventBus,

    // UI state
    status: String,
    message: Option<String>,
    fetch_count: usize,
    data: Option<FetchResult>,
}

impl TasksDemo {
    fn new() -> Self {
        let mut timer = Timer::new();
        // Auto-refresh every 5 seconds
        timer.set_repeating("auto_refresh", Duration::from_secs(5));

        let mut bus = EventBus::new();
        bus.subscribe("task:started");
        bus.subscribe("task:completed");

        Self {
            tasks: TaskRunner::new(),
            timer,
            bus,
            status: "Ready".to_string(),
            message: None,
            fetch_count: 0,
            data: None,
        }
    }

    fn start_fetch(&mut self) {
        if self.tasks.is_running("fetch_data") {
            self.show_message("Task already running!".to_string());
            return;
        }

        self.fetch_count += 1;
        let count = self.fetch_count;

        // Spawn background task
        self.tasks.spawn("fetch_data", move || {
            // Simulate network delay
            std::thread::sleep(Duration::from_millis(1500));

            FetchResult {
                data: format!("Data from API (fetch #{})", count),
                count,
            }
        });

        self.status = "Fetching data...".to_string();
        self.bus.emit("task:started", "fetch_data");
    }

    fn show_message(&mut self, text: String) {
        self.message = Some(text);
        // Clear message after 2 seconds
        self.timer.set("clear_message", Duration::from_secs(2));
    }

    fn handle_key(&mut self, key: &Key) -> bool {
        match key {
            Key::Char('f') => {
                self.start_fetch();
                true
            }
            Key::Char('c') => {
                self.data = None;
                self.status = "Cleared".to_string();
                self.show_message("Data cleared".to_string());
                true
            }
            Key::Char('s') => {
                self.show_message("Manual message test".to_string());
                true
            }
            _ => false,
        }
    }

    fn tick(&mut self) -> bool {
        let mut updated = false;

        // Poll tasks
        while let Some(result) = self.tasks.poll() {
            if result.id == "fetch_data" {
                match result.result {
                    Ok(data) => {
                        self.status = "Fetch completed!".to_string();
                        self.data = Some(data);
                        self.bus.emit("task:completed", "success");
                    }
                    Err(e) => {
                        self.status = format!("Error: {}", e);
                        self.bus.emit("task:completed", "error");
                    }
                }
                updated = true;
            }
        }

        // Poll timers
        while let Some(id) = self.timer.poll_expired() {
            match id {
                "clear_message" => {
                    self.message = None;
                    updated = true;
                }
                "auto_refresh" if self.data.is_some() && !self.tasks.is_running("fetch_data") => {
                    self.status = "Auto-refreshing...".to_string();
                    self.start_fetch();
                    updated = true;
                }
                _ => {}
            }
        }

        // Poll event bus
        while let Some(event) = self.bus.poll() {
            match event.id {
                "task:started" => {
                    println!("Event: Task started");
                }
                "task:completed" => {
                    if let Some(result) = event.data::<&str>() {
                        println!("Event: Task completed with {}", result);
                    }
                }
                _ => {}
            }
        }

        updated
    }
}

impl View for TasksDemo {
    fn render(&self, ctx: &mut RenderContext) {
        let is_running = self.tasks.is_running("fetch_data");
        let _has_timer = self.timer.has_pending();

        // Every fixed-height row/box is `child_sized`; the Data box takes the
        // rest with `child_flex` - a stack would otherwise size it to its content.
        let view = vstack()
            .gap(1)
            .child_sized(
                Border::panel().title("📋 Tasks Module Demo").child(
                    vstack()
                        .child_sized(
                            hstack()
                                .gap(2)
                                .child_sized(Text::new("Status:").bold(), cols("Status:"))
                                .child(Text::new(&self.status).fg(Color::CYAN)),
                            1,
                        )
                        .child_sized(
                            hstack()
                                .gap(2)
                                .child_sized(Text::new("Task running:"), cols("Task running:"))
                                .child(if is_running {
                                    Text::new("Yes").fg(Color::YELLOW)
                                } else {
                                    Text::new("No").fg(Color::rgb(100, 100, 100))
                                }),
                            1,
                        )
                        .child_sized(
                            hstack()
                                .gap(2)
                                .child_sized(Text::new("Timers active:"), cols("Timers active:"))
                                .child(Text::new(format!("{}", self.timer.count()))),
                            1,
                        ),
                ),
                5,
            )
            .child_flex(
                Border::single()
                    .title("Data")
                    .child(if let Some(data) = &self.data {
                        vstack()
                            .child_sized(Text::success(format!("✓ {}", data.data)), 1)
                            .child_sized(Text::muted(format!("  Fetch #{}", data.count)), 1)
                    } else {
                        vstack()
                            .child_sized(Text::muted("No data yet"), 1)
                            .child_sized(Text::muted("Press 'f' to fetch"), 1)
                    }),
                1.0,
            )
            .child_sized(
                if let Some(msg) = &self.message {
                    Border::rounded()
                        .title("Message")
                        .child(Text::new(msg).fg(Color::YELLOW))
                } else {
                    Border::rounded()
                        .title("Message")
                        .child(Text::muted("(no message)"))
                },
                3,
            )
            .child_sized(
                Border::success_box()
                    .title("✨ Features Demonstrated")
                    .child(
                        vstack()
                            .child_sized(
                                Text::success("✓ TaskRunner: Background fetch with result polling"),
                                1,
                            )
                            .child_sized(
                                Text::success("✓ Timer: Message auto-clear after 2 seconds"),
                                1,
                            )
                            .child_sized(Text::success("✓ Timer: Auto-refresh every 5 seconds"), 1)
                            .child_sized(Text::success("✓ EventBus: Task lifecycle events"), 1),
                    ),
                6,
            )
            .child_sized(
                Border::rounded().title("Controls").child(
                    vstack()
                        .child_sized(
                            hstack()
                                .gap(2)
                                .child_sized(Text::muted("[f]"), KEY_COLS)
                                .child(Text::new("Fetch data")),
                            1,
                        )
                        .child_sized(
                            hstack()
                                .gap(2)
                                .child_sized(Text::muted("[c]"), KEY_COLS)
                                .child(Text::new("Clear data")),
                            1,
                        )
                        .child_sized(
                            hstack()
                                .gap(2)
                                .child_sized(Text::muted("[s]"), KEY_COLS)
                                .child(Text::new("Show test message")),
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
                6,
            );

        view.render(ctx);
    }

    fn meta(&self) -> WidgetMeta {
        WidgetMeta::new("TasksDemo")
    }
}

fn main() -> Result<()> {
    println!("📋 Tasks Module Usage Example");
    println!("Demonstrates TaskRunner, Timer, and EventBus.\n");

    let mut app = App::builder().build();
    let demo = TasksDemo::new();

    app.run(demo, |event, demo, _app| match event {
        Event::Key(key_event) => demo.handle_key(&key_event.key),
        Event::Tick => demo.tick(),
        _ => false,
    })
}
