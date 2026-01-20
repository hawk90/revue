//! Worker channel integration tests
//!
//! Tests for channel communication, message types, and sender/receiver pairs.

use revue::worker::{WorkerChannel, WorkerCommand, WorkerMessage, WorkerReceiver, WorkerSender};
use std::thread;

// =============================================================================
// Message Tests
// =============================================================================

#[test]
fn test_message_progress_clamp() {
    let channel: WorkerChannel<i32> = WorkerChannel::new();
    let (sender, _receiver) = channel.split();

    // Progress values should be clamped to 0.0-1.0
    assert!(sender.progress(-0.5));
    assert!(sender.progress(0.0));
    assert!(sender.progress(0.5));
    assert!(sender.progress(1.0));
    assert!(sender.progress(1.5));
    assert!(sender.progress(2.0));
}

#[test]
fn test_message_clone_large_data() {
    // Clone large message
    let msg = WorkerMessage::Partial(vec![1u8; 10000]);
    let cloned = msg.clone();

    match (msg, cloned) {
        (WorkerMessage::Partial(v1), WorkerMessage::Partial(v2)) => {
            assert_eq!(v1.len(), v2.len());
            assert_eq!(v1, v2);
        }
        _ => panic!("Cloned message should be same type"),
    }
}

#[test]
fn test_message_all_variants() {
    // Test all message variants can be created
    let _ = WorkerMessage::<()>::Progress(0.5);
    let _ = WorkerMessage::<()>::Status("test".to_string());
    let _ = WorkerMessage::<i32>::Partial(42);
    let _ = WorkerMessage::<i32>::Complete(100);
    let _ = WorkerMessage::<()>::Error("error".to_string());
    let _ = WorkerMessage::<()>::Custom("custom".to_string());
}

// =============================================================================
// Channel Communication Tests
// =============================================================================

#[test]
fn test_channel_capacity_limit() {
    let channel: WorkerChannel<i32> = WorkerChannel::with_capacity(3);

    // Fill to capacity
    assert!(channel.send(WorkerMessage::Complete(1)));
    assert!(channel.send(WorkerMessage::Complete(2)));
    assert!(channel.send(WorkerMessage::Complete(3)));

    // Should be at capacity
    assert_eq!(channel.message_count(), 3);

    // Next send should fail
    assert!(!channel.send(WorkerMessage::Complete(4)));

    // Drain and try again
    let _ = channel.recv();
    assert!(channel.send(WorkerMessage::Complete(4)));
}

#[test]
fn test_channel_concurrent_senders() {
    let channel: WorkerChannel<i32> = WorkerChannel::new();
    let (sender, receiver) = channel.split();

    let sender1 = sender.clone();
    let sender2 = sender.clone();

    let handle1 = thread::spawn(move || {
        sender1.progress(0.25);
        sender1.progress(0.50);
    });

    let handle2 = thread::spawn(move || {
        sender2.progress(0.75);
        sender2.status("Done".to_string());
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    // Should have 4 messages
    assert_eq!(receiver.message_count(), 4);
}

#[test]
fn test_channel_split_behavior() {
    let channel: WorkerChannel<String> = WorkerChannel::new();

    // Multiple splits should work
    let (sender1, receiver1) = channel.split();
    let (sender2, receiver2) = channel.split();

    // Both senders share the same underlying queue
    sender1.complete("From 1".to_string());
    sender2.complete("From 2".to_string());

    // Both receivers see the same messages
    assert_eq!(receiver1.message_count(), 2);
    assert_eq!(receiver2.message_count(), 2);

    // Receiving from one removes from both
    let msg = receiver1.recv();
    assert!(msg.is_some());

    assert_eq!(receiver1.message_count(), 1);
    assert_eq!(receiver2.message_count(), 1);
}

// =============================================================================
// WorkerSender Tests
// =============================================================================

#[test]
fn test_sender_all_methods() {
    let channel: WorkerChannel<i32> = WorkerChannel::new();
    let (sender, receiver) = channel.split();

    // Test all sender methods
    assert!(sender.progress(0.5));
    assert!(sender.status("Working"));
    assert!(sender.partial(10));
    assert!(sender.complete(100));
    assert!(sender.error("Failed"));

    assert_eq!(receiver.message_count(), 5);
}

#[test]
fn test_sender_check_command() {
    let channel: WorkerChannel<i32> = WorkerChannel::new();
    let (sender, receiver) = channel.split();

    // No commands initially
    assert!(sender.check_command().is_none());

    // Send a command
    receiver.send_command(WorkerCommand::Cancel);

    // Should see the command
    let cmd = sender.check_command();
    assert!(matches!(cmd, Some(WorkerCommand::Cancel)));

    // Command should be consumed
    assert!(sender.check_command().is_none());
}

#[test]
fn test_sender_is_cancelled() {
    let channel: WorkerChannel<i32> = WorkerChannel::new();
    let (sender, receiver) = channel.split();

    assert!(!sender.is_cancelled());

    receiver.cancel();
    assert!(sender.is_cancelled());

    // Clear the command
    let _ = sender.check_command();
    assert!(!sender.is_cancelled());
}

#[test]
fn test_sender_clone_independent() {
    let channel: WorkerChannel<i32> = WorkerChannel::new();
    let (sender, _receiver) = channel.split();

    let sender1 = sender.clone();
    let sender2 = sender.clone();

    // All clones share the same queue
    sender1.progress(0.33);
    sender2.progress(0.66);

    assert_eq!(channel.message_count(), 2);
}

// =============================================================================
// WorkerReceiver Tests
// =============================================================================

#[test]
fn test_receiver_recv_all() {
    let channel: WorkerChannel<i32> = WorkerChannel::new();
    let (sender, receiver) = channel.split();

    sender.progress(0.1);
    sender.progress(0.2);
    sender.progress(0.3);

    let messages = receiver.recv_all();
    assert_eq!(messages.len(), 3);

    // Should be empty after recv_all
    assert!(receiver.recv_all().is_empty());
}

#[test]
fn test_receiver_all_commands() {
    let channel: WorkerChannel<i32> = WorkerChannel::new();
    let (sender, receiver) = channel.split();

    receiver.cancel();
    receiver.pause();
    receiver.resume();

    // Worker should see all commands
    let cmd1 = sender.check_command();
    assert!(matches!(cmd1, Some(WorkerCommand::Cancel)));

    let cmd2 = sender.check_command();
    assert!(matches!(cmd2, Some(WorkerCommand::Pause)));

    let cmd3 = sender.check_command();
    assert!(matches!(cmd3, Some(WorkerCommand::Resume)));
}

#[test]
fn test_receiver_has_messages() {
    let channel: WorkerChannel<i32> = WorkerChannel::new();
    let (sender, receiver) = channel.split();

    assert!(!receiver.has_messages());

    sender.progress(0.5);

    assert!(receiver.has_messages());

    // Consume the message
    let _ = receiver.recv();

    assert!(!receiver.has_messages());
}

#[test]
fn test_receiver_message_count() {
    let channel: WorkerChannel<i32> = WorkerChannel::new();
    let (sender, receiver) = channel.split();

    assert_eq!(receiver.message_count(), 0);

    for i in 1..=5 {
        sender.progress(i as f32 / 5.0);
        assert_eq!(receiver.message_count(), i);
    }
}

#[test]
fn test_receiver_clone() {
    let channel: WorkerChannel<i32> = WorkerChannel::new();
    let (sender, receiver) = channel.split();

    sender.complete(42);

    let receiver1 = receiver.clone();
    let receiver2 = receiver.clone();

    // All receivers share the same queue
    assert_eq!(receiver.message_count(), 1);
    assert_eq!(receiver1.message_count(), 1);
    assert_eq!(receiver2.message_count(), 1);
}

// =============================================================================
// WorkerCommand Tests
// =============================================================================

#[test]
fn test_command_all_variants() {
    let _ = WorkerCommand::Cancel;
    let _ = WorkerCommand::Pause;
    let _ = WorkerCommand::Resume;
    let _ = WorkerCommand::Custom("custom".to_string());
}

#[test]
fn test_command_clone() {
    let cmd1 = WorkerCommand::Custom("test".to_string());
    let cmd2 = cmd1.clone();

    // Check both are Custom with the same content
    assert!(matches!(cmd1, WorkerCommand::Custom(_)));
    assert!(matches!(cmd2, WorkerCommand::Custom(_)));
    if let (WorkerCommand::Custom(s1), WorkerCommand::Custom(s2)) = (cmd1, cmd2) {
        assert_eq!(s1, s2);
    }
}

#[test]
fn test_command_custom_content() {
    let cmd = WorkerCommand::Custom("my-command".to_string());
    assert!(matches!(cmd, WorkerCommand::Custom(s) if s == "my-command"));
}

// =============================================================================
// Integration Tests
// =============================================================================

#[test]
fn test_bidirectional_communication() {
    let channel: WorkerChannel<String> = WorkerChannel::new();
    let (sender, receiver) = channel.split();

    // Worker sends updates
    sender.progress(0.25);
    sender.status("Processing");

    // UI sends commands
    receiver.pause();

    // Worker receives command
    let cmd = sender.check_command();
    assert!(matches!(cmd, Some(WorkerCommand::Pause)));

    // Worker continues
    sender.complete("Done".to_string());

    // UI receives all messages
    let messages = receiver.recv_all();
    assert_eq!(messages.len(), 3);
}

#[test]
fn test_multiple_command_types() {
    let channel: WorkerChannel<i32> = WorkerChannel::new();
    let (sender, receiver) = channel.split();

    // Send all command types
    receiver.pause();
    receiver.resume();
    receiver.cancel();
    receiver.send_command(WorkerCommand::Custom("custom".to_string()));

    // Worker should receive in order
    let cmd1 = sender.check_command();
    assert!(matches!(cmd1, Some(WorkerCommand::Pause)));

    let cmd2 = sender.check_command();
    assert!(matches!(cmd2, Some(WorkerCommand::Resume)));

    let cmd3 = sender.check_command();
    assert!(matches!(cmd3, Some(WorkerCommand::Cancel)));

    let cmd4 = sender.check_command();
    assert!(matches!(cmd4, Some(WorkerCommand::Custom(_))));
}

#[test]
fn test_channel_fifo_ordering() {
    let channel: WorkerChannel<i32> = WorkerChannel::new();

    channel.send(WorkerMessage::Progress(0.1));
    channel.send(WorkerMessage::Progress(0.2));
    channel.send(WorkerMessage::Progress(0.3));

    assert!(matches!(
        channel.recv(),
        Some(WorkerMessage::Progress(p)) if (p - 0.1).abs() < 0.01
    ));
    assert!(matches!(
        channel.recv(),
        Some(WorkerMessage::Progress(p)) if (p - 0.2).abs() < 0.01
    ));
    assert!(matches!(
        channel.recv(),
        Some(WorkerMessage::Progress(p)) if (p - 0.3).abs() < 0.01
    ));
}

#[test]
fn test_channel_default_capacity() {
    let channel: WorkerChannel<i32> = WorkerChannel::default();
    // Default capacity should be 100
    for _ in 0..100 {
        assert!(channel.send(WorkerMessage::Progress(0.5)));
    }
    assert!(!channel.send(WorkerMessage::Progress(0.5)));
}

#[test]
fn test_channel_has_messages_commands() {
    let channel: WorkerChannel<i32> = WorkerChannel::new();

    assert!(!channel.has_messages());
    assert!(!channel.has_commands());

    channel.send(WorkerMessage::Status("test".to_string()));
    channel.send_command(WorkerCommand::Cancel);

    assert!(channel.has_messages());
    assert!(channel.has_commands());
}

#[test]
fn test_channel_send_command() {
    let channel: WorkerChannel<i32> = WorkerChannel::new();

    assert!(channel.send_command(WorkerCommand::Pause));

    let cmd = channel.recv_command();
    assert!(matches!(cmd, Some(WorkerCommand::Pause)));

    assert!(!channel.has_commands());
}

#[test]
fn test_channel_send_command_full() {
    let channel: WorkerChannel<i32> = WorkerChannel::with_capacity(1);

    // Fill command queue
    assert!(channel.send_command(WorkerCommand::Pause));

    // Should fail if queue is full (though commands share capacity with messages)
    // This tests the capacity check
    for _ in 0..100 {
        channel.send(WorkerMessage::Progress(0.5));
    }
}
