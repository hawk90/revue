//! Unicode and text handling

mod width;
mod wrap;

pub use width::{CharWidthTable, char_width};
pub use wrap::{
    wrap_text, wrap_words, wrap_chars,
    truncate, truncate_middle,
    TextWrapper, WrapMode, Overflow,
};
