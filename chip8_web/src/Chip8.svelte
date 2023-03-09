<script lang="ts">
  import type { Chip8Wrap } from "chip8_wasm";
  import { onDestroy, getContext, onMount } from "svelte";
  import CanvasDisplay from "./lib/CanvasDisplay.svelte";
  import {
    registerImmediate,
    clearImmediate,
    runImmediate,
  } from "./util/setImmediate";

  import roms from "./util/roms";
  import DevTools from "./lib/DevTools.svelte";

  const emu: Chip8Wrap = getContext("emu");
  emu.load_default();

  const memory: WebAssembly.Memory = getContext("memory");

  // debugging
  globalThis.chip8 = emu;

  let canvas: CanvasDisplay;
  let timers: HTMLElement;

  function mainLoop() {
    // canvas.textContent = emu.render_text();
    let timerTimes = emu.get_timers();
    if (timers != undefined)
      timers.textContent = `Delay: ${timerTimes.delay_timer}\nSound: ${timerTimes.sound_timer}\n`;
    emu.tick();

    // setTimeout(mainLoop, 0);
    runImmediate(immediateTag);
  }

  let immediateTag = registerImmediate(mainLoop);
  onDestroy(() => clearImmediate(immediateTag));

  onMount(() => {
    // canvas = document.getElementById("chip8-canvas");
    timers = document.getElementById("timers");
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
    if (event.key === "F8") {
      devTools = !devTools;
    }

    let emuKey = KEYMAP[event.key.toLowerCase()];
    if (emuKey == undefined) {
      // filter out non-emu keys
      return;
    }
    emu.key_down(emuKey);
  }

  function handleKeyup(event: KeyboardEvent) {
    let emuKey = KEYMAP[event.key.toLowerCase()];
    if (emuKey == undefined) {
      // filter out non-emu keys
      return;
    }
    emu.key_up(emuKey);
  }

  let selectedUrl;

  async function switchRoms() {
    console.log(selectedUrl);
    let response = await fetch(selectedUrl, {});
    if (!response.ok) {
      let message = `Error fetching url '${selectedUrl}': \n${response.statusText}`;
      console.error(message);
      alert(message);
      return;
    }
    let data = new Uint8Array(await response.arrayBuffer());
    emu.reset();
    emu.load_rom(data);
  }

  let devTools = false;
</script>

<svelte:window on:keydown={handleKeydown} on:keyup={handleKeyup} />

<headers>
  <div id="selectRom">
    Select ROM:
    <select bind:value={selectedUrl} on:change={() => switchRoms()}>
      {#each roms as { title, filename, url }}
        <option
          value={typeof filename === "string" ? "./roms/" + filename : url}
        >
          {title}
        </option>
      {/each}
    </select>
  </div>

  <h3>Chip8-rs</h3>
  <div id="timers" />
</headers>

<CanvasDisplay bind:this={canvas} />

{#if devTools}
  <DevTools on:close={() => (devTools = false)} />
{/if}

<!-- <pre id="chip8-canvas" /> -->
<style>
  headers {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    display: flex;
    flex-direction: row;
    justify-content: space-around;
    flex-wrap: nowrap;
    border-bottom: black 1px solid;
    padding: 0.4em 0px;
  }

  headers > #selectRom {
    float: left;
  }

  headers h3 {
    margin: 0;
  }
</style>
