//! HTTP Client widget: view, method, scroll and history controls

use super::types::{HttpMethod, ResponseView};
use super::HttpClient;

impl HttpClient {
    /// Set response view
    pub fn set_view(&mut self, view: ResponseView) {
        // Each view is scrolled on its own lines: switching starts at the top
        if view != self.view {
            self.scroll.scroll_to_top();
        }
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
        self.scroll.set_content(self.view_lines());
        self.scroll
            .scroll_by(amount.min(isize::MAX as usize) as isize);
    }

    /// Scroll body up
    pub fn scroll_up(&mut self, amount: usize) {
        self.scroll.set_content(self.view_lines());
        self.scroll
            .scroll_by(-(amount.min(isize::MAX as usize) as isize));
    }

    /// The lines of the shown view: the body's (Body, Raw) or one per header
    pub(super) fn view_lines(&self) -> usize {
        match (&self.response, self.view) {
            (None, _) => 0,
            (Some(response), ResponseView::Headers) => response.headers.len(),
            (Some(response), _) => response.body.lines().count(),
        }
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
