<script lang="ts">
  import { type Emote } from '$lib/Emote';
  import Box from '$lib/Box.svelte';
  import { BoxState } from '$lib/BoxState';

  interface Props {
    room_id: string | undefined;
    emote: Emote;
  }

  let { room_id = undefined, emote }: Props = $props();
  let user_input: string = $state('');
  let emote_guess_field: HTMLInputElement;

  let states: BoxState[] = $derived.by(() => {
    return [...emote.name].map((chr, index) => {
      if (index >= user_input.length) {
        return BoxState.EMPTY;
      }

      return user_input[index] === chr ? BoxState.CORRECT : BoxState.WRONG;
    });
  });

  function handleKeydown() {
    emote_guess_field.focus();
  }
</script>

<h1 class="text-3xl font-bold underline">Room ID: {room_id}</h1>
<div class="flex flex-col items-center gap-2">
  <img class="max-w-lg flex-none" src={emote.link} alt="pepega just look at the emote" />
  <div class="flex flex-row gap-3">
    {#each emote.name as chr, index (index)}
      <Box letter={chr} state={states[index]} />
    {/each}
  </div>
  <input
    bind:this={emote_guess_field}
    bind:value={user_input}
    maxlength={emote.name.length}
    id="emote_guess"
    class="h-0 outline-none"
    type="text"
  />
</div>

<svelte:window onkeydown={handleKeydown} />
