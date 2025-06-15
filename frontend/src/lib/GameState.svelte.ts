export const gameState = $state({
  started: false,
  user_id: '',
  room_id: '',
  currentEmote: {
    name: '',
    url: '',
  },
  guess: '',
  expectedDuration: 1,
});
