//! Clean shutdown on termination signals.
//!
//! In raw mode the terminal does not turn Ctrl+C into `SIGINT`: it arrives as an
//! ordinary key event, and [`App`](super::App) handles it like any other quit
//! key. A process can still be told to stop from outside, though - `kill` sends
//! `SIGTERM`, closing the terminal window sends `SIGHUP`, `kill -INT` sends
//! `SIGINT`. The default action for all three kills the process on the spot: no
//! plugin `on_unmount`, no `Drop`, and the shell is left in raw mode on the
//! alternate screen.
//!
//! While [`App::run`](super::App::run) is running, these signals instead set a
//! flag that the event loop checks every frame. The loop then stops exactly as
//! [`App::quit`](super::App::quit) stops it: plugins unmount, the terminal is
//! restored, and `run` returns `Ok(())`.
//!
//! - A second signal while that shutdown is pending runs the signal's default
//!   action, so an app wedged in its shutdown can still be killed.
//! - Outside `App::run` the default action applies, as if revue were not there.
//! - A signal that is ignored when revue first sees it (`nohup` ignores
//!   `SIGHUP`) stays ignored. A handler another library installed first keeps
//!   running; revue then only adds its shutdown request and never runs the
//!   default action on that handler's behalf.
//!
//! Only unix has these signals. On Windows `ShutdownSignals` never reports a
//! request: Ctrl+C still arrives as a key event, but Ctrl+Break and closing the
//! console window end the process without a clean shutdown (the panic hook does
//! not run either - nothing panics).

/// Listens for termination signals for the duration of one `App::run`.
///
/// Dropping it hands the signals back to their default behavior.
pub(crate) struct ShutdownSignals {
    _session: (),
}

#[cfg(unix)]
mod imp {
    use std::io;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Arc, OnceLock};

    use signal_hook::consts::{SIGHUP, SIGINT, SIGTERM};
    use signal_hook::flag;

    /// The signals that ask a process to stop.
    pub(super) const SIGNALS: [libc::c_int; 3] = [SIGTERM, SIGHUP, SIGINT];

    pub(super) struct Flags {
        /// No `App::run` is listening: behave as if revue were not here.
        pub(super) idle: Arc<AtomicBool>,
        /// A signal arrived while `App::run` was listening.
        pub(super) requested: Arc<AtomicBool>,
    }

    /// The process-wide flags, with the signal actions registered on first use.
    ///
    /// Actions are registered once and never removed: `signal-hook` cannot put
    /// the default disposition back, so unregistering would leave the signals
    /// ignored. The `idle` flag does that job instead.
    pub(super) fn flags() -> &'static Flags {
        static FLAGS: OnceLock<Flags> = OnceLock::new();
        FLAGS.get_or_init(|| {
            let flags = Flags {
                idle: Arc::new(AtomicBool::new(true)),
                requested: Arc::new(AtomicBool::new(false)),
            };
            for signal in SIGNALS {
                if let Err(e) = register(signal, &flags) {
                    crate::log_warn!("Cannot listen for signal {}: {}", signal, e);
                }
            }
            flags
        })
    }

    fn register(signal: libc::c_int, flags: &Flags) -> io::Result<()> {
        match disposition(signal) {
            Disposition::Ignored => return Ok(()),
            Disposition::Default => {
                // signal-hook runs actions in registration order, so these two
                // see the flags as they were before this delivery.
                flag::register_conditional_default(signal, Arc::clone(&flags.idle))?;
                flag::register_conditional_default(signal, Arc::clone(&flags.requested))?;
            }
            Disposition::Handled => {}
        }
        flag::register(signal, Arc::clone(&flags.requested))?;
        Ok(())
    }

    pub(super) enum Disposition {
        Default,
        Ignored,
        /// Someone else's handler, or a disposition we could not read.
        Handled,
    }

    pub(super) fn disposition(signal: libc::c_int) -> Disposition {
        // SAFETY: with a null `act`, `sigaction` only reads the current
        // disposition into `old`, a valid, writable `sigaction`. All-zero
        // bytes are a valid `sigaction` value (plain integers and a mask).
        let (rc, old) = unsafe {
            let mut old: libc::sigaction = std::mem::zeroed();
            let rc = libc::sigaction(signal, std::ptr::null(), &mut old);
            (rc, old)
        };
        if rc != 0 {
            Disposition::Handled
        } else if old.sa_sigaction == libc::SIG_DFL {
            Disposition::Default
        } else if old.sa_sigaction == libc::SIG_IGN {
            Disposition::Ignored
        } else {
            Disposition::Handled
        }
    }

    pub(super) fn listen() {
        let flags = flags();
        // Forget a request left over from an earlier run before arming.
        flags.requested.store(false, Ordering::SeqCst);
        flags.idle.store(false, Ordering::SeqCst);
    }

    pub(super) fn requested() -> bool {
        flags().requested.load(Ordering::SeqCst)
    }

    pub(super) fn stop_listening() {
        flags().idle.store(true, Ordering::SeqCst);
    }
}

impl ShutdownSignals {
    /// Start treating termination signals as a request to quit.
    pub(crate) fn listen() -> Self {
        #[cfg(unix)]
        imp::listen();
        Self { _session: () }
    }

    /// Has a termination signal arrived since [`listen`](Self::listen)?
    pub(crate) fn requested(&self) -> bool {
        #[cfg(unix)]
        {
            imp::requested()
        }
        #[cfg(not(unix))]
        {
            false
        }
    }
}

impl Drop for ShutdownSignals {
    fn drop(&mut self) {
        #[cfg(unix)]
        imp::stop_listening();
    }
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use serial_test::serial;
    use signal_hook::low_level::raise;

    // Raising a signal outside a listening session would run its default
    // action and kill the test process, so every `raise` below happens while
    // a `ShutdownSignals` is alive.

    #[test]
    #[serial]
    fn each_termination_signal_requests_shutdown() {
        for signal in imp::SIGNALS {
            let signals = ShutdownSignals::listen();
            assert!(!signals.requested());
            // A signal this process inherited as ignored stays ignored - a
            // background job of a non-interactive shell starts with SIGINT
            // ignored, for one. Nothing is registered for it, so it is still
            // `SIG_IGN` here, and raising it must not request a shutdown.
            let inherited_ignore = matches!(imp::disposition(signal), imp::Disposition::Ignored);

            raise(signal).unwrap();

            if inherited_ignore {
                assert!(
                    !signals.requested(),
                    "ignored signal {signal} requested shutdown"
                );
            } else {
                assert!(signals.requested(), "signal {signal} was not noticed");
            }
        }
    }

    #[test]
    #[serial]
    fn a_new_session_forgets_an_old_request() {
        {
            let signals = ShutdownSignals::listen();
            raise(libc::SIGTERM).unwrap();
            assert!(signals.requested());
        }

        let signals = ShutdownSignals::listen();
        assert!(!signals.requested());
    }

    #[test]
    #[serial]
    fn dropping_the_session_hands_signals_back_to_their_default() {
        drop(ShutdownSignals::listen());
        assert!(imp::flags().idle.load(std::sync::atomic::Ordering::SeqCst));
    }
}
