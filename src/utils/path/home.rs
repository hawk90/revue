use std::path::{Path, PathBuf};

use crate::utils::path::error::PathError;
use crate::utils::path::validate::validate_no_traversal;
use crate::utils::path::validate::validate_relative_only;

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
/// Returns error if the path contains traversal patterns or would escape home directory.
///
/// # Example
///
/// ```rust,ignore
/// // Use with proper error handling:
/// let path = expand_home(user_input)?;
///
/// // For hardcoded safe paths, use unwrap():
/// let path = expand_home("~/Documents").unwrap();
/// ```
pub fn expand_home(path: impl AsRef<Path>) -> Result<PathBuf, PathError> {
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
