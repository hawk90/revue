//! Core types and enums for lazy loading patterns

/// Loading state for lazy data
///
/// Represents the current state of data that is being loaded asynchronously.
/// Used across all lazy loading patterns to track progress.
///
/// # States
///
/// - **Idle** - Data has not been loaded yet
/// - **Loading** - Data is currently being fetched
/// - **Loaded** - Data has been successfully loaded
/// - **Failed** - Loading attempt failed
///
/// # Example
///
/// ```rust,ignore
/// use revue::patterns::lazy::LoadState;
///
/// fn render_status(state: LoadState) -> &'static str {
///     match state {
///         LoadState::Idle => "Press enter to load",
///         LoadState::Loading => "Loading...",
///         LoadState::Loaded => "✓ Loaded",
///         LoadState::Failed => "✗ Failed to load",
///     }
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadState {
    /// Not yet loaded
    Idle,
    /// Currently loading
    Loading,
    /// Successfully loaded
    Loaded,
    /// Loading failed
    Failed,
}

impl LoadState {
    /// Check if data is currently loading
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// if state.is_loading() {
    ///     show_spinner();
    /// }
    /// ```
    pub const fn is_loading(&self) -> bool {
        matches!(self, LoadState::Loading)
    }

    /// Check if data has successfully loaded
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// if state.is_loaded() {
    ///     display_data();
    /// }
    /// ```
    pub const fn is_loaded(&self) -> bool {
        matches!(self, LoadState::Loaded)
    }

    /// Check if data is ready (loaded or idle, not loading/failed)
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// if state.is_ready() {
    ///     allow_user_interaction();
    /// }
    /// ```
    pub const fn is_ready(&self) -> bool {
        matches!(self, LoadState::Idle | LoadState::Loaded)
    }

    /// Check if loading has failed
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// if state.is_failed() {
    ///     show_error_message();
    /// }
    /// ```
    pub const fn is_failed(&self) -> bool {
        matches!(self, LoadState::Failed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_state_is_loading() {
        assert!(LoadState::Loading.is_loading());
        assert!(!LoadState::Idle.is_loading());
        assert!(!LoadState::Loaded.is_loading());
        assert!(!LoadState::Failed.is_loading());
    }

    #[test]
    fn test_load_state_is_loaded() {
        assert!(LoadState::Loaded.is_loaded());
        assert!(!LoadState::Idle.is_loaded());
        assert!(!LoadState::Loading.is_loaded());
        assert!(!LoadState::Failed.is_loaded());
    }

    #[test]
    fn test_load_state_is_ready() {
        assert!(LoadState::Idle.is_ready());
        assert!(LoadState::Loaded.is_ready());
        assert!(!LoadState::Loading.is_ready());
        assert!(!LoadState::Failed.is_ready());
    }

    #[test]
    fn test_load_state_is_failed() {
        assert!(LoadState::Failed.is_failed());
        assert!(!LoadState::Idle.is_failed());
        assert!(!LoadState::Loading.is_failed());
        assert!(!LoadState::Loaded.is_failed());
    }

    #[test]
    fn test_load_state_equality() {
        assert_eq!(LoadState::Idle, LoadState::Idle);
        assert_ne!(LoadState::Idle, LoadState::Loading);
        assert_ne!(LoadState::Loaded, LoadState::Failed);
    }

    #[test]
    fn test_load_state_copy() {
        let state = LoadState::Loading;
        let copied = state;
        assert_eq!(state, copied);
    }

    #[test]
    fn test_load_state_clone() {
        let state = LoadState::Loaded;
        let cloned = state.clone();
        assert_eq!(state, cloned);
    }
}
