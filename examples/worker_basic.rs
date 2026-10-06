//! Worker basic usage example
//!
//! Demonstrates WorkerHandle usage for background tasks.
//!
//! Run with: cargo run --example worker_basic

use revue::prelude::*;
use revue::utils::unicode::display_width;
use revue::worker::{WorkerHandle, WorkerState};
use std::time::Duration;

/// Width of the key column in the Controls list.
const KEY_COLS: u16 = 3;

/// Columns `text` occupies, for sizing single-line text in an `hstack`.
fn cols(text: &str) -> u16 {
    display_width(text) as u16
}

struct WorkerDemo {
    handle: Option<WorkerHandle<String>>,
    result: Option<String>,
    status: String,
    task_count: usize,
}

impl WorkerDemo {
    fn new() -> Self {
        Self {
            handle: None,
            result: None,
            status: "Ready".to_string(),
            task_count: 0,
        }
    }

    fn start_task(&mut self, duration_ms: u64) {
        if self.handle.is_some() {
            self.status = "Task already running!".to_string();
            return;
        }

        self.task_count += 1;
        let count = self.task_count;

        self.status = format!("Starting task #{} ({}ms)...", count, duration_ms);

        let handle = WorkerHandle::spawn_blocking(move || {
            // Simulate work
            std::thread::sleep(Duration::from_millis(duration_ms));
            format!("Task #{} completed after {}ms", count, duration_ms)
        });

        self.handle = Some(handle);
    }

    fn cancel_task(&mut self) {
        if let Some(handle) = &self.handle {
            handle.cancel();
            self.status = "Cancelled!".to_string();
            self.handle = None;
        }
    }

    fn handle_key(&mut self, key: &Key) -> bool {
        match key {
            Key::Char('1') => {
                self.start_task(1000); // 1 second
                true
            }
            Key::Char('2') => {
                self.start_task(3000); // 3 seconds
                true
            }
            Key::Char('3') => {
                self.start_task(5000); // 5 seconds
                true
            }
            Key::Char('c') => {
                self.cancel_task();
                true
            }
            Key::Char('r') => {
                self.result = None;
                self.status = "Cleared".to_string();
                true
            }
            _ => false,
        }
    }

    fn tick(&mut self) -> bool {
        if let Some(handle) = &mut self.handle {
            let state = handle.state();

            match state {
                WorkerState::Running => {
                    self.status = "Task running...".to_string();
                }
                WorkerState::Completed => {
                    // Take ownership and get result
                    if let Some(h) = self.handle.take() {
                        match h.join() {
                            Ok(result) => {
                                self.result = Some(result);
                                self.status = "Completed!".to_string();
                            }
                            Err(e) => {
                                self.status = format!("Error: {}", e);
                            }
                        }
                        return true;
                    }
                }
                WorkerState::Failed => {
                    if let Some(h) = self.handle.take() {
                        if let Err(e) = h.join() {
                            self.status = format!("Failed: {}", e);
                        }
                        return true;
                    }
                }
                WorkerState::Cancelled => {
                    self.handle = None;
                    return true;
                }
                _ => {}
            }
        }

        false
    }
}

impl View for WorkerDemo {
    fn render(&self, ctx: &mut RenderContext) {
        let state = self.handle.as_ref().map(|h| h.state());
        let _is_running = matches!(state, Some(WorkerState::Running));

        let state_text = match state {
            Some(WorkerState::Pending) => "Pending",
            Some(WorkerState::Running) => "Running",
            Some(WorkerState::Completed) => "Completed",
            Some(WorkerState::Failed) => "Failed",
            Some(WorkerState::Cancelled) => "Cancelled",
            None => "None",
        };

        let state_color = match state {
            Some(WorkerState::Running) => Color::YELLOW,
            Some(WorkerState::Completed) => Color::GREEN,
            Some(WorkerState::Failed) | Some(WorkerState::Cancelled) => Color::RED,
            _ => Color::rgb(100, 100, 100),
        };

        // Every fixed-height row/box is `child_sized`; the Result box takes the
        // rest with `child_flex` - a stack would otherwise size it to its content.
        let view = vstack()
            .gap(1)
            .child_sized(
                Border::panel().title("⚙️  Worker Handle Demo").child(
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
                                .child_sized(Text::new("Worker state:"), cols("Worker state:"))
                                .child(Text::new(state_text).fg(state_color)),
                            1,
                        )
                        .child_sized(
                            hstack()
                                .gap(2)
                                .child_sized(
                                    Text::new("Tasks completed:"),
                                    cols("Tasks completed:"),
                                )
                                .child(Text::new(format!("{}", self.task_count))),
                            1,
                        ),
                ),
                5,
            )
            .child_flex(
                Border::single()
                    .title("Result")
                    .child(if let Some(result) = &self.result {
                        vstack()
                            .child_sized(Text::success("✓ Task completed"), 1)
                            .child_sized(Text::new(result).fg(Color::WHITE), 1)
                    } else {
                        vstack().child_sized(Text::muted("No result yet"), 1)
                    }),
                1.0,
            )
            .child_sized(
                Border::success_box()
                    .title("✨ Features Demonstrated")
                    .child(
                        vstack()
                            .child_sized(Text::success("✓ WorkerHandle: Spawn blocking tasks"), 1)
                            .child_sized(
                                Text::success("✓ State tracking: Pending → Running → Completed"),
                                1,
                            )
                            .child_sized(Text::success("✓ Result retrieval with join()"), 1)
                            .child_sized(Text::success("✓ Cancellation support"), 1)
                            .child_sized(Text::success("✓ Panic handling"), 1),
                    ),
                7,
            )
            .child_sized(
                Border::rounded().title("Controls").child(
                    vstack()
                        .child_sized(
                            hstack()
                                .gap(2)
                                .child_sized(Text::muted("[1]"), KEY_COLS)
                                .child(Text::new("Start 1s task")),
                            1,
                        )
                        .child_sized(
                            hstack()
                                .gap(2)
                                .child_sized(Text::muted("[2]"), KEY_COLS)
                                .child(Text::new("Start 3s task")),
                            1,
                        )
                        .child_sized(
                            hstack()
                                .gap(2)
                                .child_sized(Text::muted("[3]"), KEY_COLS)
                                .child(Text::new("Start 5s task")),
                            1,
                        )
                        .child_sized(
                            hstack()
                                .gap(2)
                                .child_sized(Text::muted("[c]"), KEY_COLS)
                                .child(Text::new("Cancel task")),
                            1,
                        )
                        .child_sized(
                            hstack()
                                .gap(2)
                                .child_sized(Text::muted("[r]"), KEY_COLS)
                                .child(Text::new("Clear result")),
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
                8,
            );

        view.render(ctx);
    }

    fn meta(&self) -> WidgetMeta {
        WidgetMeta::new("WorkerDemo")
    }
}

fn main() -> Result<()> {
    println!("⚙️  Worker Handle Basic Usage Example");
    println!("Demonstrates WorkerHandle state tracking and control.\n");

    let mut app = App::builder().build();
    let demo = WorkerDemo::new();

    app.run(demo, |event, demo, _app| match event {
        Event::Key(key_event) => demo.handle_key(&key_event.key),
        Event::Tick => demo.tick(),
        _ => false,
    })
}
