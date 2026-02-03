use std::path::Path;

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
