<script lang="ts">
  export let usernames: string[];
</script>

<h2 class="text-2xl">Player List</h2>
<div class="p-2">
  <ul>
    {#each usernames as username, idx (username + idx)}
      <li>{username}</li>
    {/each}
  </ul>
</div>
