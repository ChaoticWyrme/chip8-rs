<script lang="ts">
  import type { Chip8 } from "chip8_wasm";
  import { getContext, onMount } from "svelte";
  import CanvasDisplay from "./lib/CanvasDisplay.svelte";
  import { cyclesPerFrame } from "./stores";
  import DevTools from "./lib/DevTools.svelte";
  import { Debouncer } from "./util/functions";

  const emu: Chip8 = getContext("emu");
  emu.load_default();

  // const memory: WebAssembly.Memory = getContext("memory");

  // debugging
  globalThis.chip8 = emu;

  let canvas: CanvasDisplay;
  let timers: HTMLElement;

  let debouncer = new Debouncer(() => emu.tick());
  debouncer.setPerSecond(0);
  // debugging
  globalThis.debouncer = debouncer;

  let intervalID;

  function renderTimer() {
    let timerTimes = emu.timers;
    if (timers != undefined)
      timers.textContent = `Delay: ${timerTimes.delay}\nSound: ${timerTimes.sound}\n`;
  }

  /**
   * For this loop, we run it on an interval.
   * For now, we are not tracking the delta
   */
  function newMainLoop() {
    if (!emu.running) return;
    renderTimer();
    canvas.renderFrame();
    for (let i = 0; i < $cyclesPerFrame; i++) {
      emu.tick();
      if (
        emu.quirks.display_wait &&
        // check if the previous instruction was a draw instruction
        (emu.get_instruction(emu.program_counter - 2) & 0xf000) === 0xd000
      ) {
        // skip other instructions in frame
        break;
      }
    }
  }

  onMount(() => {
    timers = document.getElementById("timers");
    intervalID = setInterval(newMainLoop, 1000 / 60);
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

  let devTools = false;
</script>

<svelte:window on:keydown={handleKeydown} on:keyup={handleKeyup} />

<headers>
  <h3>Chip8-rs</h3>
  <div id="timers" />
</headers>

<CanvasDisplay bind:this={canvas} />

{#if devTools}
  <DevTools on:close={() => (devTools = false)} />
{/if}

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

  headers h3 {
    margin: 0;
  }
</style>
