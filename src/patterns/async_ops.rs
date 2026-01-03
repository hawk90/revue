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

    // AsyncTask tests
    #[test]
    fn test_async_task_spawn_and_wait() {
        let task = AsyncTask::spawn(|| 42);
        assert_eq!(task.wait(), Some(42));
    }

    #[test]
    fn test_async_task_spawn_string() {
        let task = AsyncTask::spawn(|| "hello".to_string());
        assert_eq!(task.wait(), Some("hello".to_string()));
    }

    #[test]
    fn test_async_task_spawn_vec() {
        let task = AsyncTask::spawn(|| vec![1, 2, 3]);
        assert_eq!(task.wait(), Some(vec![1, 2, 3]));
    }

    #[test]
    fn test_async_task_try_recv_completed() {
        let mut task = AsyncTask::spawn(|| 42);
        // Wait for completion
        thread::sleep(Duration::from_millis(10));
        // May or may not be ready, but shouldn't panic
        let _ = task.try_recv();
    }

    #[test]
    fn test_async_task_cancel() {
        let task = AsyncTask::spawn(|| {
            thread::sleep(Duration::from_millis(1000));
            42
        });
        // Cancel should not panic
        task.cancel();
    }

    #[test]
    #[ignore] // Flaky in CI due to timing sensitivity
    fn test_async_task_timing() {
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
    #[ignore] // Flaky in CI due to timing sensitivity
    fn test_async_task_is_running() {
        let task = AsyncTask::spawn(|| {
            thread::sleep(Duration::from_millis(100));
            42
        });

        assert!(task.is_running());
        thread::sleep(Duration::from_millis(150));
        assert!(!task.is_running());
    }

    // Spinner tests
    #[test]
    fn test_spinner_frames_count() {
        assert_eq!(SPINNER_FRAMES.len(), 8);
    }

    #[test]
    fn test_spinner_char_first() {
        assert_eq!(spinner_char(0), "⣾");
    }

    #[test]
    fn test_spinner_char_second() {
        assert_eq!(spinner_char(1), "⣽");
    }

    #[test]
    fn test_spinner_char_all_frames() {
        assert_eq!(spinner_char(0), "⣾");
        assert_eq!(spinner_char(1), "⣽");
        assert_eq!(spinner_char(2), "⣻");
        assert_eq!(spinner_char(3), "⢿");
        assert_eq!(spinner_char(4), "⡿");
        assert_eq!(spinner_char(5), "⣟");
        assert_eq!(spinner_char(6), "⣯");
        assert_eq!(spinner_char(7), "⣷");
    }

    #[test]
    fn test_spinner_char_wraps() {
        assert_eq!(spinner_char(8), "⣾"); // wraps to 0
        assert_eq!(spinner_char(9), "⣽"); // wraps to 1
        assert_eq!(spinner_char(16), "⣾"); // wraps to 0
    }

    #[test]
    fn test_spinner_char_large_number() {
        // Should not panic with large frame numbers
        let _ = spinner_char(1000);
        let _ = spinner_char(usize::MAX);
    }

    // spawn_task tests
    #[test]
    fn test_spawn_task_int() {
        let (rx, handle) = spawn_task(|| 42);
        assert_eq!(rx.recv().unwrap(), 42);
        handle.join().unwrap();
    }

    #[test]
    fn test_spawn_task_string() {
        let (rx, handle) = spawn_task(|| "result".to_string());
        assert_eq!(rx.recv().unwrap(), "result");
        handle.join().unwrap();
    }

    #[test]
    fn test_spawn_task_result() {
        let (rx, handle) = spawn_task(|| -> Result<i32, &str> { Ok(42) });
        assert_eq!(rx.recv().unwrap(), Ok(42));
        handle.join().unwrap();
    }

    #[test]
    fn test_spawn_task_computation() {
        let (rx, _) = spawn_task(|| {
            let mut sum = 0;
            for i in 0..100 {
                sum += i;
            }
            sum
        });
        assert_eq!(rx.recv().unwrap(), 4950);
    }

    // spawn_with_sender tests
    #[test]
    fn test_spawn_with_sender_single() {
        let rx = spawn_with_sender(|tx| {
            tx.send(42).unwrap();
        });
        assert_eq!(rx.recv().unwrap(), 42);
    }

    #[test]
    fn test_spawn_with_sender_multiple() {
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
    fn test_spawn_with_sender_collect() {
        let rx = spawn_with_sender(|tx| {
            for i in 0..5 {
                tx.send(i).unwrap();
            }
        });

        let results: Vec<i32> = rx.iter().collect();
        assert_eq!(results, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_spawn_with_sender_strings() {
        let rx = spawn_with_sender(|tx| {
            tx.send("hello".to_string()).unwrap();
            tx.send("world".to_string()).unwrap();
        });

        assert_eq!(rx.recv().unwrap(), "hello");
        assert_eq!(rx.recv().unwrap(), "world");
    }

    #[test]
    fn test_spawn_with_sender_channel_closes() {
        let rx = spawn_with_sender(|tx| {
            tx.send(1).unwrap();
            // Channel closes when tx is dropped
        });

        assert_eq!(rx.recv().unwrap(), 1);
        // After sender is dropped, recv returns error
        assert!(rx.recv().is_err());
    }
}
