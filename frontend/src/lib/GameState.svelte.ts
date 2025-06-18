export enum GameStateIdentifier {
  ROOM_CONFIG,
  STARTED,
  GAME_OVER
}

export const gameState = $state({
  started: GameStateIdentifier.ROOM_CONFIG,
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
