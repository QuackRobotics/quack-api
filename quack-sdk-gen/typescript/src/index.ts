// SPDX-FileCopyrightText: 2026 Quack Robotics contributors
// SPDX-License-Identifier: Apache-2.0

// Binds the generic openapi-fetch client to the types generated from the
// spec (src/schema.d.ts, written by `npm run generate`, never edited).
import createFetchClient, { type ClientOptions } from "openapi-fetch";
import type { components, paths } from "./schema.js";

export type { components, paths };
export type JointState = components["schemas"]["JointState"];
export type RobotState = components["schemas"]["RobotState"];
export type TelemetryFrame = components["schemas"]["TelemetryFrame"];

export function createClient(options: ClientOptions) {
  return createFetchClient<paths>(options);
}
