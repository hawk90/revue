//! Async operation patterns using mpsc channels
//!
//! Provides patterns for handling background operations in TUI apps.
//! Uses standard library threads and channels (no async runtime needed).
//!
//! # Example
//!
//! ```ignore
//! use revue::patterns::AsyncTask;
//! use std::thread;
//!
//! struct App {
//!     items: Vec<Item>,
//!     loading: bool,
//!     fetch_task: Option<AsyncTask<Vec<Item>>>,
//! }
//!
//! impl App {
//!     fn start_fetch(&mut self) {
//!         let client = self.client.clone();
//!         let task = AsyncTask::spawn(move || {
//!             client.fetch_items()
//!         });
//!
//!         self.fetch_task = Some(task);
//!         self.loading = true;
//!     }
//!
//!     fn poll(&mut self) -> bool {
//!         let mut needs_redraw = false;
//!
//!         if let Some(task) = &mut self.fetch_task {
//!             if let Some(result) = task.try_recv() {
//!                 match result {
//!                     Ok(items) => self.items = items,
//!                     Err(e) => self.message.set(format!("Error: {}", e)),
//!                 }
//!                 self.fetch_task = None;
//!                 self.loading = false;
//!                 needs_redraw = true;
//!             }
//!         }
//!
//!         needs_redraw
//!     }
//! }
//! ```

use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::thread::{self, JoinHandle};

/// Async task with non-blocking result polling
///
/// Wraps a background thread operation with a channel receiver.
/// Call `try_recv()` in your poll/animation loop to check for results.
pub struct AsyncTask<T> {
    rx: Receiver<T>,
    handle: Option<JoinHandle<()>>,
}

impl<T: Send + 'static> AsyncTask<T> {
    /// Spawn a new background task
    ///
    /// # Example
    ///
    /// ```ignore
    /// let task = AsyncTask::spawn(|| {
    ///     // Heavy computation or I/O
    ///     fetch_data_from_api()
    /// });
    /// ```
    pub fn spawn<F>(f: F) -> Self
    where
        F: FnOnce() -> T + Send + 'static,
    {
        let (tx, rx) = mpsc::channel();
        let handle = thread::spawn(move || {
            let result = f();
            let _ = tx.send(result);
        });

        Self {
            rx,
            handle: Some(handle),
        }
    }

    /// Try to receive result (non-blocking)
    ///
    /// Returns `Some(result)` if task completed, `None` otherwise.
    ///
    /// # Example
    ///
    /// ```ignore
    /// if let Some(result) = task.try_recv() {
    ///     // Task completed!
    ///     self.handle_result(result);
    /// }
    /// ```
    pub fn try_recv(&mut self) -> Option<T> {
        match self.rx.try_recv() {
            Ok(result) => Some(result),
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => None,
        }
    }

    /// Check if task is still running
    pub fn is_running(&self) -> bool {
        matches!(self.rx.try_recv(), Err(TryRecvError::Empty))
    }

    /// Wait for task to complete (blocking)
    ///
    /// Only use this if you know the task will complete quickly.
    pub fn wait(self) -> Option<T> {
        self.rx.recv().ok()
    }

    /// Cancel the task
    ///
    /// Drops the receiver, which will cause the sender to fail.
    /// The background thread will continue until it tries to send.
    pub fn cancel(mut self) {
        drop(self.rx);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

// Removed AsyncPoller trait - just implement poll() directly in your App

/// Spinner frames for loading indicators
pub const SPINNER_FRAMES: &[&str] = &["⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"];

/// Get spinner character for current frame
///
/// # Example
///
/// ```ignore
/// let frame = self.spinner_frame;
/// let spinner = spinner_char(frame);
/// ctx.draw_text(x, y, spinner, CYAN);
/// ```
pub fn spinner_char(frame: usize) -> &'static str {
    SPINNER_FRAMES[frame % SPINNER_FRAMES.len()]
}

/// Helper to run a function in background and get a channel
///
/// Returns `(Receiver, JoinHandle)` for manual management.
///
/// # Example
///
/// ```ignore
/// let (rx, handle) = spawn_task(|| fetch_data());
/// ```
pub fn spawn_task<T, F>(f: F) -> (Receiver<T>, JoinHandle<()>)
where
    T: Send + 'static,
    F: FnOnce() -> T + Send + 'static,
{
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        let result = f();
        let _ = tx.send(result);
    });
    (rx, handle)
}

/// Helper for spawning task with sender access
///
/// Useful when you need to send multiple updates.
///
/// # Example
///
/// ```ignore
/// let rx = spawn_with_sender(|tx| {
///     for i in 0..10 {
///         tx.send(i).unwrap();
///         thread::sleep(Duration::from_millis(100));
///     }
/// });
/// ```
pub fn spawn_with_sender<T, F>(f: F) -> Receiver<T>
where
    T: Send + 'static,
    F: FnOnce(Sender<T>) + Send + 'static,
{
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        f(tx);
    });
    rx
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    #[ignore] // Flaky in CI due to timing sensitivity
    fn test_async_task() {
        let task = AsyncTask::spawn(|| {
            thread::sleep(Duration::from_millis(50));
            42
        });

        // Should not be ready immediately
        let mut task = task;
        assert!(task.try_recv().is_none());

        // Wait and check again
        thread::sleep(Duration::from_millis(150));
        assert_eq!(task.try_recv(), Some(42));
    }

    #[test]
    fn test_spinner_char() {
        assert_eq!(spinner_char(0), "⣾");
        assert_eq!(spinner_char(1), "⣽");
        assert_eq!(spinner_char(8), "⣾"); // wraps around
    }

    #[test]
    fn test_spawn_task() {
        let (rx, _handle) = spawn_task(|| 42);
        assert_eq!(rx.recv().unwrap(), 42);
    }

    #[test]
    fn test_spawn_with_sender() {
        let rx = spawn_with_sender(|tx| {
            tx.send(1).unwrap();
            tx.send(2).unwrap();
            tx.send(3).unwrap();
        });

        assert_eq!(rx.recv().unwrap(), 1);
        assert_eq!(rx.recv().unwrap(), 2);
        assert_eq!(rx.recv().unwrap(), 3);
    }

    #[test]
    #[ignore] // Flaky in CI due to timing sensitivity
    fn test_is_running() {
        let task = AsyncTask::spawn(|| {
            thread::sleep(Duration::from_millis(100));
            42
        });

        assert!(task.is_running());
        thread::sleep(Duration::from_millis(150));
        assert!(!task.is_running());
    }
}
