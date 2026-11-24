// SPDX-FileCopyrightText: 2026 Quack Robotics contributors
// SPDX-License-Identifier: Apache-2.0

import type { TelemetryFrame } from "@quack/sdk";
import { useEffect, useState } from "react";
import { DuckScene } from "./DuckScene.tsx";
import { defaultTelemetryUrl, parseFrame } from "./telemetry.ts";

type Link = "connecting" | "live" | "closed";

const telemetryUrl =
  import.meta.env.VITE_QUACK_WS_URL ?? defaultTelemetryUrl(window.location.hostname);

export function App() {
  const [link, setLink] = useState<Link>("connecting");
  const [closeReason, setCloseReason] = useState("");
  const [frame, setFrame] = useState<TelemetryFrame | null>(null);

  useEffect(() => {
    const socket = new WebSocket(telemetryUrl);
    socket.onopen = () => setLink("live");
    socket.onmessage = (event) => setFrame(parseFrame(String(event.data)));
    socket.onclose = (event) => {
      setLink("closed");
      setCloseReason(`${event.code} ${event.reason}`.trim());
    };
    return () => socket.close();
  }, []);

  return (
    <main style={{ display: "grid", gridTemplateColumns: "1fr 320px", height: "100vh", margin: 0 }}>
      <DuckScene />
      <aside style={{ padding: 12, fontFamily: "system-ui", overflow: "auto" }}>
        <h1 style={{ fontSize: 18 }}>Micro Duck</h1>
        <p data-testid="link">
          Telemetry: {link}
          {closeReason && ` (${closeReason})`} - {telemetryUrl}
        </p>
        <p>Skeleton: static pose (v0)</p>
        <table>
          <tbody>
            {frame?.joints.map((joint) => (
              <tr key={joint.name}>
                <td>{joint.name}</td>
                <td>{joint.position_rad.toFixed(3)} rad</td>
              </tr>
            ))}
          </tbody>
        </table>
      </aside>
    </main>
  );
}
