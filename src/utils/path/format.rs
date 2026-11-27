use std::path::Path;

use crate::utils::path::util::normalize_separators;
use crate::utils::unicode::{display_width, truncate_to_width};

/// Shorten a path to fit within a maximum width
///
/// The width is in terminal columns, so a wide (CJK) character counts as two.
/// Replaces leading directories with "..." if needed.
///
/// # Example
///
/// ```
/// use revue::utils::path::shorten_path;
///
/// let short = shorten_path("/very/long/path/to/file.txt", 20);
/// assert_eq!(short, ".../path/to/file.txt");
/// ```
pub fn shorten_path(path: impl AsRef<Path>, max_width: usize) -> String {
    let path = path.as_ref();

    // Convert to display string with normalized separators for consistent handling
    let full = normalize_separators(&path.display().to_string());

    if display_width(&full) <= max_width {
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

    if display_width(filename) + 4 > max_width {
        // Below 3 columns not even "..." fits: cut it to the width, as
        // `truncate_with_suffix` cuts its suffix (#1534)
        if max_width < 3 {
            return truncate_to_width("...", max_width).to_string();
        }
        // Even filename doesn't fit, truncate it to the columns left after "..."
        let truncated = truncate_to_width(filename, max_width - 3);
        return format!("...{}", truncated);
    }

    // Try to include as many trailing components as possible
    let mut result = String::new();
    let mut result_width = 0;
    let mut included = 0;

    for (i, component) in components.iter().rev().enumerate() {
        let needed = if i == 0 {
            display_width(component)
        } else {
            display_width(component) + 1 // +1 for separator
        };

        if result_width + needed + 4 <= max_width {
            result_width += needed;
            if i > 0 {
                result = format!("{}/{}", component, result);
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
    } else if full.starts_with('/') {
        format!("/{}", result)
    } else {
        // Every component fits once empty ones (`a//b`) are dropped; a
        // relative path stays relative (#1534)
        result
    }
}

/// Abbreviate path by shortening middle directories to first character
///
/// Keeps the last N components fully visible.
///
/// # Example
///
/// ```
/// use revue::utils::path::abbreviate_path;
///
/// let abbr = abbreviate_path("/Users/john/Documents/Projects/rust/main.rs");
/// assert_eq!(abbr, "/U/j/D/P/rust/main.rs");
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
