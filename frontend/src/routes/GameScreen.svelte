<script lang="ts">
  import { type Emote } from '$lib/Emote';
  import Box from '$lib/Box.svelte';
  import { BoxState } from '$lib/BoxState';

  interface Props {
    room_id: string | undefined;
    emote: Emote;
    user_input: string;
    submitGuess: () => void;
    showWrong: boolean;
    onTyping: () => void;
    score: number;
    scores: [string, number][];
    skip: () => void;
  }

  let {
    room_id = undefined,
    emote,
    user_input = $bindable(''),
    submitGuess = () => {},
    showWrong,
    onTyping = () => {},
    score,
    scores,
    skip
  }: Props = $props();
  let emote_guess_field: HTMLInputElement;
  let skip_latch = false;

  let states: BoxState[] = $derived.by(() => {
    return [...emote.matched_chars].map((chr, index) => {
      if (index >= user_input.length) {
        return BoxState.EMPTY;
      }

      if (user_input[index] === chr) {
        return BoxState.CORRECT;
      }

      return showWrong ? BoxState.WRONG : BoxState.EMPTY;
    });
  });

  function handleKeydown(evt: KeyboardEvent) {
    if (evt.key === ' ' && !skip_latch) {
      skip_latch = true;
      evt.preventDefault();
      skip();
    }

    emote_guess_field.focus();
  }

  function onTextFieldKeydown(evt: KeyboardEvent) {
    if (evt.key === 'Enter') {
      submitGuess();
    }

    onTyping();
  }
</script>

<h1 class="text-3xl font-bold underline">Room ID: {room_id}</h1>
<div class="flex flex-col items-center gap-2">
  <img class="max-w-lg flex-none" src={emote.url} alt="pepega just look at the emote" />
  <div class="flex flex-row gap-3">
    {#each emote.matched_chars as chr, index (chr + index)}
      <Box letter={user_input[index] ?? ' '} state={states[index]} />
    {/each}
  </div>
  <input
    bind:this={emote_guess_field}
    bind:value={user_input}
    onkeydown={onTextFieldKeydown}
    maxlength={emote.matched_chars.length}
    id="emote_guess"
    class="h-0 outline-none"
    type="text"
  />
  <p>Your Score: {score.toFixed(1)}</p>
</div>
<div class="flex flex-col items-center gap-2">
  <button
    type="button"
    class="w-full rounded-lg bg-blue-700 px-5 py-2.5 text-center text-sm font-medium
    text-white hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 focus:outline-none sm:w-auto
    dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
    onclick={skip}
    tabindex="0">Skip</button
  >
</div>
<div class="flex flex-col items-center gap-2">
  <h1 class="center text-4xl font-bold">Scoreboard</h1>
  <ul>
    {#each scores as [player, score] (player)}
      <li>{player}: {score}</li>
    {/each}
  </ul>
</div>

<svelte:window
  onkeydown={handleKeydown}
  onkeyup={() => {
    skip_latch = false;
  }}
/>
