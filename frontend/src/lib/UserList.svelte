<script lang="ts">
  export let usernames: string[];
</script>

<h2 class="text-xl">Player List</h2>
<ul class="list-decimal">
  {#each usernames as username (username)}
    <li>{username}</li>
  {/each}
</ul>
