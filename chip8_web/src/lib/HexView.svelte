<script lang="ts">
  import { getContext, onDestroy, onMount } from "svelte";
  import type { Chip8 } from "chip8_wasm";
  import { formatHex } from "../util/format";
  import { generateHexRow } from "../util/functions";

  const ROM_SIZE = 0x1000;
  const MAX_ROW_COUNT = 8;

  let rangeInput = 0;
  let rangeStart, rangeEnd, rowCount;

  // This is because rangeInput can be undefined if the input is empty
  $: rangeStart = rangeInput || 0;

  $: rangeEnd = clampRangeEnd(rangeStart);
  function clampRangeEnd(rangeStart): number {
    // Caculate range end based on rangeStart and ROM_SIZE
    //
    let rangeEnd = rangeStart + 0x10 * MAX_ROW_COUNT - 1;
    rangeEnd = Math.min(rangeEnd, ROM_SIZE);

    return rangeEnd;
  }

  $: rowCount = (rangeEnd - rangeStart) / 0x10;

  const memory: WebAssembly.Memory = getContext("memory");
  const emu: Chip8 = getContext("emu");
  const ram_ptr = emu.get_ram_pointer();
  let hexRows: string[] = [];

  function renderHexView() {
    const ram_view = new DataView(memory.buffer, ram_ptr, ROM_SIZE);

    const rowOffsets = [0, 1, 2, 3, 4, 5, 6, 7].map(
      (rowNum) => rangeStart + rowNum * 16
    );

    const rows = rowOffsets
      .filter((rowOffset) => rowOffset <= rangeEnd)
      .map((rowOffset) => generateHexRow(ram_view, rowOffset));

    hexRows = rows;
  }

  let timeoutID;

  function renderLoop() {
    renderHexView();
    timeoutID = setTimeout(renderLoop, 0);
  }

  onMount(() => {
    renderLoop();
  });

  onDestroy(() => {
    clearTimeout(timeoutID);
  });
</script>

<section>
  <h4>hex view</h4>
  <form class="hex-form">
    <input
      id="hex-view-mem-offset"
      name="memOffset"
      type="number"
      min="0"
      max={ROM_SIZE - 0x1}
      step={0x10}
      on:keydown|stopPropagation
      bind:value={rangeInput}
    />
    <label for="hex-view-mem-offset">
      range: {formatHex(rangeStart)} - {formatHex(rangeEnd)}
    </label>
  </form>

  <div class="hex-view">
    <div class="hex-header">
      00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F
    </div>
    {#each hexRows as row}
      <div>{row}</div>
    {/each}
  </div>
</section>

<style>
  .hex-form {
    font-size: 0.875em;
    display: flex;
    justify-content: flex-start;
    align-items: center;
    gap: 4px;
  }

  .hex-form input[type="number"] {
    width: 75px;
    border: medium none;
    margin: 0px;
  }

  .hex-view {
    padding: 0.2em 0px;
    font-size: 0.6em;
    width: max-content;
    white-space: nowrap;
    overflow-x: hidden;
    text-transform: uppercase;
    min-height: 80px;
  }

  .hex-view .hex-header {
    border-bottom: 1px solid var(--chip8-text-primary, #fff);
    margin-bottom: 0.5em;
    padding: 0.5em 0px;
  }
</style>
