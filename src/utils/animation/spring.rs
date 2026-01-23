//! Spring physics simulation for smooth, natural motion

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
