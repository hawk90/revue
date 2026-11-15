//! An HTTP backend that really sends: reqwest's blocking client

use std::time::Instant;

use super::backend::HttpBackend;
use super::response::HttpResponse;
use super::HttpRequest;

/// [`HttpBackend`] that sends the request over the network with reqwest's
/// blocking client (`http` feature)
///
/// `send` blocks until the whole response has arrived, so call it off the UI
/// thread: take the request with
/// [`HttpClient::take_request`](super::HttpClient::take_request), run it in a
/// task, and pass the result to
/// [`HttpClient::complete`](super::HttpClient::complete) on the UI thread.
/// Do not call it inside an async task either: reqwest's blocking client
/// panics when it blocks inside a tokio runtime.
///
/// # Example
///
/// ```rust,no_run
/// use revue::widget::{HttpBackend, HttpClient, ReqwestBackend};
/// use revue::worker::WorkerPool;
/// use std::sync::mpsc;
///
/// let backend = ReqwestBackend::new();
/// let pool = WorkerPool::new(2);
/// let (tx, rx) = mpsc::channel();
/// let mut client = HttpClient::new().url("https://example.com");
///
/// client.send();
/// if let Some(pending) = client.take_request() {
///     let (backend, tx, id) = (backend.clone(), tx.clone(), pending.id);
///     let queued = pool.submit(move || {
///         let _ = tx.send((pending.id, backend.send(&pending.request)));
///     });
///     if !queued {
///         client.complete(id, Err("too many requests queued".into()));
///     }
/// }
///
/// // On each tick, on the UI thread:
/// while let Ok((id, result)) = rx.try_recv() {
///     client.complete(id, result);
/// }
/// ```
///
/// See `examples/http_client.rs` for a whole app.
#[derive(Clone, Debug, Default)]
pub struct ReqwestBackend {
    client: reqwest::blocking::Client,
}

impl ReqwestBackend {
    /// A backend with reqwest's default client
    pub fn new() -> Self {
        Self::default()
    }

    /// A backend with a client you configured (timeouts, proxy, TLS)
    pub fn with_client(client: reqwest::blocking::Client) -> Self {
        Self { client }
    }
}

impl HttpBackend for ReqwestBackend {
    fn send(&self, request: &HttpRequest) -> Result<HttpResponse, String> {
        let method = reqwest::Method::from_bytes(request.method.name().as_bytes())
            .map_err(|e| e.to_string())?;
        let mut builder = self.client.request(method, request.full_url());
        for (name, value) in &request.headers {
            builder = builder.header(name.as_str(), value.as_str());
        }
        if !request.body.is_empty() {
            builder = builder.body(request.body.clone());
        }

        let start = Instant::now();
        let response = builder.send().map_err(|e| e.to_string())?;
        let status = response.status();

        let mut headers = std::collections::HashMap::<String, String>::new();
        for (name, value) in response.headers() {
            let value = String::from_utf8_lossy(value.as_bytes());
            // A repeated header becomes one comma-separated value (RFC 9110
            // 5.3), since the response keeps one value per name.
            headers
                .entry(name.as_str().to_string())
                .and_modify(|joined| {
                    joined.push_str(", ");
                    joined.push_str(&value);
                })
                .or_insert_with(|| value.into_owned());
        }

        let bytes = response.bytes().map_err(|e| e.to_string())?;
        Ok(HttpResponse {
            status: status.as_u16(),
            status_text: status.canonical_reason().unwrap_or("").to_string(),
            headers,
            body: String::from_utf8_lossy(&bytes).into_owned(),
            time: start.elapsed(),
            size: bytes.len(),
        })
    }
}
