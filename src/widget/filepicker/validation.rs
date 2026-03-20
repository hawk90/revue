//! Validation and security utilities for the file picker

use std::io;
use std::path::{Path, PathBuf};

/// Truncate a string safely at UTF-8 character boundaries
///
/// Returns a string truncated to at most `max_len` bytes,
/// ensuring we don't cut in the middle of a multi-byte UTF-8 character.
pub(crate) fn truncate_string_safe(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        return s.to_string();
    }

    // Find the last valid UTF-8 boundary before max_len
    let mut end = max_len;
    while !s.is_char_boundary(end) {
        end -= 1;
        if end == 0 {
            // Character is too long, return first character only
            return format!("{}...", s.chars().next().unwrap_or('�'));
        }
    }

    format!("{}...", &s[..end])
}

/// Error type for file picker operations
#[derive(Debug, thiserror::Error)]
pub enum FilePickerError {
    /// Path traversal detected
    #[error("Path traversal detected: {0}")]
    PathTraversal(String),

    /// Path is outside allowed directory
    #[error("Path is outside allowed directory")]
    OutsideAllowedDirectory,

    /// Invalid path
    #[error("Invalid path: {0}")]
    InvalidPath(String),

    /// Path contains invalid characters (null bytes, etc.)
    #[error("Path contains invalid characters")]
    InvalidCharacters,

    /// Path contains Windows reserved device name
    #[error("Path contains Windows reserved device name: {0}")]
    ReservedDeviceName(String),

    /// Symlink detected and not allowed
    #[error("Symbolic links are not allowed")]
    SymlinkNotAllowed,

    /// IO error
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
}

/// Validate that path doesn't contain invalid characters
///
/// Returns error if the path contains null bytes or other invalid sequences.
pub(crate) fn validate_path_characters(path: &Path) -> Result<(), FilePickerError> {
    // Check for null bytes in path
    let path_str = path.to_string_lossy();
    if path_str.contains('\0') {
        return Err(FilePickerError::InvalidCharacters);
    }

    // Check each component for null bytes
    for component in path.components() {
        if let std::path::Component::Normal(os_str) = component {
            if os_str.as_encoded_bytes().contains(&b'\0') {
                return Err(FilePickerError::InvalidCharacters);
            }
        }
    }

    Ok(())
}

/// Check for Windows reserved device names
///
/// Returns error if path contains Windows reserved names like CON, PRN, AUX, etc.
#[cfg(windows)]
pub(crate) fn validate_windows_device_names(path: &Path) -> Result<(), FilePickerError> {
    use std::path::Component;

    let reserved = [
        "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8",
        "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
    ];

    for component in path.components() {
        if let Component::Normal(name) = component {
            if let Some(name_str) = name.to_str() {
                let name_upper = name_str.to_uppercase();
                // Check exact match (CON) or match with extension (CON.txt)
                let base_name = name_upper.split('.').next().unwrap_or(&name_upper);
                if reserved.contains(&base_name) {
                    return Err(FilePickerError::ReservedDeviceName(base_name.to_string()));
                }
            }
        }
    }

    Ok(())
}

#[cfg(not(windows))]
pub(crate) fn validate_windows_device_names(_path: &Path) -> Result<(), FilePickerError> {
    Ok(())
}

/// Validate that path doesn't contain traversal patterns
///
/// Returns error if the path contains `../` or other traversal sequences.
pub(crate) fn validate_path_no_traversal(path: &Path) -> Result<(), FilePickerError> {
    // Check components using the Path API (more reliable than string checks)
    for component in path.components() {
        match component {
            std::path::Component::ParentDir => {
                return Err(FilePickerError::PathTraversal(
                    "path contains parent directory component".to_string(),
                ));
            }
            std::path::Component::Normal(_) => {
                // Normal component, continue
            }
            std::path::Component::Prefix(prefix) => {
                // Validate Windows prefix paths
                // Device namespace paths like \\.\COM1 are potentially dangerous
                // We check the string representation for device namespace indicators
                let prefix_str = prefix.as_os_str().to_string_lossy();
                if prefix_str.starts_with("\\\\?\\") || prefix_str.starts_with("\\\\.") {
                    // These are potentially dangerous device namespace paths
                    // Allow only for normal disk access, reject suspicious patterns
                    if prefix_str.contains('\\') && prefix_str.len() < 10 {
                        // Short device paths like \\.\C: are suspicious
                        return Err(FilePickerError::InvalidPath(
                            "suspicious device namespace path".to_string(),
                        ));
                    }
                }
            }
            std::path::Component::RootDir => {
                // Root is always valid
            }
            std::path::Component::CurDir => {
                // Current directory reference (.) is fine
            }
        }
    }

    Ok(())
}

/// Validate a path for security issues only (doesn't check existence or bounds)
///
/// This is used for `start_dir()` to allow setting paths that don't exist yet.
pub(crate) fn validate_security_only(path: &Path) -> Result<PathBuf, FilePickerError> {
    // Check for invalid characters first
    validate_path_characters(path)?;

    // Check for traversal patterns
    validate_path_no_traversal(path)?;

    // Check Windows device names
    validate_windows_device_names(path)?;

    Ok(path.to_path_buf())
}

/// Validate and canonicalize a path
///
/// Returns the canonical form of the path, or an error if validation fails.
/// This requires the path to exist and be within the allowed directory.
///
/// Security considerations:
/// - Checks for path traversal before filesystem access
/// - Validates symlinks don't escape the allowed directory
/// - Uses canonicalization to resolve all path components
pub(crate) fn validate_and_canonicalize(
    path: &Path,
    base_dir: &Path,
) -> Result<PathBuf, FilePickerError> {
    // Validate characters before any filesystem access
    validate_path_characters(path)?;

    // Check for traversal patterns before canonicalization
    validate_path_no_traversal(path)?;

    // Check Windows device names
    validate_windows_device_names(path)?;

    // Canonicalize the base directory once (if it exists)
    let base_canonical = if base_dir.exists() {
        Some(
            base_dir
                .canonicalize()
                .map_err(|_| FilePickerError::InvalidPath("invalid base directory".to_string()))?,
        )
    } else {
        None
    };

    // Try to canonicalize the target path
    let canonical = path.canonicalize().map_err(|_| {
        FilePickerError::InvalidPath("path does not exist or cannot be accessed".to_string())
    })?;

    // Check if the canonical path is within base directory bounds
    if let Some(ref base) = base_canonical {
        // Verify the canonical path starts with the base canonical path
        // This prevents symlink escapes and other boundary violations
        if !canonical.starts_with(base) {
            return Err(FilePickerError::OutsideAllowedDirectory);
        }

        // Additional check: verify we're not escaping via ".." components
        // by ensuring the canonical path length is >= base length
        if canonical.as_os_str().len() < base.as_os_str().len() {
            return Err(FilePickerError::OutsideAllowedDirectory);
        }
    }

    Ok(canonical)
}
