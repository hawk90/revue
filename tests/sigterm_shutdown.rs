//! `App::run` shuts down cleanly on `SIGTERM`.
//!
//! A real `App::run` needs a real terminal, so the test runs one on a
//! pseudo-terminal: it re-executes this test binary as a child whose stdin,
//! stdout and stderr are the PTY's slave side, waits for the child to enter the
//! alternate screen, sends `SIGTERM`, and then checks what a user would see -
//! the process exited successfully, plugins unmounted, and the terminal was
//! restored exactly once.
#![cfg(unix)]

use std::fs::{File, OpenOptions};
use std::io::Read;
use std::os::fd::{AsRawFd, FromRawFd};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::time::{Duration, Instant};

use revue::plugin::{Plugin, PluginContext};
use revue::prelude::*;

/// Set in the child: run the app instead of the test.
const CHILD_ENV: &str = "REVUE_SIGTERM_CHILD";
/// Where the child's plugin records that `on_unmount` ran.
const MARKER_ENV: &str = "REVUE_SIGTERM_MARKER";

const ENTER_ALT_SCREEN: &str = "\x1b[?1049h";
const LEAVE_ALT_SCREEN: &str = "\x1b[?1049l";

struct UnmountMarker;

impl Plugin for UnmountMarker {
    fn name(&self) -> &str {
        "unmount-marker"
    }

    fn on_unmount(&mut self, _ctx: &mut PluginContext) -> revue::Result<()> {
        let path = std::env::var(MARKER_ENV).expect("marker path");
        std::fs::write(path, "unmounted").expect("write marker");
        Ok(())
    }
}

/// The child side: an ordinary app that only stops when told to.
fn run_child_app() {
    App::builder()
        .plugin(UnmountMarker)
        .build()
        .run(Text::new("waiting for SIGTERM"), |_, _, _| false)
        .expect("App::run returns Ok after SIGTERM");
}

/// Open an 80x24 PTY pair: the master end and the slave end.
fn open_pty() -> (File, File) {
    // SAFETY: plain libc calls on descriptors this function owns; `ptsname`
    // returns a NUL-terminated string valid until the next call, copied out
    // immediately (nothing else in this test binary calls it).
    let (master, slave_path) = unsafe {
        let master = libc::posix_openpt(libc::O_RDWR | libc::O_NOCTTY);
        assert!(master >= 0, "posix_openpt failed");
        assert_eq!(libc::grantpt(master), 0, "grantpt failed");
        assert_eq!(libc::unlockpt(master), 0, "unlockpt failed");

        let name = libc::ptsname(master);
        assert!(!name.is_null(), "ptsname failed");
        let slave = std::ffi::CStr::from_ptr(name).to_str().unwrap().to_owned();

        (File::from_raw_fd(master), PathBuf::from(slave))
    };

    let slave = OpenOptions::new()
        .read(true)
        .write(true)
        .open(&slave_path)
        .expect("open PTY slave");
    let size = libc::winsize {
        ws_row: 24,
        ws_col: 80,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    // SAFETY: TIOCSWINSZ reads a `winsize` from a valid pointer.
    assert_eq!(
        unsafe { libc::ioctl(slave.as_raw_fd(), libc::TIOCSWINSZ, &size) },
        0,
        "TIOCSWINSZ failed"
    );

    (master, slave)
}

#[test]
fn sigterm_unmounts_plugins_and_restores_the_terminal_once() {
    if std::env::var_os(CHILD_ENV).is_some() {
        run_child_app();
        return;
    }

    let (mut master, slave_end) = open_pty();
    let slave = || Stdio::from(slave_end.try_clone().expect("dup PTY slave"));
    let marker = std::env::temp_dir().join(format!("revue-sigterm-{}", std::process::id()));
    let _ = std::fs::remove_file(&marker);

    let mut child = Command::new(std::env::current_exe().unwrap())
        .args([
            "sigterm_unmounts_plugins_and_restores_the_terminal_once",
            "--exact",
            "--nocapture",
            "--test-threads=1",
        ])
        .env(CHILD_ENV, "1")
        .env(MARKER_ENV, &marker)
        .stdin(slave())
        .stdout(slave())
        .stderr(slave())
        .spawn()
        .expect("spawn child");
    // Only the child may hold the slave open, or the master never sees EOF.
    drop(slave_end);

    // Drain the master in the background so the child never blocks on a full
    // PTY buffer; the reader ends with EIO/EOF once the child is gone.
    let (tx, rx) = mpsc::channel::<Vec<u8>>();
    std::thread::spawn(move || {
        let mut buf = [0u8; 4096];
        while let Ok(n) = master.read(&mut buf) {
            if n == 0 || tx.send(buf[..n].to_vec()).is_err() {
                break;
            }
        }
    });

    let mut output = Vec::new();
    let collect = |output: &mut Vec<u8>, until: Instant| {
        while let Ok(chunk) = rx.recv_timeout(until.saturating_duration_since(Instant::now())) {
            output.extend(chunk);
            if Instant::now() >= until {
                break;
            }
        }
    };

    // Wait until the app is in TUI mode.
    let deadline = Instant::now() + Duration::from_secs(20);
    while !String::from_utf8_lossy(&output).contains(ENTER_ALT_SCREEN) {
        assert!(Instant::now() < deadline, "child never entered TUI mode");
        if let Ok(chunk) = rx.recv_timeout(Duration::from_millis(100)) {
            output.extend(chunk);
        }
    }

    // SAFETY: `kill` on our own child's pid.
    assert_eq!(
        unsafe { libc::kill(child.id() as libc::pid_t, libc::SIGTERM) },
        0
    );

    let deadline = Instant::now() + Duration::from_secs(10);
    let status = loop {
        if let Some(status) = child.try_wait().unwrap() {
            break status;
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            panic!("child did not exit after SIGTERM");
        }
        std::thread::sleep(Duration::from_millis(20));
    };
    collect(&mut output, Instant::now() + Duration::from_millis(300));
    let text = String::from_utf8_lossy(&output);

    assert!(
        status.success(),
        "SIGTERM should end the app like a quit, got {status:?}; output: {text:?}"
    );
    assert_eq!(
        std::fs::read_to_string(&marker).ok().as_deref(),
        Some("unmounted"),
        "plugin on_unmount did not run"
    );
    let _ = std::fs::remove_file(&marker);
    assert_eq!(
        text.matches(LEAVE_ALT_SCREEN).count(),
        1,
        "the terminal must be restored exactly once: {text:?}"
    );
}
