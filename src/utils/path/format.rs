use std::path::Path;

use crate::utils::path::util::normalize_separators;

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
    let filename = match components.last() {
        Some(f) => f,
        // This is unreachable due to is_empty check above
        None => return full,
    };

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

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // shorten_path tests
    // =========================================================================

    #[test]
    fn test_shorten_path_no_shortening_needed() {
        let path = "/usr/local/bin";
        let result = shorten_path(path, 100);
        assert_eq!(result, "/usr/local/bin");
    }

    #[test]
    fn test_shorten_path_exact_fit() {
        let path = "/usr/bin";
        let result = shorten_path(path, 7);
        // Function adds "..." prefix when approaching limit
        assert!(result.contains("bin"));
    }

    #[test]
    fn test_shorten_path_preserves_filename() {
        let path = "/very/long/path/to/file.txt";
        let result = shorten_path(path, 20);
        // Should include filename
        assert!(result.contains("file.txt"));
    }

    #[test]
    fn test_shorten_path_with_ellipsis() {
        let path = "/a/b/c/d/e/f/g/h/file.txt";
        let result = shorten_path(path, 20);
        // Should start with "..."
        assert!(result.starts_with("..."));
    }

    #[test]
    fn test_shorten_path_single_component() {
        let path = "file.txt";
        let result = shorten_path(path, 20);
        assert_eq!(result, "file.txt");
    }

    #[test]
    fn test_shorten_path_empty() {
        let path = "";
        let result = shorten_path(path, 10);
        assert_eq!(result, "");
    }

    // =========================================================================
    // abbreviate_path tests
    // =========================================================================

    #[test]
    fn test_abbreviate_path_short_path() {
        let path = "/usr/bin";
        let result = abbreviate_path(path);
        // Path is too short to abbreviate
        assert_eq!(result, "/usr/bin");
    }

    #[test]
    fn test_abbreviate_path_long_path() {
        let path = "/Users/john/Documents/Projects/rust/main.rs";
        let result = abbreviate_path(path);
        // Should abbreviate middle components
        assert!(result.contains("/U/"));
        assert!(result.contains("main.rs"));
    }

    #[test]
    fn test_abbreviate_path_no_leading_slash() {
        let path = "Users/john/Documents/file.txt";
        let result = abbreviate_path(path);
        // Should work without leading slash
        assert!(result.contains("file.txt"));
    }

    // =========================================================================
    // abbreviate_path_keep tests
    // =========================================================================

    #[test]
    fn test_abbreviate_path_keep_zero() {
        let path = "/a/b/c/d/e";
        let result = abbreviate_path_keep(path, 0);
        // Should abbreviate all but last
        assert!(result.contains("/e"));
    }

    #[test]
    fn test_abbreviate_path_keep_one() {
        let path = "/a/b/c/d/e";
        let result = abbreviate_path_keep(path, 1);
        // Should keep only last component
        assert!(result.ends_with("/e"));
    }

    #[test]
    fn test_abbreviate_path_keep_two() {
        let path = "/a/b/c/d/e";
        let result = abbreviate_path_keep(path, 2);
        // Should keep last two components
        assert!(result.contains("/d/e"));
    }

    #[test]
    fn test_abbreviate_path_keep_all() {
        let path = "/a/b/c";
        let result = abbreviate_path_keep(path, 10);
        // Should keep all components when keep is large enough
        assert_eq!(result, "/a/b/c");
    }

    #[test]
    fn test_abbreviate_path_keep_with_leading_slash() {
        let path = "/usr/local/bin";
        let result = abbreviate_path_keep(path, 1);
        assert!(result.starts_with('/'));
    }

    // =========================================================================
    // relative_to tests
    // =========================================================================

    #[test]
    fn test_relative_to_direct_child() {
        let base = Path::new("/home/user");
        let path = Path::new("/home/user/documents");
        let result = relative_to(path, base);
        assert_eq!(result, "documents");
    }

    #[test]
    fn test_relative_to_nested() {
        let base = Path::new("/home/user");
        let path = Path::new("/home/user/projects/rust/app");
        let result = relative_to(path, base);
        assert_eq!(result, "projects/rust/app");
    }

    #[test]
    fn test_relative_to_same_path() {
        let base = Path::new("/home/user");
        let path = Path::new("/home/user");
        let result = relative_to(path, base);
        assert_eq!(result, "");
    }

    #[test]
    fn test_relative_to_no_common_prefix() {
        let base = Path::new("/home/user");
        let path = Path::new("/var/log");
        let result = relative_to(path, base);
        // Should return full path when no common prefix
        assert_eq!(result, "/var/log");
    }

    #[test]
    fn test_relative_to_partial_overlap() {
        let base = Path::new("/home/user1");
        let path = Path::new("/home/user2");
        let result = relative_to(path, base);
        // No true common prefix, should return full path
        assert_eq!(result, "/home/user2");
    }
}
