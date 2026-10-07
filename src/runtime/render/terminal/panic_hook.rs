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
//!
//! # Restoring exactly once
//!
//! With unwinding (the dev profile), a panic used to restore the terminal twice:
//! the hook first, then the `Drop` of the `Terminal` the unwind passes through.
//! The second restore is not harmless. `LeaveAlternateScreen` (`?1049l`) also
//! restores the cursor position saved on entry, so a second one moves the
//! cursor back above the panic message the hook just printed, and whatever is
//! written next - the shell prompt - overwrites it.
//!
//! So a TUI session is restored once. `ARMED` doubles as the session's "still
//! needs restoring" flag: every restore path - the hook, [`restore_terminal`],
//! `Terminal::restore` and `CrosstermBackend::restore` - goes through
//! `claim_restore`, and only the caller that flips it from `true` to `false`
//! writes the restore sequence.

use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Once;

use crossterm::{
    cursor::Show,
    event::{DisableBracketedPaste, DisableFocusChange, DisableMouseCapture},
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
            // A panic revue is about to catch leaves the app running, so the
            // terminal must stay in TUI mode. The chained hook is skipped too:
            // its message would land on the alternate screen, and the catcher
            // reports the panic itself.
            if panic_is_caught() && ARMED.load(Ordering::SeqCst) {
                crate::log_warn!("revue caught a panic: {}", info);
                return;
            }
            // Restore first: the default hook writes to stderr, and that message
            // is worthless if it lands on an alternate screen we are about to
            // tear down. Claiming the session also turns the unwinding `Drop`
            // that follows into a no-op, which keeps the message on screen.
            if claim_restore() {
                write_restore_sequence();
            }
            previous(info);
        }));
    });
}

thread_local! {
    /// How many [`catch_panic`] calls are on this thread's stack.
    static CATCHING: std::cell::Cell<u32> = const { std::cell::Cell::new(0) };
}

/// [`std::panic::catch_unwind`], for the places revue catches a panic and keeps
/// going: background tasks and workers that report a panic as an error, and
/// `ErrorBoundary` showing its fallback.
///
/// The panic hook runs *before* `catch_unwind` receives the panic, and on its
/// own it cannot tell a caught panic from a fatal one. Inside `catch_panic` it
/// knows: it leaves the terminal in TUI mode instead of restoring it under the
/// app that is still running. A panic the user's own `catch_unwind` catches
/// still restores the terminal - the hook cannot see that one.
pub(crate) fn catch_panic<F, R>(f: F) -> std::thread::Result<R>
where
    F: FnOnce() -> R + std::panic::UnwindSafe,
{
    struct Depth;
    impl Drop for Depth {
        fn drop(&mut self) {
            CATCHING.with(|c| c.set(c.get() - 1));
        }
    }

    CATCHING.with(|c| c.set(c.get() + 1));
    let _depth = Depth;
    std::panic::catch_unwind(f)
}

/// Is the current panic inside a [`catch_panic`] on this thread?
fn panic_is_caught() -> bool {
    // `try_with`: a panic during thread-local destruction must not panic again.
    CATCHING.try_with(|c| c.get() > 0).unwrap_or(false)
}

/// Claim the live TUI session for restoring, and disarm the hook.
///
/// Returns `true` to exactly one caller per session - the one that should write
/// the restore sequence. Every later caller gets `false`: the terminal is
/// already back to normal, and restoring it again would move the cursor back
/// over whatever was printed since (see the module docs).
///
/// After this, a later panic does not emit restore sequences either - the
/// process may be doing ordinary stdout work by then.
pub(crate) fn claim_restore() -> bool {
    ARMED.swap(false, Ordering::SeqCst)
}

/// Issue each restore command on its own and report the first error.
///
/// Chaining them through one `execute!` aborts the rest at the first failure,
/// and on a Windows console without VT processing crossterm dispatches to
/// WinAPI, where a command such as `DisableBracketedPaste` has no counterpart
/// and errors. A restore that stops there leaves the cursor hidden and the
/// alternate screen up - and since a session is restored once, nothing tries
/// again.
macro_rules! restore_each {
    ($writer:expr, $($command:expr),+ $(,)?) => {{
        $crate::runtime::render::terminal::panic_hook::note_restore_written();
        let mut first: ::std::io::Result<()> = Ok(());
        $(
            if let Err(error) = ::crossterm::execute!($writer, $command) {
                if first.is_ok() {
                    first = Err(error);
                }
            }
        )+
        first
    }};
}
pub(crate) use restore_each;

/// Record that a restore sequence is being written. Test builds count these per
/// thread; release builds compile it away.
#[inline]
pub(crate) fn note_restore_written() {
    #[cfg(test)]
    WRITES.with(|c| c.set(c.get() + 1));
}

#[cfg(test)]
thread_local! {
    /// Restore sequences written on this thread. Counting them, rather than
    /// the escape bytes in a writer, works on every platform: on Windows
    /// crossterm drives the console through WinAPI and writes no bytes a test
    /// could count.
    static WRITES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// How many restore sequences this thread has written so far.
#[cfg(test)]
pub(crate) fn restores_performed() -> usize {
    WRITES.with(|c| c.get())
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
/// It ends the current TUI session: the panic hook and the `restore` (or
/// `Drop`) of the [`Terminal`](super::Terminal) or
/// [`CrosstermBackend`](crate::render::CrosstermBackend) that started the
/// session will not restore a second time. If you enter TUI mode again by hand
/// afterwards, call [`install_panic_hook`] again to start a new session.
///
/// Errors are deliberately ignored: this runs on the way out, and there is
/// nothing useful to do if the terminal will not take the bytes.
pub fn restore_terminal() {
    claim_restore();
    write_restore_sequence();
}

/// Write the restore sequence to stdout and leave raw mode, unconditionally.
fn write_restore_sequence() {
    let mut out = std::io::stdout();

    // Errors are dropped: this runs on the way out, with nowhere to report them.
    let _ = restore_each!(
        out,
        DisableMouseCapture,
        DisableBracketedPaste,
        DisableFocusChange,
        ResetColor,
        Show,
        LeaveAlternateScreen,
    );

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

    /// The hook decides by the catch depth, so it must be exact: inside a
    /// `catch_panic` (also nested, also after a caught panic) and zero outside.
    #[test]
    fn catch_panic_marks_exactly_its_own_extent() {
        assert!(!panic_is_caught());
        let inner = catch_panic(|| {
            assert!(panic_is_caught());
            let nested: std::thread::Result<()> = catch_panic(|| panic!("nested"));
            assert!(nested.is_err());
            assert!(panic_is_caught(), "the outer catch is still running");
            7
        });
        assert_eq!(inner.ok(), Some(7));
        assert!(!panic_is_caught(), "a caught panic left the depth raised");
    }

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
    fn install_arms_and_claim_disarms() {
        install_panic_hook();
        assert!(is_armed());

        assert!(claim_restore());
        assert!(!is_armed());

        // Re-arming works after a restore - an app may enter TUI mode again.
        install_panic_hook();
        assert!(is_armed());
        assert!(claim_restore());
    }

    /// One session, one restore: whoever claims it first restores, and every
    /// later restore path is told the terminal is already back to normal.
    #[test]
    #[serial]
    fn claim_restore_succeeds_once_per_session() {
        install_panic_hook();

        assert!(claim_restore(), "the first restore path must restore");
        assert!(
            !claim_restore(),
            "a second restore would leave the alternate screen twice"
        );
    }
}
