//! Screen reader backend - global backend management

use std::sync::OnceLock;

use super::core::ScreenReaderBackend;
use super::types::BackendType;
use crate::utils::accessibility::Priority;

/// Global screen reader backend
static BACKEND: OnceLock<ScreenReaderBackend> = OnceLock::new();

/// Initialize the global backend
pub fn init_backend(backend_type: BackendType) -> &'static ScreenReaderBackend {
    BACKEND.get_or_init(|| ScreenReaderBackend::new(backend_type))
}

/// Get the global backend (initializes with auto-detect if needed)
pub fn get_backend() -> &'static ScreenReaderBackend {
    BACKEND.get_or_init(|| ScreenReaderBackend::new(BackendType::Auto))
}

/// Set a custom backend (must be called before any other backend functions)
pub fn set_backend(backend: ScreenReaderBackend) -> bool {
    BACKEND.set(backend).is_ok()
}

/// Announce to the global screen reader
pub fn announce_to_screen_reader(message: impl Into<String>, priority: Priority) {
    get_backend().announce(message.into(), priority);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_backend_auto_initializes() {
        // Should not panic and should return a backend
        let backend = get_backend();
        // We can't test much without exposing internals, but we can verify it doesn't crash
        let _ = backend;
    }

    #[test]
    fn test_init_backend_with_auto() {
        // Initialize with Auto type
        let backend = init_backend(BackendType::Auto);
        let _ = backend;
    }

    #[test]
    fn test_init_backend_with_logging() {
        // Initialize with Logging type for testing
        let backend = init_backend(BackendType::Logging);
        let _ = backend;
    }

    #[test]
    fn test_init_backend_with_none() {
        // Initialize with None type (silent)
        let backend = init_backend(BackendType::None);
        let _ = backend;
    }

    #[test]
    fn test_announce_to_screen_reader_wrapper() {
        // Should not panic
        announce_to_screen_reader("Test message", Priority::Polite);
    }

    #[test]
    fn test_announce_with_assertive_priority() {
        // Should not panic with assertive priority
        announce_to_screen_reader("Important message", Priority::Assertive);
    }

    #[test]
    fn test_announce_with_polite_priority() {
        // Should not panic with polite priority
        announce_to_screen_reader("Polite message", Priority::Polite);
    }

    #[test]
    fn test_multiple_announces() {
        // Should handle multiple announces
        announce_to_screen_reader("Message 1", Priority::Polite);
        announce_to_screen_reader("Message 2", Priority::Polite);
        announce_to_screen_reader("Message 3", Priority::Polite);
    }

    #[test]
    fn test_get_backend_returns_same_instance() {
        // get_backend should return the same instance each time
        let backend1 = get_backend();
        let backend2 = get_backend();
        // They should be the same reference (same address)
        assert_eq!(std::ptr::from_ref(backend1), std::ptr::from_ref(backend2));
    }

    #[test]
    fn test_init_backend_returns_same_instance() {
        // init_backend should return the same instance each time
        let backend1 = init_backend(BackendType::None);
        let backend2 = get_backend(); // get_backend should return the same instance
        assert_eq!(std::ptr::from_ref(backend1), std::ptr::from_ref(backend2));
    }

    // Note: set_backend tests are difficult because OnceLock can only be set once
    // and the state persists across tests, so we skip them
}
