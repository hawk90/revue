//! Worker basic usage example
//!
//! Demonstrates WorkerHandle usage for background tasks.
//!
//! Run with: cargo run --example worker_basic

use revue::prelude::*;
use revue::worker::{WorkerHandle, WorkerState};
use std::time::Duration;

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
        let is_running = matches!(state, Some(WorkerState::Running));

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

        let view = vstack()
            .gap(1)
            .child(
                Border::panel().title("⚙️  Worker Handle Demo").child(
                    vstack()
                        .child(
                            hstack()
                                .gap(2)
                                .child(Text::new("Status:").bold())
                                .child(Text::new(&self.status).fg(Color::CYAN)),
                        )
                        .child(
                            hstack()
                                .gap(2)
                                .child(Text::new("Worker state:"))
                                .child(Text::new(state_text).fg(state_color)),
                        )
                        .child(
                            hstack()
                                .gap(2)
                                .child(Text::new("Tasks completed:"))
                                .child(Text::new(format!("{}", self.task_count))),
                        ),
                ),
            )
            .child(
                Border::single()
                    .title("Result")
                    .child(if let Some(result) = &self.result {
                        vstack()
                            .child(Text::success("✓ Task completed"))
                            .child(Text::new(result).fg(Color::WHITE))
                    } else {
                        vstack().child(Text::muted("No result yet"))
                    }),
            )
            .child(
                Border::success_box()
                    .title("✨ Features Demonstrated")
                    .child(
                        vstack()
                            .child(Text::success("✓ WorkerHandle: Spawn blocking tasks"))
                            .child(Text::success(
                                "✓ State tracking: Pending → Running → Completed",
                            ))
                            .child(Text::success("✓ Result retrieval with join()"))
                            .child(Text::success("✓ Cancellation support"))
                            .child(Text::success("✓ Panic handling")),
                    ),
            )
            .child(
                Border::rounded().title("Controls").child(
                    vstack()
                        .child(
                            hstack()
                                .gap(2)
                                .child(Text::muted("[1]"))
                                .child(Text::new("Start 1s task")),
                        )
                        .child(
                            hstack()
                                .gap(2)
                                .child(Text::muted("[2]"))
                                .child(Text::new("Start 3s task")),
                        )
                        .child(
                            hstack()
                                .gap(2)
                                .child(Text::muted("[3]"))
                                .child(Text::new("Start 5s task")),
                        )
                        .child(
                            hstack()
                                .gap(2)
                                .child(Text::muted("[c]"))
                                .child(Text::new("Cancel task")),
                        )
                        .child(
                            hstack()
                                .gap(2)
                                .child(Text::muted("[r]"))
                                .child(Text::new("Clear result")),
                        )
                        .child(
                            hstack()
                                .gap(2)
                                .child(Text::muted("[q]"))
                                .child(Text::new("Quit")),
                        ),
                ),
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
    let mut demo = WorkerDemo::new();

    app.run(demo, |event, demo, _app| match event {
        Event::Key(key_event) => demo.handle_key(&key_event.key),
        Event::Tick => demo.tick(),
        _ => false,
    })
}
