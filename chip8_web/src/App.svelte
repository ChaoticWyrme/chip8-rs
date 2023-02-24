<script lang="ts">
  import type { InitOutput } from "chip8_wasm/chip8_wasm";
  import Chip8 from "./lib/Chip8.svelte";
  export let bindings: InitOutput;
  globalThis.bindings = bindings;
</script>

<main>
  <Chip8 {bindings} />
</main>

<style>
</style>
