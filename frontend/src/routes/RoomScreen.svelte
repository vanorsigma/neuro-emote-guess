<script lang="ts">
  import Scoreboard from '$lib/Scoreboard.svelte';

  interface Props {
    room_id: string;
    joinGame: () => void;
    onGenerateRoom: () => void;
    startGame: () => void;
    expectedDuration: number;
    scores: [string, number][];
    room_owner: boolean;
    disabled: boolean;
    joinedRoom: boolean;
  }

  let {
    room_id = $bindable(''),
    joinGame = () => {},
    onGenerateRoom = () => {},
    startGame = () => {},
    expectedDuration = $bindable(0),
    scores = [],
    room_owner = false,
    disabled = false,
    joinedRoom = false
  }: Props = $props();

  let local_room_id = $state(room_id);

  function localJoinGame() {
    room_id = local_room_id;
    joinGame();
  }
</script>

<div class="flex flex-row">
  <div class="flex-grow">
    <h1 class="text-3xl font-bold">Room Setup</h1>
    <div>
      <label for="room_id" class="mb-2 block text-sm font-medium text-gray-900 dark:text-white"
        >Room ID</label
      >
      <input
        type="text"
        id="room_id"
        class="block w-full rounded-lg border border-gray-300 bg-gray-50 p-2.5 text-sm text-gray-900 focus:border-blue-500 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white dark:placeholder-gray-400 dark:focus:border-blue-500 dark:focus:ring-blue-500"
        required
        {disabled}
        bind:value={local_room_id}
      />
    </div>

    <div class="flex flex-row justify-end gap-5">
      <button type="button" onclick={localJoinGame} disabled={disabled || room_id == local_room_id}
        >Join</button
      >
      <button type="button" onclick={onGenerateRoom} {disabled}>Generate Room ID</button>
      <button type="button" onclick={startGame} disabled={!room_owner || disabled}>Start</button>
    </div>

    <div>
      <label for="duration">Duration in seconds:</label>
      <input
        type="number"
        id="duration"
        name="duration"
        min="1"
        max="3000"
        disabled={!room_owner || disabled}
        bind:value={expectedDuration}
      />
    </div>
  </div>

  <Scoreboard {scores}></Scoreboard>
</div>
