//
// Licensed to the Apache Software Foundation (ASF) under one or more
// contributor license agreements.  See the NOTICE file distributed with
// this work for additional information regarding copyright ownership.
// The ASF licenses this file to You under the Apache License, Version 2.0
// (the "License"); you may not use this file except in compliance with
// the License.  You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use thiserror::Error;

/// Error types for Mahout QDP operations.
///
/// Each variant captures a human-readable message describing the failure context.
/// Use the [`Display`](std::fmt::Display) impl (via `thiserror`) for user-facing
/// error messages, and [`Debug`] for developer diagnostics.
#[derive(Error, Debug)]
pub enum MahoutError {
    /// A CUDA runtime API call failed.
    ///
    /// The inner string contains the function name and the CUDA error code
    /// translated by [`cuda_error_to_string`].
    #[error("CUDA error: {0}")]
    Cuda(String),

    /// The caller provided invalid input (e.g., zero qubits, wrong tensor shape).
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// A GPU memory allocation failed, typically due to out-of-memory.
    ///
    /// The message includes requested size, available memory, and suggested remediation.
    #[error("Memory allocation failed: {0}")]
    MemoryAllocation(String),

    /// A CUDA kernel launch failed.
    ///
    /// The message includes the kernel name and the CUDA error code.
    #[error("Kernel launch failed: {0}")]
    KernelLaunch(String),

    /// A DLPack protocol operation failed (e.g., null pointer, missing deleter).
    #[error("DLPack operation failed: {0}")]
    DLPack(String),

    /// An I/O error without an underlying [`std::io::Error`] source.
    #[error("I/O error: {0}")]
    Io(String),

    /// An I/O error with the underlying [`std::io::Error`] preserved as a source.
    ///
    /// Enables [`Error::source()`](std::error::Error::source) and downcasting.
    #[error("I/O error: {message}")]
    IoWithSource {
        message: String,
        #[source]
        source: std::io::Error,
    },

    /// Functionality that is not yet implemented.
    #[error("Not implemented: {0}")]
    NotImplemented(String),
}

/// Result type alias for Mahout operations
pub type Result<T> = std::result::Result<T, MahoutError>;

/// Convert CUDA error code to human-readable string
#[cfg(target_os = "linux")]
pub fn cuda_error_to_string(code: i32) -> &'static str {
    match code {
        0 => "cudaSuccess",
        1 => "cudaErrorInvalidValue",
        2 => "cudaErrorMemoryAllocation",
        3 => "cudaErrorInitializationError",
        4 => "cudaErrorLaunchFailure",
        6 => "cudaErrorInvalidDevice",
        8 => "cudaErrorInvalidConfiguration",
        11 => "cudaErrorInvalidHostPointer",
        12 => "cudaErrorInvalidDevicePointer",
        17 => "cudaErrorInvalidMemcpyDirection",
        30 => "cudaErrorUnknown",
        400 => "cudaErrorInvalidResourceHandle",
        999 => "CUDA unavailable (non-Linux stub)",
        _ => "Unknown CUDA error",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error as StdError;
    use std::io;

    #[test]
    fn io_with_source_provides_source_and_downcast() {
        let inner = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let err = MahoutError::IoWithSource {
            message: format!("open failed: {}", inner),
            source: inner,
        };
        assert!(err.to_string().contains("open failed"));
        let source = err.source().expect("IoWithSource must have source");
        assert!(source.downcast_ref::<io::Error>().is_some());
    }

    // TODO(GSoC): Add tests for cuda_error_to_string and MahoutError Display — currently only 1 test.
    // Planned tests (~3):
    //
    // cuda_error_to_string:
    //   - known error code (e.g., 2 → "cudaErrorMemoryAllocation")
    //   - unknown error code (e.g., 9999 → "Unknown CUDA error")
    //   - CUDA unavailable stub (999 → "CUDA unavailable (non-Linux stub)")
    //
    // MahoutError Display:
    //   - Cuda variant formats correctly
    //   - InvalidInput variant formats correctly
}
