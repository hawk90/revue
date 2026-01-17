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
            log_warn!(
                "Worker channel overflow: message dropped (queue full at {} items)",
                self.capacity
            );
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
            log_warn!(
                "Worker channel overflow: command {:?} dropped (queue full at {} items)",
                cmd,
                self.capacity
            );
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

    // =========================================================================
    // WorkerMessage tests
    // =========================================================================

    #[test]
    fn test_worker_message_progress() {
        let msg: WorkerMessage<i32> = WorkerMessage::Progress(0.5);
        assert!(matches!(msg, WorkerMessage::Progress(p) if (p - 0.5).abs() < 0.01));
    }

    #[test]
    fn test_worker_message_status() {
        let msg: WorkerMessage<i32> = WorkerMessage::Status("working".to_string());
        assert!(matches!(msg, WorkerMessage::Status(s) if s == "working"));
    }

    #[test]
    fn test_worker_message_partial() {
        let msg: WorkerMessage<i32> = WorkerMessage::Partial(42);
        assert!(matches!(msg, WorkerMessage::Partial(42)));
    }

    #[test]
    fn test_worker_message_complete() {
        let msg: WorkerMessage<i32> = WorkerMessage::Complete(100);
        assert!(matches!(msg, WorkerMessage::Complete(100)));
    }

    #[test]
    fn test_worker_message_error() {
        let msg: WorkerMessage<i32> = WorkerMessage::Error("failed".to_string());
        assert!(matches!(msg, WorkerMessage::Error(e) if e == "failed"));
    }

    #[test]
    fn test_worker_message_custom() {
        let msg: WorkerMessage<i32> = WorkerMessage::Custom("custom data".to_string());
        assert!(matches!(msg, WorkerMessage::Custom(c) if c == "custom data"));
    }

    #[test]
    fn test_worker_message_clone() {
        let msg: WorkerMessage<i32> = WorkerMessage::Complete(42);
        let cloned = msg.clone();
        assert!(matches!(cloned, WorkerMessage::Complete(42)));
    }

    // =========================================================================
    // WorkerChannel tests
    // =========================================================================

    #[test]
    fn test_channel_default() {
        let channel: WorkerChannel<i32> = WorkerChannel::default();
        assert!(!channel.has_messages());
        assert!(!channel.has_commands());
    }

    #[test]
    fn test_channel_with_capacity() {
        let channel: WorkerChannel<i32> = WorkerChannel::with_capacity(10);
        assert_eq!(channel.message_count(), 0);
    }

    #[test]
    fn test_channel_has_messages() {
        let channel: WorkerChannel<i32> = WorkerChannel::new();
        assert!(!channel.has_messages());

        channel.send(WorkerMessage::Complete(42));
        assert!(channel.has_messages());
    }

    #[test]
    fn test_channel_has_commands() {
        let channel: WorkerChannel<i32> = WorkerChannel::new();
        assert!(!channel.has_commands());

        channel.send_command(WorkerCommand::Cancel);
        assert!(channel.has_commands());
    }

    #[test]
    fn test_channel_message_count() {
        let channel: WorkerChannel<i32> = WorkerChannel::new();
        assert_eq!(channel.message_count(), 0);

        channel.send(WorkerMessage::Progress(0.1));
        channel.send(WorkerMessage::Progress(0.2));
        channel.send(WorkerMessage::Progress(0.3));
        assert_eq!(channel.message_count(), 3);
    }

    #[test]
    fn test_channel_recv_command() {
        let channel: WorkerChannel<i32> = WorkerChannel::new();
        channel.send_command(WorkerCommand::Pause);

        let cmd = channel.recv_command();
        assert!(matches!(cmd, Some(WorkerCommand::Pause)));
        assert!(channel.recv_command().is_none());
    }

    #[test]
    fn test_channel_clone() {
        let channel: WorkerChannel<i32> = WorkerChannel::new();
        channel.send(WorkerMessage::Complete(42));

        let cloned = channel.clone();
        // Both should share the same queue
        let msg = cloned.recv();
        assert!(matches!(msg, Some(WorkerMessage::Complete(42))));
    }

    // =========================================================================
    // WorkerCommand tests
    // =========================================================================

    #[test]
    fn test_worker_command_cancel() {
        let cmd = WorkerCommand::Cancel;
        assert!(matches!(cmd, WorkerCommand::Cancel));
    }

    #[test]
    fn test_worker_command_pause() {
        let cmd = WorkerCommand::Pause;
        assert!(matches!(cmd, WorkerCommand::Pause));
    }

    #[test]
    fn test_worker_command_resume() {
        let cmd = WorkerCommand::Resume;
        assert!(matches!(cmd, WorkerCommand::Resume));
    }

    #[test]
    fn test_worker_command_custom() {
        let cmd = WorkerCommand::Custom("stop-early".to_string());
        assert!(matches!(cmd, WorkerCommand::Custom(s) if s == "stop-early"));
    }

    #[test]
    fn test_worker_command_clone() {
        let cmd = WorkerCommand::Custom("test".to_string());
        let cloned = cmd.clone();
        assert!(matches!(cloned, WorkerCommand::Custom(s) if s == "test"));
    }

    // =========================================================================
    // WorkerSender tests
    // =========================================================================

    #[test]
    fn test_sender_progress() {
        let channel: WorkerChannel<i32> = WorkerChannel::new();
        let (sender, receiver) = channel.split();

        assert!(sender.progress(0.5));
        let msg = receiver.recv();
        assert!(matches!(msg, Some(WorkerMessage::Progress(p)) if (p - 0.5).abs() < 0.01));
    }

    #[test]
    fn test_sender_progress_clamp() {
        let channel: WorkerChannel<i32> = WorkerChannel::new();
        let (sender, receiver) = channel.split();

        // Should clamp to 0.0-1.0 range
        assert!(sender.progress(-0.5));
        assert!(sender.progress(1.5));

        let msg1 = receiver.recv();
        let msg2 = receiver.recv();
        assert!(matches!(msg1, Some(WorkerMessage::Progress(p)) if (p - 0.0).abs() < 0.01));
        assert!(matches!(msg2, Some(WorkerMessage::Progress(p)) if (p - 1.0).abs() < 0.01));
    }

    #[test]
    fn test_sender_status() {
        let channel: WorkerChannel<i32> = WorkerChannel::new();
        let (sender, receiver) = channel.split();

        assert!(sender.status("Processing..."));
        let msg = receiver.recv();
        assert!(matches!(msg, Some(WorkerMessage::Status(s)) if s == "Processing..."));
    }

    #[test]
    fn test_sender_partial() {
        let channel: WorkerChannel<i32> = WorkerChannel::new();
        let (sender, receiver) = channel.split();

        assert!(sender.partial(42));
        let msg = receiver.recv();
        assert!(matches!(msg, Some(WorkerMessage::Partial(42))));
    }

    #[test]
    fn test_sender_complete() {
        let channel: WorkerChannel<i32> = WorkerChannel::new();
        let (sender, receiver) = channel.split();

        assert!(sender.complete(100));
        let msg = receiver.recv();
        assert!(matches!(msg, Some(WorkerMessage::Complete(100))));
    }

    #[test]
    fn test_sender_error() {
        let channel: WorkerChannel<i32> = WorkerChannel::new();
        let (sender, receiver) = channel.split();

        assert!(sender.error("Something went wrong"));
        let msg = receiver.recv();
        assert!(matches!(msg, Some(WorkerMessage::Error(e)) if e == "Something went wrong"));
    }

    #[test]
    fn test_sender_check_command() {
        let channel: WorkerChannel<i32> = WorkerChannel::new();
        let (sender, receiver) = channel.split();

        receiver.send_command(WorkerCommand::Pause);
        let cmd = sender.check_command();
        assert!(matches!(cmd, Some(WorkerCommand::Pause)));
    }

    #[test]
    fn test_sender_is_cancelled() {
        let channel: WorkerChannel<i32> = WorkerChannel::new();
        let (sender, receiver) = channel.split();

        assert!(!sender.is_cancelled());
        receiver.cancel();
        assert!(sender.is_cancelled());
    }

    #[test]
    fn test_sender_clone() {
        let channel: WorkerChannel<i32> = WorkerChannel::new();
        let (sender, _receiver) = channel.split();

        let cloned = sender.clone();
        cloned.progress(0.5);
        // Both senders share the same queue
        assert_eq!(channel.message_count(), 1);
    }

    // =========================================================================
    // WorkerReceiver tests
    // =========================================================================

    #[test]
    fn test_receiver_recv_all() {
        let channel: WorkerChannel<i32> = WorkerChannel::new();
        let (sender, receiver) = channel.split();

        sender.progress(0.1);
        sender.progress(0.2);
        sender.progress(0.3);

        let messages = receiver.recv_all();
        assert_eq!(messages.len(), 3);
        assert!(receiver.recv_all().is_empty());
    }

    #[test]
    fn test_receiver_cancel() {
        let channel: WorkerChannel<i32> = WorkerChannel::new();
        let (sender, receiver) = channel.split();

        assert!(receiver.cancel());
        assert!(sender.is_cancelled());
    }

    #[test]
    fn test_receiver_pause() {
        let channel: WorkerChannel<i32> = WorkerChannel::new();
        let (sender, receiver) = channel.split();

        assert!(receiver.pause());
        let cmd = sender.check_command();
        assert!(matches!(cmd, Some(WorkerCommand::Pause)));
    }

    #[test]
    fn test_receiver_resume() {
        let channel: WorkerChannel<i32> = WorkerChannel::new();
        let (sender, receiver) = channel.split();

        assert!(receiver.resume());
        let cmd = sender.check_command();
        assert!(matches!(cmd, Some(WorkerCommand::Resume)));
    }

    #[test]
    fn test_receiver_has_messages() {
        let channel: WorkerChannel<i32> = WorkerChannel::new();
        let (sender, receiver) = channel.split();

        assert!(!receiver.has_messages());
        sender.complete(42);
        assert!(receiver.has_messages());
    }

    #[test]
    fn test_receiver_message_count() {
        let channel: WorkerChannel<i32> = WorkerChannel::new();
        let (sender, receiver) = channel.split();

        assert_eq!(receiver.message_count(), 0);
        sender.progress(0.1);
        sender.progress(0.2);
        assert_eq!(receiver.message_count(), 2);
    }

    #[test]
    fn test_receiver_clone() {
        let channel: WorkerChannel<i32> = WorkerChannel::new();
        let (sender, receiver) = channel.split();

        sender.complete(42);
        let cloned = receiver.clone();
        // Both receivers share the same queue
        let msg = cloned.recv();
        assert!(matches!(msg, Some(WorkerMessage::Complete(42))));
        assert!(receiver.recv().is_none());
    }

    // =========================================================================
    // Integration tests
    // =========================================================================

    #[test]
    fn test_bidirectional_communication() {
        let channel: WorkerChannel<String> = WorkerChannel::new();
        let (sender, receiver) = channel.split();

        // Worker sends progress
        sender.progress(0.25);
        sender.status("Started");

        // UI receives
        let _ = receiver.recv();
        let _ = receiver.recv();

        // UI sends command
        receiver.pause();

        // Worker receives command
        let cmd = sender.check_command();
        assert!(matches!(cmd, Some(WorkerCommand::Pause)));

        // Worker continues
        sender.complete("Done".to_string());

        let msg = receiver.recv();
        assert!(matches!(msg, Some(WorkerMessage::Complete(s)) if s == "Done"));
    }

    #[test]
    fn test_multiple_messages_fifo() {
        let channel: WorkerChannel<i32> = WorkerChannel::new();

        channel.send(WorkerMessage::Progress(0.1));
        channel.send(WorkerMessage::Progress(0.2));
        channel.send(WorkerMessage::Progress(0.3));

        // Should receive in FIFO order
        assert!(
            matches!(channel.recv(), Some(WorkerMessage::Progress(p)) if (p - 0.1).abs() < 0.01)
        );
        assert!(
            matches!(channel.recv(), Some(WorkerMessage::Progress(p)) if (p - 0.2).abs() < 0.01)
        );
        assert!(
            matches!(channel.recv(), Some(WorkerMessage::Progress(p)) if (p - 0.3).abs() < 0.01)
        );
    }
}
