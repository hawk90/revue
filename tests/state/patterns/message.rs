//! `MessageState::set_with_duration` overrides the duration for that one
//! message; the next `set` goes back to the state's own duration.

use revue::patterns::MessageState;
use std::time::Duration;

const LONG: Duration = Duration::from_secs(3600);

#[test]
fn the_override_applies_to_its_own_message() {
    let mut message = MessageState::new();
    message.set_with_duration("saving".to_string(), LONG);
    assert!(message.remaining().unwrap() > Duration::from_secs(3500));
}

#[test]
fn the_next_set_uses_the_state_duration_again() {
    let mut message = MessageState::with_duration(Duration::ZERO);
    message.set_with_duration("saving".to_string(), LONG);
    message.set("saved".to_string());

    assert!(
        message.check_timeout(),
        "the one-time override still applied to the next message"
    );
    assert_eq!(message.get(), None);
}

#[test]
fn an_override_shorter_than_the_default_expires_on_time() {
    let mut message = MessageState::with_duration(LONG);
    message.set_with_duration("flash".to_string(), Duration::ZERO);
    assert!(message.check_timeout());

    message.set("stays".to_string());
    assert!(!message.check_timeout());
    assert_eq!(message.get(), Some("stays"));
}
