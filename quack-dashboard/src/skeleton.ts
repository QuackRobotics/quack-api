// SPDX-FileCopyrightText: 2026 Quack Robotics contributors
// SPDX-License-Identifier: Apache-2.0

// Static v0 pose of the duck skeleton, in metres (y up, z forward). Not
// driven by joint angles yet: that needs the frozen joint table and link
// lengths from the MJCF (docs/proposals/joints-v1-provisional.md).

export type Point = readonly [number, number, number];

export const POINTS = {
  pelvis: [0, 0.2, 0],
  neck: [0, 0.25, 0.03],
  head: [0, 0.33, 0.06],
  left_hip: [0.045, 0.19, 0],
  left_knee: [0.045, 0.12, 0.025],
  left_ankle: [0.045, 0.05, -0.01],
  left_toe: [0.045, 0, 0.04],
  right_hip: [-0.045, 0.19, 0],
  right_knee: [-0.045, 0.12, 0.025],
  right_ankle: [-0.045, 0.05, -0.01],
  right_toe: [-0.045, 0, 0.04],
  left_antenna: [0.025, 0.38, 0.05],
  right_antenna: [-0.025, 0.38, 0.05],
} as const satisfies Record<string, Point>;

export type PointName = keyof typeof POINTS;

export const BONES: ReadonlyArray<readonly [PointName, PointName]> = [
  ["pelvis", "neck"],
  ["neck", "head"],
  ["head", "left_antenna"],
  ["head", "right_antenna"],
  ["pelvis", "left_hip"],
  ["left_hip", "left_knee"],
  ["left_knee", "left_ankle"],
  ["left_ankle", "left_toe"],
  ["pelvis", "right_hip"],
  ["right_hip", "right_knee"],
  ["right_knee", "right_ankle"],
  ["right_ankle", "right_toe"],
];
