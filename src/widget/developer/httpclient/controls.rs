//! HTTP Client widget: view, method, scroll and history controls

use super::types::{HttpMethod, ResponseView};
use super::HttpClient;

impl HttpClient {
    /// Set response view
    pub fn set_view(&mut self, view: ResponseView) {
        self.view = view;
    }

    /// Toggle headers panel
    pub fn toggle_headers(&mut self) {
        self.show_headers = !self.show_headers;
    }

    /// Cycle method
    pub fn cycle_method(&mut self) {
        self.request.method = match self.request.method {
            HttpMethod::GET => HttpMethod::POST,
            HttpMethod::POST => HttpMethod::PUT,
            HttpMethod::PUT => HttpMethod::DELETE,
            HttpMethod::DELETE => HttpMethod::PATCH,
            HttpMethod::PATCH => HttpMethod::HEAD,
            HttpMethod::HEAD => HttpMethod::OPTIONS,
            HttpMethod::OPTIONS => HttpMethod::GET,
        };
    }

    /// Scroll body down
    pub fn scroll_down(&mut self, amount: usize) {
        self.body_scroll = self.body_scroll.saturating_add(amount);
    }

    /// Scroll body up
    pub fn scroll_up(&mut self, amount: usize) {
        self.body_scroll = self.body_scroll.saturating_sub(amount);
    }

    /// Navigate history back
    pub fn history_back(&mut self) {
        if self.history_index > 0 {
            self.history_index -= 1;
            if let Some(req) = self.history.get(self.history_index) {
                self.request = req.clone();
            }
        }
    }

    /// Navigate history forward
    pub fn history_forward(&mut self) {
        if self.history_index < self.history.len() {
            self.history_index += 1;
            if let Some(req) = self.history.get(self.history_index) {
                self.request = req.clone();
            }
        }
    }
}
