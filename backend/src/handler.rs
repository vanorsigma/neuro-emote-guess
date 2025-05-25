use std::{
    convert::Infallible,
    ops::{Deref, DerefMut},
    time::Duration,
};

use backend::{
    data::{AppData, AppDataSync, Emote, GameState, RoomID, User},
    models::{
        requests::{
            EditRoomData, JoinRoomData, Request, StartGameData, SubmitGuessData,
        },
        responses::{EmoteData, NewUserData, Response},
    },
};
use futures_util::{SinkExt, stream::SplitSink};
use rand::Rng;
use uuid::{Uuid, uuid};
use warp::filters::ws::{Message, WebSocket, Ws};

const DEFAULT_DURATION_SEC: u64 = 30;

/// Utilities

pub async fn is_user_exists(app_data: &AppData, user: User) -> bool {
    app_data.users.contains_key(&user)
}

pub async fn reply_to_user(app_data: &mut AppData, user: User, message: Message) {
    match app_data.users.get_mut(&user) {
        Some(m) => m.ws.send(message).await.unwrap(),
        None => return,
    };
}

/// Room Handlers

pub async fn handle_create_room(app_data: AppDataSync, user_id: User) {
    let uuid = Uuid::new_v4();
    let seed: u64 = rand::random();

    if !is_user_exists(app_data.read().await.deref(), user_id.clone()).await {
        return;
    }

    let mut app_data = app_data.write().await;
    app_data.deref_mut().game_states.insert(
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
        &mut app_data,
        user_id,
        Message::text(format!("{},{}", uuid, seed)),
    )
    .await
}

pub async fn handle_edit_room(app_data: AppDataSync, user_id: User, data: EditRoomData) {
    if !is_user_exists(app_data.read().await.deref(), user_id.clone()).await {
        return;
    }

    let mut app_data = app_data.write().await;
    let game_states = &mut app_data.game_states;
    let game_state = match game_states.get_mut(&data.room_id) {
        Some(gs) => gs,
        None => {
            tracing::info!("Edit room attempted on room ID that doesn't exist");
            return;
        }
    };

    game_state.duration = data.game_duration;
    reply_to_user(&mut app_data, user_id, Message::text("OK")).await
}

pub async fn handle_join_room(app_data: AppDataSync, user_id: User, data: JoinRoomData) {
    if !is_user_exists(app_data.read().await.deref(), user_id.clone()).await {
        return;
    }

    let mut app_data = app_data.write().await;
    let game_state = match app_data.game_states.get_mut(&data.room_id) {
        Some(gs) => gs,
        None => return,
    };

    game_state.user_map.insert(user_id.clone(), 0);
    reply_to_user(&mut app_data, user_id, Message::text("ok")).await
}

pub async fn handle_start_game(app_data: AppDataSync, user_id: User, data: StartGameData) {
    if !is_user_exists(app_data.read().await.deref(), user_id.clone()).await {
        return;
    }

    let mut app_data = app_data.write().await;
    let game_state = match app_data.game_states.get(&data.room_id) {
        Some(gs) => gs.clone(),
        None => return,
    };

    if game_state.room_owner != user_id {
        return;
    }

    let users = &mut app_data.users;
    for user in game_state.user_map.keys() {
        let data = match users.get_mut(user) {
            Some(d) => d,
            None => continue,
        };

        let _ = data
            .ws
            .send(Message::text(
                serde_json::to_string(&Response::Emote(EmoteData {
                    emote: Emote {
                        id: "placeholder".to_string(), // TODO
                        name: "placeholder name".to_string(),
                    },
                }))
                .unwrap(),
            ))
            .await;
    }

    reply_to_user(&mut app_data, user_id, Message::text("OK")).await;
}

pub async fn handle_submit_guess(app_data: AppDataSync, user_id: User, data: SubmitGuessData) {
    if !is_user_exists(app_data.read().await.deref(), user_id.clone()).await {
        return;
    }

    let mut app_data = app_data.write().await;
    let mut game_states = &mut app_data.game_states;
    let mut users = &mut app_data.users;
    let game_state = match &mut app_data.game_states.get_mut(&data.room_id) {
        Some(gs) => gs,
        None => return,
    };

    todo!()
}

pub async fn handle_create_user(
    app_data: AppDataSync,
    mut ws: SplitSink<WebSocket, Message>,
) -> User {
    let mut app_data = app_data.write().await;
    let users = &mut app_data.users;
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
    let mut app_data = app_data.write().await;
    let users = &mut app_data.users;
    users.remove(&user);
}
