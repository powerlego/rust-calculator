<script lang="ts">
  import { afterUpdate, onMount } from "svelte";

  export let value: string = "0";
  let previousValue: string = "0";
  let isScalingText: boolean = false;

  $: haveCalculatedMax = false;
  let displayContainer: HTMLDivElement;
  let displayTextElement: HTMLSpanElement;
  const maxFontSizes = {
    l: 72,
    m: 46,
    s: 26,
  };

  onMount(() => {
    displayTextElement = document.getElementById("displayText") as HTMLSpanElement;
    displayContainer = document.getElementById("displayContainer") as HTMLDivElement;
    // eslint-disable-next-line @typescript-eslint/no-unsafe-call, @typescript-eslint/no-unsafe-member-access
    window.api.on("windowResize", () => {
      updateFontSize();
    });
    return () => {
      // eslint-disable-next-line @typescript-eslint/no-unsafe-call, @typescript-eslint/no-unsafe-member-access
      window.api.off("windowResize", () => {
        updateFontSize();
      });
    };
  });

  afterUpdate(updateFontSize);

  function addThousandsSeparator(value: string) {
    const [integerPart, decimalPart] = value.split(".");
    return (
      integerPart.replace(/\B(?=(\d{3})+(?!\d))/g, ",")
      + (value.includes(".") ? "." : "")
      + (decimalPart ? `${decimalPart}` : "")
    );
  }

  function updateFontSize() {
    if (!displayTextElement) {
      return;
    }
    displayTextElement.style.opacity = "0";
    const width = displayContainer.clientWidth;
    const maxFontSize = maxFontSizes[window.innerHeight > 800 ? "l" : window.innerHeight > 480 ? "m" : "s"];
    if (!isScalingText || value !== previousValue) {
      previousValue = value;
      isScalingText = true;
      haveCalculatedMax = false;
    }
    const widthDiff = Math.abs(displayTextElement.offsetWidth - width);
    let fontSizeChange = 1;
    if (widthDiff > 50) {
      fontSizeChange = Math.min(Math.max(Math.floor(0.0556513 * widthDiff) - 3, 1), 5);
    }
    if (
      displayTextElement.offsetWidth < width
      && Math.abs(parseInt(displayTextElement.style.fontSize) - maxFontSize) > 0.001
      && !haveCalculatedMax
    ) {
      displayTextElement.style.fontSize = `${Math.min(Math.max(parseInt(displayTextElement.style.fontSize) + fontSizeChange, 12), maxFontSize)}px`;
      return updateFontSize();
    }
    if (fontSizeChange < 5) {
      haveCalculatedMax = true;
    }
    if (displayTextElement.offsetWidth >= width && Math.abs(parseInt(displayTextElement.style.fontSize) - 12) > 0.001) {
      displayTextElement.style.fontSize = `${Math.min(Math.max(parseInt(displayTextElement.style.fontSize) - fontSizeChange, 12), maxFontSize)}px`;
      return updateFontSize();
    }
    isScalingText = false;
    displayTextElement.style.opacity = "1";
  }
</script>

<div
  id="displayContainer"
  class={`flex items-start justify-end align-top w-[calc(100%-1.5rem)] ml-auto mr-3 ${$$props.class}`}
>
  <span id="displayText" class="font-semibold text-base-content" style={"font-size: 46px;"}>
    {addThousandsSeparator(value)}
  </span>
</div>

<style>
  #displayContainer {
    min-height: 42px;
    height: 42px;
    flex: 1 1 72px;
  }
  @media screen and (min-height: 480px) {
    #displayContainer {
      min-height: 72px;
      height: 72px;
    }
  }
  @media screen and (min-height: 800px) {
    #displayContainer {
      min-height: 108px;
      height: 72px;
    }
  }
</style>
