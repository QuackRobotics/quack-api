// SPDX-FileCopyrightText: 2026 Quack Robotics contributors
// SPDX-License-Identifier: Apache-2.0

//! Guards against the server drifting from the OpenAPI contract. Full schema
//! validation of the spec runs in CI (openapi-spec-validator).

use quack_api::API_VERSION;

const SPEC: &str = include_str!("../../openapi/quack-api.yaml");

#[test]
fn spec_is_openapi_3_1() {
    assert!(SPEC.lines().any(|line| line == "openapi: 3.1.0"));
}

#[test]
fn spec_version_matches_api_version() {
    assert!(
        SPEC.lines()
            .any(|line| line == format!("  version: {API_VERSION}")),
        "info.version in openapi/quack-api.yaml must equal API_VERSION {API_VERSION}"
    );
}

#[test]
fn spec_documents_every_route() {
    for path in ["/health", "/info", "/state"] {
        assert!(
            SPEC.lines().any(|line| line == format!("  {path}:")),
            "missing path {path}"
        );
    }
    for operation in ["getHealth", "getInfo", "getState", "moveJoints"] {
        assert!(
            SPEC.contains(&format!("operationId: {operation}")),
            "missing operation {operation}"
        );
    }
}
