//! Servo identifiers.
//!
//! Bus servos use ids 1..=14, whatever the servo model. Joints that are not on the bus,
//! such as the two antennas on Pi PWM (GPIO12/13), carry the sentinel 255.
//! Id 0 is never assigned on our bus, so it is rejected rather than
//! silently treated as "unassigned".

use std::fmt;

/// Wire value for a joint that has no bus servo.
pub const UNASSIGNED_SERVO_ID: u32 = 255;

/// Highest id assigned on the servo bus.
pub const MAX_BUS_SERVO_ID: u32 = 14;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServoId {
    Bus(u8),
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

impl TryFrom<u32> for ServoId {
    type Error = InvalidServoId;

    fn try_from(raw: u32) -> Result<Self, Self::Error> {
        match raw {
            UNASSIGNED_SERVO_ID => Ok(ServoId::Unassigned),
            1..=MAX_BUS_SERVO_ID => Ok(ServoId::Bus(raw as u8)),
            _ => Err(InvalidServoId(raw)),
        }
    }
}

impl From<ServoId> for u32 {
    fn from(id: ServoId) -> Self {
        match id {
            ServoId::Bus(bus_id) => u32::from(bus_id),
            ServoId::Unassigned => UNASSIGNED_SERVO_ID,
        }
    }
}
