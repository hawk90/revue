//! Character wrapping when a wide character is wider than the room left

use revue::text::{wrap_chars, TextWrapper, WrapMode};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// Run `f` on another thread and fail instead of hanging if it does not return
fn within_a_second<T: Send + 'static>(f: impl FnOnce() -> T + Send + 'static) -> T {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let _ = tx.send(f());
    });
    rx.recv_timeout(Duration::from_secs(1))
        .expect("wrapping did not finish")
}

#[test]
fn a_wide_char_wider_than_the_width_gets_a_line_of_its_own() {
    let lines = within_a_second(|| wrap_chars("中文", 1));
    assert_eq!(lines, vec!["中", "文"]);
}

#[test]
fn a_wide_char_wider_than_the_room_after_the_indent_is_kept() {
    let lines = within_a_second(|| {
        TextWrapper::new(4)
            .mode(WrapMode::Char)
            .subsequent_indent("   ")
            .wrap("中文中文")
    });
    assert_eq!(lines, vec!["中文", "   中", "   文"]);
}
