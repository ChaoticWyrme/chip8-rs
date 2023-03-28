<script lang="ts">
  import type { Chip8 } from "chip8_wasm";
  import { getContext } from "svelte";
  import roms, { type Rom } from "../util/roms";

  let emu: Chip8 = getContext("emu");

  let selectedRom: Rom;
  let romSelector: HTMLSelectElement;

  async function switchRoms() {
    console.log(selectedRom);
    let url =
      typeof selectedRom.filename === "string"
        ? "./roms/" + selectedRom.filename
        : selectedRom.url;

    let response = await fetch(url, {});
    if (!response.ok) {
      let message = `Error fetching url '${url}': \n${response.statusText}`;
      console.error(message);
      alert(message);
      return;
    }

    if (selectedRom.quirks != undefined) {
      emu.quirks.use_preset(selectedRom.quirks);
    }

    let data = new Uint8Array(await response.arrayBuffer());
    emu.reset();
    emu.load_rom(data);
    romSelector.blur();
  }
</script>

<div id="selectRom">
  Select ROM:
  <select
    bind:this={romSelector}
    bind:value={selectedRom}
    on:change={() => switchRoms()}
  >
    {#each roms as rom}
      <option value={rom}>
        {rom.title}
      </option>
    {/each}
  </select>
</div>

<style>
  #selectRom {
    float: left;
  }
</style>
