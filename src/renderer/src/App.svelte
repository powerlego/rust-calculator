<script lang="ts">
  import BasicNumpad from "./components/BasicNumpad.svelte";
  import type { ComponentType } from "svelte";
  const numpad: ComponentType = BasicNumpad;
</script>

<div class="flex flex-col w-full h-full">
  <!-- Calculator Type select -->
  <div></div>
  <!-- Input Display -->
  <div></div>
  <!-- Memory Functions -->
  <div></div>
  <!-- Number Pad -->
  <svelte:component this={numpad} />
</div>
