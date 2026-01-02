//! CSS styling system for Revue.
//!
//! Full CSS support with variables, selectors, transitions, and theming.
//! Revue uses a subset of CSS3 optimized for terminal UIs.
//!
//! # Supported CSS Features
//!
//! | Feature | Support | Example |
//! |---------|---------|---------|
//! | Variables | ✅ Full | `--primary: #bd93f9` |
//! | Selectors | ✅ Full | `.class`, `#id`, `:hover` |
//! | Transitions | ✅ Full | `transition: all 0.3s ease` |
//! | Animations | ✅ Full | `@keyframes fade { ... }` |
//! | Colors | ✅ Full | `#hex`, `rgb()`, named |
//! | Units | ⚡ Partial | `px`, `%` (no `em`/`rem`) |
//!
//! # Quick Start
//!
//! ```rust,ignore
//! use revue::style::{parse_css, Color};
//!
//! let css = r#"
//!     :root {
//!         --primary: #bd93f9;
//!         --bg: #282a36;
//!     }
//!
//!     .button {
//!         background: var(--primary);
//!         color: var(--bg);
//!     }
//!
//!     .button:hover {
//!         background: #ff79c6;
//!     }
//! "#;
//!
//! let stylesheet = parse_css(css)?;
//! ```
//!
//! # CSS Variables
//!
//! Define variables in `:root` and use with `var()`:
//!
//! ```css
//! :root {
//!     --primary: #bd93f9;
//!     --success: #50fa7b;
//!     --error: #ff5555;
//! }
//!
//! .success { color: var(--success); }
//! .error { color: var(--error); }
//! .button { background: var(--primary, #888); } /* with fallback */
//! ```
//!
//! # Selectors
//!
//! Supported selectors:
//!
//! | Selector | Example | Description |
//! |----------|---------|-------------|
//! | Type | `button` | Element type |
//! | Class | `.primary` | CSS class |
//! | ID | `#submit` | Unique ID |
//! | Pseudo | `:hover`, `:focus` | State |
//! | Descendant | `.panel .title` | Nested |
//! | Child | `.panel > .title` | Direct child |
//!
//! # Transitions
//!
//! Animate property changes:
//!
//! ```css
//! .button {
//!     background: #444;
//!     transition: background 0.3s ease;
//! }
//!
//! .button:hover {
//!     background: #666;
//! }
//! ```
//!
//! Available easing functions:
//! - `linear` - Constant speed
//! - `ease` - Smooth acceleration
//! - `ease-in` - Slow start
//! - `ease-out` - Slow end
//! - `ease-in-out` - Slow start and end
//! - `cubic-bezier(x1, y1, x2, y2)` - Custom curve
//!
//! # Built-in Themes
//!
//! Pre-built themes in [`themes`] module:
//!
//! | Theme | Description |
//! |-------|-------------|
//! | [`themes::dracula`] | Dark purple theme |
//! | [`themes::nord`] | Arctic blue theme |
//! | [`themes::monokai`] | Classic dark theme |
//! | [`themes::gruvbox`] | Retro groove theme |
//! | [`themes::catppuccin`] | Pastel dark theme |
//!
//! ```rust,ignore
//! use revue::style::themes::dracula;
//!
//! let bg = dracula::BG_PRIMARY;    // #282a36
//! let accent = dracula::ACCENT;     // #bd93f9
//! ```
//!
//! # Color
//!
//! The [`Color`] type supports:
//!
//! ```rust,ignore
//! use revue::style::Color;
//!
//! // Named colors
//! let red = Color::RED;
//! let cyan = Color::CYAN;
//!
//! // RGB
//! let custom = Color::rgb(189, 147, 249);
//!
//! // Hex (in CSS)
//! // #bd93f9
//! ```

mod parser;
mod properties;
mod computed;
mod theme;
mod animation;
mod transition;
mod theme_signal;
pub mod themes;
pub mod error;

pub use parser::{StyleSheet, Rule, Declaration, apply_declaration};
pub use properties::*;
pub use computed::ComputedStyle;
pub use theme::{
    Theme, ThemeVariant, ThemeBuilder, ThemeColors, Palette, Themes,
    ThemeManager, SharedTheme, ThemeChangeListener,
    theme_manager, shared_theme,
};
pub use theme_signal::{
    use_theme, set_theme, set_theme_by_id, toggle_theme, cycle_theme,
    theme_ids, register_theme, get_theme, theme_to_css_variables,
};
pub use animation::{
    Tween, Animation, Animations, AnimationState, easing,
    // CSS @keyframes animations
    CssKeyframe, KeyframeAnimation, AnimationDirection, AnimationFillMode,
    // Choreography
    Stagger, AnimationGroup, GroupMode, Choreographer,
    // Widget animation presets
    widget_animations,
};
pub use transition::{
    Transition, Transitions, Easing,
    ActiveTransition, TransitionManager,
    lerp_u8, lerp_f32,
    // Reduced motion support
    should_skip_animation, effective_duration,
};
pub use themes::BuiltinTheme;
pub use error::{
    ErrorCode, Severity, SourceLocation, Suggestion,
    RichParseError, ParseErrors, suggest_property, KNOWN_PROPERTIES,
};

/// Parse a CSS file
pub fn parse_css(css: &str) -> Result<StyleSheet, ParseError> {
    parser::parse(css)
}

/// CSS parsing error with rich context
///
/// Provides detailed error messages with source location, suggestions,
/// and error codes for easy debugging.
///
/// # Example
///
/// ```rust,ignore
/// use revue::style::parse_css;
///
/// let css = ".button { colr: red; }";
/// match parse_css(css) {
///     Ok(_) => {},
///     Err(e) => {
///         // Get rich error output
///         println!("{}", e.pretty_print(css));
///     }
/// }
/// ```
#[derive(Debug, Clone)]
pub struct ParseError {
    /// Line number where error occurred (1-indexed)
    pub line: usize,
    /// Column number (1-indexed)
    pub column: usize,
    /// Error message
    pub message: String,
    /// Error code for documentation lookup
    pub code: ErrorCode,
    /// Source offset
    pub offset: usize,
    /// Span length
    pub length: usize,
    /// Suggestions for fixing
    pub suggestions: Vec<String>,
}

impl ParseError {
    /// Create a new parse error
    pub fn new(message: impl Into<String>, line: usize, column: usize) -> Self {
        Self {
            line,
            column,
            message: message.into(),
            code: ErrorCode::InvalidSyntax,
            offset: 0,
            length: 1,
            suggestions: Vec::new(),
        }
    }

    /// Create from a byte offset in source
    pub fn at_offset(message: impl Into<String>, source: &str, offset: usize) -> Self {
        let loc = SourceLocation::from_offset(source, offset);
        Self {
            line: loc.line,
            column: loc.column,
            message: message.into(),
            code: ErrorCode::InvalidSyntax,
            offset,
            length: 1,
            suggestions: Vec::new(),
        }
    }

    /// Set error code
    pub fn with_code(mut self, code: ErrorCode) -> Self {
        self.code = code;
        self
    }

    /// Set span length
    pub fn with_length(mut self, length: usize) -> Self {
        self.length = length;
        self
    }

    /// Add a suggestion
    pub fn suggest(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestions.push(suggestion.into());
        self
    }

    /// Pretty print with source context
    pub fn pretty_print(&self, source: &str) -> String {
        let rich = self.to_rich();
        rich.pretty_print(source)
    }

    /// Convert to RichParseError
    pub fn to_rich(&self) -> RichParseError {
        let mut error = RichParseError::new(
            self.code,
            &self.message,
            SourceLocation::new(self.line, self.column, self.offset, self.length),
        );
        for s in &self.suggestions {
            error.suggestions.push(Suggestion::new(s.clone()));
        }
        error
    }
}

impl std::error::Error for ParseError {}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] CSS error at line {}, column {}: {}",
            self.code, self.line, self.column, self.message
        )
    }
}
