// SPDX-FileCopyrightText: 2026 Quack Robotics contributors
// SPDX-License-Identifier: Apache-2.0

//! gRPC integration tests: a real tonic server on a loopback port over the
//! mock hardware, called through the generated client.

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};

use quack_api::grpc::pb::quack::control::v1::control_service_client::ControlServiceClient;
use quack_api::grpc::pb::quack::control::v1::{GetStateRequest, JointTarget, MoveJointsRequest};
use quack_api::grpc::{incoming, ControlApi};
use quack_api::hardware::MockHardware;
use quack_api::AppState;
use tokio::net::TcpListener;
use tonic::transport::Channel;
use tonic::Code;

async fn client() -> ControlServiceClient<Channel> {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr: SocketAddr = listener.local_addr().unwrap();
    let service = ControlApi::new(AppState::new(Arc::new(MockHardware::new()))).into_server();
    tokio::spawn(
        tonic::transport::Server::builder()
            .add_service(service)
            .serve_with_incoming(incoming(listener)),
    );
    ControlServiceClient::connect(format!("http://{addr}"))
        .await
        .unwrap()
}

fn target(name: &str, position_rad: f32) -> JointTarget {
    JointTarget {
        name: name.into(),
        position_rad,
    }
}

#[tokio::test]
async fn get_state_returns_all_joints_with_antennas_off_bus() {
    let mut client = client().await;
    let state = client
        .get_state(GetStateRequest {})
        .await
        .unwrap()
        .into_inner()
        .state
        .unwrap();
    assert_eq!(state.joints.len(), 16);
    assert!(state.timestamp_us > 0);
    let antenna = state
        .joints
        .iter()
        .find(|joint| joint.name == "left_antenna")
        .unwrap();
    assert_eq!(antenna.servo_id, 255);
    assert!(!antenna.has_feedback);
    assert_eq!(antenna.velocity_rad_s, 0.0);
}

#[tokio::test]
async fn move_joints_moves_and_returns_new_state() {
    let mut client = client().await;
    let state = client
        .move_joints(MoveJointsRequest {
            joints: vec![target("left_knee", 0.5)],
        })
        .await
        .unwrap()
        .into_inner()
        .state
        .unwrap();
    let knee = state
        .joints
        .iter()
        .find(|joint| joint.name == "left_knee")
        .unwrap();
    assert_eq!(knee.position_rad, 0.5);
}

#[tokio::test]
async fn move_joints_rejects_invalid_moves_like_rest() {
    let mut client = client().await;
    for (joints, expected) in [
        (vec![], "joints must not be empty"),
        (vec![target("tail", 0.1)], "unknown joint tail"),
        (
            vec![target("left_knee", 0.1), target("left_knee", 0.2)],
            "listed twice",
        ),
        (vec![target("left_knee", f32::NAN)], "must be finite"),
    ] {
        let status = client
            .move_joints(MoveJointsRequest { joints })
            .await
            .unwrap_err();
        assert_eq!(status.code(), Code::InvalidArgument);
        assert!(status.message().starts_with("invalid_move: "));
        assert!(status.message().contains(expected), "{}", status.message());
    }
    // A rejected move must not have moved anything.
    let state = client
        .get_state(GetStateRequest {})
        .await
        .unwrap()
        .into_inner()
        .state
        .unwrap();
    assert!(state.joints.iter().all(|joint| joint.position_rad == 0.0));
}

#[tokio::test]
async fn get_state_round_trip_median_is_under_5_ms() {
    let mut client = client().await;
    let mut samples: Vec<Duration> = Vec::new();
    for _ in 0..200 {
        let start = Instant::now();
        client.get_state(GetStateRequest {}).await.unwrap();
        samples.push(start.elapsed());
    }
    samples.sort();
    let median = samples[samples.len() / 2];
    assert!(median < Duration::from_millis(5), "median {median:?}");
}
