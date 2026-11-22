use quack_core::joint::{JointState, RawJointState};
use quack_core::servo::ServoId;

fn raw(name: &str, servo_id: u32, has_feedback: bool) -> RawJointState {
    RawJointState {
        name: name.to_string(),
        servo_id,
        has_feedback,
        position_rad: 0.25,
        velocity_rad_s: 9_999.0,
        current_ma: -1.0,
        temperature_c: 1_234.5,
    }
}

#[test]
fn antenna_without_feedback_drops_junk_fields() {
    let state = JointState::try_from(raw("left_antenna", 255, false)).unwrap();
    assert_eq!(state.servo_id, ServoId::Unassigned);
    assert!(!state.has_feedback);
    assert_eq!(state.position_rad, 0.25);
    assert_eq!(state.velocity_rad_s, None);
    assert_eq!(state.current_ma, None);
    assert_eq!(state.temperature_c, None);
}

#[test]
fn bus_servo_with_feedback_keeps_fields() {
    let state = JointState::try_from(raw("left_hip_yaw", 1, true)).unwrap();
    assert_eq!(state.servo_id, ServoId::Bus(1));
    assert_eq!(state.velocity_rad_s, Some(9_999.0));
    assert_eq!(state.current_ma, Some(-1.0));
    assert_eq!(state.temperature_c, Some(1_234.5));
}

#[test]
fn joint_with_servo_id_zero_is_rejected() {
    assert!(JointState::try_from(raw("left_antenna", 0, false)).is_err());
}
