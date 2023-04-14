<script lang="ts">
  // internal imports
  import RomSwitcher from "./RomSwitcher.svelte";
  import SettingsMenu from "./SettingsMenu.svelte";

  // icons
  import IconGithub from "~icons/mdi/github";
  import IconCog from "~icons/mdi/cog";

  import { ActionIcon, Group, Modal } from "@svelteuidev/core";

  let settingsOpened = false;
</script>

<headers>
  <RomSwitcher />
  <h3>Chip8-rs</h3>
  <Group position="right" spacing="xs">
    <ActionIcon on:click={() => (settingsOpened = true)}>
      <IconCog />
    </ActionIcon>
    <ActionIcon href="https://github.com/ChaoticWyrme/chip8-rs" root="a">
      <IconGithub />
    </ActionIcon>
  </Group>
</headers>

<Modal
  opened={settingsOpened}
  on:close={() => (settingsOpened = false)}
  target="body"
  title="Settings"
>
  <SettingsMenu />
</Modal>

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
