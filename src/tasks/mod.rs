//! Task management for async operations in TUI apps
//!
//! Provides Timer, TaskRunner, PooledTaskRunner, and EventBus for managing async patterns
//! that integrate with the App's tick loop.
//!
//! # Features
//!
//! | Component | Description | Use Case |
//!|-----------|-------------|----------|
//! | [`TaskRunner`] | Spawns OS thread per task | Few concurrent tasks |
//! | [`PooledTaskRunner`] | Fixed thread pool | Many concurrent tasks |
//! | [`Timer`] | Delayed callbacks | Debouncing, intervals |
//! | [`EventBus`] | Pub/sub messaging | Component communication |
//!
//! # Quick Start
//!
//! ## TaskRunner vs PooledTaskRunner
//!
//! - **TaskRunner**: Spawns a new OS thread for each task. Good for a few concurrent tasks.
//! - **PooledTaskRunner**: Uses a fixed thread pool. Prevents thread explosion when processing many items.
//!
//! ## Basic Example
//!
//! ```rust,ignore
//! use revue::tasks::{PooledTaskRunner, Timer};
//! use std::time::Duration;
//!
//! struct MyApp {
//!     tasks: PooledTaskRunner<String>,
//!     timers: Timer,
//! }
//!
//! impl MyApp {
//!     fn new() -> Self {
//!         Self {
//!             tasks: PooledTaskRunner::new(4), // Max 4 concurrent tasks
//!             timers: Timer::new(),
//!         }
//!     }
//!
//!     fn mount(&mut self) {
//!         // Spawn many background tasks (won't create 100 threads!)
//!         for i in 0..100 {
//!             self.tasks.spawn(format!("fetch_{}", i), move || {
//!                 fetch_data(i)
//!             });
//!         }
//!
//!         // Set a timer
//!         self.timers.set("auto_save", Duration::from_secs(30));
//!     }
//!
//!     fn poll(&mut self) -> bool {
//!         let mut updated = false;
//!
//!         // Check task results
//!         while let Some(result) = self.tasks.poll() {
//!             self.handle_result(result.id, result.result);
//!             updated = true;
//!         }
//!
//!         // Check timers
//!         while let Some(id) = self.timers.poll_expired() {
//!             self.handle_timer(id);
//!             updated = true;
//!         }
//!
//!         updated
//!     }
//! }
//! ```
//!
//! # TaskRunner
//!
//! Simple API for spawning tasks:
//!
//! ```rust,ignore
//! use revue::tasks::TaskRunner;
//!
//! let mut tasks = TaskRunner::new();
//!
//! // Spawn a task
//! tasks.spawn("my_task", || {
//!     // Do expensive work
//!     42
//! });
//!
//! // Poll for results
//! while let Some(result) = tasks.poll() {
//!     println!("Task {} returned: {:?}", result.id, result.result);
//! }
//! ```
//!
//! # PooledTaskRunner
//!
//! Thread pool for many concurrent tasks:
//!
//! ```rust,ignore
//! use revue::tasks::PooledTaskRunner;
//!
//! // Create pool with max 8 threads
//! let mut tasks = PooledTaskRunner::new(8);
//!
//! // Spawn many tasks
//! for i in 0..1000 {
//!     tasks.spawn(format!("task_{}", i), move || {
//!         process_item(i)
//!     });
//! }
//!
//! // Tasks are executed by the pool, max 8 at a time
//! while let Some(result) = tasks.poll() {
//!     handle_result(result.id, result.result);
//! }
//! ```
//!
//! # Timer
//!
//! Schedule delayed callbacks:
//!
//! ```rust,ignore
//! use revue::tasks::Timer;
//! use std::time::Duration;
//!
//! let mut timer = Timer::new();
//!
//! // Set a one-shot timer
//! timer.set("debounce", Duration::from_millis(300));
//!
//! // Set a repeating timer
//! timer.set_repeating("tick", Duration::from_secs(1));
//!
//! // Poll for expired timers
//! while let Some(id) = timer.poll_expired() {
//!     match id {
//!         "debounce" => handle_debounce(),
//!         "tick" => handle_tick(),
//!         _ => {}
//!     }
//! }
//! ```
//!
//! # EventBus
//!
//! Publish/subscribe for component communication:
//!
//! ```rust,ignore
//! use revue::tasks::EventBus;
//!
//! let mut bus = EventBus::new();
//!
//! // Subscribe to events
//! let sub = bus.subscribe("user_login", |data| {
//!     println!("User logged in: {:?}", data);
//! });
//!
//! // Publish events
//! bus.publish("user_login", json!({"user": "Alice"}));
//!
//! // Unsubscribe when done
//! bus.unsubscribe(sub);
//! ```

mod event_bus;
mod pooled_runner;
mod runner;
mod timer;

pub use event_bus::{EventBus, EventId, Subscription};
pub use pooled_runner::PooledTaskRunner;
pub use runner::{TaskId, TaskResult, TaskRunner};
pub use timer::{Timer, TimerEntry, TimerId};
