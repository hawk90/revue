//! HTTP Client widget: the request/response exchange and its state

use super::response::HttpResponse;
use super::types::{PendingRequest, RequestId, RequestState};
use super::{HttpClient, HttpRequest};

/// The error a refused URL sets: it names the rules `HttpRequest::new` checks
const URL_REFUSED: &str = "Invalid URL: only http:// and https:// schemes are allowed, and URL must not exceed 8192 characters";

impl HttpClient {
    /// Update URL with validation
    ///
    /// Invalid URLs will set an error state instead of panicking.
    pub fn set_url(&mut self, url: impl Into<String>) {
        let url_str = url.into();
        if let Some(valid_req) = HttpRequest::new(&url_str) {
            self.accept_url(valid_req.url);
        } else {
            self.reject_url();
        }
    }

    /// Send the current request
    ///
    /// The widget does not run the request itself. `send` records it in the
    /// history, switches to [`RequestState::Sending`] and queues it; the app
    /// takes it with [`take_request`](Self::take_request), runs it off the UI
    /// thread (for example with an [`HttpBackend`](super::HttpBackend) in a
    /// task) and hands the result back with [`complete`](Self::complete).
    ///
    /// Only the latest request counts: a second `send` replaces a request
    /// the app has not taken yet, and the result of an earlier one is
    /// dropped when it arrives.
    ///
    /// A URL that [`set_url`](Self::set_url) refused, or an empty one, is not
    /// sent: the widget stays in the error state instead of sending the
    /// previous URL.
    pub fn send(&mut self) {
        if self.url_rejected || HttpRequest::new(self.request.url()).is_none() {
            // This send is the latest one: an earlier request's result must
            // not replace the refusal when it arrives.
            self.pending = None;
            self.in_flight = None;
            self.reject_url();
            return;
        }

        self.error = None;
        self.state = RequestState::Sending;

        // Save to history
        self.history.push(self.request.clone());
        self.history_index = self.history.len();

        self.next_request_id += 1;
        let id = RequestId(self.next_request_id);
        self.in_flight = Some(id);
        self.pending = Some(PendingRequest {
            id,
            request: self.request.clone(),
        });
    }

    /// Take the request queued by [`send`](Self::send), if any
    ///
    /// Each request is handed out once. Run it and pass its result to
    /// [`complete`](Self::complete) with the same id.
    pub fn take_request(&mut self) -> Option<PendingRequest> {
        self.pending.take()
    }

    /// Apply the result of a request taken with
    /// [`take_request`](Self::take_request)
    ///
    /// Applies it only when `id` is the latest request sent and not yet
    /// completed or cleared, and returns whether it did: a stale result
    /// changes nothing, so the view needs no redraw. A response is applied
    /// as [`set_response`](Self::set_response) does (a non-2xx status is an
    /// error state that keeps the response); an `Err` is shown as the error.
    pub fn complete(&mut self, id: RequestId, result: Result<HttpResponse, String>) -> bool {
        if self.in_flight != Some(id) {
            return false;
        }
        self.in_flight = None;
        match result {
            Ok(response) => self.set_response(response),
            Err(error) => self.set_error(error),
        }
        true
    }

    /// Set mock response (for testing)
    pub fn set_response(&mut self, response: HttpResponse) {
        let is_success = response.is_success();
        self.response = Some(response);
        // The answer replaces an earlier failure: it is what is shown (#1083)
        self.error = None;
        // A new response is read from its first line
        self.scroll.scroll_to_top();
        self.state = if is_success {
            RequestState::Success
        } else {
            RequestState::Error
        };
    }

    /// Set error
    pub fn set_error(&mut self, error: impl Into<String>) {
        self.error = Some(error.into());
        self.state = RequestState::Error;
    }

    /// Clear response
    ///
    /// Also forgets a request that is queued or running: its result is
    /// dropped when it arrives.
    pub fn clear(&mut self) {
        self.pending = None;
        self.in_flight = None;
        self.response = None;
        self.error = None;
        self.state = RequestState::Idle;
        self.scroll.set_content(0);
    }

    /// Take a URL `HttpRequest::new` accepted. A refusal still shown is
    /// dropped, and the state is again the one the exchange is in: sending
    /// while a request is in flight, else the response's, else idle (#1083).
    /// Any other error (a failed request, `set_error`) stays.
    pub(super) fn accept_url(&mut self, url: String) {
        self.request.url = url;
        self.url_cursor = self.request.url.len();
        self.url_rejected = false;
        if self.error.as_deref() == Some(URL_REFUSED) {
            self.error = None;
            self.state = if self.in_flight.is_some() {
                RequestState::Sending
            } else {
                match &self.response {
                    Some(response) if response.is_success() => RequestState::Success,
                    Some(_) => RequestState::Error,
                    None => RequestState::Idle,
                }
            };
        }
    }

    /// Record that a URL was refused: the message names the rules
    /// `HttpRequest::new` checks.
    pub(super) fn reject_url(&mut self) {
        self.url_rejected = true;
        self.error = Some(URL_REFUSED.to_string());
        self.state = RequestState::Error;
    }
}
