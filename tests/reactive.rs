//! Reactive integration tests - split into modules

#[path = "reactive/async_state.rs"]
mod async_state;
#[path = "reactive/batch.rs"]
mod batch;
#[path = "reactive/computed.rs"]
mod computed;
#[path = "reactive/context.rs"]
mod context;
#[path = "reactive/effect.rs"]
mod effect;
#[path = "reactive/integration.rs"]
mod integration;
#[path = "reactive/runtime.rs"]
mod runtime;
#[path = "reactive/signal.rs"]
mod signal;
