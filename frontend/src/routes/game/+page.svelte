<script lang="ts">
  import { type Emote } from '$lib/Emote';
  import GameScreen from './GameScreen.svelte';

  const emote: Emote = {
    name: 'someemote',
    link: 'https://cdn.7tv.app/emote/01JR5WKYHX4A5KMQ8KJPG4YSVF/4x.webp'
  };
</script>

<GameScreen room_id="whateveriwant" {emote}></GameScreen>
