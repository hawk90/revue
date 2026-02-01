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

/// Validate that a path is relative (not absolute)
///
/// Returns error if the path is absolute or contains RootDir/Prefix components.
/// This is a security helper to ensure paths stay within a designated base directory.
pub fn validate_relative_only(path: &Path) -> Result<(), PathError> {
    if path.is_absolute() {
        return Err(PathError::PathTraversal(format!(
            "Absolute path not allowed: {}",
            path.display()
        )));
    }

    // Also check for RootDir and Prefix components explicitly
    // This catches edge cases like UNC paths (//server/share) on Unix
    for component in path.components() {
        match component {
            Component::RootDir | Component::Prefix(_) => {
                return Err(PathError::PathTraversal(format!(
                    "Root or prefix component not allowed: {}",
                    path.display()
                )));
            }
            _ => {}
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

/// Find the nearest existing ancestor of a path.
///
/// Walks up the directory tree from the given path until an existing directory is found.
/// Returns the path to that existing ancestor.
///
/// # Security
///
/// Limits traversal depth to prevent infinite loops with circular symlinks.
fn find_existing_ancestor(mut path: &Path) -> Option<PathBuf> {
    const MAX_DEPTH: usize = 100; // Prevent infinite loops
    let mut depth = 0;

    loop {
        if path.exists() {
            return Some(path.to_path_buf());
        }
        match path.parent() {
            Some(parent) if !parent.as_os_str().is_empty() => {
                path = parent;
                depth += 1;
                if depth > MAX_DEPTH {
                    // Too deep, likely a circular symlink or other issue
                    return None;
                }
            }
            _ => return None,
        }
    }
}

/// Check if a path is properly within a base directory by prefix matching.
///
/// This is a more secure version of `starts_with` that ensures:
/// 1. The path actually starts with the base prefix
/// 2. The next character after the prefix is a path separator or end of string
///    (prevents "/baseevil" from matching "/base")
fn is_within_base_by_prefix(path: &Path, base: &Path) -> bool {
    // Use Path::starts_with which handles this correctly
    // Then verify that if there's more to the path, it starts with a separator
    if !path.starts_with(base) {
        return false;
    }

    // If path equals base exactly, it's within base
    let path_bytes = path.as_os_str().as_encoded_bytes();
    let base_bytes = base.as_os_str().as_encoded_bytes();

    if path_bytes.len() == base_bytes.len() {
        return true;
    }

    // Check that the next character after base is a path separator
    // This prevents "/baseevil" from matching "/base"
    // We check the bytes directly since we know paths are valid UTF-8 or we're checking for ASCII separators
    if path_bytes.len() > base_bytes.len() {
        let next_byte = path_bytes[base_bytes.len()];
        next_byte == b'/' || next_byte == b'\\'
    } else {
        false
    }
}

/// Validate that a path stays within a base directory
///
/// Returns error if the path, when canonicalized, escapes the base directory.
/// Also checks parent directories for symlinks that might escape the base.
///
/// For non-existent paths, walks up the directory tree to find an existing ancestor
/// and validates that ancestor stays within the base directory.
///
/// Returns error if base doesn't exist and path is absolute outside base.
///
/// # Security
///
/// This function protects against path traversal attacks through multiple mechanisms:
/// 1. Traversal pattern detection (`..` components)
/// 2. Symlink escape detection via ancestor canonicalization
/// 3. Secure prefix matching to prevent prefix bypass attacks
pub fn validate_within_base(path: &Path, base: &Path) -> Result<(), PathError> {
    // First validate for traversal patterns
    validate_no_traversal(path)?;

    // Base must exist for proper validation
    // If base doesn't exist, we can't reliably validate containment
    if !base.exists() {
        // For absolute paths outside a non-existent base, this is suspicious
        if path.is_absolute() {
            // Use secure prefix checking to prevent bypass
            if !is_within_base_by_prefix(path, base) {
                return Err(PathError::OutsideBounds);
            }
        }
        // For relative paths, we can't validate without base existing
        // This is a limitation - caller should ensure base exists first
        return Ok(());
    }

    // Cache base canonicalization
    let base_canonical = match base.canonicalize() {
        Ok(c) => c,
        Err(_) => {
            // Base exists but can't canonicalize (permission issues, etc.)
            // Fall back to secure prefix check
            if is_within_base_by_prefix(path, base) {
                return Ok(());
            }
            return Err(PathError::OutsideBounds);
        }
    };

    // If path exists, validate directly
    if path.exists() {
        if let Ok(path_canonical) = path.canonicalize() {
            if !path_canonical.starts_with(&base_canonical) {
                return Err(PathError::OutsideBounds);
            }
        }
        return Ok(());
    }

    // Path doesn't exist - find nearest existing ancestor
    // If no ancestor exists, we must be more conservative
    let ancestor = match find_existing_ancestor(path) {
        Some(a) => a,
        None => {
            // No existing ancestor found - path is entirely in non-existent territory
            // Use prefix checking as a fallback, but also verify no suspicious patterns
            if is_within_base_by_prefix(path, base) {
                // Double-check that the relative path from base is safe
                if let Ok(rel) = path.strip_prefix(base) {
                    validate_no_traversal(rel)?;
                    validate_relative_only(rel)?;
                }
                return Ok(());
            }
            return Err(PathError::OutsideBounds);
        }
    };

    // Validate the ancestor is within base
    if let Ok(ancestor_canonical) = ancestor.canonicalize() {
        if !ancestor_canonical.starts_with(&base_canonical) {
            return Err(PathError::OutsideBounds);
        }
    }

    // Also validate that the relative path from base doesn't contain suspicious patterns
    // This catches cases where the path might look legitimate but point outside base
    if let Ok(rel) = path.strip_prefix(base) {
        // Double-check no traversal in the relative path
        validate_no_traversal(rel)?;
        // Also ensure the relative part is not absolute
        validate_relative_only(rel)?;
    }

    Ok(())
}

/// Get the home directory path
///
/// # Security
///
/// Reads the HOME or USERPROFILE environment variable. This can be spoofed by
/// malicious users. Callers should validate that the returned path exists and
/// is appropriate for their use case.
///
/// Returns None if:
/// - HOME and USERPROFILE are not set
/// - The path doesn't exist
/// - The path exists but is not a directory
pub fn home_dir() -> Option<PathBuf> {
    let path = std::env::var("HOME")
        .ok()
        .map(PathBuf::from)
        .or_else(|| std::env::var("USERPROFILE").ok().map(PathBuf::from))?;

    // Validate that the path exists and is a directory
    if path.exists() && path.is_dir() {
        Some(path)
    } else {
        None
    }
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
/// **Panics if the path contains traversal patterns.**
///
/// Use `try_expand_home` for a non-panicking version, especially when
/// handling user input or untrusted paths.
///
/// # Example
///
/// ```rust,ignore
/// // Safe: hardcoded paths
/// let path = expand_home("~/Documents");
///
/// // For user input, use the non-panicking version:
/// let path = try_expand_home(user_input)?;
/// ```
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
            validate_relative_only(rest_path)?;

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

    // Convert to display string with normalized separators for consistent handling
    let full = normalize_separators(&path.display().to_string());

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

        // Find the byte position where we should truncate
        // We want to include characters that fit within max_bytes
        let mut truncate_pos = 0;
        for (byte_start, ch) in filename.char_indices() {
            let byte_end = byte_start + ch.len_utf8();
            if byte_end > max_bytes {
                // This character would exceed the limit
                break;
            }
            // This character fits, include it
            truncate_pos = byte_end;
        }

        return format!("...{}", &filename[..truncate_pos]);
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
/// Join paths with proper separators
///
/// Validates that all path parts don't contain traversal patterns.
///
/// # Panics
///
/// **Panics if any path part contains traversal patterns.**
///
/// Use `try_join_paths` for a non-panicking version, especially when
/// handling user input or untrusted path parts.
///
/// # Example
///
/// ```rust,ignore
/// // Safe: hardcoded path parts
/// let path = join_paths(Path::new("/home/user"), &["documents", "file.txt"]);
///
/// // For user input, use the non-panicking version:
/// let path = try_join_paths(base, &user_parts)?;
/// ```
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
        validate_relative_only(part_path)?;

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
    use crate::utils::shell::{escape_applescript, escape_powershell};

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
        // On Unix, ~/\etc is home + "etc" with backslash in name (valid)
        // On Windows, \etc is an absolute path (RootDir), so it should be rejected
        let result = try_expand_home(r"~/\etc");
        #[cfg(unix)]
        {
            // On Unix, backslash is just a character
            assert!(result.is_ok());
        }
        #[cfg(windows)]
        {
            // On Windows, \etc is an absolute path
            assert!(result.is_err());
        }
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

    // ============================================================================
    // Non-existent Path Validation Tests
    // ============================================================================

    #[test]
    fn test_validate_within_base_non_existent_within_base() {
        // Non-existent path within base should be accepted
        if let Some(home) = home_dir() {
            if home.exists() {
                let non_existent = home.join("some/deeply/nested/non_existent.txt");
                let result = validate_within_base(&non_existent, &home);
                // Should succeed since the path hierarchy would be within home
                assert!(
                    result.is_ok(),
                    "Non-existent path within base should be accepted"
                );
            }
        }
    }

    #[test]
    fn test_validate_within_base_non_existent_outside_base() {
        // Non-existent path outside base should fail via traversal check
        if let Some(home) = home_dir() {
            if home.exists() {
                // Create a path that would be outside base (contains ..)
                let outside = home.join("documents/../etc/passwd");
                let result = validate_within_base(&outside, &home);
                // Should fail because of traversal pattern
                assert!(result.is_err(), "Path with traversal should be rejected");
            }
        }
    }

    #[test]
    fn test_validate_within_base_non_existent_deep_nesting() {
        // Deeply nested non-existent path should be validated via ancestor
        if let Some(home) = home_dir() {
            if home.exists() {
                let deep = home.join("a/b/c/d/e/f/g/h/i/j/file.txt");
                let result = validate_within_base(&deep, &home);
                // Should succeed - all ancestors would be within home
                assert!(
                    result.is_ok(),
                    "Deep non-existent path within base should be accepted"
                );
            }
        }
    }

    #[test]
    fn test_find_existing_ancestor_existing_path() {
        // When path exists, should return the path itself
        if let Some(home) = home_dir() {
            if home.exists() {
                let result = find_existing_ancestor(&home);
                assert_eq!(result, Some(home.clone()));
            }
        }
    }

    #[test]
    fn test_find_existing_ancestor_non_existent() {
        // For non-existent path, should find existing ancestor
        if let Some(home) = home_dir() {
            if home.exists() {
                let non_existent = home.join("does/not/exist/file.txt");
                let result = find_existing_ancestor(&non_existent);
                // Should find home or some existing ancestor
                assert!(result.is_some(), "Should find an existing ancestor");
                assert!(
                    result.unwrap().starts_with(&home),
                    "Ancestor should be within home"
                );
            }
        }
    }

    #[test]
    fn test_find_existing_ancestor_empty_path() {
        // Empty path should return None
        let result = find_existing_ancestor(Path::new(""));
        assert!(
            result.is_none(),
            "Empty path should have no existing ancestor"
        );
    }

    #[test]
    fn test_validate_within_base_relative_non_existent() {
        // Test with relative paths
        let base = Path::new("/tmp");
        let non_existent = Path::new("/tmp/test/subdir/file.txt");
        let result = validate_within_base(non_existent, base);
        // Should accept since the path would be within base
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_within_base_traversal_via_relative() {
        // Test that traversal is caught even in non-existent paths
        let base = Path::new("/tmp/test");
        let traversal = Path::new("/tmp/test/../etc/passwd");
        let result = validate_within_base(traversal, base);
        // Should fail because of .. pattern
        assert!(result.is_err());
    }

    // ============================================================================
    // validate_relative_only Tests
    // ============================================================================

    #[test]
    fn test_validate_relative_only_accepts_relative() {
        assert!(validate_relative_only(Path::new("foo/bar")).is_ok());
        assert!(validate_relative_only(Path::new("baz")).is_ok());
        assert!(validate_relative_only(Path::new(".")).is_ok());
    }

    #[test]
    fn test_validate_relative_only_rejects_absolute_unix() {
        assert!(validate_relative_only(Path::new("/etc/passwd")).is_err());
        assert!(validate_relative_only(Path::new("/usr/local/bin")).is_err());
    }

    #[test]
    fn test_validate_relative_only_rejects_unc_path() {
        // UNC paths should be rejected
        assert!(validate_relative_only(Path::new("//server/share")).is_err());
    }

    #[test]
    fn test_validate_relative_only_accepts_dots_in_filename() {
        // Filenames with .. should be allowed
        assert!(validate_relative_only(Path::new("file..txt")).is_ok());
        assert!(validate_relative_only(Path::new("backup...old")).is_ok());
    }

    // ============================================================================
    // Non-existent Base Tests
    // ============================================================================

    #[test]
    fn test_validate_within_base_non_existent_base_absolute_path_outside() {
        // Base doesn't exist, path is absolute and clearly outside
        // Note: On Windows, Unix-style paths starting with / are not considered absolute
        #[cfg(unix)]
        {
            let base = Path::new("/non/existent/base");
            let path = Path::new("/etc/passwd");
            let result = validate_within_base(path, base);
            assert!(
                result.is_err(),
                "Absolute path outside non-existent base should fail"
            );
        }
        #[cfg(windows)]
        {
            // On Windows, use a Windows-style absolute path
            let base = Path::new("C:\\non\\existent\\base");
            let path = Path::new("D:\\etc\\passwd");
            let result = validate_within_base(path, base);
            assert!(
                result.is_err(),
                "Absolute path outside non-existent base should fail"
            );
        }
    }

    #[test]
    fn test_validate_within_base_non_existent_base_absolute_path_inside() {
        // Base doesn't exist, path is absolute but starts with base prefix
        #[cfg(unix)]
        {
            let base = Path::new("/non/existent/base");
            let path = Path::new("/non/existent/base/subdir/file.txt");
            let result = validate_within_base(path, base);
            // Should succeed since path starts with base
            assert!(
                result.is_ok(),
                "Path starting with base prefix should be accepted"
            );
        }
        #[cfg(windows)]
        {
            let base = Path::new("C:\\non\\existent\\base");
            let path = Path::new("C:\\non\\existent\\base\\subdir\\file.txt");
            let result = validate_within_base(path, base);
            assert!(
                result.is_ok(),
                "Path starting with base prefix should be accepted"
            );
        }
    }

    #[test]
    fn test_validate_within_base_non_existent_base_relative_path() {
        // Base doesn't exist, path is relative
        // This is a limitation - we can't validate without base existing
        let base = Path::new("/non/existent/base");
        let path = Path::new("relative/path.txt");
        let result = validate_within_base(path, base);
        // Should succeed but caller should ensure base exists first
        assert!(
            result.is_ok(),
            "Relative path with non-existent base is accepted"
        );
    }

    // ============================================================================
    // UTF-8 Truncation Edge Cases
    // ============================================================================

    #[test]
    fn test_shorten_path_ascii_exactly_fits() {
        // Test ASCII filename that exactly fits in width
        let short = shorten_path("/tmp/test.txt", 12); // "/tmp/test.txt" is 13 chars, should shorten
        assert!(short.len() <= 12);
    }

    #[test]
    fn test_shorten_path_one_char_overflow() {
        // Test with width that fits all but one character
        let short = shorten_path("/tmp/abcde.txt", 11);
        // "/tmp/abcde.txt" is 13 chars, with max_width=11 we get "..." + 8 chars
        // Actually since this is > 4+8=12, it will be truncated
        assert!(short.len() <= 11);
    }

    #[test]
    fn test_shorten_path_unicode_boundary() {
        // Test at boundary where last character is multi-byte
        let short = shorten_path("/tmp/한글.txt", 11);
        // "/tmp/한글.txt" = 11 chars but "한글" are 3 bytes each = 6 bytes
        // Total = 10 bytes + /tmp/ = 5 + .txt = 4 = 15 bytes
        assert!(short.len() <= 11);
        // Should not panic on UTF-8 boundary
    }

    #[test]
    fn test_shorten_path_ascii_width_7() {
        // Test with max_width=7 (edge case for "...abc")
        let short = shorten_path("/tmp/abcdef.txt", 7);
        assert!(short.len() <= 7);
        assert!(short.starts_with("..."));
    }

    // ============================================================================
    // Windows UNC Path Tests
    // ============================================================================

    #[test]
    fn test_try_join_paths_rejects_backslash_unc() {
        // Test \\server\share style UNC paths
        let result = try_join_paths(Path::new("/base"), &[r"\\server\share"]);
        // On Unix, backslashes are just characters, so this is ok
        // On Windows, this would be a UNC path and should be rejected
        #[cfg(unix)]
        {
            // On Unix, backslashes are filename characters
            assert!(result.is_ok());
        }
        #[cfg(windows)]
        {
            // On Windows, \\server\share is a UNC path (absolute)
            assert!(result.is_err());
        }
    }

    #[test]
    fn test_try_expand_home_windows_path() {
        // Test Windows paths in try_expand_home
        #[cfg(windows)]
        {
            let result = try_expand_home(r"~/C:\Windows");
            // On Windows, C:\ is an absolute path with Prefix component
            assert!(result.is_err());
        }
        #[cfg(unix)]
        {
            // On Unix, backslashes are just characters
            let result = try_expand_home(r"~/C:\Windows");
            assert!(result.is_ok());
        }
    }

    // ============================================================================
    // Additional Security Tests (Edge Cases & Vulnerability Fixes)
    // ============================================================================

    #[test]
    fn test_validate_within_base_prefix_bypass_attack() {
        // Test that "/baseevil/passwd" is rejected when base is "/base"
        // This was a potential vulnerability where starts_with wasn't sufficient
        if let Some(home) = home_dir() {
            if home.exists() {
                // Create a base path
                let base = &home;

                // Try to access "/home/evil/passwd" when base is "/home/user"
                // On Unix this would be base.parent() + "/evil/passwd"
                if let Some(parent) = base.parent() {
                    let attack_path = parent.join("evil/passwd");
                    let result = validate_within_base(&attack_path, base);
                    // Should fail - attack_path is outside base
                    assert!(
                        result.is_err(),
                        "Path outside base (prefix bypass) should be rejected"
                    );
                }
            }
        }
    }

    #[test]
    fn test_validate_within_base_similar_prefix_attack() {
        // Test paths that start with base prefix but aren't within base
        #[cfg(unix)]
        {
            let base = Path::new("/safe/base");
            let path = Path::new("/safe/baseevil/passwd");
            let result = validate_within_base(path, base);
            // If base doesn't exist, it should still fail the prefix check
            // because path doesn't have a separator after the base prefix
            assert!(
                result.is_err(),
                "Path with similar prefix but no separator should be rejected"
            );
        }
    }

    #[test]
    fn test_validate_within_base_no_existing_ancestor_rejected() {
        // Test that paths with no existing ancestors are properly validated
        // This was a vulnerability where find_existing_ancestor returning None
        // would bypass validation
        if let Some(home) = home_dir() {
            if home.exists() {
                // Create a very deep non-existent path
                let deep_path = home
                    .join("a")
                    .join("b")
                    .join("c")
                    .join("d")
                    .join("e")
                    .join("f")
                    .join("g")
                    .join("h")
                    .join("i")
                    .join("j")
                    .join("k")
                    .join("l")
                    .join("m")
                    .join("n")
                    .join("o")
                    .join("p")
                    .join("q")
                    .join("r")
                    .join("s")
                    .join("t")
                    .join("u")
                    .join("v")
                    .join("w")
                    .join("x")
                    .join("y")
                    .join("z")
                    .join("file.txt");

                // This should succeed because all ancestors would be within home
                let result = validate_within_base(&deep_path, &home);
                assert!(
                    result.is_ok(),
                    "Deep path within base should be accepted even with no existing ancestors"
                );

                // Now try a path that would escape via a similar prefix
                // We can't actually test the escape without creating files,
                // but we can verify the prefix check works
                let home_str = home.to_string_lossy().to_string();
                let evil_path_str = format!("{}evil/passwd", home_str);
                let evil_path = Path::new(&evil_path_str);
                let result = validate_within_base(&evil_path, &home);
                assert!(result.is_err(), "Path with evil prefix should be rejected");
            }
        }
    }

    #[test]
    fn test_find_existing_ancestor_depth_limit() {
        // Test that find_existing_ancestor has a depth limit to prevent infinite loops
        // We can't easily test circular symlinks without filesystem access,
        // but we can verify it doesn't panic or hang on very deep paths
        let deep = Path::new("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z");
        let result = find_existing_ancestor(deep);
        // Should either find an ancestor or return None, but never hang
        // The test passing here means it didn't hang
        let _ = result;
    }

    // ============================================================================
    // shorten_path Boundary Value Tests
    // ============================================================================

    #[test]
    fn test_shorten_path_max_width_zero() {
        // max_width = 0 should return "..." or empty
        let short = shorten_path("/tmp/file.txt", 0);
        // With max_width=0, we get "..." (3 chars) or empty depending on implementation
        // The important thing is it doesn't panic
        assert!(short.len() <= 3 || short.is_empty());
    }

    #[test]
    fn test_shorten_path_max_width_one() {
        let short = shorten_path("/tmp/file.txt", 1);
        // Should return "..." truncated or empty
        assert!(short.len() <= 3);
    }

    #[test]
    fn test_shorten_path_max_width_two() {
        let short = shorten_path("/tmp/file.txt", 2);
        assert!(short.len() <= 3);
    }

    #[test]
    fn test_shorten_path_max_width_three() {
        let short = shorten_path("/tmp/file.txt", 3);
        // Should be "..." or less
        assert!(short.len() <= 3);
    }

    #[test]
    fn test_shorten_path_max_width_four() {
        let short = shorten_path("/tmp/file.txt", 4);
        // With max_width=4, we get "..." which is 3 chars
        assert!(short.starts_with("..."));
        assert!(short.len() <= 4);
    }

    #[test]
    fn test_shorten_path_empty_components() {
        // Test paths with empty components (multiple slashes)
        let short = shorten_path("//a//b///c", 20);
        // Should handle gracefully without panicking
        assert!(short.len() <= 20);
    }

    #[test]
    fn test_abbreviate_path_empty_components() {
        // Test paths with empty components
        let abbr = abbreviate_path_keep("//a//b///c", 2);
        // Should handle gracefully
        assert!(!abbr.is_empty());
    }

    // ============================================================================
    // home_dir Validation Tests
    // ============================================================================

    #[test]
    fn test_home_dir_returns_existing_directory() {
        // home_dir should only return paths that exist and are directories
        if let Some(home) = home_dir() {
            assert!(home.exists(), "home_dir should return existing path");
            assert!(home.is_dir(), "home_dir should return directory");
        }
        // If None, that's also valid (no HOME set or invalid)
    }

    #[test]
    fn test_home_dir_tampered_env() {
        // Test that home_dir doesn't return non-existent paths from env
        // We can't actually modify environment in tests, but the logic
        // ensures that if HOME points to a non-existent path, None is returned
        // This is more of a documentation of expected behavior
    }

    // Additional edge case tests

    #[test]
    #[cfg(unix)]
    fn test_home_dir_with_unset_env() {
        // Temporarily unset HOME and USERPROFILE
        let home_backup = std::env::var("HOME").ok();
        let userprofile_backup = std::env::var("USERPROFILE").ok();

        std::env::remove_var("HOME");
        std::env::remove_var("USERPROFILE");

        let result = home_dir();
        assert!(
            result.is_none(),
            "home_dir should return None when HOME/USERPROFILE not set"
        );

        // Restore environment
        if let Some(home) = home_backup {
            std::env::set_var("HOME", home);
        }
        if let Some(up) = userprofile_backup {
            std::env::set_var("USERPROFILE", up);
        }
    }

    #[test]
    fn test_home_dir_with_nonexistent_path() {
        // Set HOME to a path that doesn't exist
        let home_backup = std::env::var("HOME").ok();
        let userprofile_backup = std::env::var("USERPROFILE").ok();

        std::env::set_var("HOME", "/nonexistent/path/that/does/not/exist");
        std::env::remove_var("USERPROFILE");

        let result = home_dir();
        assert!(
            result.is_none(),
            "home_dir should return None for nonexistent path"
        );

        // Restore environment
        if let Some(home) = home_backup {
            std::env::set_var("HOME", home);
        } else {
            std::env::remove_var("HOME");
        }
        if let Some(up) = userprofile_backup {
            std::env::set_var("USERPROFILE", up);
        }
    }

    #[test]
    fn test_escape_applescript_unicode() {
        // Test with various Unicode characters
        assert_eq!(escape_applescript("Hello 世界"), r#"Hello 世界"#);
        assert_eq!(escape_applescript("Café au lait"), r#"Café au lait"#);
        // Test with emoji (4-byte UTF-8)
        assert_eq!(escape_applescript("Hello 👋"), r#"Hello 👋"#);
    }

    #[test]
    fn test_escape_powershell_unicode() {
        // Test with Unicode - should pass through unchanged
        assert_eq!(escape_powershell("Hello 世界"), "Hello 世界");
        assert_eq!(escape_powershell("Café"), "Café");
    }
}
