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
