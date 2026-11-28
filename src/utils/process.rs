//! Starting external programs while the terminal UI is on screen
//!
//! An external program (an opener, a speech command) must not inherit the
//! app's stdio, or what it prints lands on top of the terminal UI, and it
//! must be waited on, or it stays a zombie until revue exits (#937, #1551).

use std::process::{Command, ExitStatus, Stdio};
use std::thread::JoinHandle;

/// Start `cmd` with null stdin, stdout and stderr, and wait for it on its
/// own thread so it leaves no zombie behind; the handle yields the exit
/// status.
///
/// Returns the spawn error when the program cannot be started.
pub(crate) fn spawn_reaped(
    mut cmd: Command,
) -> std::io::Result<JoinHandle<std::io::Result<ExitStatus>>> {
    let mut child = cmd
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    std::thread::Builder::new()
        .name("revue-reaper".to_string())
        .spawn(move || child.wait())
}

// KEEP HERE - `spawn_reaped` is crate-private

#[cfg(test)]
mod tests {
    use super::*;

    /// The child gets null stdin, stdout and stderr, and the returned
    /// thread reaps it: joining yields its exit status
    #[test]
    #[cfg(unix)]
    fn test_spawn_reaped_nulls_stdio_and_waits() {
        let mut cmd = Command::new("sh");
        cmd.arg("-c").arg(
            "[ /dev/stdin -ef /dev/null ] && [ /dev/stdout -ef /dev/null ] \
             && [ /dev/stderr -ef /dev/null ] && exit 7",
        );
        let status = spawn_reaped(cmd).unwrap().join().unwrap().unwrap();
        assert_eq!(status.code(), Some(7));
    }

    #[test]
    #[cfg(windows)]
    fn test_spawn_reaped_waits() {
        let mut cmd = Command::new("cmd");
        cmd.args(["/C", "exit 7"]);
        let status = spawn_reaped(cmd).unwrap().join().unwrap().unwrap();
        assert_eq!(status.code(), Some(7));
    }

    #[test]
    fn test_spawn_reaped_reports_a_missing_program() {
        let cmd = Command::new("revue-no-such-program-937");
        assert!(spawn_reaped(cmd).is_err());
    }
}
