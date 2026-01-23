//! Core types for BiDi text handling

use super::helpers::detect_direction;

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
    pub range: std::ops::Range<usize>,
    /// The embedding level (even = LTR, odd = RTL)
    pub level: u8,
    /// The resolved direction of this run
    pub direction: ResolvedDirection,
}

impl BidiRun {
    /// Create a new BiDi run
    pub fn new(text: String, range: std::ops::Range<usize>, level: u8) -> Self {
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
        let runs = Vec::new(); // Simplified - would compute runs
        let visual_order = Vec::new(); // Simplified

        Self {
            text: text.to_string(),
            base_direction,
            runs,
            visual_order,
        }
    }

    /// Get the text reordered for visual display
    pub fn visual_text(&self) -> String {
        self.text.clone() // Simplified
    }

    /// Check if the text contains any RTL characters
    pub fn has_rtl(&self) -> bool {
        self.base_direction.is_rtl()
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
