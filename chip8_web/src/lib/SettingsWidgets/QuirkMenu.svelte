<script lang="ts">
  import { type Chip8, QuirkPresets } from "chip8_wasm";
  import { getContext } from "svelte";
  import { matchPreset } from "../../util/functions";

  const emu: Chip8 = getContext("emu");

  // after making a change, do quirks = emu.quirks,
  // which will update dynamic statements based on quirks
  let quirks = emu.quirks;

  let presets = Object.entries(QuirkPresets);
  // enums have keys for values as well as names, so get the name -> value mappings
  presets = presets.slice(presets.length / 2, presets.length);

  let presetSelector: HTMLSelectElement;

  $: selectedPreset = matchPreset(quirks);

  $: {
    if (presetSelector != undefined) {
      presetSelector.selectedIndex = selectedPreset + 1;
    }
  }

  function onPresetChange(evt: Event) {
    let ele = evt.target as HTMLSelectElement;

    let val = parseInt(ele.value);

    if (val >= 0) {
      quirks.use_preset(val);
      emu.quirks = quirks;
    }
    quirks = emu.quirks;
  }

  function onQuirkChange(evt: Event) {
    let ele: HTMLInputElement = <HTMLInputElement>evt.target;
    emu.change_quirk(ele.name, ele.checked);
    quirks = emu.quirks;
  }
</script>

<form>
  <select name="preset" on:input={onPresetChange} bind:this={presetSelector}>
    <option value={-1} selected={selectedPreset === undefined} disabled>
      Custom Options
    </option>
    {#each presets as [name, value]}
      <option {value} selected={value === selectedPreset}>{name}</option>
    {/each}
  </select>
  <fieldset on:change={onQuirkChange} id="quirkConfig">
    <label for="flag_reset">Reset carry on XOR</label>
    <input
      type="checkbox"
      id="flag_reset"
      name="flag_reset"
      checked={quirks.flag_reset}
    />

    <label for="flag_reset">Set pointer on register load/save</label>
    <input
      type="checkbox"
      id="save_load_set_pointer"
      name="save_load_set_pointer"
      checked={quirks.save_load_set_pointer}
    />

    <label for="display_wait">Wait for next render on sprite draw</label>
    <input
      type="checkbox"
      id="display_wait"
      name="display_wait"
      checked={quirks.display_wait}
    />

    <label for="partial_wrap">Wrap sprites partially</label>
    <input
      type="checkbox"
      id="partial_wrap"
      name="partial_wrap"
      checked={quirks.partial_wrap}
    />

    <label for="alt_shift">Shift destination register directly</label>
    <input
      type="checkbox"
      id="alt_shift"
      name="alt_shift"
      checked={quirks.alt_shift}
    />

    <label for="alt_rel_jump">
      Relative jump uses first nibble as register value
    </label>
    <input
      type="checkbox"
      id="alt_rel_jump"
      name="alt_rel_jump"
      checked={quirks.alt_rel_jump}
    />
  </fieldset>
</form>

<style>
  #quirkConfig {
    display: grid;
    grid-template-columns: auto 1fr;
  }

  #quirkConfig label {
    margin-bottom: 10px;
  }

  #quirkConfig input {
    margin-left: 10px;
  }
</style>
