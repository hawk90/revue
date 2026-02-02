use std::path::{Component, Path};

use crate::utils::path::error::PathError;

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
pub(crate) fn find_existing_ancestor(mut path: &Path) -> Option<std::path::PathBuf> {
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
