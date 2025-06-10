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

Todo:
- Need to send "current user ID" as well

- [ ] Why does startup take forever?
- [ ] Should not send names to the client
- [ ] Backend should determine if `joingame` succeeded (i.e. client should make no decisions without the backend)
- [X] Refactor joining page to its own component
- [ ] Address TODOs in the code
- [ ] Use a mutable hashmap so that other users are not affected by a single user submitting a guess
