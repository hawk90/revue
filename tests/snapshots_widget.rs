//! Widget snapshot tests - split into modules by widget category
//!
//! Tests that verify widget rendering output matches expected snapshots.
//! Run with: cargo test
//! Update snapshots: REVUE_UPDATE_SNAPSHOTS=1 cargo test

#[path = "snapshots_widget/data.rs"]
mod data;
#[path = "snapshots_widget/form.rs"]
mod form;
#[path = "snapshots_widget/layout.rs"]
mod layout;
#[path = "snapshots_widget/rich_content.rs"]
mod rich_content;
#[path = "snapshots_widget/text.rs"]
mod text;
#[path = "snapshots_widget/time_text.rs"]
mod time_text;
#[path = "snapshots_widget/visual.rs"]
mod visual;
#[path = "snapshots_widget/widgets_a.rs"]
mod widgets_a;
#[path = "snapshots_widget/widgets_b.rs"]
mod widgets_b;
