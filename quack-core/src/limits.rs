// SPDX-FileCopyrightText: 2026 Quack Robotics contributors
// SPDX-License-Identifier: Apache-2.0

//! Per-joint position limits, joint table v1 (PROVISIONAL).
//!
//! Copied from `docs/proposals/joints-v1-provisional.md` (Atlas, accepted
//! 2026-11-24): joints 0-13 from the Open_Duck_Playground MJCF at commit
//! `b9be205` (MJCF sha256 `968b18de`), antennas 14-15 pending a bench check.
//! Every move target on every API surface is checked against this one table,
//! so the limits cannot drift between REST, gRPC and expression offsets.
//! This table is the only source of limits: there is no runtime override.
//!
//! The API matches joints by name only, never by index. The order here is the
//! policy/MJCF order and differs from the mock bus order on purpose.
//!
//! Precondition for any real servo driver: the name -> servo ID / bus index
//! map lives in the driver/HAL (owned by Atlas, IDs confirmed by Woz), not in
//! the API. A real driver must not ship until that map exists and covers every
//! joint in this table.

/// Inclusive position range of one joint, in radians.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct JointLimit {
    pub name: &'static str,
    pub min_rad: f32,
    pub max_rad: f32,
}

impl JointLimit {
    pub fn contains(&self, position_rad: f32) -> bool {
        (self.min_rad..=self.max_rad).contains(&position_rad)
    }
}

const fn limit(name: &'static str, min_rad: f32, max_rad: f32) -> JointLimit {
    JointLimit {
        name,
        min_rad,
        max_rad,
    }
}

/// In policy/MJCF index order. Hip pitch is mirrored left vs right.
// The values are the table's 4-decimal radians, kept verbatim so they can be
// diffed against the doc; swapping in exact PI fractions would move the bounds.
#[allow(clippy::approx_constant)]
pub const JOINT_LIMITS_V1: [JointLimit; 16] = [
    limit("left_hip_yaw", -0.5236, 0.5236),
    limit("left_hip_roll", -0.4363, 0.4363),
    limit("left_hip_pitch", -1.2217, 0.5236),
    limit("left_knee", -1.5708, 1.5708),
    limit("left_ankle", -1.5708, 1.5708),
    limit("neck_pitch", -0.3491, 1.1345),
    limit("head_pitch", -0.7854, 0.7854),
    limit("head_yaw", -2.7925, 2.7925),
    limit("head_roll", -0.5236, 0.5236),
    limit("right_hip_yaw", -0.5236, 0.5236),
    limit("right_hip_roll", -0.4363, 0.4363),
    limit("right_hip_pitch", -0.5236, 1.2217),
    limit("right_knee", -1.5708, 1.5708),
    limit("right_ankle", -1.5708, 1.5708),
    limit("left_antenna", -0.7854, 0.7854),
    limit("right_antenna", -0.7854, 0.7854),
];

/// The limit for a joint, or `None` if the table has no such joint.
pub fn limit_for(name: &str) -> Option<&'static JointLimit> {
    JOINT_LIMITS_V1.iter().find(|limit| limit.name == name)
}
