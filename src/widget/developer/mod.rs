//! Developer widgets - Development and debugging tools
//!
//! Widgets for developers and debugging purposes.

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
