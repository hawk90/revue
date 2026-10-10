//! The functions `revue::a11y` exports at its root
//!
//! They go through the process-wide backend, so every test is `#[serial]`.

use revue::a11y::{
    active_screen_reader, announce, announce_now, init, init_with, is_available, BackendType,
};
use serial_test::serial;

// =========================================================================
// Initialization tests
// =========================================================================

#[test]
#[serial]
fn test_init_returns_backend() {
    let backend = init();
    assert!(
        backend as *const _ as usize != 0,
        "init should return a valid reference"
    );
}

#[test]
#[serial]
fn test_init_with_logging() {
    let backend = init_with(BackendType::Logging);
    assert!(
        backend as *const _ as usize != 0,
        "init_with should return a valid reference"
    );
}

// =========================================================================
// Announcement tests
// =========================================================================

#[test]
#[serial]
fn test_announce_does_not_panic() {
    // Should not panic with various message types
    announce("Test message");
    announce("".to_string());
    announce("Message with unicode: 你好");
}

#[test]
#[serial]
fn test_announce_now_does_not_panic() {
    announce_now("Urgent message");
    announce_now("".to_string());
}

#[test]
#[serial]
fn test_announce_with_emoji() {
    announce("🎉 Success! 🎊");
    announce("⚠️ Warning ⚠️");
    announce("❌ Error ❌");
}

#[test]
#[serial]
fn test_announce_with_newlines() {
    announce("Line 1\nLine 2\nLine 3");
}

#[test]
#[serial]
fn test_announce_with_tabs() {
    announce("Tab\tseparated\ttext");
}

#[test]
#[serial]
fn test_announce_long_message() {
    let long_msg = "A".repeat(10000);
    announce(long_msg);
}

#[test]
#[serial]
fn test_announce_with_special_chars() {
    announce("Special: @#$%^&*()[]{}|\\:;\"'<>?,./");
}

#[test]
#[serial]
fn test_announce_now_with_unicode() {
    announce_now("🔔 Notification: 你好 🎊");
}

#[test]
#[serial]
fn test_announce_now_empty() {
    announce_now("");
}

#[test]
#[serial]
fn test_announce_now_with_numbers() {
    announce_now("12345");
    announce_now("99.9%");
}

// =========================================================================
// Availability tests
// =========================================================================

#[test]
#[serial]
fn test_is_available_returns_bool() {
    let result = is_available();
    // Should return a boolean value (true or false is fine)
    match result {
        true | false => {}
    }
}

#[test]
#[serial]
fn test_active_screen_reader_returns_option() {
    let result = active_screen_reader();
    // Should return Some(String) or None
    match result {
        Some(_) | None => {}
    }
}
