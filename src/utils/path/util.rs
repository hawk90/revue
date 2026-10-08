use std::path::{Path, PathBuf};

use crate::utils::path::error::PathError;
use crate::utils::path::validate::validate_no_traversal;
use crate::utils::path::validate::validate_relative_only;

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
/// Returns error if any path part contains traversal patterns or is absolute.
///
/// # Example
///
/// ```
/// use revue::utils::path::join_paths;
///
/// let path = join_paths("/home/user", &["documents", "file.txt"]).unwrap();
/// assert!(path.ends_with("documents/file.txt"));
///
/// // A part that climbs out of the base is refused
/// assert!(join_paths("/home/user", &["..", "etc"]).is_err());
/// ```
pub fn join_paths(base: impl AsRef<Path>, parts: &[&str]) -> Result<PathBuf, PathError> {
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
