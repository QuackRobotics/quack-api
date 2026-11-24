<!-- SPDX-FileCopyrightText: 2026 Quack Robotics contributors -->
<!-- SPDX-License-Identifier: Apache-2.0 -->

# quack-dashboard

Dashboard v0: React + Three.js. Shows a static 3D duck skeleton and the
live joint table from the telemetry WebSocket (`ws://<host>:8081/telemetry`,
override with `VITE_QUACK_WS_URL`). Types come from `@quack/sdk`, so build
the SDK first with `quack-sdk-gen/gen-typescript.sh`.

```sh
npm install
npm test
npm run build   # typecheck + bundle into dist/, no CDN assets
```
