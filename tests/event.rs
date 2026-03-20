//! Event integration tests - split into modules

#[path = "event/custom.rs"]
mod custom;
#[path = "event/drag.rs"]
mod drag;
#[path = "event/focus.rs"]
mod focus;
#[path = "event/focus_edge_cases.rs"]
mod focus_edge_cases;
#[path = "event/gesture.rs"]
mod gesture;
#[path = "event/handler.rs"]
mod handler;
#[path = "event/ime.rs"]
mod ime;
#[path = "event/keymap.rs"]
mod keymap;
#[path = "event/mod_tests.rs"]
mod mod_tests;
#[path = "event/reader.rs"]
mod reader;
