<script lang="ts">
  import HexView from "./HexView.svelte";

  // if this is true, float to the left, to the right otherwise
  let left = false;

  let minimized = false;
</script>

<div class="fixed-wrapper" class:right={!left} class:left>
  <div class="devtools-wrapper bg-primary">
    <div class="devtools-top">
      <h2>devtools</h2>
      <div class="devtools-top-buttons">
        <button
          class="minimize-button"
          on:click={() => (minimized = !minimized)}>_</button
        >
        <button
          class="toggle-pos-btn"
          type="button"
          aria-label="move {left ? 'left' : 'right'}"
          title="move {left ? 'left' : 'right'}"
          on:click={() => (left = !left)}
        >
          &gt;
        </button>
        <button>X</button>
      </div>
    </div>
    <div class="devtools-body" class:minimized>
      <HexView />
    </div>
  </div>
</div>

<style>
  .fixed-wrapper {
    position: fixed;
    top: 2.1em;
    right: 0px;
    margin: 6px;
    padding: 4px;
    height: 100vh;
    z-index: 11;
  }

  .left {
    right: auto;
    left: 0px;
  }

  .right {
    right: 0px;
    left: auto;
  }

  .devtools-wrapper {
    display: block;
    border: 1px solid gray;
    overflow: hidden;
    width: 320px;
    max-height: 100%;
  }

  .devtools-top {
    display: grid;
    grid-template-areas: ". win-btn";
    justify-content: space-between;
    align-items: center;
    overflow: hidden;
    padding: 0px 8px;
    width: 100%;
    height: 2.9em;
  }

  .devtools-top-buttons {
    grid-area: win-btn;
    height: inherit;
    user-select: none;
    display: flex;
    justify-content: flex-end;
    align-items: center;
    gap: 4px;
  }

  .devtools-top-buttons button {
    border-radius: 4px;
    background: transparent;
    box-shadow: none;
    color: inherit;
    display: flex;
    justify-content: center;
    align-items: center;
    width: 2.2em;
    height: 2.2em;
    cursor: pointer;
    border: medium none;
  }

  .fixed-wrapper.right .toggle-pos-btn {
    transform: rotateY(-180deg);
  }

  h2 {
    font-size: 1em;
  }

  .devtools-body {
    padding: 0px 8px 8px;
  }

  .minimized {
    display: none;
  }
</style>
