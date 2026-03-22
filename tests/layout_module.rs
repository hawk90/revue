//! Integration tests for the layout module - split into modules

#[path = "layout_module/breakpoints.rs"]
mod breakpoints;
#[path = "layout_module/constraints.rs"]
mod constraints;
#[path = "layout_module/engine.rs"]
mod engine;
#[path = "layout_module/flex_edge_cases.rs"]
mod flex_edge_cases;
#[path = "layout_module/merge.rs"]
mod merge;
#[path = "layout_module/rect.rs"]
mod rect;
#[path = "layout_module/responsive.rs"]
mod responsive;
