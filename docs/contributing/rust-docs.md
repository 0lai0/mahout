---
title: Rust Documentation Standards
sidebar_label: Rust Docs
---

# Rust Documentation Standards for QDP

This guide covers the documentation conventions used in `qdp-core` and related Rust crates. All contributors should follow these standards when adding or modifying public API items.

## Quick Reference

| Rule | Standard |
|------|----------|
| Every public item | Must have a `///` doc comment |
| Module-level docs | Use `//!` at the top of the file |
| First line | Brief summary (sentence fragment, no period) |
| Sections | `# Arguments`, `# Returns`, `# Errors`, `# Safety`, `# Examples`, `# Panics` |
| Unsafe code | Every `unsafe` block must have a `// SAFETY:` comment |
| CI enforcement | `RUSTDOCFLAGS="-D warnings"` in `.github/workflows/rustdoc.yml` |

## RFC 505 Documentation Format

All public items must follow [RFC 505 — API Comment Conventions](https://rust-lang.github.io/rfcs/0505-api-comment-conventions.html).

### Functions and Methods

```rust
/// Encode classical data into a quantum state vector on the GPU.
///
/// Selects an encoding strategy by name and executes the kernel on the
/// engine's CUDA device.
///
/// # Arguments
///
/// * `data` — Input data as a flat `f64` slice
/// * `num_qubits` — Number of qubits in the output state
/// * `encoding_method` — One of `"amplitude"`, `"angle"`, `"basis"`, `"iqp"`, `"iqp-z"`
///
/// # Returns
///
/// A raw pointer to a [`DLManagedTensor`] for zero-copy PyTorch integration.
/// The pointer is freed by the DLPack deleter when PyTorch releases the tensor.
///
/// # Errors
///
/// Returns [`MahoutError::InvalidInput`] if:
/// - `data` is empty
/// - `num_qubits` is 0 or exceeds `MAX_QUBITS`
/// - `encoding_method` is not recognized
///
/// # Examples
///
/// ```rust,ignore
/// let engine = QdpEngine::new(0)?;
/// let dlpack_ptr = engine.encode(&[1.0, 0.0, 0.0, 0.0], 2, "amplitude")?;
/// ```
pub fn encode(&self, data: &[f64], num_qubits: usize, encoding_method: &str) -> Result<*mut DLManagedTensor> {
    // ...
}
```

### Structs and Enums

```rust
/// Error types for Mahout QDP operations.
///
/// Each variant captures a human-readable message describing the failure context.
#[derive(Error, Debug)]
pub enum MahoutError {
    /// A CUDA runtime API call failed.
    ///
    /// The inner string contains the function name and the CUDA error code.
    #[error("CUDA error: {0}")]
    Cuda(String),
}
```

### Modules

```rust
//! GPU memory management for QDP.
//!
//! This module provides safe wrappers around CUDA memory allocation,
//! pinned host buffers, and precision conversion between Float32 and Float64.
```

## SAFETY Comments

Every `unsafe` block **must** have a `// SAFETY:` comment explaining why the operation is sound. The comment should appear immediately above the `unsafe` keyword.

### Format

```rust
// SAFETY: `ptr` was allocated by `cudaHostAlloc` in `new()` with
// `self.size_elements * size_of::<f64>()` bytes. The pointer remains valid
// for the lifetime of `self` (freed in `Drop`). `&mut self` guarantees
// exclusive access.
unsafe { std::slice::from_raw_parts_mut(self.ptr, self.size_elements) }
```

### What to Document

| Question | Example |
|----------|---------|
| **Why is the pointer valid?** | "Allocated by `cudaHostAlloc` in `new()`, not freed yet" |
| **Why is the size correct?** | "Length matches `ndim` set at creation time" |
| **Why is there no aliasing?** | "`&mut self` guarantees exclusive access" |
| **Why is the lifetime sufficient?** | "Freed in `Drop`, which runs after this reference expires" |

## Building and Checking Documentation

### Local Build

```bash
cd qdp
cargo doc --no-deps --open
```

### Strict Mode (matches CI)

```bash
cd qdp
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
```

This will fail if:
- Public items lack documentation
- Documentation has broken links
- Doc examples don't compile

### CI Integration

Documentation is checked automatically by the `rustdoc.yml` GitHub Actions workflow on every PR that touches `qdp/`. See [`.github/workflows/rustdoc.yml`](/.github/workflows/rustdoc.yml).
