//! Property-based tests for Revue core components - split into modules

#[path = "property/buffer_ops.rs"]
mod buffer_ops;
#[path = "property/color.rs"]
mod color;
#[path = "property/css_parsing.rs"]
mod css_parsing;
#[path = "property/float_conversion.rs"]
mod float_conversion;
#[path = "property/layout_edge.rs"]
mod layout_edge;
#[path = "property/rect.rs"]
mod rect;
#[path = "property/saturating_arithmetic.rs"]
mod saturating_arithmetic;
#[path = "property/signal.rs"]
mod signal;
#[path = "property/spacing.rs"]
mod spacing;
#[path = "property/splitter.rs"]
mod splitter;
#[path = "property/stack_layout.rs"]
mod stack_layout;
#[path = "property/state_transition.rs"]
mod state_transition;
#[path = "property/string_text.rs"]
mod string_text;
#[path = "property/widget_constraints.rs"]
mod widget_constraints;
