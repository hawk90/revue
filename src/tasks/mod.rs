//! Task management for async operations in TUI apps
//!
//! Provides Timer, TaskRunner, PooledTaskRunner, and EventBus for managing async patterns
//! that integrate with the App's tick loop.
//!
//! # TaskRunner vs PooledTaskRunner
//!
//! - **TaskRunner**: Spawns a new OS thread for each task. Good for a few concurrent tasks.
//! - **PooledTaskRunner**: Uses a fixed thread pool. Prevents thread explosion when processing many items.
//!
//! # Example
//!
//! ```ignore
//! use revue::tasks::{PooledTaskRunner, Timer, EventBus};
//!
//! struct MyApp {
//!     tasks: PooledTaskRunner<MyMessage>,
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
//!             self.tasks.spawn(format!("fetch_{}", i), move || fetch_data(i));
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

mod timer;
mod runner;
mod pooled_runner;
mod event_bus;

pub use timer::{Timer, TimerId, TimerEntry};
pub use runner::{TaskRunner, TaskId, TaskResult};
pub use pooled_runner::PooledTaskRunner;
pub use event_bus::{EventBus, EventId, Subscription};
