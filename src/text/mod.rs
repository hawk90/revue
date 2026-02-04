//! Unicode and text handling
//!
//! This module provides utilities for working with text in a terminal context,
//! including bidirectional text support, character width calculation, and text wrapping.
//!
//! # Features
//!
//! | Feature | Description | Use Case |
//!|---------|-------------|----------|
//! | **Bidirectional Text** | RTL/LTR text detection and rendering | Arabic, Hebrew |
//! | **Character Width** | Display width calculation | Layout, alignment |
//! | **Text Wrapping** | Word/character wrapping with overflow | Truncation, wrapping |
//! | **Grapheme Clusters** | Unicode-aware text operations | Emoji, CJK |
//!
//! # Quick Start
//!
//! ## Character Width
//!
//! ```rust,ignore
//! use revue::text::char_width;
//!
//! // ASCII characters are width 1
//! assert_eq!(char_width('A'), 1);
//!
//! // Wide characters (CJK, emoji) are width 2
//! assert_eq!(char_width('中'), 2);
//! assert_eq!(char_width('🎨'), 2);
//!
//! // Combining characters are width 0
//! assert_eq!(char_width('\u{0300}'), 0);
//! ```
//!
//! ## Text Wrapping
//!
//! ```rust,ignore
//! use revue::text::{wrap_text, wrap_words, truncate};
//!
//! // Wrap text to fit width
//! let wrapped = wrap_text("Hello world!", 5);
//! // ["Hello", "world!"]
//!
//! // Wrap by words
//! let wrapped = wrap_words("The quick brown fox", 10);
//! // ["The quick", "brown fox"]
//!
//! // Truncate with ellipsis
//! let truncated = truncate("Long text here", 10);
//! // "Long text ..."
//! ```
//!
//! ## Bidirectional Text
//!
//! ```rust,ignore
//! use revue::text::{detect_direction, contains_rtl, mirror_char};
//!
//! // Detect text direction
//! assert_eq!(detect_direction("Hello"), TextDirection::LTR);
//! assert_eq!(detect_direction("مرحبا"), TextDirection::RTL);
//!
//! // Check for RTL
//! assert!(contains_rtl("Hello مرحبا World"));
//!
//! // Mirror characters for RTL display
//! assert_eq!(mirror_char('('), ')');
//! assert_eq!(mirror_char('<'), '>');
//! ```
//!
//! # Bidi (Bidirectional Text)
//!
//! Supports proper rendering of mixed LTR/RTL text:
//!
//! ```rust,ignore
//! use revue::text::{BidiInfo, BidiConfig};
//!
//! let text = "Hello مرحبا World";
//! let config = BidiConfig::default();
//! let bidi = BidiInfo::new(text, &config);
//!
//! // Get visual runs for rendering
//! for run in bidi.visual_runs() {
//!     println!("{:?}: {}", run.dir, &text[run.start..run.end]);
//! }
//! ```
//!
//! # Text Overflow
//!
//! Handle text that exceeds available space:
//!
//! ```rust,ignore
//! use revue::text::{truncate, truncate_middle, Overflow};
//!
//! // Truncate at end
//! truncate("Very long text", 10, &Overflow::Ellipsis);
//! // "Very long ..."
//!
//! // Truncate in middle
//! truncate_middle("Very long text here", 10);
//! // "Very ... here"
//!
//! // Truncate without indicator
//! truncate("Very long text", 10, &Overflow::Clip);
//! // "Very long "
//! ```
//!
//! # Character Width Tables
//!
//! Uses Unicode standard width assignments:
//!
//! | Category | Width | Examples |
//!|----------|-------|----------|
//! | ASCII | 1 | A-Z, a-z, 0-9, punctuation |
//! | Wide (East Asian) | 2 | CJK, Hangul, fullwidth |
//! | Ambiguous | 1 (or 2 in East Asian context) | Greek, Cyrillic (some) |
//! | Combining | 0 | Accents, diacritics |
//! | Zero-width | 0 | Zero-width joiner, non-joiner |
//!
//! # Text Alignment
//!
//! ```rust,ignore
//! use revue::text::{TextAlign, calculate_alignment};
//!
//! let text = "Hello";
//! let width = 10;
//!
//! // Left align
//! let aligned = calculate_alignment(text, width, TextAlign::Left);
//! // "Hello     "
//!
//! // Center align
//! let aligned = calculate_alignment(text, width, TextAlign::Center);
//! // "  Hello   "
//!
//! // Right align
//! let aligned = calculate_alignment(text, width, TextAlign::Right);
//! // "     Hello"
//! ```

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
