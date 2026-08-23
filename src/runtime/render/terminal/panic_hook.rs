//! Terminal restoration on panic (`INV-09`, `REV-SEC-005`).
//!
//! Both [`Terminal`](super::Terminal) and
//! [`CrosstermBackend`](crate::render::CrosstermBackend) implement `Drop`, and
//! `Drop` restores the terminal on a normal exit. That is not enough:
//!
//! - This crate's release profile sets `panic = "abort"` (`Cargo.toml`), so a
//!   panic in a release build aborts without unwinding and **no `Drop` runs**.
//! - Even when unwinding, a panic on a thread that does not own the terminal
//!   leaves the process alive with raw mode still on.
//!
//! A panic *hook* runs in both unwind and abort mode, before the process dies.
//! So the hook - not `Drop` - is what actually upholds the invariant "terminal
//! state is restored after a panic".
//!
//! The hook is installed automatically when a terminal enters TUI mode. It
//! chains to the previously installed hook, so the panic message still prints,
//! and it prints *after* the alternate screen has been left - which is the
//! whole point, since a message written to the alternate screen disappears with
//! it.

use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Once;

use crossterm::{
    cursor::Show,
    event::{DisableBracketedPaste, DisableFocusChange, DisableMouseCapture},
    execute,
    style::ResetColor,
    terminal::{disable_raw_mode, LeaveAlternateScreen},
};

/// Is a terminal currently in TUI mode?
///
/// Guards the hook so that a program which already left TUI mode - or which
/// links `revue` without ever entering it - does not get escape sequences
/// sprayed into its output when it panics.
static ARMED: AtomicBool = AtomicBool::new(false);

/// The hook is installed at most once per process.
static INSTALL: Once = Once::new();

/// Install the panic hook that restores terminal state, and arm it.
///
/// Called automatically by [`Terminal::init_with_mouse`](super::Terminal::init_with_mouse)
/// and by [`CrosstermBackend::init_with_mouse`](crate::render::Backend::init_with_mouse).
/// Call it yourself only if you drive the terminal through your own backend and
/// still want revue's restore-on-panic behavior.
///
/// Idempotent. The hook itself is installed once; every call re-arms it.
///
/// # Example
///
/// ```no_run
/// use revue::render::install_panic_hook;
///
/// // Custom backend that enabled raw mode by hand.
/// crossterm::terminal::enable_raw_mode().unwrap();
/// install_panic_hook();
/// ```
pub fn install_panic_hook() {
    ARMED.store(true, Ordering::SeqCst);

    INSTALL.call_once(|| {
        let previous = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            // Restore first: the default hook writes to stderr, and that message
            // is worthless if it lands on an alternate screen we are about to
            // tear down. `swap` also makes a double panic restore only once.
            if ARMED.swap(false, Ordering::SeqCst) {
                restore_terminal();
            }
            previous(info);
        }));
    });
}

/// Disarm the hook after a clean shutdown.
///
/// After this, a later panic will not emit restore sequences - the terminal is
/// already back to normal and the process may be doing ordinary stdout work.
pub(crate) fn disarm() {
    ARMED.store(false, Ordering::SeqCst);
}

/// Is the panic hook currently armed?
#[cfg(test)]
pub(crate) fn is_armed() -> bool {
    ARMED.load(Ordering::SeqCst)
}

/// Restore the terminal to a usable state, immediately and unconditionally.
///
/// This is what the panic hook runs. It is also safe to call directly - from a
/// signal handler's cleanup path, before shelling out to `$EDITOR`, or from a
/// custom panic hook of your own.
///
/// Every operation is a no-op when the corresponding mode was never enabled, so
/// calling this without a live TUI session is harmless, as is calling it twice.
/// Errors are deliberately ignored: this runs on the way out, and there is
/// nothing useful to do if the terminal will not take the bytes.
pub fn restore_terminal() {
    let mut out = std::io::stdout();

    // Each command is issued on its own and its error dropped. Chaining them
    // through one `execute!` would abort the rest of the restore at the first
    // failure - and on a Windows console without VT processing, crossterm
    // dispatches to WinAPI, where `DisableBracketedPaste` has no counterpart
    // and errors. Leaving the cursor hidden because an unrelated command was
    // unsupported is exactly the outcome this function exists to prevent.
    let _ = execute!(out, DisableMouseCapture);
    let _ = execute!(out, DisableBracketedPaste);
    let _ = execute!(out, DisableFocusChange);
    let _ = execute!(out, ResetColor);
    let _ = execute!(out, Show);
    let _ = execute!(out, LeaveAlternateScreen);

    let _ = out.flush();
    let _ = disable_raw_mode();
}

/// The restore sequence in its ANSI form.
///
/// [`restore_terminal`] goes through `execute!`, which on Windows may dispatch
/// to WinAPI instead of writing bytes. This renders the same commands as ANSI
/// unconditionally, so a test can assert on the exact sequence on any platform.
#[cfg(test)]
fn ansi_restore_sequence() -> String {
    use crossterm::Command;

    let mut s = String::new();
    let _ = DisableMouseCapture.write_ansi(&mut s);
    let _ = DisableBracketedPaste.write_ansi(&mut s);
    let _ = DisableFocusChange.write_ansi(&mut s);
    let _ = ResetColor.write_ansi(&mut s);
    let _ = Show.write_ansi(&mut s);
    let _ = LeaveAlternateScreen.write_ansi(&mut s);
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    /// The sequence must leave the alternate screen and show the cursor - the
    /// two things whose absence makes a terminal look broken after a crash.
    #[test]
    fn restore_sequence_leaves_alternate_screen_and_shows_cursor() {
        let s = ansi_restore_sequence();

        assert!(
            s.contains("\x1b[?1049l"),
            "must leave alternate screen: {s:?}"
        );
        assert!(s.contains("\x1b[?25h"), "must show the cursor: {s:?}");
    }

    /// Mouse, bracketed paste and focus reporting all leak into the shell as
    /// garbage input if they survive the crash.
    #[test]
    fn restore_sequence_disables_input_modes() {
        let s = ansi_restore_sequence();

        assert!(
            s.contains("\x1b[?1000l"),
            "must disable mouse capture: {s:?}"
        );
        assert!(
            s.contains("\x1b[?2004l"),
            "must disable bracketed paste: {s:?}"
        );
        assert!(
            s.contains("\x1b[?1004l"),
            "must disable focus change: {s:?}"
        );
    }

    /// Leaving the alternate screen must come last, so the sequences that undo
    /// input modes are still interpreted by the alternate screen's terminal
    /// state rather than the restored one.
    #[test]
    fn restore_sequence_leaves_the_screen_last() {
        let s = ansi_restore_sequence();

        let leave = s.find("\x1b[?1049l").unwrap();
        assert!(s.find("\x1b[?1000l").unwrap() < leave);
        assert!(s.find("\x1b[?25h").unwrap() < leave);
    }

    #[test]
    #[serial]
    fn install_arms_and_disarm_clears() {
        install_panic_hook();
        assert!(is_armed());

        disarm();
        assert!(!is_armed());

        // Re-arming works after a disarm - an app may enter TUI mode again.
        install_panic_hook();
        assert!(is_armed());
        disarm();
    }
}
