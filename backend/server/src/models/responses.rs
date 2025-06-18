use std::collections::HashMap;

use serde::Serialize;

use crate::{data::{RoomID, User}, seventv::FinalEmote};

#[derive(Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ErrorDataType {
    AuthFailed,
}

#[derive(Serialize, Debug)]
pub struct ErrorData {
    pub error_type: ErrorDataType,
    pub error_msg: String
}

#[derive(Serialize, Debug)]
pub struct EmoteResponse {
    pub matched_chars: String,
    pub url: String,
}

#[derive(Serialize, Debug)]
pub struct NewUserData {
    pub user_id: User,
}

#[derive(Serialize, Debug)]
pub struct RoomJoinData {
    pub room_id: RoomID,
}

#[derive(Serialize, Debug)]
pub struct GameUpdateData {
    pub scores: HashMap<String, f32>
}

#[derive(Serialize, Debug)]
pub struct EmoteData {
    pub emote: EmoteResponse
}

#[derive(Serialize, Debug)]
pub struct GuessData {
    pub matched_chars: String,
    pub score: f32,
}

#[derive(Serialize, Debug)]
pub struct GameOverData {
    // TODO: winning information
    // pub emote: FinalEmote
    pub new_room_id: RoomID,
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
    GameUpdate(GameUpdateData),
    Error(ErrorData),
}
