<script lang="ts">
  import UserList from '$lib/UserList.svelte';

  interface Props {
    room_id: string;
    joinGame: () => void;
    onGenerateRoom: () => void;
    startGame: () => void;
    expectedDuration: number;
    usernames: string[];
  }

  let {
    room_id = $bindable(''),
    joinGame = () => {},
    onGenerateRoom = () => {},
    startGame = () => {},
    expectedDuration = 0,
    usernames = []
  }: Props = $props();
</script>

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
    bind:value={room_id}
  />
</div>

<div>
  <button type="button" onclick={joinGame}>Join</button>
</div>

<div>
  <button type="button" onclick={onGenerateRoom}>Generate Room ID</button>
</div>

<div>
  <button type="button" onclick={startGame}>Start</button>
</div>

<div>
  <label for="duration">Duration in seconds:</label>
  <input
    type="number"
    id="duration"
    name="duration"
    min="1"
    max="3000"
    bind:value={expectedDuration}
  />
</div>

<div>
  <UserList {usernames} />
</div>
