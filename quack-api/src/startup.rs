// SPDX-FileCopyrightText: 2026 Quack Robotics contributors
// SPDX-License-Identifier: Apache-2.0

//! Startup guard. Joint limits have one source: the compiled-in table in
//! `quack_core::limits`. There is no runtime override, so a set
//! `QUACK_JOINT_LIMITS` is refused rather than silently ignored. Real hardware
//! is refused until a real driver exists.

use std::fmt;

/// Which hardware backend to run against. Set with `QUACK_HARDWARE`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Backend {
    /// The in-process mock robot (default).
    Mock,
    /// The real servo bus.
    Real,
}

/// Why the server refused to start.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StartupError {
    /// `QUACK_HARDWARE` held something other than `mock` or `real`.
    UnknownBackend(String),
    /// `QUACK_JOINT_LIMITS` was set, but limits come only from the
    /// compiled-in table.
    JointLimitsOverride,
    /// No real hardware driver exists yet.
    RealBackendNotImplemented,
}

impl fmt::Display for StartupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownBackend(value) => {
                write!(f, "QUACK_HARDWARE={value:?} is not one of: mock, real")
            }
            Self::JointLimitsOverride => write!(
                f,
                "QUACK_JOINT_LIMITS is not supported: joint limits come only from the compiled-in table (quack_core::limits); unset it"
            ),
            Self::RealBackendNotImplemented => {
                write!(f, "real hardware backend is not implemented yet")
            }
        }
    }
}

impl std::error::Error for StartupError {}

/// Decides whether the server may start, from the values of `QUACK_HARDWARE`
/// and `QUACK_JOINT_LIMITS`. A blank `QUACK_JOINT_LIMITS` counts as unset.
pub fn check(hardware: Option<&str>, joint_limits: Option<&str>) -> Result<Backend, StartupError> {
    let backend = match hardware.unwrap_or("mock") {
        "mock" => Backend::Mock,
        "real" => Backend::Real,
        other => return Err(StartupError::UnknownBackend(other.to_string())),
    };
    if joint_limits.is_some_and(|value| !value.trim().is_empty()) {
        return Err(StartupError::JointLimitsOverride);
    }
    if backend == Backend::Real {
        return Err(StartupError::RealBackendNotImplemented);
    }
    Ok(backend)
}
