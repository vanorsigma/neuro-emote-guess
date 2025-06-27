<script lang="ts">
  import { gameState, GameStateIdentifier } from '$lib/GameState.svelte';
  import { Game } from '$lib/Game';
  import GameScreen from './GameScreen.svelte';
  import GameOver from './GameOver.svelte';
  import RoomScreen from './RoomScreen.svelte';

  let game = new Game(import.meta.env.VITE_HOST, getSessionTokenOrRedirect());

  function getSessionTokenOrRedirect() {
    const sessionToken = document.cookie
      .split('; ')
      .find((row) => row.startsWith('session_token='))
      ?.split('=')[1];

    if (!sessionToken) {
      window.history.pushState({}, '', '/login');
      return '';
    }

    return sessionToken;
  }

  function onGenerateRoom() {
    game.createRoom();
  }

  function joinGame() {
    game.joinGame(gameState.room_id);
  }

  function startGame() {
    game.startGame();
  }

  function submitGuess() {
    game.submitGuess(gameState.guess.padEnd(gameState.currentEmote.matched_chars.length, 'ඬ'));
  }

  function toMenu() {
    game.resetState();
  }

  function onTyping() {
    gameState.display_wrong = false;
  }

  function skip() {
    game.skip();
  }

  $effect(() => {
    if (gameState.started !== GameStateIdentifier.STARTED) {
      game.editGame();
      return;
    }
  });
</script>

{#if gameState.started === GameStateIdentifier.ROOM_CONFIG || gameState.started === GameStateIdentifier.ROOM_INIT}
  {#key gameState.room_id}
    <RoomScreen
      bind:room_id={gameState.room_id}
      {joinGame}
      {onGenerateRoom}
      {startGame}
      bind:expectedDuration={gameState.expectedDuration}
      scores={gameState.scores}
      room_owner={gameState.is_owner}
      disabled={!gameState.connected}
      joinedRoom={gameState.started === GameStateIdentifier.ROOM_CONFIG}
    ></RoomScreen>
  {/key}
{/if}

{#if gameState.started === GameStateIdentifier.STARTED}
  <GameScreen
    room_id={gameState.room_id}
    emote={gameState.currentEmote}
    bind:user_input={gameState.guess}
    {submitGuess}
    showWrong={gameState.display_wrong}
    {onTyping}
    score={gameState.score}
    {skip}
    scores={gameState.scores}
    duration={gameState.expectedDuration}
  ></GameScreen>
{/if}

{#if gameState.started === GameStateIdentifier.GAME_OVER}
  <GameOver room_id={gameState.room_id} scores={gameState.scores} onMenu={toMenu}></GameOver>
{/if}

<footer class="center">
  <p>
    Status: <span
      class:text-green-600={gameState.connected}
      class:text-red-600={!gameState.connected}
      >{gameState.connected ? 'Connected' : 'Disconnected'}</span
    >
  </p>
</footer>
