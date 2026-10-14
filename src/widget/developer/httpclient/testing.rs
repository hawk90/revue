//! HTTP Client widget: doc-hidden accessors for tests

use super::types::{HttpColors, ResponseView};
use super::{HttpClient, HttpRequest};

impl HttpClient {
    /// Get colors for testing (doc hidden)
    #[doc(hidden)]
    pub fn colors_for_testing(&self) -> &HttpColors {
        &self.colors
    }

    /// Get view for testing (doc hidden)
    #[doc(hidden)]
    pub fn view_for_testing(&self) -> ResponseView {
        self.view
    }

    /// Get show_headers for testing (doc hidden)
    #[doc(hidden)]
    pub fn show_headers_for_testing(&self) -> bool {
        self.show_headers
    }

    /// Get url_cursor for testing (doc hidden)
    #[doc(hidden)]
    pub fn url_cursor_for_testing(&self) -> usize {
        self.url_cursor
    }

    /// Get body_scroll for testing (doc hidden)
    #[doc(hidden)]
    pub fn body_scroll_for_testing(&self) -> usize {
        self.body_scroll
    }

    /// Set body_scroll for testing (doc hidden)
    #[doc(hidden)]
    pub fn set_body_scroll_for_testing(&mut self, value: usize) {
        self.body_scroll = value;
    }

    /// Get history for testing (doc hidden)
    #[doc(hidden)]
    pub fn history_for_testing(&self) -> &[HttpRequest] {
        &self.history
    }

    /// Get history_index for testing (doc hidden)
    #[doc(hidden)]
    pub fn history_index_for_testing(&self) -> usize {
        self.history_index
    }

    /// Add current request to history for testing (doc hidden)
    #[doc(hidden)]
    pub fn add_to_history_for_testing(&mut self) {
        self.history.push(self.request.clone());
        self.history_index = self.history.len();
    }

    /// Add default request to history for testing (doc hidden)
    #[doc(hidden)]
    pub fn add_default_to_history_for_testing(&mut self) {
        self.history.push(HttpRequest::default());
        self.history_index = 1;
    }
}
