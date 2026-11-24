// SPDX-FileCopyrightText: 2026 Quack Robotics contributors
// SPDX-License-Identifier: Apache-2.0

import { useEffect, useRef } from "react";
import * as THREE from "three";
import { BONES, POINTS } from "./skeleton.ts";

/** Renders the static skeleton, slowly turning so it reads as 3D. */
export function DuckScene() {
  const container = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const host = container.current;
    if (!host) return;
    const renderer = new THREE.WebGLRenderer({ antialias: true });
    renderer.setSize(host.clientWidth, host.clientHeight);
    host.appendChild(renderer.domElement);

    const scene = new THREE.Scene();
    scene.background = new THREE.Color("#101418");
    const camera = new THREE.PerspectiveCamera(40, host.clientWidth / host.clientHeight, 0.01, 10);
    camera.position.set(0.45, 0.3, 0.55);
    camera.lookAt(0, 0.18, 0);
    scene.add(new THREE.GridHelper(0.6, 12, "#3a4450", "#222a33"));

    const duck = new THREE.Group();
    const jointMaterial = new THREE.MeshBasicMaterial({ color: "#f5c542" });
    const jointGeometry = new THREE.SphereGeometry(0.008);
    for (const [x, y, z] of Object.values(POINTS)) {
      const sphere = new THREE.Mesh(jointGeometry, jointMaterial);
      sphere.position.set(x, y, z);
      duck.add(sphere);
    }
    const bonePositions = BONES.flatMap(([from, to]) => [...POINTS[from], ...POINTS[to]]);
    const boneGeometry = new THREE.BufferGeometry();
    boneGeometry.setAttribute("position", new THREE.Float32BufferAttribute(bonePositions, 3));
    duck.add(new THREE.LineSegments(boneGeometry, new THREE.LineBasicMaterial({ color: "#e8eef4" })));
    scene.add(duck);

    renderer.setAnimationLoop((timeMs: number) => {
      duck.rotation.y = timeMs / 4000;
      renderer.render(scene, camera);
    });
    return () => {
      renderer.setAnimationLoop(null);
      renderer.dispose();
      host.removeChild(renderer.domElement);
    };
  }, []);

  return <div ref={container} style={{ width: "100%", height: "100%" }} />;
}
