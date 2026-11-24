// SPDX-FileCopyrightText: 2026 Quack Robotics contributors
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashSet;

use quack_core::limits::{limit_for, JOINT_LIMITS_V1};

#[test]
fn table_has_sixteen_unique_joints_with_min_below_max() {
    let names: HashSet<_> = JOINT_LIMITS_V1.iter().map(|limit| limit.name).collect();
    assert_eq!(names.len(), 16);
    for limit in JOINT_LIMITS_V1 {
        assert!(limit.min_rad < limit.max_rad, "{}", limit.name);
    }
}

#[test]
fn antennas_are_indices_14_and_15() {
    assert_eq!(JOINT_LIMITS_V1[14].name, "left_antenna");
    assert_eq!(JOINT_LIMITS_V1[15].name, "right_antenna");
}

#[test]
fn bounds_are_inclusive_and_hip_pitch_is_mirrored() {
    let left = limit_for("left_hip_pitch").unwrap();
    let right = limit_for("right_hip_pitch").unwrap();
    assert!(left.contains(-1.2217) && !left.contains(1.2217));
    assert!(right.contains(1.2217) && !right.contains(-1.2217));
    assert!(!left.contains(f32::NAN));
}

#[test]
fn unknown_joint_has_no_limit() {
    assert!(limit_for("tail").is_none());
}
