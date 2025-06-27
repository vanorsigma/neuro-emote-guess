<script lang="ts">
  interface Props {
    start_seconds: number;
  }

  let { start_seconds = 100 }: Props = $props();
  let seconds = $state(start_seconds);

  $effect(() => {
    // eslint-disable-next-line @typescript-eslint/no-unused-expressions
    seconds;
    setTimeout(() => {
      seconds -= 1;
    }, 1000);
  });
</script>

<div class="fixed right-5 bottom-5 w-fit rounded border text-center">
  <p class="text-9xl">{seconds}</p>
  <p>seconds</p>
</div>
