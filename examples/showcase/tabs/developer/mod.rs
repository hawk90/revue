//! Developer widgets tab - Code, Terminal, HTTP, AI, Diff, Monitor

mod ai;
mod code;
mod diff;
mod http;
mod monitor;
mod terminal;

pub use ai::examples as ai_examples;
pub use code::examples as code_examples;
pub use diff::examples as diff_examples;
pub use http::examples as http_examples;
pub use monitor::examples as monitor_examples;
pub use terminal::examples as terminal_examples;
