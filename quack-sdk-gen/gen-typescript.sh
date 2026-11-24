#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2026 Quack Robotics contributors
# SPDX-License-Identifier: Apache-2.0

# Generates and builds the TypeScript SDK (@quack/sdk) from
# openapi/quack-api.yaml and packs it into quack-sdk-gen/out/dist, the local
# package registry. Generated types are never edited or committed.
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
cd "$here/typescript"
npm install --no-audit --no-fund --silent
npm run --silent build
mkdir -p "$here/out/dist"
npm pack --silent --pack-destination "$here/out/dist"
