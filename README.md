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

- [X] Why does startup take forever?
- [X] Should not send names to the client
- [X] Backend should determine if `joingame` succeeded (i.e. client should make no decisions without the backend)
- [X] Refactor joining page to its own component
- [X] Game end
- [X] Restrict input to exclude '?'
- [ ] Make URLS be env variables
- [ ] Address TODOs in the code
- [ ] Use a mutable hashmap so that other users are not affected by a single user submitting a guess
- [ ] If the emote name is too long, need to flex the boxes
- [ ] Editing room needs to reflect for other players
- [ ] Need security against copy & paste
- [ ] Need timer in frontend
- [ ] Player list does not refresh immediately on restart
- [ ] Restarting is broken
- [ ] Changing duration of the game is broken
