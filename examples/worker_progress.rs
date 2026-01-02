//! Worker progress example
//!
//! Demonstrates WorkerChannel for progress updates and bidirectional communication.
//!
//! Run with: cargo run --example worker_progress

use revue::prelude::*;
use revue::worker::{WorkerChannel, WorkerCommand, WorkerMessage, WorkerSender};
use std::thread;
use std::time::Duration;

struct ProgressDemo {
    receiver: Option<revue::worker::WorkerReceiver<String>>,
    progress: f32,
    status: String,
    partial_results: Vec<String>,
    final_result: Option<String>,
    is_running: bool,
}

impl ProgressDemo {
    fn new() -> Self {
        Self {
            receiver: None,
            progress: 0.0,
            status: "Ready".to_string(),
            partial_results: Vec::new(),
            final_result: None,
            is_running: false,
        }
    }

    fn start_processing(&mut self) {
        if self.is_running {
            self.status = "Already running!".to_string();
            return;
        }

        let channel = WorkerChannel::new();
        let (sender, receiver) = channel.split();
        self.receiver = Some(receiver);
        self.is_running = true;
        self.progress = 0.0;
        self.status = "Starting...".to_string();
        self.partial_results.clear();
        self.final_result = None;

        // Spawn worker thread
        thread::spawn(move || {
            process_items(sender);
        });
    }

    fn cancel_task(&mut self) {
        if let Some(receiver) = &self.receiver {
            receiver.cancel();
            self.status = "Cancellation requested...".to_string();
        }
    }

    fn pause_task(&mut self) {
        if let Some(receiver) = &self.receiver {
            receiver.pause();
            self.status = "Pause requested...".to_string();
        }
    }

    fn resume_task(&mut self) {
        if let Some(receiver) = &self.receiver {
            receiver.resume();
            self.status = "Resume requested...".to_string();
        }
    }

    fn handle_key(&mut self, key: &Key) -> bool {
        match key {
            Key::Char('s') => {
                self.start_processing();
                true
            }
            Key::Char('c') => {
                self.cancel_task();
                true
            }
            Key::Char('p') => {
                self.pause_task();
                true
            }
            Key::Char('r') => {
                self.resume_task();
                true
            }
            Key::Char('x') => {
                self.receiver = None;
                self.is_running = false;
                self.progress = 0.0;
                self.status = "Cleared".to_string();
                self.partial_results.clear();
                self.final_result = None;
                true
            }
            _ => false,
        }
    }

    fn tick(&mut self) -> bool {
        let mut updated = false;

        if let Some(receiver) = &self.receiver {
            while let Some(msg) = receiver.recv() {
                match msg {
                    WorkerMessage::Progress(p) => {
                        self.progress = p;
                        updated = true;
                    }
                    WorkerMessage::Status(s) => {
                        self.status = s;
                        updated = true;
                    }
                    WorkerMessage::Partial(result) => {
                        self.partial_results.push(result);
                        if self.partial_results.len() > 5 {
                            self.partial_results.remove(0);
                        }
                        updated = true;
                    }
                    WorkerMessage::Complete(result) => {
                        self.final_result = Some(result);
                        self.is_running = false;
                        self.status = "Completed!".to_string();
                        updated = true;
                    }
                    WorkerMessage::Error(e) => {
                        self.status = format!("Error: {}", e);
                        self.is_running = false;
                        updated = true;
                    }
                    WorkerMessage::Custom(msg) => {
                        self.status = format!("Custom: {}", msg);
                        updated = true;
                    }
                }
            }
        }

        updated
    }
}

impl View for ProgressDemo {
    fn render(&self, ctx: &mut RenderContext) {
        let progress_percent = (self.progress * 100.0) as i32;
        let progress_bar_width = 40;
        let filled_width = ((progress_bar_width as f32) * self.progress) as usize;
        let progress_bar = format!(
            "[{}{}] {}%",
            "=".repeat(filled_width),
            " ".repeat(progress_bar_width - filled_width),
            progress_percent
        );

        let view = vstack()
            .gap(1)
            .child(
                Border::panel().title("📊 Worker Progress Demo").child(
                    vstack()
                        .gap(1)
                        .child(
                            hstack()
                                .gap(2)
                                .child(Text::new("Status:").bold())
                                .child(Text::new(&self.status).fg(Color::CYAN)),
                        )
                        .child(hstack().gap(2).child(Text::new("Running:")).child(
                            if self.is_running {
                                Text::new("Yes").fg(Color::YELLOW)
                            } else {
                                Text::new("No").fg(Color::rgb(100, 100, 100))
                            },
                        )),
                ),
            )
            .child(
                Border::single().title("Progress").child(
                    vstack()
                        .child(Text::new(progress_bar).fg(Color::GREEN))
                        .child(Text::muted(format!(
                            "{:.1}% complete",
                            self.progress * 100.0
                        ))),
                ),
            )
            .child(
                Border::single()
                    .title(format!("Partial Results (last 5)"))
                    .child({
                        let mut stack = vstack();
                        if self.partial_results.is_empty() {
                            stack = stack.child(Text::muted("No partial results yet"));
                        } else {
                            for result in &self.partial_results {
                                stack = stack.child(Text::new(format!("• {}", result)));
                            }
                        }
                        stack
                    }),
            )
            .child(Border::single().title("Final Result").child(
                if let Some(result) = &self.final_result {
                    vstack()
                        .child(Text::success("✓ Task completed"))
                        .child(Text::new(result).fg(Color::WHITE))
                } else {
                    vstack().child(Text::muted("Not completed yet"))
                },
            ))
            .child(
                Border::success_box()
                    .title("✨ Features Demonstrated")
                    .child(
                        vstack()
                            .child(Text::success(
                                "✓ WorkerChannel: Bidirectional communication",
                            ))
                            .child(Text::success("✓ Progress updates (0.0 to 1.0)"))
                            .child(Text::success("✓ Status messages"))
                            .child(Text::success("✓ Partial results"))
                            .child(Text::success("✓ Commands: Cancel, Pause, Resume")),
                    ),
            )
            .child(
                Border::rounded().title("Controls").child(
                    vstack()
                        .child(
                            hstack()
                                .gap(2)
                                .child(Text::muted("[s]"))
                                .child(Text::new("Start processing")),
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
                                .child(Text::muted("[p]"))
                                .child(Text::new("Pause task")),
                        )
                        .child(
                            hstack()
                                .gap(2)
                                .child(Text::muted("[r]"))
                                .child(Text::new("Resume task")),
                        )
                        .child(
                            hstack()
                                .gap(2)
                                .child(Text::muted("[x]"))
                                .child(Text::new("Clear all")),
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
        WidgetMeta::new("ProgressDemo")
    }
}

fn process_items(sender: WorkerSender<String>) {
    sender.status("Initializing...".to_string());
    thread::sleep(Duration::from_millis(500));

    let total_items = 20;

    for i in 0..total_items {
        // Check for commands
        if let Some(cmd) = sender.check_command() {
            match cmd {
                WorkerCommand::Cancel => {
                    sender.status("Cancelled by user".to_string());
                    sender.error("Task was cancelled");
                    return;
                }
                WorkerCommand::Pause => {
                    sender.status("Paused".to_string());
                    // Wait for resume
                    loop {
                        thread::sleep(Duration::from_millis(100));
                        if let Some(WorkerCommand::Resume) = sender.check_command() {
                            sender.status("Resumed".to_string());
                            break;
                        }
                        if sender.is_cancelled() {
                            sender.error("Cancelled while paused");
                            return;
                        }
                    }
                }
                WorkerCommand::Resume => {
                    sender.status("Already running".to_string());
                }
                WorkerCommand::Custom(msg) => {
                    sender.status(format!("Custom command: {}", msg));
                }
            }
        }

        // Update progress
        let progress = (i + 1) as f32 / total_items as f32;
        sender.progress(progress);

        // Update status
        sender.status(format!("Processing item {}/{}", i + 1, total_items));

        // Simulate work
        thread::sleep(Duration::from_millis(200));

        // Send partial result every 5 items
        if (i + 1) % 5 == 0 {
            sender.partial(format!("Completed batch {}", (i + 1) / 5));
        }
    }

    // Send final result
    sender.complete(format!("Successfully processed {} items", total_items));
}

fn main() -> Result<()> {
    println!("📊 Worker Progress Example");
    println!("Demonstrates WorkerChannel with progress updates.\n");

    let mut app = App::builder().build();
    let mut demo = ProgressDemo::new();

    app.run(demo, |event, demo, _app| match event {
        Event::Key(key_event) => demo.handle_key(&key_event.key),
        Event::Tick => demo.tick(),
        _ => false,
    })
}
