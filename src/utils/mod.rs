//! Common utilities for widget rendering
//!
//! This module provides shared functionality used across multiple widgets.
//! Utilities are organized by category for easy discovery.
//!
//! # Categories
//!
//! ## Text Processing
//!
//! | Module | Description | Use Case |
//!|--------|-------------|----------|
//! | [`text`] | Text truncation, padding, wrapping | Text display |
//! | [`textbuffer`] | UTF-8 aware text buffer | Text editing |
//! | [`unicode`] | Unicode display width | Emoji, CJK support |
//!
//! ## Styling & Visual
//!
//! | Module | Description | Use Case |
//!|--------|-------------|----------|
//! | [`color`] | Color manipulation | Blending, darkening |
//! | [`gradient`] | Multi-stop gradients | Linear, radial |
//! | [`border`] | Border rendering | Bordered containers |
//! | [`mod@format`] | Human formatting | Duration, file size |
//! | [`syntax`] | Syntax highlighting | Code display |
//! | [`mod@figlet`] | ASCII art text | Large text |
//! | [`mod@highlight`] | Search highlighting | Fuzzy matches |
//!
//! ## Data Structures
//!
//! | Module | Description | Use Case |
//!|--------|-------------|----------|
//! | [`table`] | Table formatting | Column alignment |
//! | [`tree`] | Tree navigation | Collapsible trees |
//! | [`selection`] | List selection | Viewport scrolling |
//! | [`layout`] | Box layout | Bordered widgets |
//!
//! ## Utilities
//!
//! | Module | Description | Use Case |
//!|--------|-------------|----------|
//! | [`fuzzy`] | Fuzzy string matching | Search/filter |
//! | [`sort`] | Natural sorting | File names (file2 < file10) |
//! | [`diff`] | Text comparison | Diffs, patches |
//! | [`path`] | Path manipulation | File display |
//! | [`ansi`] | ANSI escape parsing | Terminal output |
//! | [`easing`] | Animation easing | Smooth transitions |
//! | [`animation`] | Frame-based animation | Spring, keyframes |
//! | [`debounce`] | Event debouncing | Rate limiting |
//! | [`mod@once`] | One-shot execution | Call once pattern |
//! | [`profiler`] | Performance timing | Benchmarks |
//! | [`lock`] | Lock handling | Poison errors |
//! | [`shell`] | Shell escaping | Command safety |
//!
//! ## System Integration
//!
//! | Module | Description | Use Case |
//!|--------|-------------|----------|
//! | [`clipboard`] | Clipboard access | Copy/paste |
//! | [`i18n`] | Internationalization | Multi-language |
//! | [`accessibility`] | Screen reader support | A11y features |
//! | [`validation`] | Form validation | Input checking |
//! | [`keymap`] | Key bindings | Keyboard shortcuts |
//! | [`browser`] | System browser | Open URLs |
//!
//! # Quick Start
//!
//! ## Color Manipulation
//!
//! ```rust,ignore
//! use revue::utils::color::{darken, lighten, blend};
//! use revue::style::Color;
//!
//! let color = Color::BLUE;
//! let darker = darken(color, 0.2);
//! let lighter = lighten(color, 0.3);
//! let blended = blend(color, Color::WHITE, 0.5);
//! ```
//!
//! ## Fuzzy Search
//!
//! ```rust,ignore
//! use revue::utils::fuzzy::fuzzy_match;
//!
//! let matches = fuzzy_match("hello", "hallo"); // High score
//! let no_match = fuzzy_match("hello", "bye"); // Low score
//! ```
//!
//! ## Natural Sorting
//!
//! ```rust,ignore
//! use revue::utils::sort::natural_cmp;
//!
//! let files = vec!["file10.txt", "file2.txt"];
//! files.sort_by(|a, b| natural_cmp(a, b));
//! // Result: file2.txt, file10.txt
//! ```

pub mod accessibility;
pub mod accessibility_signal;
pub mod animation;
pub mod ansi;
pub mod border;
pub mod browser;
pub mod clipboard;
pub mod color;
pub mod debounce;
pub mod diff;
pub mod easing;
pub mod figlet;
pub mod filter;
pub mod format;
pub mod fuzzy;
pub mod gradient;
pub mod highlight;
pub mod i18n;
pub mod keymap;
pub mod layout;
pub mod lock;
pub mod once;
pub mod overlay;
pub mod path;
pub mod profiler;
pub mod selection;
pub mod shell;
pub mod sort;
pub mod syntax;
pub mod table;
pub mod terminal;
pub mod text;
pub mod text_sizing;
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
    shorten_path, stem, validate_characters, validate_no_traversal, validate_within_base,
    PathDisplay, PathError,
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

// Text Sizing (Kitty OSC 66 protocol)
pub use text_sizing::{is_supported as text_sizing_supported, TextSizing};

// Lock utilities
pub use lock::{lock_or_recover, read_or_recover, write_or_recover};

// Shell escaping
pub use shell::{escape_applescript, escape_powershell, sanitize_string};

// Debounce and Throttle
pub use debounce::{debounce_ms, debouncer, throttle, throttle_ms, Debouncer, Edge, Throttle};

// Once (one-shot execution)
pub use once::{once, Once};

// Filter mode
pub use filter::FilterMode;

// Overlay rendering
pub use overlay::{draw_separator_overlay, draw_text_overlay};

// Terminal detection
pub use terminal::{is_sixel_capable, terminal_type, TerminalType};

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // Module Export Tests
    // =========================================================================
    //
    // This test module verifies that all publicly exported types and functions
    // from the utils module are accessible and compile correctly.
    //
    // These tests use compile-time type checking to verify the API surface
    // without executing functionality, which is tested in individual modules.

    #[test]
    fn test_all_modules_compile() {
        // This test ensures that importing and using the utility API compiles correctly.
        // The actual functionality is tested in each module's own test suite.

        // Filter modes
        let mode = FilterMode::default();
        assert_eq!(mode, FilterMode::Fuzzy);

        // Terminal type
        let term = terminal_type();
        match term {
            TerminalType::Kitty | TerminalType::Iterm2 | TerminalType::Unknown => {}
        }

        // Once utility
        let mut once = Once::new();
        assert!(once.call());
        assert!(!once.call());

        // Text sizing
        let _ = text_sizing_supported();

        // Shell escaping
        let escaped = escape_applescript("test");
        assert_eq!(escaped, "test");
        let escaped = escape_powershell("test");
        assert_eq!(escaped, "test");
        let sanitized = sanitize_string("test");
        assert_eq!(sanitized, "test");

        // Format utilities
        let size = format_size(1024);
        assert!(!size.is_empty());
        let duration = format_duration(60); // Takes u64 seconds
        assert!(!duration.is_empty());

        // Sort utilities
        let order = natural_cmp("file1", "file2");
        let _ = order; // Verify it compiles
        let _ = NaturalKey::new("file1");

        // Unicode utilities
        let width = display_width("hello");
        assert_eq!(width, 5);

        // Highlight utilities
        let spans = highlight_substring("hello world", "hello");
        assert!(!spans.is_empty());

        // ANSI utilities
        let stripped = strip_ansi("test");
        assert_eq!(stripped, "test");

        // Easing functions
        let eased = ease_in_out_quad(0.5);
        assert!((0.0..=1.0).contains(&eased));

        // Color utilities
        let c = crate::style::Color::rgb(100, 100, 100);
        let darker = darken(c, 0.2);
        let lighter = lighten(c, 0.2);

        // Border chars
        let border = BorderChars::ROUNDED;
        let _ = border.top_left;
        let _ = border.top_right;
        let _ = border.bottom_left;
        let _ = border.bottom_right;

        // Figlet
        let art = figlet("Hi");
        assert!(!art.is_empty());

        // I18n
        let mut i18n = I18n::new();
        i18n.add_translation("en", "test", "Test");
        assert_eq!(i18n.t("test"), "Test");

        // Validation
        let validator = required();
        let result = validator("value");
        assert!(result.is_ok());

        // Path utilities
        let name = filename("/path/to/file.txt");
        assert_eq!(name, Some("file.txt".to_string()));

        // Diff utilities
        let diffs = diff_lines("line1\nline2", "line1\nline3");
        assert!(!diffs.is_empty());

        // Fuzzy matching - use pattern that matches in order
        let score = fuzzy_score("hl", "hello");
        assert!(score > 0);

        // Filter mode
        assert!(FilterMode::Fuzzy.matches("hello", "hl"));
        assert!(FilterMode::Prefix.matches("hello", "he"));
        assert!(FilterMode::Contains.matches("hello", "el"));
        assert!(FilterMode::Exact.matches("hello", "hello"));
        assert!(FilterMode::None.matches("hello", "xyz"));

        // Debounce/throttle - just verify types compile
        let _ = debouncer(std::time::Duration::from_millis(100));
        let _ = debounce_ms(100);
        let _ = throttle(std::time::Duration::from_millis(100));
        let _ = throttle_ms(100);

        // Verify lock utilities compile
        let mutex = std::sync::Mutex::new(42);
        let guard = lock_or_recover(&mutex);
        assert_eq!(*guard, 42);

        let rwlock = std::sync::RwLock::new(42);
        let read_guard = read_or_recover(&rwlock);
        assert_eq!(*read_guard, 42);

        // Browser utilities - these return bool but we can't test actual execution
        // without a real environment
        let result = open_browser("https://example.com");
        let _ = result; // Just verify it compiles

        // Profiler
        let _ = profiler_report();

        // Text buffer
        let buffer = TextBuffer::new();
        assert!(buffer.is_empty());

        // Undo history
        let history: UndoHistory<String> = UndoHistory::new();
        assert!(!history.can_undo());
        assert!(!history.can_redo());

        // Selection
        let selection = Selection::new(10);
        assert_eq!(selection.index, 0);

        // Table
        let table = Table::new();
        assert_eq!(table.total_width(), 0);

        // Tree navigation
        let tree = TreeNav::new();
        assert_eq!(tree.selected(), 0);

        // Gradient - verify compilation only
        use crate::utils::gradient::Gradient;
        let _ = LinearGradient::new(
            Gradient::linear(crate::style::Color::RED, crate::style::Color::BLUE),
            GradientDirection::ToRight,
        );

        // Animation - verify compilation only (call one of the preset functions)
        let _ = animation_presets::fade_in(300);

        // Syntax highlighting - verify compilation only
        let _ = Language::from_fence("rust");
        let _ = SyntaxTheme::monokai();

        // Accessibility - verify compilation only
        let _ = shared_accessibility();
        let _ = accessibility_manager();
        announce("Test announcement");
        let _ = take_announcements();
        let _ = has_announcements();
        set_reduced_motion(false);
        let _ = prefers_reduced_motion();
        set_high_contrast(false);
        let _ = is_high_contrast();
        set_accessibility_enabled(true);
        let _ = is_accessibility_enabled();

        // Keymap - verify compilation only
        let _ = emacs_preset();
        let _ = vim_preset();
        let _ = KeymapConfig::default();

        // Box layout
        let layout = BoxLayout::new(0, 0, 20, 10);
        let _ = layout;

        // Check types are Copy/Clone where expected
        let mode1 = FilterMode::Fuzzy;
        let mode2 = mode1;
        assert_eq!(mode1, mode2);

        let term1 = TerminalType::Kitty;
        let term2 = term1;
        assert_eq!(term1, term2);

        // Verify Direction is Copy/Clone
        let dir1 = Direction::Ltr;
        let dir2 = dir1;
        assert_eq!(dir1, dir2);

        // Verify LocaleId is just String
        let id: LocaleId = "en".to_string();
        assert_eq!(id, "en");
    }
}
