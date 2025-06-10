use serde::Serialize;

use crate::{data::{RoomID, User}, seventv::FinalEmote};

#[derive(Serialize, Debug)]
pub struct NewUserData {
    pub user_id: User,
}

#[derive(Serialize, Debug)]
pub struct RoomJoinData {
    pub room_id: RoomID,
}

#[derive(Serialize, Debug)]
pub struct EmoteData {
    pub emote: FinalEmote
}

#[derive(Serialize, Debug)]
pub struct GuessData {
    pub matched_chars: String,
}

#[derive(Serialize, Debug)]
pub struct GameOverData {
    pub emote: FinalEmote
}

#[derive(Serialize, Debug)]
#[serde(tag = "command", rename_all = "snake_case")]
pub enum Response {
    NewUser(NewUserData),
    RoomJoin(RoomJoinData),
    Emote(EmoteData),
    GuessResponse(GuessData),
    GameStarted,
    GameOver(GameOverData),
}
