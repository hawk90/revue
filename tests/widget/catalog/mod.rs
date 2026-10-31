//! Every public widget, run through one harness: the matrix renders each
//! across sizes, contents and focus states; the sequences drive each with
//! random keys and mouse events. Both check that nothing panics and nothing
//! is written outside the widget's area.

mod caller_sized;
mod emulated;
mod matrix;
mod sequences;
