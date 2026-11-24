// SPDX-FileCopyrightText: 2026 Quack Robotics contributors
// SPDX-License-Identifier: Apache-2.0

//! REST surface of the Micro Duck API. The OpenAPI 3.1 spec in
//! `openapi/quack-api.yaml` is the contract; these handlers implement it.

pub mod grpc;
pub mod hardware;
pub mod startup;
pub mod telemetry;

use std::collections::HashSet;
use std::sync::Arc;

use axum::extract::rejection::JsonRejection;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use quack_core::joint::JointState;
use serde::{Deserialize, Serialize};

use crate::hardware::{Hardware, JointTarget};

/// Version of the REST contract, matching `info.version` in the spec.
pub const API_VERSION: &str = "0.1.0";

#[derive(Clone)]
pub struct AppState {
    hardware: Arc<dyn Hardware>,
}

impl AppState {
    pub fn new(hardware: Arc<dyn Hardware>) -> Self {
        Self { hardware }
    }
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/info", get(info))
        .route("/state", get(get_state).post(post_state))
        .with_state(state)
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Health {
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Info {
    pub name: String,
    pub version: String,
    pub api_version: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct JointView {
    pub name: String,
    pub servo_id: u32,
    pub has_feedback: bool,
    pub position_rad: f32,
    pub velocity_rad_s: Option<f32>,
    pub current_ma: Option<f32>,
    pub temperature_c: Option<f32>,
}

impl From<JointState> for JointView {
    fn from(joint: JointState) -> Self {
        Self {
            servo_id: u32::from(joint.servo_id),
            name: joint.name,
            has_feedback: joint.has_feedback,
            position_rad: joint.position_rad,
            velocity_rad_s: joint.velocity_rad_s,
            current_ma: joint.current_ma,
            temperature_c: joint.temperature_c,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct RobotState {
    pub joints: Vec<JointView>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct JointTargetBody {
    pub name: String,
    pub position_rad: f32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MoveRequest {
    pub joints: Vec<JointTargetBody>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ApiError {
    pub code: String,
    pub message: String,
}

pub struct ErrorResponse(StatusCode, ApiError);

impl ErrorResponse {
    fn new(status: StatusCode, code: &str, message: impl Into<String>) -> Self {
        Self(
            status,
            ApiError {
                code: code.to_string(),
                message: message.into(),
            },
        )
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        (self.0, Json(self.1)).into_response()
    }
}

async fn health() -> Json<Health> {
    Json(Health {
        status: "ok".into(),
    })
}

async fn info() -> Json<Info> {
    Json(Info {
        name: "quack-api".into(),
        version: env!("CARGO_PKG_VERSION").into(),
        api_version: API_VERSION.into(),
    })
}

fn read_state(hardware: &dyn Hardware) -> Result<RobotState, ErrorResponse> {
    let raw = hardware.read_joints().map_err(|error| {
        ErrorResponse::new(
            StatusCode::SERVICE_UNAVAILABLE,
            "hardware_unavailable",
            error.to_string(),
        )
    })?;
    let joints = raw
        .into_iter()
        .map(|joint| JointState::try_from(joint).map(JointView::from))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| {
            ErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "invalid_hardware_state",
                error.to_string(),
            )
        })?;
    Ok(RobotState { joints })
}

async fn get_state(State(state): State<AppState>) -> Result<Json<RobotState>, ErrorResponse> {
    read_state(state.hardware.as_ref()).map(Json)
}

/// Synchronous move: returns the robot state after the move has finished.
async fn post_state(
    State(state): State<AppState>,
    body: Result<Json<MoveRequest>, JsonRejection>,
) -> Result<Json<RobotState>, ErrorResponse> {
    let Json(request) = body.map_err(|rejection| {
        ErrorResponse::new(
            StatusCode::BAD_REQUEST,
            "invalid_body",
            rejection.body_text(),
        )
    })?;
    let targets = validate_move(state.hardware.as_ref(), request)?;
    state.hardware.move_joints(&targets).map_err(|error| {
        ErrorResponse::new(
            StatusCode::SERVICE_UNAVAILABLE,
            "hardware_unavailable",
            error.to_string(),
        )
    })?;
    read_state(state.hardware.as_ref()).map(Json)
}

fn validate_move(
    hardware: &dyn Hardware,
    request: MoveRequest,
) -> Result<Vec<JointTarget>, ErrorResponse> {
    let invalid = |message: String| {
        ErrorResponse::new(StatusCode::UNPROCESSABLE_ENTITY, "invalid_move", message)
    };
    if request.joints.is_empty() {
        return Err(invalid("joints must not be empty".into()));
    }
    let known: HashSet<String> = read_state(hardware)?
        .joints
        .into_iter()
        .map(|joint| joint.name)
        .collect();
    let mut seen = HashSet::new();
    request
        .joints
        .into_iter()
        .map(|target| {
            if !known.contains(&target.name) {
                return Err(invalid(format!("unknown joint {}", target.name)));
            }
            if !seen.insert(target.name.clone()) {
                return Err(invalid(format!("joint {} listed twice", target.name)));
            }
            if !target.position_rad.is_finite() {
                return Err(invalid(format!(
                    "position_rad for {} must be finite",
                    target.name
                )));
            }
            Ok(JointTarget {
                name: target.name,
                position_rad: target.position_rad,
            })
        })
        .collect()
}
