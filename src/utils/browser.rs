//! System browser and URL utilities
//!
//! Platform-aware utilities for opening URLs and files in the system browser/application.
//!
//! # Example
//! ```
//! use revue::utils::browser::open_browser;
//!
//! // Open URL in default browser
//! open_browser("https://github.com");
//!
//! // Open file with default application
//! open_browser("/path/to/file.pdf");
//! ```

use std::path::Path;

use std::process::{Command, ExitStatus};
use std::thread::JoinHandle;

/// Error type for browser operations
#[derive(Debug, Clone, thiserror::Error)]
pub enum BrowserError {
    /// Invalid URL format
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    /// URL contains dangerous characters
    #[error("URL contains dangerous characters: {0}")]
    DangerousCharacters(String),

    /// IO error from command execution
    #[error("IO error: {0}")]
    IoError(String),
}

/// Validate that input doesn't contain shell metacharacters
///
/// Blocks characters that could be used for command injection:
/// - `&`, `|`, `;` - command separators
/// - `` ` `` - command substitution
/// - `$(` - command substitution (bash)
/// - `\n`, `\r` - command separators
/// - `0x00` - null byte
/// - Unicode control characters (U+0080-U+009F)
fn validate_shell_safe(input: &str) -> Result<(), BrowserError> {
    let dangerous = ['&', '|', ';', '`', '\n', '\r', '\x00'];

    for ch in input.chars() {
        // Check ASCII dangerous characters
        if dangerous.contains(&ch) {
            return Err(BrowserError::DangerousCharacters(format!(
                "character '{}' not allowed",
                if ch == '\n' {
                    "\\n".to_string()
                } else if ch == '\r' {
                    "\\r".to_string()
                } else if ch == '\x00' {
                    "\\x00".to_string()
                } else {
                    ch.to_string()
                }
            )));
        }

        // Check Unicode control characters (C1 control characters: U+0080-U+009F)
        if ('\u{0080}'..='\u{009F}').contains(&ch) {
            return Err(BrowserError::DangerousCharacters(format!(
                "control character U+{:04X} not allowed",
                ch as u32
            )));
        }
    }

    // Also check for $( pattern (bash command substitution)
    if input.contains("$(") {
        return Err(BrowserError::DangerousCharacters(
            "pattern '$(' not allowed".to_string(),
        ));
    }

    Ok(())
}

/// Validate URL format
///
/// Performs basic validation to ensure the input looks like a URL or file path.
fn validate_url_format(url: &str) -> Result<(), BrowserError> {
    if url.is_empty() {
        return Err(BrowserError::InvalidUrl("URL cannot be empty".to_string()));
    }

    // Block dangerous schemes that use single colon (without //)
    let dangerous_schemes = ["javascript:", "data:", "vbscript:"];
    for dangerous in dangerous_schemes {
        if url.to_lowercase().starts_with(dangerous) {
            return Err(BrowserError::InvalidUrl(format!(
                "URL scheme '{}' is not allowed",
                dangerous.trim_end_matches(':')
            )));
        }
    }

    // Allow file paths (starting with / or ./ or ../ or containing \ on Windows)
    if url.starts_with('/')
        || url.starts_with("./")
        || url.starts_with("../")
        || url.contains('\\')
        || (url.len() > 1 && url.as_bytes().get(1) == Some(&b':'))
    // Windows drive letter (use byte indexing for O(1))
    {
        return Ok(());
    }

    // Check for URL scheme (http, https, ftp, etc.)
    if let Some(scheme_end) = url.find("://") {
        let scheme = &url[..scheme_end];

        // Only allow certain schemes
        let allowed_schemes = [
            "http", "https", "ftp", "ftps", "file", "mailto", "tel", "ws", "wss",
        ];

        if !allowed_schemes.contains(&scheme) {
            return Err(BrowserError::InvalidUrl(format!(
                "URL scheme '{}' is not allowed",
                scheme
            )));
        }

        // Additional validation for file:// URLs to prevent access to sensitive files
        if scheme == "file" {
            validate_file_url(url)?;
        }
    }

    Ok(())
}

/// Validate file:// URLs to prevent access to sensitive system files
fn validate_file_url(url: &str) -> Result<(), BrowserError> {
    // Extract the path from file:// URL
    // file:///etc/passwd -> /etc/passwd
    // file://localhost/etc/passwd -> /etc/passwd
    let path_str = if let Some(stripped) = url.strip_prefix("file://localhost/") {
        stripped.to_string()
    } else if let Some(stripped) = url.strip_prefix("file:///") {
        stripped.to_string()
    } else if let Some(stripped) = url.strip_prefix("file://") {
        // file:// with something after it - check if it's a remote host
        let rest = stripped;
        // If there's no "/" after "file://", or the first "/" is after a hostname,
        // it's a remote URL (file://evil.com/etc/passwd)
        if let Some(first_slash) = rest.find('/') {
            // Check if there's a hostname before the first slash
            // (i.e., the path doesn't start with "/")
            if !rest[..first_slash].is_empty() && !rest.starts_with('/') {
                return Err(BrowserError::InvalidUrl(
                    "Remote file:// URLs not allowed".to_string(),
                ));
            }
        }
        // Local file:// URL (file:///path or file://localhost/path)
        rest.to_string()
    } else {
        return Err(BrowserError::InvalidUrl("Invalid file:// URL".to_string()));
    };

    let path = Path::new(&path_str);

    // Check for path traversal components in file:// URLs
    for component in path.components() {
        use std::path::Component;
        match component {
            Component::ParentDir => {
                return Err(BrowserError::InvalidUrl(
                    "Path traversal not allowed in file:// URLs".to_string(),
                ));
            }
            Component::RootDir => {
                // Absolute paths in file:// URLs could access system files
                // Block access to sensitive system directories
                let path_str_lower = path_str.to_lowercase();
                for sensitive in &[
                    "/etc/passwd",
                    "/etc/shadow",
                    "/etc/hosts",
                    "/etc/sudoers",
                    "/root/",
                    "/boot/",
                    "/sys/",
                    "/proc/",
                ] {
                    if path_str_lower.starts_with(sensitive) {
                        return Err(BrowserError::InvalidUrl(format!(
                            "Access to {} is not allowed",
                            sensitive
                        )));
                    }
                }
            }
            _ => {}
        }
    }

    Ok(())
}

/// Validate input is safe for shell execution
fn validate_input(input: &str) -> Result<(), BrowserError> {
    validate_shell_safe(input)?;
    validate_url_format(input)?;
    Ok(())
}

/// Environment variable that suppresses actually launching an external
/// application (browser, file manager, opener).
///
/// When set to any non-empty value, the `open_*` / `reveal_*` functions still
/// validate their input and report success, but do not spawn a process. This is
/// useful for running examples, demos, tests, and CI without real browser
/// windows popping up.
pub const NO_LAUNCH_ENV: &str = "REVUE_NO_BROWSER";

/// Whether launching external applications is currently suppressed.
///
/// See [`NO_LAUNCH_ENV`].
pub fn launch_suppressed() -> bool {
    suppressed_by(std::env::var_os(NO_LAUNCH_ENV).as_deref())
}

/// The suppression rule, separate from reading the environment: set and
/// non-empty.
fn suppressed_by(value: Option<&std::ffi::OsStr>) -> bool {
    value.is_some_and(|v| !v.is_empty())
}

/// Open a URL or file in the system's default browser/application
///
/// Platform support:
/// - macOS: Uses `open`
/// - Linux: Uses `xdg-open`
/// - Windows: Uses `explorer`
///
/// # Arguments
/// * `url` - URL or file path to open
///
/// # Returns
/// * `true` if the command was spawned successfully
/// * `false` if spawning failed or validation failed
///
/// # Security
/// The input is validated for shell metacharacters to prevent command injection.
pub fn open_browser(url: &str) -> bool {
    if validate_input(url).is_err() {
        return false;
    }

    // Suppressed (e.g. demos/tests/CI): validated successfully, but don't launch.
    if launch_suppressed() {
        return true;
    }

    launch(opener(std::env::consts::OS, url)).is_ok()
}

/// Start an opener without waiting for it (#1551).
///
/// The opener gets none of the app's stdio, so what it prints (xdg-open
/// warnings, a browser's messages) does not land on the terminal UI, and a
/// thread waits for it, so it leaves no zombie behind. `None` (no opener on
/// this platform) is an `Unsupported` error. Dropping the returned handle
/// leaves the thread waiting; joining it yields the exit status.
fn launch(cmd: Option<Command>) -> std::io::Result<JoinHandle<std::io::Result<ExitStatus>>> {
    let cmd = cmd.ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::Unsupported, "Unsupported platform")
    })?;
    crate::utils::process::spawn_reaped(cmd)
}

/// The command that opens `target` with the default application on `os`
/// (a value of [`std::env::consts::OS`]), or `None` on other platforms.
///
/// No shell is involved: `target` reaches the opener as a single argument.
/// On Windows this matters, since `cmd /C start` would interpret `>`, `^`,
/// `%VAR%` and the like in a URL, and `%` cannot be rejected because URLs
/// use it for percent-encoding.
fn opener(os: &str, target: &str) -> Option<Command> {
    let program = match os {
        "macos" => "open",
        "linux" => "xdg-open",
        "windows" => "explorer",
        _ => return None,
    };
    let mut cmd = Command::new(program);
    cmd.arg(target);
    Some(cmd)
}

/// Open a URL in the system default browser
///
/// Same as `open_browser` but returns a Result for error handling.
///
/// # Errors
///
/// Returns `Err(BrowserError)` if:
/// - The URL contains dangerous characters
/// - The URL format is invalid
/// - The platform is not supported (not macOS, Linux, or Windows)
/// - The browser command cannot be spawned
pub fn open_url(url: &str) -> Result<(), BrowserError> {
    validate_input(url)?;

    // Suppressed (e.g. demos/tests/CI): validated successfully, but don't launch.
    if launch_suppressed() {
        return Ok(());
    }

    launch(opener(std::env::consts::OS, url))
        .map(drop)
        .map_err(|e| BrowserError::IoError(e.to_string()))
}

/// Open a file with its default application
///
/// Alias for `open_browser` - works with file paths too.
pub fn open_file(path: &str) -> bool {
    open_browser(path)
}

/// Open a folder in the system file manager
///
/// # Arguments
/// * `path` - Path to the folder
///
/// # Security
/// The input is validated for shell metacharacters to prevent command injection.
pub fn open_folder(path: &str) -> bool {
    if validate_input(path).is_err() {
        return false;
    }

    // Suppressed (e.g. demos/tests/CI): validated successfully, but don't launch.
    if launch_suppressed() {
        return true;
    }

    // The file manager opens a folder like any other target
    launch(opener(std::env::consts::OS, path)).is_ok()
}

/// Reveal a file in the system file manager (highlight the file)
///
/// # Arguments
/// * `path` - Path to the file to reveal
///
/// # Security
/// The input is validated for shell metacharacters to prevent command injection.
pub fn reveal_in_finder(path: &str) -> bool {
    if validate_input(path).is_err() {
        return false;
    }

    // Suppressed (e.g. demos/tests/CI): validated successfully, but don't launch.
    if launch_suppressed() {
        return true;
    }

    launch(revealer(std::env::consts::OS, path)).is_ok()
}

/// The command that shows `path` selected in the file manager on `os` (a
/// value of [`std::env::consts::OS`]), or `None` on other platforms.
///
/// Linux has no standard "reveal": the parent folder is opened instead.
fn revealer(os: &str, path: &str) -> Option<Command> {
    let cmd = match os {
        "macos" => {
            let mut cmd = Command::new("open");
            cmd.args(["-R", path]);
            cmd
        }
        "linux" => {
            let parent = Path::new(path)
                .parent()
                .map(|p| p.to_string_lossy().into_owned())
                .unwrap_or_else(|| path.to_string());
            let mut cmd = Command::new("xdg-open");
            cmd.arg(parent);
            cmd
        }
        "windows" => {
            let mut cmd = Command::new("explorer");
            cmd.args(["/select,", path]);
            cmd
        }
        _ => return None,
    };
    Some(cmd)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suppression_needs_a_non_empty_value() {
        use std::ffi::OsStr;
        assert!(suppressed_by(Some(OsStr::new("1"))));
        assert!(suppressed_by(Some(OsStr::new("yes"))));
        assert!(!suppressed_by(Some(OsStr::new(""))));
        assert!(!suppressed_by(None));
    }

    fn program_and_args(cmd: &Command) -> (String, Vec<String>) {
        (
            cmd.get_program().to_string_lossy().into_owned(),
            cmd.get_args()
                .map(|a| a.to_string_lossy().into_owned())
                .collect(),
        )
    }

    // A URL with characters cmd.exe would act on: redirection, its escape
    // character and variable expansion. `%` is also how URLs encode bytes,
    // so validation cannot reject it; the URL must not go through a shell.
    const URL: &str = "https://example.com/a?b=1%20c>d^e%PATH%";

    #[test]
    fn windows_opens_without_a_shell() {
        let (program, args) = program_and_args(&opener("windows", URL).unwrap());
        assert_ne!(program, "cmd", "cmd.exe would interpret the URL");
        assert_eq!(args, [URL], "the URL is passed as one argument, unchanged");
    }

    #[test]
    fn macos_and_linux_pass_the_url_as_one_argument() {
        assert_eq!(
            program_and_args(&opener("macos", URL).unwrap()),
            ("open".to_string(), vec![URL.to_string()])
        );
        assert_eq!(
            program_and_args(&opener("linux", URL).unwrap()),
            ("xdg-open".to_string(), vec![URL.to_string()])
        );
    }

    #[test]
    fn other_platforms_have_no_opener() {
        assert!(opener("plan9", URL).is_none());
        assert!(revealer("plan9", "/tmp/a.txt").is_none());
    }

    #[test]
    fn reveal_selects_the_file_on_macos_and_windows() {
        assert_eq!(
            program_and_args(&revealer("macos", "/tmp/a b.txt").unwrap()),
            (
                "open".to_string(),
                vec!["-R".to_string(), "/tmp/a b.txt".to_string()]
            )
        );
        assert_eq!(
            program_and_args(&revealer("windows", r"C:\a b.txt").unwrap()),
            (
                "explorer".to_string(),
                vec!["/select,".to_string(), r"C:\a b.txt".to_string()]
            )
        );
    }

    /// Linux has no "reveal": the parent folder opens; the root opens
    /// itself
    #[test]
    fn reveal_opens_the_parent_folder_on_linux() {
        for (path, opened) in [("/tmp/dir/a.txt", "/tmp/dir"), ("/a.txt", "/"), ("/", "/")] {
            assert_eq!(
                program_and_args(&revealer("linux", path).unwrap()),
                ("xdg-open".to_string(), vec![opened.to_string()]),
                "{path}"
            );
        }
    }

    /// An opener runs with null stdin, stdout and stderr (nothing it prints
    /// reaches the terminal UI) and is waited on: the handle yields its exit
    /// status (#1551)
    #[test]
    #[cfg(unix)]
    fn launch_nulls_stdio_and_reaps() {
        let mut cmd = Command::new("sh");
        cmd.arg("-c").arg(
            "[ /dev/stdin -ef /dev/null ] && [ /dev/stdout -ef /dev/null ] \
             && [ /dev/stderr -ef /dev/null ] && exit 7",
        );
        let status = launch(Some(cmd)).unwrap().join().unwrap().unwrap();
        assert_eq!(status.code(), Some(7));
    }

    #[test]
    fn launch_without_an_opener_is_unsupported() {
        let err = launch(None).unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::Unsupported);
        assert_eq!(err.to_string(), "Unsupported platform");
    }

    #[test]
    fn launch_reports_a_missing_program() {
        assert!(launch(Some(Command::new("revue-no-such-program-1551"))).is_err());
    }
}
