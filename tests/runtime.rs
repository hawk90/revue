//! Tests for the runtime modules (`revue::{dom, event, layout, render, style}`),
//! laid out like `src/runtime/`

#[path = "support/alloc_cap.rs"]
mod alloc_cap;
#[path = "support/emulator.rs"]
mod emulator;

#[path = "runtime/dom/mod.rs"]
mod dom;
#[path = "runtime/event/mod.rs"]
mod event;
#[path = "runtime/layout/mod.rs"]
mod layout;
#[path = "runtime/render/mod.rs"]
mod render;
#[path = "runtime/style/mod.rs"]
mod style;
