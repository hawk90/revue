//! The stylesheet reaches the screen: every documented paint property,
//! every widget that carries a DOM node, and the builder/inline/stylesheet
//! precedence. `src/widget/traits/render_context/css.rs` is where widgets
//! read their computed style.

mod background;
mod border;
mod builder_outranks_stylesheet;
mod card_and_overflow;
mod defaults;
mod inline_style;
mod ratchet;
mod reaches_the_screen;
mod widgets;
