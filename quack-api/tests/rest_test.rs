// SPDX-FileCopyrightText: 2026 Quack Robotics contributors
// SPDX-License-Identifier: Apache-2.0

//! Integration tests: the real router over the mock hardware, in-process.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{header, Method, Request, StatusCode};
use axum::Router;
use http_body_util::BodyExt;
use quack_api::hardware::{Hardware, HardwareError, JointTarget, MockHardware};
use quack_api::{router, ApiError, AppState, Health, Info, RobotState, API_VERSION};
use quack_core::joint::RawJointState;
use serde::de::DeserializeOwned;
use serde_json::{json, Value};

fn app() -> Router {
    router(AppState::new(Arc::new(MockHardware::new())))
}

async fn call(
    app: Router,
    method: Method,
    uri: &str,
    body: Option<String>,
) -> (StatusCode, Vec<u8>) {
    use tower::ServiceExt;
    let mut request = Request::builder().method(method).uri(uri);
    if body.is_some() {
        request = request.header(header::CONTENT_TYPE, "application/json");
    }
    let request = request
        .body(body.map(Body::from).unwrap_or_else(Body::empty))
        .unwrap();
    let response = app.oneshot(request).await.unwrap();
    let status = response.status();
    let bytes = response
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes()
        .to_vec();
    (status, bytes)
}

fn parse<T: DeserializeOwned>(bytes: &[u8]) -> T {
    serde_json::from_slice(bytes)
        .unwrap_or_else(|error| panic!("{error}: {}", String::from_utf8_lossy(bytes)))
}

async fn post_move(app: Router, body: Value) -> (StatusCode, Vec<u8>) {
    call(app, Method::POST, "/state", Some(body.to_string())).await
}

#[tokio::test]
async fn health_is_ok() {
    let (status, body) = call(app(), Method::GET, "/health", None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        parse::<Health>(&body),
        Health {
            status: "ok".into()
        }
    );
}

#[tokio::test]
async fn info_reports_versions() {
    let (status, body) = call(app(), Method::GET, "/info", None).await;
    assert_eq!(status, StatusCode::OK);
    let info: Info = parse(&body);
    assert_eq!(info.name, "quack-api");
    assert_eq!(info.version, env!("CARGO_PKG_VERSION"));
    assert_eq!(info.api_version, API_VERSION);
}

#[tokio::test]
async fn state_lists_14_bus_joints_and_2_antennas_without_feedback() {
    let (status, body) = call(app(), Method::GET, "/state", None).await;
    assert_eq!(status, StatusCode::OK);
    let state: RobotState = parse(&body);
    assert_eq!(state.joints.len(), 16);
    let bus_ids: Vec<u32> = state
        .joints
        .iter()
        .filter(|j| j.has_feedback)
        .map(|j| j.servo_id)
        .collect();
    assert_eq!(bus_ids, (1..=14).collect::<Vec<u32>>());
    for antenna in state.joints.iter().filter(|j| !j.has_feedback) {
        assert_eq!(antenna.servo_id, 255);
        assert_eq!(antenna.velocity_rad_s, None);
        assert_eq!(antenna.current_ma, None);
        assert_eq!(antenna.temperature_c, None);
    }
}

#[tokio::test]
async fn state_json_has_null_feedback_for_antennas() {
    let (_, body) = call(app(), Method::GET, "/state", None).await;
    let raw: Value = parse(&body);
    let antenna = raw["joints"]
        .as_array()
        .unwrap()
        .iter()
        .find(|j| j["name"] == "left_antenna")
        .unwrap();
    assert_eq!(antenna["velocity_rad_s"], Value::Null);
    assert_eq!(antenna["current_ma"], Value::Null);
    assert_eq!(antenna["temperature_c"], Value::Null);
}

#[tokio::test]
async fn post_state_moves_and_returns_new_state() {
    let app = app();
    let body = json!({"joints": [{"name": "left_knee", "position_rad": 0.5}, {"name": "left_antenna", "position_rad": -0.25}]});
    let (status, bytes) = post_move(app.clone(), body).await;
    assert_eq!(status, StatusCode::OK);
    let after: RobotState = parse(&bytes);
    let position = |state: &RobotState, name: &str| {
        state
            .joints
            .iter()
            .find(|j| j.name == name)
            .unwrap()
            .position_rad
    };
    assert_eq!(position(&after, "left_knee"), 0.5);
    assert_eq!(position(&after, "left_antenna"), -0.25);
    assert_eq!(position(&after, "right_knee"), 0.0);

    let (_, bytes) = call(app, Method::GET, "/state", None).await;
    assert_eq!(position(&parse(&bytes), "left_knee"), 0.5);
}

#[tokio::test]
async fn post_state_rejects_unknown_joint_without_moving() {
    let app = app();
    let body = json!({"joints": [{"name": "left_knee", "position_rad": 0.5}, {"name": "tail", "position_rad": 1.0}]});
    let (status, bytes) = post_move(app.clone(), body).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    let error: ApiError = parse(&bytes);
    assert_eq!(error.code, "invalid_move");
    assert!(error.message.contains("tail"), "{}", error.message);

    let (_, bytes) = call(app, Method::GET, "/state", None).await;
    let state: RobotState = parse(&bytes);
    assert_eq!(
        state
            .joints
            .iter()
            .find(|j| j.name == "left_knee")
            .unwrap()
            .position_rad,
        0.0
    );
}

#[tokio::test]
async fn post_state_rejects_empty_and_duplicate_targets() {
    let (status, _) = post_move(app(), json!({"joints": []})).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);

    let duplicate = json!({"joints": [{"name": "left_knee", "position_rad": 0.1}, {"name": "left_knee", "position_rad": 0.2}]});
    let (status, bytes) = post_move(app(), duplicate).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    assert!(parse::<ApiError>(&bytes).message.contains("twice"));
}

#[tokio::test]
async fn post_state_rejects_malformed_body() {
    let (status, bytes) = call(app(), Method::POST, "/state", Some("{not json".into())).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(parse::<ApiError>(&bytes).code, "invalid_body");

    let (status, _) = post_move(app(), json!({"joints": [{"name": "left_knee"}]})).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

struct BrokenHardware {
    servo_id: u32,
    fail_reads: bool,
}

impl Hardware for BrokenHardware {
    fn read_joints(&self) -> Result<Vec<RawJointState>, HardwareError> {
        if self.fail_reads {
            return Err(HardwareError("bus timeout".into()));
        }
        Ok(vec![RawJointState {
            name: "left_knee".into(),
            servo_id: self.servo_id,
            has_feedback: true,
            position_rad: 0.0,
            velocity_rad_s: 0.0,
            current_ma: 0.0,
            temperature_c: 0.0,
        }])
    }

    fn move_joints(&self, _targets: &[JointTarget]) -> Result<(), HardwareError> {
        Err(HardwareError("bus timeout".into()))
    }
}

fn broken(servo_id: u32, fail_reads: bool) -> Router {
    router(AppState::new(Arc::new(BrokenHardware {
        servo_id,
        fail_reads,
    })))
}

#[tokio::test]
async fn hardware_read_failure_is_503() {
    let (status, bytes) = call(broken(1, true), Method::GET, "/state", None).await;
    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(parse::<ApiError>(&bytes).code, "hardware_unavailable");
}

#[tokio::test]
async fn hardware_move_failure_is_503() {
    let body = json!({"joints": [{"name": "left_knee", "position_rad": 0.1}]});
    let (status, bytes) = post_move(broken(1, false), body).await;
    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(parse::<ApiError>(&bytes).code, "hardware_unavailable");
}

#[tokio::test]
async fn invalid_servo_id_from_hardware_is_500_not_passed_through() {
    let (status, bytes) = call(broken(0, false), Method::GET, "/state", None).await;
    assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(parse::<ApiError>(&bytes).code, "invalid_hardware_state");
}

#[tokio::test]
async fn post_state_rejects_target_outside_joint_limits_without_moving() {
    let app = app();
    let body = json!({"joints": [{"name": "left_knee", "position_rad": 0.5}, {"name": "head_yaw", "position_rad": 2.9}]});
    let (status, bytes) = post_move(app.clone(), body).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    let error: ApiError = parse(&bytes);
    assert_eq!(error.code, "invalid_move");
    assert!(error.message.contains("head_yaw"), "{}", error.message);
    assert!(error.message.contains("outside"), "{}", error.message);

    let (_, bytes) = call(app, Method::GET, "/state", None).await;
    let state: RobotState = parse(&bytes);
    assert!(state.joints.iter().all(|joint| joint.position_rad == 0.0));
}

#[tokio::test]
async fn post_state_accepts_targets_exactly_on_joint_limits() {
    let body = json!({"joints": [{"name": "left_hip_pitch", "position_rad": -1.2217}, {"name": "right_hip_pitch", "position_rad": 1.2217}]});
    let (status, bytes) = post_move(app(), body).await;
    assert_eq!(
        status,
        StatusCode::OK,
        "{}",
        String::from_utf8_lossy(&bytes)
    );
}

#[tokio::test]
async fn post_state_checks_mirrored_hip_pitch_per_side() {
    for (name, position_rad) in [("left_hip_pitch", 1.2217), ("right_hip_pitch", -1.2217)] {
        let body = json!({"joints": [{"name": name, "position_rad": position_rad}]});
        let (status, bytes) = post_move(app(), body).await;
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{name}");
        let error: ApiError = parse(&bytes);
        assert!(error.message.contains(name), "{}", error.message);
    }
}
