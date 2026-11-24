// SPDX-FileCopyrightText: 2026 Quack Robotics contributors
// SPDX-License-Identifier: Apache-2.0

//! Generates the gRPC server and client from quack-proto. Uses a vendored
//! protoc so builds do not depend on a system install.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("PROTOC", protoc_bin_vendored::protoc_bin_path()?);
    tonic_prost_build::configure().compile_protos(
        &[
            "../quack-proto/proto/quack/telemetry/v1/telemetry.proto",
            "../quack-proto/proto/quack/control/v1/control.proto",
        ],
        &["../quack-proto/proto"],
    )?;
    Ok(())
}
