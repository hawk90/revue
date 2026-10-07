//! System clipboard backend that runs the platform's copy and paste tools

use super::{prepare_content, ClipboardBackend, ClipboardError, ClipboardResult};
use crate::constants::MAX_CLIPBOARD_SIZE;
use std::io::{Read, Write};
use std::process::{Command, ExitStatus, Stdio};
use std::sync::OnceLock;
use std::thread;
use std::time::{Duration, Instant};

/// How long a clipboard tool may run before it is killed and the operation
/// fails. A tool that hangs - a paste tool waiting on a display server that
/// never answers - must not freeze the app with it.
const TOOL_TIMEOUT: Duration = Duration::from_secs(10);

/// Cached clipboard command detection to avoid repeated subprocess spawning
static COPY_COMMAND_CACHE: OnceLock<Option<(&'static str, &'static [&'static str])>> =
    OnceLock::new();
static PASTE_COMMAND_CACHE: OnceLock<Option<(&'static str, &'static [&'static str])>> =
    OnceLock::new();

/// Platform-specific clipboard implementation
#[derive(Clone, Debug, Default)]
pub struct SystemClipboard;

impl SystemClipboard {
    /// Create new system clipboard instance
    pub fn new() -> Self {
        Self
    }

    /// Get the appropriate copy command for the platform (cached)
    fn copy_command() -> Option<(&'static str, &'static [&'static str])> {
        *COPY_COMMAND_CACHE.get_or_init(|| {
            #[cfg(target_os = "macos")]
            {
                Some(("pbcopy", &[]))
            }

            #[cfg(target_os = "linux")]
            {
                // Try xclip first, then xsel, then wl-copy
                // Use direct command execution instead of shell to avoid injection risk
                let check_cmd = |cmd: &str| -> bool {
                    // Most clipboard tools support --version or --help for availability check
                    // Try to execute the command directly without shell
                    Command::new(cmd)
                        .arg("--version")
                        .stdout(Stdio::null())
                        .stderr(Stdio::null())
                        .status()
                        .map(|s| s.success())
                        .unwrap_or(false)
                };

                if check_cmd("xclip") {
                    Some(("xclip", &["-selection", "clipboard"]))
                } else if check_cmd("xsel") {
                    Some(("xsel", &["--clipboard", "--input"]))
                } else if check_cmd("wl-copy") {
                    Some(("wl-copy", &[]))
                } else {
                    None
                }
            }

            #[cfg(target_os = "windows")]
            {
                Some(("clip", &[]))
            }

            #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
            {
                None
            }
        })
    }

    /// Get the appropriate paste command for the platform (cached)
    fn paste_command() -> Option<(&'static str, &'static [&'static str])> {
        *PASTE_COMMAND_CACHE.get_or_init(|| {
            #[cfg(target_os = "macos")]
            {
                Some(("pbpaste", &[]))
            }

            #[cfg(target_os = "linux")]
            {
                // Try xclip first, then xsel, then wl-paste
                // Use direct command execution instead of shell to avoid injection risk
                let check_cmd = |cmd: &str| -> bool {
                    // Most clipboard tools support --version or --help for availability check
                    // Try to execute the command directly without shell
                    Command::new(cmd)
                        .arg("--version")
                        .stdout(Stdio::null())
                        .stderr(Stdio::null())
                        .status()
                        .map(|s| s.success())
                        .unwrap_or(false)
                };

                if check_cmd("xclip") {
                    Some(("xclip", &["-selection", "clipboard", "-o"]))
                } else if check_cmd("xsel") {
                    Some(("xsel", &["--clipboard", "--output"]))
                } else if check_cmd("wl-paste") {
                    Some(("wl-paste", &[]))
                } else {
                    None
                }
            }

            #[cfg(target_os = "windows")]
            {
                // Windows paste is more complex, use PowerShell
                Some(("powershell", &["-Command", "Get-Clipboard"]))
            }

            #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
            {
                None
            }
        })
    }
}

/// Run a clipboard tool: feed it `input` (copy) or collect its output
/// (paste), and kill it if it has not finished within `timeout`.
///
/// Output is kept up to one byte past [`MAX_CLIPBOARD_SIZE`] - enough to
/// tell it is too large - and the rest is read and dropped, so a tool
/// printing without end neither fills memory nor blocks on a full pipe.
fn run_tool(
    cmd: &str,
    args: &[&str],
    input: Option<String>,
    timeout: Duration,
) -> ClipboardResult<(ExitStatus, Vec<u8>)> {
    let copying = input.is_some();
    let mut child = Command::new(cmd)
        .args(args)
        .stdin(if copying {
            Stdio::piped()
        } else {
            Stdio::null()
        })
        .stdout(if copying {
            Stdio::null()
        } else {
            Stdio::piped()
        })
        .stderr(Stdio::null())
        .spawn()?;

    // Write and read on their own threads: either can block on a pipe the
    // tool does not service, and the timeout below must still fire.
    let writer = match (input, child.stdin.take()) {
        (Some(text), Some(mut stdin)) => Some(thread::spawn(move || {
            stdin.write_all(text.as_bytes())
            // `stdin` drops here, so the tool sees end of input.
        })),
        _ => None,
    };
    let reader = child.stdout.take().map(|stdout| {
        thread::spawn(move || -> std::io::Result<Vec<u8>> {
            let mut stdout = stdout;
            let mut kept = Vec::new();
            (&mut stdout)
                .take(MAX_CLIPBOARD_SIZE as u64 + 1)
                .read_to_end(&mut kept)?;
            std::io::copy(&mut stdout, &mut std::io::sink())?;
            Ok(kept)
        })
    });

    // Wait for the tool to exit *and* for its pipes to close: a process the
    // tool started may hold them open after the tool itself is gone (a shell
    // that forks its command, a tool that daemonizes), and the whole call is
    // bounded by `timeout`, not just the tool.
    let deadline = Instant::now() + timeout;
    let mut status = None;
    loop {
        if status.is_none() {
            status = child.try_wait()?;
        }
        let io_done = writer.as_ref().is_none_or(|w| w.is_finished())
            && reader.as_ref().is_none_or(|r| r.is_finished());
        if status.is_some() && io_done {
            break;
        }
        if Instant::now() >= deadline {
            if status.is_none() {
                let _ = child.kill();
                let _ = child.wait();
            }
            // The threads finish once whatever still holds the pipes closes
            // them; nothing waits for that.
            return Err(ClipboardError::CommandFailed(format!(
                "{cmd} did not finish within {timeout:?}"
            )));
        }
        thread::sleep(Duration::from_millis(5));
    }
    let Some(status) = status else {
        unreachable!("the loop only ends once the tool has exited");
    };

    let written = writer.map(|w| w.join());
    let read = reader.map(|r| r.join());
    match written {
        Some(Ok(Err(e))) => return Err(e.into()),
        Some(Err(_)) => {
            return Err(ClipboardError::CommandFailed(format!(
                "{cmd}: writer failed"
            )))
        }
        _ => {}
    }
    let output = match read {
        Some(Ok(result)) => result?,
        Some(Err(_)) => {
            return Err(ClipboardError::CommandFailed(format!(
                "{cmd}: reader failed"
            )))
        }
        None => Vec::new(),
    };
    Ok((status, output))
}

impl ClipboardBackend for SystemClipboard {
    fn set(&self, content: &str) -> ClipboardResult<()> {
        let sanitized = prepare_content(content)?;

        let (cmd, args) = Self::copy_command().ok_or(ClipboardError::NoClipboardTool)?;

        let (status, _) = run_tool(cmd, args, Some(sanitized), TOOL_TIMEOUT)?;
        if status.success() {
            Ok(())
        } else {
            Err(ClipboardError::CommandFailed(format!(
                "{} exited with status: {:?}",
                cmd,
                status.code()
            )))
        }
    }

    fn get(&self) -> ClipboardResult<String> {
        let (cmd, args) = Self::paste_command().ok_or(ClipboardError::NoClipboardTool)?;

        let (status, stdout) = run_tool(cmd, args, None, TOOL_TIMEOUT)?;

        if status.success() {
            // Validate size to prevent DoS through large clipboard content
            if stdout.len() > MAX_CLIPBOARD_SIZE {
                return Err(ClipboardError::InvalidInput(format!(
                    "Clipboard content too large: more than {} bytes",
                    MAX_CLIPBOARD_SIZE
                )));
            }
            String::from_utf8(stdout).map_err(|_| ClipboardError::InvalidUtf8)
        } else {
            Err(ClipboardError::CommandFailed(format!(
                "{} exited with status: {:?}",
                cmd,
                status.code()
            )))
        }
    }

    fn has_text(&self) -> ClipboardResult<bool> {
        match self.get() {
            Ok(content) => Ok(!content.is_empty()),
            Err(ClipboardError::NoClipboardTool) => Err(ClipboardError::NoClipboardTool),
            _ => Ok(false),
        }
    }

    fn clear(&self) -> ClipboardResult<()> {
        self.set("")
    }
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;

    #[test]
    fn a_tool_that_hangs_is_killed_at_the_timeout() {
        let start = Instant::now();
        // `; :` keeps the shell from exec-ing sleep: the shell is killed, but
        // the sleep it forked still holds stdout open.
        let result = run_tool(
            "sh",
            &["-c", "sleep 30; :"],
            None,
            Duration::from_millis(200),
        );
        assert!(matches!(result, Err(ClipboardError::CommandFailed(_))));
        assert!(start.elapsed() < Duration::from_secs(5));
    }

    #[test]
    fn a_tool_that_never_reads_its_input_is_killed_at_the_timeout() {
        // More than a pipe buffer, so the write blocks until the tool dies.
        let input = "x".repeat(1 << 20);
        let start = Instant::now();
        let result = run_tool(
            "sh",
            &["-c", "sleep 30; :"],
            Some(input),
            Duration::from_millis(200),
        );
        assert!(result.is_err());
        assert!(start.elapsed() < Duration::from_secs(5));
    }

    #[test]
    fn output_past_the_limit_is_cut_and_drained() {
        let script = format!("head -c {} /dev/zero", MAX_CLIPBOARD_SIZE + 4096);
        let (status, out) =
            run_tool("sh", &["-c", &script], None, Duration::from_secs(10)).unwrap();
        assert!(status.success());
        assert_eq!(out.len(), MAX_CLIPBOARD_SIZE + 1);
    }

    #[test]
    fn a_tool_that_works_is_unchanged() {
        let (status, out) = run_tool(
            "sh",
            &["-c", "cat"],
            Some("hello".into()),
            Duration::from_secs(10),
        )
        .unwrap();
        assert!(status.success());
        assert!(out.is_empty(), "copy mode does not collect output");
        let (status, out) =
            run_tool("sh", &["-c", "printf hi"], None, Duration::from_secs(10)).unwrap();
        assert!(status.success());
        assert_eq!(out, b"hi");
    }
}
