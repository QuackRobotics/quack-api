//! Joint state as reported by the hardware layer, and the view exposed to
//! API clients. Joints without feedback (PWM antennas) report whatever the
//! driver left in the velocity/current/temperature registers; those values
//! are junk and must never reach REST, WebSocket or dashboard clients.

use crate::servo::{InvalidServoId, ServoId};

/// Raw reading straight from the hardware layer (wire `servo_id` included).
#[derive(Debug, Clone, PartialEq)]
pub struct RawJointState {
    pub name: String,
    pub servo_id: u32,
    pub has_feedback: bool,
    pub position_rad: f32,
    pub velocity_rad_s: f32,
    pub current_ma: f32,
    pub temperature_c: f32,
}

/// What clients see. Feedback fields are `None` when the joint has no feedback.
#[derive(Debug, Clone, PartialEq)]
pub struct JointState {
    pub name: String,
    pub servo_id: ServoId,
    pub has_feedback: bool,
    pub position_rad: f32,
    pub velocity_rad_s: Option<f32>,
    pub current_ma: Option<f32>,
    pub temperature_c: Option<f32>,
}

impl TryFrom<RawJointState> for JointState {
    type Error = InvalidServoId;

    fn try_from(raw: RawJointState) -> Result<Self, Self::Error> {
        let servo_id = ServoId::try_from(raw.servo_id)?;
        let feedback = |value: f32| raw.has_feedback.then_some(value);
        Ok(JointState {
            velocity_rad_s: feedback(raw.velocity_rad_s),
            current_ma: feedback(raw.current_ma),
            temperature_c: feedback(raw.temperature_c),
            name: raw.name,
            servo_id,
            has_feedback: raw.has_feedback,
            position_rad: raw.position_rad,
        })
    }
}
