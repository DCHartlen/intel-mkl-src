# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

This is a Cargo workspace providing Intel MKL (Math Kernel Library) integration for Rust. It contains three crates:

- **`intel-mkl-src`** — A `*-src` crate that links MKL libraries to executables via `build.rs`. Does not provide Rust bindings; intended to be used alongside `blas-sys`, `lapack-sys`, or `fftw-sys`.
- **`intel-mkl-tool`** — Library used by `build.rs` to locate MKL on the system. Searches via `pkg-config`, `$MKLROOT` env var, and well-known install paths. Also available as a standalone example (`seek`).
- **`intel-mkl-sys`** — FFI bindings to MKL-specific features: Vector Mathematical Functions (VM) and Statistical Functions (VSL). Generated via `bindgen`.

## Commands

```bash
# Build
cargo build

# Run tests
cargo test

# Run tests for a specific crate
cargo test -p intel-mkl-tool

# Run a single test
cargo test -p intel-mkl-tool config::tests::name_to_config

# Format check (as CI does)
cargo fmt -- --check

# Format
cargo fmt

# Lint
cargo clippy

# Run the seek example (shows which MKL configs are found on the system)
cargo run --example seek -p intel-mkl-tool

# Run benchmarks (intel-mkl-sys, requires MKL to be available)
cargo bench -p intel-mkl-sys

# Regenerate FFI bindings (requires bindgen CLI)
cd intel-mkl-sys && bash bindgen.sh
```

## MKL Configuration Features

There are 8 feature flags (2×2×2) for `intel-mkl-src` and `intel-mkl-sys` controlling how MKL is linked:

```
mkl-{static|dynamic}-{lp64|ilp64}-{iomp|seq}
```

- **Link type**: `static` (embedded in binary) vs `dynamic` (loaded at runtime)
- **Data model**: `lp64` (32-bit `int`) vs `ilp64` (64-bit `int`, `long`, and pointers)
- **Threading**: `iomp` (Intel OpenMP runtime) vs `seq` (single-threaded)

Default when no feature is specified: `mkl-static-ilp64-iomp`

## Architecture

### Library Discovery (`intel-mkl-tool`)

`Library::new(config)` tries in order:
1. `pkg-config --variable=prefix <config-name>`
2. `$MKLROOT` environment variable
3. Hardcoded paths: `/opt/intel` (Linux), `C:/Program Files (x86)/IntelSWTools/` and `C:/Program Files (x86)/Intel/oneAPI/` (Windows)

`Library::seek_directory` walks the directory recursively looking for `mkl.h`, the core MKL `.a`/`.so`/`.lib` files, and the OpenMP runtime (`libiomp5`/`libiomp5md`). It skips 32-bit directories (`ia32*/`, `win-x86`).

### Build Script (`intel-mkl-src/build.rs`)

1. Calls `Library::new(cfg)` to find system MKL and emit `cargo:rustc-link-*` metadata.
2. If not found and link type is `static`, falls back to downloading pre-built binaries from `ghcr.io/rust-math/rust-mkl` via `ocipkg`.

### FFI Bindings (`intel-mkl-sys`)

`src/mkl.rs` is auto-generated from `wrapper.h` using `bindgen`. The features in `intel-mkl-sys` forward to corresponding features in `intel-mkl-src`.

## Toolchain

Pinned to Rust `1.72.0` via `rust-toolchain.toml`.

## GNU OpenMP Note

GNU OpenMP (`libgomp`) is not supported; only Intel OpenMP (`iomp5`) is supported for `iomp` configurations. See issue #97.
