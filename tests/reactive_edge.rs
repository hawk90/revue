//! Edge case tests for the reactive system - split into modules

#[path = "reactive_edge/caching.rs"]
mod caching;
#[path = "reactive_edge/concurrent.rs"]
mod concurrent;
#[path = "reactive_edge/conditional.rs"]
mod conditional;
#[path = "reactive_edge/deep_chain.rs"]
mod deep_chain;
#[path = "reactive_edge/diamond.rs"]
mod diamond;
#[path = "reactive_edge/empty_deps.rs"]
mod empty_deps;
#[path = "reactive_edge/memory_leak.rs"]
mod memory_leak;
#[path = "reactive_edge/self_ref.rs"]
mod self_ref;
#[path = "reactive_edge/stress.rs"]
mod stress;
