// SPDX-FileCopyrightText: 2026 Quack Robotics contributors
// SPDX-License-Identifier: Apache-2.0

import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";

// Everything is bundled into dist/ (no CDN), so the dashboard loads from the
// Radxa's static files without internet access.
export default defineConfig({
  plugins: [react()],
  base: "./",
});
