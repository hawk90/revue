//! Runtime systems for Revue
//!
//! This module contains the core runtime systems that power Revue applications:
//! - **event**: Event handling and input processing
//! - **render**: Terminal rendering and display
//! - **layout**: Layout engine for flexible UI
//! - **dom**: DOM/CSS system for styled components
//! - **style**: Styling and theming system

pub mod dom;
pub mod event;
pub mod layout;
pub mod render;
pub mod style;
