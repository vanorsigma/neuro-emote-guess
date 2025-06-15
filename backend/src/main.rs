mod handler;

use std::{collections::HashMap, convert::Infallible, ops::Deref, sync::Arc};

use backend::{
    data::{AppData, AppDataSync, GameState, RoomID, User},
    models::requests::Request,
};
use futures_util::{SinkExt, StreamExt, TryFutureExt};
use handler::{
    handle_create_room, handle_create_user, handle_delete_user, handle_edit_room, handle_join_room,
    handle_start_game, handle_submit_guess,
};
use tokio::{
    pin,
    sync::{RwLock, mpsc},
};
use warp::ws::WebSocket;
use warp::{Filter, filters::ws::Ws};

async fn handle_room(app_data: AppDataSync, current_user: User, request: Request) {
    tracing::debug!("Incoming: {request:#?}");
    match request {
        Request::CreateRoom => handle_create_room(app_data, current_user).await,
        Request::EditRoom(edit_room_data) => {
            handle_edit_room(app_data, current_user, edit_room_data).await
        }
        Request::JoinRoom(join_room_data) => {
            handle_join_room(app_data, current_user, join_room_data).await
        }
        Request::StartGame(start_game_data) => {
            handle_start_game(app_data, current_user, start_game_data).await
        }
        Request::SubmitGuess(submit_guess_data) => {
            handle_submit_guess(app_data, current_user, submit_guess_data).await
        }
    }
}

async fn handle_upgrade(app_data: AppDataSync, ws: WebSocket) {
    let (ws_tx, mut ws_rx) = ws.split();
    let (tx, mut rx) = mpsc::unbounded_channel();
    let user = handle_create_user(app_data.clone(), ws_tx).await;

    tracing::info!("Websocket connect");

    let app_data_copy = app_data.clone();
    let user_copy = user.clone();
    tokio::task::spawn(async move {
        while let Some(request) = rx.recv().await {
            handle_room(app_data_copy.clone(), user_copy.clone(), request).await;
        }
    });

    while let Some(result) = ws_rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                tracing::error!("websocket error {e}");
                break;
            }
        };

        tracing::debug!("msg is {:#?}", msg);

        let request = match serde_json::from_str(msg.to_str().expect("can convert to string")) {
            Ok(r) => r,
            Err(e) => {
                tracing::error!("websocket {e}");
                continue;
            }
        };
        tx.send(request).expect("can send to unbounded channel");
    }

    handle_delete_user(app_data, user).await;
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let state: AppDataSync = Arc::new(AppData::default().into());

    let room_operations = warp::path!("ws").and(warp::ws::ws()).map(move |ws: Ws| {
        let state = state.clone();
        ws.on_upgrade(move |socket| handle_upgrade(state, socket))
    });

    warp::serve(room_operations)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
