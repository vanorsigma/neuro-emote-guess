<script lang="ts">
  interface Props {
    scores: [string, number][];
  }

  let { scores }: Props = $props();
</script>

<div class="flex flex-col gap-2 p-0">
  <h1 class="center text-3xl font-bold">Scoreboard</h1>
  <ul>
    {#each scores as [player, score], idx (player + idx)}
      <li>{player}: {score.toFixed(1)}</li>
    {/each}
  </ul>
</div>

<style>
  div {
    padding: 0px !important;
  }
</style>
