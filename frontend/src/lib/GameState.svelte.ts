export enum GameStateIdentifier {
  ROOM_INIT,
  ROOM_CONFIG,
  STARTED,
  GAME_OVER
}

export const gameState = $state({
  started: GameStateIdentifier.ROOM_INIT,
  connected: false,
  display_wrong: false,
  user_id: '',
  room_id: '',
  is_owner: false,
  currentEmote: {
    matched_chars: '',
    url: ''
  },
  guess: '',
  score: 0,
  scores: [] as unknown as [string, number][],
  expectedDuration: 100
});
