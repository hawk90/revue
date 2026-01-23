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
