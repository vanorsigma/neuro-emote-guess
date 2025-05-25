use serde::Serialize;

use crate::data::{Emote, RoomID, User};

#[derive(Serialize, Debug)]
pub struct NewUserData {
    pub user_id: User,
}

#[derive(Serialize, Debug)]
pub struct RoomCreateData {
    pub room_id: RoomID,
    pub seed: u64,
}

#[derive(Serialize, Debug)]
pub struct EmoteData {
    pub emote: Emote
}

#[derive(Serialize, Debug)]
pub struct GuessData {
    pub matched_chars: String,
}

#[derive(Serialize, Debug)]
pub struct GameOverData {
    pub emote: Emote
}

#[derive(Serialize, Debug)]
#[serde(tag = "command", rename_all = "snake_case")]
pub enum Response {
    NewUser(NewUserData),
    RoomCreate(RoomCreateData),
    Emote(EmoteData),
    GuessResponse(GuessData),
    GameStarted,
    GameOver(GameOverData),
}
