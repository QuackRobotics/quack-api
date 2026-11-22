// SPDX-FileCopyrightText: 2026 Quack Robotics contributors
// SPDX-License-Identifier: Apache-2.0

//! Servo identifiers.
//!
//! Bus servos use ids 1..=14, whatever the servo model. Joints that are not on the bus,
//! such as the two antennas on Pi PWM (GPIO12/13), carry the sentinel 255.
//! Id 0 is never assigned on our bus, so it is rejected rather than
//! silently treated as "unassigned".
//!
//! A bus id can only be built through `TryFrom`, so an out-of-range id never exists.

use std::fmt;

/// Wire value for a joint that has no bus servo.
pub const UNASSIGNED_SERVO_ID: u32 = 255;

/// Highest id assigned on the servo bus.
pub const MAX_BUS_SERVO_ID: u32 = 14;

/// A validated servo bus id in 1..=`MAX_BUS_SERVO_ID`.
///
/// The field is private, so it cannot be built directly:
///
/// ```compile_fail
/// let _ = quack_core::servo::BusServoId(0);
/// ```
///
/// ```compile_fail
/// let _ = quack_core::servo::ServoId::Bus(quack_core::servo::BusServoId(15));
/// ```
///
/// Use `TryFrom` instead:
///
/// ```
/// use quack_core::servo::BusServoId;
/// assert_eq!(BusServoId::try_from(3).map(BusServoId::get), Ok(3));
/// assert!(BusServoId::try_from(0).is_err());
/// assert!(BusServoId::try_from(15).is_err());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BusServoId(u8);

impl BusServoId {
    /// The raw bus id.
    pub fn get(self) -> u8 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServoId {
    Bus(BusServoId),
    Unassigned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidServoId(pub u32);

impl fmt::Display for InvalidServoId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "invalid servo_id {}: expected 1..={} or {} (unassigned)",
            self.0, MAX_BUS_SERVO_ID, UNASSIGNED_SERVO_ID
        )
    }
}

impl std::error::Error for InvalidServoId {}

impl TryFrom<u32> for BusServoId {
    type Error = InvalidServoId;

    fn try_from(raw: u32) -> Result<Self, Self::Error> {
        match raw {
            1..=MAX_BUS_SERVO_ID => Ok(BusServoId(raw as u8)),
            _ => Err(InvalidServoId(raw)),
        }
    }
}

impl TryFrom<u32> for ServoId {
    type Error = InvalidServoId;

    fn try_from(raw: u32) -> Result<Self, Self::Error> {
        match raw {
            UNASSIGNED_SERVO_ID => Ok(ServoId::Unassigned),
            _ => BusServoId::try_from(raw).map(ServoId::Bus),
        }
    }
}

impl From<ServoId> for u32 {
    fn from(id: ServoId) -> Self {
        match id {
            ServoId::Bus(bus_id) => u32::from(bus_id.get()),
            ServoId::Unassigned => UNASSIGNED_SERVO_ID,
        }
    }
}
