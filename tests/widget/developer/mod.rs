//! Tests for `revue::widget::developer`, laid out like `src/widget/developer/`

mod aistream;
mod code_editor;
#[cfg(feature = "diff")]
mod diff;
mod httpclient;
mod presentation;
#[cfg(feature = "sysinfo")]
mod procmon;
mod terminal;
#[cfg(feature = "syntax-highlighting")]
mod tree_sitter_highlight;
mod vim;
