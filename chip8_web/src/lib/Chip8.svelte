<script lang="ts">
  import type { InitOutput } from "chip8_wasm/chip8_wasm";
  import { Chip8Wrap } from "chip8_wasm";
  import { tick } from "svelte";
  export let bindings: InitOutput;

  let emu = new Chip8Wrap();
  emu.load_ibm();

  let canvas;

  function mainLoop() {
    if (canvas != null) {
      canvas.textContent = emu.render_text();
      emu.tick();
    }

    requestAnimationFrame(mainLoop);
  }

  tick().then(() => {
    canvas = document.getElementById("chip8-canvas");
    mainLoop();
  });

  const KEYMAP = {
    "1": 0x1,
    "2": 0x2,
    "3": 0x3,
    "4": 0xc,
    q: 0x4,
    w: 0x5,
    e: 0x6,
    r: 0xd,
    a: 0x7,
    s: 0x8,
    d: 0x9,
    f: 0xe,
    z: 0xa,
    x: 0x0,
    c: 0xb,
    v: 0xf,
  };

  function handleKeydown(event: KeyboardEvent) {
    let emuKey = KEYMAP[event.key];
    if (emuKey == undefined) {
      // filter out non-emu keys
      return;
    }
    emu.key_down(emuKey);
  }

  function handleKeyup(event: KeyboardEvent) {
    let emuKey = KEYMAP[event.key];
    if (emuKey == undefined) {
      // filter out non-emu keys
      return;
    }
    emu.key_up(emuKey);
  }

  bindings.memory;
</script>

<svelte:window on:keydown={handleKeydown} on:keyup={handleKeyup} />

<pre id="chip8-canvas" />
