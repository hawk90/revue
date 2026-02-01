//! System browser and URL utilities
//!
//! Platform-aware utilities for opening URLs and files in the system browser/application.
//!
//! # Example
//! ```ignore
//! use revue::utils::browser::open_browser;
//!
//! // Open URL in default browser
//! open_browser("https://github.com");
//!
//! // Open file with default application
//! open_browser("/path/to/file.pdf");
//! ```

use std::path::Path;

use std::process::Command;

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
        if rest.contains('/') {
            let first_slash = rest.find('/').unwrap();
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

/// Open a URL or file in the system's default browser/application
///
/// Platform support:
/// - macOS: Uses `open`
/// - Linux: Uses `xdg-open`
/// - Windows: Uses `start`
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

    #[cfg(target_os = "macos")]
    let result = Command::new("open").arg(url).spawn();

    #[cfg(target_os = "linux")]
    let result = Command::new("xdg-open").arg(url).spawn();

    #[cfg(target_os = "windows")]
    let result = Command::new("cmd").args(["/C", "start", "", url]).spawn();

    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    let result: Result<std::process::Child, std::io::Error> = Err(std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        "Unsupported platform",
    ));

    result.is_ok()
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

    #[cfg(target_os = "macos")]
    let child = Command::new("open")
        .arg(url)
        .spawn()
        .map_err(|e| BrowserError::IoError(e.to_string()))?;

    #[cfg(target_os = "linux")]
    let child = Command::new("xdg-open")
        .arg(url)
        .spawn()
        .map_err(|e| BrowserError::IoError(e.to_string()))?;

    #[cfg(target_os = "windows")]
    let child = Command::new("cmd")
        .args(["/C", "start", "", url])
        .spawn()
        .map_err(|e| BrowserError::IoError(e.to_string()))?;

    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    return Err(BrowserError::IoError("Unsupported platform".to_string()));

    // Detach - don't wait for browser to close
    // Dropping the child detaches it (no wait() call)
    drop(child);
    Ok(())
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

    #[cfg(target_os = "macos")]
    let result = Command::new("open").arg(path).spawn();

    #[cfg(target_os = "linux")]
    let result = Command::new("xdg-open").arg(path).spawn();

    #[cfg(target_os = "windows")]
    let result = Command::new("explorer").arg(path).spawn();

    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    let result: Result<std::process::Child, std::io::Error> = Err(std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        "Unsupported platform",
    ));

    result.is_ok()
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

    #[cfg(target_os = "macos")]
    let result = Command::new("open").args(["-R", path]).spawn();

    #[cfg(target_os = "linux")]
    // Linux doesn't have a standard "reveal" - just open parent folder
    let result = {
        let parent = std::path::Path::new(path)
            .parent()
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_else(|| path.to_string());
        Command::new("xdg-open").arg(&parent).spawn()
    };

    #[cfg(target_os = "windows")]
    let result = Command::new("explorer").args(["/select,", path]).spawn();

    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    let result: Result<std::process::Child, std::io::Error> = Err(std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        "Unsupported platform",
    ));

    result.is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_functions_exist() {
        // Just verify the functions compile
        let _ = open_browser;
        let _ = open_url;
        let _ = open_file;
        let _ = open_folder;
        let _ = reveal_in_finder;
    }

    #[test]
    fn test_open_browser_return_type() {
        // Verify open_browser returns a bool
        let _: fn(&str) -> bool = open_browser;
    }

    #[test]
    fn test_open_url_return_type() {
        // Verify open_url returns a Result
        let _: fn(&str) -> Result<(), BrowserError> = open_url;
    }

    #[test]
    fn test_open_file_return_type() {
        // Verify open_file returns a bool
        let _: fn(&str) -> bool = open_file;
    }

    #[test]
    fn test_open_folder_return_type() {
        // Verify open_folder returns a bool
        let _: fn(&str) -> bool = open_folder;
    }

    #[test]
    fn test_reveal_in_finder_return_type() {
        // Verify reveal_in_finder returns a bool
        let _: fn(&str) -> bool = reveal_in_finder;
    }

    // Security tests

    #[test]
    fn test_reject_shell_metacharacter_ampersand() {
        assert!(!open_browser("https://example.com & malware.exe"));
        assert!(open_url("https://example.com & malware.exe").is_err());
        assert!(!open_folder("/path & rm -rf /"));
    }

    #[test]
    fn test_reject_shell_metacharacter_pipe() {
        assert!(!open_browser("https://example.com | cat /etc/passwd"));
        assert!(open_url("url | malicious").is_err());
    }

    #[test]
    fn test_reject_shell_metacharacter_semicolon() {
        assert!(!open_browser("https://example.com; rm -rf /"));
        assert!(open_url("url; malicious").is_err());
    }

    #[test]
    fn test_reject_backtick() {
        assert!(!open_browser("https://example.com`malicious`"));
        assert!(open_url("url`malicious`").is_err());
    }

    #[test]
    fn test_reject_newline() {
        assert!(!open_browser("https://example.com\nrm -rf /"));
        assert!(open_url("url\nmalicious").is_err());
    }

    #[test]
    fn test_reject_command_substitution() {
        assert!(!open_browser("https://example.com$(rm -rf /)"));
        assert!(open_url("url$(malicious)").is_err());
    }

    #[test]
    fn test_allow_valid_urls() {
        // These should pass validation (may fail on command execution in tests)
        assert!(validate_input("https://example.com").is_ok());
        assert!(validate_input("http://example.com").is_ok());
        assert!(validate_input("https://github.com/hawk90/revue").is_ok());
        assert!(validate_input("ftp://files.example.com/file.txt").is_ok());
    }

    #[test]
    fn test_allow_valid_paths() {
        assert!(validate_input("/path/to/file.pdf").is_ok());
        assert!(validate_input("./relative/path").is_ok());
        assert!(validate_input("../parent/path").is_ok());
    }

    #[test]
    fn test_reject_empty_url() {
        assert!(validate_input("").is_err());
        assert!(open_url("").is_err());
    }

    #[test]
    fn test_reject_dangerous_scheme() {
        assert!(validate_input("javascript:alert(1)").is_err());
        assert!(validate_input("data:text/html,<script>").is_err());
    }

    #[test]
    fn test_reject_unicode_control_characters() {
        // C1 control characters (U+0080-U+009F)
        assert!(validate_input("https://example.com\u{0080}test").is_err());
        assert!(validate_input("https://example.com\u{009F}test").is_err());
        assert!(open_url("https://example.com\u{0090}malicious").is_err());
    }

    #[test]
    fn test_reject_file_url_with_path_traversal() {
        // file:// URLs with .. should be rejected
        assert!(validate_input("file:///etc/../passwd").is_err());
        assert!(open_url("file://../../../etc/passwd").is_err());
    }

    #[test]
    fn test_reject_file_url_sensitive_paths() {
        // Sensitive system files should be blocked
        assert!(validate_input("file:///etc/passwd").is_err());
        assert!(validate_input("file:///etc/shadow").is_err());
        assert!(validate_input("file:///etc/hosts").is_err());
        assert!(validate_input("file:///root/.ssh/id_rsa").is_err());
        assert!(open_url("file:///etc/passwd").is_err());
    }

    #[test]
    fn test_allow_safe_file_urls() {
        // Safe file:// URLs should be allowed
        assert!(validate_input("file:///home/user/document.pdf").is_ok());
        assert!(validate_input("file:///tmp/file.txt").is_ok());
    }

    #[test]
    fn test_reject_remote_file_urls() {
        // file:// URLs with remote hosts should be rejected
        assert!(validate_input("file://evil.com/etc/passwd").is_err());
        assert!(validate_input("file://192.168.1.1/share/file").is_err());
    }

    #[test]
    fn test_error_messages() {
        let err = open_url("https://example.com & malware").unwrap_err();
        assert!(err.to_string().contains("dangerous") || err.to_string().contains("character"));
    }
}
