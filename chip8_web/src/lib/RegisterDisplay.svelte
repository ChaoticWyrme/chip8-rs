<script lang="ts">
  import { getContext } from "svelte";
  import type { Chip8 } from "chip8_wasm";

  /**
   * Displays the values of registers and maybe also lets you edit them
   */
  let emu: Chip8 = getContext("chip8");
</script>
