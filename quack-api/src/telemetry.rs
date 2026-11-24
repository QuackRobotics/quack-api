// SPDX-FileCopyrightText: 2026 Quack Robotics contributors
// SPDX-License-Identifier: Apache-2.0

//! WebSocket telemetry (default :8081). `GET /telemetry` upgrades to a
//! WebSocket and streams one JSON `TelemetryFrame` per tick at
//! [`JOINT_RATE_HZ`]. The frame mirrors `quack.telemetry.v1.JointStates`,
//! with null feedback fields for joints that have none, as in `GET /state`.

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use axum::extract::ws::{close_code, CloseFrame, Message, WebSocket, WebSocketUpgrade};
use axum::extract::State;
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use serde::{Deserialize, Serialize};
use tokio::time::MissedTickBehavior;

use crate::{read_state, AppState, JointView};

/// Joint telemetry rate.
pub const JOINT_RATE_HZ: u64 = 50;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TelemetryFrame {
    /// Microseconds since the Unix epoch when the joints were read.
    pub timestamp_us: u64,
    pub joints: Vec<JointView>,
}

/// Microseconds since the Unix epoch (0 if the clock is before 1970).
pub fn now_us() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|elapsed| elapsed.as_micros() as u64)
        .unwrap_or(0)
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/telemetry", get(upgrade))
        .with_state(state)
}

async fn upgrade(socket: WebSocketUpgrade, State(state): State<AppState>) -> Response {
    socket.on_upgrade(move |socket| stream(socket, state))
}

async fn stream(mut socket: WebSocket, state: AppState) {
    let mut ticker = tokio::time::interval(Duration::from_micros(1_000_000 / JOINT_RATE_HZ));
    ticker.set_missed_tick_behavior(MissedTickBehavior::Skip);
    loop {
        ticker.tick().await;
        let frame = match read_state(state.hardware.as_ref()) {
            Ok(robot) => TelemetryFrame {
                timestamp_us: now_us(),
                joints: robot.joints,
            },
            Err(error) => {
                eprintln!(
                    "telemetry stream closed: {}: {}",
                    error.1.code, error.1.message
                );
                let close = CloseFrame {
                    code: close_code::ERROR,
                    reason: error.1.code.into(),
                };
                // The stream ends either way; a failed close means the client is gone.
                let _ = socket.send(Message::Close(Some(close))).await;
                return;
            }
        };
        let text = match serde_json::to_string(&frame) {
            Ok(text) => text,
            Err(error) => {
                eprintln!("telemetry frame not serializable: {error}");
                return;
            }
        };
        if socket.send(Message::Text(text.into())).await.is_err() {
            // Client disconnected.
            return;
        }
    }
}
