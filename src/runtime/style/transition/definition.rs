//! Transition definitions parsed from CSS: easing, per-property transitions

use std::time::Duration;

/// Easing function for transitions
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Easing {
    /// Linear interpolation
    #[default]
    Linear,
    /// Ease in (slow start)
    EaseIn,
    /// Ease out (slow end)
    EaseOut,
    /// Ease in and out (slow start and end)
    EaseInOut,
    /// Custom cubic bezier
    CubicBezier(f32, f32, f32, f32),
}

impl Easing {
    /// Apply easing function to a progress value (0.0 to 1.0)
    pub fn apply(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);

        match self {
            Easing::Linear => t,
            Easing::EaseIn => t * t,
            Easing::EaseOut => t * (2.0 - t),
            Easing::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    -1.0 + (4.0 - 2.0 * t) * t
                }
            }
            Easing::CubicBezier(x1, y1, x2, y2) => cubic_bezier(*x1, *y1, *x2, *y2, t),
        }
    }

    /// Parse easing from string
    pub fn parse(s: &str) -> Option<Self> {
        match s.trim().to_lowercase().as_str() {
            "linear" => Some(Easing::Linear),
            "ease" => Some(Easing::EaseInOut),
            "ease-in" => Some(Easing::EaseIn),
            "ease-out" => Some(Easing::EaseOut),
            "ease-in-out" => Some(Easing::EaseInOut),
            s if s.starts_with("cubic-bezier(") => {
                let inner = s.strip_prefix("cubic-bezier(")?.strip_suffix(')')?;
                let parts: Vec<f32> = inner
                    .split(',')
                    .filter_map(|p| p.trim().parse().ok())
                    .collect();
                // As in CSS, x has to stay in 0..=1 so the curve is a function
                // of time; y may leave it, which is how a curve overshoots.
                if parts.len() == 4
                    && (0.0..=1.0).contains(&parts[0])
                    && (0.0..=1.0).contains(&parts[2])
                    && parts[1].is_finite()
                    && parts[3].is_finite()
                {
                    Some(Easing::CubicBezier(parts[0], parts[1], parts[2], parts[3]))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

/// The CSS `cubic-bezier` curve at time `t`
///
/// The input is the curve's x, not its parameter, so the parameter is found
/// first. x only grows with the parameter while x1 and x2 are in 0..=1, so
/// bisection always converges, and 24 steps go past `f32` precision. The result
/// can leave 0..=1: that is the overshoot the curve asks for.
pub(crate) fn cubic_bezier(x1: f32, y1: f32, x2: f32, y2: f32, t: f32) -> f32 {
    if t <= 0.0 {
        return 0.0;
    }
    if t >= 1.0 {
        return 1.0;
    }
    let (x1, x2) = (x1.clamp(0.0, 1.0), x2.clamp(0.0, 1.0));
    // B(s) = 3(1-s)^2 s P1 + 3(1-s) s^2 P2 + s^3, with P0 = (0, 0), P3 = (1, 1)
    let bezier = |p1: f32, p2: f32, s: f32| {
        let m = 1.0 - s;
        3.0 * m * m * s * p1 + 3.0 * m * s * s * p2 + s * s * s
    };
    let (mut low, mut high) = (0.0_f32, 1.0_f32);
    for _ in 0..24 {
        let middle = (low + high) / 2.0;
        if bezier(x1, x2, middle) < t {
            low = middle;
        } else {
            high = middle;
        }
    }
    bezier(y1, y2, (low + high) / 2.0)
}

/// Split `s` at the characters `is_separator` accepts, except inside parentheses
///
/// `cubic-bezier(0, 1, 0, 1)` has both commas and spaces in it and is one value.
/// Empty pieces are left out.
pub(crate) fn split_outside_parens(s: &str, is_separator: impl Fn(char) -> bool) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut depth = 0_usize;
    let mut start = 0;
    for (i, c) in s.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => depth = depth.saturating_sub(1),
            c if depth == 0 && is_separator(c) => {
                parts.push(&s[start..i]);
                start = i + c.len_utf8();
            }
            _ => {}
        }
    }
    parts.push(&s[start..]);
    parts.retain(|part| !part.trim().is_empty());
    parts
}

/// A single property transition definition
#[derive(Debug, Clone, PartialEq)]
pub struct Transition {
    /// Property name to transition
    pub property: String,
    /// Duration of the transition
    pub duration: Duration,
    /// Delay before starting
    pub delay: Duration,
    /// Easing function
    pub easing: Easing,
}

impl Transition {
    /// Create a new transition
    pub fn new(property: impl Into<String>, duration: Duration) -> Self {
        Self {
            property: property.into(),
            duration,
            delay: Duration::ZERO,
            easing: Easing::EaseInOut,
        }
    }

    /// Set delay
    pub fn delay(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self
    }

    /// Set easing
    pub fn easing(mut self, easing: Easing) -> Self {
        self.easing = easing;
        self
    }

    /// Parse from CSS string like "opacity 0.3s ease-in-out"
    pub fn parse(s: &str) -> Option<Self> {
        let parts = split_outside_parens(s, char::is_whitespace);
        if parts.is_empty() {
            return None;
        }

        let property = parts[0].to_string();
        let mut duration = Duration::from_millis(300);
        let mut delay = Duration::ZERO;
        let mut easing = Easing::EaseInOut;
        // As in CSS, the first time is the duration and the second the delay
        let mut duration_seen = false;

        for part in parts.iter().skip(1) {
            if let Some(dur) = parse_duration(part) {
                if duration_seen {
                    delay = dur;
                } else {
                    duration = dur;
                    duration_seen = true;
                }
            } else if let Some(e) = Easing::parse(part) {
                easing = e;
            }
        }

        Some(Self {
            property,
            duration,
            delay,
            easing,
        })
    }
}

impl Default for Transition {
    fn default() -> Self {
        Self::new("all", Duration::from_millis(300))
    }
}

/// Collection of transitions for multiple properties
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Transitions {
    /// Individual transitions
    pub items: Vec<Transition>,
}

impl Transitions {
    /// Create empty transitions
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Add a transition
    pub fn with(mut self, transition: Transition) -> Self {
        self.items.push(transition);
        self
    }

    /// Get transition for a specific property
    ///
    /// The last one that applies wins, as in CSS: with `all 1s, opacity 0.2s`,
    /// opacity takes 0.2s.
    pub fn get(&self, property: &str) -> Option<&Transition> {
        self.items
            .iter()
            .rev()
            .find(|t| t.property == property || t.property == "all")
    }

    /// Check if a property should be transitioned
    pub fn has(&self, property: &str) -> bool {
        self.get(property).is_some()
    }

    /// Parse from CSS string like "opacity 0.3s, transform 0.5s ease-out"
    pub fn parse(s: &str) -> Self {
        let items = split_outside_parens(s, |c| c == ',')
            .into_iter()
            .filter_map(|part| Transition::parse(part.trim()))
            .collect();
        Self { items }
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

/// Parse duration from CSS string like "0.3s" or "300ms"
pub(crate) fn parse_duration(s: &str) -> Option<Duration> {
    let s = s.trim();

    if let Some(ms) = s.strip_suffix("ms") {
        ms.parse::<u64>().ok().map(Duration::from_millis)
    } else if let Some(secs) = s.strip_suffix('s') {
        // Negative, NaN, infinite or too large is no duration - as `-1ms`
        // already is - rather than a panic in `from_secs_f64`.
        secs.parse::<f64>()
            .ok()
            .and_then(|secs| Duration::try_from_secs_f64(secs).ok())
    } else {
        None
    }
}

// Tests below use private function parse_duration

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_duration() {
        assert_eq!(parse_duration("300ms"), Some(Duration::from_millis(300)));
        assert_eq!(parse_duration("0.3s"), Some(Duration::from_millis(300)));
        assert_eq!(parse_duration("1s"), Some(Duration::from_secs(1)));
    }

    #[test]
    fn test_easing_apply_linear() {
        assert_eq!(Easing::Linear.apply(0.0), 0.0);
        assert_eq!(Easing::Linear.apply(0.5), 0.5);
        assert_eq!(Easing::Linear.apply(1.0), 1.0);
    }

    #[test]
    fn test_easing_apply_ease_in() {
        assert_eq!(Easing::EaseIn.apply(0.0), 0.0);
        assert_eq!(Easing::EaseIn.apply(0.5), 0.25);
        assert_eq!(Easing::EaseIn.apply(1.0), 1.0);
    }

    #[test]
    fn test_easing_apply_ease_out() {
        assert_eq!(Easing::EaseOut.apply(0.0), 0.0);
        assert_eq!(Easing::EaseOut.apply(0.5), 0.75);
        assert_eq!(Easing::EaseOut.apply(1.0), 1.0);
    }

    #[test]
    fn test_easing_apply_ease_in_out() {
        assert_eq!(Easing::EaseInOut.apply(0.0), 0.0);
        assert_eq!(Easing::EaseInOut.apply(0.25), 0.125);
        assert_eq!(Easing::EaseInOut.apply(0.5), 0.5);
        assert_eq!(Easing::EaseInOut.apply(0.75), 0.875);
        assert_eq!(Easing::EaseInOut.apply(1.0), 1.0);
    }

    #[test]
    fn test_easing_apply_clamps() {
        assert_eq!(Easing::Linear.apply(-0.5), 0.0);
        assert_eq!(Easing::Linear.apply(1.5), 1.0);
    }

    #[test]
    fn test_easing_parse() {
        assert_eq!(Easing::parse("linear"), Some(Easing::Linear));
        assert_eq!(Easing::parse("ease"), Some(Easing::EaseInOut));
        assert_eq!(Easing::parse("ease-in"), Some(Easing::EaseIn));
        assert_eq!(Easing::parse("ease-out"), Some(Easing::EaseOut));
        assert_eq!(Easing::parse("ease-in-out"), Some(Easing::EaseInOut));
    }

    #[test]
    fn test_easing_parse_case_insensitive() {
        assert_eq!(Easing::parse("LINEAR"), Some(Easing::Linear));
        assert_eq!(Easing::parse("Ease-In"), Some(Easing::EaseIn));
        assert_eq!(Easing::parse("  EASE-OUT  "), Some(Easing::EaseOut));
    }

    #[test]
    fn test_easing_parse_cubic_bezier() {
        assert_eq!(
            Easing::parse("cubic-bezier(0.25, 0.1, 0.25, 1.0)"),
            Some(Easing::CubicBezier(0.25, 0.1, 0.25, 1.0))
        );
    }

    #[test]
    fn test_easing_parse_invalid() {
        assert_eq!(Easing::parse("invalid"), None);
        assert_eq!(Easing::parse("cubic-bezier(0.25, 0.1)"), None);
    }

    #[test]
    fn test_easing_default() {
        assert_eq!(Easing::default(), Easing::Linear);
    }

    #[test]
    fn test_transition_new() {
        let transition = Transition::new("opacity", Duration::from_millis(300));
        assert_eq!(transition.property, "opacity");
        assert_eq!(transition.duration, Duration::from_millis(300));
        assert_eq!(transition.delay, Duration::ZERO);
        assert_eq!(transition.easing, Easing::EaseInOut);
    }

    #[test]
    fn test_transition_builder() {
        let transition = Transition::new("color", Duration::from_millis(200))
            .delay(Duration::from_millis(50))
            .easing(Easing::EaseOut);
        assert_eq!(transition.property, "color");
        assert_eq!(transition.duration, Duration::from_millis(200));
        assert_eq!(transition.delay, Duration::from_millis(50));
        assert_eq!(transition.easing, Easing::EaseOut);
    }

    #[test]
    fn test_transition_parse() {
        let transition = Transition::parse("opacity 0.3s ease-in-out").unwrap();
        assert_eq!(transition.property, "opacity");
        assert_eq!(transition.duration, Duration::from_millis(300));
        assert_eq!(transition.easing, Easing::EaseInOut);
    }

    #[test]
    fn test_transition_parse_with_delay() {
        let transition = Transition::parse("opacity 0.1s 0.3s").unwrap();
        assert_eq!(transition.property, "opacity");
        // First duration (100ms) is used as duration since it's the first parsed
        assert_eq!(transition.duration, Duration::from_millis(100));
        // Second duration (300ms) becomes delay
        assert_eq!(transition.delay, Duration::from_millis(300));
    }

    #[test]
    fn test_transition_default() {
        let transition = Transition::default();
        assert_eq!(transition.property, "all");
        assert_eq!(transition.duration, Duration::from_millis(300));
    }

    #[test]
    fn test_transitions_new() {
        let transitions = Transitions::new();
        assert!(transitions.is_empty());
    }

    #[test]
    fn test_transitions_default() {
        let transitions = Transitions::default();
        assert!(transitions.is_empty());
    }

    #[test]
    fn test_transitions_with() {
        let transitions =
            Transitions::new().with(Transition::new("opacity", Duration::from_millis(300)));
        assert!(!transitions.is_empty());
    }

    #[test]
    fn test_transitions_get() {
        let transitions =
            Transitions::new().with(Transition::new("opacity", Duration::from_millis(300)));
        assert!(transitions.get("opacity").is_some());
        assert!(transitions.get("color").is_none());
    }

    #[test]
    fn test_transitions_get_all() {
        let transitions =
            Transitions::new().with(Transition::new("all", Duration::from_millis(300)));
        assert!(transitions.get("opacity").is_some());
        assert!(transitions.get("color").is_some());
    }

    #[test]
    fn test_transitions_has() {
        let transitions =
            Transitions::new().with(Transition::new("opacity", Duration::from_millis(300)));
        assert!(transitions.has("opacity"));
        assert!(!transitions.has("color"));
    }

    #[test]
    fn test_transitions_parse() {
        let transitions = Transitions::parse("opacity 0.3s, color 0.5s ease-out");
        assert_eq!(transitions.items.len(), 2);
    }
}
