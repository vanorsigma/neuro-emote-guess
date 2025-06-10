<script lang="ts">
  import UserList from '$lib/UserList.svelte';
  import { gameState } from '$lib/GameState.svelte';
  import { Game } from '$lib/Game';
  import GameScreen from './GameScreen.svelte';

  let game = new Game('ws://127.0.0.1:3030/ws');

  function onGenerateRoom() {
    game.createRoom();
  }

  function joinGame() {
    game.joinGame(gameState.room_id);
  }

  function startGame() {
    game.startGame();
  }
</script>

{#if !gameState.started}
  <h1 class="text-3xl font-bold underline">Welcome to my epic game xdxing</h1>
  <div>
    <label for="room_id" class="mb-2 block text-sm font-medium text-gray-900 dark:text-white"
      >Room ID</label
    >
    <input
      type="text"
      id="room_id"
      class="block w-full rounded-lg border border-gray-300 bg-gray-50 p-2.5 text-sm text-gray-900 focus:border-blue-500 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white dark:placeholder-gray-400 dark:focus:border-blue-500 dark:focus:ring-blue-500"
      required
      bind:value={gameState.room_id}
    />
  </div>

  <div>
    <button
      type="button"
      class="w-full rounded-lg bg-blue-700 px-5 py-2.5 text-center text-sm font-medium
    text-white hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 focus:outline-none sm:w-auto
    dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
      on:click={joinGame}>Join</button
    >
  </div>

  <div>
    <button
      type="button"
      class="w-full rounded-lg bg-blue-700 px-5 py-2.5 text-center text-sm font-medium
    text-white hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 focus:outline-none sm:w-auto
    dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
      on:click={onGenerateRoom}>Generate Room ID</button
    >
  </div>

  <div>
    <button
      type="button"
      class="w-full rounded-lg bg-blue-700 px-5 py-2.5 text-center text-sm font-medium
    text-white hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 focus:outline-none sm:w-auto
    dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
      on:click={startGame}>Start</button
    >
  </div>

  <div>
    <UserList usernames={[]} />
  </div>
{/if}

{#if gameState.started}
  <GameScreen room_id={gameState.room_id} emote={gameState.currentEmote}></GameScreen>
{/if}
