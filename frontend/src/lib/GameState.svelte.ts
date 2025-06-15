export const gameState = $state({
  started: false,
  display_wrong: false,
  user_id: '',
  room_id: '',
  currentEmote: {
    matched_chars: '',
    url: '',
  },
  guess: '',
  expectedDuration: 100,
});
