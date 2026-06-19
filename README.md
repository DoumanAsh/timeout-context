# timeout-context

[![Rust](https://github.com/DoumanAsh/timeout-context/actions/workflows/rust.yml/badge.svg)](https://github.com/DoumanAsh/timeout-context/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/timeout-context.svg)](https://crates.io/crates/timeout-context)
[![Documentation](https://docs.rs/timeout-context/badge.svg)](https://docs.rs/crate/timeout-context/)

Contextual timeout utilities to handle timeouts across your applications

This crate provides simple utility to provide uniform timeout handling for requests, and propagation utilities.
It uses [grpc semantics](https://github.com/grpc/grpc/blob/master/doc/PROTOCOL-HTTP2.md) to parse timeout value
