use quack_core::servo::{InvalidServoId, ServoId, UNASSIGNED_SERVO_ID};

#[test]
fn bus_ids_1_to_16_are_valid_and_round_trip() {
    for raw in 1..=16u32 {
        let id = ServoId::try_from(raw).expect("bus id should be valid");
        assert_eq!(id, ServoId::Bus(raw as u8));
        assert_eq!(u32::from(id), raw);
    }
}

#[test]
fn sentinel_255_is_unassigned_and_round_trips() {
    let id = ServoId::try_from(UNASSIGNED_SERVO_ID).expect("255 should be valid");
    assert_eq!(id, ServoId::Unassigned);
    assert_eq!(u32::from(id), 255);
}

#[test]
fn zero_is_rejected() {
    assert_eq!(ServoId::try_from(0), Err(InvalidServoId(0)));
}

#[test]
fn ids_outside_bus_range_and_not_sentinel_are_rejected() {
    for raw in [17u32, 100, 254, 256, u32::MAX] {
        assert_eq!(ServoId::try_from(raw), Err(InvalidServoId(raw)), "id {raw}");
    }
}
