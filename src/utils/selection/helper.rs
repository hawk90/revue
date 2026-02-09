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
