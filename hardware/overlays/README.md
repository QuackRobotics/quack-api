<!--
SPDX-FileCopyrightText: 2026 Quack Robotics contributors
SPDX-License-Identifier: Apache-2.0
-->

# Audio overlay (draft)

`quack-voicehat-overlay.dts`: googlevoicehat-soundcard fork with SD_MODE on
GPIO4. MAX98357A speaker plus one TDK T5848 I2S MEMS mic, 1 channel (PROPOSED,
Woz; the driver compatible string stays `ics43432`), I2S 48 kHz, 2 x 32-bit slots
(64 fs, BCLK 3.072 MHz). The `googlevoicehat` codec driver owns GPIO4 and
toggles SD_MODE with stream start/stop, so a power-fail or safe-stop mute must
go through the audio server stopping the stream (or a separate gate), not by
driving GPIO4 externally. Woz decides. SD_MODE has a pull-down
(`brcm,pull = <1>`), so the amp is muted by default until the driver raises
it.

Upstream: raspberrypi/linux branch `rpi-6.12.y`,
`arch/arm/boot/dts/overlays/googlevoicehat-soundcard-overlay.dts`, last
changed in commit `3a44d1cb497e2c28bdcbda21369111287be953db` (blob
`1063f18985628833b4d143150fac34ce389741e0`). The upstream file has no SPDX
header, so it falls under the kernel's `COPYING`: GPL-2.0-only. Our fork
keeps `GPL-2.0-only`.

Build:

```sh
dtc -@ -I dts -O dtb -o quack-voicehat.dtbo quack-voicehat-overlay.dts
```

Compiles with dtc 1.6.1 (1272 B). Not tested on a live kernel.

## Bench TODO (Pi Zero 2W)

- [ ] `modinfo snd-soc-googlevoicehat-codec` on the shipped kernel
- [ ] `aplay` and `arecord -f S32_LE -r 48000 -c 2` running at the same time
- [ ] scope: BCLK 3.072 MHz, mic in the left slot
- [ ] SD_MODE on GPIO4 follows stream start/stop
- [ ] SD_MODE reads low (amp muted) with no stream and with the codec driver unloaded
