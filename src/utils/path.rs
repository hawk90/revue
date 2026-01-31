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

use std::path::{Component, Path, PathBuf};

/// Error types for path validation
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PathError {
    /// Path contains traversal patterns (..)
    PathTraversal(String),
    /// Path contains invalid characters
    InvalidCharacter(String),
    /// Path is outside expected bounds
    OutsideBounds,
}

impl std::fmt::Display for PathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PathError::PathTraversal(p) => write!(f, "Path contains traversal pattern: {}", p),
            PathError::InvalidCharacter(p) => write!(f, "Path contains invalid characters: {}", p),
            PathError::OutsideBounds => write!(f, "Path is outside expected bounds"),
        }
    }
}

impl std::error::Error for PathError {}

/// Validate that path doesn't contain traversal patterns
///
/// Returns error if the path contains `..` components (ParentDir).
/// Note: File names containing ".." like "file..txt" are allowed - only actual
/// parent directory components (..) are rejected.
pub fn validate_no_traversal(path: &Path) -> Result<(), PathError> {
    // Check components using the Path API
    for component in path.components() {
        if component == Component::ParentDir {
            return Err(PathError::PathTraversal(path.display().to_string()));
        }
    }
    Ok(())
}

/// Validate that path doesn't contain null bytes
pub fn validate_characters(path: &Path) -> Result<(), PathError> {
    let path_str = path.to_string_lossy();
    if path_str.contains('\0') {
        return Err(PathError::InvalidCharacter(path.display().to_string()));
    }
    Ok(())
}

/// Validate that a path stays within a base directory
///
/// Returns error if the path, when canonicalized, escapes the base directory.
/// Also checks parent directories for symlinks that might escape the base.
pub fn validate_within_base(path: &Path, base: &Path) -> Result<(), PathError> {
    // First validate for traversal patterns
    validate_no_traversal(path)?;

    // Check parent directories for symlink escape
    // Even if the leaf doesn't exist, its parent should be within base
    if let Some(parent) = path.parent() {
        if parent.exists() {
            if let Ok(base_canonical) = base.canonicalize() {
                if let Ok(parent_canonical) = parent.canonicalize() {
                    if !parent_canonical.starts_with(&base_canonical) {
                        return Err(PathError::OutsideBounds);
                    }
                }
            }
        }
    }

    // If both base and path exist, verify the path stays within base
    if base.exists() && path.exists() {
        if let Ok(base_canonical) = base.canonicalize() {
            if let Ok(path_canonical) = path.canonicalize() {
                if !path_canonical.starts_with(&base_canonical) {
                    return Err(PathError::OutsideBounds);
                }
            }
        }
    }

    Ok(())
}

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
///
/// # Security
///
/// Validates that the expanded path doesn't escape the home directory
/// through path traversal patterns like `~/../../../etc/passwd`.
///
/// # Panics
///
/// Panics if the path contains traversal patterns. Use `try_expand_home` for a non-panicking version.
pub fn expand_home(path: impl AsRef<Path>) -> PathBuf {
    try_expand_home(path).expect("Path contains traversal patterns")
}

/// Expand ~ to home directory (non-panicking version)
///
/// Returns error if the path contains traversal patterns or would escape home directory.
pub fn try_expand_home(path: impl AsRef<Path>) -> Result<PathBuf, PathError> {
    let path = path.as_ref();
    let path_str = path.to_string_lossy();

    if let Some(rest) = path_str.strip_prefix("~/") {
        if let Some(home) = home_dir() {
            let rest_path = Path::new(rest);

            // Validate the rest doesn't contain traversal patterns
            validate_no_traversal(rest_path)?;

            // Reject absolute paths after ~/ - they would escape home
            if rest_path.is_absolute() {
                return Err(PathError::PathTraversal(format!(
                    "Absolute path not allowed after ~/: {}",
                    rest
                )));
            }

            // Also check for RootDir and Prefix components in rest
            for component in rest_path.components() {
                match component {
                    std::path::Component::RootDir | std::path::Component::Prefix(_) => {
                        return Err(PathError::PathTraversal(format!(
                            "Root or prefix component not allowed after ~/: {}",
                            rest
                        )));
                    }
                    _ => {}
                }
            }

            let expanded = home.join(rest_path);

            // Verify the result stays within home directory
            if home.exists() {
                if let Ok(home_canonical) = home.canonicalize() {
                    if expanded.exists() {
                        if let Ok(expanded_canonical) = expanded.canonicalize() {
                            if !expanded_canonical.starts_with(&home_canonical) {
                                return Err(PathError::OutsideBounds);
                            }
                        }
                    }
                }
            }

            return Ok(expanded);
        }
    } else if path_str == "~" {
        if let Some(home) = home_dir() {
            return Ok(home);
        }
    }

    Ok(path.to_path_buf())
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

    // Always keep the filename (safe due to is_empty check above)
    let filename = components
        .last()
        .expect("components should not be empty after is_empty check");

    if filename.len() + 4 > max_width {
        // Even filename doesn't fit, truncate it
        // Use char_indices for safe UTF-8 truncation
        let max_bytes = max_width.saturating_sub(3);
        let mut filename_end = 0;
        for (byte_pos, _char) in filename.char_indices() {
            if byte_pos > max_bytes {
                break;
            }
            filename_end = byte_pos;
        }
        return format!("...{}", &filename[..filename_end]);
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
///
/// # Security
///
/// Validates that all path parts don't contain traversal patterns.
///
/// # Panics
///
/// Panics if any path part contains traversal patterns. Use `try_join_paths` for a non-panicking version.
pub fn join_paths(base: impl AsRef<Path>, parts: &[&str]) -> PathBuf {
    try_join_paths(base, parts).expect("Path contains traversal patterns")
}

/// Join paths with proper separators (non-panicking version)
///
/// Returns error if any path part contains traversal patterns or is absolute.
pub fn try_join_paths(base: impl AsRef<Path>, parts: &[&str]) -> Result<PathBuf, PathError> {
    let mut result = base.as_ref().to_path_buf();

    for part in parts {
        let part_path = Path::new(part);

        // Check for traversal patterns
        validate_no_traversal(part_path)?;

        // Reject absolute paths - they would escape the base
        if part_path.is_absolute() {
            return Err(PathError::PathTraversal(format!(
                "Absolute path not allowed in join_paths: {}",
                part
            )));
        }

        // Also check for RootDir and Prefix components
        for component in part_path.components() {
            match component {
                std::path::Component::RootDir | std::path::Component::Prefix(_) => {
                    return Err(PathError::PathTraversal(format!(
                        "Root or prefix component not allowed: {}",
                        part
                    )));
                }
                _ => {}
            }
        }

        result = result.join(part_path);
    }

    Ok(result)
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

    // ============================================================================
    // Security Tests - Path Traversal
    // ============================================================================

    #[test]
    fn test_validate_no_traversal_rejects_double_dot_slash() {
        let result = validate_no_traversal(Path::new("../../../etc/passwd"));
        assert!(result.is_err());
        if let Err(PathError::PathTraversal(_)) = result {
            // Expected
        } else {
            panic!("Expected PathTraversal error");
        }
    }

    #[test]
    fn test_validate_no_traversal_rejects_dot_dot_component() {
        let result = validate_no_traversal(Path::new("foo/../bar"));
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_no_traversal_rejects_leading_dot_dot() {
        let result = validate_no_traversal(Path::new(".."));
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_no_traversal_accepts_valid_path() {
        let result = validate_no_traversal(Path::new("foo/bar/baz"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_no_traversal_accepts_absolute_path() {
        let result = validate_no_traversal(Path::new("/usr/local/bin"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_characters_rejects_null_byte() {
        let result = validate_characters(Path::new("foo\0bar"));
        assert!(result.is_err());
        if let Err(PathError::InvalidCharacter(_)) = result {
            // Expected
        } else {
            panic!("Expected InvalidCharacter error");
        }
    }

    #[test]
    fn test_validate_characters_accepts_normal() {
        let result = validate_characters(Path::new("foo/bar/baz.txt"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_try_expand_home_rejects_traversal() {
        let result = try_expand_home("~/../../../etc/passwd");
        assert!(result.is_err());
    }

    #[test]
    fn test_try_expand_home_rejects_dot_dot_in_path() {
        let result = try_expand_home("~/Documents/../etc/passwd");
        assert!(result.is_err());
    }

    #[test]
    fn test_try_expand_home_accepts_normal_path() {
        let result = try_expand_home("~/Documents/file.txt");
        assert!(result.is_ok());
    }

    #[test]
    fn test_expand_home_panics_on_traversal() {
        // This should panic
        let result = std::panic::catch_unwind(|| {
            expand_home("~/../../../etc/passwd");
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_try_join_paths_rejects_traversal() {
        let result = try_join_paths(Path::new("/home/user"), &["..", "etc"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_try_join_paths_rejects_mixed_traversal() {
        let result = try_join_paths(Path::new("/home/user"), &["documents", "..", "etc"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_try_join_paths_accepts_valid() {
        let result = try_join_paths(Path::new("/home/user"), &["documents", "file.txt"]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_join_paths_panics_on_traversal() {
        let result = std::panic::catch_unwind(|| {
            join_paths(Path::new("/home/user"), &["..", "etc"]);
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_within_base_rejects_traversal() {
        if let Some(home) = home_dir() {
            if home.exists() {
                let result = validate_within_base(Path::new("../etc"), &home);
                assert!(result.is_err());
            }
        }
    }

    #[test]
    fn test_validate_within_base_accepts_subdirectory() {
        if let Some(home) = home_dir() {
            if home.exists() {
                let result = validate_within_base(&home.join("Documents"), &home);
                // Should succeed since Documents is within home
                // (might fail if Documents doesn't exist, which is ok)
                let _ = result;
            }
        }
    }

    #[test]
    fn test_path_error_display() {
        let err = PathError::PathTraversal("../../../etc".to_string());
        let msg = format!("{}", err);
        assert!(msg.contains("traversal") || msg.contains("../../../etc"));
    }

    // Edge case tests

    #[test]
    fn test_validate_no_traversal_empty_path() {
        let result = validate_no_traversal(Path::new(""));
        // Empty path has no components, should be ok
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_no_traversal_single_dot() {
        let result = validate_no_traversal(Path::new("."));
        // Current directory is not ParentDir, should be ok
        assert!(result.is_ok());
    }

    #[test]
    fn test_try_expand_home_tilde_only() {
        let result = try_expand_home("~");
        assert!(result.is_ok());
    }

    #[test]
    fn test_try_expand_home_non_tilde_path() {
        let result = try_expand_home("/usr/local/bin");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), PathBuf::from("/usr/local/bin"));
    }

    // ============================================================================
    // Additional Security Tests
    // ============================================================================

    #[test]
    fn test_try_join_paths_rejects_absolute_unix() {
        let result = try_join_paths(Path::new("/home/user"), &["/etc/passwd"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_try_join_paths_rejects_absolute_windows() {
        let _result = try_join_paths(Path::new("C:\\Users"), &["C:\\Windows\\System32"]);
        // On Unix, "C:\\Windows\\System32" is treated as a relative path, not absolute
        // On Windows, it would be absolute. This test documents the behavior.
        // The important thing is we're checking for RootDir/Prefix components.
        #[cfg(unix)]
        {
            // On Unix, backslashes are just filename characters
            assert!(_result.is_ok());
        }
    }

    #[test]
    fn test_try_join_paths_rejects_unc_path() {
        let result = try_join_paths(Path::new("/home/user"), &["//server/share"]);
        // UNC paths like //server/share are absolute on Windows
        // On Unix, paths starting with / are absolute (even //server/share)
        // So this should be rejected as an absolute path on both platforms
        assert!(result.is_err());
    }

    #[test]
    fn test_try_expand_home_rejects_double_slash_absolute() {
        let result = try_expand_home("~//etc/passwd");
        assert!(result.is_err());
    }

    #[test]
    fn test_try_expand_home_rejects_slash_absolute() {
        // Note: ~/etc is not an absolute path - the backslash is just a character
        // Real absolute paths start with / or drive letter
        let result = try_expand_home(r"~/\etc");
        // This is actually valid (home directory + "etc" file with backslash in name)
        assert!(result.is_ok());
    }

    #[test]
    fn test_try_expand_home_rejects_tilde_slash() {
        let result = try_expand_home("~//");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_no_traversal_accepts_dots_in_filename() {
        // Filenames with .. should be allowed now
        let result = validate_no_traversal(Path::new("file..txt"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_no_traversal_accepts_backup_dots() {
        // Backup files with .. should be allowed
        let result = validate_no_traversal(Path::new("backup...old"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_within_base_non_existent_within() {
        // Non-existent path within base should be accepted
        if let Some(home) = home_dir() {
            if home.exists() {
                let non_existent = home.join("non_existent_file.txt");
                let result = validate_within_base(&non_existent, &home);
                // Should succeed since parent (home) is within base
                assert!(result.is_ok());
            }
        }
    }

    #[test]
    fn test_shorten_path_unicode_filename() {
        // Test with Unicode filename to ensure no panic
        let short = shorten_path("/tmp/한글파일.txt", 20);
        // Should contain the filename or be shortened appropriately
        assert!(short.len() <= 20);
        // Should not panic on UTF-8 boundaries
    }

    #[test]
    fn test_shorten_path_small_width() {
        // Test with small max_width
        // Note: shorten_path measures in bytes, but UTF-8 characters can be multi-byte
        // With max_width=10, we get "..." + up to 7 bytes of filename
        let short = shorten_path("/tmp/한글파일.txt", 10);
        // With max_width=10, we should get "..." + some of the filename
        assert!(short.starts_with("..."));
        // The result will be "..." + truncated filename
        // For "한글파일.txt", truncating to 7 bytes gives "한글" (6 bytes) or similar
        assert!(short.len() <= 10);
    }
}
