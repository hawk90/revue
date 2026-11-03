//! The value: setting it within the bounds, stepping it, and formatting
//! it for display

use super::NumberInput;

impl NumberInput {
    /// Get the current value
    pub fn get_value(&self) -> f64 {
        self.value
    }

    /// Get the current value as an integer
    pub fn get_int(&self) -> i64 {
        self.value.round() as i64
    }

    /// Set the value (with clamping)
    pub fn set_value(&mut self, value: f64) {
        self.value = self.clamp_value(value);
        self.editing = false;
    }

    /// Increment the value by step
    pub fn increment(&mut self) {
        self.set_value(self.value + self.step);
    }

    /// Decrement the value by step
    pub fn decrement(&mut self) {
        self.set_value(self.value - self.step);
    }

    /// Increment by large step (10x normal step)
    pub fn increment_large(&mut self) {
        self.set_value(self.value + self.step * 10.0);
    }

    /// Decrement by large step (10x normal step)
    pub fn decrement_large(&mut self) {
        self.set_value(self.value - self.step * 10.0);
    }

    /// Set to the lower bound (if defined)
    pub fn set_to_min(&mut self) {
        if let (Some(low), _) = self.bounds() {
            self.set_value(low);
        }
    }

    /// Set to the upper bound (if defined)
    pub fn set_to_max(&mut self) {
        if let (_, Some(high)) = self.bounds() {
            self.set_value(high);
        }
    }

    /// The bounds, low first, at the precision: a range given high to low
    /// (`min > max`) is swapped, as Slider's is
    fn bounds(&self) -> (Option<f64>, Option<f64>) {
        let (low, high) = match (self.min, self.max) {
            (Some(min), Some(max)) if min > max => (Some(max), Some(min)),
            bounds => bounds,
        };
        (low.map(|v| self.rounded(v)), high.map(|v| self.rounded(v)))
    }

    /// `value` at the precision: the number its shown text reads back as,
    /// so the two never disagree (rounding included), and one that rounds
    /// to zero is zero, not -0; as it is without one
    fn rounded(&self, value: f64) -> f64 {
        match self.precision {
            Some(p) if value.is_finite() => {
                let v: f64 = format!("{value:.0$}", p as usize).parse().unwrap_or(value);
                v + 0.0
            }
            _ => value,
        }
    }

    /// `value` within the bounds. NaN is no number to hold: the current
    /// value is kept.
    pub(super) fn clamp_value(&self, value: f64) -> f64 {
        if value.is_nan() {
            return self.value;
        }
        let mut v = value;
        let (low, high) = self.bounds();
        if let Some(low) = low {
            v = v.max(low);
        }
        if let Some(high) = high {
            v = v.min(high);
        }
        self.rounded(v)
    }

    /// Format the current value for display
    pub(super) fn format_value(&self) -> String {
        match self.precision {
            Some(p) => format!("{:.prec$}", self.value, prec = p as usize),
            None => self.value.to_string(),
        }
    }

    /// Get the full display string (with prefix/suffix)
    pub fn display_string(&self) -> String {
        let value_str = if self.editing {
            self.buffer().to_string()
        } else {
            self.format_value()
        };

        let mut result = String::new();
        if let Some(ref prefix) = self.prefix {
            result.push_str(prefix);
        }
        result.push_str(&value_str);
        if let Some(ref suffix) = self.suffix {
            result.push_str(suffix);
        }
        result
    }
}
