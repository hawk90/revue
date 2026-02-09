//! Screen reader backend - core implementation

use super::platform::{LinuxBackend, LoggingBackend, MacOSBackend, NullBackend, WindowsBackend};
use super::types::{BackendType, ScreenReader, ScreenReaderConfig};
use crate::utils::accessibility::Priority;
use crate::utils::lock::write_or_recover;
use std::sync::RwLock;

/// Main screen reader backend
pub struct ScreenReaderBackend {
    inner: Box<dyn ScreenReader>,
    config: ScreenReaderConfig,
    last_announcement: RwLock<Option<std::time::Instant>>,
}

impl ScreenReaderBackend {
    /// Create a new backend with the given type
    pub fn new(backend_type: BackendType) -> Self {
        let resolved_type = if backend_type == BackendType::Auto {
            BackendType::detect()
        } else {
            backend_type
        };

        let inner: Box<dyn ScreenReader> = match resolved_type {
            BackendType::MacOS => Box::new(MacOSBackend::new()),
            BackendType::Windows => Box::new(WindowsBackend::new()),
            BackendType::Linux => Box::new(LinuxBackend::new()),
            BackendType::Logging => Box::new(LoggingBackend::new()),
            BackendType::None | BackendType::Auto => Box::new(NullBackend),
        };

        Self {
            inner,
            config: ScreenReaderConfig {
                backend_type: resolved_type,
                ..Default::default()
            },
            last_announcement: RwLock::new(None),
        }
    }

    /// Create with custom configuration
    pub fn with_config(config: ScreenReaderConfig) -> Self {
        let mut backend = Self::new(config.backend_type);
        backend.config = config;
        backend
    }

    /// Get the backend type
    pub fn backend_type(&self) -> BackendType {
        self.config.backend_type
    }

    /// Announce a message
    pub fn announce(&self, message: impl AsRef<str>, priority: Priority) {
        let message = message.as_ref();

        // Debounce check
        if self.config.debounce_ms > 0 {
            let now = std::time::Instant::now();
            let mut last = write_or_recover(&self.last_announcement);

            if let Some(last_time) = *last {
                let elapsed = now.duration_since(last_time).as_millis() as u64;
                if elapsed < self.config.debounce_ms && priority == Priority::Polite {
                    return;
                }
            }

            *last = Some(now);
        }

        // Log if enabled
        if self.config.log_announcements {
            let priority_str = match priority {
                Priority::Polite => "polite",
                Priority::Assertive => "assertive",
            };
            eprintln!("[a11y:{}] {}", priority_str, message);
        }

        self.inner.announce(message, priority);
    }

    /// Check if screen reader is available
    pub fn is_available(&self) -> bool {
        self.inner.is_available()
    }

    /// Get active screen reader name
    pub fn active_screen_reader(&self) -> Option<String> {
        self.inner.active_screen_reader()
    }

    /// Announce focus change
    pub fn announce_focus(&self, label: &str, role: &str) {
        if self.config.announce_roles {
            self.inner.announce_focus(label, role);
        } else {
            self.inner.announce(label, Priority::Polite);
        }
    }

    /// Stop current speech
    pub fn stop(&self) {
        self.inner.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_screen_reader_backend_new() {
        let backend = ScreenReaderBackend::new(BackendType::None);
        assert_eq!(backend.backend_type(), BackendType::None);
    }

    #[test]
    fn test_screen_reader_backend_auto() {
        let backend = ScreenReaderBackend::new(BackendType::Auto);
        // Should resolve to some backend type
        let _ = backend.backend_type();
    }

    #[test]
    fn test_screen_reader_backend_new_logging() {
        let backend = ScreenReaderBackend::new(BackendType::Logging);
        assert_eq!(backend.backend_type(), BackendType::Logging);
    }

    #[test]
    fn test_screen_reader_backend_is_available() {
        let backend = ScreenReaderBackend::new(BackendType::None);
        // Just verify the method is callable - actual availability depends on backend
        let _ = backend.is_available();
    }

    #[test]
    fn test_screen_reader_backend_active_screen_reader() {
        let backend = ScreenReaderBackend::new(BackendType::None);
        // NullBackend returns None for active screen reader
        assert!(backend.active_screen_reader().is_none());
    }

    #[test]
    fn test_screen_reader_backend_announce() {
        let backend = ScreenReaderBackend::new(BackendType::None);
        // Should not panic
        backend.announce("Test message", Priority::Polite);
    }

    #[test]
    fn test_screen_reader_backend_announce_assertive() {
        let backend = ScreenReaderBackend::new(BackendType::None);
        // Should not panic
        backend.announce("Important message", Priority::Assertive);
    }

    #[test]
    fn test_screen_reader_backend_announce_focus() {
        let backend = ScreenReaderBackend::new(BackendType::None);
        // Should not panic
        backend.announce_focus("Button", "button");
    }

    #[test]
    fn test_screen_reader_backend_stop() {
        let backend = ScreenReaderBackend::new(BackendType::None);
        // Should not panic
        backend.stop();
    }

    #[test]
    fn test_screen_reader_backend_with_config() {
        use crate::a11y::backend::types::ScreenReaderConfig;

        let config = ScreenReaderConfig {
            backend_type: BackendType::None,
            debounce_ms: 100,
            log_announcements: false,
            announce_roles: false,
        };

        let backend = ScreenReaderBackend::with_config(config);
        assert_eq!(backend.backend_type(), BackendType::None);
    }
}
