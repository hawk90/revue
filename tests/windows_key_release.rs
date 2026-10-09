//! On Windows, crossterm reports a key release after each press. The reader
//! must hand the app one key event per press, not two (#869).
//!
//! This writes a real press and release of `a` into the console input
//! buffer and reads them back through crossterm.

#![cfg(windows)]

use revue::event::{Event, EventReader, Key};
use std::time::{Duration, Instant};
use windows_sys::Win32::Foundation::{GENERIC_READ, GENERIC_WRITE, HANDLE, INVALID_HANDLE_VALUE};
use windows_sys::Win32::Storage::FileSystem::{
    CreateFileW, FILE_SHARE_READ, FILE_SHARE_WRITE, OPEN_EXISTING,
};
use windows_sys::Win32::System::Console::{
    AllocConsole, WriteConsoleInputW, INPUT_RECORD, INPUT_RECORD_0, KEY_EVENT, KEY_EVENT_RECORD,
    KEY_EVENT_RECORD_0,
};

/// The console input buffer crossterm reads from, creating a console if the
/// test process has none (as on a CI runner).
fn console_input() -> HANDLE {
    let open = || {
        let name: Vec<u16> = "CONIN$\0".encode_utf16().collect();
        // SAFETY: `name` is a NUL-terminated UTF-16 string that outlives the call.
        unsafe {
            CreateFileW(
                name.as_ptr(),
                GENERIC_READ | GENERIC_WRITE,
                FILE_SHARE_READ | FILE_SHARE_WRITE,
                std::ptr::null(),
                OPEN_EXISTING,
                0,
                std::ptr::null_mut(),
            )
        }
    };
    let mut handle = open();
    if handle == INVALID_HANDLE_VALUE {
        // SAFETY: no arguments; fails harmlessly if a console already exists.
        unsafe { AllocConsole() };
        handle = open();
    }
    assert_ne!(handle, INVALID_HANDLE_VALUE, "no console input buffer");
    handle
}

fn key_record(down: bool) -> INPUT_RECORD {
    INPUT_RECORD {
        EventType: KEY_EVENT as u16,
        Event: INPUT_RECORD_0 {
            KeyEvent: KEY_EVENT_RECORD {
                bKeyDown: i32::from(down),
                wRepeatCount: 1,
                wVirtualKeyCode: 0x41, // VK_A
                wVirtualScanCode: 0x1E,
                uChar: KEY_EVENT_RECORD_0 {
                    UnicodeChar: u16::from(b'a'),
                },
                dwControlKeyState: 0,
            },
        },
    }
}

#[test]
fn a_press_and_release_give_one_key_event() {
    let input = console_input();
    let records = [key_record(true), key_record(false)];
    let mut written = 0;
    // SAFETY: `records` holds `records.len()` initialized records, and
    // `written` is a valid out pointer.
    let ok = unsafe { WriteConsoleInputW(input, records.as_ptr(), 2, &mut written) };
    assert!(ok != 0 && written == 2, "could not write console input");

    let reader = EventReader::new(Duration::from_millis(50));
    let mut keys = Vec::new();
    let deadline = Instant::now() + Duration::from_secs(2);
    while Instant::now() < deadline {
        match reader.read().expect("read event") {
            Event::Key(key) => keys.push(key.key),
            Event::Tick if !keys.is_empty() => break,
            _ => {}
        }
    }

    assert_eq!(keys, vec![Key::Char('a')]);
}
