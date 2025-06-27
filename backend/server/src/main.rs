#![feature(map_try_insert)]
mod handler;

use std::{collections::HashMap, convert::Infallible, ops::Deref, str::FromStr, sync::Arc};

use backend::{
    data::{AppData, AppDataSync, GameState, RoomID, User},
    jwt::{JWTClaim, JWTClaimError},
    models::{
        requests::{AuthenticateData, Request},
        responses::{ErrorData, ErrorDataType, Response},
    },
    twitch::TwitchUserResponse,
};
use futures_util::{SinkExt, StreamExt, TryFutureExt};
use handler::{
    handle_create_room, handle_create_user, handle_delete_user, handle_edit_room, handle_join_room,
    handle_skip, handle_start_game, handle_submit_guess,
};
use jwt_simple::prelude::{HS256Key, HS512Key};
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderName, HeaderValue};
use std::fs::File;
use std::io::prelude::*;
use thiserror::Error;
use tokio::{
    pin,
    sync::{RwLock, mpsc},
};
use warp::{
    Filter,
    filters::ws::{Message, Ws},
};
use warp::{reply::Reply, ws::WebSocket};

// TODO: read from environment
const CLIENT_ID: &str = "ee92s9l7bxh4fslbqh3svb3ul7hmfi";
const KEY_FILE: &str = "secret.key";

#[derive(Error, Debug)]
pub enum TokenSubmissionError {
    #[error("JWT Error: {0}")]
    ClaimGenerationError(#[from] JWTClaimError),

    #[error("Reqwest Error: {0}")]
    ReqwestError(#[from] reqwest::Error),
}

async fn handle_room(app_data: AppDataSync, current_user: User, request: Request) {
    tracing::debug!("Incoming request {request:#?} from user {current_user:#?}");
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
        Request::Skip(skip_data) => handle_skip(app_data, current_user, skip_data).await,
    }
}

async fn handle_token_submission(
    app_data: AppDataSync,
    token: String,
) -> Result<warp::reply::Json, TokenSubmissionError> {
    tracing::debug!("Token: {}", token);

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {token}"))
            .inspect_err(|e| tracing::error!("{e}"))
            .unwrap(),
    );
    headers.insert(
        HeaderName::from_str("Client-Id").unwrap(),
        HeaderValue::from_static(CLIENT_ID),
    );

    let client = reqwest::Client::new();
    let response = client
        .get("https://api.twitch.tv/helix/users")
        .headers(headers)
        .send()
        .await
        .unwrap();

    let twitch_user_response = match response.json::<TwitchUserResponse>().await {
        Ok(j) => j,
        Err(e) => {
            tracing::error!("Cannot deserialize json from Twitch API {e}");
            return Err(TokenSubmissionError::ReqwestError(e));
        }
    };

    tracing::debug!("User {} logins.", twitch_user_response.data[0].login);

    Ok(warp::reply::json(
        &app_data
            .jwt
            .create_user_token(twitch_user_response.data[0].clone())?,
    ))
}

async fn handle_authenticate_websocket(
    app_data: &AppDataSync,
    ws: &mut WebSocket,
) -> Option<JWTClaim> {
    while let Some(result) = ws.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                tracing::error!("websocket error {e}");
                break;
            }
        };

        tracing::debug!("msg is {:#?}", msg);

        match serde_json::from_str::<AuthenticateData>(msg.to_str().expect("can convert to string"))
            .map(|data| app_data.jwt.verify_user_token(data.jwt))
        {
            Ok(r) => match r {
                Ok(r) => return Some(r),
                Err(e) => {
                    tracing::error!("websocket error {e}");
                    return None;
                }
            },
            Err(e) => {
                // probably an unrecognized message
                tracing::error!("websocket error {e}");
                continue;
            }
        };
    }

    None
}

async fn handle_upgrade(app_data: AppDataSync, mut ws: WebSocket) {
    let claim = match handle_authenticate_websocket(&app_data, &mut ws).await {
        Some(c) => c,
        None => {
            tracing::warn!("Connection did not authenticate on time, return");
            let _ = ws
                .send(Message::text(
                    serde_json::to_string(&Response::Error(ErrorData {
                        error_type: ErrorDataType::AuthFailed,
                        error_msg: "Authentication failed, need to reauth".to_string(),
                    }))
                    .expect("can create error struct for json"),
                ))
                .await;
            return;
        }
    };

    let (ws_tx, mut ws_rx) = ws.split();
    let (tx, mut rx) = mpsc::unbounded_channel();

    let user = handle_create_user(app_data.clone(), ws_tx, claim).await;

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
    console_subscriber::init();
    env_logger::init();

    let key = {
        let mut file = File::open(KEY_FILE)
            .expect("Cannot open key file; try generating it with cargo run --bin generate_key");
        let mut buf: Vec<u8> = Vec::new();
        file.read_to_end(&mut buf)
            .expect("Cannot read from key file");
        HS256Key::from_bytes(&buf)
    };

    let state: AppDataSync = Arc::new(AppData::new(key).into());
    let moved_state = state.clone();

    let room_operations = warp::path!("ws").and(warp::ws::ws()).map(move |ws: Ws| {
        let state = moved_state.clone();
        ws.on_upgrade(move |socket| handle_upgrade(state, socket))
    });

    let cors = warp::cors().allow_any_origin();

    let token_submission = warp::path!("token")
        .and(warp::post())
        .and(warp::body::bytes())
        .and(warp::body::content_length_limit(1024 * 16))
        .and_then(move |data: warp::hyper::body::Bytes| {
            let state = state.clone();
            async move {
                let state = state.clone();
                Ok::<_, Infallible>(
                    handle_token_submission(
                        state,
                        String::from_utf8(data.into_iter().collect()).unwrap(),
                    )
                    .await
                    .map(|j| j.into_response())
                    .inspect_err(|e| tracing::error!("Error while processing token: {e}"))
                    .unwrap_or(warp::reply().into_response()),
                )
            }
        });

    warp::serve(room_operations.or(token_submission).with(cors))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
