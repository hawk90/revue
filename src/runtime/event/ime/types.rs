//! Composition state, events, candidates and display configuration

// =============================================================================
// Composition State
// =============================================================================

/// IME composition state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CompositionState {
    /// Not composing
    #[default]
    Idle,
    /// Currently composing
    Composing,
    /// Selecting from candidates
    Selecting,
}

/// Composition event
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompositionEvent {
    /// Composition started
    Start,
    /// Composition text updated
    Update {
        /// Current composing text
        text: String,
        /// Cursor position within composition
        cursor: usize,
    },
    /// Composition ended (committed or cancelled)
    End {
        /// Committed text (None if cancelled)
        text: Option<String>,
    },
    /// Candidate list updated
    CandidatesChanged {
        /// List of candidates
        candidates: Vec<String>,
        /// Currently selected index
        selected: usize,
    },
}

// =============================================================================
// Candidate
// =============================================================================

/// A candidate for selection during composition
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Candidate {
    /// The candidate text
    pub text: String,
    /// Optional label (e.g., "1", "a")
    pub label: Option<String>,
    /// Optional annotation/description
    pub annotation: Option<String>,
}

impl Candidate {
    /// Create a new candidate
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            label: None,
            annotation: None,
        }
    }

    /// Set label
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set annotation
    pub fn with_annotation(mut self, annotation: impl Into<String>) -> Self {
        self.annotation = Some(annotation.into());
        self
    }
}

impl From<&str> for Candidate {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for Candidate {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

// =============================================================================
// Composition Style
// =============================================================================

/// Visual style for composition text
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CompositionStyle {
    /// Underline the composing text
    #[default]
    Underline,
    /// Highlight with background color
    Highlight,
    /// Use a different foreground color
    Colored,
    /// Combine underline and highlight
    UnderlineHighlight,
}

/// Configuration for IME display
#[derive(Debug, Clone)]
pub struct ImeConfig {
    /// Style for composing text
    pub composition_style: CompositionStyle,
    /// Show candidate window
    pub show_candidates: bool,
    /// Maximum candidates to display
    pub max_candidates: usize,
    /// Candidate window position relative to cursor
    pub candidate_offset: (i16, i16),
    /// Show composition inline (vs separate window)
    pub inline_composition: bool,
}

impl Default for ImeConfig {
    fn default() -> Self {
        Self {
            composition_style: CompositionStyle::Underline,
            show_candidates: true,
            max_candidates: 9,
            candidate_offset: (0, 1),
            inline_composition: true,
        }
    }
}
