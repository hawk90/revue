//! List selection with viewport scrolling - helper functions

/// Wrap index forward (standalone function)
pub fn wrap_next(index: usize, len: usize) -> usize {
    if len == 0 {
        0
    } else {
        (index + 1) % len
    }
}

/// Wrap index backward (standalone function)
pub fn wrap_prev(index: usize, len: usize) -> usize {
    if len == 0 {
        0
    } else if index == 0 {
        len - 1
    } else {
        index - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrap_next_basic() {
        assert_eq!(wrap_next(0, 5), 1);
        assert_eq!(wrap_next(1, 5), 2);
        assert_eq!(wrap_next(2, 5), 3);
        assert_eq!(wrap_next(3, 5), 4);
        assert_eq!(wrap_next(4, 5), 0);
    }

    #[test]
    fn test_wrap_next_empty() {
        assert_eq!(wrap_next(0, 0), 0);
        assert_eq!(wrap_next(5, 0), 0);
    }

    #[test]
    fn test_wrap_next_single() {
        assert_eq!(wrap_next(0, 1), 0);
        assert_eq!(wrap_next(1, 1), 0);
    }

    #[test]
    fn test_wrap_prev_basic() {
        assert_eq!(wrap_prev(0, 5), 4);
        assert_eq!(wrap_prev(1, 5), 0);
        assert_eq!(wrap_prev(2, 5), 1);
        assert_eq!(wrap_prev(3, 5), 2);
        assert_eq!(wrap_prev(4, 5), 3);
    }

    #[test]
    fn test_wrap_prev_empty() {
        assert_eq!(wrap_prev(0, 0), 0);
        assert_eq!(wrap_prev(5, 0), 0);
    }

    #[test]
    fn test_wrap_prev_single() {
        assert_eq!(wrap_prev(0, 1), 0);
        assert_eq!(wrap_prev(1, 1), 0);
    }

    #[test]
    fn test_wrap_next_prev_roundtrip() {
        for len in [1, 3, 5, 10] {
            for i in 0..len {
                let next = wrap_next(i, len);
                assert_eq!(wrap_prev(next, len), i);
            }
        }
    }

    // =========================================================================
    // Additional wrap tests
    // =========================================================================

    #[test]
    fn test_wrap_next_large_index() {
        // Large index should wrap correctly
        assert_eq!(wrap_next(999, 1000), 0);
        // wrap_next(1000, 1000) = (1000 + 1) % 1000 = 1
        assert_eq!(wrap_next(1000, 1000), 1);
    }

    #[test]
    fn test_wrap_prev_large_index() {
        // Large index should wrap correctly
        assert_eq!(wrap_prev(1000, 1000), 999);
        assert_eq!(wrap_prev(0, 1000), 999);
    }

    #[test]
    fn test_wrap_next_full_cycle() {
        // Going forward n times should return to start
        let mut idx = 0;
        let len = 10;
        for _ in 0..len {
            idx = wrap_next(idx, len);
        }
        assert_eq!(idx, 0);
    }

    #[test]
    fn test_wrap_prev_full_cycle() {
        // Going backward n times should return to start
        let mut idx = 0;
        let len = 10;
        for _ in 0..len {
            idx = wrap_prev(idx, len);
        }
        assert_eq!(idx, 0);
    }

    #[test]
    fn test_wrap_next_all_values() {
        // Test that wrap_next produces all values
        let len = 5;
        let mut values = vec![false; len];
        let mut idx = 0;
        for _ in 0..(len * 2) {
            values[idx] = true;
            idx = wrap_next(idx, len);
        }
        // All values should be visited
        assert!(values.iter().all(|v| *v));
    }

    #[test]
    fn test_wrap_prev_all_values() {
        // Test that wrap_prev produces all values
        let len = 5;
        let mut values = vec![false; len];
        let mut idx = 0;
        for _ in 0..(len * 2) {
            values[idx] = true;
            idx = wrap_prev(idx, len);
        }
        // All values should be visited
        assert!(values.iter().all(|v| *v));
    }

    #[test]
    fn test_wrap_next_alternating() {
        // Test alternating next/prev returns to original
        for len in 2..=10 {
            for i in 0..len {
                let next = wrap_next(i, len);
                let prev = wrap_prev(next, len);
                assert_eq!(prev, i);
            }
        }
    }

    #[test]
    fn test_wrap_prev_alternating() {
        // Test alternating prev/next returns to original
        for len in 2..=10 {
            for i in 0..len {
                let prev = wrap_prev(i, len);
                let next = wrap_next(prev, len);
                assert_eq!(next, i);
            }
        }
    }

    #[test]
    fn test_wrap_next_two_steps() {
        // wrap_next(wrap_next(0, 5), 5) = wrap_next(1, 5) = 2
        assert_eq!(wrap_next(wrap_next(0, 5), 5), 2);
        // wrap_next(wrap_next(3, 10), 10) = wrap_next(4, 10) = 5
        assert_eq!(wrap_next(wrap_next(3, 10), 10), 5);
    }

    #[test]
    fn test_wrap_prev_two_steps() {
        // wrap_prev(wrap_prev(0, 5), 5) = wrap_prev(4, 5) = 3
        assert_eq!(wrap_prev(wrap_prev(0, 5), 5), 3);
        // wrap_prev(wrap_prev(5, 10), 10) = wrap_prev(4, 10) = 3
        assert_eq!(wrap_prev(wrap_prev(5, 10), 10), 3);
    }

    #[test]
    fn test_wrap_next_max_usize() {
        // Test with maximum usize values
        // wrap_next(usize::MAX - 1, usize::MAX) = (usize::MAX - 1 + 1) % usize::MAX = 0
        let idx = usize::MAX - 1;
        let len = usize::MAX;
        let result = wrap_next(idx, len);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_wrap_prev_max_usize() {
        // Test edge case with large values
        let idx = 1;
        let len = usize::MAX;
        let result = wrap_prev(idx, len);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_wrap_next_middle_of_list() {
        let len = 100;
        let mid = len / 2;
        assert_eq!(wrap_next(mid, len), mid + 1);
        assert_eq!(wrap_prev(mid, len), mid - 1);
    }
}
