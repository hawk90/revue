//! Spring physics animations

/// Spring physics simulation for smooth, natural motion
///
/// Uses a critically damped spring model for smooth animations
/// without oscillation (unless configured to bounce).
#[derive(Clone, Debug)]
pub struct Spring {
    /// Current value
    value: f64,
    /// Target value
    target: f64,
    /// Velocity
    velocity: f64,
    /// Stiffness (spring constant)
    stiffness: f64,
    /// Damping ratio (1.0 = critical damping)
    damping: f64,
    /// Mass
    mass: f64,
    /// Threshold for settling
    threshold: f64,
}

impl Spring {
    /// Create a new spring at initial value
    pub fn new(initial: f64, target: f64) -> Self {
        Self {
            value: initial,
            target,
            velocity: 0.0,
            stiffness: 180.0,
            damping: 12.0,
            mass: 1.0,
            threshold: 0.01,
        }
    }

    /// Create a spring starting at target (no animation)
    pub fn at(value: f64) -> Self {
        Self::new(value, value)
    }

    /// Set stiffness (higher = faster, snappier)
    pub fn stiffness(mut self, stiffness: f64) -> Self {
        self.stiffness = stiffness.max(0.1);
        self
    }

    /// Set damping ratio (1.0 = critical, <1 = bouncy, >1 = sluggish)
    pub fn damping(mut self, damping: f64) -> Self {
        self.damping = damping.max(0.1);
        self
    }

    /// Set mass (higher = slower, more momentum)
    pub fn mass(mut self, mass: f64) -> Self {
        self.mass = mass.max(0.01);
        self
    }

    /// Set settling threshold
    pub fn threshold(mut self, threshold: f64) -> Self {
        self.threshold = threshold.max(0.0001);
        self
    }

    /// Preset: snappy animation
    pub fn snappy() -> Self {
        Self::at(0.0).stiffness(400.0).damping(30.0)
    }

    /// Preset: gentle animation
    pub fn gentle() -> Self {
        Self::at(0.0).stiffness(100.0).damping(15.0)
    }

    /// Preset: bouncy animation
    pub fn bouncy() -> Self {
        Self::at(0.0).stiffness(200.0).damping(8.0)
    }

    /// Preset: slow animation
    pub fn slow() -> Self {
        Self::at(0.0).stiffness(50.0).damping(10.0)
    }

    /// Set target value
    pub fn set_target(&mut self, target: f64) {
        self.target = target;
    }

    /// Set value immediately (no animation)
    pub fn set_value(&mut self, value: f64) {
        self.value = value;
        self.velocity = 0.0;
    }

    /// Get current value
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Get target value
    pub fn target(&self) -> f64 {
        self.target
    }

    /// Get velocity
    pub fn velocity(&self) -> f64 {
        self.velocity
    }

    /// Check if spring has settled (close to target with low velocity)
    pub fn is_settled(&self) -> bool {
        (self.value - self.target).abs() < self.threshold && self.velocity.abs() < self.threshold
    }

    /// Update spring simulation
    ///
    /// Call this every frame with the time delta.
    /// Returns the current value.
    pub fn update(&mut self, dt: f64) -> f64 {
        if self.is_settled() {
            self.value = self.target;
            self.velocity = 0.0;
            return self.value;
        }

        // Spring force: F = -k * x
        let displacement = self.value - self.target;
        let spring_force = -self.stiffness * displacement;

        // Damping force: F = -c * v
        let damping_force = -self.damping * self.velocity;

        // Acceleration: a = F / m
        let acceleration = (spring_force + damping_force) / self.mass;

        // Update velocity and position
        self.velocity += acceleration * dt;
        self.value += self.velocity * dt;

        self.value
    }

    /// Update with fixed 60fps timestep
    pub fn tick(&mut self) -> f64 {
        self.update(1.0 / 60.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spring_new() {
        let spring = Spring::new(0.0, 100.0);
        assert_eq!(spring.value(), 0.0);
        assert_eq!(spring.target(), 100.0);
        assert_eq!(spring.velocity(), 0.0);
    }

    #[test]
    fn test_spring_at() {
        let spring = Spring::at(50.0);
        assert_eq!(spring.value(), 50.0);
        assert_eq!(spring.target(), 50.0);
        assert!(spring.is_settled());
    }

    #[test]
    fn test_spring_stiffness() {
        let spring = Spring::new(0.0, 100.0).stiffness(200.0);
        assert_eq!(spring.stiffness, 200.0);
    }

    #[test]
    fn test_spring_stiffness_clamps_minimum() {
        let spring = Spring::new(0.0, 100.0).stiffness(0.0);
        assert_eq!(spring.stiffness, 0.1);
    }

    #[test]
    fn test_spring_damping() {
        let spring = Spring::new(0.0, 100.0).damping(20.0);
        assert_eq!(spring.damping, 20.0);
    }

    #[test]
    fn test_spring_damping_clamps_minimum() {
        let spring = Spring::new(0.0, 100.0).damping(0.0);
        assert_eq!(spring.damping, 0.1);
    }

    #[test]
    fn test_spring_mass() {
        let spring = Spring::new(0.0, 100.0).mass(2.0);
        assert_eq!(spring.mass, 2.0);
    }

    #[test]
    fn test_spring_mass_clamps_minimum() {
        let spring = Spring::new(0.0, 100.0).mass(0.0);
        assert_eq!(spring.mass, 0.01);
    }

    #[test]
    fn test_spring_threshold() {
        let spring = Spring::new(0.0, 100.0).threshold(0.1);
        assert_eq!(spring.threshold, 0.1);
    }

    #[test]
    fn test_spring_threshold_clamps_minimum() {
        let spring = Spring::new(0.0, 100.0).threshold(0.0);
        assert_eq!(spring.threshold, 0.0001);
    }

    #[test]
    fn test_spring_snappy() {
        let spring = Spring::snappy();
        assert_eq!(spring.value(), 0.0);
        assert_eq!(spring.target(), 0.0);
        assert_eq!(spring.stiffness, 400.0);
        assert_eq!(spring.damping, 30.0);
    }

    #[test]
    fn test_spring_gentle() {
        let spring = Spring::gentle();
        assert_eq!(spring.value(), 0.0);
        assert_eq!(spring.target(), 0.0);
        assert_eq!(spring.stiffness, 100.0);
        assert_eq!(spring.damping, 15.0);
    }

    #[test]
    fn test_spring_bouncy() {
        let spring = Spring::bouncy();
        assert_eq!(spring.value(), 0.0);
        assert_eq!(spring.target(), 0.0);
        assert_eq!(spring.stiffness, 200.0);
        assert_eq!(spring.damping, 8.0);
    }

    #[test]
    fn test_spring_slow() {
        let spring = Spring::slow();
        assert_eq!(spring.value(), 0.0);
        assert_eq!(spring.target(), 0.0);
        assert_eq!(spring.stiffness, 50.0);
        assert_eq!(spring.damping, 10.0);
    }

    #[test]
    fn test_spring_set_target() {
        let mut spring = Spring::new(0.0, 100.0);
        spring.set_target(50.0);
        assert_eq!(spring.target(), 50.0);
    }

    #[test]
    fn test_spring_set_value() {
        let mut spring = Spring::new(0.0, 100.0);
        spring.set_value(75.0);
        assert_eq!(spring.value(), 75.0);
        assert_eq!(spring.velocity(), 0.0);
    }

    #[test]
    fn test_spring_is_settled() {
        let spring = Spring::at(50.0);
        assert!(spring.is_settled());
    }

    #[test]
    fn test_spring_is_settled_false() {
        let spring = Spring::new(0.0, 100.0);
        assert!(!spring.is_settled());
    }

    #[test]
    fn test_spring_update_changes_value() {
        let mut spring = Spring::new(0.0, 100.0);
        let value1 = spring.update(0.016);
        let value2 = spring.update(0.016);
        // Spring should move toward target
        assert!(value2 > value1);
        assert!(value2 <= 100.0);
    }

    #[test]
    fn test_spring_tick() {
        let mut spring = Spring::new(0.0, 100.0);
        let value1 = spring.tick();
        let value2 = spring.tick();
        // Spring should move toward target
        assert!(value2 > value1);
    }

    #[test]
    fn test_spring_update_settled() {
        let mut spring = Spring::at(50.0);
        spring.set_target(50.0);
        let value = spring.update(0.016);
        // Already at target
        assert_eq!(value, 50.0);
    }

    #[test]
    fn test_spring_default_fields() {
        let spring = Spring::new(0.0, 100.0);
        assert_eq!(spring.velocity(), 0.0);
        assert_eq!(spring.stiffness, 180.0);
        assert_eq!(spring.damping, 12.0);
        assert_eq!(spring.mass, 1.0);
        assert_eq!(spring.threshold, 0.01);
    }

    #[test]
    fn test_spring_velocity_changes() {
        let mut spring = Spring::new(0.0, 100.0);
        let v1 = spring.velocity();
        spring.update(0.016);
        let v2 = spring.velocity();
        // Velocity should change as spring accelerates
        assert_ne!(v1, v2);
    }

    #[test]
    fn test_spring_velocity_toward_target() {
        let mut spring = Spring::new(0.0, 100.0);
        spring.update(0.016);
        // Velocity should be positive (toward 100)
        assert!(spring.velocity() > 0.0);
    }

    #[test]
    fn test_spring_velocity_from_target() {
        let mut spring = Spring::new(100.0, 0.0);
        spring.update(0.016);
        // Velocity should be negative (toward 0)
        assert!(spring.velocity() < 0.0);
    }

    #[test]
    fn test_spring_is_settled_near_target() {
        let mut spring = Spring::new(0.0, 10.0);
        // Move close to target
        for _ in 0..1000 {
            spring.update(0.016);
            if spring.is_settled() {
                break;
            }
        }
        // Should eventually settle
        assert!(spring.is_settled());
        assert!((spring.value() - 10.0).abs() < 0.1);
    }
}
