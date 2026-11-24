// SPDX-FileCopyrightText: 2026 Quack Robotics contributors
// SPDX-License-Identifier: Apache-2.0

//! WebSocket telemetry integration tests: the real telemetry router on a
//! loopback port, read with a WebSocket client.

use std::sync::Arc;
use std::time::Duration;

use futures_util::StreamExt;
use quack_api::hardware::{Hardware, HardwareError, JointTarget, MockHardware};
use quack_api::telemetry::{self, TelemetryFrame};
use quack_api::AppState;
use quack_core::joint::RawJointState;
use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode;
use tokio_tungstenite::tungstenite::Message;

const SPEC: &str = include_str!("../../openapi/quack-api.yaml");

async fn serve(hardware: Arc<dyn Hardware>) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let app = telemetry::router(AppState::new(hardware));
    tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
    format!("ws://{addr}/telemetry")
}

#[tokio::test]
async fn streams_joint_frames_at_50_hz() {
    let url = serve(Arc::new(MockHardware::new())).await;
    let (mut socket, _) = tokio_tungstenite::connect_async(url).await.unwrap();
    let mut frames: Vec<TelemetryFrame> = Vec::new();
    for _ in 0..5 {
        let message = tokio::time::timeout(Duration::from_secs(2), socket.next())
            .await
            .unwrap()
            .unwrap()
            .unwrap();
        let text = message.into_text().unwrap();
        frames.push(serde_json::from_str(text.as_str()).unwrap());
    }
    for frame in &frames {
        assert_eq!(frame.joints.len(), 16);
        let antenna = frame
            .joints
            .iter()
            .find(|joint| joint.name == "right_antenna")
            .unwrap();
        assert_eq!(antenna.servo_id, 255);
        assert_eq!(antenna.velocity_rad_s, None);
    }
    assert!(frames
        .windows(2)
        .all(|pair| pair[1].timestamp_us > pair[0].timestamp_us));
    // 4 intervals at 20 ms each; allow scheduler jitter.
    let span_us = frames[4].timestamp_us - frames[0].timestamp_us;
    assert!((60_000..=200_000).contains(&span_us), "span {span_us} us");
}

struct DeadBus;

impl Hardware for DeadBus {
    fn read_joints(&self) -> Result<Vec<RawJointState>, HardwareError> {
        Err(HardwareError("bus timeout".into()))
    }

    fn move_joints(&self, _targets: &[JointTarget]) -> Result<(), HardwareError> {
        Err(HardwareError("bus timeout".into()))
    }
}

#[tokio::test]
async fn hardware_failure_closes_stream_with_error_code() {
    let url = serve(Arc::new(DeadBus)).await;
    let (mut socket, _) = tokio_tungstenite::connect_async(url).await.unwrap();
    let message = tokio::time::timeout(Duration::from_secs(2), socket.next())
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    let Message::Close(Some(close)) = message else {
        panic!("expected a close frame, got {message:?}");
    };
    assert_eq!(close.code, CloseCode::Error);
    assert_eq!(close.reason.as_str(), "hardware_unavailable");
}

#[test]
fn spec_documents_the_telemetry_frame() {
    assert!(SPEC.contains("TelemetryFrame:"));
    assert!(SPEC.contains("ws://localhost:8081/telemetry"));
}
