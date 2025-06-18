<script lang="ts">
  interface Props {
    room_id: string | undefined;
    scores: [string, number][];
    onMenu: () => void;
  }

  let { room_id = undefined, scores, onMenu = () => {} }: Props = $props();
</script>

<h1 class="text-3xl font-bold underline">Room ID: {room_id}</h1>
<div class="flex flex-col items-center gap-2">
  <h1 class="center text-4xl font-bold">Scoreboard</h1>
  <ul>
    {#each scores as [player, score] (player)}
      <li>{player}: {score.toFixed(1)}</li>
    {/each}
  </ul>
</div>
<button onclick={onMenu}>Go to menu</button>
