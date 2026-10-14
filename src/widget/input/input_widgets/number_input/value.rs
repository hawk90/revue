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

    /// Set to minimum value (if defined)
    pub fn set_to_min(&mut self) {
        if let Some(min) = self.min {
            self.set_value(min);
        }
    }

    /// Set to maximum value (if defined)
    pub fn set_to_max(&mut self) {
        if let Some(max) = self.max {
            self.set_value(max);
        }
    }

    /// Clamp value to min/max constraints
    pub(super) fn clamp_value(&self, value: f64) -> f64 {
        let mut v = value;
        if let Some(min) = self.min {
            v = v.max(min);
        }
        if let Some(max) = self.max {
            v = v.min(max);
        }
        v
    }

    /// Format the current value for display
    pub(super) fn format_value(&self) -> String {
        format!("{:.prec$}", self.value, prec = self.precision as usize)
    }

    /// Get the full display string (with prefix/suffix)
    pub fn display_string(&self) -> String {
        let value_str = if self.editing {
            self.input_buffer.clone()
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
