<script lang="ts">
  import '../app.css';

  let { children } = $props();
</script>

<main class="align-center align-items align-center flex w-full content-center justify-center">
  <div class="w-[90%]">
    {@render children()}
  </div>
</main>
