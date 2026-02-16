//! Easing function implementations
//!
//! Based on Robert Penner's easing equations.

use std::f64::consts::PI;

// ============================================================================
// Constants for back and elastic easing
// ============================================================================

const C1: f64 = 1.70158;
const C2: f64 = C1 * 1.525;
const C3: f64 = C1 + 1.0;

const C4: f64 = (2.0 * PI) / 3.0;
const C5: f64 = (2.0 * PI) / 4.5;

const N1: f64 = 7.5625;
const D1: f64 = 2.75;

// ============================================================================
// Linear
// ============================================================================

/// Linear interpolation (no easing)
pub fn linear(t: f64) -> f64 {
    t.clamp(0.0, 1.0)
}

// ============================================================================
// Quadratic
// ============================================================================

/// Quadratic ease in
pub fn ease_in_quad(t: f64) -> f64 {
    t * t
}

/// Quadratic ease out
pub fn ease_out_quad(t: f64) -> f64 {
    1.0 - (1.0 - t) * (1.0 - t)
}

/// Quadratic ease in-out
pub fn ease_in_out_quad(t: f64) -> f64 {
    if t < 0.5 {
        2.0 * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
    }
}

// ============================================================================
// Cubic
// ============================================================================

/// Cubic ease in
pub fn ease_in_cubic(t: f64) -> f64 {
    t * t * t
}

/// Cubic ease out
pub fn ease_out_cubic(t: f64) -> f64 {
    1.0 - (1.0 - t).powi(3)
}

/// Cubic ease in-out
pub fn ease_in_out_cubic(t: f64) -> f64 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
    }
}

// ============================================================================
// Quartic
// ============================================================================

/// Quartic ease in
pub fn ease_in_quart(t: f64) -> f64 {
    t * t * t * t
}

/// Quartic ease out
pub fn ease_out_quart(t: f64) -> f64 {
    1.0 - (1.0 - t).powi(4)
}

/// Quartic ease in-out
pub fn ease_in_out_quart(t: f64) -> f64 {
    if t < 0.5 {
        8.0 * t * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(4) / 2.0
    }
}

// ============================================================================
// Quintic
// ============================================================================

/// Quintic ease in
pub fn ease_in_quint(t: f64) -> f64 {
    t * t * t * t * t
}

/// Quintic ease out
pub fn ease_out_quint(t: f64) -> f64 {
    1.0 - (1.0 - t).powi(5)
}

/// Quintic ease in-out
pub fn ease_in_out_quint(t: f64) -> f64 {
    if t < 0.5 {
        16.0 * t * t * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(5) / 2.0
    }
}

// ============================================================================
// Sine
// ============================================================================

/// Sine ease in
pub fn ease_in_sine(t: f64) -> f64 {
    1.0 - (t * PI / 2.0).cos()
}

/// Sine ease out
pub fn ease_out_sine(t: f64) -> f64 {
    (t * PI / 2.0).sin()
}

/// Sine ease in-out
pub fn ease_in_out_sine(t: f64) -> f64 {
    -(((t * PI).cos() - 1.0) / 2.0)
}

// ============================================================================
// Exponential
// ============================================================================

/// Exponential ease in
pub fn ease_in_expo(t: f64) -> f64 {
    if t == 0.0 {
        0.0
    } else {
        2.0_f64.powf(10.0 * t - 10.0)
    }
}

/// Exponential ease out
pub fn ease_out_expo(t: f64) -> f64 {
    if t == 1.0 {
        1.0
    } else {
        1.0 - 2.0_f64.powf(-10.0 * t)
    }
}

/// Exponential ease in-out
pub fn ease_in_out_expo(t: f64) -> f64 {
    if t == 0.0 {
        0.0
    } else if t == 1.0 {
        1.0
    } else if t < 0.5 {
        2.0_f64.powf(20.0 * t - 10.0) / 2.0
    } else {
        (2.0 - 2.0_f64.powf(-20.0 * t + 10.0)) / 2.0
    }
}

// ============================================================================
// Circular
// ============================================================================

/// Circular ease in
pub fn ease_in_circ(t: f64) -> f64 {
    1.0 - (1.0 - t * t).sqrt()
}

/// Circular ease out
pub fn ease_out_circ(t: f64) -> f64 {
    (1.0 - (t - 1.0).powi(2)).sqrt()
}

/// Circular ease in-out
pub fn ease_in_out_circ(t: f64) -> f64 {
    if t < 0.5 {
        (1.0 - (1.0 - (2.0 * t).powi(2)).sqrt()) / 2.0
    } else {
        ((1.0 - (-2.0 * t + 2.0).powi(2)).sqrt() + 1.0) / 2.0
    }
}

// ============================================================================
// Back (overshoots)
// ============================================================================

/// Back ease in (overshoots at start)
pub fn ease_in_back(t: f64) -> f64 {
    C3 * t * t * t - C1 * t * t
}

/// Back ease out (overshoots at end)
pub fn ease_out_back(t: f64) -> f64 {
    1.0 + C3 * (t - 1.0).powi(3) + C1 * (t - 1.0).powi(2)
}

/// Back ease in-out
pub fn ease_in_out_back(t: f64) -> f64 {
    if t < 0.5 {
        ((2.0 * t).powi(2) * ((C2 + 1.0) * 2.0 * t - C2)) / 2.0
    } else {
        ((2.0 * t - 2.0).powi(2) * ((C2 + 1.0) * (t * 2.0 - 2.0) + C2) + 2.0) / 2.0
    }
}

// ============================================================================
// Elastic
// ============================================================================

/// Elastic ease in
pub fn ease_in_elastic(t: f64) -> f64 {
    if t == 0.0 {
        0.0
    } else if t == 1.0 {
        1.0
    } else {
        -2.0_f64.powf(10.0 * t - 10.0) * ((t * 10.0 - 10.75) * C4).sin()
    }
}

/// Elastic ease out
pub fn ease_out_elastic(t: f64) -> f64 {
    if t == 0.0 {
        0.0
    } else if t == 1.0 {
        1.0
    } else {
        2.0_f64.powf(-10.0 * t) * ((t * 10.0 - 0.75) * C4).sin() + 1.0
    }
}

/// Elastic ease in-out
pub fn ease_in_out_elastic(t: f64) -> f64 {
    if t == 0.0 {
        0.0
    } else if t == 1.0 {
        1.0
    } else if t < 0.5 {
        -(2.0_f64.powf(20.0 * t - 10.0) * ((20.0 * t - 11.125) * C5).sin()) / 2.0
    } else {
        (2.0_f64.powf(-20.0 * t + 10.0) * ((20.0 * t - 11.125) * C5).sin()) / 2.0 + 1.0
    }
}

// ============================================================================
// Bounce
// ============================================================================

/// Bounce ease out (base function)
pub fn ease_out_bounce(t: f64) -> f64 {
    if t < 1.0 / D1 {
        N1 * t * t
    } else if t < 2.0 / D1 {
        let t = t - 1.5 / D1;
        N1 * t * t + 0.75
    } else if t < 2.5 / D1 {
        let t = t - 2.25 / D1;
        N1 * t * t + 0.9375
    } else {
        let t = t - 2.625 / D1;
        N1 * t * t + 0.984375
    }
}

/// Bounce ease in
pub fn ease_in_bounce(t: f64) -> f64 {
    1.0 - ease_out_bounce(1.0 - t)
}

/// Bounce ease in-out
pub fn ease_in_out_bounce(t: f64) -> f64 {
    if t < 0.5 {
        (1.0 - ease_out_bounce(1.0 - 2.0 * t)) / 2.0
    } else {
        (1.0 + ease_out_bounce(2.0 * t - 1.0)) / 2.0
    }
}
