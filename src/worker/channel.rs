//! Worker channel for communication between workers and UI

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

/// Message types for worker communication
#[derive(Debug, Clone)]
pub enum WorkerMessage<T> {
    /// Progress update (0.0 to 1.0)
    Progress(f32),
    /// Status message
    Status(String),
    /// Partial result
    Partial(T),
    /// Final result
    Complete(T),
    /// Error occurred
    Error(String),
    /// Custom message
    Custom(String),
}

/// Bidirectional channel for worker communication
pub struct WorkerChannel<T> {
    /// Messages from worker to UI
    to_ui: Arc<Mutex<VecDeque<WorkerMessage<T>>>>,
    /// Messages from UI to worker
    to_worker: Arc<Mutex<VecDeque<WorkerCommand>>>,
    /// Channel capacity
    capacity: usize,
}

impl<T: Clone> WorkerChannel<T> {
    /// Create a new channel with default capacity
    pub fn new() -> Self {
        Self::with_capacity(100)
    }

    /// Create with specific capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            to_ui: Arc::new(Mutex::new(VecDeque::with_capacity(capacity))),
            to_worker: Arc::new(Mutex::new(VecDeque::with_capacity(capacity))),
            capacity,
        }
    }

    /// Create a sender/receiver pair
    pub fn split(&self) -> (WorkerSender<T>, WorkerReceiver<T>) {
        (
            WorkerSender {
                to_ui: self.to_ui.clone(),
                to_worker: self.to_worker.clone(),
                capacity: self.capacity,
            },
            WorkerReceiver {
                to_ui: self.to_ui.clone(),
                to_worker: self.to_worker.clone(),
            },
        )
    }

    /// Send message from worker side
    pub fn send(&self, msg: WorkerMessage<T>) -> bool {
        if let Ok(mut queue) = self.to_ui.lock() {
            if queue.len() < self.capacity {
                queue.push_back(msg);
                return true;
            }
        }
        false
    }

    /// Receive message on UI side
    pub fn recv(&self) -> Option<WorkerMessage<T>> {
        self.to_ui.lock().ok().and_then(|mut q| q.pop_front())
    }

    /// Send command from UI to worker
    pub fn send_command(&self, cmd: WorkerCommand) -> bool {
        if let Ok(mut queue) = self.to_worker.lock() {
            if queue.len() < self.capacity {
                queue.push_back(cmd);
                return true;
            }
        }
        false
    }

    /// Receive command on worker side
    pub fn recv_command(&self) -> Option<WorkerCommand> {
        self.to_worker.lock().ok().and_then(|mut q| q.pop_front())
    }

    /// Check if there are pending messages for UI
    pub fn has_messages(&self) -> bool {
        self.to_ui.lock().map(|q| !q.is_empty()).unwrap_or(false)
    }

    /// Check if there are pending commands for worker
    pub fn has_commands(&self) -> bool {
        self.to_worker
            .lock()
            .map(|q| !q.is_empty())
            .unwrap_or(false)
    }

    /// Get number of pending messages
    pub fn message_count(&self) -> usize {
        self.to_ui.lock().map(|q| q.len()).unwrap_or(0)
    }
}

impl<T: Clone> Default for WorkerChannel<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> Clone for WorkerChannel<T> {
    fn clone(&self) -> Self {
        Self {
            to_ui: self.to_ui.clone(),
            to_worker: self.to_worker.clone(),
            capacity: self.capacity,
        }
    }
}

/// Commands from UI to worker
#[derive(Debug, Clone)]
pub enum WorkerCommand {
    /// Cancel the task
    Cancel,
    /// Pause the task
    Pause,
    /// Resume the task
    Resume,
    /// Custom command
    Custom(String),
}

/// Sender half of worker channel (used by worker)
pub struct WorkerSender<T> {
    to_ui: Arc<Mutex<VecDeque<WorkerMessage<T>>>>,
    to_worker: Arc<Mutex<VecDeque<WorkerCommand>>>,
    capacity: usize,
}

impl<T: Clone> WorkerSender<T> {
    /// Send message to UI
    pub fn send(&self, msg: WorkerMessage<T>) -> bool {
        if let Ok(mut queue) = self.to_ui.lock() {
            if queue.len() < self.capacity {
                queue.push_back(msg);
                return true;
            }
        }
        false
    }

    /// Send progress update
    pub fn progress(&self, value: f32) -> bool {
        self.send(WorkerMessage::Progress(value.clamp(0.0, 1.0)))
    }

    /// Send status message
    pub fn status(&self, msg: impl Into<String>) -> bool {
        self.send(WorkerMessage::Status(msg.into()))
    }

    /// Send partial result
    pub fn partial(&self, value: T) -> bool {
        self.send(WorkerMessage::Partial(value))
    }

    /// Send complete message
    pub fn complete(&self, value: T) -> bool {
        self.send(WorkerMessage::Complete(value))
    }

    /// Send error
    pub fn error(&self, msg: impl Into<String>) -> bool {
        self.send(WorkerMessage::Error(msg.into()))
    }

    /// Check for commands from UI
    pub fn check_command(&self) -> Option<WorkerCommand> {
        self.to_worker.lock().ok().and_then(|mut q| q.pop_front())
    }

    /// Check if cancelled
    pub fn is_cancelled(&self) -> bool {
        self.to_worker
            .lock()
            .ok()
            .map(|q| q.iter().any(|cmd| matches!(cmd, WorkerCommand::Cancel)))
            .unwrap_or(false)
    }
}

impl<T: Clone> Clone for WorkerSender<T> {
    fn clone(&self) -> Self {
        Self {
            to_ui: self.to_ui.clone(),
            to_worker: self.to_worker.clone(),
            capacity: self.capacity,
        }
    }
}

/// Receiver half of worker channel (used by UI)
pub struct WorkerReceiver<T> {
    to_ui: Arc<Mutex<VecDeque<WorkerMessage<T>>>>,
    to_worker: Arc<Mutex<VecDeque<WorkerCommand>>>,
}

impl<T: Clone> WorkerReceiver<T> {
    /// Receive message from worker
    pub fn recv(&self) -> Option<WorkerMessage<T>> {
        self.to_ui.lock().ok().and_then(|mut q| q.pop_front())
    }

    /// Receive all pending messages
    pub fn recv_all(&self) -> Vec<WorkerMessage<T>> {
        self.to_ui
            .lock()
            .ok()
            .map(|mut q| q.drain(..).collect())
            .unwrap_or_default()
    }

    /// Send command to worker
    pub fn send_command(&self, cmd: WorkerCommand) -> bool {
        if let Ok(mut queue) = self.to_worker.lock() {
            queue.push_back(cmd);
            return true;
        }
        false
    }

    /// Send cancel command
    pub fn cancel(&self) -> bool {
        self.send_command(WorkerCommand::Cancel)
    }

    /// Send pause command
    pub fn pause(&self) -> bool {
        self.send_command(WorkerCommand::Pause)
    }

    /// Send resume command
    pub fn resume(&self) -> bool {
        self.send_command(WorkerCommand::Resume)
    }

    /// Check if there are pending messages
    pub fn has_messages(&self) -> bool {
        self.to_ui.lock().map(|q| !q.is_empty()).unwrap_or(false)
    }

    /// Get message count
    pub fn message_count(&self) -> usize {
        self.to_ui.lock().map(|q| q.len()).unwrap_or(0)
    }
}

impl<T: Clone> Clone for WorkerReceiver<T> {
    fn clone(&self) -> Self {
        Self {
            to_ui: self.to_ui.clone(),
            to_worker: self.to_worker.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_send_recv() {
        let channel: WorkerChannel<i32> = WorkerChannel::new();

        channel.send(WorkerMessage::Progress(0.5));
        channel.send(WorkerMessage::Status("working".to_string()));
        channel.send(WorkerMessage::Complete(42));

        assert!(
            matches!(channel.recv(), Some(WorkerMessage::Progress(p)) if (p - 0.5).abs() < 0.01)
        );
        assert!(matches!(channel.recv(), Some(WorkerMessage::Status(_))));
        assert!(matches!(channel.recv(), Some(WorkerMessage::Complete(42))));
        assert!(channel.recv().is_none());
    }

    #[test]
    fn test_channel_split() {
        let channel: WorkerChannel<String> = WorkerChannel::new();
        let (sender, receiver) = channel.split();

        sender.progress(0.75);
        sender.status("Loading...");
        sender.complete("Done".to_string());

        let messages = receiver.recv_all();
        assert_eq!(messages.len(), 3);
    }

    #[test]
    fn test_commands() {
        let channel: WorkerChannel<()> = WorkerChannel::new();
        let (sender, receiver) = channel.split();

        receiver.cancel();
        assert!(sender.is_cancelled());
    }
}
