// SPDX-FileCopyrightText: 2026 Quack Robotics contributors
// SPDX-License-Identifier: Apache-2.0

//! Startup guard: the server only runs against real hardware once per-joint
//! position limits are configured. Moves are already checked against the
//! provisional compiled-in table (`quack_core::limits`); this guard stays
//! until a real driver and its bench-confirmed limits exist.

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
    /// Real hardware was requested without `QUACK_JOINT_LIMITS`.
    JointLimitsMissing,
    /// Limits are configured, but no real hardware driver exists yet.
    RealBackendNotImplemented,
}

impl fmt::Display for StartupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownBackend(value) => {
                write!(f, "QUACK_HARDWARE={value:?} is not one of: mock, real")
            }
            Self::JointLimitsMissing => write!(
                f,
                "refusing to start on real hardware: per-joint position limits are not configured (set QUACK_JOINT_LIMITS)"
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
    if backend == Backend::Real {
        if joint_limits.is_none_or(|value| value.trim().is_empty()) {
            return Err(StartupError::JointLimitsMissing);
        }
        return Err(StartupError::RealBackendNotImplemented);
    }
    Ok(backend)
}
