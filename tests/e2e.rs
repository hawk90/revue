//! Whole-app tests: they drive `App` or the `PipelineHarness` end to end, so
//! no single module owns them.

#[path = "support/alloc_cap.rs"]
mod alloc_cap;

#[path = "e2e/config_matrix.rs"]
mod config_matrix;
#[path = "e2e/incremental_differential.rs"]
mod incremental_differential;
#[path = "e2e/invariants.rs"]
mod invariants;
#[path = "e2e/scenarios.rs"]
mod scenarios;
#[path = "e2e/screens.rs"]
mod screens;
#[path = "e2e/widget_interaction.rs"]
mod widget_interaction;
