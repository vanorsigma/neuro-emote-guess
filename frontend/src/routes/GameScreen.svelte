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
  }

  let {
    room_id = undefined,
    emote,
    user_input = $bindable(''),
    submitGuess = () => {},
    showWrong,
    onTyping = () => {},
    score
  }: Props = $props();
  let emote_guess_field: HTMLInputElement;

  let states: BoxState[] = $derived.by(() => {
    return [...emote.matched_chars].map((chr, index) => {
      if (index >= user_input.length) {
        return BoxState.EMPTY;
      }

      if (user_input[index] === chr) {
        return BoxState.CORRECT;
      }

      console.log('oijawfoei', showWrong);

      return showWrong ? BoxState.WRONG : BoxState.EMPTY;
    });
  });

  function handleKeydown() {
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
  <p>Score:{score}</p>
</div>

<svelte:window onkeydown={handleKeydown} />
