//! Clipboard utilities tests

use revue::utils::clipboard::{Clipboard, ClipboardBackend, ClipboardHistory, MemoryClipboard};

#[test]
fn test_memory_clipboard() {
    let clipboard = MemoryClipboard::new();

    assert!(!clipboard.has_text().unwrap());

    clipboard.set("Hello").unwrap();
    assert!(clipboard.has_text().unwrap());
    assert_eq!(clipboard.get().unwrap(), "Hello");

    clipboard.clear().unwrap();
    assert!(!clipboard.has_text().unwrap());
}

#[test]
fn test_clipboard_with_memory_backend() {
    let clipboard = Clipboard::memory();

    clipboard.set("Test content").unwrap();
    assert_eq!(clipboard.get().unwrap(), "Test content");
}

#[test]
fn test_clipboard_history() {
    let mut history = ClipboardHistory::new(5);

    history.push("first".to_string());
    history.push("second".to_string());
    history.push("third".to_string());

    assert_eq!(history.latest(), Some("third"));
    assert_eq!(history.get(1), Some("second"));
    assert_eq!(history.get(2), Some("first"));
    assert_eq!(history.len(), 3);
}

#[test]
fn test_clipboard_history_no_duplicates() {
    let mut history = ClipboardHistory::new(5);

    history.push("a".to_string());
    history.push("b".to_string());
    history.push("a".to_string()); // Should move to top

    assert_eq!(history.len(), 2);
    assert_eq!(history.latest(), Some("a"));
    assert_eq!(history.get(1), Some("b"));
}

#[test]
fn test_clipboard_history_max_size() {
    let mut history = ClipboardHistory::new(3);

    history.push("1".to_string());
    history.push("2".to_string());
    history.push("3".to_string());
    history.push("4".to_string()); // Should evict "1"

    assert_eq!(history.len(), 3);
    assert_eq!(history.latest(), Some("4"));
    assert_eq!(history.get(2), Some("2"));
}

#[test]
fn test_clipboard_history_no_duplicate_at_top() {
    let mut history = ClipboardHistory::new(5);

    history.push("same".to_string());
    history.push("same".to_string()); // Should be ignored

    assert_eq!(history.len(), 1);
}
