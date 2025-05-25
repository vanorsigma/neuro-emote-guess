use std::{collections::HashMap, sync::Arc};
use futures_util::stream::SplitSink;
use tokio::{sync::RwLock, time::Duration};

use serde::{Deserialize, Serialize};
use warp::filters::ws::{Message, WebSocket, Ws};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[repr(transparent)]
pub struct RoomID(pub String);

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[repr(transparent)]
pub struct User(pub String);

#[derive(Debug, Serialize)]
pub struct Emote {
    pub name: String,
    pub id: String,
}

#[derive(Debug, Clone)]
pub struct GameState {
    pub room_id: RoomID,
    pub room_owner: User,
    pub user_map: HashMap<User, u32>,
    pub emote_set_id: String,
    pub duration: Duration,
    pub seed: u64,
}

#[derive(Debug)]
pub struct UserData {
    pub user: User,
    pub ws: SplitSink<WebSocket, Message>
}

#[derive(Debug, Default)]
pub struct AppData {
    pub game_states: HashMap<RoomID, GameState>,
    pub users: HashMap<User, UserData>,
}
pub type AppDataSync = Arc<RwLock<AppData>>;

impl GameState {
    pub fn new(room_id: RoomID, owner_id: User, duration: Duration, seed: u64) -> Self {
        Self {
            room_id,
            room_owner: owner_id.to_owned(),
            user_map: HashMap::from([(owner_id, 0)]),
            emote_set_id: Default::default(),
            duration,
            seed,
        }
    }
}
