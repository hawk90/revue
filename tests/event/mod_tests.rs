//! Event module tests

use revue::event::{Event, EventReader, Key, KeyEvent};
use revue::layout::Rect;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};
use std::time::Duration;

#[test]
fn test_key_event_new() {
    let event = KeyEvent::new(Key::Enter);
    assert_eq!(event.key, Key::Enter);
    assert!(!event.ctrl);
    assert!(!event.alt);
    assert!(!event.shift);
}

#[test]
fn test_key_event_ctrl() {
    let event = KeyEvent::ctrl(Key::Char('c'));
    assert_eq!(event.key, Key::Char('c'));
    assert!(event.ctrl);
    assert!(event.is_ctrl_c());
}

#[test]
fn test_key_event_checks() {
    assert!(KeyEvent::new(Key::Escape).is_escape());
    assert!(KeyEvent::new(Key::Enter).is_enter());
    assert!(KeyEvent::new(Key::Tab).is_tab());

    let shift_tab = KeyEvent {
        key: Key::Tab,
        ctrl: false,
        alt: false,
        shift: true,
    };
    assert!(shift_tab.is_shift_tab());
    assert!(!shift_tab.is_tab());
}

#[test]
fn test_key_event_to_binding() {
    let event = KeyEvent::ctrl(Key::Char('s'));
    let binding = event.to_binding();

    assert_eq!(binding.key, Key::Char('s'));
    assert!(binding.ctrl);
}

#[test]
fn test_event_focus_gained() {
    let event = Event::FocusGained;
    assert!(matches!(event, Event::FocusGained));
}

#[test]
fn test_event_focus_lost() {
    let event = Event::FocusLost;
    assert!(matches!(event, Event::FocusLost));
}

#[test]
fn test_event_paste() {
    let event = Event::Paste("hello world".to_string());
    if let Event::Paste(text) = event {
        assert_eq!(text, "hello world");
    } else {
        panic!("Expected Paste event");
    }
}

#[test]
fn test_event_variants_equality() {
    assert_eq!(Event::FocusGained, Event::FocusGained);
    assert_eq!(Event::FocusLost, Event::FocusLost);
    assert_eq!(
        Event::Paste("test".to_string()),
        Event::Paste("test".to_string())
    );
    assert_ne!(Event::FocusGained, Event::FocusLost);
    assert_ne!(Event::Paste("a".to_string()), Event::Paste("b".to_string()));
}
