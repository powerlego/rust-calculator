<script lang="ts">
  import BasicNumpad from "./components/BasicNumpad.svelte";
  import type { ComponentType } from "svelte";
  import Display from "./components/Display.svelte";
  const numpad: ComponentType = BasicNumpad;
</script>

<div class="w-full h-full">
  <div class="relative flex flex-col w-full h-full">
    <!-- Calculator Type select -->
    <div class="flex-grow-0 h-12 bg-green-800 min-h-12"></div>
    <div class="min-h-0 bg-white expression"></div>

    <!-- Input Display -->
    <Display />
    <!-- Memory Functions -->
    <div class="flex flex-row min-h-0 memory mx-[3px]">
      <button
        class="flex-auto h-full min-h-0 text-base font-normal rounded-md btn btn-ghost max-w-20 font-calculator text-base-content"
        ></button
      >
      <button
        class="flex-auto h-full min-h-0 text-base font-normal rounded-md btn btn-ghost max-w-20 font-calculator text-base-content"
        ></button
      >
      <button
        class="flex-auto h-full min-h-0 text-base font-normal rounded-md btn btn-ghost max-w-20 font-calculator text-base-content"
        ></button
      >
      <button
        class="flex-auto h-full min-h-0 text-base font-normal rounded-md btn btn-ghost max-w-20 font-calculator text-base-content"
        ></button
      >
      <button
        class="flex-auto h-full min-h-0 text-base font-normal rounded-md btn btn-ghost max-w-20 font-calculator text-base-content"
        ></button
      >
      <div class="flex-auto h-full"></div>
      <button
        class="h-full min-h-0 text-base font-normal rounded-md max-w-20 btn btn-ghost sm:max-w-0 font-calculator sm:min-w-0 sm:w-0 sm:hidden text-base-content"
        ></button
      >
    </div>
    <!-- Number Pad -->
    <svelte:component this={numpad} class="min-h-0 w-[calc(100%-6px)] mx-[3px] mb-[3px]" />
  </div>
</div>

<style>
  .expression {
    flex: 0.25 3 22px;
  }
  .memory {
    flex: 0.25 3 32px;
  }
</style>
