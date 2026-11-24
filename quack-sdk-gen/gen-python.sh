#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2026 Quack Robotics contributors
# SPDX-License-Identifier: Apache-2.0

# Generates the Python SDK from openapi/quack-api.yaml and builds its wheel
# into quack-sdk-gen/out/dist (the local package registry). The generated
# code is never edited or committed; rerun this script instead.
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
repo="$(dirname "$here")"
mkdir -p "$here/out/python"
uvx --quiet openapi-python-client@0.28.0 generate \
  --path "$repo/openapi/quack-api.yaml" \
  --config "$here/python/config.yaml" \
  --meta uv --overwrite --output-path "$here/out/python"
uv build --quiet --directory "$here/out/python" --out-dir "$here/out/dist"
ls "$here/out/dist"
