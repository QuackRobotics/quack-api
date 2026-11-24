// SPDX-FileCopyrightText: 2026 Quack Robotics contributors
// SPDX-License-Identifier: Apache-2.0

// Smoke test for the packed TypeScript SDK against a running quack-api.
import assert from "node:assert/strict";
import { createClient } from "@quack/sdk";

const client = createClient({ baseUrl: process.env.QUACK_URL });

const health = await client.GET("/health");
assert.equal(health.response.status, 200);

const state = await client.GET("/state");
assert.equal(state.data.joints.length, 16);

const moved = await client.POST("/state", {
  body: { joints: [{ name: "left_knee", position_rad: 0.25 }] },
});
assert.equal(moved.response.status, 200);
const knee = moved.data.joints.find((joint) => joint.name === "left_knee");
assert.ok(Math.abs(knee.position_rad - 0.25) < 1e-6, String(knee.position_rad));

const rejected = await client.POST("/state", {
  body: { joints: [{ name: "no_such_joint", position_rad: 0 }] },
});
assert.equal(rejected.response.status, 422);
assert.ok(rejected.error, "422 must carry an ApiError body");
console.log("typescript sdk smoke: ok");
