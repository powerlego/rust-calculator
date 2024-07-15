<script lang="ts"></script>

<div class={`grid grid-cols-4 grid-rows-6 gap-0.5 keypad ${$$props.class}`}>
  <button
    id="percent"
    class="border-none h-full bg-[color-mix(in_hsl,var(--fallback-n,oklch(var(--n)/var(--tw-bg-opacity))),black_22%)] text-neutral-content font-calculator btn btn-neutral min-h-0"
    ></button
  >
  <button
    id="clear-entry"
    class="bg-[color-mix(in_hsl,var(--fallback-n,oklch(var(--n)/var(--tw-bg-opacity))),black_22%)] text-neutral-content btn btn-neutral border-none font-semibold h-full min-h-0"
    >CE</button
  >
  <button
    id="clear"
    class="border-none bg-[color-mix(in_hsl,var(--fallback-n,oklch(var(--n)/var(--tw-bg-opacity))),black_22%)] text-neutral-content btn btn-neutral font-semibold h-full min-h-0"
    >C</button
  >
  <button
    id="backspace"
    class="border-none bg-[color-mix(in_hsl,var(--fallback-n,oklch(var(--n)/var(--tw-bg-opacity))),black_22%)] text-neutral-content font-calculator btn btn-neutral h-full min-h-0"
    ></button
  >
  <button
    id="inverse"
    class="border-none bg-[color-mix(in_hsl,var(--fallback-n,oklch(var(--n)/var(--tw-bg-opacity))),black_22%)] text-neutral-content font-calculator btn btn-neutral h-full min-h-0"
    ></button
  >
  <button
    id="square"
    class="border-none bg-[color-mix(in_hsl,var(--fallback-n,oklch(var(--n)/var(--tw-bg-opacity))),black_22%)] text-neutral-content font-calculator btn btn-neutral h-full min-h-0"
    ></button
  >
  <button
    id="square-root"
    class="border-none bg-[color-mix(in_hsl,var(--fallback-n,oklch(var(--n)/var(--tw-bg-opacity))),black_22%)] text-neutral-content font-calculator btn btn-neutral h-full min-h-0"
    ></button
  >
  <button
    id="divide"
    class="border-none bg-[color-mix(in_hsl,var(--fallback-n,oklch(var(--n)/var(--tw-bg-opacity))),black_22%)] text-neutral-content font-calculator btn btn-neutral h-full min-h-0"
    ></button
  >
  <button
    id="seven"
    class="h-full min-h-0 text-lg font-semibold border-none bg-neutral text-neutral-content btn btn-neutral">7</button
  >
  <button
    id="eight"
    class="h-full min-h-0 text-lg font-semibold border-none bg-neutral text-neutral-content btn btn-neutral">8</button
  >
  <button
    id="nine"
    class="h-full min-h-0 text-lg font-semibold border-none bg-neutral text-neutral-content btn btn-neutral">9</button
  >
  <button
    id="multiply"
    class="border-none bg-[color-mix(in_hsl,var(--fallback-n,oklch(var(--n)/var(--tw-bg-opacity))),black_22%)] text-neutral-content font-calculator btn btn-neutral h-full min-h-0"
    ></button
  >
  <button
    id="four"
    class="h-full min-h-0 text-lg font-semibold border-none bg-neutral text-neutral-content btn btn-neutral">4</button
  >
  <button
    id="five"
    class="h-full min-h-0 text-lg font-semibold border-none bg-neutral text-neutral-content btn btn-neutral">5</button
  >
  <button
    id="six"
    class="h-full min-h-0 text-lg font-semibold border-none bg-neutral text-neutral-content btn btn-neutral">6</button
  >
  <button
    id="subtract"
    class="border-none bg-[color-mix(in_hsl,var(--fallback-n,oklch(var(--n)/var(--tw-bg-opacity))),black_22%)] text-neutral-content font-calculator btn btn-neutral h-full min-h-0"
    ></button
  >
  <button
    id="one"
    class="h-full min-h-0 text-lg font-semibold border-none bg-neutral text-neutral-content btn btn-neutral">1</button
  >
  <button
    id="two"
    class="h-full min-h-0 text-lg font-semibold border-none bg-neutral text-neutral-content btn btn-neutral">2</button
  >
  <button
    id="three"
    class="h-full min-h-0 text-lg font-semibold border-none bg-neutral text-neutral-content btn btn-neutral">3</button
  >
  <button
    id="add"
    class="border-none bg-[color-mix(in_hsl,var(--fallback-n,oklch(var(--n)/var(--tw-bg-opacity))),black_22%)] text-neutral-content font-calculator btn btn-neutral h-full min-h-0"
    ></button
  >
  <button id="negate" class="h-full min-h-0 border-none bg-neutral text-neutral-content font-calculator btn btn-neutral"
    ></button
  >
  <button
    id="zero"
    class="h-full min-h-0 text-lg font-semibold border-none bg-neutral text-neutral-content btn btn-neutral">0</button
  >
  <button
    id="decimal"
    class="h-full min-h-0 text-lg font-semibold border-none bg-neutral text-neutral-content btn btn-neutral">.</button
  >
  <button id="equals" class="h-full min-h-0 bg-primary text-primary-content font-calculator btn btn-primary"></button>
</div>

<style>
  .keypad {
    flex: 3 1 308px;
  }
</style>
