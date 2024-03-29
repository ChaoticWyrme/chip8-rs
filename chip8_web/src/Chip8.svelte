<script lang="ts">
  import type { Chip8 } from "chip8_wasm";
  import { getContext, onMount } from "svelte";
  import CanvasDisplay from "./lib/CanvasDisplay.svelte";
  import { cyclesPerFrame, running } from "./stores";
  import DevTools from "./lib/DevTools.svelte";
  import { Debouncer } from "./util/functions";
  import NavBar from "./lib/NavBar.svelte";
  import { useRafFn, useThrottle } from "@svelteuidev/composables";

  const emu: Chip8 = getContext("emu");
  emu.load_default();

  // const memory: WebAssembly.Memory = getContext("memory");

  // debugging
  globalThis.chip8 = emu;

  let canvas: CanvasDisplay;

  let debouncer = new Debouncer(() => emu.tick());
  debouncer.setPerSecond(0);
  // debugging
  globalThis.debouncer = debouncer;

  let intervalID;

  /**
   * For this loop, we run it on an interval.
   * For now, we are not tracking the delta
   */
  function mainLoop() {
    if (!emu.running) return;
    for (let i = 0; i < $cyclesPerFrame; i++) {
      emu.tick();
      if (
        emu.quirks.display_wait &&
        // check if the next instruction is a draw instruction
        (emu.get_instruction(emu.program_counter) & 0xf000) === 0xd000
      ) {
        // skip other instructions in frame
        i = $cyclesPerFrame;
      }
    }
    canvas?.renderFrame();
  }

  $: emu.running = $running;

  const { pause, resume } = useRafFn(useThrottle(mainLoop, 1000 / 60));

  $: $running ? pause() : resume();

  onMount(() => {
    intervalID = setInterval(mainLoop, 1000 / 60);
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

    if (event.key.toLowerCase() === "p") {
      running.update((curVal) => !curVal);
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

<NavBar />

<CanvasDisplay bind:this={canvas} />

{#if devTools}
  <DevTools on:close={() => (devTools = false)} />
{/if}
