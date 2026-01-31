//! One-shot execution utility
//!
//! A simple utility to ensure a callback is executed only once.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::utils::Once;
//!
//! let mut once = Once::new();
//! for _ in 0..10 {
//!     if once.call() {
//!         println!("This will only print once!");
//!     }
//! }
//! ```

use std::sync::atomic::{AtomicBool, Ordering};

/// A one-shot execution guard
///
/// Ensures that code is executed only once, even if `call()` is invoked multiple times.
/// This is useful for initialization, cleanup, or ensuring side effects happen once.
///
/// # Thread Safety
///
/// `Once` uses atomic operations and is safe to use across threads.
#[derive(Debug, Default)]
pub struct Once {
    /// Flag indicating if the action has been executed
    executed: AtomicBool,
}

impl Once {
    /// Create a new `Once` guard
    ///
    /// # Example
    ///
    /// ```rust
    /// use revue::utils::Once;
    ///
    /// let once = Once::new();
    /// assert!(!once.is_called());
    /// ```
    pub fn new() -> Self {
        Self {
            executed: AtomicBool::new(false),
        }
    }

    /// Attempt to execute the one-shot action
    ///
    /// Returns `true` on the first call (allowing execution), and `false` on all
    /// subsequent calls (preventing re-execution).
    ///
    /// # Example
    ///
    /// ```rust
    /// use revue::utils::Once;
    ///
    /// let mut once = Once::new();
    /// assert!(once.call());  // First call returns true
    /// assert!(!once.call()); // Subsequent calls return false
    /// assert!(!once.call()); // Always returns false after first
    /// ```
    #[inline]
    pub fn call(&mut self) -> bool {
        self.call_impl()
    }

    /// Internal implementation using atomic operations
    #[inline]
    fn call_impl(&self) -> bool {
        !self.executed.swap(true, Ordering::AcqRel)
    }

    /// Check if the one-shot has been called
    ///
    /// # Example
    ///
    /// ```rust
    /// use revue::utils::Once;
    ///
    /// let mut once = Once::new();
    /// assert!(!once.is_called());
    /// once.call();
    /// assert!(once.is_called());
    /// ```
    #[inline]
    pub fn is_called(&self) -> bool {
        self.executed.load(Ordering::Acquire)
    }

    /// Reset the one-shot, allowing it to be called again
    ///
    /// # Warning
    ///
    /// This can be useful in certain scenarios, but be careful not to create
    /// unexpected behavior. Use with caution!
    ///
    /// # Example
    ///
    /// ```rust
    /// use revue::utils::Once;
    ///
    /// let mut once = Once::new();
    /// once.call();
    /// assert!(!once.call()); // Already called
    ///
    /// once.reset();
    /// assert!(once.call()); // Can call again after reset
    /// ```
    #[inline]
    pub fn reset(&mut self) {
        self.executed.store(false, Ordering::Release);
    }

    /// Create a new `Once` that's already in the called state
    ///
    /// This is useful when you want to skip execution based on some condition.
    ///
    /// # Example
    ///
    /// ```rust
    /// use revue::utils::Once;
    ///
    /// let already_initialized = true;
    /// let mut once = Once::from(already_initialized);
    ///
    /// if once.call() {
    ///     println!("This won't print");
    /// }
    /// ```
    pub fn from(called: bool) -> Self {
        Self {
            executed: AtomicBool::new(called),
        }
    }
}

impl Clone for Once {
    fn clone(&self) -> Self {
        Self {
            executed: AtomicBool::new(self.is_called()),
        }
    }
}

/// Helper function to create a new `Once` guard
///
/// # Example
///
/// ```rust
/// use revue::utils::once;
///
/// let mut one_shot = once();
/// if one_shot.call() {
///     // Execute once
/// }
/// ```
pub fn once() -> Once {
    Once::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_once_new() {
        let once = Once::new();
        assert!(!once.is_called());
    }

    #[test]
    fn test_once_single_call() {
        let mut once = Once::new();
        assert!(once.call());
        assert!(once.is_called());
    }

    #[test]
    fn test_once_multiple_calls() {
        let mut once = Once::new();
        assert!(once.call()); // First
        assert!(!once.call()); // Second
        assert!(!once.call()); // Third
        assert!(!once.call()); // Fourth
        assert!(once.is_called());
    }

    #[test]
    fn test_once_reset() {
        let mut once = Once::new();
        assert!(once.call());
        assert!(!once.call());

        once.reset();
        assert!(!once.is_called());
        assert!(once.call());
        assert!(!once.call());
    }

    #[test]
    fn test_once_from_true() {
        let mut once = Once::from(true);
        assert!(once.is_called());
        assert!(!once.call());
    }

    #[test]
    fn test_once_from_false() {
        let mut once = Once::from(false);
        assert!(!once.is_called());
        assert!(once.call());
    }

    #[test]
    fn test_once_helper() {
        let mut one_shot = once();
        assert!(one_shot.call());
        assert!(!one_shot.call());
    }

    #[test]
    fn test_once_default() {
        let once = Once::default();
        assert!(!once.is_called());
    }

    #[test]
    fn test_once_clone() {
        let mut once1 = Once::new();
        once1.call();

        let mut once2 = once1.clone();
        assert!(once2.is_called());
        assert!(!once2.call());
    }

    #[test]
    fn test_once_clone_uncalled() {
        let once1 = Once::new();
        let mut once2 = once1.clone();

        assert!(once2.call());
        // Cloned Once has its own independent state
        assert!(!once1.is_called());
    }

    #[test]
    fn test_once_in_loop() {
        let mut once = Once::new();
        let mut count = 0;

        for _ in 0..100 {
            if once.call() {
                count += 1;
            }
        }

        assert_eq!(count, 1);
    }

    #[test]
    fn test_once_with_callback() {
        let mut once = Once::new();
        let mut executed = false;

        for _ in 0..10 {
            if once.call() {
                executed = true;
            }
        }

        assert!(executed);
    }
}
