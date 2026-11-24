// SPDX-FileCopyrightText: 2026 Quack Robotics contributors
// SPDX-License-Identifier: Apache-2.0

use quack_api::startup::{check, Backend, StartupError};

#[test]
fn defaults_to_mock() {
    assert_eq!(check(None, None), Ok(Backend::Mock));
    assert_eq!(check(Some("mock"), None), Ok(Backend::Mock));
}

#[test]
fn blank_joint_limits_env_counts_as_unset() {
    assert_eq!(check(Some("mock"), Some("")), Ok(Backend::Mock));
    assert_eq!(check(Some("mock"), Some("  ")), Ok(Backend::Mock));
}

#[test]
fn joint_limits_env_is_refused_on_every_backend() {
    assert_eq!(
        check(Some("mock"), Some("limits.toml")),
        Err(StartupError::JointLimitsOverride)
    );
    assert_eq!(
        check(Some("real"), Some("limits.toml")),
        Err(StartupError::JointLimitsOverride)
    );
}

#[test]
fn real_hardware_is_refused_until_a_driver_exists() {
    assert_eq!(
        check(Some("real"), None),
        Err(StartupError::RealBackendNotImplemented)
    );
}

#[test]
fn unknown_backend_is_refused() {
    assert_eq!(
        check(Some("Real"), None),
        Err(StartupError::UnknownBackend("Real".to_string()))
    );
}
