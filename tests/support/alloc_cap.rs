//! A heap cap for every test binary.
//!
//! macOS has no per-process memory limit that works (`ulimit -v` is ignored):
//! a loop that stops advancing while it pushes - a sabotage edit or a mutant
//! turning `offset += n` into `offset *= n` - grows until the machine runs out
//! of memory and the kernel panics (watchdog timeout, compressor full). The
//! test binaries peak below 500 MB, so a binary holding more than [`LIMIT`] is
//! a runaway. Past it, allocation fails and the binary aborts with
//! "memory allocation of N bytes failed" - this cap refusing, while the
//! machine still has memory to spare.
//!
//! Included with `#[path]` by `src/lib.rs` (unit tests) and by each
//! `tests/*.rs` crate root, so each test binary installs it once.

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

/// Live heap bytes one test binary may hold.
pub const LIMIT: usize = 1 << 30;

struct CappedAlloc {
    used: AtomicUsize,
}

impl CappedAlloc {
    /// Counts `size` more bytes as used, or refuses if that passes [`LIMIT`].
    fn reserve(&self, size: usize) -> bool {
        let mut used = self.used.load(Ordering::Relaxed);
        loop {
            let Some(total) = used.checked_add(size).filter(|&total| total <= LIMIT) else {
                return false;
            };
            match self
                .used
                .compare_exchange_weak(used, total, Ordering::Relaxed, Ordering::Relaxed)
            {
                Ok(_) => return true,
                Err(current) => used = current,
            }
        }
    }

    fn release(&self, size: usize) {
        self.used.fetch_sub(size, Ordering::Relaxed);
    }
}

// SAFETY: every call forwards to `System` with the caller's layout; the
// counter only decides whether to forward and never touches the memory.
unsafe impl GlobalAlloc for CappedAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if !self.reserve(layout.size()) {
            return std::ptr::null_mut();
        }
        let ptr = unsafe { System.alloc(layout) };
        if ptr.is_null() {
            self.release(layout.size());
        }
        ptr
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        if !self.reserve(layout.size()) {
            return std::ptr::null_mut();
        }
        let ptr = unsafe { System.alloc_zeroed(layout) };
        if ptr.is_null() {
            self.release(layout.size());
        }
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) };
        self.release(layout.size());
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let old_size = layout.size();
        if new_size > old_size && !self.reserve(new_size - old_size) {
            return std::ptr::null_mut();
        }
        let new_ptr = unsafe { System.realloc(ptr, layout, new_size) };
        if new_ptr.is_null() {
            // The old block is still live at `old_size`.
            if new_size > old_size {
                self.release(new_size - old_size);
            }
        } else if new_size < old_size {
            self.release(old_size - new_size);
        }
        new_ptr
    }
}

#[global_allocator]
static ALLOC: CappedAlloc = CappedAlloc {
    used: AtomicUsize::new(0),
};

#[cfg(test)]
mod tests {
    use super::LIMIT;

    // Runs in every binary that includes this file, so each one shows its
    // own cap is installed. `try_reserve` gets the refusal as an `Err`
    // instead of an abort; without the cap the reservation succeeds.
    #[test]
    fn heap_cap_refuses_a_reservation_past_the_limit() {
        assert!(Vec::<u8>::new().try_reserve_exact(LIMIT + 1).is_err());
    }
}
