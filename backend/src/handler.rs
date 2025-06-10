use std::{
    collections::HashMap,
    convert::Infallible,
    ops::{Deref, DerefMut},
    time::Duration,
};

use backend::{
    data::{AppData, AppDataSync, GameState, RoomID, User, UserData, UserGameData},
    models::{
        requests::{EditRoomData, JoinRoomData, Request, StartGameData, SubmitGuessData},
        responses::{EmoteData, NewUserData, Response, RoomJoinData},
    },
    seventv::{FinalEmote, get_emote_for_emote_set_id},
};
use futures_util::{SinkExt, stream::SplitSink};
use rand::{Rng, SeedableRng, seq::IndexedRandom};
use rand_chacha::ChaCha8Rng;
use uuid::{Uuid, uuid};
use warp::filters::ws::{Message, WebSocket, Ws};

// TODO: temporary constant
const EMOTE_SET_ID: &str = "01J452JCVG0000352W25T9VEND";
const DEFAULT_DURATION_SEC: u64 = 30;

/// Utilities

pub async fn is_user_exists(app_data: &AppData, user: User) -> bool {
    app_data.users.read().await.contains_key(&user)
}

pub async fn reply_to_user(user_map: &mut HashMap<User, UserData>, user: User, message: Message) {
    match user_map.get_mut(&user) {
        Some(m) => m.ws.send(message).await.unwrap(),
        None => return,
    };
}

/// Room Handlers

pub async fn handle_create_room(app_data: AppDataSync, user_id: User) {
    let uuid = Uuid::new_v4();
    let seed: u64 = rand::random();

    if !is_user_exists(&app_data, user_id.clone()).await {
        return;
    }

    let mut game_states = app_data.game_states.write().await;
    game_states.insert(
        RoomID(uuid.to_string()),
        GameState::new(
            RoomID(uuid.to_string()),
            user_id.clone(),
            Duration::from_secs(DEFAULT_DURATION_SEC),
            seed,
        ),
    );

    // TODO: fix this
    reply_to_user(
        &mut (*app_data.users.write().await),
        user_id,
        Message::text(
            serde_json::to_string(&Response::RoomJoin(RoomJoinData {
                room_id: RoomID(uuid.to_string()),
            }))
            .unwrap(),
        ),
    )
    .await
}

pub async fn handle_edit_room(app_data: AppDataSync, user_id: User, data: EditRoomData) {
    if !is_user_exists(&app_data, user_id.clone()).await {
        return;
    }

    let mut game_states = app_data.game_states.write().await;
    let game_state = match game_states.get_mut(&data.room_id) {
        Some(gs) => gs,
        None => {
            tracing::info!("Edit room attempted on room ID that doesn't exist");
            return;
        }
    };

    game_state.duration = data.game_duration;
    reply_to_user(
        &mut (*app_data.users.write().await),
        user_id,
        Message::text("OK"),
    )
    .await
}

pub async fn handle_join_room(app_data: AppDataSync, user_id: User, data: JoinRoomData) {
    if !is_user_exists(&app_data, user_id.clone()).await {
        return;
    }

    let mut game_states = app_data.game_states.write().await;
    let game_state = match game_states.get_mut(&data.room_id) {
        Some(gs) => gs,
        None => return,
    };

    game_state
        .user_data
        .insert(user_id.clone(), Default::default());
    reply_to_user(
        &mut (*app_data.users.write().await),
        user_id,
        Message::text(
            serde_json::to_string(&Response::RoomJoin(RoomJoinData {
                room_id: data.room_id,
            }))
            .unwrap(),
        ),
    )
    .await;
}

fn choose_random_emote(emote: &Vec<FinalEmote>, seed: u64, emote_index: u32) -> FinalEmote {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);

    // "wastes" the random for up to emote_index times
    for _ in 0..emote_index {
        let _ = emote.choose(&mut rng).unwrap();
    }

    emote.choose(&mut rng).unwrap().clone()
}

async fn send_random_emote(app_data: &mut AppDataSync, user: User, room_id: RoomID) {
    let game_states = app_data.game_states.read().await;
    let game_state = match game_states.get(&room_id) {
        Some(gs) => gs,
        None => return,
    };

    let game_user_data = match game_state.user_data.get(&user) {
        Some(d) => d,
        None => return,
    };

    let emotes = get_emote_for_emote_set_id(EMOTE_SET_ID.to_string())
        .await
        .inspect_err(|e| tracing::error!("{}", e))
        .unwrap();
    let emote = choose_random_emote(&emotes, game_state.seed, game_user_data.emote);

    reply_to_user(
        &mut (*app_data.users.write().await),
        user,
        Message::text(serde_json::to_string(&Response::Emote(EmoteData { emote })).unwrap()),
    )
    .await;
}

async fn send_random_emote_to_room(app_data: &mut AppDataSync, room_id: RoomID) {
    let users = {
        let game_states = app_data.game_states.read().await;
        match game_states.get(&room_id) {
            Some(gs) => gs.user_data.keys().cloned().collect::<Vec<User>>(),
            None => return,
        }
    };

    for user in users {
        send_random_emote(app_data, user, room_id.clone()).await;
    }
}

pub async fn handle_start_game(mut app_data: AppDataSync, user_id: User, data: StartGameData) {
    if !is_user_exists(&app_data, user_id.clone()).await {
        return;
    }

    let is_room_owner = {
        let game_states = app_data.game_states.read().await;
        let game_state = match game_states.get(&data.room_id) {
            Some(gs) => gs,
            None => return,
        };

        game_state.room_owner == user_id
    };

    if is_room_owner {
        send_random_emote_to_room(&mut app_data, data.room_id).await
    }
}

pub async fn handle_submit_guess(mut app_data: AppDataSync, user_id: User, data: SubmitGuessData) {
    if !is_user_exists(&app_data, user_id.clone()).await {
        return;
    }

    let scored_increase = {
        let game_states = &mut app_data.game_states.write().await;
        let game_state = match game_states.get_mut(&data.room_id) {
            Some(gs) => gs,
            None => return,
        };

        let user_data = match game_state.user_data.get_mut(&user_id) {
            Some(u) => u,
            None => return,
        };

        let emotes = get_emote_for_emote_set_id(EMOTE_SET_ID.to_string())
            .await
            .inspect_err(|e| tracing::error!("{}", e))
            .unwrap();
        let target_emote = choose_random_emote(&emotes, game_state.seed, user_data.emote);

        tracing::debug!("Target Emote: {:#?}", target_emote);
        if target_emote.name == data.guess {
            user_data.score += 1;
            user_data.emote += 1;
            true
        } else {
            false
        }
    };

    if scored_increase {
        send_random_emote(&mut app_data, user_id.clone(), data.room_id.clone()).await;
    }
}

pub async fn handle_create_user(
    app_data: AppDataSync,
    mut ws: SplitSink<WebSocket, Message>,
) -> User {
    let users = &mut app_data.users.write().await;
    let uuid = Uuid::new_v4();

    let user = User(uuid.to_string());

    let _ = ws
        .send(Message::text(
            serde_json::to_string(&Response::NewUser(NewUserData {
                user_id: user.clone(),
            }))
            .unwrap(),
        ))
        .await
        .unwrap();

    users.insert(
        user.clone(),
        backend::data::UserData {
            user: user.clone(),
            ws,
        },
    );

    user
}

pub async fn handle_delete_user(app_data: AppDataSync, user: User) {
    let users = &mut app_data.users.write().await;
    users.remove(&user);
}
