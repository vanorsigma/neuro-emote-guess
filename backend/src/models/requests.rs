use std::time::Duration;

use serde::Deserialize;

use crate::data::{RoomID, User};

#[derive(Deserialize, Debug, Clone)]
pub struct StartGameData {
    pub room_id: RoomID,
}

#[derive(Deserialize, Debug, Clone)]
pub struct EditRoomData {
    pub room_id: RoomID,
    pub game_duration: u64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct JoinRoomData {
    pub room_id: RoomID,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SubmitGuessData {
    pub room_id: RoomID,
    pub guess: String
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "command", rename_all = "snake_case")]
pub enum Request {
    CreateRoom,
    EditRoom(EditRoomData),
    JoinRoom(JoinRoomData),
    StartGame(StartGameData),
    SubmitGuess(SubmitGuessData)
}
