# Emote Guessing Multiplayer

Pre-alpha first version MVP frfr ong

- generate room ID
- people joins room ID with websockets
- generate a random emote set list, send to all websockets
- people starts guessing emotes for fixed duration, say x seconds
-- each user can choose to skip a emote, to break combo
-- if get correct sequentially, continue to have combo
-- combo multiplies score
- once round is over, present leaderboard

websocket spec, everything goes through root:

Client -> Server:
- `{command: create_room, data: {}}`
- `{command: edit_room, data: {game_time: ''}}`
- `{command: join_room, data: {room_id: ''}}`
- `{command: start_game, data: {}}`
- `{command: submit_guess, data: {guess: ''}}`

Server -> Client:
- `{command: emote, data: {emote: {name:, id:}}}`
- `{command: guess_response, data: {matched_chars: '??x??'}}`
- `{command: game_over, data: {emote: {name:, id:}}}`
