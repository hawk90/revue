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
}
