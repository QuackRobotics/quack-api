// SPDX-FileCopyrightText: 2026 Quack Robotics contributors
// SPDX-License-Identifier: Apache-2.0

//! The hardware boundary the API talks to, plus an in-memory mock.

use std::fmt;
use std::sync::Mutex;

use quack_core::joint::RawJointState;
use quack_core::servo::UNASSIGNED_SERVO_ID;

/// A requested joint position.
#[derive(Debug, Clone, PartialEq)]
pub struct JointTarget {
    pub name: String,
    pub position_rad: f32,
}

/// The hardware could not carry out a request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HardwareError(pub String);

impl fmt::Display for HardwareError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "hardware error: {}", self.0)
    }
}

impl std::error::Error for HardwareError {}

/// What the API needs from the robot. Implementations must be thread-safe.
pub trait Hardware: Send + Sync + 'static {
    /// Current reading of every joint.
    fn read_joints(&self) -> Result<Vec<RawJointState>, HardwareError>;

    /// Move to the targets and return once the move has finished.
    /// Targets have already been checked against `read_joints` names.
    fn move_joints(&self, targets: &[JointTarget]) -> Result<(), HardwareError>;
}

/// Joint names of the mock duck: 14 bus servos, then 2 antennas off the bus.
pub const MOCK_BUS_JOINTS: [&str; 14] = [
    "left_hip_yaw",
    "left_hip_roll",
    "left_hip_pitch",
    "left_knee",
    "left_ankle",
    "right_hip_yaw",
    "right_hip_roll",
    "right_hip_pitch",
    "right_knee",
    "right_ankle",
    "neck_pitch",
    "head_pitch",
    "head_yaw",
    "head_roll",
];

pub const MOCK_ANTENNA_JOINTS: [&str; 2] = ["left_antenna", "right_antenna"];

/// In-memory robot: moves are applied instantly. Antennas report junk in
/// their feedback registers on purpose, so the API's filtering is exercised.
pub struct MockHardware {
    joints: Mutex<Vec<RawJointState>>,
}

impl MockHardware {
    pub fn new() -> Self {
        let bus = MOCK_BUS_JOINTS
            .iter()
            .zip(1u32..)
            .map(|(name, servo_id)| RawJointState {
                name: (*name).to_string(),
                servo_id,
                has_feedback: true,
                position_rad: 0.0,
                velocity_rad_s: 0.0,
                current_ma: 120.0,
                temperature_c: 31.5,
            });
        let antennas = MOCK_ANTENNA_JOINTS.iter().map(|name| RawJointState {
            name: (*name).to_string(),
            servo_id: UNASSIGNED_SERVO_ID,
            has_feedback: false,
            position_rad: 0.0,
            velocity_rad_s: 9_999.0,
            current_ma: -1.0,
            temperature_c: 1_234.5,
        });
        Self {
            joints: Mutex::new(bus.chain(antennas).collect()),
        }
    }
}

impl Default for MockHardware {
    fn default() -> Self {
        Self::new()
    }
}

impl Hardware for MockHardware {
    fn read_joints(&self) -> Result<Vec<RawJointState>, HardwareError> {
        let joints = self
            .joints
            .lock()
            .map_err(|_| HardwareError("mock state lock poisoned".into()))?;
        Ok(joints.clone())
    }

    fn move_joints(&self, targets: &[JointTarget]) -> Result<(), HardwareError> {
        let mut joints = self
            .joints
            .lock()
            .map_err(|_| HardwareError("mock state lock poisoned".into()))?;
        for target in targets {
            let joint = joints
                .iter_mut()
                .find(|joint| joint.name == target.name)
                .ok_or_else(|| HardwareError(format!("unknown joint {}", target.name)))?;
            joint.position_rad = target.position_rad;
        }
        Ok(())
    }
}
