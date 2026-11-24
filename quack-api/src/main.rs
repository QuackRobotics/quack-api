// SPDX-FileCopyrightText: 2026 Quack Robotics contributors
// SPDX-License-Identifier: Apache-2.0

//! Runs the Micro Duck API on three ports:
//! - REST: `QUACK_REST_ADDR`, default 127.0.0.1:8080
//! - WebSocket telemetry: `QUACK_WS_ADDR`, default 127.0.0.1:8081
//! - gRPC control: `QUACK_GRPC_ADDR`, default 127.0.0.1:50051
//!
//! Backend: `QUACK_HARDWARE` = `mock` (default) or `real`; `real` is refused
//! until a real driver exists. Joint limits are compiled in; a set
//! `QUACK_JOINT_LIMITS` is refused.

use std::sync::Arc;

use quack_api::grpc::{self, ControlApi};
use quack_api::hardware::MockHardware;
use quack_api::{router, startup, telemetry, AppState};
use tokio::net::TcpListener;
use tokio::sync::watch;

type BoxError = Box<dyn std::error::Error + Send + Sync>;

fn addr(var: &str, default: &str) -> String {
    std::env::var(var).unwrap_or_else(|_| default.to_string())
}

async fn stopped(mut shutdown: watch::Receiver<bool>) {
    // Err means the sender is gone, which also means shut down.
    let _ = shutdown.wait_for(|stop| *stop).await;
}

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    let hardware = std::env::var("QUACK_HARDWARE").ok();
    let joint_limits = std::env::var("QUACK_JOINT_LIMITS").ok();
    startup::check(hardware.as_deref(), joint_limits.as_deref())?;

    let state = AppState::new(Arc::new(MockHardware::new()));
    let rest = TcpListener::bind(addr("QUACK_REST_ADDR", "127.0.0.1:8080")).await?;
    let ws = TcpListener::bind(addr("QUACK_WS_ADDR", "127.0.0.1:8081")).await?;
    let grpc = TcpListener::bind(addr("QUACK_GRPC_ADDR", "127.0.0.1:50051")).await?;
    eprintln!(
        "quack-api REST {} | WS {} | gRPC {}",
        rest.local_addr()?,
        ws.local_addr()?,
        grpc.local_addr()?
    );

    let (stop, shutdown) = watch::channel(false);
    tokio::spawn(async move {
        if let Err(error) = tokio::signal::ctrl_c().await {
            eprintln!("ctrl-c handler failed: {error}");
        }
        // Err means every server has already stopped.
        let _ = stop.send(true);
    });

    let rest_server =
        axum::serve(rest, router(state.clone())).with_graceful_shutdown(stopped(shutdown.clone()));
    let ws_server = axum::serve(ws, telemetry::router(state.clone()))
        .with_graceful_shutdown(stopped(shutdown.clone()));
    let grpc_server = tonic::transport::Server::builder()
        .add_service(ControlApi::new(state).into_server())
        .serve_with_incoming_shutdown(grpc::incoming(grpc), stopped(shutdown));

    tokio::try_join!(
        async { rest_server.await.map_err(BoxError::from) },
        async { ws_server.await.map_err(BoxError::from) },
        async { grpc_server.await.map_err(BoxError::from) },
    )?;
    Ok(())
}
