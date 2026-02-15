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

// Tests extracted to tests/a11y_backend_core_tests.rs
// All tests use only public methods from ScreenReaderBackend
