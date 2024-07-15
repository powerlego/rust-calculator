<script lang="ts"></script>

<div class={`grid grid-cols-4 grid-rows-6 keypad ${$$props.class}`}>
  <button id="percent" class="calc-button operator-button operator-button-symbol"></button>
  <button id="clear-entry" class="calc-button operator-button">CE</button>
  <button id="clear" class="calc-button operator-button">C</button>
  <button id="backspace" class="calc-button operator-button operator-button-symbol"></button>
  <button id="inverse" class="calc-button operator-button operator-button-symbol"></button>
  <button id="square" class="calc-button operator-button operator-button-symbol"></button>
  <button id="square-root" class="calc-button operator-button operator-button-symbol"></button>
  <button id="divide" class="calc-button operator-button operator-button-symbol"></button>
  <button id="seven" class="calc-button numeric">7</button>
  <button id="eight" class="calc-button numeric">8</button>
  <button id="nine" class="calc-button numeric">9</button>
  <button id="multiply" class="calc-button operator-button operator-button-symbol"></button>
  <button id="four" class="calc-button numeric">4</button>
  <button id="five" class="calc-button numeric">5</button>
  <button id="six" class="calc-button numeric">6</button>
  <button id="subtract" class="calc-button operator-button operator-button-symbol"></button>
  <button id="one" class="calc-button numeric">1</button>
  <button id="two" class="calc-button numeric">2</button>
  <button id="three" class="calc-button numeric">3</button>
  <button id="add" class="calc-button operator-button operator-button-symbol"></button>
  <button id="negate" class="calc-button operator-button-keypad operator-button-symbol"></button>
  <button id="zero" class="calc-button numeric">0</button>
  <button id="decimal" class="calc-button numeric">.</button>
  <button id="equals" class="h-full p-0 m-px font-normal min-h-3 min-w-6 bg-primary font-calculator btn btn-primary"
    ></button
  >
</div>

<style>
  .keypad {
    flex: 3 1 308px;
    container: keypad / size;
  }

  #percent {
    font-size: 14px;
  }

  #clear-entry {
    font-size: 16px;
  }

  #clear {
    font-size: 16px;
  }

  #backspace {
    font-size: 16px;
  }

  #inverse {
    font-size: 18px;
  }

  #square {
    font-size: 18px;
  }

  #square-root {
    font-size: 14px;
  }

  #divide {
    font-size: 14px;
  }

  #multiply {
    font-size: 14px;
  }

  #subtract {
    font-size: 14px;
  }

  #add {
    font-size: 14px;
  }

  #negate {
    font-size: 14px;
  }

  #equals {
    font-size: 14px;
  }

  .numeric {
    font-size: 24px;
  }

  @container (min-height: 0px) or (min-width: 0px) {
    #percent {
      font-size: 12px;
    }

    #clear-entry {
      font-size: 12px;
    }

    #clear {
      font-size: 12px;
    }

    #backspace {
      font-size: 12px;
    }

    #inverse {
      font-size: 12px;
    }

    #square {
      font-size: 12px;
    }

    #square-root {
      font-size: 12px;
    }

    #divide {
      font-size: 12px;
    }

    #multiply {
      font-size: 12px;
    }

    #subtract {
      font-size: 12px;
    }

    #add {
      font-size: 12px;
    }

    #negate {
      font-size: 12px;
    }

    #equals {
      font-size: 12px;
    }

    .numeric {
      font-size: 12px;
    }
  }

  @container (min-height: 260px) {
    #percent {
      font-size: 14px;
    }

    #clear-entry {
      font-size: 14px;
    }

    #clear {
      font-size: 14px;
    }

    #backspace {
      font-size: 14px;
    }

    #inverse {
      font-size: 14px;
    }

    #square {
      font-size: 14px;
    }

    #square-root {
      font-size: 14px;
    }

    #divide {
      font-size: 14px;
    }

    #multiply {
      font-size: 14px;
    }

    #subtract {
      font-size: 14px;
    }

    #add {
      font-size: 14px;
    }

    #negate {
      font-size: 14px;
    }

    #equals {
      font-size: 14px;
    }

    .numeric {
      font-size: 18px;
    }
  }

  @container (min-height: 500px) or (min-width: 468px) {
    #percent {
      font-size: 20px;
    }

    #clear-entry {
      font-size: 20px;
    }

    #clear {
      font-size: 20px;
    }

    #backspace {
      font-size: 20px;
    }

    #inverse {
      font-size: 20px;
    }

    #square {
      font-size: 20px;
    }

    #square-root {
      font-size: 20px;
    }

    #divide {
      font-size: 20px;
    }

    #multiply {
      font-size: 20px;
    }

    #subtract {
      font-size: 20px;
    }

    #add {
      font-size: 20px;
    }

    #negate {
      font-size: 20px;
    }

    #equals {
      font-size: 20px;
    }

    .numeric {
      font-size: 24px;
    }
  }

  @container (min-height: 814px) or (min-width: 780px) {
    #percent {
      font-size: 34px;
    }

    #clear-entry {
      font-size: 34px;
    }

    #clear {
      font-size: 34px;
    }

    #backspace {
      font-size: 34px;
    }

    #inverse {
      font-size: 34px;
    }

    #square {
      font-size: 34px;
    }

    #square-root {
      font-size: 34px;
    }

    #divide {
      font-size: 34px;
    }

    #multiply {
      font-size: 34px;
    }

    #subtract {
      font-size: 34px;
    }

    #add {
      font-size: 34px;
    }

    #negate {
      font-size: 34px;
    }

    #equals {
      font-size: 34px;
    }

    .numeric {
      font-size: 38px;
    }
  }
</style>
