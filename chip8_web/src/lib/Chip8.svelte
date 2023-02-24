<script lang="ts">
  import type { InitOutput } from "chip8_wasm/chip8_wasm";
  import { Chip8Wrap } from "chip8_wasm";
  import { tick } from "svelte";
  export let bindings: InitOutput;

  let emu = new Chip8Wrap();

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

  bindings.memory;
</script>

<pre id="chip8-canvas" />
