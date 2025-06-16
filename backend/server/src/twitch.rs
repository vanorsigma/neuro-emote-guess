use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TwitchUserData {
    pub id: String,
    pub login: String,
    pub display_name: String
}

#[derive(Deserialize, Clone)]
pub struct TwitchUserResponse {
    pub data: Vec<TwitchUserData>
}
