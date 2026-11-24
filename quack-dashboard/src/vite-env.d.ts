// SPDX-FileCopyrightText: 2026 Quack Robotics contributors
// SPDX-License-Identifier: Apache-2.0

interface ImportMetaEnv {
  /** Overrides the telemetry WebSocket URL (default ws://<page host>:8081/telemetry). */
  readonly VITE_QUACK_WS_URL?: string;
}
