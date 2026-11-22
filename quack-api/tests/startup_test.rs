// SPDX-FileCopyrightText: 2026 Quack Robotics contributors
// SPDX-License-Identifier: Apache-2.0

use quack_api::startup::{check, Backend, StartupError};

#[test]
fn defaults_to_mock() {
    assert_eq!(check(None, None), Ok(Backend::Mock));
    assert_eq!(check(Some("mock"), None), Ok(Backend::Mock));
}

#[test]
fn real_hardware_without_limits_is_refused() {
    assert_eq!(
        check(Some("real"), None),
        Err(StartupError::JointLimitsMissing)
    );
    assert_eq!(
        check(Some("real"), Some("")),
        Err(StartupError::JointLimitsMissing)
    );
    assert_eq!(
        check(Some("real"), Some("  ")),
        Err(StartupError::JointLimitsMissing)
    );
}

#[test]
fn real_hardware_with_limits_is_still_refused_until_a_driver_exists() {
    assert_eq!(
        check(Some("real"), Some("limits.toml")),
        Err(StartupError::RealBackendNotImplemented)
    );
}

#[test]
fn unknown_backend_is_refused() {
    assert_eq!(
        check(Some("Real"), Some("limits.toml")),
        Err(StartupError::UnknownBackend("Real".to_string()))
    );
}
