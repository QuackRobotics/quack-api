// SPDX-FileCopyrightText: 2026 Quack Robotics contributors
// SPDX-License-Identifier: Apache-2.0

use quack_api::hardware::{MOCK_ANTENNA_JOINTS, MOCK_BUS_JOINTS};
use quack_core::limits::{limit_for, JOINT_LIMITS_V1};

#[test]
fn every_mock_joint_has_a_limit_and_every_limit_a_mock_joint() {
    let mock: Vec<&str> = MOCK_BUS_JOINTS
        .iter()
        .chain(MOCK_ANTENNA_JOINTS.iter())
        .copied()
        .collect();
    for name in &mock {
        assert!(limit_for(name).is_some(), "no limit for {name}");
    }
    for limit in JOINT_LIMITS_V1 {
        assert!(mock.contains(&limit.name), "{} not in mock", limit.name);
    }
}
