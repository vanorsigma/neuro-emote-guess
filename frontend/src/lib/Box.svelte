<script lang="ts">
  import { BoxState } from '$lib/BoxState';

  export let letter = 'A';
  export let state: BoxState = BoxState.EMPTY;
</script>

<div
  class="flex items-center rounded-lg"
  class:empty={state === BoxState.EMPTY}
  class:correct={state === BoxState.CORRECT}
  class:wrong={state === BoxState.WRONG}
>
  <p class="flex-auto text-center text-6xl" class:hidden={letter === 'ඬ'}>
    {letter}
  </p>
</div>

<style>
  div {
    background-color: grey;
    height: 5em;
    width: 5em;
  }

  .empty {
    background-color: grey;
  }

  .correct {
    background-color: green;
  }

  .wrong {
    background-color: red;
  }
</style>
