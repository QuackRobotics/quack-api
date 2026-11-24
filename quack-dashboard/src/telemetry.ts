// SPDX-FileCopyrightText: 2026 Quack Robotics contributors
// SPDX-License-Identifier: Apache-2.0

// Client side of the WebSocket telemetry stream (TelemetryFrame in
// openapi/quack-api.yaml, 50 Hz on :8081/telemetry).
import type { TelemetryFrame } from "@quack/sdk";

/** Parses one message, throwing on anything that is not a TelemetryFrame. */
export function parseFrame(text: string): TelemetryFrame {
  const frame: unknown = JSON.parse(text);
  if (
    typeof frame !== "object" || frame === null ||
    typeof (frame as TelemetryFrame).timestamp_us !== "number" ||
    !Array.isArray((frame as TelemetryFrame).joints)
  ) {
    throw new Error(`not a TelemetryFrame: ${text.slice(0, 80)}`);
  }
  return frame as TelemetryFrame;
}

/** Default stream URL: same host as the page, port 8081. */
export function defaultTelemetryUrl(pageHost: string): string {
  return `ws://${pageHost}:8081/telemetry`;
}
