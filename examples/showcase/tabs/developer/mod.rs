//! Developer widgets tab - Code, Terminal, HTTP, AI, Diff, Monitor

mod ai;
mod code;
mod diff;
mod http;
mod monitor;
mod terminal;

pub use ai::render as render_ai;
pub use code::render as render_code;
pub use diff::render as render_diff;
pub use http::render as render_http;
pub use monitor::render as render_monitor;
pub use terminal::render as render_terminal;
