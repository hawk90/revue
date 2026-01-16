//! Unicode and text handling

pub mod bidi;
mod width;
mod wrap;

pub use bidi::{
    contains_rtl, detect_direction, is_rtl_char, mirror_char, reverse_graphemes, BidiClass,
    BidiConfig, BidiInfo, BidiRun, ResolvedDirection, RtlLayout, TextAlign, TextDirection,
};
pub use width::{char_width, CharWidthTable};
pub use wrap::{
    truncate, truncate_middle, wrap_chars, wrap_text, wrap_words, Overflow, TextWrapper, WrapMode,
};
