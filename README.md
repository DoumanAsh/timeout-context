# timeout-context

[![Rust](https://github.com/DoumanAsh/timeout-context/actions/workflows/rust.yml/badge.svg)](https://github.com/DoumanAsh/timeout-context/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/timeout-context.svg)](https://crates.io/crates/timeout-context)
[![Documentation](https://docs.rs/timeout-context/badge.svg)](https://docs.rs/crate/timeout-context/)

Contextual timeout utilities to handle timeouts across your applications

This crate provides simple utility to provide uniform timeout handling for requests, and propagation utilities.
It uses [grpc semantics](https://github.com/grpc/grpc/blob/master/doc/PROTOCOL-HTTP2.md) to parse timeout value

## Requirements

- Rust 1.85

## Features:

- `std` - Enables use of [std time](https://doc.rust-lang.org/std/time/struct.Instant.html).
- `tokio` - Enables use of tokio runtime to execute tasks and access time.
- `http` - Enables [TimeoutPropagation](https://docs.rs/timeout-context/latest/timeout_context/trait.TimeoutPropagation.html) for [http](https://crates.io/crates/http) header map and request types
- `tonic014` - Enables [TimeoutPropagation](https://docs.rs/timeout-context/latest/timeout_context/trait.TimeoutPropagation.html) for [tonic 0.14](https://crates.io/crates/tonic/0.14.9) metadata map and request types

## Usage

Example of getting gRPC timeout to process your request with the help of [context](https://docs.rs/timeout-context/latest/timeout_context/struct.Timeout.html)

```rust
use timeout_context::{time, Timeout, TimeoutPropagation, GRPC_HEADER_NAME};

async fn handle_request<I: time::Instant>(source: &impl TimeoutPropagation, out: &mut impl TimeoutPropagation) {
    let timeout = Timeout::new_grpc_from(I::now(), source, time::Duration::from_secs(10));

    //Run your async task using `Timeout::run_tokio` or something

    //Propagate timeout value if you need
    let remaining_time = timeout.get_remaining_timeout();
    out.set_timeout_ctx(GRPC_HEADER_NAME, remaining_time);
}
```
