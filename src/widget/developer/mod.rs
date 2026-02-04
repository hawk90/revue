//! Developer widgets - Development and debugging tools
//!
//! This module provides widgets specifically designed for developer tools,
//! debugging, and terminal-based development environments.
//!
//! # Widget Categories
//!
//! ## Editor & Terminal
//!
//! | Widget | Description | Constructor |
//! |--------|-------------|-------------|
//! | [`CodeEditor`] | Code editor with syntax highlighting | [`code_editor()`] |
//! | [`Terminal`] | Embedded terminal emulator | [`terminal()`] |
//!
//! ## HTTP & API
//!
//! | Widget | Description | Constructor |
//! |--------|-------------|-------------|
//! | [`HttpClient`] | HTTP request widget | [`http_client()`] |
//! | `http_get`, `http_post`, etc. | HTTP method shortcuts | See crate::http_get |
//!
//! ## AI & Streaming
//!
//! | Widget | Description | Constructor |
//! |--------|-------------|-------------|
//! | [`AiStream`] | AI streaming text widget | [`ai_stream()`] |
//!
//! ## Presentations
//!
//! | Widget | Description | Constructor |
//! |--------|-------------|-------------|
//! | [`Presentation`] | Slideshow/presentation mode | [`presentation()`] |
//!
//! ## Monitoring
//!
//! | Widget | Description | Constructor |
//! |--------|-------------|-------------|
//! | [`ProcessMonitor`] | System process monitor | [`htop()`], [`process_monitor()`] |
//!
//! ## Text Processing
//!
//! | Widget | Description | Constructor |
//! |--------|-------------|-------------|
//! | [`DiffViewer`] | Diff/patch viewer | [`diff_viewer()`] |
//! | [`TreeSitterHighlighter`] | Syntax highlighting | - |
//!
//! ## Editor Modes
//!
//! | Widget | Description | Constructor |
//! |--------|-------------|-------------|
//! | [`VimState`] | Vim mode for text input | [`vim_state()`] |
//!
//! # Quick Start
//!
//! ## Code Editor
//!
//! ```rust,ignore
//! use revue::prelude::*;
//!
//! code_editor()
//!     .language("rust")
//!     .content("fn main() { println!(\"Hello\"); }")
//!     .line_numbers(true)
//!     .width(60)
//!     .height(20);
//! ```
//!
//! ## HTTP Client
//!
//! ```rust,ignore
//! use revue::prelude::*;
//!
//! http_client()
//!     .url("https://api.example.com/data")
//!     .method(HttpMethod::GET)
//!     .on_response(|response| {
//!         println!("Status: {}", response.status);
//!     });
//! ```
//!
//! ## Process Monitor
//!
//! ```rust,ignore
//! use revue::prelude::*;
//!
//! htop()
//!     .sort_by(ProcessSort::CPU)
//!     .refresh_rate(Duration::from_millis(1000));
//! ```
//!
//! ## Vim Mode
//!
//! ```rust,ignore
//! use revue::prelude::*;
//!
//! vim_state()
//!     .mode(VimMode::Normal)
//!     .handle_key(Key::Char('i'));  // Enter insert mode
//! ```

pub mod aistream;
pub mod code_editor;
#[cfg(feature = "diff")]
pub mod diff;
pub mod httpclient;
pub mod presentation;
#[cfg(feature = "sysinfo")]
pub mod procmon;
pub mod terminal;
#[cfg(feature = "syntax-highlighting")]
pub mod tree_sitter_highlight;
pub mod vim;

// Re-exports for convenience
pub use aistream::{ai_response, ai_stream, AiStream, StreamCursor, StreamStatus, TypingStyle};
pub use code_editor::{
    code_editor, BracketMatch, BracketPair, CodeEditor, EditorConfig, IndentStyle,
};
#[cfg(feature = "diff")]
pub use diff::{diff, diff_viewer, ChangeType, DiffColors, DiffLine, DiffMode, DiffViewer};
pub use httpclient::{
    delete as http_delete, get as http_get, http_client, patch as http_patch, post as http_post,
    put as http_put, ContentType, HttpBackend, HttpClient, HttpMethod, HttpRequest, HttpResponse,
    MockHttpBackend, RequestBuilder, RequestState, ResponseView,
};
pub use presentation::{presentation, slide, Presentation, Slide, SlideAlign, Transition};
#[cfg(feature = "sysinfo")]
pub use procmon::{
    htop, process_monitor, ProcColors, ProcessInfo, ProcessMonitor, ProcessSort, ProcessView,
};
pub use terminal::{terminal, CursorStyle, TermCell, TermLine, Terminal, TerminalAction};
#[cfg(feature = "syntax-highlighting")]
pub use tree_sitter_highlight::TreeSitterHighlighter;
pub use vim::{vim_state, VimAction, VimCommandResult, VimMode, VimMotion, VimState};
