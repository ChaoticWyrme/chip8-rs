<script lang="ts">
  import { NativeSelect, type SelectItem } from "@svelteuidev/core";
  import QuirkMenu from "./SettingsWidgets/QuirkMenu.svelte";

  let speedData: SelectItem[] = [7, 15, 20, 30, 100, 200, 500, 1000].map(
    (value) => ({
      label: `${value} Cycles/Frame`,
      value: `${value}`,
    })
  );
</script>

<div>
  <h3>Select Quirks</h3>
  <QuirkMenu />

  <h3>Misc. Settings</h3>
  <NativeSelect data={speedData} label="Emulation Speed" />
</div>

<style>
</style>
