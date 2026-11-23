<!--
SPDX-FileCopyrightText: 2026 Quack Robotics contributors
SPDX-License-Identifier: Apache-2.0
-->

# Audio overlay (draft)

`quack-voicehat-overlay.dts`: googlevoicehat-soundcard fork with SD_MODE on
GPIO4. MAX98357A speaker plus one I2S MEMS mic (part TBD-Woz), I2S 48 kHz, 2 x 32-bit slots
(64 fs, BCLK 3.072 MHz). The `googlevoicehat` codec driver owns GPIO4 and
toggles SD_MODE with stream start/stop, so a power-fail or safe-stop mute must
go through the audio server stopping the stream (or a separate gate), not by
driving GPIO4 externally. Woz decides.

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
