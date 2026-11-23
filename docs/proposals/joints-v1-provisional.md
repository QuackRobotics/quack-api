<!--
SPDX-FileCopyrightText: 2026 Quack Robotics contributors
SPDX-License-Identifier: Apache-2.0
-->

# Joint table v1 (PROVISIONAL)

Source: Atlas, 2026-11-23 10:14 Paris.

Provisional pin for joints 0–13: Open_Duck_Playground MJCF
`playground/open_duck_mini_v2/xmls/open_duck_mini_v2.xml`
(sha256 `968b18de4e3f55b31252155f52779fa490989f5da92bc9b308e0bb4e81d6bb5c`).
Code/harness pin: Playground `b9be205ac64488c23504ca42e5ec790337adeec3`. The
MJCF bytes there equal those at `f6988368da959156d94fcd53b76864724dc52860`,
whose MJCF change was only the sts3215 kp (17.12 → 13.37) and the IMU site.
`b23317a` is an Open_Duck_Mini commit and pins mechanics/BOM only.
PROVISIONAL until Hinton gives the quack-sim sha and his training commit.

Never use the Open_Duck_Mini 16-actuator order (old `robot.xml`, antennas
at 9–10).

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
| 14 | left_antenna | −1.5708 | +1.5708 | SG90; proposed (see Antennas) |
| 15 | right_antenna | −1.5708 | +1.5708 | SG90; proposed (see Antennas) |

## Rules

- Hip pitch is mirrored. Validate limits per joint; never assume left = right.
- The joint-index test must match MJCF joint names to indices, not just
  count them.
- Foot switches: GPIO22 left, GPIO27 right, active-low, 3-sample debounce
  (15 ms at 200 Hz). A stuck switch is a fault and sets
  `stance_source = PHASE`.

## Antennas (14–15): open questions

The pinned MJCF has 14 hinge joints and no antenna joints, so it does not
confirm the antenna range. Upstream Open_Duck_Mini_Runtime (`e4a7ba6f`,
`mini_bdx_runtime/antennas.py`) drives the antennas with a normalized value
in [−1, 1] mapped to a 1–2 ms pulse, with `LEFT_ANTENNA_PIN = board.D13`,
`RIGHT_ANTENNA_PIN = board.D12` and right sign −1. Until Atlas and Woz
settle it:

- Range: ±1.5708 rad is proposed; a 1–2 ms SG90 pulse is usually narrower.
- Pins: upstream has left = GPIO13, right = GPIO12, the reverse of the
  10:14 proposal (left GPIO12, right GPIO13).
- Units: radians in this API, normalized [−1, 1] in the upstream runtime.

## TODO before this leaves proposals/

- Hinton names the training commit; re-check joints 0–13 against it.
- Settle antenna range, pins and sign (above).
