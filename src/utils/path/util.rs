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
/// ```rust,ignore
/// // Use with proper error handling:
/// let path = join_paths(base, &parts)?;
///
/// // For hardcoded safe paths, use unwrap():
/// let path = join_paths(Path::new("/home/user"), &["documents", "file.txt"]).unwrap();
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
