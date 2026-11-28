//! The CPU time of the calling thread, for the "too slow" checks.
//!
//! A slow check is there to catch a parser or widget doing too much work on
//! one input (a quadratic loop, a runaway backtrack). Wall-clock time also
//! counts the time the thread waited for a core, so on a loaded machine (the
//! parallel pre-push hook, a busy CI runner) a fast input looked slow (#923).
//! The thread's own CPU time counts only the work it did.
//!
//! Waits for other processes or for file events (a clipboard tool, a file
//! watcher) and the "hung" limits stay on wall-clock time: those measure how
//! long something takes to happen, not how much work this thread does.
//!
//! Only the calling thread is counted: a check timed this way must do its
//! work on that thread (the parser, session and resource targets do; none of
//! them spawns threads).

use std::time::Duration;

/// CPU time (user + system) the calling thread has used so far.
#[cfg(unix)]
pub fn thread_cpu_time() -> Duration {
    let mut ts = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    // SAFETY: `ts` is a valid, writable timespec, and
    // CLOCK_THREAD_CPUTIME_ID is supported on every unix revue builds for.
    let rc = unsafe { libc::clock_gettime(libc::CLOCK_THREAD_CPUTIME_ID, &mut ts) };
    assert_eq!(rc, 0, "clock_gettime(CLOCK_THREAD_CPUTIME_ID) failed");
    Duration::new(ts.tv_sec as u64, ts.tv_nsec as u32)
}

/// CPU time (user + kernel) the calling thread has used so far.
#[cfg(windows)]
pub fn thread_cpu_time() -> Duration {
    use windows_sys::Win32::Foundation::FILETIME;
    use windows_sys::Win32::System::Threading::{GetCurrentThread, GetThreadTimes};

    let zero = || FILETIME {
        dwLowDateTime: 0,
        dwHighDateTime: 0,
    };
    let (mut creation, mut exit, mut kernel, mut user) = (zero(), zero(), zero(), zero());
    // SAFETY: GetCurrentThread returns a pseudo handle that needs no closing,
    // and every out pointer is a valid, writable FILETIME.
    let ok = unsafe {
        GetThreadTimes(
            GetCurrentThread(),
            &mut creation,
            &mut exit,
            &mut kernel,
            &mut user,
        )
    };
    assert_ne!(ok, 0, "GetThreadTimes failed");
    // FILETIME counts 100 ns intervals
    let ticks = |t: FILETIME| (u64::from(t.dwHighDateTime) << 32) | u64::from(t.dwLowDateTime);
    Duration::from_nanos((ticks(kernel) + ticks(user)) * 100)
}

/// Run `f`; its result and the CPU time this thread spent in it.
pub fn cpu_timed<R>(f: impl FnOnce() -> R) -> (R, Duration) {
    let start = thread_cpu_time();
    let r = f();
    (r, thread_cpu_time().saturating_sub(start))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn waiting_is_not_counted() {
        let ((), took) = cpu_timed(|| std::thread::sleep(Duration::from_millis(300)));
        assert!(
            took < Duration::from_millis(100),
            "sleeping 300ms used {took:?} of CPU"
        );
    }

    #[test]
    fn work_is_counted() {
        // Spin until 50ms of CPU time have passed; wall clock bounds the loop
        // only so a clock that never moves fails instead of hanging
        let wall = Instant::now();
        let start = thread_cpu_time();
        let mut x = 0u64;
        while thread_cpu_time().saturating_sub(start) < Duration::from_millis(50) {
            x = std::hint::black_box(x.wrapping_mul(31).wrapping_add(7));
            assert!(
                wall.elapsed() < Duration::from_secs(30),
                "CPU time did not advance"
            );
        }
    }

    #[test]
    fn other_threads_work_is_not_counted() {
        let ((), took) = cpu_timed(|| {
            std::thread::spawn(|| {
                let start = thread_cpu_time();
                let mut x = 0u64;
                while thread_cpu_time().saturating_sub(start) < Duration::from_millis(200) {
                    x = std::hint::black_box(x.wrapping_add(1));
                }
            })
            .join()
            .unwrap();
        });
        assert!(
            took < Duration::from_millis(100),
            "another thread's 200ms counted as {took:?}"
        );
    }
}
