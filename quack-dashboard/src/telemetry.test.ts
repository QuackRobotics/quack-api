// SPDX-FileCopyrightText: 2026 Quack Robotics contributors
// SPDX-License-Identifier: Apache-2.0

import assert from "node:assert/strict";
import { test } from "node:test";
import { defaultTelemetryUrl, parseFrame } from "./telemetry.ts";

test("parses a frame", () => {
  const frame = parseFrame('{"timestamp_us": 1, "joints": []}');
  assert.equal(frame.timestamp_us, 1);
});

test("rejects a message without joints", () => {
  assert.throws(() => parseFrame('{"timestamp_us": 1}'), /not a TelemetryFrame/);
});

test("default URL uses the page host and port 8081", () => {
  assert.equal(defaultTelemetryUrl("radxa.local"), "ws://radxa.local:8081/telemetry");
});
