//! The IME composition state machine

use super::{Candidate, CompositionEvent, CompositionState, ImeConfig};
use std::sync::Arc;

/// Maximum allowed composition text length to prevent memory exhaustion
const MAX_COMPOSITION_LENGTH: usize = 10_000;

/// Maximum allowed number of candidates to prevent memory exhaustion
const MAX_CANDIDATES: usize = 1_000;

// =============================================================================
// IME State
// =============================================================================

type CompositionCallback = Arc<dyn Fn(&CompositionEvent) + Send + Sync>;

/// IME state manager
pub struct ImeState {
    /// Current composition state
    state: CompositionState,
    /// Current composing text
    composing_text: String,
    /// Cursor position within composition
    cursor: usize,
    /// Available candidates
    candidates: Vec<Candidate>,
    /// Selected candidate index
    selected_candidate: usize,
    /// Configuration
    config: ImeConfig,
    /// Composition event callbacks
    callbacks: Vec<CompositionCallback>,
    /// Whether IME is enabled
    enabled: bool,
}

impl ImeState {
    /// Create new IME state
    pub fn new() -> Self {
        Self {
            state: CompositionState::Idle,
            composing_text: String::new(),
            cursor: 0,
            candidates: Vec::new(),
            selected_candidate: 0,
            config: ImeConfig::default(),
            callbacks: Vec::new(),
            enabled: true,
        }
    }

    /// Create with custom configuration
    pub fn with_config(config: ImeConfig) -> Self {
        Self {
            config,
            ..Self::new()
        }
    }

    // -------------------------------------------------------------------------
    // Configuration
    // -------------------------------------------------------------------------

    /// Get configuration
    pub fn config(&self) -> &ImeConfig {
        &self.config
    }

    /// Set configuration
    pub fn set_config(&mut self, config: ImeConfig) {
        self.config = config;
    }

    /// Enable IME
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable IME
    pub fn disable(&mut self) {
        self.enabled = false;
        if self.state != CompositionState::Idle {
            self.cancel();
        }
    }

    /// Check if IME is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    // -------------------------------------------------------------------------
    // State Queries
    // -------------------------------------------------------------------------

    /// Get current composition state
    pub fn state(&self) -> CompositionState {
        self.state
    }

    /// Check if currently composing
    pub fn is_composing(&self) -> bool {
        self.state != CompositionState::Idle
    }

    /// Get composing text
    pub fn composing_text(&self) -> &str {
        &self.composing_text
    }

    /// Get cursor position in composition
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    /// Get candidates
    pub fn candidates(&self) -> &[Candidate] {
        &self.candidates
    }

    /// Get selected candidate index
    pub fn selected_candidate(&self) -> usize {
        self.selected_candidate
    }

    /// Get the currently selected candidate text
    pub fn selected_text(&self) -> Option<&str> {
        self.candidates
            .get(self.selected_candidate)
            .map(|c| c.text.as_str())
    }

    // -------------------------------------------------------------------------
    // Composition Control
    // -------------------------------------------------------------------------

    /// Emit CandidatesChanged event with current candidates
    ///
    /// Helper method to avoid repeated string cloning code.
    /// Called whenever the candidate selection changes.
    fn emit_candidates_changed(&mut self) {
        let candidates: Vec<String> = self.candidates.iter().map(|c| c.text.clone()).collect();
        self.emit(CompositionEvent::CandidatesChanged {
            candidates,
            selected: self.selected_candidate,
        });
    }

    /// Start composition
    pub fn start_composition(&mut self) {
        if !self.enabled {
            return;
        }

        self.state = CompositionState::Composing;
        self.composing_text.clear();
        self.cursor = 0;
        self.candidates.clear();
        self.selected_candidate = 0;

        self.emit(CompositionEvent::Start);
    }

    /// Update composition text
    pub fn update_composition(&mut self, text: &str, cursor: usize) {
        if self.state == CompositionState::Idle {
            self.start_composition();
        }

        // Reject overly long compositions to prevent memory exhaustion
        // Check both byte length and char count for proper Unicode handling
        // Validation must happen BEFORE any string allocation
        let byte_len = text.len();
        let char_count = text.chars().count();

        if byte_len > MAX_COMPOSITION_LENGTH || char_count > MAX_COMPOSITION_LENGTH {
            return;
        }

        // Only allocate after validation passes
        self.composing_text = text.to_string();
        self.cursor = cursor.min(char_count);

        self.emit(CompositionEvent::Update {
            text: self.composing_text.clone(),
            cursor: self.cursor,
        });
    }

    /// Set candidates
    pub fn set_candidates(&mut self, candidates: Vec<Candidate>) {
        // Reject excessive candidates to prevent memory exhaustion
        if candidates.len() > MAX_CANDIDATES {
            return;
        }

        self.candidates = candidates;
        self.selected_candidate = 0;

        if !self.candidates.is_empty() {
            self.state = CompositionState::Selecting;
        }

        self.emit_candidates_changed();
    }

    /// Select next candidate
    pub fn next_candidate(&mut self) {
        if self.candidates.is_empty() {
            return;
        }

        self.selected_candidate = (self.selected_candidate + 1) % self.candidates.len();

        self.emit_candidates_changed();
    }

    /// Select previous candidate
    pub fn prev_candidate(&mut self) {
        if self.candidates.is_empty() {
            return;
        }

        self.selected_candidate = if self.selected_candidate == 0 {
            self.candidates.len() - 1
        } else {
            self.selected_candidate - 1
        };

        self.emit_candidates_changed();
    }

    /// Select candidate by index
    pub fn select_candidate(&mut self, index: usize) {
        if index < self.candidates.len() {
            self.selected_candidate = index;

            self.emit_candidates_changed();
        }
    }

    /// Commit composition with current text or selected candidate
    pub fn commit(&mut self, text: &str) -> Option<String> {
        if self.state == CompositionState::Idle {
            return None;
        }

        let committed = text.to_string();
        self.reset_state();

        self.emit(CompositionEvent::End {
            text: Some(committed.clone()),
        });

        Some(committed)
    }

    /// Commit the currently selected candidate
    pub fn commit_selected(&mut self) -> Option<String> {
        if let Some(candidate) = self.candidates.get(self.selected_candidate) {
            let text = candidate.text.clone();
            self.commit(&text)
        } else if !self.composing_text.is_empty() {
            let text = self.composing_text.clone();
            self.commit(&text)
        } else {
            None
        }
    }

    /// Cancel composition
    pub fn cancel(&mut self) {
        if self.state == CompositionState::Idle {
            return;
        }

        self.reset_state();

        self.emit(CompositionEvent::End { text: None });
    }

    /// Handle backspace during composition
    pub fn backspace(&mut self) -> bool {
        if self.state == CompositionState::Idle || self.composing_text.is_empty() {
            return false;
        }

        // Remove last character
        let mut chars: Vec<char> = self.composing_text.chars().collect();
        if self.cursor > 0 && self.cursor <= chars.len() {
            chars.remove(self.cursor - 1);
            self.composing_text = chars.into_iter().collect();
            self.cursor = self.cursor.saturating_sub(1);

            if self.composing_text.is_empty() {
                self.cancel();
            } else {
                self.emit(CompositionEvent::Update {
                    text: self.composing_text.clone(),
                    cursor: self.cursor,
                });
            }

            true
        } else {
            false
        }
    }

    /// Move cursor left within composition
    pub fn move_cursor_left(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
            self.emit(CompositionEvent::Update {
                text: self.composing_text.clone(),
                cursor: self.cursor,
            });
        }
    }

    /// Move cursor right within composition
    pub fn move_cursor_right(&mut self) {
        let len = self.composing_text.chars().count();
        if self.cursor < len {
            self.cursor += 1;
            self.emit(CompositionEvent::Update {
                text: self.composing_text.clone(),
                cursor: self.cursor,
            });
        }
    }

    // -------------------------------------------------------------------------
    // Callbacks
    // -------------------------------------------------------------------------

    /// Register composition event callback
    pub fn on_composition<F>(&mut self, callback: F)
    where
        F: Fn(&CompositionEvent) + Send + Sync + 'static,
    {
        self.callbacks.push(Arc::new(callback));
    }

    /// Clear all callbacks
    pub fn clear_callbacks(&mut self) {
        self.callbacks.clear();
    }

    // -------------------------------------------------------------------------
    // Internal
    // -------------------------------------------------------------------------

    fn reset_state(&mut self) {
        self.state = CompositionState::Idle;
        self.composing_text.clear();
        self.cursor = 0;
        self.candidates.clear();
        self.selected_candidate = 0;
    }

    fn emit(&self, event: CompositionEvent) {
        for callback in &self.callbacks {
            callback(&event);
        }
    }
}

impl Default for ImeState {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Security: IME Validation Tests
// =============================================================================

#[cfg(test)]
mod validation_tests {
    use super::*;

    #[test]
    fn test_update_composition_rejects_excessive_byte_length() {
        let mut ime = ImeState::new();
        ime.start_composition();

        // Create a string that exceeds MAX_COMPOSITION_LENGTH in bytes
        // but not necessarily in chars (e.g., multi-byte UTF-8)
        let long_text = "あ".repeat(MAX_COMPOSITION_LENGTH + 1); // Each 'あ' is 3 bytes

        ime.update_composition(&long_text, 0);

        // Should not update - text should remain empty
        assert_eq!(ime.composing_text(), "");
        assert!(!ime.is_composing() || ime.composing_text().is_empty());
    }

    #[test]
    fn test_update_composition_rejects_excessive_char_count() {
        let mut ime = ImeState::new();
        ime.start_composition();

        // Create a string with exactly MAX_COMPOSITION_LENGTH + 1 ASCII chars
        // (ASCII is 1 byte per char, so both byte and char counts exceed limit)
        let long_text = "a".repeat(MAX_COMPOSITION_LENGTH + 1);

        ime.update_composition(&long_text, 0);

        // Should not update - text should remain empty
        assert_eq!(ime.composing_text(), "");
    }

    #[test]
    fn test_update_composition_accepts_at_max_length() {
        let mut ime = ImeState::new();
        ime.start_composition();

        // Create a string exactly at the limit
        let text = "a".repeat(MAX_COMPOSITION_LENGTH);

        ime.update_composition(&text, MAX_COMPOSITION_LENGTH);

        // Should update successfully
        assert_eq!(ime.composing_text().len(), MAX_COMPOSITION_LENGTH);
        assert_eq!(ime.cursor(), MAX_COMPOSITION_LENGTH);
    }

    #[test]
    fn test_update_composition_validates_before_allocation() {
        let mut ime = ImeState::new();
        ime.start_composition();

        // This test verifies that validation happens BEFORE allocation
        // by checking that attempting to set an excessively long string
        // doesn't result in that string being stored

        let original_text = "test";
        ime.composing_text = original_text.to_string();

        // Try to update with excessively long text
        let long_text = "x".repeat(MAX_COMPOSITION_LENGTH * 2);
        ime.update_composition(&long_text, 0);

        // Text should remain unchanged (no allocation happened for the long text)
        assert_eq!(ime.composing_text(), original_text);
    }

    #[test]
    fn test_update_composition_unicode_char_count() {
        let mut ime = ImeState::new();
        ime.start_composition();

        // Test with emoji and other multi-byte Unicode characters
        // Each emoji can be multiple chars (grapheme clusters)
        let emoji_text = "😀".repeat(MAX_COMPOSITION_LENGTH + 1);

        ime.update_composition(&emoji_text, 0);

        // Should reject even though byte length might vary
        assert_eq!(ime.composing_text(), "");
    }

    #[test]
    fn test_update_composition_mixed_width_unicode() {
        let mut ime = ImeState::new();
        ime.start_composition();

        // Mix of ASCII (1 byte/char) and CJK (3 bytes/char)
        let mixed_text = "aあ".repeat(MAX_COMPOSITION_LENGTH / 2 + 1);

        ime.update_composition(&mixed_text, 0);

        // Should reject - exceeds limit in both bytes and chars
        assert_eq!(ime.composing_text(), "");
    }

    #[test]
    fn test_update_composition_empty_text_always_accepted() {
        let mut ime = ImeState::new();
        ime.start_composition();

        // Empty text should always be accepted
        ime.update_composition("", 0);

        assert_eq!(ime.composing_text(), "");
        assert_eq!(ime.cursor(), 0);
    }

    #[test]
    fn test_update_composition_short_text_accepted() {
        let mut ime = ImeState::new();
        ime.start_composition();

        // Normal short text should be accepted
        ime.update_composition("こんにちは", 5);

        assert_eq!(ime.composing_text(), "こんにちは");
        assert_eq!(ime.cursor(), 5);
    }

    #[test]
    fn test_update_composition_cursor_clamped_to_char_count() {
        let mut ime = ImeState::new();
        ime.start_composition();

        // Cursor position beyond char count should be clamped
        ime.update_composition("Hello", 100);

        assert_eq!(ime.composing_text(), "Hello");
        assert_eq!(ime.cursor(), 5); // Clamped to actual char count
    }
}
