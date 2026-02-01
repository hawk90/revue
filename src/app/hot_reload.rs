//! Hot reload support for CSS stylesheets

use crate::constants::DEBOUNCE_FILE_SYSTEM;
use notify::{Event, EventKind, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver};
use std::time::{Duration, Instant};

/// Validate and sanitize a path to prevent path traversal attacks
///
/// Returns an error if the path:
/// - Contains excessive parent directory references (`../`)
/// - Attempts to escape the current directory
/// - Contains null bytes
fn validate_path(path: &Path) -> Result<(), PathValidationError> {
    let path_str = path.to_string_lossy();

    // Check for null bytes (potential for string truncation attacks)
    if path_str.contains('\0') {
        return Err(PathValidationError::NullByte);
    }

    // Normalize the path and check if it escapes the current directory
    match path.canonicalize() {
        Ok(canonical) => {
            // Get current directory for comparison
            let current_dir =
                std::env::current_dir().map_err(|_| PathValidationError::CurrentDirAccess)?;

            // Check if canonical path starts with current directory
            // This prevents watching sensitive system files outside the project
            if !canonical.starts_with(&current_dir) {
                return Err(PathValidationError::EscapeAttempt {
                    path: path.to_path_buf(),
                    canonical,
                });
            }

            Ok(())
        }
        Err(_) => {
            // If path doesn't exist yet, check the parent directory
            if let Some(parent) = path.parent() {
                if parent.exists() {
                    match parent.canonicalize() {
                        Ok(canonical_parent) => {
                            let current_dir = std::env::current_dir()
                                .map_err(|_| PathValidationError::CurrentDirAccess)?;

                            if !canonical_parent.starts_with(&current_dir) {
                                return Err(PathValidationError::ParentEscapeAttempt {
                                    path: path.to_path_buf(),
                                    parent_canonical: canonical_parent,
                                });
                            }
                        }
                        Err(_) => {
                            return Err(PathValidationError::InvalidPath {
                                path: path.to_path_buf(),
                            });
                        }
                    }
                }
            }
            // Path doesn't exist and parent doesn't exist - allow it (might be created later)
            Ok(())
        }
    }
}

/// Errors that can occur during path validation
#[derive(Debug, Clone, PartialEq)]
pub enum PathValidationError {
    /// Path contains null byte
    NullByte,
    /// Path attempts to escape the current directory
    EscapeAttempt { path: PathBuf, canonical: PathBuf },
    /// Parent directory attempts to escape the current directory
    ParentEscapeAttempt {
        path: PathBuf,
        parent_canonical: PathBuf,
    },
    /// Invalid path that cannot be canonicalized
    InvalidPath { path: PathBuf },
    /// Cannot access current directory
    CurrentDirAccess,
}

impl std::fmt::Display for PathValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NullByte => write!(f, "Path contains null byte"),
            Self::EscapeAttempt { path, canonical } => write!(
                f,
                "Path escapes current directory: {:?} -> {:?}",
                path, canonical
            ),
            Self::ParentEscapeAttempt {
                path,
                parent_canonical,
            } => write!(
                f,
                "Parent directory escapes current directory: {:?} -> {:?}",
                path, parent_canonical
            ),
            Self::InvalidPath { path } => {
                write!(f, "Invalid path that cannot be canonicalized: {:?}", path)
            }
            Self::CurrentDirAccess => write!(f, "Cannot access current directory"),
        }
    }
}

impl std::error::Error for PathValidationError {}

/// Hot reload event
#[derive(Debug, Clone)]
pub enum HotReloadEvent {
    /// A stylesheet was modified
    StylesheetChanged(PathBuf),
    /// A file was created
    FileCreated(PathBuf),
    /// A file was deleted
    FileDeleted(PathBuf),
    /// Watcher error
    Error(String),
}

/// Hot reload watcher configuration
pub struct HotReloadConfig {
    /// Debounce duration to avoid rapid duplicate events
    pub debounce: Duration,
    /// Watch recursively
    pub recursive: bool,
}

impl Default for HotReloadConfig {
    fn default() -> Self {
        Self {
            debounce: DEBOUNCE_FILE_SYSTEM,
            recursive: true,
        }
    }
}

/// Hot reload watcher
pub struct HotReload {
    _watcher: notify::RecommendedWatcher,
    receiver: Receiver<HotReloadEvent>,
    watched_paths: Vec<PathBuf>,
    /// Per-path debounce tracking: maps (event_type_variant, path) -> last_event_time
    /// This prevents different files from debouncing each other
    last_events: HashMap<String, Instant>,
    debounce: Duration,
}

impl HotReload {
    /// Create a new hot reload watcher
    ///
    /// # Errors
    ///
    /// Returns `notify::Error` if the file system watcher cannot be created.
    /// This can happen if:
    /// - The operating system doesn't support file watching
    /// - The watcher limit has been reached
    /// - Insufficient permissions
    pub fn new() -> Result<Self, notify::Error> {
        Self::with_config(HotReloadConfig::default())
    }

    /// Create with custom configuration
    ///
    /// # Errors
    ///
    /// Returns `notify::Error` if the file system watcher cannot be created
    /// with the specified configuration.
    pub fn with_config(config: HotReloadConfig) -> Result<Self, notify::Error> {
        let (tx, rx) = channel();
        let sender = tx.clone();

        let watcher =
            notify::recommended_watcher(
                move |result: Result<Event, notify::Error>| match result {
                    Ok(event) => {
                        let reload_event = match event.kind {
                            EventKind::Modify(_) => event
                                .paths
                                .first()
                                .map(|p| HotReloadEvent::StylesheetChanged(p.clone())),
                            EventKind::Create(_) => event
                                .paths
                                .first()
                                .map(|p| HotReloadEvent::FileCreated(p.clone())),
                            EventKind::Remove(_) => event
                                .paths
                                .first()
                                .map(|p| HotReloadEvent::FileDeleted(p.clone())),
                            _ => None,
                        };

                        if let Some(e) = reload_event {
                            let _ = sender.send(e);
                        }
                    }
                    Err(e) => {
                        let _ = sender.send(HotReloadEvent::Error(e.to_string()));
                    }
                },
            )?;

        Ok(Self {
            _watcher: watcher,
            receiver: rx,
            watched_paths: Vec::new(),
            last_events: HashMap::new(),
            debounce: config.debounce,
        })
    }

    /// Watch a file or directory for changes
    ///
    /// Directories are watched recursively. Files are watched non-recursively.
    ///
    /// # Security
    ///
    /// This function validates paths to prevent path traversal attacks.
    /// Paths attempting to escape the current directory will be rejected.
    ///
    /// # Errors
    ///
    /// Returns `notify::Error` if:
    /// - The path doesn't exist
    /// - Insufficient permissions to watch the path
    /// - The watcher limit has been reached
    ///
    /// Returns `PathValidationError` if:
    /// - The path contains null bytes
    /// - The path attempts to escape the current directory (e.g., `../../../etc/passwd`)
    pub fn watch(&mut self, path: impl AsRef<Path>) -> Result<(), notify::Error> {
        let path = path.as_ref().to_path_buf();

        // Validate path to prevent traversal attacks
        validate_path(&path).map_err(|e| {
            notify::Error::new(notify::ErrorKind::Io(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                format!("Path validation failed: {}", e),
            )))
        })?;

        let mode = if path.is_dir() {
            RecursiveMode::Recursive
        } else {
            RecursiveMode::NonRecursive
        };

        self._watcher.watch(&path, mode)?;
        self.watched_paths.push(path);
        Ok(())
    }

    /// Unwatch a previously watched path
    ///
    /// # Errors
    ///
    /// Returns `notify::Error` if the path was not being watched.
    pub fn unwatch(&mut self, path: impl AsRef<Path>) -> Result<(), notify::Error> {
        let path = path.as_ref();
        self._watcher.unwatch(path)?;
        self.watched_paths.retain(|p| p != path);
        Ok(())
    }

    /// Get watched paths
    pub fn watched_paths(&self) -> &[PathBuf] {
        &self.watched_paths
    }

    /// Poll for events (non-blocking)
    pub fn poll(&mut self) -> Option<HotReloadEvent> {
        match self.receiver.try_recv() {
            Ok(event) => {
                // Apply per-path debouncing
                let now = Instant::now();
                // Create a unique key based on event type and path
                let event_key = match &event {
                    HotReloadEvent::StylesheetChanged(p) => format!("changed:{}", p.display()),
                    HotReloadEvent::FileCreated(p) => format!("created:{}", p.display()),
                    HotReloadEvent::FileDeleted(p) => format!("deleted:{}", p.display()),
                    HotReloadEvent::Error(_) => format!("error:{}", now.elapsed().as_millis()),
                };

                if let Some(&last) = self.last_events.get(&event_key) {
                    if now.duration_since(last) < self.debounce {
                        return None; // Debounced
                    }
                }
                self.last_events.insert(event_key, now);
                Some(event)
            }
            Err(_) => None,
        }
    }

    /// Wait for next event (blocking)
    pub fn wait(&mut self) -> Option<HotReloadEvent> {
        match self.receiver.recv() {
            Ok(event) => {
                let now = Instant::now();
                // Create a unique key based on event type and path
                let event_key = match &event {
                    HotReloadEvent::StylesheetChanged(p) => format!("changed:{}", p.display()),
                    HotReloadEvent::FileCreated(p) => format!("created:{}", p.display()),
                    HotReloadEvent::FileDeleted(p) => format!("deleted:{}", p.display()),
                    HotReloadEvent::Error(_) => format!("error:{}", now.elapsed().as_millis()),
                };
                self.last_events.insert(event_key, now);
                Some(event)
            }
            Err(_) => None,
        }
    }

    /// Wait for next event with timeout
    pub fn wait_timeout(&mut self, timeout: Duration) -> Option<HotReloadEvent> {
        match self.receiver.recv_timeout(timeout) {
            Ok(event) => {
                let now = Instant::now();
                // Create a unique key based on event type and path
                let event_key = match &event {
                    HotReloadEvent::StylesheetChanged(p) => format!("changed:{}", p.display()),
                    HotReloadEvent::FileCreated(p) => format!("created:{}", p.display()),
                    HotReloadEvent::FileDeleted(p) => format!("deleted:{}", p.display()),
                    HotReloadEvent::Error(_) => format!("error:{}", now.elapsed().as_millis()),
                };
                self.last_events.insert(event_key, now);
                Some(event)
            }
            Err(_) => None,
        }
    }

    /// Check if any CSS files changed
    pub fn css_changed(&mut self) -> Option<PathBuf> {
        while let Some(event) = self.poll() {
            if let HotReloadEvent::StylesheetChanged(path) = event {
                // Case-insensitive extension check (handles .CSS, .Css, etc.)
                if path
                    .extension()
                    .and_then(|e| e.to_str())
                    .map(|e| e.eq_ignore_ascii_case("css"))
                    .unwrap_or(false)
                {
                    return Some(path);
                }
            }
        }
        None
    }
}

/// Builder for hot reload
pub struct HotReloadBuilder {
    config: HotReloadConfig,
    paths: Vec<PathBuf>,
}

impl HotReloadBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            config: HotReloadConfig::default(),
            paths: Vec::new(),
        }
    }

    /// Set debounce duration
    pub fn debounce(mut self, duration: Duration) -> Self {
        self.config.debounce = duration;
        self
    }

    /// Add a path to watch
    pub fn watch(mut self, path: impl AsRef<Path>) -> Self {
        self.paths.push(path.as_ref().to_path_buf());
        self
    }

    /// Build the hot reload watcher
    ///
    /// # Errors
    ///
    /// Returns `notify::Error` if:
    /// - The watcher cannot be created
    /// - Any of the configured paths cannot be watched
    pub fn build(self) -> Result<HotReload, notify::Error> {
        let mut hr = HotReload::with_config(self.config)?;
        for path in self.paths {
            hr.watch(&path)?;
        }
        Ok(hr)
    }
}

impl Default for HotReloadBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to create a hot reload watcher
pub fn hot_reload() -> HotReloadBuilder {
    HotReloadBuilder::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hot_reload_new() {
        let hr = HotReload::new();
        assert!(hr.is_ok());
    }

    #[test]
    fn test_hot_reload_with_config() {
        let config = HotReloadConfig {
            debounce: Duration::from_millis(50),
            recursive: false,
        };
        let hr = HotReload::with_config(config);
        assert!(hr.is_ok());
        let hr = hr.unwrap();
        assert_eq!(hr.debounce, Duration::from_millis(50));
    }

    #[test]
    fn test_hot_reload_config_default() {
        let config = HotReloadConfig::default();
        assert_eq!(config.debounce, Duration::from_millis(100));
        assert!(config.recursive);
    }

    #[test]
    fn test_hot_reload_builder_new() {
        let builder = HotReloadBuilder::new();
        assert!(builder.paths.is_empty());
        assert_eq!(builder.config.debounce, Duration::from_millis(100));
    }

    #[test]
    fn test_hot_reload_builder_default() {
        let builder = HotReloadBuilder::default();
        assert!(builder.paths.is_empty());
        assert_eq!(builder.config.debounce, Duration::from_millis(100));
    }

    #[test]
    fn test_hot_reload_builder_debounce() {
        let builder = HotReloadBuilder::new().debounce(Duration::from_millis(200));
        assert_eq!(builder.config.debounce, Duration::from_millis(200));
    }

    #[test]
    fn test_hot_reload_builder_watch() {
        let builder = HotReloadBuilder::new().watch("src").watch("tests");
        assert_eq!(builder.paths.len(), 2);
    }

    #[test]
    fn test_hot_reload_builder_build() {
        let hr = HotReloadBuilder::new()
            .debounce(Duration::from_millis(75))
            .watch(".")
            .build();
        assert!(hr.is_ok());
        let hr = hr.unwrap();
        assert_eq!(hr.debounce, Duration::from_millis(75));
        assert_eq!(hr.watched_paths().len(), 1);
    }

    #[test]
    fn test_hot_reload_watch_current_dir() {
        let mut hr = HotReload::new().unwrap();
        // Watch current directory (always exists)
        let result = hr.watch(".");
        assert!(result.is_ok());
        assert_eq!(hr.watched_paths().len(), 1);
    }

    #[test]
    fn test_hot_reload_watch_multiple_paths() {
        let mut hr = HotReload::new().unwrap();
        let _ = hr.watch(".");
        let _ = hr.watch("src");
        // Both should be tracked
        assert_eq!(hr.watched_paths().len(), 2);
    }

    #[test]
    fn test_hot_reload_unwatch() {
        let mut hr = HotReload::new().unwrap();
        let _ = hr.watch(".");
        assert_eq!(hr.watched_paths().len(), 1);
        let result = hr.unwatch(".");
        assert!(result.is_ok());
        assert!(hr.watched_paths().is_empty());
    }

    #[test]
    fn test_hot_reload_poll_empty() {
        let mut hr = HotReload::new().unwrap();
        assert!(hr.poll().is_none());
    }

    #[test]
    fn test_hot_reload_wait_timeout_empty() {
        let mut hr = HotReload::new().unwrap();
        let event = hr.wait_timeout(Duration::from_millis(1));
        assert!(event.is_none());
    }

    #[test]
    fn test_hot_reload_css_changed_empty() {
        let mut hr = HotReload::new().unwrap();
        let changed = hr.css_changed();
        assert!(changed.is_none());
    }

    #[test]
    fn test_hot_reload_helper() {
        let builder = hot_reload().debounce(Duration::from_millis(50));
        assert_eq!(builder.config.debounce, Duration::from_millis(50));
    }

    #[test]
    fn test_hot_reload_event_stylesheet_changed() {
        let event = HotReloadEvent::StylesheetChanged(PathBuf::from("test.css"));
        let debug = format!("{:?}", event);
        assert!(debug.contains("StylesheetChanged"));
        assert!(debug.contains("test.css"));
    }

    #[test]
    fn test_hot_reload_event_file_created() {
        let event = HotReloadEvent::FileCreated(PathBuf::from("new.css"));
        let debug = format!("{:?}", event);
        assert!(debug.contains("FileCreated"));
        assert!(debug.contains("new.css"));
    }

    #[test]
    fn test_hot_reload_event_file_deleted() {
        let event = HotReloadEvent::FileDeleted(PathBuf::from("deleted.css"));
        let debug = format!("{:?}", event);
        assert!(debug.contains("FileDeleted"));
        assert!(debug.contains("deleted.css"));
    }

    #[test]
    fn test_hot_reload_event_error() {
        let event = HotReloadEvent::Error("test error".to_string());
        let debug = format!("{:?}", event);
        assert!(debug.contains("Error"));
        assert!(debug.contains("test error"));
    }

    #[test]
    fn test_hot_reload_event_clone() {
        let event = HotReloadEvent::StylesheetChanged(PathBuf::from("style.css"));
        let cloned = event.clone();
        match cloned {
            HotReloadEvent::StylesheetChanged(path) => {
                assert_eq!(path, PathBuf::from("style.css"));
            }
            _ => panic!("Expected StylesheetChanged"),
        }
    }

    #[test]
    fn test_hot_reload_watched_paths_empty() {
        let hr = HotReload::new().unwrap();
        assert!(hr.watched_paths().is_empty());
    }

    #[test]
    fn test_hot_reload_builder_chaining() {
        let builder = HotReloadBuilder::new()
            .debounce(Duration::from_millis(150))
            .watch("src")
            .watch("tests")
            .watch("examples");

        assert_eq!(builder.config.debounce, Duration::from_millis(150));
        assert_eq!(builder.paths.len(), 3);
    }

    // Security tests for path validation
    #[test]
    fn test_hot_reload_watch_null_byte_rejected() {
        let mut hr = HotReload::new().unwrap();
        // Path with null byte should be rejected
        let result = hr.watch("test\x00file");
        assert!(result.is_err());
    }

    #[test]
    fn test_hot_reload_watch_path_traversal_rejected() {
        let mut hr = HotReload::new().unwrap();
        // Attempt to escape current directory should be rejected
        let result = hr.watch("../../../etc/passwd");
        assert!(result.is_err());
    }

    #[test]
    fn test_hot_reload_watch_absolute_path_outside_rejected() {
        let mut hr = HotReload::new().unwrap();
        // Absolute path outside project should be rejected
        let result = hr.watch("/etc/passwd");
        assert!(result.is_err());
    }

    #[test]
    fn test_hot_reload_watch_relative_path_accepted() {
        let mut hr = HotReload::new().unwrap();
        // Valid relative path within project should be accepted
        let result = hr.watch("src");
        // Only check if src exists, otherwise we expect an error from the watcher
        if std::path::Path::new("src").exists() {
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_hot_reload_watch_current_dir_accepted() {
        let mut hr = HotReload::new().unwrap();
        // Current directory should always be accepted
        let result = hr.watch(".");
        assert!(result.is_ok());
    }

    #[test]
    fn test_path_validation_error_display() {
        let err = PathValidationError::NullByte;
        assert!(err.to_string().contains("null byte"));

        let err = PathValidationError::EscapeAttempt {
            path: PathBuf::from("../../../etc/passwd"),
            canonical: PathBuf::from("/etc/passwd"),
        };
        assert!(err.to_string().contains("escapes"));
    }
}
