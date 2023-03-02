<script lang="ts">
  import { getContext, onMount, tick } from "svelte";
  import type { Chip8Wrap } from "chip8_wasm";

  export let gridWidth = 64;
  export let gridHeight = 32;

  export let pixelSize = 10;

  export let pixelOnColor = "#FFFFFF";
  export let pixelOffColor = "#000000";

  export let enableRenderLoop = true;

  let canvas_ele: HTMLCanvasElement;

  let ctx: CanvasRenderingContext2D;

  const emu: Chip8Wrap = getContext("emu");
  const memory: WebAssembly.Memory = getContext("memory");

  function getIndex(row: number, column: number): number {
    return row * gridWidth + column;
  }

  export function drawPixels() {
    const displayPtr = emu.get_display_pointer();
    const pixels = new Uint8Array(
      memory.buffer,
      displayPtr,
      gridWidth * gridHeight
    );

    ctx.beginPath();

    for (let row = 0; row < gridHeight; row++) {
      for (let col = 0; col < gridWidth; col++) {
        const idx = getIndex(row, col);

        ctx.fillStyle = pixels[idx] === 1 ? pixelOffColor : pixelOnColor;

        ctx.fillRect(col * pixelSize, row * pixelSize, pixelSize, pixelSize);
      }
    }

    ctx.stroke();
  }

  export function renderLoop() {
    // maybe add query of emu.isDisplayDirty() when that is done
    drawPixels();
    if (enableRenderLoop) {
      // will stop if render loop is disabled
      requestAnimationFrame(renderLoop);
    }
  }

  onMount(() => {
    ctx = canvas_ele.getContext("2d");

    if (enableRenderLoop) {
      renderLoop();
    }
  });
</script>

<canvas
  bind:this={canvas_ele}
  width={gridWidth * pixelSize}
  height={gridHeight * pixelSize}
/>

<style>
  canvas {
    margin-top: 35px;
  }
</style>
