//!Contextual timeout utilities to handle timeouts across your applications
//!
//!This crate provides simple utility to provide uniform timeout handling for requests, and propagation utilities.
//!It uses [grpc semantics](https://github.com/grpc/grpc/blob/master/doc/PROTOCOL-HTTP2.md) to parse timeout value

#![no_std]
#![warn(missing_docs)]
#![allow(clippy::style)]

use core::{fmt, str, slice};

pub mod time;
#[cfg(feature = "bytes")]
mod bytes;
#[cfg(feature = "http")]
mod http;
#[cfg(feature = "tonic014")]
mod tonic014;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
///Possible parsing errors
pub enum ParseError<'a> {
    ///Input is not valid utf-8
    InvalidUtf8,
    ///No timeout value is found
    MissingValue,
    ///Indicates unit within timeout description is invalid
    InvalidUnit(char),
    ///Indicates value of the timeout is not valid integer.
    InvalidValue(&'a str),
}

impl<'a> ParseError<'a> {
    #[cold]
    #[inline(never)]
    const fn invalid_utf8() -> Self {
        Self::InvalidUtf8
    }
}

///Parses timeout expression, returning duration on success, otherwise error.
///
///Follows grpc semantics on textual representation of timeout: <https://github.com/grpc/grpc/blob/master/doc/PROTOCOL-HTTP2.md>
pub const fn parse_timeout<'a>(text: &'a [u8]) -> Result<time::Duration, ParseError<'a>> {
    const HOUR_SECONDS: u64 = 60 * 60;

    let split_idx = text.len().saturating_sub(1);
    let value = str::from_utf8(
        unsafe {
            slice::from_raw_parts(text.as_ptr(), split_idx)
        }
    );

    let value = match value {
        Ok(value) => value,
        Err(_) => return Err(ParseError::invalid_utf8())
    };
    if value.is_empty() {
        return Err(ParseError::MissingValue);
    }
    let value = match u64::from_str_radix(value, 10) {
        Ok(value) => value,
        Err(_) => return Err(ParseError::InvalidValue(value)),
    };
    let result = match text[split_idx] {
        //hours
        b'H' => time::Duration::from_secs(value.saturating_mul(HOUR_SECONDS)),
        //minutes
        b'M' => time::Duration::from_secs(value.saturating_mul(60)),
        //seconds
        b'S' => time::Duration::from_secs(value),
        //millis
        b'm' => time::Duration::from_millis(value),
        //micros
        b'u' => time::Duration::from_micros(value),
        //nanos
        b'n' => time::Duration::from_nanos(value),
        unknown => return Err(ParseError::InvalidUnit(unknown as _)),
    };

    Ok(result)
}


///Interface to propagate [Timeout]
pub trait TimeoutPropagation {
    ///Provides way to insert `value` under specified `key` which can be header name or whatever destination supports
    fn set_timeout(&mut self, key: &str, value: time::Duration);
    ///Provides access to the raw bytes under `key`, which expected to be valid utf-8 string if it is timeout header.
    fn get_header_value(&self, key: &str) -> Option<&[u8]>;
    #[inline]
    ///Access header value via [TimeoutPropagation::get_header_value] and attempts to parse it returning timeout value on success
    fn get_timeout(&self, key: &str) -> Option<time::Duration> {
        self.get_header_value(key).and_then(|value| parse_timeout(value).ok())
    }
}

impl TimeoutPropagation for &mut dyn TimeoutPropagation {
    #[inline]
    fn set_timeout(&mut self, key: &str, value: time::Duration) {
        TimeoutPropagation::set_timeout(&mut **self, key, value)
    }

    #[inline]
    fn get_header_value(&self, key: &str) -> Option<&[u8]> {
        TimeoutPropagation::get_header_value(&**self, key)
    }
}

#[derive(Copy, Clone)]
///Represents ongoing timeout
pub struct Timeout<I> {
    started_at: I,
    timeout: time::Duration,
}

impl<I: time::Instant> Timeout<I> {
    #[inline]
    ///Creates new timeout with at `started_at` time, expiring after `timeout`
    pub const fn new(started_at: I, timeout: time::Duration) -> Self {
        Self {
            started_at,
            timeout,
        }
    }

    #[inline]
    ///Returns timeout value
    pub const fn timeout(&self) -> time::Duration {
        self.timeout
    }

    #[inline]
    ///Returns remaining time
    pub fn get_remaining_timeout(&self) -> time::Duration {
        self.timeout.saturating_sub(self.started_at.elapsed())
    }
}

#[cfg(feature = "std")]
impl Timeout<time::StdInstant> {
    ///Creates new timeout using [std time](https://doc.rust-lang.org/std/time/struct.Instant.html)
    pub fn new_std(timeout: time::Duration) -> Self {
        Self::new(time::StdInstant::now(), timeout)
    }
}

#[cfg(feature = "tokio")]
impl Timeout<time::TokioInstant> {
    ///Creates new timeout using [std time](https://doc.rust-lang.org/std/time/struct.Instant.html)
    pub fn new_tokio(timeout: time::Duration) -> Self {
        Self::new(time::TokioInstant::now(), timeout)
    }
}

impl<I: time::Instant> fmt::Display for Timeout<I> {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let timeout = self.timeout.saturating_sub(self.started_at.elapsed());
        fmt.write_fmt(format_args!("{}m", timeout.as_millis()))
    }
}

impl<I: time::Instant> From<time::Duration> for Timeout<I> {
    #[inline(always)]
    fn from(value: time::Duration) -> Self {
        Self::new(I::now(), value)
    }
}
