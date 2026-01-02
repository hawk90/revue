//! CSS Transition animations
//!
//! Supports smooth transitions between style property values.
//!
//! # Example
//!
//! ```css
//! .button {
//!     background: #333;
//!     transition: background 0.3s ease-in-out;
//! }
//!
//! .button:hover {
//!     background: #555;
//! }
//! ```

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
            Easing::CubicBezier(_x1, y1, _x2, y2) => {
                // Simplified cubic bezier - more accurate would need iteration
                let t2 = t * t;
                let t3 = t2 * t;
                let mt = 1.0 - t;
                let mt2 = mt * mt;

                // B(t) = (1-t)^3*P0 + 3*(1-t)^2*t*P1 + 3*(1-t)*t^2*P2 + t^3*P3
                // P0 = (0, 0), P3 = (1, 1)
                let y = 3.0 * mt2 * t * (*y1) + 3.0 * mt * t2 * (*y2) + t3;
                y.clamp(0.0, 1.0)
            }
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
                if parts.len() == 4 {
                    Some(Easing::CubicBezier(parts[0], parts[1], parts[2], parts[3]))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
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
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.is_empty() {
            return None;
        }

        let property = parts[0].to_string();
        let mut duration = Duration::from_millis(300);
        let mut delay = Duration::ZERO;
        let mut easing = Easing::EaseInOut;

        for (i, part) in parts.iter().enumerate().skip(1) {
            if let Some(dur) = parse_duration(part) {
                if i == 1 || duration == Duration::from_millis(300) {
                    duration = dur;
                } else {
                    delay = dur;
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
    pub fn get(&self, property: &str) -> Option<&Transition> {
        self.items
            .iter()
            .find(|t| t.property == property || t.property == "all")
    }

    /// Check if a property should be transitioned
    pub fn has(&self, property: &str) -> bool {
        self.get(property).is_some()
    }

    /// Parse from CSS string like "opacity 0.3s, transform 0.5s ease-out"
    pub fn parse(s: &str) -> Self {
        let items = s
            .split(',')
            .filter_map(|part| Transition::parse(part.trim()))
            .collect();
        Self { items }
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

/// Active transition state for a property
#[derive(Debug, Clone)]
pub struct ActiveTransition {
    /// Property being transitioned
    pub property: String,
    /// Start value
    pub from: f32,
    /// End value
    pub to: f32,
    /// Duration
    pub duration: Duration,
    /// Delay
    pub delay: Duration,
    /// Easing
    pub easing: Easing,
    /// Elapsed time
    pub elapsed: Duration,
    /// Whether the transition has started (after delay)
    pub started: bool,
}

impl ActiveTransition {
    /// Create a new active transition
    pub fn new(
        property: impl Into<String>,
        from: f32,
        to: f32,
        transition: &Transition,
    ) -> Self {
        Self {
            property: property.into(),
            from,
            to,
            duration: transition.duration,
            delay: transition.delay,
            easing: transition.easing,
            elapsed: Duration::ZERO,
            started: transition.delay.is_zero(),
        }
    }

    /// Update the transition with elapsed time
    pub fn update(&mut self, delta: Duration) {
        self.elapsed += delta;

        if !self.started && self.elapsed >= self.delay {
            self.started = true;
            self.elapsed = self.elapsed.saturating_sub(self.delay);
        }
    }

    /// Get current interpolated value
    pub fn current(&self) -> f32 {
        if !self.started {
            return self.from;
        }

        let progress = if self.duration.is_zero() {
            1.0
        } else {
            (self.elapsed.as_secs_f32() / self.duration.as_secs_f32()).min(1.0)
        };

        let eased = self.easing.apply(progress);
        self.from + (self.to - self.from) * eased
    }

    /// Check if transition is complete
    pub fn is_complete(&self) -> bool {
        self.started && self.elapsed >= self.duration
    }
}

/// Transition manager for handling multiple active transitions
#[derive(Debug, Clone, Default)]
pub struct TransitionManager {
    /// Active transitions (legacy, not node-aware)
    active: Vec<ActiveTransition>,
    /// Node-aware transitions: maps element ID to active transitions
    node_transitions: std::collections::HashMap<String, Vec<ActiveTransition>>,
}

impl TransitionManager {
    /// Create a new transition manager
    pub fn new() -> Self {
        Self {
            active: Vec::new(),
            node_transitions: std::collections::HashMap::new(),
        }
    }

    /// Start a transition for a property
    ///
    /// If reduced motion is preferred, the transition completes instantly
    /// (no animation, just jumps to final value).
    pub fn start(
        &mut self,
        property: impl Into<String>,
        from: f32,
        to: f32,
        transition: &Transition,
    ) {
        let property = property.into();

        // Remove existing transition for this property
        self.active.retain(|t| t.property != property);

        // Skip animation entirely if reduced motion is preferred
        if should_skip_animation() {
            // Don't add any transition - the property will use final value directly
            return;
        }

        // Add new transition
        self.active
            .push(ActiveTransition::new(&property, from, to, transition));
    }

    /// Update all transitions
    pub fn update(&mut self, delta: Duration) {
        for transition in &mut self.active {
            transition.update(delta);
        }

        // Remove completed transitions
        self.active.retain(|t| !t.is_complete());
    }

    /// Get current value for a property
    pub fn get(&self, property: &str) -> Option<f32> {
        self.active
            .iter()
            .find(|t| t.property == property)
            .map(|t| t.current())
    }

    /// Check if there are active transitions
    pub fn has_active(&self) -> bool {
        !self.active.is_empty() || !self.node_transitions.is_empty()
    }

    /// Get all active transition property names
    pub fn active_properties(&self) -> impl Iterator<Item = &str> {
        self.active.iter().map(|t| t.property.as_str())
    }

    /// Clear all transitions
    pub fn clear(&mut self) {
        self.active.clear();
        self.node_transitions.clear();
    }

    /// Get all current transition values as a map
    ///
    /// Returns a HashMap that can be passed to RenderContext for widgets
    /// to access animated property values during rendering.
    pub fn current_values(&self) -> std::collections::HashMap<String, f32> {
        self.active
            .iter()
            .map(|t| (t.property.clone(), t.current()))
            .collect()
    }

    // =========================================================================
    // Node-aware transition methods for partial rendering optimization
    // =========================================================================

    /// Start a transition for a specific element
    ///
    /// Associates the transition with an element ID for partial rendering.
    /// If reduced motion is preferred, no transition is added.
    pub fn start_for_node(
        &mut self,
        element_id: impl Into<String>,
        property: impl Into<String>,
        from: f32,
        to: f32,
        transition: &Transition,
    ) {
        // Skip animation entirely if reduced motion is preferred
        if should_skip_animation() {
            return;
        }

        let element_id = element_id.into();
        let property = property.into();

        let transitions = self.node_transitions.entry(element_id).or_default();

        // Remove existing transition for this property
        transitions.retain(|t| t.property != property);

        // Add new transition
        transitions.push(ActiveTransition::new(&property, from, to, transition));
    }

    /// Update all node-aware transitions
    pub fn update_nodes(&mut self, delta: Duration) {
        for transitions in self.node_transitions.values_mut() {
            for transition in transitions.iter_mut() {
                transition.update(delta);
            }
            // Remove completed transitions
            transitions.retain(|t| !t.is_complete());
        }

        // Remove entries with no active transitions
        self.node_transitions.retain(|_, v| !v.is_empty());
    }

    /// Get current value for a property on a specific node
    pub fn get_for_node(&self, element_id: &str, property: &str) -> Option<f32> {
        self.node_transitions
            .get(element_id)?
            .iter()
            .find(|t| t.property == property)
            .map(|t| t.current())
    }

    /// Get all element IDs with active transitions
    ///
    /// Used for partial rendering - only these nodes need to be redrawn.
    pub fn active_node_ids(&self) -> impl Iterator<Item = &str> {
        self.node_transitions.keys().map(|s| s.as_str())
    }

    /// Check if a specific node has active transitions
    pub fn node_has_active(&self, element_id: &str) -> bool {
        self.node_transitions
            .get(element_id)
            .map(|v| !v.is_empty())
            .unwrap_or(false)
    }

    /// Get all current transition values for a specific node
    pub fn current_values_for_node(&self, element_id: &str) -> std::collections::HashMap<String, f32> {
        self.node_transitions
            .get(element_id)
            .map(|transitions| {
                transitions
                    .iter()
                    .map(|t| (t.property.clone(), t.current()))
                    .collect()
            })
            .unwrap_or_default()
    }
}

/// Check if animations should be skipped due to reduced motion preference
///
/// When reduced motion is preferred, this returns true and animations
/// should complete instantly instead of animating.
pub fn should_skip_animation() -> bool {
    crate::utils::prefers_reduced_motion()
}

/// Get effective duration considering reduced motion preference
///
/// Returns Duration::ZERO if reduced motion is preferred (instant transition).
pub fn effective_duration(duration: Duration) -> Duration {
    if should_skip_animation() {
        Duration::ZERO
    } else {
        duration
    }
}

/// Parse duration from CSS string like "0.3s" or "300ms"
fn parse_duration(s: &str) -> Option<Duration> {
    let s = s.trim();

    if let Some(ms) = s.strip_suffix("ms") {
        ms.parse::<u64>().ok().map(Duration::from_millis)
    } else if let Some(secs) = s.strip_suffix('s') {
        secs.parse::<f64>().ok().map(Duration::from_secs_f64)
    } else {
        None
    }
}

/// Interpolate between two u8 values
pub fn lerp_u8(from: u8, to: u8, t: f32) -> u8 {
    let from = from as f32;
    let to = to as f32;
    (from + (to - from) * t).round() as u8
}

/// Interpolate between two f32 values
pub fn lerp_f32(from: f32, to: f32, t: f32) -> f32 {
    from + (to - from) * t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_easing_linear() {
        assert_eq!(Easing::Linear.apply(0.0), 0.0);
        assert_eq!(Easing::Linear.apply(0.5), 0.5);
        assert_eq!(Easing::Linear.apply(1.0), 1.0);
    }

    #[test]
    fn test_easing_ease_in() {
        let e = Easing::EaseIn;
        assert_eq!(e.apply(0.0), 0.0);
        assert!(e.apply(0.5) < 0.5); // Slow start
        assert_eq!(e.apply(1.0), 1.0);
    }

    #[test]
    fn test_easing_ease_out() {
        let e = Easing::EaseOut;
        assert_eq!(e.apply(0.0), 0.0);
        assert!(e.apply(0.5) > 0.5); // Fast start
        assert_eq!(e.apply(1.0), 1.0);
    }

    #[test]
    fn test_easing_parse() {
        assert_eq!(Easing::parse("linear"), Some(Easing::Linear));
        assert_eq!(Easing::parse("ease-in"), Some(Easing::EaseIn));
        assert_eq!(Easing::parse("ease-out"), Some(Easing::EaseOut));
        assert_eq!(Easing::parse("ease-in-out"), Some(Easing::EaseInOut));
    }

    #[test]
    fn test_transition_parse() {
        let t = Transition::parse("opacity 0.3s ease-in").unwrap();
        assert_eq!(t.property, "opacity");
        assert_eq!(t.duration, Duration::from_millis(300));
        assert_eq!(t.easing, Easing::EaseIn);
    }

    #[test]
    fn test_transition_parse_with_delay() {
        let t = Transition::parse("transform 0.5s 0.1s ease-out").unwrap();
        assert_eq!(t.property, "transform");
        assert_eq!(t.duration, Duration::from_millis(500));
        assert_eq!(t.delay, Duration::from_millis(100));
        assert_eq!(t.easing, Easing::EaseOut);
    }

    #[test]
    fn test_transitions_parse() {
        let ts = Transitions::parse("opacity 0.3s, background 0.5s ease-out");
        assert_eq!(ts.items.len(), 2);
        assert!(ts.has("opacity"));
        assert!(ts.has("background"));
    }

    #[test]
    fn test_active_transition() {
        let t = Transition::new("opacity", Duration::from_secs(1));
        let mut active = ActiveTransition::new("opacity", 0.0, 1.0, &t);

        assert_eq!(active.current(), 0.0);

        active.update(Duration::from_millis(500));
        let mid = active.current();
        assert!(mid > 0.0 && mid < 1.0);

        active.update(Duration::from_millis(500));
        assert_eq!(active.current(), 1.0);
        assert!(active.is_complete());
    }

    #[test]
    fn test_transition_manager() {
        let mut manager = TransitionManager::new();
        let t = Transition::new("opacity", Duration::from_secs(1));

        manager.start("opacity", 0.0, 1.0, &t);
        assert!(manager.has_active());

        manager.update(Duration::from_secs(2));
        assert!(!manager.has_active()); // Completed and removed
    }

    #[test]
    fn test_parse_duration() {
        assert_eq!(parse_duration("300ms"), Some(Duration::from_millis(300)));
        assert_eq!(parse_duration("0.3s"), Some(Duration::from_millis(300)));
        assert_eq!(parse_duration("1s"), Some(Duration::from_secs(1)));
    }

    #[test]
    fn test_lerp() {
        assert_eq!(lerp_u8(0, 100, 0.5), 50);
        assert_eq!(lerp_f32(0.0, 1.0, 0.5), 0.5);
    }

    // =========================================================================
    // Node-aware transition tests
    // =========================================================================

    #[test]
    fn test_node_aware_transition() {
        let mut manager = TransitionManager::new();
        let t = Transition::new("opacity", Duration::from_secs(1));

        manager.start_for_node("btn1", "opacity", 0.0, 1.0, &t);

        assert!(manager.has_active());
        assert!(manager.node_has_active("btn1"));
        assert!(!manager.node_has_active("btn2"));

        // Check active node IDs
        let active_ids: Vec<&str> = manager.active_node_ids().collect();
        assert_eq!(active_ids, vec!["btn1"]);
    }

    #[test]
    fn test_node_transition_update() {
        let mut manager = TransitionManager::new();
        let t = Transition::new("opacity", Duration::from_secs(1));

        manager.start_for_node("btn1", "opacity", 0.0, 1.0, &t);

        // Update halfway
        manager.update_nodes(Duration::from_millis(500));

        let value = manager.get_for_node("btn1", "opacity").unwrap();
        assert!(value > 0.0 && value < 1.0);

        // Complete the transition
        manager.update_nodes(Duration::from_millis(600));

        // Should be removed after completion
        assert!(!manager.node_has_active("btn1"));
    }

    #[test]
    fn test_multiple_node_transitions() {
        let mut manager = TransitionManager::new();
        let t1 = Transition::new("opacity", Duration::from_secs(1));
        let t2 = Transition::new("scale", Duration::from_millis(500));

        manager.start_for_node("btn1", "opacity", 0.0, 1.0, &t1);
        manager.start_for_node("btn1", "scale", 1.0, 1.5, &t2);
        manager.start_for_node("btn2", "opacity", 1.0, 0.0, &t1);

        assert!(manager.node_has_active("btn1"));
        assert!(manager.node_has_active("btn2"));

        // btn1 should have 2 properties
        let values = manager.current_values_for_node("btn1");
        assert_eq!(values.len(), 2);
        assert!(values.contains_key("opacity"));
        assert!(values.contains_key("scale"));
    }

    #[test]
    fn test_clear_clears_node_transitions() {
        let mut manager = TransitionManager::new();
        let t = Transition::new("opacity", Duration::from_secs(1));

        manager.start_for_node("btn1", "opacity", 0.0, 1.0, &t);
        assert!(manager.has_active());

        manager.clear();

        assert!(!manager.has_active());
        assert!(!manager.node_has_active("btn1"));
    }
}
