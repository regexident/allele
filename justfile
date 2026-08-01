#!/usr/bin/env -S just --justfile

default:
    @just --list

# Show Rust toolchain info
info:
    rustup show
    rustup --version

# Cargo check
check:
    cargo check --workspace

# Run test suite
test:
    cargo test --workspace

# Check formatting
fmt:
    cargo fmt --all -- --check

# Clippy linting
clippy:
    cargo clippy --workspace --all-targets --all-features --tests --examples --benches -- -D warnings

# Run clippy --fix then fmt
fix:
    cargo clippy --workspace --all-targets --all-features --tests --examples --benches --fix -- -D warnings
    cargo fmt --all

# Check with minimal dependency versions
minimal-versions:
    CARGO_UNSTABLE_DIRECT_MINIMAL_VERSIONS=true cargo generate-lockfile
    cargo check --workspace --all-features --all-targets

# Verify MSRV
msrv:
    cargo hack check --rust-version --workspace --ignore-private

# Run all CI checks
all: info check test fmt clippy minimal-versions msrv
