<script lang="ts">
  import BasicNumpad from "./components/BasicNumpad.svelte";
  import type { ComponentType } from "svelte";
  import Display from "./components/Display.svelte";
  const numpad: ComponentType = BasicNumpad;
</script>

<div class="relative flex flex-col w-full h-full">
  <!-- Calculator Type select -->
  <div class="flex-grow-0 h-12 bg-green-800 min-h-12"></div>
  <div class="min-h-0 bg-white expression"></div>

  <!-- Input Display -->
  <Display />
  <!-- Memory Functions -->
  <div class="min-h-0 bg-blue-600 memory"></div>
  <!-- Number Pad -->
  <svelte:component this={numpad} class="min-h-0 w-[calc(100%-6px)] mx-[3px] mb-[3px]" />
</div>

<style>
  .expression {
    flex: 0.25 3 22px;
  }
  .memory {
    flex: 0.25 3 32px;
  }
</style>
