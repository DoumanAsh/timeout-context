//! Time module

#[cfg(feature = "std")]
extern crate std;

use core::fmt;
pub use core::time::Duration;

#[cfg(feature = "std")]
///Alias to standard library time
pub type StdInstant = std::time::Instant;
#[cfg(feature = "tokio")]
///Alias to tokio library time
pub type TokioInstant = tokio::time::Instant;

///Describes generic Instant
pub trait Instant: Sized {
    ///Returns current [Instant]
    fn now() -> Self;
    ///Returns interval elapsed since creation of the [Instant]
    fn elapsed(&self) -> Duration;
    ///Adds interval to self, returning new [Instant] if it is within bounds of underlying integer representation
    fn checked_add(&self, time: core::time::Duration) -> Option<Self>;
}

#[cfg(feature = "tokio")]
impl Instant for tokio::time::Instant {
    #[inline(always)]
    fn now() -> Self {
        Self::now()
    }
    #[inline(always)]
    fn checked_add(&self, time: Duration) -> Option<Self> {
        self.checked_add(time)
    }
    #[inline(always)]
    fn elapsed(&self) -> Duration {
        self.elapsed()
    }
}

#[cfg(feature = "std")]
impl Instant for std::time::Instant {
    #[inline(always)]
    fn now() -> Self {
        Self::now()
    }
    #[inline(always)]
    fn checked_add(&self, time: Duration) -> Option<Self> {
        self.checked_add(time)
    }
    #[inline(always)]
    fn elapsed(&self) -> Duration {
        self.elapsed()
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
///Timeout interval formatter
pub struct TimeoutValue(pub Duration);

impl fmt::Debug for TimeoutValue {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, fmt)
    }
}

impl fmt::Display for TimeoutValue {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_fmt(format_args!("{}m", self.0.as_millis()))
    }
}
