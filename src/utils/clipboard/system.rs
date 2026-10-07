//! System clipboard backend that runs the platform's copy and paste tools

use super::{ClipboardBackend, ClipboardError, ClipboardResult};
use crate::constants::MAX_CLIPBOARD_SIZE;
use std::io::Write;
use std::process::{Command, Stdio};
use std::sync::OnceLock;

/// Sanitize clipboard content by removing dangerous characters and ANSI escapes
fn sanitize_clipboard_content(content: &str) -> String {
    // First strip ANSI escape sequences to prevent terminal injection
    let stripped = crate::utils::ansi::strip_ansi(content);
    stripped
        .chars()
        .filter(|&c| {
            // Keep printable characters and common whitespace
            // Filter out null bytes, ESC (0x1B), and other control characters except:
            // - \t (tab), \n (newline), \r (carriage return)
            (c >= ' ' && c != '\x1b') || c == '\t' || c == '\n' || c == '\r'
        })
        .collect()
}

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

impl ClipboardBackend for SystemClipboard {
    fn set(&self, content: &str) -> ClipboardResult<()> {
        // Validate size to prevent DoS
        if content.len() > MAX_CLIPBOARD_SIZE {
            return Err(ClipboardError::InvalidInput(format!(
                "Clipboard content too large ({} bytes, max {})",
                content.len(),
                MAX_CLIPBOARD_SIZE
            )));
        }

        // Sanitize content to remove dangerous control characters
        let sanitized = sanitize_clipboard_content(content);

        let (cmd, args) = Self::copy_command().ok_or(ClipboardError::NoClipboardTool)?;

        let mut child = Command::new(cmd)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(sanitized.as_bytes())?;
        }

        let status = child.wait()?;
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

        let output = Command::new(cmd)
            .args(args)
            .stdin(Stdio::null())
            .stderr(Stdio::null())
            .output()?;

        if output.status.success() {
            // Validate size to prevent DoS through large clipboard content
            if output.stdout.len() > MAX_CLIPBOARD_SIZE {
                return Err(ClipboardError::InvalidInput(format!(
                    "Clipboard content too large: {} bytes (max: {})",
                    output.stdout.len(),
                    MAX_CLIPBOARD_SIZE
                )));
            }
            String::from_utf8(output.stdout).map_err(|_| ClipboardError::InvalidUtf8)
        } else {
            Err(ClipboardError::CommandFailed(format!(
                "{} exited with status: {:?}",
                cmd,
                output.status.code()
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
