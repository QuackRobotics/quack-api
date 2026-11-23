<!--
SPDX-FileCopyrightText: 2026 Quack Robotics contributors
SPDX-License-Identifier: Apache-2.0
-->

# Joint table v1 (PROVISIONAL)

Source: Atlas, 2026-11-23 10:14 Paris. Read from upstream
Open_Duck_Playground `main` (last touched at `f698836`), not our pinned
`b23317a`. Do not freeze until it has been diffed against `b23317a`.

Indices 0–13 follow the policy/MJCF order. The antennas are 14–15.
Ranges are in radians.

| Index | Name | Min | Max | Notes |
| ----: | ---- | --: | --: | ----- |
| 0 | left_hip_yaw | −0.5236 | +0.5236 | |
| 1 | left_hip_roll | −0.4363 | +0.4363 | |
| 2 | left_hip_pitch | −1.2217 | +0.5236 | mirrored vs right |
| 3 | left_knee | −1.5708 | +1.5708 | |
| 4 | left_ankle | −1.5708 | +1.5708 | |
| 5 | neck_pitch | −0.3491 | +1.1345 | |
| 6 | head_pitch | −0.7854 | +0.7854 | |
| 7 | head_yaw | −2.7925 | +2.7925 | |
| 8 | head_roll | −0.5236 | +0.5236 | |
| 9 | right_hip_yaw | −0.5236 | +0.5236 | |
| 10 | right_hip_roll | −0.4363 | +0.4363 | |
| 11 | right_hip_pitch | −0.5236 | +1.2217 | mirrored vs left |
| 12 | right_knee | −1.5708 | +1.5708 | |
| 13 | right_ankle | −1.5708 | +1.5708 | |
| 14 | left_antenna | −1.5708 | +1.5708 | SG90, GPIO12; proposed, unverified |
| 15 | right_antenna | −1.5708 | +1.5708 | SG90, GPIO13; proposed, unverified |

## Rules

- Hip pitch is mirrored. Validate limits per joint; never assume left = right.
- The joint-index test must match MJCF joint names to indices, not just
  count them.
- Foot switches: GPIO22 left, GPIO27 right, active-low, 3-sample debounce
  (15 ms at 200 Hz). A stuck switch is a fault and sets
  `stance_source = PHASE`.

## TODO before this leaves proposals/

- Diff against Open_Duck_Playground `b23317a`.
- Verify the antenna ranges on hardware; they may narrow.
