<script lang="ts">
  import { getContext } from "svelte";
  import type { Chip8Wrap } from "chip8_wasm";

  /**
   * Displays the values of registers and maybe also lets you edit them
   */
  let emu: Chip8Wrap = getContext("chip8");
</script>
