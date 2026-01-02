//! Path manipulation utilities
//!
//! Provides utilities for displaying and manipulating file paths
//! in user-friendly formats.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::utils::path::{shorten_path, home_relative, abbreviate_path};
//!
//! // Replace home directory with ~
//! let path = home_relative("/Users/john/Documents/file.txt");
//! assert_eq!(path, "~/Documents/file.txt");
//!
//! // Shorten to fit width
//! let short = shorten_path("/very/long/path/to/file.txt", 20);
//! assert_eq!(short, ".../path/to/file.txt");
//!
//! // Abbreviate middle directories
//! let abbr = abbreviate_path("/Users/john/Documents/Projects/rust/src/main.rs");
//! assert_eq!(abbr, "/U/j/D/P/rust/src/main.rs");
//! ```

use std::path::{Path, PathBuf};

/// Get the home directory path
pub fn home_dir() -> Option<PathBuf> {
    std::env::var("HOME")
        .ok()
        .map(PathBuf::from)
        .or_else(|| std::env::var("USERPROFILE").ok().map(PathBuf::from))
}

/// Replace home directory with ~ in path display
///
/// # Example
///
/// ```rust,ignore
/// let path = home_relative("/Users/john/Documents/file.txt");
/// // Returns "~/Documents/file.txt" if home is /Users/john
/// ```
pub fn home_relative(path: impl AsRef<Path>) -> String {
    let path = path.as_ref();

    if let Some(home) = home_dir() {
        if let Ok(stripped) = path.strip_prefix(&home) {
            return format!("~/{}", stripped.display());
        }
    }

    path.display().to_string()
}

/// Expand ~ to home directory
pub fn expand_home(path: impl AsRef<Path>) -> PathBuf {
    let path = path.as_ref();
    let path_str = path.to_string_lossy();

    if let Some(rest) = path_str.strip_prefix("~/") {
        if let Some(home) = home_dir() {
            return home.join(rest);
        }
    } else if path_str == "~" {
        if let Some(home) = home_dir() {
            return home;
        }
    }

    path.to_path_buf()
}

/// Shorten a path to fit within a maximum width
///
/// Replaces leading directories with "..." if needed.
///
/// # Example
///
/// ```rust,ignore
/// let short = shorten_path("/very/long/path/to/file.txt", 20);
/// assert_eq!(short, ".../path/to/file.txt");
/// ```
pub fn shorten_path(path: impl AsRef<Path>, max_width: usize) -> String {
    let path = path.as_ref();
    let full = path.display().to_string();

    if full.len() <= max_width {
        return full;
    }

    // Try to preserve the filename and as much of the path as possible
    let components: Vec<&str> = full.split('/').filter(|s| !s.is_empty()).collect();

    if components.is_empty() {
        return full;
    }

    // Always keep the filename
    let filename = components.last().unwrap();

    if filename.len() + 4 > max_width {
        // Even filename doesn't fit, truncate it
        return format!(
            "...{}",
            &filename[filename.len().saturating_sub(max_width - 3)..]
        );
    }

    // Try to include as many trailing components as possible
    let mut result = String::new();
    let mut included = 0;

    for (i, component) in components.iter().rev().enumerate() {
        let needed = if i == 0 {
            component.len()
        } else {
            component.len() + 1 // +1 for separator
        };

        if result.len() + needed + 4 <= max_width {
            if i > 0 {
                result = format!("/{}{}", component, result);
            } else {
                result = component.to_string();
            }
            included += 1;
        } else {
            break;
        }
    }

    if included < components.len() {
        if result.starts_with('/') {
            format!("...{}", result)
        } else {
            format!(".../{}", result)
        }
    } else {
        format!("/{}", result)
    }
}

/// Abbreviate path by shortening middle directories to first character
///
/// Keeps the last N components fully visible.
///
/// # Example
///
/// ```rust,ignore
/// let abbr = abbreviate_path("/Users/john/Documents/Projects/rust/main.rs");
/// // Returns "/U/j/D/Projects/rust/main.rs"
/// ```
pub fn abbreviate_path(path: impl AsRef<Path>) -> String {
    abbreviate_path_keep(path, 2)
}

/// Abbreviate path, keeping the last N components fully visible
pub fn abbreviate_path_keep(path: impl AsRef<Path>, keep_last: usize) -> String {
    let path = path.as_ref();
    let path_str = path.display().to_string();

    let starts_with_slash = path_str.starts_with('/');
    let components: Vec<&str> = path_str.split('/').filter(|s| !s.is_empty()).collect();

    if components.len() <= keep_last {
        return path_str;
    }

    let abbrev_count = components.len() - keep_last;
    let mut result = String::new();

    if starts_with_slash {
        result.push('/');
    }

    for (i, component) in components.iter().enumerate() {
        if i > 0 {
            result.push('/');
        }

        if i < abbrev_count {
            // Abbreviate to first character
            if let Some(first) = component.chars().next() {
                result.push(first);
            }
        } else {
            result.push_str(component);
        }
    }

    result
}

/// Get the relative path from a base directory
pub fn relative_to(path: impl AsRef<Path>, base: impl AsRef<Path>) -> String {
    let path = path.as_ref();
    let base = base.as_ref();

    if let Ok(rel) = path.strip_prefix(base) {
        rel.display().to_string()
    } else {
        path.display().to_string()
    }
}

/// Get the file extension (without dot)
pub fn extension(path: impl AsRef<Path>) -> Option<String> {
    path.as_ref()
        .extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_string())
}

/// Get the filename without extension
pub fn stem(path: impl AsRef<Path>) -> Option<String> {
    path.as_ref()
        .file_stem()
        .and_then(|s| s.to_str())
        .map(|s| s.to_string())
}

/// Get the filename (with extension)
pub fn filename(path: impl AsRef<Path>) -> Option<String> {
    path.as_ref()
        .file_name()
        .and_then(|s| s.to_str())
        .map(|s| s.to_string())
}

/// Get the parent directory
pub fn parent(path: impl AsRef<Path>) -> Option<String> {
    path.as_ref().parent().map(|p| p.display().to_string())
}

/// Check if path is hidden (starts with .)
pub fn is_hidden(path: impl AsRef<Path>) -> bool {
    path.as_ref()
        .file_name()
        .and_then(|s| s.to_str())
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

/// Normalize path separators (convert \ to /)
pub fn normalize_separators(path: &str) -> String {
    path.replace('\\', "/")
}

/// Join paths with proper separators
pub fn join_paths(base: impl AsRef<Path>, parts: &[&str]) -> PathBuf {
    let mut result = base.as_ref().to_path_buf();
    for part in parts {
        result = result.join(part);
    }
    result
}

/// Get common prefix of multiple paths
pub fn common_prefix(paths: &[&Path]) -> Option<PathBuf> {
    if paths.is_empty() {
        return None;
    }

    let first = paths[0];
    let mut common = PathBuf::new();

    for component in first.components() {
        let test = common.join(component);
        if paths.iter().all(|p| p.starts_with(&test)) {
            common = test;
        } else {
            break;
        }
    }

    if common.as_os_str().is_empty() {
        None
    } else {
        Some(common)
    }
}

/// Path display options
#[derive(Clone, Debug)]
pub struct PathDisplay {
    /// Maximum display width
    pub max_width: Option<usize>,
    /// Replace home with ~
    pub use_tilde: bool,
    /// Abbreviate middle directories
    pub abbreviate: bool,
    /// Number of directories to keep unabbreviated
    pub keep_dirs: usize,
}

impl Default for PathDisplay {
    fn default() -> Self {
        Self {
            max_width: None,
            use_tilde: true,
            abbreviate: false,
            keep_dirs: 2,
        }
    }
}

impl PathDisplay {
    /// Create a new path display config
    pub fn new() -> Self {
        Self::default()
    }

    /// Set maximum width
    pub fn max_width(mut self, width: usize) -> Self {
        self.max_width = Some(width);
        self
    }

    /// Enable/disable tilde for home
    pub fn tilde(mut self, use_tilde: bool) -> Self {
        self.use_tilde = use_tilde;
        self
    }

    /// Enable/disable abbreviation
    pub fn abbreviate(mut self, abbrev: bool) -> Self {
        self.abbreviate = abbrev;
        self
    }

    /// Set number of directories to keep
    pub fn keep(mut self, count: usize) -> Self {
        self.keep_dirs = count;
        self
    }

    /// Format a path according to options
    pub fn format(&self, path: impl AsRef<Path>) -> String {
        let mut result = if self.use_tilde {
            home_relative(path.as_ref())
        } else {
            path.as_ref().display().to_string()
        };

        if self.abbreviate {
            result = abbreviate_path_keep(&result, self.keep_dirs);
        }

        if let Some(max) = self.max_width {
            if result.len() > max {
                result = shorten_path(&result, max);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shorten_path() {
        // "/a/b/c/d/e/file.txt" is 20 chars, test with smaller width
        let short = shorten_path("/a/b/c/d/e/file.txt", 15);
        assert!(short.len() <= 15);
        assert!(short.ends_with("file.txt"));
        assert!(short.starts_with("..."));
    }

    #[test]
    fn test_shorten_path_fits() {
        let path = "/short/path.txt";
        let short = shorten_path(path, 30);
        assert_eq!(short, path);
    }

    #[test]
    fn test_abbreviate_path() {
        let abbr = abbreviate_path_keep("/Users/john/Documents/Projects/file.txt", 2);
        assert!(abbr.contains("/U/"));
        assert!(abbr.ends_with("Projects/file.txt"));
    }

    #[test]
    fn test_abbreviate_path_short() {
        let abbr = abbreviate_path_keep("/a/b", 2);
        assert_eq!(abbr, "/a/b");
    }

    #[test]
    fn test_expand_home() {
        let expanded = expand_home("~");
        if home_dir().is_some() {
            assert!(!expanded.to_string_lossy().contains('~'));
        }
    }

    #[test]
    fn test_extension() {
        assert_eq!(extension("file.txt"), Some("txt".to_string()));
        assert_eq!(extension("file.tar.gz"), Some("gz".to_string()));
        assert_eq!(extension("noext"), None);
    }

    #[test]
    fn test_stem() {
        assert_eq!(stem("file.txt"), Some("file".to_string()));
        assert_eq!(stem("file.tar.gz"), Some("file.tar".to_string()));
    }

    #[test]
    fn test_filename() {
        assert_eq!(filename("/path/to/file.txt"), Some("file.txt".to_string()));
    }

    #[test]
    fn test_is_hidden() {
        assert!(is_hidden(".hidden"));
        assert!(is_hidden("/path/.hidden"));
        assert!(!is_hidden("visible"));
    }

    #[test]
    fn test_normalize_separators() {
        assert_eq!(normalize_separators("a\\b\\c"), "a/b/c");
    }

    #[test]
    fn test_relative_to() {
        let rel = relative_to("/home/user/docs/file.txt", "/home/user");
        assert_eq!(rel, "docs/file.txt");
    }

    #[test]
    fn test_path_display() {
        let display = PathDisplay::new().abbreviate(true).keep(1);

        let result = display.format("/a/b/c/d/file.txt");
        assert!(result.ends_with("file.txt"));
    }
}
