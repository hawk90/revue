//! Concurrency faults: tasks that panic, never finish or are cancelled,
//! queues that fill, locks that are poisoned, reactive callbacks that
//! re-enter, and plugins that fail.
//!
//! Injected:
//!
//! - **tasks** (`TaskRunner`, `PooledTaskRunner`): a task that panics or
//!   returns an error, one that does not finish while the runner is dropped,
//!   cancel-then-respawn under the same id, a full work queue, 500 spawns;
//! - **workers** (`WorkerHandle`, `WorkerPool`, `WorkerChannel`): a blocking
//!   task or a future that panics, a future that never finishes and is
//!   cancelled, dropping a handle or a pool while a task runs, a full pool
//!   queue, submitting after shutdown, cancelling through a full (or
//!   zero-capacity) command queue, a dropped receiver, 4 senders at once;
//! - **locks**: a `Signal::update`, a `Computed` and a `SignalVec` diff
//!   subscriber that panic while a lock is held;
//! - **reactive**: a subscriber or an effect that sets the signal it reacts
//!   to (converging, and forever), an effect that panics, dropping a
//!   subscription from inside a notification, a `SignalVec` subscriber that
//!   pushes to its own vector, cyclic and self-referential `Computed`s, a
//!   batched update that queues another, a panicking batch, two overlapping
//!   `use_async` triggers, a custom event handler that registers another;
//! - **plugins**: a plugin whose init, mount, tick or unmount errors or
//!   panics.
//!
//! Invariants: no panic escapes to the caller - except a documented one
//! (the reactive "maximum update depth" panic for an update loop); nothing
//! deadlocks or hangs (every case has a time limit; the update-loop cases run
//! in a child process, since a stack overflow aborts); state stays consistent
//! (pending counts return to 0, a cancelled task's result is not delivered,
//! the dependency tracker is left clean, every mounted plugin is unmounted, a
//! panicking plugin is disabled while the others run their whole lifecycle);
//! errors keep their message.
//!
//! Background tasks that "panic" do it with `resume_unwind`, which skips the
//! panic hook, so the run stays quiet; a task that "never finishes" blocks on
//! a [`Gate`] the case opens when it is done (or that lets go after 30 s).
//!
//! Case keys: `"<area> <fault>"`, e.g. `"runner cancel-then-respawn"`,
//! `"reactive computed-cycle"`, `"plugin mount-error"`.

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, Instant};

use super::{eventually, ratchet, run_bounded, run_in_child, Case, Gate};

const LAYER: &str = "concurrency";
const LIMIT: Duration = Duration::from_secs(10);
/// For a case whose failure mode is a hang: no need to wait long.
const SHORT: Duration = Duration::from_secs(4);
/// How long a case waits for something that should happen promptly.
const PROMPT: Duration = Duration::from_secs(3);

/// Environment variable naming the case a child process runs.
const CHILD: &str = "REVUE_FAULT_CONCURRENCY_CASE";

/// Panic without the panic hook - quiet in any thread.
fn quiet_panic(msg: &'static str) -> ! {
    std::panic::resume_unwind(Box::new(msg))
}

/// Run `f`; `Some(message)` if it panicked.
fn panics(f: impl FnOnce()) -> Option<String> {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(f))
        .err()
        .map(|p| {
            p.downcast_ref::<&str>()
                .map(|s| s.to_string())
                .or_else(|| p.downcast_ref::<String>().cloned())
                .unwrap_or_else(|| "<non-string panic>".into())
        })
}

fn leak(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

// ─── Tasks ──────────────────────────────────────────────────────────────────

fn task_cases(cases: &mut Vec<Case>) {
    use revue::tasks::{PooledTaskRunner, TaskRunner};

    cases.push(Case::new("runner panic-message", LIMIT, || {
        let mut runner: TaskRunner<u32> = TaskRunner::new();
        runner.spawn("boom", || quiet_panic("boom: the disk is gone"));
        let mut result = None;
        eventually(PROMPT, || {
            result = runner.poll();
            result.is_some()
        });
        match result {
            Some(r) => match r.result {
                Err(e) if e.contains("the disk is gone") => None,
                other => Some(format!("panic message lost: {other:?}")),
            },
            None => Some("no result for a panicking task".into()),
        }
    }));

    cases.push(Case::new("runner cancelled-result", LIMIT, || {
        let gate = Gate::new();
        let g = gate.clone();
        let mut runner: TaskRunner<u32> = TaskRunner::new();
        runner.spawn("slow", move || {
            g.wait();
            1
        });
        runner.cancel("slow");
        gate.open();
        std::thread::sleep(Duration::from_millis(200));
        let got = runner.poll();
        if runner.pending_count() != 0 {
            return Some("pending count is not 0 after cancel".into());
        }
        got.map(|r| format!("a cancelled task's result was delivered: {:?}", r.result))
    }));

    cases.push(Case::new("runner cancel-then-respawn", LIMIT, || {
        let (first, second) = (Gate::new(), Gate::new());
        let mut runner: TaskRunner<u32> = TaskRunner::new();
        let g = first.clone();
        runner.spawn("job", move || {
            g.wait();
            1
        });
        runner.cancel("job");
        let g = second.clone();
        runner.spawn("job", move || {
            g.wait();
            2
        });
        first.open();
        std::thread::sleep(Duration::from_millis(200));
        let early = runner.poll();
        let running = runner.is_running("job");
        second.open();
        let mut late = None;
        eventually(PROMPT, || {
            late = runner.poll();
            late.is_some()
        });
        if let Some(r) = early {
            return Some(format!(
                "the cancelled first run's result was delivered: {:?} (respawn still running: {running})",
                r.result
            ));
        }
        if !running {
            return Some("the respawned task is not reported running".into());
        }
        match late.map(|r| r.result) {
            Some(Ok(2)) => None,
            other => Some(format!("the respawned task's result: {other:?}")),
        }
    }));

    cases.push(Case::new("runner drop-running", SHORT, || {
        let gate = Gate::new();
        let g = gate.clone();
        let mut runner: TaskRunner<u32> = TaskRunner::new();
        runner.spawn("forever", move || {
            g.wait();
            0
        });
        let start = Instant::now();
        drop(runner);
        gate.open();
        let took = start.elapsed();
        (took > Duration::from_secs(1))
            .then(|| format!("dropping the runner blocked for {took:?} on a running task"))
    }));

    cases.push(Case::new("runner many-spawns", LIMIT, || {
        let mut runner: TaskRunner<usize> = TaskRunner::new();
        for i in 0..500 {
            runner.spawn(leak(format!("t{i}")), move || i);
        }
        let mut got = 0;
        eventually(Duration::from_secs(8), || {
            while runner.poll().is_some() {
                got += 1;
            }
            got == 500
        });
        (got != 500 || runner.pending_count() != 0).then(|| {
            format!(
                "{got}/500 results, {} still pending",
                runner.pending_count()
            )
        })
    }));

    cases.push(Case::new("pooled panic-message", LIMIT, || {
        let mut runner: PooledTaskRunner<u32> = PooledTaskRunner::new(2);
        runner.spawn("boom", || quiet_panic("boom: the disk is gone"));
        let mut result = None;
        eventually(PROMPT, || {
            result = runner.poll();
            result.is_some()
        });
        match result.map(|r| r.result) {
            Some(Err(e)) if e.contains("the disk is gone") => None,
            other => Some(format!("panic message lost: {other:?}")),
        }
    }));

    cases.push(Case::new("pooled error-message", LIMIT, || {
        let mut runner: PooledTaskRunner<u32> = PooledTaskRunner::new(2);
        runner.spawn_result("err", || Err::<u32, _>("permission denied"));
        let mut result = None;
        eventually(PROMPT, || {
            result = runner.poll();
            result.is_some()
        });
        match result.map(|r| r.result) {
            Some(Err(e)) if e.contains("permission denied") => None,
            other => Some(format!("error message lost: {other:?}")),
        }
    }));

    cases.push(Case::new("pooled full-queue", LIMIT, || {
        let gate = Gate::new();
        let mut runner: PooledTaskRunner<usize> = PooledTaskRunner::new(1);
        let total = revue::constants::MAX_TASK_QUEUE_SIZE + 10;
        for i in 0..total {
            let g = gate.clone();
            runner.spawn(format!("t{i}"), move || {
                g.wait();
                i
            });
        }
        gate.open();
        let mut got = 0;
        eventually(Duration::from_secs(8), || {
            while runner.poll().is_some() {
                got += 1;
            }
            runner.pending_count() == 0
        });
        let pending = runner.pending_count();
        (pending != 0).then(|| {
            format!(
                "{got} of {total} tasks ran (the rest were refused by the full queue), \
                 but {pending} stay pending forever"
            )
        })
    }));

    cases.push(Case::new("pooled drop-running", SHORT, || {
        let gate = Gate::new();
        let g = gate.clone();
        let mut runner: PooledTaskRunner<u32> = PooledTaskRunner::new(1);
        runner.spawn("forever", move || {
            g.wait();
            0
        });
        std::thread::sleep(Duration::from_millis(50));
        let start = Instant::now();
        drop(runner);
        gate.open();
        let took = start.elapsed();
        (took > Duration::from_secs(1)).then(|| format!("dropping the runner blocked for {took:?}"))
    }));

    cases.push(Case::new("pooled many-spawns", LIMIT, || {
        let mut runner: PooledTaskRunner<usize> = PooledTaskRunner::new(4);
        for i in 0..500 {
            runner.spawn(format!("t{i}"), move || i);
        }
        let mut got = 0;
        eventually(Duration::from_secs(8), || {
            while runner.poll().is_some() {
                got += 1;
            }
            got == 500
        });
        (got != 500 || runner.pending_count() != 0)
            .then(|| format!("{got}/500 results, {} pending", runner.pending_count()))
    }));
}

// ─── Workers ────────────────────────────────────────────────────────────────

fn worker_cases(cases: &mut Vec<Case>) {
    use revue::worker::{
        WorkerChannel, WorkerCommand, WorkerConfig, WorkerError, WorkerHandle, WorkerMessage,
        WorkerPool, WorkerState,
    };

    cases.push(Case::new("handle blocking-panic", LIMIT, || {
        let handle: WorkerHandle<u32> =
            WorkerHandle::spawn_blocking(|| quiet_panic("boom: blocking"));
        match handle.join_timeout(PROMPT) {
            Err(WorkerError::Panicked(m)) if m.contains("blocking") => None,
            other => Some(format!("expected Panicked, got {other:?}")),
        }
    }));

    // The future runs on tokio with `async`, on a polling loop without.
    let executor = if cfg!(feature = "async") {
        "tokio"
    } else {
        "polling"
    };
    cases.push(Case::new(format!("handle {executor}-panic"), LIMIT, || {
        let handle: WorkerHandle<u32> = WorkerHandle::spawn(async { quiet_panic("boom: async") });
        match handle.join_timeout(PROMPT) {
            Err(WorkerError::Panicked(m)) if m.contains("async") => None,
            other => Some(format!("expected Panicked, got {other:?}")),
        }
    }));

    cases.push(Case::new(
        format!("handle {executor}-cancel"),
        LIMIT,
        || {
            let handle: WorkerHandle<u32> = WorkerHandle::spawn(std::future::pending());
            eventually(PROMPT, || handle.state() == WorkerState::Running);
            handle.cancel();
            if !eventually(PROMPT, || handle.is_finished()) {
                return Some(format!(
                    "a cancelled future that never finishes is still {:?}",
                    handle.state()
                ));
            }
            match handle.join() {
                Err(WorkerError::Cancelled) => None,
                other => Some(format!("expected Cancelled, got {other:?}")),
            }
        },
    ));

    cases.push(Case::new("handle drop-running", SHORT, || {
        let gate = Gate::new();
        let g = gate.clone();
        let handle = WorkerHandle::spawn_blocking(move || g.wait());
        let start = Instant::now();
        drop(handle);
        gate.open();
        (start.elapsed() > Duration::from_secs(1)).then(|| "dropping a handle blocked".into())
    }));

    cases.push(Case::new("handle join-timeout", LIMIT, || {
        let gate = Gate::new();
        let g = gate.clone();
        let handle = WorkerHandle::spawn_blocking(move || g.wait());
        let start = Instant::now();
        let r = handle.join_timeout(Duration::from_millis(200));
        gate.open();
        if start.elapsed() > Duration::from_secs(2) {
            return Some("join_timeout overran its timeout".into());
        }
        match r {
            Err(WorkerError::Timeout) => None,
            other => Some(format!("expected Timeout, got {other:?}")),
        }
    }));

    cases.push(Case::new("pool task-panic", LIMIT, || {
        let pool = WorkerPool::new(1);
        pool.submit(|| quiet_panic("boom: pool task"));
        let ran = Arc::new(AtomicBool::new(false));
        let r = ran.clone();
        pool.submit(move || r.store(true, Ordering::SeqCst));
        if !eventually(PROMPT, || ran.load(Ordering::SeqCst)) {
            return Some(format!(
                "after a panicking task the pool runs nothing (active workers: {}, queued: {})",
                pool.active_workers(),
                pool.queue_len()
            ));
        }
        None
    }));

    cases.push(Case::new("pool full-queue", LIMIT, || {
        let gate = Gate::new();
        let pool = WorkerPool::with_config(WorkerConfig {
            threads: 1,
            queue_capacity: 4,
            default_timeout_ms: None,
        });
        let ran = Arc::new(AtomicUsize::new(0));
        let mut accepted = 0;
        for _ in 0..20 {
            let g = gate.clone();
            let r = ran.clone();
            if pool.submit(move || {
                g.wait();
                r.fetch_add(1, Ordering::SeqCst);
            }) {
                accepted += 1;
            }
        }
        gate.open();
        if accepted >= 20 {
            return Some("a full queue accepted every task".into());
        }
        if !eventually(PROMPT, || ran.load(Ordering::SeqCst) == accepted) {
            return Some(format!(
                "{} of {accepted} accepted tasks ran",
                ran.load(Ordering::SeqCst)
            ));
        }
        None
    }));

    cases.push(Case::new("pool drop-running", SHORT, || {
        let gate = Gate::new();
        let g = gate.clone();
        let pool = WorkerPool::new(1);
        pool.submit(move || g.wait());
        std::thread::sleep(Duration::from_millis(50));
        let start = Instant::now();
        drop(pool);
        gate.open();
        let took = start.elapsed();
        (took > Duration::from_secs(1))
            .then(|| format!("dropping the pool blocked for {took:?} on a running task"))
    }));

    cases.push(Case::new("pool after-shutdown", LIMIT, || {
        let pool = WorkerPool::new(2);
        pool.shutdown();
        pool.submit(|| ())
            .then(|| "a shut-down pool accepted a task".into())
    }));

    cases.push(Case::new("channel cancel-full-queue", LIMIT, || {
        let channel: WorkerChannel<u32> = WorkerChannel::with_capacity(2);
        let (worker, ui) = channel.split();
        ui.pause();
        ui.resume();
        let sent = ui.cancel();
        if !worker.is_cancelled() {
            return Some(format!(
                "cancel through a full command queue was lost (cancel() returned {sent})"
            ));
        }
        None
    }));

    cases.push(Case::new("channel cancel-zero-capacity", LIMIT, || {
        let channel: WorkerChannel<u32> = WorkerChannel::with_capacity(0);
        let (worker, ui) = channel.split();
        ui.cancel();
        (!worker.is_cancelled()).then(|| "a zero-capacity channel cannot be cancelled".into())
    }));

    cases.push(Case::new("channel cancel-consumed", LIMIT, || {
        let channel: WorkerChannel<u32> = WorkerChannel::new();
        let (worker, ui) = channel.split();
        ui.cancel();
        // The worker drains its commands, as a loop handling Pause/Resume does.
        while let Some(cmd) = worker.check_command() {
            let _ = matches!(cmd, WorkerCommand::Cancel);
        }
        (!worker.is_cancelled())
            .then(|| "is_cancelled() forgot the cancel once the command was read".into())
    }));

    cases.push(Case::new("channel receiver-dropped", LIMIT, || {
        let channel: WorkerChannel<u32> = WorkerChannel::with_capacity(16);
        let (worker, ui) = channel.split();
        drop(ui);
        drop(channel);
        let mut refused = false;
        for i in 0..100 {
            if !worker.progress(i as f32 / 100.0) {
                refused = true;
                break;
            }
        }
        (!refused).then(|| "unbounded sends to a dropped receiver".into())
    }));

    cases.push(Case::new("channel threads", LIMIT, || {
        let channel: WorkerChannel<usize> = WorkerChannel::with_capacity(1 << 20);
        let (worker, ui) = channel.split();
        let senders: Vec<_> = (0..4)
            .map(|t| {
                let w = worker.clone();
                std::thread::spawn(move || {
                    for i in 0..2000 {
                        w.partial(t * 10_000 + i);
                    }
                })
            })
            .collect();
        let mut got = 0;
        let start = Instant::now();
        while got < 8000 && start.elapsed() < PROMPT {
            got += ui.recv_all().len();
        }
        for s in senders {
            let _ = s.join();
        }
        got += ui.recv_all().len();
        let left = ui.message_count();
        (got != 8000 || left != 0).then(|| format!("received {got}/8000, count says {left} left"))
    }));

    let _ = WorkerMessage::<u32>::Progress(0.0);
}

// ─── Locks ──────────────────────────────────────────────────────────────────

fn lock_cases(cases: &mut Vec<Case>) {
    use revue::reactive::{signal, signal_vec, Computed};

    cases.push(Case::new("lock signal-update-panics", LIMIT, || {
        let s = signal(1);
        let seen = Arc::new(AtomicUsize::new(0));
        let n = seen.clone();
        let _sub = s.subscribe(move || {
            n.fetch_add(1, Ordering::SeqCst);
        });
        panics(|| s.update(|_| panic!("boom: inside update")))?;
        s.set(5);
        let v = s.get();
        (v != 5 || seen.load(Ordering::SeqCst) == 0).then(|| {
            format!(
                "after a poisoned update: value {v}, notified {}",
                seen.load(Ordering::SeqCst)
            )
        })
    }));

    cases.push(Case::new("lock computed-panics", LIMIT, || {
        let input = signal(0);
        let i = input.clone();
        let c = Computed::new(move || {
            if i.get() == 0 {
                panic!("boom: compute");
            }
            i.get() * 2
        });
        panics(|| {
            c.get();
        })?;
        input.set(4);
        let v = panics(|| {
            assert_eq!(c.get(), 8);
        });
        v.map(|p| format!("after a panicking compute: {p}"))
    }));

    cases.push(Case::new(
        "lock signal-vec-subscriber-panics",
        LIMIT,
        || {
            let v = signal_vec(vec![1]);
            let bad = v.subscribe_diff(|_| panic!("boom: diff subscriber"));
            panics(|| v.push(2))?;
            drop(bad);
            let seen = Arc::new(AtomicUsize::new(0));
            let n = seen.clone();
            let _good = v.subscribe_diff(move |_| {
                n.fetch_add(1, Ordering::SeqCst);
            });
            let _ = panics(|| v.push(3));
            (seen.load(Ordering::SeqCst) == 0)
                .then(|| "after a subscriber panicked, no subscriber is ever notified again".into())
        },
    ));
}

// ─── Reactive ───────────────────────────────────────────────────────────────

/// Cases that may overflow the stack: run in a child process. Each returns
/// `None` when the loop ended in the documented panic.
type ChildCheck = fn() -> Option<String>;

const CHILD_CASES: &[(&str, ChildCheck)] = &[
    ("reactive subscriber-loop", || {
        use revue::reactive::signal;
        let s = signal(0u64);
        let s2 = s.clone();
        let _sub = s.subscribe(move || s2.set(s2.get() + 1));
        match panics(|| s.set(1)) {
            Some(m) if m.contains("depth") => None,
            Some(m) => Some(format!("unexpected panic: {m}")),
            None => Some("a subscriber setting its own signal forever ended".into()),
        }
    }),
    ("reactive effect-loop", || {
        use revue::reactive::{effect, is_tracking, signal};
        let s = signal(0u64);
        let s2 = s.clone();
        let p = panics(|| {
            let _e = effect(move || s2.set(s2.get() + 1));
        });
        let tracking = is_tracking();
        match p {
            Some(m) if m.contains("depth") && !tracking => None,
            Some(m) if m.contains("depth") => {
                Some("after the update-depth panic the tracker still tracks".into())
            }
            Some(m) => Some(format!("unexpected panic: {m}")),
            None => Some("an effect setting its own signal forever ended".into()),
        }
    }),
];

fn reactive_cases(cases: &mut Vec<Case>) {
    use revue::event::custom::{CustomEvent, EventDispatcher, EventResponse};
    use revue::reactive::{
        batch, effect, end_batch, is_batching, is_tracking, queue_update, signal, signal_vec,
        start_batch, use_async, AsyncState, Computed, Subscription,
    };

    for &(name, _) in CHILD_CASES {
        cases.push(Case::new(name, Duration::from_secs(30), move || {
            run_in_child(
                "fault::concurrency::concurrency_faults",
                &[(CHILD, std::ffi::OsStr::new(name))],
                Duration::from_secs(25),
            )
        }));
    }

    cases.push(Case::new("reactive subscriber-converges", LIMIT, || {
        let s = signal(0);
        let s2 = s.clone();
        let _sub = s.subscribe(move || {
            if s2.get() < 10 {
                s2.set(s2.get() + 1);
            }
        });
        s.set(1);
        (s.get() != 10).then(|| format!("settled at {}", s.get()))
    }));

    cases.push(Case::new("reactive effect-converges", LIMIT, || {
        let s = signal(0);
        let s2 = s.clone();
        let _e = effect(move || {
            if s2.get() < 10 {
                s2.set(s2.get() + 1);
            }
        });
        (s.get() != 10).then(|| format!("settled at {}", s.get()))
    }));

    cases.push(Case::new("reactive effect-panics", LIMIT, || {
        let s = signal(0);
        let s2 = s.clone();
        let runs = Arc::new(AtomicUsize::new(0));
        let r = runs.clone();
        let _ = panics(move || {
            let e = effect(move || {
                r.fetch_add(1, Ordering::SeqCst);
                if s2.get() == 0 {
                    panic!("boom: effect");
                }
            });
            std::mem::forget(e);
        });
        if is_tracking() {
            return Some("a panicking effect left the tracker tracking".into());
        }
        // A signal read and set outside any effect must not re-run it.
        let other = signal(0);
        let _ = other.get();
        let before = runs.load(Ordering::SeqCst);
        other.set(1);
        let after = runs.load(Ordering::SeqCst);
        (after != before).then(|| "an unrelated signal re-ran the panicked effect".into())
    }));

    cases.push(Case::new(
        "reactive drop-subscription-in-callback",
        LIMIT,
        || {
            let s = signal(0);
            let slot: Arc<Mutex<Option<Subscription>>> = Arc::new(Mutex::new(None));
            let other: Arc<Mutex<Option<Subscription>>> = Arc::new(Mutex::new(None));
            let (sl, ot) = (slot.clone(), other.clone());
            let sub = s.subscribe(move || {
                sl.lock().unwrap().take();
                ot.lock().unwrap().take();
            });
            *slot.lock().unwrap() = Some(sub);
            *other.lock().unwrap() = Some(s.subscribe(|| {}));
            s.set(1);
            s.set(2);
            None
        },
    ));

    cases.push(Case::new(
        "reactive vec-drop-subscription-in-callback",
        LIMIT,
        || {
            let v = signal_vec(vec![0]);
            type Slot = Arc<Mutex<Option<revue::reactive::VecSubscription<i32>>>>;
            let slot: Slot = Arc::new(Mutex::new(None));
            let sl = slot.clone();
            let sub = v.subscribe_diff(move |_| {
                sl.lock().unwrap().take();
            });
            *slot.lock().unwrap() = Some(sub);
            v.push(1);
            v.push(2);
            None
        },
    ));

    cases.push(Case::new(
        "reactive vec-push-from-subscriber",
        LIMIT,
        || {
            let v = signal_vec(vec![0]);
            let v2 = v.clone();
            let _sub = v.subscribe_diff(move |_| {
                if v2.len() < 5 {
                    v2.push(9);
                }
            });
            v.push(1);
            (v.len() != 5).then(|| format!("settled at {} items", v.len()))
        },
    ));

    cases.push(Case::new("reactive computed-cycle", SHORT, || {
        let b_cell: Arc<OnceLock<Computed<i32>>> = Arc::new(OnceLock::new());
        let bc = b_cell.clone();
        let a = Computed::new(move || bc.get().map_or(0, |b| b.get()) + 1);
        let a2 = a.clone();
        let _ = b_cell.set(Computed::new(move || a2.get() + 1));
        match panics(|| {
            a.get();
        }) {
            Some(_) => None,
            None => Some("a cyclic computed returned a value".into()),
        }
    }));

    cases.push(Case::new("reactive computed-self", SHORT, || {
        let cell: Arc<OnceLock<Computed<i32>>> = Arc::new(OnceLock::new());
        let c2 = cell.clone();
        let c = Computed::new(move || c2.get().map_or(0, |c| c.get()) + 1);
        let _ = cell.set(c.clone());
        match panics(|| {
            c.get();
        }) {
            Some(_) => None,
            None => Some("a self-referential computed returned a value".into()),
        }
    }));

    cases.push(Case::new("reactive batch-queue-in-flush", LIMIT, || {
        let ran = Arc::new(AtomicUsize::new(0));
        let r = ran.clone();
        let p = panics(|| {
            batch(|| {
                queue_update(move || {
                    let r2 = r.clone();
                    queue_update(move || {
                        r2.fetch_add(1, Ordering::SeqCst);
                    });
                });
            });
        });
        if let Some(p) = p {
            return Some(format!("an update queued while flushing panicked: {p}"));
        }
        (ran.load(Ordering::SeqCst) != 1).then(|| "the nested update did not run".into())
    }));

    cases.push(Case::new("reactive batch-panics", LIMIT, || {
        let _ = panics(|| batch(|| panic!("boom: in batch")));
        if is_batching() {
            // Leave the thread clean for the record.
            end_batch();
            return Some("a panicking batch left the thread batching forever".into());
        }
        let _ = start_batch;
        None
    }));

    cases.push(Case::new("reactive use-async-stale", LIMIT, || {
        let gate = Gate::new();
        let calls = Arc::new(AtomicUsize::new(0));
        let (g, c) = (gate.clone(), calls.clone());
        let (state, trigger) = use_async(move || {
            let n = c.fetch_add(1, Ordering::SeqCst) + 1;
            if n == 1 {
                g.wait();
            }
            Ok(n)
        });
        trigger();
        eventually(PROMPT, || calls.load(Ordering::SeqCst) == 1);
        trigger();
        if !eventually(PROMPT, || matches!(state.get(), AsyncState::Ready(2))) {
            gate.open();
            return Some(format!(
                "the second trigger never landed: {:?}",
                state.get()
            ));
        }
        gate.open();
        std::thread::sleep(Duration::from_millis(300));
        match state.get() {
            AsyncState::Ready(2) => None,
            other => Some(format!(
                "the first, superseded run overwrote the newer result: {other:?}"
            )),
        }
    }));

    cases.push(Case::new("reactive use-async-panic", LIMIT, || {
        let (state, trigger) = use_async(|| -> Result<u32, String> { quiet_panic("boom") });
        trigger();
        (!eventually(PROMPT, || state.get().is_error()))
            .then(|| format!("a panicking task left {:?}", state.get()))
    }));

    #[derive(Clone)]
    struct Ping;
    impl CustomEvent for Ping {
        fn event_type() -> &'static str {
            "fault-ping"
        }
    }

    cases.push(Case::new("dispatch on-in-handler", SHORT, || {
        let dispatcher = EventDispatcher::new();
        let inner = Arc::new(Mutex::new(dispatcher.clone()));
        let mut outer = dispatcher.clone();
        let i = inner.clone();
        outer.on::<Ping>(move |_, _| {
            i.lock().unwrap().on::<Ping>(|_, _| EventResponse::Handled);
            EventResponse::Handled
        });
        dispatcher.dispatch(Ping);
        let n = dispatcher.handler_count::<Ping>();
        (n != 2).then(|| format!("{n} handlers after one registered another"))
    }));

    cases.push(Case::new("dispatch reentrant", SHORT, || {
        let dispatcher = EventDispatcher::new();
        let depth = Arc::new(AtomicUsize::new(0));
        let (d, inner) = (depth.clone(), dispatcher.clone());
        let mut outer = dispatcher.clone();
        outer.on::<Ping>(move |_, _| {
            if d.fetch_add(1, Ordering::SeqCst) < 3 {
                inner.dispatch(Ping);
            }
            EventResponse::Handled
        });
        dispatcher.dispatch(Ping);
        (depth.load(Ordering::SeqCst) != 4)
            .then(|| format!("handler ran {} times", depth.load(Ordering::SeqCst)))
    }));
}

// ─── Plugins ────────────────────────────────────────────────────────────────

/// What a probe plugin does in a hook.
#[derive(Clone, Copy, PartialEq)]
enum Act {
    Ok,
    Fail,
    Panic,
}

struct Probe {
    name: &'static str,
    priority: i32,
    hook: &'static str,
    act: Act,
    log: Arc<Mutex<Vec<String>>>,
}

impl Probe {
    fn run(&mut self, hook: &str) -> revue::Result<()> {
        self.log
            .lock()
            .unwrap()
            .push(format!("{}:{hook}", self.name));
        if hook == self.hook {
            match self.act {
                Act::Ok => {}
                Act::Fail => return Err(revue::Error::Render(format!("{hook} failed"))),
                Act::Panic => quiet_panic("boom: plugin"),
            }
        }
        Ok(())
    }
}

impl revue::plugin::Plugin for Probe {
    fn name(&self) -> &str {
        self.name
    }
    fn priority(&self) -> i32 {
        self.priority
    }
    fn on_init(&mut self, _: &mut revue::plugin::PluginContext) -> revue::Result<()> {
        self.run("init")
    }
    fn on_mount(&mut self, _: &mut revue::plugin::PluginContext) -> revue::Result<()> {
        self.run("mount")
    }
    fn on_tick(&mut self, _: &mut revue::plugin::PluginContext, _: Duration) -> revue::Result<()> {
        self.run("tick")
    }
    fn on_unmount(&mut self, _: &mut revue::plugin::PluginContext) -> revue::Result<()> {
        self.run("unmount")
    }
}

/// A registry of three probes - `a` (first), `bad` (second, which does `act`
/// in `hook`) and `c` (last) - and their shared call log.
fn registry(
    hook: &'static str,
    act: Act,
) -> (revue::plugin::PluginRegistry, Arc<Mutex<Vec<String>>>) {
    let log = Arc::new(Mutex::new(Vec::new()));
    let mut reg = revue::plugin::PluginRegistry::new();
    for (name, priority, a) in [("a", 10, Act::Ok), ("bad", 5, act), ("c", 0, Act::Ok)] {
        reg.register(Probe {
            name,
            priority,
            hook,
            act: a,
            log: log.clone(),
        });
    }
    (reg, log)
}

fn count(log: &Mutex<Vec<String>>, entry: &str) -> usize {
    log.lock().unwrap().iter().filter(|e| *e == entry).count()
}

fn plugin_cases(cases: &mut Vec<Case>) {
    cases.push(Case::new("plugin init-error", LIMIT, || {
        let (mut reg, log) = registry("init", Act::Fail);
        if reg.init().is_ok() {
            return Some("a failing init returned Ok".into());
        }
        // Retried, as an app may: nothing is initialized twice.
        let _ = reg.init();
        let twice = count(&log, "a:init");
        (twice > 1).then(|| format!("plugin `a` was initialized {twice} times"))
    }));

    cases.push(Case::new("plugin mount-error", LIMIT, || {
        let (mut reg, log) = registry("mount", Act::Fail);
        let _ = reg.init();
        if reg.mount().is_ok() {
            return Some("a failing mount returned Ok".into());
        }
        let _ = reg.unmount();
        // `a` mounted before `bad` failed: it must be unmounted.
        (count(&log, "a:mount") == 1 && count(&log, "a:unmount") == 0)
            .then(|| "a plugin mounted before another's mount failed is never unmounted".into())
    }));

    cases.push(Case::new("plugin tick-error", LIMIT, || {
        let (mut reg, log) = registry("tick", Act::Fail);
        let _ = reg.init();
        let _ = reg.mount();
        let _ = reg.tick(Duration::from_millis(16));
        (count(&log, "c:tick") != 1).then(|| "a failing tick stopped the plugins after it".into())
    }));

    cases.push(Case::new("plugin unmount-error", LIMIT, || {
        let (mut reg, log) = registry("unmount", Act::Fail);
        let _ = reg.init();
        let _ = reg.mount();
        let _ = reg.unmount();
        (count(&log, "a:unmount") != 1)
            .then(|| "a failing unmount stopped the plugins after it".into())
    }));

    // A panicking hook disables its plugin: the panic is caught, the hook
    // reports an error, the plugin gets no more hooks, and every other plugin
    // still runs its whole lifecycle.
    for hook in ["init", "mount", "tick", "unmount"] {
        cases.push(Case::new(
            format!("plugin {hook}-panic"),
            LIMIT,
            move || {
                let (mut reg, log) = registry(hook, Act::Panic);
                let mut errors = Vec::new();
                let p = panics(|| {
                    errors.push(("init", reg.init().is_err()));
                    errors.push(("mount", reg.mount().is_err()));
                    errors.push(("tick", reg.tick(Duration::from_millis(16)).is_err()));
                    errors.push(("tick", reg.tick(Duration::from_millis(16)).is_err()));
                    errors.push(("unmount", reg.unmount().is_err()));
                });
                if let Some(m) = p {
                    return Some(format!("a panic in on_{hook} escaped the registry: {m}"));
                }
                if !errors.iter().any(|&(h, err)| h == hook && err) {
                    return Some(format!("a panic in on_{hook} was not reported as an error"));
                }
                if let Some(&(h, _)) = errors.iter().find(|&&(h, err)| h != hook && err) {
                    return Some(format!("{h} failed although only on_{hook} panicked"));
                }
                if reg.disabled_plugins() != ["bad"] {
                    return Some(format!(
                        "disabled plugins are {:?}, not [\"bad\"]",
                        reg.disabled_plugins()
                    ));
                }
                let log = log.lock().unwrap().clone();
                let after = log
                    .iter()
                    .skip_while(|e| *e != &format!("bad:{hook}"))
                    .skip(1)
                    .find(|e| e.starts_with("bad:"));
                if let Some(e) = after {
                    return Some(format!("the disabled plugin still got `{e}`"));
                }
                for h in ["init", "mount", "tick", "unmount"] {
                    let want = if h == "tick" { 2 } else { 1 };
                    for name in ["a", "c"] {
                        let got = log.iter().filter(|e| **e == format!("{name}:{h}")).count();
                        if got != want {
                            return Some(format!(
                                "plugin `{name}` ran on_{h} {got} times, not {want}, after `bad` panicked in on_{hook}"
                            ));
                        }
                    }
                }
                None
            },
        ));
    }
}

// ─── The layer ──────────────────────────────────────────────────────────────

fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    task_cases(&mut cases);
    worker_cases(&mut cases);
    lock_cases(&mut cases);
    reactive_cases(&mut cases);
    plugin_cases(&mut cases);
    cases
}

#[test]
fn concurrency_faults() {
    if let Ok(name) = std::env::var(CHILD) {
        let (_, check) = CHILD_CASES
            .iter()
            .find(|(n, _)| *n == name)
            .expect("known child case");
        // Run on a thread with a small stack: an unbounded loop overflows
        // quickly instead of after gigabytes.
        let check = *check;
        let failure = std::thread::Builder::new()
            .stack_size(4 << 20)
            .spawn(check)
            .expect("spawn")
            .join()
            .unwrap_or_else(|_| Some("the check itself panicked".into()));
        if let Some(failure) = failure {
            panic!("CHILD: {failure}");
        }
        return;
    }

    let (ran, failures) = run_bounded(cases());
    ratchet(LAYER, &ran, failures);
}
