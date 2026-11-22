// SPDX-FileCopyrightText: 2026 Quack Robotics contributors
// SPDX-License-Identifier: Apache-2.0

//! Runs the REST API. Address: `QUACK_REST_ADDR`, default 127.0.0.1:8080.
//! Backend: `QUACK_HARDWARE` = `mock` (default) or `real`; `real` needs
//! `QUACK_JOINT_LIMITS` and is refused until a real driver exists.

use std::sync::Arc;

use quack_api::hardware::MockHardware;
use quack_api::{router, startup, AppState};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let hardware = std::env::var("QUACK_HARDWARE").ok();
    let joint_limits = std::env::var("QUACK_JOINT_LIMITS").ok();
    startup::check(hardware.as_deref(), joint_limits.as_deref())?;

    let addr = std::env::var("QUACK_REST_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    let app = router(AppState::new(Arc::new(MockHardware::new())));
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    eprintln!("quack-api REST listening on {}", listener.local_addr()?);
    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            if let Err(error) = tokio::signal::ctrl_c().await {
                eprintln!("ctrl-c handler failed: {error}");
            }
        })
        .await?;
    Ok(())
}
