# SPDX-FileCopyrightText: 2026 Quack Robotics contributors
# SPDX-License-Identifier: Apache-2.0

"""Smoke test for the generated Python SDK against a running quack-api."""

import os
from http import HTTPStatus

from quack_sdk import Client
from quack_sdk.api.state import get_state, move_joints
from quack_sdk.api.system import get_health, get_info
from quack_sdk.models import ApiError, JointTarget, MoveRequest, RobotState


def main() -> None:
    client = Client(base_url=os.environ["QUACK_URL"])

    assert get_health.sync_detailed(client=client).status_code == HTTPStatus.OK
    info = get_info.sync(client=client)
    assert info is not None, "GET /info returned no body"

    state = get_state.sync(client=client)
    assert isinstance(state, RobotState), state
    assert len(state.joints) == 16, len(state.joints)

    moved = move_joints.sync(
        client=client,
        body=MoveRequest(joints=[JointTarget(name="left_knee", position_rad=0.25)]),
    )
    assert isinstance(moved, RobotState), moved
    knee = next(joint for joint in moved.joints if joint.name == "left_knee")
    assert abs(knee.position_rad - 0.25) < 1e-6, knee.position_rad

    rejected = move_joints.sync_detailed(
        client=client,
        body=MoveRequest(joints=[JointTarget(name="no_such_joint", position_rad=0.0)]),
    )
    assert rejected.status_code == HTTPStatus.UNPROCESSABLE_ENTITY, rejected.status_code
    assert isinstance(rejected.parsed, ApiError), rejected.parsed
    print("python sdk smoke: ok")


if __name__ == "__main__":
    main()
