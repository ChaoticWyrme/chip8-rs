<script lang="ts">
  import { getContext } from "svelte";
  import type { Chip8 } from "chip8_wasm";
  import { useRafFn } from "@svelteuidev/composables";

  /**
   * Displays the values of registers and maybe also lets you edit them
   */
  let emu: Chip8 = getContext("emu");

  let timerTimes = emu.timers;

  function updateValues() {
    timerTimes = emu.timers;
  }

  // can get pause and resume functions from this
  useRafFn(updateValues);
</script>

<h3>Timers</h3>
<div id="timers">
  Delay: {timerTimes.delay}
  Sound: {timerTimes.sound}
</div>

<h3>Registers</h3>
<div id="registers" />
