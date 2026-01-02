//! Unicode and text handling

mod width;
mod wrap;

pub use width::{char_width, CharWidthTable};
pub use wrap::{
    truncate, truncate_middle, wrap_chars, wrap_text, wrap_words, Overflow, TextWrapper, WrapMode,
};
