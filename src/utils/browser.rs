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

use std::process::Command;

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
/// * `false` if spawning failed
pub fn open_browser(url: &str) -> bool {
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
/// Returns `Err(io::Error)` if:
/// - The platform is not supported (not macOS, Linux, or Windows)
/// - The browser command cannot be spawned
/// - The URL is invalid or inaccessible
pub fn open_url(url: &str) -> std::io::Result<()> {
    #[cfg(target_os = "macos")]
    let child = Command::new("open").arg(url).spawn()?;

    #[cfg(target_os = "linux")]
    let child = Command::new("xdg-open").arg(url).spawn()?;

    #[cfg(target_os = "windows")]
    let child = Command::new("cmd").args(["/C", "start", "", url]).spawn()?;

    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    return Err(std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        "Unsupported platform",
    ));

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
pub fn open_folder(path: &str) -> bool {
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
pub fn reveal_in_finder(path: &str) -> bool {
    #[cfg(target_os = "macos")]
    let result = Command::new("open").args(["-R", path]).spawn();

    #[cfg(target_os = "linux")]
    // Linux doesn't have a standard "reveal" - just open parent folder
    let result = {
        let parent = std::path::Path::new(path)
            .parent()
            .map(|p| p.to_string_lossy().to_string())
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
    // Note: These tests don't actually open browsers, just check compilation
    // Actual browser opening behavior cannot be tested in unit tests
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
        let _: fn(&str) -> std::io::Result<()> = open_url;
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

    // Note: We can't actually test the browser opening behavior
    // without mocking the Command execution, as it would:
    // 1. Open actual browsers/file managers
    // 2. Be platform-dependent
    // 3. Fail in CI environments without display
    //
    // The main test coverage comes from integration tests or manual testing.
}
