//! Easing functions for animations

/// Linear interpolation (no easing)
pub fn linear(t: f32) -> f32 {
    t
}

/// Ease in (slow start)
pub fn ease_in(t: f32) -> f32 {
    t * t
}

/// Ease out (slow end)
pub fn ease_out(t: f32) -> f32 {
    1.0 - (1.0 - t) * (1.0 - t)
}

/// Ease in-out (slow start and end)
pub fn ease_in_out(t: f32) -> f32 {
    if t < 0.5 {
        2.0 * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
    }
}

/// Cubic ease in
pub fn ease_in_cubic(t: f32) -> f32 {
    t * t * t
}

/// Cubic ease out
pub fn ease_out_cubic(t: f32) -> f32 {
    1.0 - (1.0 - t).powi(3)
}

/// Cubic ease in-out
pub fn ease_in_out_cubic(t: f32) -> f32 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
    }
}

/// Bounce ease out
pub fn bounce_out(t: f32) -> f32 {
    let n1 = 7.5625;
    let d1 = 2.75;

    if t < 1.0 / d1 {
        n1 * t * t
    } else if t < 2.0 / d1 {
        let t = t - 1.5 / d1;
        n1 * t * t + 0.75
    } else if t < 2.5 / d1 {
        let t = t - 2.25 / d1;
        n1 * t * t + 0.9375
    } else {
        let t = t - 2.625 / d1;
        n1 * t * t + 0.984375
    }
}

/// Elastic ease out
pub fn elastic_out(t: f32) -> f32 {
    if t == 0.0 {
        0.0
    } else if t == 1.0 {
        1.0
    } else {
        let c4 = (2.0 * std::f32::consts::PI) / 3.0;
        2.0_f32.powf(-10.0 * t) * ((t * 10.0 - 0.75) * c4).sin() + 1.0
    }
}

/// Back ease out (overshoot)
pub fn back_out(t: f32) -> f32 {
    let c1 = 1.70158;
    let c3 = c1 + 1.0;
    1.0 + c3 * (t - 1.0).powi(3) + c1 * (t - 1.0).powi(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear() {
        assert_eq!(linear(0.0), 0.0);
        assert_eq!(linear(0.5), 0.5);
        assert_eq!(linear(1.0), 1.0);
    }

    #[test]
    fn test_ease_in() {
        assert_eq!(ease_in(0.0), 0.0);
        assert_eq!(ease_in(1.0), 1.0);
        assert!(ease_in(0.5) < 0.5); // slow start
    }

    #[test]
    fn test_ease_out() {
        assert_eq!(ease_out(0.0), 0.0);
        assert_eq!(ease_out(1.0), 1.0);
        assert!(ease_out(0.5) > 0.5); // fast start
    }

    #[test]
    fn test_ease_in_out() {
        assert_eq!(ease_in_out(0.0), 0.0);
        assert_eq!(ease_in_out(1.0), 1.0);
        assert_eq!(ease_in_out(0.5), 0.5); // midpoint is exact
    }

    #[test]
    fn test_ease_in_cubic() {
        assert_eq!(ease_in_cubic(0.0), 0.0);
        assert_eq!(ease_in_cubic(1.0), 1.0);
        assert!(ease_in_cubic(0.5) < 0.5);
    }

    #[test]
    fn test_ease_out_cubic() {
        assert_eq!(ease_out_cubic(0.0), 0.0);
        assert_eq!(ease_out_cubic(1.0), 1.0);
        assert!(ease_out_cubic(0.5) > 0.5);
    }

    #[test]
    fn test_ease_in_out_cubic() {
        assert_eq!(ease_in_out_cubic(0.0), 0.0);
        assert_eq!(ease_in_out_cubic(1.0), 1.0);
        assert_eq!(ease_in_out_cubic(0.5), 0.5);
    }

    #[test]
    fn test_bounce_out() {
        assert_eq!(bounce_out(0.0), 0.0);
        assert!((bounce_out(1.0) - 1.0).abs() < 0.001);
        // Bounce should have values > 0 in the middle
        assert!(bounce_out(0.5) > 0.0);
    }

    #[test]
    fn test_elastic_out() {
        assert_eq!(elastic_out(0.0), 0.0);
        assert_eq!(elastic_out(1.0), 1.0);
        // Elastic can overshoot
        assert!(elastic_out(0.5) > 0.0);
    }

    #[test]
    fn test_back_out() {
        assert_eq!(back_out(0.0), 0.0);
        assert!((back_out(1.0) - 1.0).abs() < 0.001);
        // Back can overshoot
        assert!(back_out(0.5) > 0.0);
    }

    #[test]
    fn test_all_easing_bounds() {
        // All easing functions should be between -0.5 and 1.5 for t in [0, 1]
        for i in 0..=10 {
            let t = i as f32 / 10.0;
            assert!(linear(t) >= 0.0 && linear(t) <= 1.0);
            assert!(ease_in(t) >= 0.0 && ease_in(t) <= 1.0);
            assert!(ease_out(t) >= 0.0 && ease_out(t) <= 1.0);
            assert!(ease_in_out(t) >= 0.0 && ease_in_out(t) <= 1.0);
            assert!(ease_in_cubic(t) >= 0.0 && ease_in_cubic(t) <= 1.0);
            assert!(ease_out_cubic(t) >= 0.0 && ease_out_cubic(t) <= 1.0);
            assert!(ease_in_out_cubic(t) >= 0.0 && ease_in_out_cubic(t) <= 1.0);
            assert!(bounce_out(t) >= 0.0 && bounce_out(t) <= 1.1);
            // elastic and back can overshoot, so wider bounds
            assert!(elastic_out(t) >= -0.5 && elastic_out(t) <= 1.5);
            assert!(back_out(t) >= -0.5 && back_out(t) <= 1.5);
        }
    }
}
