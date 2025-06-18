export const gameState = $state({
  started: false,
  display_wrong: false,
  user_id: '',
  room_id: '',
  currentEmote: {
    matched_chars: '',
    url: ''
  },
  guess: '',
  score: 0,
  scores: [] as unknown as [string, number][],
  expectedDuration: 100
});
