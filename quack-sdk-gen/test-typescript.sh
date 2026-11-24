#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2026 Quack Robotics contributors
# SPDX-License-Identifier: Apache-2.0

# Installs the packed TypeScript SDK from the local registry into a scratch
# project and runs the smoke test against quack-api on mock hardware.
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
repo="$(dirname "$here")"
rest_addr="${QUACK_REST_ADDR:-127.0.0.1:18080}"
cargo build --quiet --manifest-path "$repo/Cargo.toml" -p quack-api
QUACK_REST_ADDR="$rest_addr" \
  QUACK_WS_ADDR="${QUACK_WS_ADDR:-127.0.0.1:18081}" \
  QUACK_GRPC_ADDR="${QUACK_GRPC_ADDR:-127.0.0.1:50551}" \
  "$repo/target/debug/quack-api" &
server_pid=$!
trap 'kill "$server_pid"' EXIT
for _ in $(seq 50); do
  curl -fsS "http://$rest_addr/health" >/dev/null 2>&1 && break
  sleep 0.1
done
project="$here/out/ts-smoke"
mkdir -p "$project"
cp "$here/tests/typescript_smoke.mjs" "$project/"
cd "$project"
[ -f package.json ] || npm init -y --silent >/dev/null
npm install --no-audit --no-fund --silent "$here/out/dist/quack-sdk-0.1.0.tgz"
QUACK_URL="http://$rest_addr" node typescript_smoke.mjs
