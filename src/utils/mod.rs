//! Common utilities for widget rendering
//!
//! This module provides shared functionality used across multiple widgets.
//!
//! # Modules
//!
//! | Module | Description |
//! |--------|-------------|
//! | [`border`] | Border rendering utilities |
//! | [`text`] | Text truncation, padding, wrapping |
//! | [`color`] | Color manipulation (blend, darken, lighten) |
//! | [`sort`] | Natural sorting (file2 < file10) |
//! | [`fuzzy`] | Fuzzy string matching for search |
//! | [`unicode`] | Unicode display width calculation |
//! | [`mod@highlight`] | Search result highlighting |
//! | [`mod@format`] | Human-friendly formatting (duration, size) |
//! | [`diff`] | Text difference comparison |
//! | [`path`] | Path manipulation and display |
//! | [`ansi`] | ANSI escape sequence parsing |
//! | [`easing`] | Animation easing functions |
//! | [`mod@figlet`] | ASCII art text rendering |
//! | [`syntax`] | Syntax highlighting for code |
//! | [`clipboard`] | System clipboard access |
//! | [`validation`] | Form input validation |
//! | [`i18n`] | Internationalization support |
//! | [`keymap`] | Key binding configuration |
//! | [`accessibility`] | Screen reader and a11y support |
//! | [`textbuffer`] | UTF-8 aware text buffer for editing |
//! | [`undo`] | Generic undo/redo history management |
//! | [`gradient`] | Multi-stop color gradients (linear, radial) |
//! | [`animation`] | Frame-based animation (spring, keyframes, sequences) |
//! | [`table`] | Table/column formatting with alignment |
//! | [`tree`] | Tree navigation with collapsible sections |
//! | [`selection`] | List selection with viewport scrolling |
//! | [`layout`] | Box layout calculation for bordered boxes |
//! | [`browser`] | System browser and URL utilities |
//! | [`profiler`] | Performance profiling and timing |

pub mod accessibility;
pub mod accessibility_signal;
pub mod animation;
pub mod ansi;
pub mod border;
pub mod browser;
pub mod clipboard;
pub mod color;
pub mod diff;
pub mod easing;
pub mod figlet;
pub mod format;
pub mod fuzzy;
pub mod gradient;
pub mod highlight;
pub mod i18n;
pub mod keymap;
pub mod layout;
pub mod path;
pub mod profiler;
pub mod selection;
pub mod sort;
pub mod syntax;
pub mod table;
pub mod text;
pub mod textbuffer;
pub mod tree;
pub mod undo;
pub mod unicode;
pub mod validation;

// Border utilities
pub use border::{
    draw_border_title, draw_border_titles, draw_title, draw_title_center, draw_title_right,
    render_border, render_rounded_border, BorderChars, BorderEdge, BorderStyle, BorderTitle,
    TitlePosition,
};

// Text utilities
pub use text::{
    byte_to_char_index,
    center,
    char_count,
    char_slice,
    // Character index utilities (UTF-8 safe)
    char_to_byte_index,
    char_to_byte_index_with_char,
    insert_at_char,
    pad_left,
    pad_right,
    remove_char_at,
    remove_char_range,
    truncate,
    truncate_start,
    wrap_text,
};

// Color utilities
pub use color::{blend, contrast_color, darken, hsl_to_rgb, lighten, rgb_to_hsl};

// Natural sorting
pub use sort::{
    natural_cmp, natural_cmp_case_sensitive, natural_sort, natural_sort_case_sensitive, NaturalKey,
};

// Fuzzy matching
pub use fuzzy::{
    fuzzy_filter, fuzzy_filter_simple, fuzzy_match, fuzzy_matches, fuzzy_score, FuzzyMatch,
    FuzzyMatcher,
};

// Unicode width
pub use unicode::{
    center_to_width, char_width, display_width, pad_to_width, right_align_to_width, split_at_width,
    truncate_to_width, truncate_with_ellipsis, truncate_with_suffix, wrap_to_width,
};

// Highlighting
pub use highlight::{
    highlight_matches, highlight_range, highlight_ranges, highlight_substring,
    highlight_substring_case, HighlightSpan, Highlighter,
};

// Formatting
pub use format::{
    // Duration
    format_duration,
    format_duration_compact,
    format_duration_short,
    // Numbers
    format_number,
    format_number_short,
    format_percent,
    format_percent_precise,
    format_rate,
    format_rate_compact,
    // Relative time
    format_relative_time,
    format_relative_time_short,
    // Size
    format_size,
    format_size_compact,
    format_size_si,
    format_std_duration,
    format_std_duration_short,
    ordinal,
    // Misc
    pluralize,
    pluralize_s,
};

// Diff
pub use diff::{
    diff_chars, diff_lines, diff_words, format_unified_diff, DiffChange, DiffOp, DiffStats,
};

// Path utilities
pub use path::{
    abbreviate_path, abbreviate_path_keep, common_prefix, expand_home, extension, filename,
    home_dir, home_relative, is_hidden, join_paths, normalize_separators, parent, relative_to,
    shorten_path, stem, PathDisplay,
};

// ANSI parsing
pub use ansi::{ansi_len, parse_ansi, strip_ansi, AnsiSpan};

// Easing functions
pub use easing::{
    ease_in_back, ease_in_bounce, ease_in_circ, ease_in_cubic, ease_in_elastic, ease_in_expo,
    ease_in_out_back, ease_in_out_bounce, ease_in_out_circ, ease_in_out_cubic, ease_in_out_elastic,
    ease_in_out_expo, ease_in_out_quad, ease_in_out_quart, ease_in_out_quint, ease_in_out_sine,
    ease_in_quad, ease_in_quart, ease_in_quint, ease_in_sine, ease_out_back, ease_out_bounce,
    ease_out_circ, ease_out_cubic, ease_out_elastic, ease_out_expo, ease_out_quad, ease_out_quart,
    ease_out_quint, ease_out_sine, lerp, lerp_fn, linear, Easing, EasingFn, Interpolator,
};

// Figlet ASCII art
pub use figlet::{figlet, figlet_lines, figlet_with_font, font_height, FigletFont};

// Syntax highlighting
pub use syntax::{
    highlight, highlight_line, Language, SyntaxHighlighter, SyntaxTheme, Token, TokenType,
};

// Clipboard
pub use clipboard::{
    clear as clear_clipboard, copy, has_text as clipboard_has_text, paste, Clipboard,
    ClipboardBackend, ClipboardError, ClipboardHistory, ClipboardResult, MemoryClipboard,
    SystemClipboard,
};

// Validation
pub use validation::{
    all_of, alphabetic, alphanumeric, any_of, custom, email, length_range, lowercase, matches,
    max_length, max_value, min_length, min_value, not_one_of, numeric, one_of, pattern, required,
    uppercase, url, value_range, FormValidator, ValidationError, ValidationResult, Validator,
};

// i18n
pub use i18n::{Direction, I18n, Locale, LocaleId, Translation};

// Keymap utilities
pub use keymap::{
    emacs_preset, format_key_binding, parse_key_binding, vim_preset, KeyChord, KeymapConfig,
    LookupResult, Mode,
};

// Accessibility
pub use accessibility::{
    accessibility_manager, shared_accessibility, AccessibilityManager, AccessibleNode,
    AccessibleState, Announcement, Priority, Role, SharedAccessibility,
};

// Accessibility signal-based API
pub use accessibility_signal::{
    // Core announcement functions
    announce,
    // Widget-specific helpers
    announce_button_clicked,
    announce_checkbox_changed,
    announce_dialog_closed,
    announce_dialog_opened,
    announce_error,
    announce_focus_region,
    announce_list_selection,
    announce_loaded,
    announce_loading,
    announce_now,
    announce_progress,
    announce_progress_complete,
    announce_success,
    announce_tab_changed,
    announce_validation_error,
    has_announcements,
    is_accessibility_enabled,
    is_high_contrast,
    prefers_reduced_motion,
    set_accessibility_enabled,
    set_high_contrast,
    // Preference getters/setters
    set_reduced_motion,
    take_announcements,
};

// Text Buffer
pub use textbuffer::TextBuffer;

// Undo/Redo
pub use undo::{GroupedUndoHistory, Mergeable, UndoGroup, UndoHistory, DEFAULT_MAX_HISTORY};

// Gradient
pub use gradient::{
    presets as gradient_presets, ColorStop, Gradient, GradientDirection, InterpolationMode,
    LinearGradient, RadialGradient, SpreadMode,
};

// Animation
pub use animation::{
    presets as animation_presets, AnimatedValue, Interpolatable, Keyframe, Keyframes, Sequence,
    SequenceStep, Spring, Ticker, Timer,
};

// Table formatting
pub use table::{align_center, align_left, align_right, align_text, Align, Column, Table};

// Tree navigation
pub use tree::{Indent, TreeIcons, TreeItem, TreeNav};

// Selection with viewport
pub use selection::{wrap_next, wrap_prev, SectionedSelection, Selection};

// Box layout
pub use layout::BoxLayout;

// Browser utilities
pub use browser::{open_browser, open_file, open_folder, open_url, reveal_in_finder};

// Profiler
pub use profiler::{
    profile, profiler_report, start_profile, thread_profiler, FlameNode, ProfileGuard, Profiler,
    Stats, Timing,
};
