// SPDX-FileCopyrightText: 2026 Quack Robotics contributors
// SPDX-License-Identifier: Apache-2.0

//! gRPC control surface (`quack.control.v1.ControlService`, default :50051).
//! Shares validation and hardware access with the REST handlers, so both
//! surfaces accept and reject exactly the same moves.

use axum::http::StatusCode;
use tokio::net::TcpListener;
use tonic::transport::server::TcpIncoming;
use tonic::{Request, Response, Status};

use crate::telemetry::now_us;
use crate::{read_state, validate_move, AppState, ErrorResponse, JointTargetBody, MoveRequest};

/// Code generated from quack-proto by `build.rs`.
pub mod pb {
    pub mod quack {
        pub mod telemetry {
            pub mod v1 {
                tonic::include_proto!("quack.telemetry.v1");
            }
        }
        pub mod control {
            pub mod v1 {
                tonic::include_proto!("quack.control.v1");
            }
        }
    }
}

use pb::quack::control::v1::control_service_server::{ControlService, ControlServiceServer};
use pb::quack::control::v1::{
    GetStateRequest, GetStateResponse, MoveJointsRequest, MoveJointsResponse,
};
use pb::quack::telemetry::v1::{JointState, JointStates};

/// Wraps the control listener so every accepted connection sets TCP_NODELAY.
/// Without it, Nagle plus delayed ACK adds about 40 ms per unary call and
/// breaks the < 5 ms control-loop budget.
pub fn incoming(listener: TcpListener) -> TcpIncoming {
    TcpIncoming::from(listener).with_nodelay(Some(true))
}

#[derive(Clone)]
pub struct ControlApi {
    state: AppState,
}

impl ControlApi {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }

    pub fn into_server(self) -> ControlServiceServer<Self> {
        ControlServiceServer::new(self)
    }

    fn snapshot(&self) -> Result<JointStates, Status> {
        let state = read_state(self.state.hardware.as_ref()).map_err(to_status)?;
        Ok(JointStates {
            timestamp_us: now_us(),
            // Feedback fields are undefined when has_feedback is false; the
            // proto has no null, so they are sent as 0.
            joints: state
                .joints
                .into_iter()
                .map(|joint| JointState {
                    name: joint.name,
                    servo_id: joint.servo_id,
                    has_feedback: joint.has_feedback,
                    position_rad: joint.position_rad,
                    velocity_rad_s: joint.velocity_rad_s.unwrap_or(0.0),
                    current_ma: joint.current_ma.unwrap_or(0.0),
                    temperature_c: joint.temperature_c.unwrap_or(0.0),
                })
                .collect(),
        })
    }
}

/// Maps the REST error to the matching gRPC status; the message keeps the
/// REST error code as a prefix (`invalid_move: ...`).
fn to_status(error: ErrorResponse) -> Status {
    let ErrorResponse(status, body) = error;
    let message = format!("{}: {}", body.code, body.message);
    match status {
        StatusCode::BAD_REQUEST | StatusCode::UNPROCESSABLE_ENTITY => {
            Status::invalid_argument(message)
        }
        StatusCode::SERVICE_UNAVAILABLE => Status::unavailable(message),
        _ => Status::internal(message),
    }
}

#[tonic::async_trait]
impl ControlService for ControlApi {
    async fn get_state(
        &self,
        _request: Request<GetStateRequest>,
    ) -> Result<Response<GetStateResponse>, Status> {
        Ok(Response::new(GetStateResponse {
            state: Some(self.snapshot()?),
        }))
    }

    async fn move_joints(
        &self,
        request: Request<MoveJointsRequest>,
    ) -> Result<Response<MoveJointsResponse>, Status> {
        let request = MoveRequest {
            joints: request
                .into_inner()
                .joints
                .into_iter()
                .map(|target| JointTargetBody {
                    name: target.name,
                    position_rad: target.position_rad,
                })
                .collect(),
        };
        let hardware = self.state.hardware.as_ref();
        let targets = validate_move(hardware, request).map_err(to_status)?;
        hardware
            .move_joints(&targets)
            .map_err(|error| Status::unavailable(format!("hardware_unavailable: {error}")))?;
        Ok(Response::new(MoveJointsResponse {
            state: Some(self.snapshot()?),
        }))
    }
}
