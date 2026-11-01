//! Where events come from, and when `Tick` arrives

use super::Event;
use crate::Result;
use std::sync::mpsc::{Receiver, RecvTimeoutError};
use std::time::{Duration, Instant};

/// What an [`EventSource`] has after waiting
#[derive(Debug, Clone, PartialEq)]
pub enum Polled {
    /// An event arrived
    Event(Event),
    /// The time was up with no event
    Timeout,
    /// No more events will come: the loop ends
    Closed,
}

/// Events for [`App::run_on`](crate::core::app::App::run_on), from something
/// other than the process terminal, such as one SSH session's channel
pub trait EventSource {
    /// Wait up to `timeout` for the next event
    ///
    /// # Errors
    ///
    /// Returns an error when reading fails; it ends the loop.
    fn poll_event(&mut self, timeout: Duration) -> Result<Polled>;
}

/// A channel is a source: dropping every sender closes it
impl EventSource for Receiver<Event> {
    fn poll_event(&mut self, timeout: Duration) -> Result<Polled> {
        Ok(match self.recv_timeout(timeout) {
            Ok(event) => Polled::Event(event),
            Err(RecvTimeoutError::Timeout) => Polled::Timeout,
            Err(RecvTimeoutError::Disconnected) => Polled::Closed,
        })
    }
}

/// When the next `Tick` is due
///
/// The time counts from the last `Tick`, not from the last input, so input
/// that never stops cannot hold ticks back (#1195).
#[derive(Debug, Clone, Copy)]
pub(crate) struct TickSchedule {
    rate: Duration,
    next: Instant,
}

impl TickSchedule {
    pub(crate) fn new(rate: Duration) -> Self {
        Self {
            rate,
            next: crate::utils::clock::now() + rate,
        }
    }

    #[cfg(test)]
    pub(crate) fn rate(&self) -> Duration {
        self.rate
    }

    /// The time left to wait for input, or `None` when a `Tick` is due
    pub(crate) fn wait(&self) -> Option<Duration> {
        let left = self
            .next
            .saturating_duration_since(crate::utils::clock::now());
        (!left.is_zero()).then_some(left)
    }

    /// A `Tick` is going out: the next one is a whole rate from now
    pub(crate) fn ticked(&mut self) {
        self.next = crate::utils::clock::now() + self.rate;
    }

    /// The next event from `source`, or `None` once it closes
    pub(crate) fn next_event(&mut self, source: &mut impl EventSource) -> Result<Option<Event>> {
        let polled = match self.wait() {
            Some(left) => source.poll_event(left)?,
            None => Polled::Timeout,
        };
        Ok(match polled {
            Polled::Event(event) => Some(event),
            Polled::Timeout => {
                self.ticked();
                Some(Event::Tick)
            }
            Polled::Closed => None,
        })
    }
}
