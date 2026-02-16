//! AIStream enum trait tests

use revue::widget::developer::{TypingStyle, StreamCursor, StreamStatus};

// =========================================================================
// TypingStyle enum trait tests
// =========================================================================

#[test]
fn test_typing_style_default() {
    assert_eq!(TypingStyle::default(), TypingStyle::Character);
}

#[test]
fn test_typing_style_clone() {
    let style = TypingStyle::Word;
    assert_eq!(style, style.clone());
}

#[test]
fn test_typing_style_copy() {
    let style1 = TypingStyle::Line;
    let style2 = style1;
    assert_eq!(style1, TypingStyle::Line);
    assert_eq!(style2, TypingStyle::Line);
}

#[test]
fn test_typing_style_equality() {
    assert_eq!(TypingStyle::None, TypingStyle::None);
    assert_eq!(TypingStyle::Character, TypingStyle::Character);
    assert_ne!(TypingStyle::None, TypingStyle::Word);
}

#[test]
fn test_typing_style_debug() {
    let debug_str = format!("{:?}", TypingStyle::Chunk);
    assert!(debug_str.contains("Chunk"));
}

// =========================================================================
// StreamCursor enum trait tests
// =========================================================================

#[test]
fn test_stream_cursor_default() {
    assert_eq!(StreamCursor::default(), StreamCursor::Block);
}

#[test]
fn test_stream_cursor_clone() {
    let cursor = StreamCursor::Bar;
    assert_eq!(cursor, cursor.clone());
}

#[test]
fn test_stream_cursor_copy() {
    let cursor1 = StreamCursor::Underline;
    let cursor2 = cursor1;
    assert_eq!(cursor1, StreamCursor::Underline);
    assert_eq!(cursor2, StreamCursor::Underline);
}

#[test]
fn test_stream_cursor_equality() {
    assert_eq!(StreamCursor::Block, StreamCursor::Block);
    assert_eq!(StreamCursor::None, StreamCursor::None);
    assert_ne!(StreamCursor::Bar, StreamCursor::Underline);
}

#[test]
fn test_stream_cursor_debug() {
    let debug_str = format!("{:?}", StreamCursor::None);
    assert!(debug_str.contains("None"));
}

// =========================================================================
// StreamStatus enum trait tests
// =========================================================================

#[test]
fn test_stream_status_default() {
    assert_eq!(StreamStatus::default(), StreamStatus::Idle);
}

#[test]
fn test_stream_status_clone() {
    let status = StreamStatus::Paused;
    assert_eq!(status, status.clone());
}

#[test]
fn test_stream_status_copy() {
    let status1 = StreamStatus::Complete;
    let status2 = status1;
    assert_eq!(status1, StreamStatus::Complete);
    assert_eq!(status2, StreamStatus::Complete);
}

#[test]
fn test_stream_status_equality() {
    assert_eq!(StreamStatus::Idle, StreamStatus::Idle);
    assert_eq!(StreamStatus::Streaming, StreamStatus::Streaming);
    assert_ne!(StreamStatus::Error, StreamStatus::Complete);
}

#[test]
fn test_stream_status_debug() {
    let debug_str = format!("{:?}", StreamStatus::Streaming);
    assert!(debug_str.contains("Streaming"));
}
