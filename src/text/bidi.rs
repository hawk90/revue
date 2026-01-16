//! RTL (Right-to-Left) and BiDi (Bidirectional) text support
//!
//! Implements Unicode Bidirectional Algorithm (UAX #9) for proper handling
//! of mixed-direction text, including Arabic, Hebrew, and other RTL scripts.

use std::ops::Range;

/// Text direction
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum TextDirection {
    /// Left-to-right (default for Latin, CJK, etc.)
    #[default]
    Ltr,
    /// Right-to-left (for Arabic, Hebrew, etc.)
    Rtl,
    /// Auto-detect from content
    Auto,
}

impl TextDirection {
    /// Returns true if this is RTL direction
    pub fn is_rtl(&self) -> bool {
        matches!(self, TextDirection::Rtl)
    }

    /// Returns true if this is LTR direction
    pub fn is_ltr(&self) -> bool {
        matches!(self, TextDirection::Ltr)
    }

    /// Returns true if direction should be auto-detected
    pub fn is_auto(&self) -> bool {
        matches!(self, TextDirection::Auto)
    }

    /// Resolve auto direction based on text content
    pub fn resolve(&self, text: &str) -> ResolvedDirection {
        match self {
            TextDirection::Ltr => ResolvedDirection::Ltr,
            TextDirection::Rtl => ResolvedDirection::Rtl,
            TextDirection::Auto => detect_direction(text),
        }
    }
}

/// Resolved (non-auto) text direction
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ResolvedDirection {
    /// Left-to-right
    #[default]
    Ltr,
    /// Right-to-left
    Rtl,
}

impl ResolvedDirection {
    /// Returns true if this is RTL
    pub fn is_rtl(&self) -> bool {
        matches!(self, ResolvedDirection::Rtl)
    }

    /// Returns true if this is LTR
    pub fn is_ltr(&self) -> bool {
        matches!(self, ResolvedDirection::Ltr)
    }

    /// Get the opposite direction
    pub fn opposite(&self) -> Self {
        match self {
            ResolvedDirection::Ltr => ResolvedDirection::Rtl,
            ResolvedDirection::Rtl => ResolvedDirection::Ltr,
        }
    }
}

/// BiDi character class (simplified from UAX #9)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BidiClass {
    /// Strong left-to-right (L)
    L,
    /// Strong right-to-left (R) - Hebrew
    R,
    /// Strong right-to-left Arabic (AL)
    AL,
    /// European number (EN)
    EN,
    /// European number separator (ES)
    ES,
    /// European number terminator (ET)
    ET,
    /// Arabic number (AN)
    AN,
    /// Common number separator (CS)
    CS,
    /// Nonspacing mark (NSM)
    NSM,
    /// Boundary neutral (BN)
    BN,
    /// Paragraph separator (B)
    B,
    /// Segment separator (S)
    S,
    /// Whitespace (WS)
    WS,
    /// Other neutrals (ON)
    ON,
    /// Left-to-right embedding (LRE)
    LRE,
    /// Left-to-right override (LRO)
    LRO,
    /// Right-to-left embedding (RLE)
    RLE,
    /// Right-to-left override (RLO)
    RLO,
    /// Pop directional formatting (PDF)
    PDF,
    /// Left-to-right isolate (LRI)
    LRI,
    /// Right-to-left isolate (RLI)
    RLI,
    /// First strong isolate (FSI)
    FSI,
    /// Pop directional isolate (PDI)
    PDI,
}

impl BidiClass {
    /// Get the BiDi class of a character
    pub fn of(c: char) -> Self {
        let code = c as u32;

        // Check for explicit directional formatting characters
        match c {
            '\u{202A}' => return BidiClass::LRE,
            '\u{202B}' => return BidiClass::RLE,
            '\u{202C}' => return BidiClass::PDF,
            '\u{202D}' => return BidiClass::LRO,
            '\u{202E}' => return BidiClass::RLO,
            '\u{2066}' => return BidiClass::LRI,
            '\u{2067}' => return BidiClass::RLI,
            '\u{2068}' => return BidiClass::FSI,
            '\u{2069}' => return BidiClass::PDI,
            _ => {}
        }

        // Arabic block (0600-06FF) and Arabic Supplement (0750-077F)
        if (0x0600..=0x06FF).contains(&code) || (0x0750..=0x077F).contains(&code) {
            // Arabic numbers
            if (0x0660..=0x0669).contains(&code) || (0x06F0..=0x06F9).contains(&code) {
                return BidiClass::AN;
            }
            return BidiClass::AL;
        }

        // Arabic Extended-A (08A0-08FF)
        if (0x08A0..=0x08FF).contains(&code) {
            return BidiClass::AL;
        }

        // Arabic Presentation Forms-A (FB50-FDFF) and B (FE70-FEFF)
        if (0xFB50..=0xFDFF).contains(&code) || (0xFE70..=0xFEFF).contains(&code) {
            return BidiClass::AL;
        }

        // Hebrew block (0590-05FF)
        if (0x0590..=0x05FF).contains(&code) {
            return BidiClass::R;
        }

        // Other RTL scripts
        if (0x07C0..=0x07FF).contains(&code)     // NKo
            || (0x0800..=0x089F).contains(&code) // Samaritan, Mandaic
            || (0x10800..=0x10FFF).contains(&code) // Phoenician, etc.
            || (0x1E800..=0x1EFFF).contains(&code)
        // Mende Kikakui, etc.
        {
            return BidiClass::R;
        }

        // European numbers (ASCII digits)
        if c.is_ascii_digit() {
            return BidiClass::EN;
        }

        // Number separators
        if matches!(c, '+' | '-') {
            return BidiClass::ES;
        }

        // Number terminators
        if matches!(c, '#' | '$' | '%' | '°' | '€' | '£' | '¥') {
            return BidiClass::ET;
        }

        // Common separators
        if matches!(c, ',' | '.' | ':') {
            return BidiClass::CS;
        }

        // Paragraph separator
        if matches!(c, '\n' | '\r' | '\u{0085}' | '\u{2029}') {
            return BidiClass::B;
        }

        // Segment separator
        if matches!(c, '\t' | '\u{001F}' | '\u{001E}' | '\u{000B}') {
            return BidiClass::S;
        }

        // Whitespace
        if c.is_whitespace() {
            return BidiClass::WS;
        }

        // Common punctuation and symbols are neutral
        if c.is_ascii_punctuation() || matches!(c, '(' | ')' | '[' | ']' | '{' | '}' | '"' | '\'') {
            return BidiClass::ON;
        }

        // Default to strong L for Latin and most other scripts
        BidiClass::L
    }

    /// Check if this is a strong type
    pub fn is_strong(&self) -> bool {
        matches!(self, BidiClass::L | BidiClass::R | BidiClass::AL)
    }

    /// Check if this is a strong RTL type
    pub fn is_strong_rtl(&self) -> bool {
        matches!(self, BidiClass::R | BidiClass::AL)
    }

    /// Check if this is a weak type
    pub fn is_weak(&self) -> bool {
        matches!(
            self,
            BidiClass::EN
                | BidiClass::ES
                | BidiClass::ET
                | BidiClass::AN
                | BidiClass::CS
                | BidiClass::NSM
                | BidiClass::BN
        )
    }

    /// Check if this is a neutral type
    pub fn is_neutral(&self) -> bool {
        matches!(
            self,
            BidiClass::B | BidiClass::S | BidiClass::WS | BidiClass::ON
        )
    }
}

/// A segment of text with a single direction
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BidiRun {
    /// The text content of this run
    pub text: String,
    /// Character range in original text
    pub range: Range<usize>,
    /// The embedding level (even = LTR, odd = RTL)
    pub level: u8,
    /// The resolved direction of this run
    pub direction: ResolvedDirection,
}

impl BidiRun {
    /// Create a new BiDi run
    pub fn new(text: String, range: Range<usize>, level: u8) -> Self {
        let direction = if level.is_multiple_of(2) {
            ResolvedDirection::Ltr
        } else {
            ResolvedDirection::Rtl
        };
        Self {
            text,
            range,
            level,
            direction,
        }
    }

    /// Get the length of this run in characters
    pub fn char_count(&self) -> usize {
        self.text.chars().count()
    }
}

/// Result of BiDi analysis
#[derive(Clone, Debug)]
pub struct BidiInfo {
    /// The original text
    pub text: String,
    /// The base/paragraph direction
    pub base_direction: ResolvedDirection,
    /// The BiDi runs (segments with uniform direction)
    pub runs: Vec<BidiRun>,
    /// Visual ordering of runs (indices into runs)
    pub visual_order: Vec<usize>,
}

impl BidiInfo {
    /// Analyze text for BiDi properties
    pub fn new(text: &str, direction: TextDirection) -> Self {
        let base_direction = direction.resolve(text);
        let runs = compute_runs(text, base_direction);
        let visual_order = compute_visual_order(&runs);

        Self {
            text: text.to_string(),
            base_direction,
            runs,
            visual_order,
        }
    }

    /// Get the text reordered for visual display
    pub fn visual_text(&self) -> String {
        let mut result = String::with_capacity(self.text.len());

        for &run_idx in &self.visual_order {
            let run = &self.runs[run_idx];
            if run.direction.is_rtl() {
                // Reverse the characters for RTL runs
                result.extend(run.text.chars().rev());
            } else {
                result.push_str(&run.text);
            }
        }

        result
    }

    /// Convert a logical (storage) position to visual (display) position
    pub fn logical_to_visual(&self, logical_pos: usize) -> usize {
        let mut visual_pos = 0;

        for &run_idx in &self.visual_order {
            let run = &self.runs[run_idx];

            if run.range.contains(&logical_pos) {
                // Found the run containing our position
                let offset_in_run = logical_pos - run.range.start;
                if run.direction.is_rtl() {
                    // In RTL run, position is from the end
                    visual_pos += run.char_count() - 1 - offset_in_run;
                } else {
                    visual_pos += offset_in_run;
                }
                return visual_pos;
            }

            visual_pos += run.char_count();
        }

        visual_pos
    }

    /// Convert a visual (display) position to logical (storage) position
    pub fn visual_to_logical(&self, visual_pos: usize) -> usize {
        let mut pos = 0;

        for &run_idx in &self.visual_order {
            let run = &self.runs[run_idx];
            let run_len = run.char_count();

            if pos + run_len > visual_pos {
                // Found the run containing our position
                let offset_in_run = visual_pos - pos;
                if run.direction.is_rtl() {
                    // In RTL run, position is from the end
                    return run.range.start + (run_len - 1 - offset_in_run);
                } else {
                    return run.range.start + offset_in_run;
                }
            }

            pos += run_len;
        }

        self.text.chars().count()
    }

    /// Get cursor movement direction based on arrow key and current position
    pub fn cursor_move(&self, logical_pos: usize, forward: bool) -> Option<usize> {
        let text_len = self.text.chars().count();
        if text_len == 0 {
            return None;
        }

        // Find which run we're in
        let run_idx = self
            .runs
            .iter()
            .position(|r| r.range.contains(&logical_pos) || r.range.end == logical_pos);

        let current_run = run_idx.map(|i| &self.runs[i]);

        // Determine actual movement direction based on run direction
        let move_right = match current_run {
            Some(run) if run.direction.is_rtl() => !forward,
            _ => forward,
        };

        if move_right {
            if logical_pos < text_len {
                Some(logical_pos + 1)
            } else {
                None
            }
        } else if logical_pos > 0 {
            Some(logical_pos - 1)
        } else {
            None
        }
    }

    /// Check if the text contains any RTL characters
    pub fn has_rtl(&self) -> bool {
        self.runs.iter().any(|r| r.direction.is_rtl())
    }

    /// Check if the text is pure LTR
    pub fn is_pure_ltr(&self) -> bool {
        !self.has_rtl()
    }

    /// Check if the text is pure RTL
    pub fn is_pure_rtl(&self) -> bool {
        self.runs.iter().all(|r| r.direction.is_rtl())
    }
}

/// Detect the base direction of text
pub fn detect_direction(text: &str) -> ResolvedDirection {
    // UAX #9: The first strong character determines paragraph direction
    for c in text.chars() {
        let class = BidiClass::of(c);
        if class == BidiClass::L {
            return ResolvedDirection::Ltr;
        }
        if class.is_strong_rtl() {
            return ResolvedDirection::Rtl;
        }
    }
    // Default to LTR if no strong characters
    ResolvedDirection::Ltr
}

/// Check if a character is RTL
pub fn is_rtl_char(c: char) -> bool {
    BidiClass::of(c).is_strong_rtl()
}

/// Check if text contains RTL characters
pub fn contains_rtl(text: &str) -> bool {
    text.chars().any(is_rtl_char)
}

/// Compute BiDi runs for text
fn compute_runs(text: &str, base_direction: ResolvedDirection) -> Vec<BidiRun> {
    if text.is_empty() {
        return vec![];
    }

    let base_level = if base_direction.is_rtl() { 1 } else { 0 };
    let mut runs = Vec::new();
    let mut current_start = 0;
    let mut current_text = String::new();
    let mut current_level = base_level;

    for (idx, c) in text.char_indices() {
        let class = BidiClass::of(c);
        let char_level = determine_char_level(class, base_level);

        if char_level != current_level && !current_text.is_empty() {
            // End current run
            runs.push(BidiRun::new(
                std::mem::take(&mut current_text),
                current_start..idx,
                current_level,
            ));
            current_start = idx;
            current_level = char_level;
        }

        current_text.push(c);
    }

    // Don't forget the last run
    if !current_text.is_empty() {
        runs.push(BidiRun::new(
            current_text,
            current_start..text.len(),
            current_level,
        ));
    }

    runs
}

/// Determine the embedding level for a character
fn determine_char_level(class: BidiClass, base_level: u8) -> u8 {
    match class {
        BidiClass::L => {
            // L characters are at base level if base is LTR, or base+1 if RTL
            if base_level.is_multiple_of(2) {
                base_level
            } else {
                base_level + 1
            }
        }
        BidiClass::R | BidiClass::AL => {
            // R/AL characters are at base+1 if base is LTR, or base if RTL
            if base_level.is_multiple_of(2) {
                base_level + 1
            } else {
                base_level
            }
        }
        BidiClass::EN | BidiClass::AN => {
            // Numbers follow the base direction typically
            base_level
        }
        _ => {
            // Neutrals inherit from context (simplified: use base)
            base_level
        }
    }
}

/// Compute visual ordering of runs
fn compute_visual_order(runs: &[BidiRun]) -> Vec<usize> {
    if runs.is_empty() {
        return vec![];
    }

    // Find the maximum level
    let max_level = runs.iter().map(|r| r.level).max().unwrap_or(0);

    // Create initial order
    let mut order: Vec<usize> = (0..runs.len()).collect();

    // Reverse runs at each level from max down to 0
    for level in (0..=max_level).rev() {
        let mut i = 0;
        while i < order.len() {
            // Find a sequence of runs at this level or higher
            if runs[order[i]].level >= level {
                let start = i;
                while i < order.len() && runs[order[i]].level >= level {
                    i += 1;
                }
                // Reverse this sequence
                order[start..i].reverse();
            } else {
                i += 1;
            }
        }
    }

    order
}

/// Configuration for BiDi text handling
#[derive(Clone, Debug)]
pub struct BidiConfig {
    /// Default text direction
    pub default_direction: TextDirection,
    /// Whether to support directional override characters
    pub enable_overrides: bool,
    /// Whether to mirror characters in RTL context
    pub enable_mirroring: bool,
}

impl Default for BidiConfig {
    fn default() -> Self {
        Self {
            default_direction: TextDirection::Auto,
            enable_overrides: true,
            enable_mirroring: true,
        }
    }
}

/// Get the mirrored version of a character for RTL display
pub fn mirror_char(c: char) -> char {
    match c {
        '(' => ')',
        ')' => '(',
        '[' => ']',
        ']' => '[',
        '{' => '}',
        '}' => '{',
        '<' => '>',
        '>' => '<',
        '«' => '»',
        '»' => '«',
        '‹' => '›',
        '›' => '‹',
        '⟨' => '⟩',
        '⟩' => '⟨',
        '⟪' => '⟫',
        '⟫' => '⟪',
        '⁅' => '⁆',
        '⁆' => '⁅',
        _ => c,
    }
}

/// Reverse a string while preserving grapheme clusters
pub fn reverse_graphemes(text: &str) -> String {
    // Simple character-based reversal
    // For proper grapheme handling, use unicode-segmentation crate
    text.chars().rev().collect()
}

/// Layout helper for RTL text in a fixed-width area
#[derive(Clone, Debug)]
pub struct RtlLayout {
    /// The available width
    pub width: usize,
    /// Text alignment
    pub align: TextAlign,
    /// Base direction
    pub direction: ResolvedDirection,
}

/// Text alignment
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TextAlign {
    /// Align to start (left for LTR, right for RTL)
    #[default]
    Start,
    /// Align to end (right for LTR, left for RTL)
    End,
    /// Align to left
    Left,
    /// Align to right
    Right,
    /// Center alignment
    Center,
}

impl RtlLayout {
    /// Create a new RTL layout helper
    pub fn new(width: usize, direction: ResolvedDirection) -> Self {
        Self {
            width,
            align: TextAlign::Start,
            direction,
        }
    }

    /// Set text alignment
    pub fn align(mut self, align: TextAlign) -> Self {
        self.align = align;
        self
    }

    /// Calculate the X position for text of given width
    pub fn position(&self, text_width: usize) -> usize {
        if text_width >= self.width {
            return 0;
        }

        let padding = self.width - text_width;

        match self.align {
            TextAlign::Start => {
                if self.direction.is_rtl() {
                    padding
                } else {
                    0
                }
            }
            TextAlign::End => {
                if self.direction.is_rtl() {
                    0
                } else {
                    padding
                }
            }
            TextAlign::Left => 0,
            TextAlign::Right => padding,
            TextAlign::Center => padding / 2,
        }
    }

    /// Pad text to fill width according to alignment
    pub fn pad(&self, text: &str, text_width: usize) -> String {
        if text_width >= self.width {
            return text.to_string();
        }

        let padding = self.width - text_width;
        let pos = self.position(text_width);
        let left_pad = pos;
        let right_pad = padding - left_pad;

        format!("{}{}{}", " ".repeat(left_pad), text, " ".repeat(right_pad))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // =============================================================================
    // TextDirection Tests
    // =============================================================================

    #[test]
    fn test_text_direction_default() {
        assert_eq!(TextDirection::default(), TextDirection::Ltr);
    }

    #[test]
    fn test_text_direction_is_methods() {
        assert!(TextDirection::Ltr.is_ltr());
        assert!(!TextDirection::Ltr.is_rtl());
        assert!(!TextDirection::Ltr.is_auto());

        assert!(!TextDirection::Rtl.is_ltr());
        assert!(TextDirection::Rtl.is_rtl());
        assert!(!TextDirection::Rtl.is_auto());

        assert!(!TextDirection::Auto.is_ltr());
        assert!(!TextDirection::Auto.is_rtl());
        assert!(TextDirection::Auto.is_auto());
    }

    #[test]
    fn test_text_direction_resolve() {
        // LTR always resolves to LTR
        assert_eq!(TextDirection::Ltr.resolve("مرحبا"), ResolvedDirection::Ltr);

        // RTL always resolves to RTL
        assert_eq!(TextDirection::Rtl.resolve("Hello"), ResolvedDirection::Rtl);

        // Auto detects from content
        assert_eq!(TextDirection::Auto.resolve("Hello"), ResolvedDirection::Ltr);
        assert_eq!(TextDirection::Auto.resolve("مرحبا"), ResolvedDirection::Rtl);
        assert_eq!(TextDirection::Auto.resolve("שלום"), ResolvedDirection::Rtl);
    }

    // =============================================================================
    // ResolvedDirection Tests
    // =============================================================================

    #[test]
    fn test_resolved_direction_opposite() {
        assert_eq!(ResolvedDirection::Ltr.opposite(), ResolvedDirection::Rtl);
        assert_eq!(ResolvedDirection::Rtl.opposite(), ResolvedDirection::Ltr);
    }

    // =============================================================================
    // BidiClass Tests
    // =============================================================================

    #[test]
    fn test_bidi_class_latin() {
        assert_eq!(BidiClass::of('A'), BidiClass::L);
        assert_eq!(BidiClass::of('z'), BidiClass::L);
    }

    #[test]
    fn test_bidi_class_arabic() {
        assert_eq!(BidiClass::of('م'), BidiClass::AL);
        assert_eq!(BidiClass::of('ر'), BidiClass::AL);
        assert_eq!(BidiClass::of('ح'), BidiClass::AL);
    }

    #[test]
    fn test_bidi_class_hebrew() {
        assert_eq!(BidiClass::of('ש'), BidiClass::R);
        assert_eq!(BidiClass::of('ל'), BidiClass::R);
        assert_eq!(BidiClass::of('ו'), BidiClass::R);
    }

    #[test]
    fn test_bidi_class_numbers() {
        assert_eq!(BidiClass::of('0'), BidiClass::EN);
        assert_eq!(BidiClass::of('9'), BidiClass::EN);
        // Arabic-Indic digits
        assert_eq!(BidiClass::of('٠'), BidiClass::AN);
        assert_eq!(BidiClass::of('٩'), BidiClass::AN);
    }

    #[test]
    fn test_bidi_class_whitespace() {
        assert_eq!(BidiClass::of(' '), BidiClass::WS);
        assert_eq!(BidiClass::of('\n'), BidiClass::B);
        assert_eq!(BidiClass::of('\t'), BidiClass::S);
    }

    #[test]
    fn test_bidi_class_strong_checks() {
        assert!(BidiClass::L.is_strong());
        assert!(BidiClass::R.is_strong());
        assert!(BidiClass::AL.is_strong());
        assert!(!BidiClass::EN.is_strong());
        assert!(!BidiClass::WS.is_strong());
    }

    #[test]
    fn test_bidi_class_weak_checks() {
        assert!(BidiClass::EN.is_weak());
        assert!(BidiClass::AN.is_weak());
        assert!(!BidiClass::L.is_weak());
        assert!(!BidiClass::WS.is_weak());
    }

    #[test]
    fn test_bidi_class_neutral_checks() {
        assert!(BidiClass::WS.is_neutral());
        assert!(BidiClass::ON.is_neutral());
        assert!(!BidiClass::L.is_neutral());
        assert!(!BidiClass::EN.is_neutral());
    }

    // =============================================================================
    // Direction Detection Tests
    // =============================================================================

    #[test]
    fn test_detect_direction_ltr() {
        assert_eq!(detect_direction("Hello World"), ResolvedDirection::Ltr);
        assert_eq!(detect_direction("123 abc"), ResolvedDirection::Ltr);
    }

    #[test]
    fn test_detect_direction_rtl() {
        assert_eq!(detect_direction("مرحبا"), ResolvedDirection::Rtl);
        assert_eq!(detect_direction("שלום עולם"), ResolvedDirection::Rtl);
    }

    #[test]
    fn test_detect_direction_mixed() {
        // First strong character wins
        assert_eq!(detect_direction("Hello مرحبا"), ResolvedDirection::Ltr);
        assert_eq!(detect_direction("مرحبا Hello"), ResolvedDirection::Rtl);
    }

    #[test]
    fn test_detect_direction_neutral_only() {
        // Numbers and spaces default to LTR
        assert_eq!(detect_direction("123"), ResolvedDirection::Ltr);
        assert_eq!(detect_direction("   "), ResolvedDirection::Ltr);
        assert_eq!(detect_direction(""), ResolvedDirection::Ltr);
    }

    #[test]
    fn test_contains_rtl() {
        assert!(!contains_rtl("Hello World"));
        assert!(contains_rtl("Hello مرحبا"));
        assert!(contains_rtl("שלום"));
        assert!(!contains_rtl("123 456"));
    }

    // =============================================================================
    // BidiInfo Tests
    // =============================================================================

    #[test]
    fn test_bidi_info_pure_ltr() {
        let info = BidiInfo::new("Hello World", TextDirection::Auto);
        assert_eq!(info.base_direction, ResolvedDirection::Ltr);
        assert!(info.is_pure_ltr());
        assert!(!info.has_rtl());
        assert_eq!(info.visual_text(), "Hello World");
    }

    #[test]
    fn test_bidi_info_pure_rtl() {
        let info = BidiInfo::new("שלום", TextDirection::Auto);
        assert_eq!(info.base_direction, ResolvedDirection::Rtl);
        assert!(info.is_pure_rtl());
        assert!(info.has_rtl());
        // RTL text should be reversed for display
        assert_eq!(info.visual_text(), "םולש");
    }

    #[test]
    fn test_bidi_info_mixed() {
        let info = BidiInfo::new("Hello שלום World", TextDirection::Auto);
        assert_eq!(info.base_direction, ResolvedDirection::Ltr);
        assert!(info.has_rtl());
        assert!(!info.is_pure_ltr());
        assert!(!info.is_pure_rtl());
    }

    #[test]
    fn test_bidi_info_empty() {
        let info = BidiInfo::new("", TextDirection::Auto);
        assert!(info.runs.is_empty());
        assert_eq!(info.visual_text(), "");
    }

    #[test]
    fn test_bidi_info_forced_direction() {
        // Force LTR on Arabic text
        let info = BidiInfo::new("مرحبا", TextDirection::Ltr);
        assert_eq!(info.base_direction, ResolvedDirection::Ltr);

        // Force RTL on English text
        let info = BidiInfo::new("Hello", TextDirection::Rtl);
        assert_eq!(info.base_direction, ResolvedDirection::Rtl);
    }

    // =============================================================================
    // Cursor Movement Tests
    // =============================================================================

    #[test]
    fn test_cursor_move_ltr() {
        let info = BidiInfo::new("Hello", TextDirection::Auto);

        // Forward movement
        assert_eq!(info.cursor_move(0, true), Some(1));
        assert_eq!(info.cursor_move(4, true), Some(5));
        assert_eq!(info.cursor_move(5, true), None);

        // Backward movement
        assert_eq!(info.cursor_move(5, false), Some(4));
        assert_eq!(info.cursor_move(1, false), Some(0));
        assert_eq!(info.cursor_move(0, false), None);
    }

    #[test]
    fn test_cursor_move_empty() {
        let info = BidiInfo::new("", TextDirection::Auto);
        assert_eq!(info.cursor_move(0, true), None);
        assert_eq!(info.cursor_move(0, false), None);
    }

    // =============================================================================
    // Position Conversion Tests
    // =============================================================================

    #[test]
    fn test_logical_to_visual_ltr() {
        let info = BidiInfo::new("Hello", TextDirection::Auto);
        assert_eq!(info.logical_to_visual(0), 0);
        assert_eq!(info.logical_to_visual(2), 2);
        assert_eq!(info.logical_to_visual(4), 4);
    }

    #[test]
    fn test_visual_to_logical_ltr() {
        let info = BidiInfo::new("Hello", TextDirection::Auto);
        assert_eq!(info.visual_to_logical(0), 0);
        assert_eq!(info.visual_to_logical(2), 2);
        assert_eq!(info.visual_to_logical(4), 4);
    }

    // =============================================================================
    // BidiRun Tests
    // =============================================================================

    #[test]
    fn test_bidi_run_new() {
        let run = BidiRun::new("Hello".to_string(), 0..5, 0);
        assert_eq!(run.direction, ResolvedDirection::Ltr);
        assert_eq!(run.char_count(), 5);

        let run = BidiRun::new("שלום".to_string(), 0..8, 1);
        assert_eq!(run.direction, ResolvedDirection::Rtl);
        assert_eq!(run.char_count(), 4);
    }

    // =============================================================================
    // Character Mirroring Tests
    // =============================================================================

    #[test]
    fn test_mirror_char() {
        assert_eq!(mirror_char('('), ')');
        assert_eq!(mirror_char(')'), '(');
        assert_eq!(mirror_char('['), ']');
        assert_eq!(mirror_char(']'), '[');
        assert_eq!(mirror_char('{'), '}');
        assert_eq!(mirror_char('}'), '{');
        assert_eq!(mirror_char('<'), '>');
        assert_eq!(mirror_char('>'), '<');
        assert_eq!(mirror_char('«'), '»');
        assert_eq!(mirror_char('»'), '«');
        // Non-mirrorable characters return themselves
        assert_eq!(mirror_char('A'), 'A');
        assert_eq!(mirror_char('م'), 'م');
    }

    #[test]
    fn test_reverse_graphemes() {
        assert_eq!(reverse_graphemes("Hello"), "olleH");
        assert_eq!(reverse_graphemes("שלום"), "םולש");
        assert_eq!(reverse_graphemes(""), "");
    }

    // =============================================================================
    // RtlLayout Tests
    // =============================================================================

    #[test]
    fn test_rtl_layout_ltr_start() {
        let layout = RtlLayout::new(20, ResolvedDirection::Ltr).align(TextAlign::Start);
        assert_eq!(layout.position(10), 0);
    }

    #[test]
    fn test_rtl_layout_ltr_end() {
        let layout = RtlLayout::new(20, ResolvedDirection::Ltr).align(TextAlign::End);
        assert_eq!(layout.position(10), 10);
    }

    #[test]
    fn test_rtl_layout_rtl_start() {
        let layout = RtlLayout::new(20, ResolvedDirection::Rtl).align(TextAlign::Start);
        assert_eq!(layout.position(10), 10);
    }

    #[test]
    fn test_rtl_layout_rtl_end() {
        let layout = RtlLayout::new(20, ResolvedDirection::Rtl).align(TextAlign::End);
        assert_eq!(layout.position(10), 0);
    }

    #[test]
    fn test_rtl_layout_center() {
        let layout = RtlLayout::new(20, ResolvedDirection::Ltr).align(TextAlign::Center);
        assert_eq!(layout.position(10), 5);
    }

    #[test]
    fn test_rtl_layout_left_right() {
        let layout_left = RtlLayout::new(20, ResolvedDirection::Rtl).align(TextAlign::Left);
        assert_eq!(layout_left.position(10), 0);

        let layout_right = RtlLayout::new(20, ResolvedDirection::Rtl).align(TextAlign::Right);
        assert_eq!(layout_right.position(10), 10);
    }

    #[test]
    fn test_rtl_layout_pad() {
        let layout = RtlLayout::new(10, ResolvedDirection::Ltr).align(TextAlign::Center);
        let padded = layout.pad("Hi", 2);
        assert_eq!(padded, "    Hi    ");
        assert_eq!(padded.len(), 10);
    }

    #[test]
    fn test_rtl_layout_pad_overflow() {
        let layout = RtlLayout::new(5, ResolvedDirection::Ltr);
        let padded = layout.pad("Hello World", 11);
        assert_eq!(padded, "Hello World");
    }

    // =============================================================================
    // BidiConfig Tests
    // =============================================================================

    #[test]
    fn test_bidi_config_default() {
        let config = BidiConfig::default();
        assert_eq!(config.default_direction, TextDirection::Auto);
        assert!(config.enable_overrides);
        assert!(config.enable_mirroring);
    }

    // =============================================================================
    // Edge Cases
    // =============================================================================

    #[test]
    fn test_bidi_class_explicit_marks() {
        assert_eq!(BidiClass::of('\u{202A}'), BidiClass::LRE);
        assert_eq!(BidiClass::of('\u{202B}'), BidiClass::RLE);
        assert_eq!(BidiClass::of('\u{202C}'), BidiClass::PDF);
        assert_eq!(BidiClass::of('\u{202D}'), BidiClass::LRO);
        assert_eq!(BidiClass::of('\u{202E}'), BidiClass::RLO);
        assert_eq!(BidiClass::of('\u{2066}'), BidiClass::LRI);
        assert_eq!(BidiClass::of('\u{2067}'), BidiClass::RLI);
        assert_eq!(BidiClass::of('\u{2068}'), BidiClass::FSI);
        assert_eq!(BidiClass::of('\u{2069}'), BidiClass::PDI);
    }

    #[test]
    fn test_text_with_numbers() {
        // Numbers embedded in RTL text
        let info = BidiInfo::new("מחיר: 100", TextDirection::Auto);
        assert_eq!(info.base_direction, ResolvedDirection::Rtl);
        assert!(info.runs.len() >= 1);
    }

    #[test]
    fn test_nko_script_rtl() {
        // NKo script (U+07C0-07FF) is RTL
        let c = '\u{07C1}'; // NKo digit one
        assert_eq!(BidiClass::of(c), BidiClass::R);
    }

    #[test]
    fn test_presentation_forms() {
        // Arabic Presentation Forms
        let c = '\u{FB50}'; // Arabic letter alef wasla isolated form
        assert_eq!(BidiClass::of(c), BidiClass::AL);
    }
}
