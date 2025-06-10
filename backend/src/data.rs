use futures_util::stream::SplitSink;
use std::{collections::HashMap, sync::Arc};
use tokio::{sync::{Mutex, RwLock}, time::Duration};

use serde::{Deserialize, Serialize};
use warp::filters::ws::{Message, WebSocket, Ws};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[repr(transparent)]
pub struct RoomID(pub String);

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[repr(transparent)]
pub struct User(pub String);

#[derive(Debug, Clone)]
pub struct GameState {
    pub room_id: RoomID,
    pub room_owner: User,
    pub user_data: HashMap<User, UserGameData>,
    pub emote_set_id: String,
    pub duration: Duration,
    pub seed: u64,
}

#[derive(Debug, Default, Clone)]
pub struct UserGameData {
    pub score: u32,
    pub emote: u32
}

#[derive(Debug)]
pub struct UserData {
    pub user: User,
    pub ws: SplitSink<WebSocket, Message>,
}

#[derive(Debug, Default)]
pub struct AppData {
    pub game_states: Arc<RwLock<HashMap<RoomID, GameState>>>,
    pub users: Arc<RwLock<HashMap<User, UserData>>>,
}
pub type AppDataSync = Arc<AppData>;

impl GameState {
    pub fn new(room_id: RoomID, owner_id: User, duration: Duration, seed: u64) -> Self {
        Self {
            room_id,
            room_owner: owner_id.to_owned(),
            user_data: HashMap::from([(owner_id.clone(), Default::default())]),
            emote_set_id: Default::default(),
            duration,
            seed,
        }
    }
}
