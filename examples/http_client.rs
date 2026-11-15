//! HttpClient with real requests
//!
//! The widget only queues what the user sends. The app takes the request,
//! runs it on a worker with `ReqwestBackend`, and hands the result back on
//! the UI thread with `complete`. A request sent again before the first one
//! answers replaces it: the late answer is dropped.
//!
//! Run with: cargo run --example http_client --features http [URL]
//!
//! Keys: Enter send, m method, b/h/r body/headers/raw, j/k scroll,
//! [ and ] history, c clear, q quit.

use revue::prelude::*;
use revue::widget::{
    HttpBackend, HttpClient, HttpResponse, RequestId, ReqwestBackend, ResponseView,
};
use revue::worker::WorkerPool;
use std::sync::mpsc::{self, Receiver, Sender};

type Done = (RequestId, std::result::Result<HttpResponse, String>);

struct Demo {
    client: HttpClient,
    backend: ReqwestBackend,
    /// Two threads, a bounded queue: a burst of sends cannot pile up threads
    pool: WorkerPool,
    done_tx: Sender<Done>,
    done_rx: Receiver<Done>,
}

impl Demo {
    fn new(url: &str) -> Self {
        let (done_tx, done_rx) = mpsc::channel();
        Self {
            client: HttpClient::new().url(url),
            backend: ReqwestBackend::new(),
            pool: WorkerPool::new(2),
            done_tx,
            done_rx,
        }
    }

    /// Run what the widget queued, off the UI thread
    fn run_queued(&mut self) {
        let Some(pending) = self.client.take_request() else {
            return;
        };
        let id = pending.id;
        let backend = self.backend.clone();
        let tx = self.done_tx.clone();
        let queued = self.pool.submit(move || {
            // The receiver is gone only when the app has quit
            let _ = tx.send((pending.id, backend.send(&pending.request)));
        });
        if !queued {
            self.client
                .complete(id, Err("too many requests queued".into()));
        }
    }

    fn handle_key(&mut self, key: &Key) -> bool {
        match key {
            Key::Enter => {
                self.client.send();
                self.run_queued();
            }
            Key::Char('m') => self.client.cycle_method(),
            Key::Char('b') => self.client.set_view(ResponseView::Body),
            Key::Char('h') => self.client.set_view(ResponseView::Headers),
            Key::Char('r') => self.client.set_view(ResponseView::Raw),
            Key::Char('j') | Key::Down => self.client.scroll_down(1),
            Key::Char('k') | Key::Up => self.client.scroll_up(1),
            Key::Char('[') => self.client.history_back(),
            Key::Char(']') => self.client.history_forward(),
            Key::Char('c') => self.client.clear(),
            _ => return false,
        }
        true
    }

    /// Apply finished requests; redraw only when one of them counted
    fn tick(&mut self) -> bool {
        let mut changed = false;
        while let Ok((id, result)) = self.done_rx.try_recv() {
            changed |= self.client.complete(id, result);
        }
        changed
    }
}

impl View for Demo {
    fn render(&self, ctx: &mut RenderContext) {
        self.client.render(ctx);
    }
}

fn main() -> Result<()> {
    let url = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "https://example.com".to_string());
    let mut app = App::builder().build();
    app.run(Demo::new(&url), |event, demo, _app| match event {
        Event::Key(key_event) => demo.handle_key(&key_event.key),
        Event::Tick => demo.tick(),
        _ => false,
    })
}
