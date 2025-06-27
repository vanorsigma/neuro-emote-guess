<script lang="ts">
  import '../app.css';

  let { children } = $props();
</script>

<main class="contnet-center align-items align-center flex h-[90%] w-full justify-center">
  <div
    class="align-center align-items box-border flex w-[95%] flex-col content-center justify-center"
  >
    {@render children()}
  </div>
</main>
