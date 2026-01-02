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

pub mod border;
pub mod tree;
pub mod table;
pub mod text;
pub mod color;
pub mod sort;
pub mod fuzzy;
pub mod unicode;
pub mod highlight;
pub mod format;
pub mod diff;
pub mod path;
pub mod ansi;
pub mod easing;
pub mod figlet;
pub mod syntax;
pub mod clipboard;
pub mod validation;
pub mod i18n;
pub mod keymap;
pub mod accessibility;
pub mod accessibility_signal;
pub mod textbuffer;
pub mod undo;
pub mod gradient;
pub mod animation;
pub mod selection;
pub mod layout;
pub mod browser;
pub mod profiler;

// Border utilities
pub use border::{
    render_border, render_rounded_border, BorderChars, BorderStyle,
    BorderTitle, BorderEdge, TitlePosition,
    draw_border_title, draw_border_titles, draw_title, draw_title_right, draw_title_center,
};

// Text utilities
pub use text::{
    truncate, truncate_start, center, pad_left, pad_right, wrap_text,
    // Character index utilities (UTF-8 safe)
    char_to_byte_index, char_to_byte_index_with_char, byte_to_char_index,
    char_count, char_slice, insert_at_char, remove_char_at, remove_char_range,
};

// Color utilities
pub use color::{blend, darken, lighten, contrast_color, rgb_to_hsl, hsl_to_rgb};

// Natural sorting
pub use sort::{natural_cmp, natural_cmp_case_sensitive, natural_sort, natural_sort_case_sensitive, NaturalKey};

// Fuzzy matching
pub use fuzzy::{fuzzy_match, fuzzy_matches, fuzzy_score, fuzzy_filter, fuzzy_filter_simple, FuzzyMatch, FuzzyMatcher};

// Unicode width
pub use unicode::{
    char_width, display_width,
    truncate_to_width, truncate_with_ellipsis, truncate_with_suffix,
    pad_to_width, center_to_width, right_align_to_width,
    split_at_width, wrap_to_width,
};

// Highlighting
pub use highlight::{
    highlight_matches, highlight_substring, highlight_substring_case,
    highlight_range, highlight_ranges,
    HighlightSpan, Highlighter,
};

// Formatting
pub use format::{
    // Duration
    format_duration, format_duration_short, format_duration_compact,
    format_std_duration, format_std_duration_short,
    // Relative time
    format_relative_time, format_relative_time_short,
    // Size
    format_size, format_size_si, format_size_compact,
    format_rate, format_rate_compact,
    // Numbers
    format_number, format_number_short,
    format_percent, format_percent_precise,
    // Misc
    pluralize, pluralize_s, ordinal,
};

// Diff
pub use diff::{
    diff_lines, diff_chars, diff_words,
    format_unified_diff,
    DiffOp, DiffChange, DiffStats,
};

// Path utilities
pub use path::{
    home_dir, home_relative, expand_home,
    shorten_path, abbreviate_path, abbreviate_path_keep,
    relative_to, extension, stem, filename, parent,
    is_hidden, normalize_separators, join_paths, common_prefix,
    PathDisplay,
};

// ANSI parsing
pub use ansi::{
    parse_ansi, strip_ansi, ansi_len,
    AnsiSpan,
};

// Easing functions
pub use easing::{
    Easing, EasingFn, Interpolator,
    linear,
    ease_in_quad, ease_out_quad, ease_in_out_quad,
    ease_in_cubic, ease_out_cubic, ease_in_out_cubic,
    ease_in_quart, ease_out_quart, ease_in_out_quart,
    ease_in_quint, ease_out_quint, ease_in_out_quint,
    ease_in_sine, ease_out_sine, ease_in_out_sine,
    ease_in_expo, ease_out_expo, ease_in_out_expo,
    ease_in_circ, ease_out_circ, ease_in_out_circ,
    ease_in_back, ease_out_back, ease_in_out_back,
    ease_in_elastic, ease_out_elastic, ease_in_out_elastic,
    ease_in_bounce, ease_out_bounce, ease_in_out_bounce,
    lerp, lerp_fn,
};

// Figlet ASCII art
pub use figlet::{
    figlet, figlet_with_font, figlet_lines, font_height,
    FigletFont,
};

// Syntax highlighting
pub use syntax::{
    highlight, highlight_line,
    SyntaxHighlighter, SyntaxTheme, Language, Token, TokenType,
};

// Clipboard
pub use clipboard::{
    copy, paste, clear as clear_clipboard, has_text as clipboard_has_text,
    Clipboard, ClipboardBackend, ClipboardError, ClipboardResult, ClipboardHistory,
    SystemClipboard, MemoryClipboard,
};

// Validation
pub use validation::{
    ValidationError, ValidationResult, Validator, FormValidator,
    required, min_length, max_length, length_range,
    email, url, pattern, numeric, alphabetic, alphanumeric,
    lowercase, uppercase, min_value, max_value, value_range,
    custom, any_of, all_of, one_of, not_one_of, matches,
};

// i18n
pub use i18n::{
    I18n, Locale, LocaleId, Direction, Translation,
};

// Keymap utilities
pub use keymap::{
    Mode, KeyChord, LookupResult, KeymapConfig,
    parse_key_binding, format_key_binding,
    vim_preset, emacs_preset,
};

// Accessibility
pub use accessibility::{
    Role, AccessibleState, AccessibleNode, Announcement, Priority,
    AccessibilityManager, SharedAccessibility,
    accessibility_manager, shared_accessibility,
};

// Accessibility signal-based API
pub use accessibility_signal::{
    // Core announcement functions
    announce, announce_now, take_announcements, has_announcements,
    // Preference getters/setters
    set_reduced_motion, prefers_reduced_motion,
    set_high_contrast, is_high_contrast,
    set_accessibility_enabled, is_accessibility_enabled,
    // Widget-specific helpers
    announce_button_clicked, announce_checkbox_changed,
    announce_list_selection, announce_tab_changed,
    announce_error, announce_success,
    announce_loading, announce_loaded,
    announce_dialog_opened, announce_dialog_closed,
    announce_validation_error, announce_focus_region,
    announce_progress, announce_progress_complete,
};

// Text Buffer
pub use textbuffer::TextBuffer;

// Undo/Redo
pub use undo::{
    UndoHistory, UndoGroup, GroupedUndoHistory, Mergeable,
    DEFAULT_MAX_HISTORY,
};

// Gradient
pub use gradient::{
    Gradient, ColorStop, InterpolationMode, SpreadMode,
    LinearGradient, RadialGradient, GradientDirection,
    presets as gradient_presets,
};

// Animation
pub use animation::{
    Interpolatable, Timer, Spring, Keyframe, Keyframes,
    AnimatedValue, SequenceStep, Sequence, Ticker,
    presets as animation_presets,
};

// Table formatting
pub use table::{
    Table, Column, Align,
    align_text, align_left, align_right, align_center,
};

// Tree navigation
pub use tree::{
    TreeNav, TreeItem, TreeIcons, Indent,
};

// Selection with viewport
pub use selection::{
    Selection, SectionedSelection, wrap_next, wrap_prev,
};

// Box layout
pub use layout::BoxLayout;

// Browser utilities
pub use browser::{
    open_browser, open_url, open_file, open_folder, reveal_in_finder,
};

// Profiler
pub use profiler::{
    Profiler, Stats, Timing, ProfileGuard, FlameNode,
    profile, start_profile, profiler_report, thread_profiler,
};
