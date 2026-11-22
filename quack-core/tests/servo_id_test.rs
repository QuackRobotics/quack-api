// SPDX-FileCopyrightText: 2026 Quack Robotics contributors
// SPDX-License-Identifier: Apache-2.0

use quack_core::servo::{BusServoId, InvalidServoId, ServoId, UNASSIGNED_SERVO_ID};

#[test]
fn bus_ids_1_to_14_are_valid_and_round_trip() {
    for raw in 1..=14u32 {
        let id = ServoId::try_from(raw).expect("bus id should be valid");
        match id {
            ServoId::Bus(bus_id) => assert_eq!(u32::from(bus_id.get()), raw),
            ServoId::Unassigned => panic!("id {raw} should be a bus id"),
        }
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
fn sentinel_255_is_not_a_bus_id() {
    assert_eq!(BusServoId::try_from(255), Err(InvalidServoId(255)));
}

#[test]
fn zero_is_rejected() {
    assert_eq!(ServoId::try_from(0), Err(InvalidServoId(0)));
    assert_eq!(BusServoId::try_from(0), Err(InvalidServoId(0)));
}

#[test]
fn ids_outside_bus_range_and_not_sentinel_are_rejected() {
    for raw in [15u32, 16, 17, 100, 254, 256, u32::MAX] {
        assert_eq!(ServoId::try_from(raw), Err(InvalidServoId(raw)), "id {raw}");
        assert_eq!(
            BusServoId::try_from(raw),
            Err(InvalidServoId(raw)),
            "id {raw}"
        );
    }
}
