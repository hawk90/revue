//! Breakpoints and values that vary by breakpoint

use std::collections::HashMap;

/// A responsive breakpoint
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Breakpoint {
    /// Breakpoint name (e.g., "sm", "md", "lg")
    pub name: &'static str,
    /// Minimum width for this breakpoint
    pub min_width: u16,
}

impl Breakpoint {
    /// Create a new breakpoint
    pub fn new(name: &'static str, min_width: u16) -> Self {
        Self { name, min_width }
    }
}

/// Common terminal breakpoints
impl Breakpoint {
    /// Extra small (< 40 columns) - minimal terminals
    pub const XS: Breakpoint = Breakpoint {
        name: "xs",
        min_width: 0,
    };
    /// Small (40-79 columns) - compact terminals
    pub const SM: Breakpoint = Breakpoint {
        name: "sm",
        min_width: 40,
    };
    /// Medium (80-119 columns) - standard terminals
    pub const MD: Breakpoint = Breakpoint {
        name: "md",
        min_width: 80,
    };
    /// Large (120-159 columns) - wide terminals
    pub const LG: Breakpoint = Breakpoint {
        name: "lg",
        min_width: 120,
    };
    /// Extra large (160+ columns) - ultra-wide terminals
    pub const XL: Breakpoint = Breakpoint {
        name: "xl",
        min_width: 160,
    };
}

/// Breakpoint collection
#[derive(Debug, Clone)]
pub struct Breakpoints {
    /// Sorted list of breakpoints (by min_width ascending)
    points: Vec<Breakpoint>,
}

impl Breakpoints {
    /// Create an empty breakpoint set
    pub fn new() -> Self {
        Self { points: Vec::new() }
    }

    /// Create standard terminal breakpoints
    pub fn terminal() -> Self {
        Self {
            points: vec![
                Breakpoint::XS,
                Breakpoint::SM,
                Breakpoint::MD,
                Breakpoint::LG,
                Breakpoint::XL,
            ],
        }
    }

    /// Create minimal breakpoints (small, medium, large)
    pub fn simple() -> Self {
        Self {
            points: vec![Breakpoint::SM, Breakpoint::MD, Breakpoint::LG],
        }
    }

    /// Add a breakpoint
    pub fn add(mut self, bp: Breakpoint) -> Self {
        self.points.push(bp);
        self.points.sort_by_key(|b| b.min_width);
        self
    }

    /// Get current breakpoint for width
    pub fn current(&self, width: u16) -> &Breakpoint {
        self.points
            .iter()
            .rev()
            .find(|bp| width >= bp.min_width)
            .unwrap_or_else(|| self.points.first().unwrap_or(&Breakpoint::XS))
    }

    /// Get breakpoint by name
    pub fn get(&self, name: &str) -> Option<&Breakpoint> {
        self.points.iter().find(|bp| bp.name == name)
    }

    /// Check if width matches a breakpoint name
    pub fn matches(&self, width: u16, name: &str) -> bool {
        self.current(width).name == name
    }

    /// Check if width is at least the given breakpoint
    pub fn at_least(&self, width: u16, name: &str) -> bool {
        if let Some(target) = self.get(name) {
            width >= target.min_width
        } else {
            false
        }
    }

    /// Check if width is less than the given breakpoint
    pub fn below(&self, width: u16, name: &str) -> bool {
        if let Some(target) = self.get(name) {
            width < target.min_width
        } else {
            true
        }
    }

    /// Get all breakpoint names in order
    pub fn names(&self) -> Vec<&'static str> {
        self.points.iter().map(|bp| bp.name).collect()
    }

    /// Iterate over breakpoints
    pub fn iter(&self) -> impl Iterator<Item = &Breakpoint> {
        self.points.iter()
    }
}

impl Default for Breakpoints {
    fn default() -> Self {
        Self::terminal()
    }
}

/// A value that varies based on breakpoint
#[derive(Debug, Clone)]
pub struct ResponsiveValue<T: Clone> {
    /// Default value (for smallest breakpoint)
    default: T,
    /// Values for each breakpoint
    values: HashMap<&'static str, T>,
}

impl<T: Clone> ResponsiveValue<T> {
    /// Create a new responsive value with a default
    pub fn new(default: T) -> Self {
        Self {
            default,
            values: HashMap::new(),
        }
    }

    /// Set value for a breakpoint
    pub fn at(mut self, breakpoint: &'static str, value: T) -> Self {
        self.values.insert(breakpoint, value);
        self
    }

    /// Resolve value for current width
    pub fn resolve(&self, breakpoints: &Breakpoints, width: u16) -> T {
        // Find the current breakpoint
        let current = breakpoints.current(width);

        // Walk from current breakpoint down to find a defined value
        for bp in breakpoints.points.iter().rev() {
            if bp.min_width <= current.min_width {
                if let Some(value) = self.values.get(bp.name) {
                    return value.clone();
                }
            }
        }

        self.default.clone()
    }

    /// Get the default value
    pub fn default_value(&self) -> &T {
        &self.default
    }
}

// Convenience constructors

/// Create a responsive value
pub fn responsive<T: Clone>(default: T) -> ResponsiveValue<T> {
    ResponsiveValue::new(default)
}

/// Create standard terminal breakpoints
pub fn breakpoints() -> Breakpoints {
    Breakpoints::terminal()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breakpoint_new() {
        let bp = Breakpoint::new("custom", 50);
        assert_eq!(bp.name, "custom");
        assert_eq!(bp.min_width, 50);
    }

    #[test]
    fn test_breakpoint_constants() {
        assert_eq!(Breakpoint::XS.name, "xs");
        assert_eq!(Breakpoint::XS.min_width, 0);
        assert_eq!(Breakpoint::SM.name, "sm");
        assert_eq!(Breakpoint::SM.min_width, 40);
        assert_eq!(Breakpoint::MD.name, "md");
        assert_eq!(Breakpoint::MD.min_width, 80);
        assert_eq!(Breakpoint::LG.name, "lg");
        assert_eq!(Breakpoint::LG.min_width, 120);
        assert_eq!(Breakpoint::XL.name, "xl");
        assert_eq!(Breakpoint::XL.min_width, 160);
    }

    #[test]
    fn test_breakpoints_new() {
        let bp = Breakpoints::new();
        assert!(bp.points.is_empty());
        assert!(bp.current(100).name == "xs"); // Falls back to XS when empty
    }

    #[test]
    fn test_breakpoints_terminal() {
        let bp = Breakpoints::terminal();
        assert_eq!(bp.points.len(), 5);
        assert_eq!(bp.current(30).name, "xs");
        assert_eq!(bp.current(40).name, "sm");
        assert_eq!(bp.current(80).name, "md");
        assert_eq!(bp.current(120).name, "lg");
        assert_eq!(bp.current(160).name, "xl");
    }

    #[test]
    fn test_breakpoints_simple() {
        let bp = Breakpoints::simple();
        assert_eq!(bp.points.len(), 3);
        // Should fall back to SM (smallest in simple set)
        assert!(bp.current(30).min_width >= 40 || bp.current(30).name == "sm");
    }

    #[test]
    fn test_breakpoints_add() {
        let bp = Breakpoints::new()
            .add(Breakpoint::new("small", 20))
            .add(Breakpoint::new("large", 100))
            .add(Breakpoint::new("medium", 60));
        assert_eq!(bp.points.len(), 3);
        // Points should be sorted by min_width
        assert_eq!(bp.points[0].name, "small");
        assert_eq!(bp.points[1].name, "medium");
        assert_eq!(bp.points[2].name, "large");
    }

    #[test]
    fn test_breakpoints_get() {
        let bp = Breakpoints::terminal();
        assert!(bp.get("sm").is_some());
        assert!(bp.get("md").is_some());
        assert!(bp.get("nonexistent").is_none());
    }

    #[test]
    fn test_breakpoints_matches() {
        let bp = Breakpoints::terminal();
        assert!(bp.matches(40, "sm"));
        assert!(bp.matches(80, "md"));
        assert!(!bp.matches(30, "sm"));
    }

    #[test]
    fn test_breakpoints_at_least() {
        let bp = Breakpoints::terminal();
        assert!(bp.at_least(80, "sm"));
        assert!(bp.at_least(80, "md"));
        assert!(!bp.at_least(40, "md"));
    }

    #[test]
    fn test_breakpoints_below() {
        let bp = Breakpoints::terminal();
        assert!(bp.below(40, "md"));
        assert!(!bp.below(80, "md"));
    }

    #[test]
    fn test_breakpoints_names() {
        let bp = Breakpoints::terminal();
        let names = bp.names();
        assert_eq!(names, vec!["xs", "sm", "md", "lg", "xl"]);
    }

    #[test]
    fn test_responsive_value_new() {
        let rv = ResponsiveValue::new(10);
        assert_eq!(*rv.default_value(), 10);
    }

    #[test]
    fn test_responsive_value_at() {
        let rv = ResponsiveValue::new(1).at("sm", 2).at("md", 3);
        let bp = Breakpoints::terminal();
        assert_eq!(rv.resolve(&bp, 40), 2);
        assert_eq!(rv.resolve(&bp, 80), 3);
        assert_eq!(rv.resolve(&bp, 10), 1); // Falls back to default
    }

    #[test]
    fn test_responsive_value_default() {
        let rv = ResponsiveValue::new(100);
        let bp = Breakpoints::terminal();
        assert_eq!(rv.resolve(&bp, 50), 100);
        assert_eq!(rv.resolve(&bp, 100), 100);
    }

    #[test]
    fn test_responsive_function() {
        let rv = responsive::<u16>(42);
        assert_eq!(*rv.default_value(), 42);
    }

    #[test]
    fn test_breakpoints_function() {
        let bp = breakpoints();
        assert_eq!(bp.current(80).name, "md");
    }
}
